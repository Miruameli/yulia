# Yulia Language — Error and Panic Model (Draft 0.5)

**Date:** 2026-11-09
**Status:** Draft for review (Phase 0 deliverable 0.5)
**Depends on:** `docs/specs/2026-11-09-yulia-operational-semantics.md` (0.2), `docs/specs/2026-11-09-yulia-region-inference.md` (0.3)
**Plan ref:** unavailable — `docs/plans/` was lost on 2026-11-10 (see decision log #82).

---

## 1. Purpose

This document defines how Yulia handles errors, panics, and region cleanup on unwind. It ensures that memory safety (MEM-SAFE-1) holds even when a program aborts or panics. **Scope:** §2–§4 describe the Phase 1 model (abort on runtime error, no recovery). Phase 2+ may introduce recoverable errors (see §10), subject to a grammar extension for generics.

## 2. Error taxonomy (Phase 1 model)

Yulia has two classes of errors:

| Class | When | Recoverable? | Example |
|---|---|---|---|
| **Compile-time error** | Type checking, region inference | No (program rejected) | Type mismatch, dangling reference, uninitialized read |
| **Runtime error** | Bounds check failure, integer overflow (if trapping) | No (program aborts) | Array index out of bounds, division by zero |
| **Panic** | Explicit `panic()` call, or unrecoverable runtime error | No (program aborts) | `panic("unreachable")` |

There is no exception mechanism. Yulia does not have `try`/`catch`/`throw`. Errors are either caught at compile time or abort the program.

**Rationale:** Exceptions complicate the memory model (regions must be cleaned up on every exit path). Aborting on runtime errors simplifies the model: the program either completes successfully or terminates. This aligns with the simplicity philosophy.

**Phase 2+ (recoverable errors):** Phase 2 may introduce a `Result` type for recoverable errors (e.g., file not found, parse failure). This requires a grammar extension for generics (`Result<T, E>`) or a built-in non-generic `Result` type with a fixed error representation (e.g., `Result` with `E = string`). The decision is deferred to Phase 2. If `Result` is added, it is a value type (not an exception), and functions return it explicitly. Panic remains for unrecoverable errors.

**Syntax note:** `panic(...)` is a reserved keyword in the 0.1 grammar (§2.5) and is documented in 0.1 §10.1. It is a Phase 2 feature.

### 2.1 Diagnostic format and exit behavior

Compile-time diagnostics are written to stderr as:

```
<file>:<line>:<col>: error[<code>]: <message>
```

`<file>` is the source path as given on the command line; `<line>` and `<col>` are 1-based. The compiler exits with status 1. The code is **stable**: a given diagnostic keeps its code across releases (0.9 §4); the message text may change.

Runtime errors are written to stderr as:

```
<file>:<line>:<col>: runtime error[<code>]: <message>
```

then the program aborts (`abort()`, SIGABRT/134) with no region cleanup (D2, §4.4).

### 2.2 Phase 1 diagnostic catalog

Codes are grouped by compiler phase. `E` = compile-time error, `R` = runtime error.

| Code | Phase | Diagnostic |
|---|---|---|
| E0101 | lexer | unexpected character |
| E0102 | lexer | unterminated string literal |
| E0103 | lexer | invalid numeric literal |
| E0104 | lexer | invalid escape sequence |
| E0201 | parser | unexpected token |
| E0202 | parser | unexpected end of file |
| E0203 | parser | reserved keyword used as an identifier (0.1 §2.5) |
| E0204 | parser | struct literal is missing fields (0.1 §3.4) |
| E0205 | parser | empty array literal needs a type annotation (0.1 §3.4) |
| E0301 | name resolution | undeclared name |
| E0302 | name resolution | duplicate definition |
| E0303 | name resolution | unknown type |
| E0304 | name resolution | assignment to a `const` |
| E0401 | type checker | type mismatch (expected vs found) |
| E0402 | type checker | bad operand type for operator |
| E0403 | type checker | `if`/`while` condition is not `bool` |
| E0404 | type checker | `for` range bounds are not `int` |
| E0405 | type checker | call arity mismatch |
| E0406 | type checker | argument type mismatch |
| E0407 | type checker | `return` type mismatch |
| E0408 | type checker | `return` arity vs `void` |
| E0409 | type checker | index is not `int` |
| E0410 | type checker | field access on a non-struct |
| E0411 | type checker | index on a non-array |
| E0501 | control flow | `break` outside a loop (0.1 §3.3) |
| E0502 | control flow | `continue` outside a loop (0.1 §3.3) |
| E0503 | control flow | `return` outside a function (0.1 §3.3) |
| E0504 | definite assignment | read before assignment (MEM-SAFE-4) |
| E0601 | program structure | missing `main` |
| E0602 | program structure | multiple `main` definitions |
| E0603 | program structure | `main` must be `fn main() -> void` (0.1 §3.6) |
| R0101 | runtime | array index out of bounds (MEM-SAFE-2, 0.2 §113) |
| R0102 | runtime | division by zero |
| R0103 | runtime | integer overflow trap (D2) |

## 3. Panic semantics

A panic immediately terminates the current function and propagates up the call stack. **(Phase 2+; in Phase 1, panic calls `abort()` — §4.4.)** No further statements in the current function are executed. The panic is not catchable.

**Panic propagation:**
1. The current function's region is reclaimed (see §4).
2. The panic propagates to the caller.
3. The caller's region is reclaimed.
4. This continues until the program's entry point (`main`) is reached.
5. At `main`, the program terminates with a non-zero exit code and prints a panic message.

**Panic message:** includes the file, line, and column of the panic site, plus a user-provided message (if any).

---

## 4. Region cleanup on unwind

### 4.1 Normal return

On normal function return, the function's region is popped and its memory is reclaimed (0.2 §5.3). The return value is copied into the caller's region before reclamation.

### 4.2 Panic unwind (Phase 2+)

**Phase 1 note:** In Phase 1, panic calls `abort()` (§4.4). No explicit region cleanup is performed; the OS reclaims all memory. The rules below apply to Phase 2+, when explicit region cleanup is implemented.

On panic, the function's region is popped and its memory is reclaimed. **No return value is produced.** The panic propagates to the caller, which also reclaims its region.

**Invariant:** Every region that was created during the execution of a panicking function (including nested function calls) is reclaimed. No memory is leaked on panic.

### 4.3 Cleanup ordering

Regions are reclaimed in reverse order of creation (LIFO), matching the region stack. This ensures that a region is never reclaimed while a nested region still references it.

```
regions = [R0, R1, R2]  (R0 = main, R1 = f, R2 = g)
panic in g:
  reclaim R2
  propagate to f
  reclaim R1
  propagate to main
  reclaim R0 (program exits)
```

### 4.4 Stack unwinding

Yulia does not use C++ exceptions or stack unwinding for panic. Instead, panic is implemented as a `longjmp`-style control transfer (or an abort in Phase 1). The runtime maintains a panic handler stack.

**Phase 1 (no references):** Panic calls `abort()` (or equivalent). The OS reclaims all memory. No explicit region cleanup is needed because the process terminates.

**Phase 2 (references):** Panic triggers explicit region cleanup. The runtime walks the region stack and reclaims each region. This requires the runtime to track which regions are active.

**Phase 5 (concurrency):** Panic in one thread does not affect other threads. Each thread has its own region stack.

---

## 5. Early return

Yulia does not have early return via exceptions. `return` is the only way to exit a function early. `return` always reclaims the function's region and copies the return value to the caller's region.

There is no `break`/`continue` at the function level (only inside loops). There is no `goto`.

---

## 6. Defer (future consideration)

A `defer` construct (run code on function exit, regardless of path) is **not** included in MVP. It is a Phase 5 consideration. Without `defer`, cleanup is limited to region reclamation, which is automatic.

**Rationale:** `defer` adds complexity to the region model (deferred code may reference values that are being reclaimed). Deferring it to Phase 5 keeps the model simple.

---

**Phase 1 note:** In Phase 1, panic calls `abort()` (§4.4). No setjmp/longjmp boundary is needed because the process terminates. The setjmp/longjmp boundary described below applies to Phase 2+.

When a panic occurs during a C FFI call:
- The C function does not return normally.
- Yulia's runtime catches the panic (via a setjmp/longjmp boundary at the FFI boundary).
- The C function's stack is unwound by the C runtime (not by Yulia).
- Yulia reclaims its regions as in §4.2.

**Invariant:** C code must not assume that Yulia will return normally from a panicking call. C code that allocates memory during a Yulia call must be prepared for the memory to be leaked (Yulia cannot reclaim C-allocated memory).

---

## 8. Interaction with growable collections

When a panic occurs while a growable collection is being modified (e.g., during reallocation):
- The collection's buffer may be in an inconsistent state.
- The collection's region is reclaimed, which frees the buffer.
- No partial state is observable because the panic terminates the function.

- **Outer-region rule:** If the growable collection resides in an **outer region** (a region that outlives the panicking function), the collection MUST NOT be observable in a partially-modified state. The runtime MUST either (a) complete the modification atomically before the panic propagates, or (b) roll back the modification to the pre-operation state. If neither is possible, the collection is in an **indeterminate state** and the program MUST panic at the outer-region boundary before the caller can observe it. This rule is enforced in Phase 2+ when references to outer-region collections are permitted. **Status:** the transactional semantics (atomicity/rollback) is not yet specified; it is a Phase 2 deliverable (D5).

---

## 9. Acceptance

- [x] Error taxonomy defined (compile-time vs runtime vs panic).
- [x] Panic propagation specified.
- [x] Region cleanup on unwind specified (normal return, panic, early return).
- [x] Cleanup ordering (LIFO) specified.
- [x] Stack unwinding approach specified (Phase 1: abort; Phase 2: explicit cleanup; Phase 5: per-thread).
- [x] C FFI interaction specified.
- [x] Growable collection interaction specified.
- [ ] Formal proof (Phase 5.4).

---

## 10. Deliberation (SOP-THINK)

| Field | Content |
|---|---|
| **Goal** | Define the error/panic model and region cleanup on unwind. |
| **Assumptions** | No exceptions (abort on runtime error); regions are reclaimed on every exit path. |
| **Evidence** | 0.2 §4–5; 0.3 §4–5; Go (panic/recover), Rust (panic/unwind), Zig (error sets). |
| **Alternatives** | (a) Exceptions with try/catch; (b) abort-only; (c) error values (Result type). |
| **Chosen** | (b) for Phase 1–2 (abort on runtime error); (c) for Phase 2+ (Result type for recoverable errors). |
| **Risks** | Abort-only may be too restrictive for production; Result type adds complexity. |
| **Mitigations** | Phase 2 may introduce a `Result` type for recoverable errors (requires grammar extension for generics or a built-in non-generic `Result`); panic remains for unrecoverable errors. Decision deferred to Phase 2. |
| **Confidence** | High on panic model; medium on Result type integration. |
| **Kill-criteria** | If the abort-only model proves too restrictive for the MVP, escalate before Phase 2. |
| **Verification plan** | Peer review; negative tests (panic in nested function, panic during FFI call); differential testing. |