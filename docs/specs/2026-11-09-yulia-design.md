# Yulia Language — Design Document

**Date:** 2026-11-09
**Status:** Draft for review

## Intent

Yulia is a procedural programming language with explicit types and a custom modern syntax. Its purpose is to establish a **new approach to memory and performance**: memory that is automatic, deterministic, and invisible to the programmer wherever the compiler can prove it safe, while targeting C-class performance and Python-class simplicity of memory management.

The language combines ideas that already exist in other languages — region-based memory, dense data layout, compile-time inference — and bounds their complexity with explicit restrictions (§Restrictions and fallback). No lifetime annotations (region lifetime is inferred; `pin` is the sole explicit escape hatch), no manual allocation of Yulia values (C memory is explicit via `c_malloc`/`c_free`, R-FFI-8), no tracing garbage collector.

**One-sentence technical claim:** *Compiler-inferred region-based memory management for a decidable safe subset, with dense packing and slot reuse — no implicit `malloc`/`free`, no tracing GC, no lifetime annotations, deterministic reclamation for scoped regions.*

## Goals

- **Extreme performance** — targeting C-class speed (0.8 §2.2); no GC overhead. Region inference is O(n) for Phase 1 and O(n·k) for Phase 2 (0.3 §8).
- **Extreme memory efficiency** — dense packing, slot reuse, no per-object headers for plain structs. Aggregates carry length/capacity metadata (0.6 §4.4).
- **Simplicity** — Python-like ergonomics of memory management; the programmer never manages Yulia memory. C memory (`c_malloc`/`c_free`) and `pin` are explicit, documented exceptions.
- **Procedural paradigm** — functions, structs, modules; clear step-by-step control flow.
- **Explicit types** — the programmer writes types; no hidden type inference. Region inference is memory inference, not type inference.
- **Custom modern syntax** — assembled from proven constructs (0.1 §1.1), designed for readability and simplicity.
- **Full ecosystem** — compiler, LSP, formatter, build tooling.

## Non-goals

- Object-oriented programming (classes, inheritance, virtual dispatch).
- Dynamic typing / runtime type checking.
- Stop-the-world garbage collection.
- Rust-style lifetime *annotations*. Ownership is enforced for resource handles (move/affine + exclusivity, D27), but there is no borrow checker over regions.
- C++-level complexity and footguns.

## Anti-goals

- Becoming C++: decades of baggage, complexity, undefined behavior.
- Becoming Python: object overhead, GC pauses, interpreter speed.
- Exposing *implicit* memory management to the programmer. C memory (`c_malloc`/`c_free`, R-FFI-8) and the `pin` escape hatch are explicit, documented exceptions.

## Constraints

- **Implementation language:** Rust.
- **Compilation target:** native machine code via LLVM IR.
- **Memory management:** deterministic; no GC.
- **Runtime:** no heavyweight runtime; libc plus a small region/panic runtime (0.5 §4).
- **License:** MPL-2.0.

## Approaches considered

| Approach | Description | Trade-off |
|---|---|---|
| A. Static memory layout | Every value assigned a slot at compile time; slots reused via liveness analysis | Most extreme performance and efficiency; dynamic data structures become very hard |
| **B. Region-based memory *(chosen)*** | Memory managed in compiler-inferred regions; a region is reclaimed when it goes out of scope | Flexible enough for dynamic data within the safe subset; no GC; deterministic; compiler is more complex |
| C. Bytecode VM | Custom VM carrying the new memory model | Easiest to iterate and extend; performance not extreme |

**Chosen: B.** It matches the vision — no visible lifecycle, slot reuse, automatic management — while staying flexible enough for real programs within the safe subset. Approach A's static layout can be layered on later as an optimization for values the compiler can prove are short-lived.

## Memory model

The programmer's model: memory is a flat, ever-present substrate. Values are written densely, and slots are reused automatically. There is no allocation or deallocation step visible in the language.

Mechanically:

- **Region-based.** Memory is allocated in *regions*. A region is a function-bound arena; its contents are reclaimed together when the region exits. (R.2-promoted values are never reclaimed — see below.)
- **Compiler-inferred.** The compiler decides where each value lives and which region it belongs to, for the decidable safe subset. The programmer writes no lifetime annotations; `pin` is the sole explicit escape hatch.
- **Dense packing.** Values are laid out compactly: no implicit padding, no per-object headers for plain structs. AoS is the default layout; SoA is an explicit Phase 5 optimization (0.6 §4.3).
- **Slot reuse.** Memory slots are recycled; there is no `malloc`/`free` churn for Yulia-managed values.
- **Deterministic.** No GC pauses. Reclamation happens at region exit, at a point the programmer can reason about. R.2-promoted values are never reclaimed (documented, warned leak).

### Metaphor → mechanism

| Metaphor | Mechanism |
|---|---|
| The standard book | The process address space / region heap |
| A neatly torn sheet | A region (allocation unit) |
| Writing with very small letters | Dense packing; AoS default, SoA on request |
| Lamp A replaced by lamp B | Slot reuse / freelist recycling, not repeated alloc+free for Yulia-managed values |
| Everything managed from behind | Compiler-managed lifetime and ownership; invisible for values in scoped regions (`pin` and C memory are explicit) |

## Restrictions and fallback

The memory-safety guarantee holds for a **decidable subset** of the language. Full region inference for arbitrary imperative programs is not generally decidable (see the design review §2); the language therefore bounds the claim and makes the boundary explicit.

- **R.1 (restrict).** A program that the compiler cannot prove safe is **rejected at compile time**. The diagnostic names the specific rule violated (the naming rule is pending, 0.4 §6). The language's real semantics is this safe subset, not the full grammar.
- **R.2 (warned escape hatch).** When a future feature cannot be made safe, the value is promoted to a **global region** (never reclaimed) and the compiler emits a **warning**. This is a documented, **unbounded** leak — not a tracing garbage collector. It is an explicit opt-in deviation, never silent. R.2 is reserved for future features; it is not available in Phase 1–2 (0.4 §5).
- **No stronger inference is claimed.** A novel inference algorithm that removes the escape hatch is not asserted. The claim "compiler-inferred region-based memory, no lifetime annotations" holds within the safe subset; R.2 is a warned deviation from deterministic reclamation.

See `docs/specs/2026-11-09-yulia-region-inference.md` (0.3 §6), `docs/specs/2026-11-09-yulia-escape-analysis.md` (0.3a), and `docs/specs/2026-11-09-yulia-fallback-policy.md` (0.4).

## Syntax

- Custom modern syntax, assembled from proven constructs (0.1 §1.1).
- Explicit types on all declarations.
- Procedural constructs: functions, structs, modules.
- Concrete grammar is specified in `docs/specs/2026-11-09-yulia-grammar-draft.md` (0.1).

## Toolchain

- **Compiler:** Rust → LLVM IR → native code (LLVM via inkwell; inkwell enters in Phase 1b, codegen).
- **LSP:** scheduled from the start (a schedule, not a design — the LSP design is a later deliverable); a custom syntax needs tooling to be usable.
- **Formatter:** scheduled from the start, driven by the same grammar (a schedule, not a design).
- **Build system:** Cargo; LLVM via inkwell.

## MVP feature set

This is the **whole-MVP** feature set, not the Phase 1 subset. Phase 1 is specified in 0.3 §3 and delivers `int`, `bool`, structs, fixed-size arrays, functions, and control flow, **plus incremental codegen** (LLVM/inkwell) for that subset — staged: 1a front-end, 1b codegen backend. No `float`, no `char`, no `string`.

- Primitive types: `int`, `float`, `bool`, `char`, `string`
- Functions / procedures
- Structs
- Arrays
- Modules
- Explicit control flow: `if`, `while`, `for`
- No classes; no tracing GC; no lifetime annotations (see §Restrictions and fallback)

## Risks

- **Custom syntax** raises adoption and tooling cost; the LSP and formatter must land early.
- **Automatic memory + extreme performance** is a strong claim; the region model must be genuinely deterministic, or the claim becomes a slogan.
- **Explicit types** may hurt ergonomics compared to Python; the syntax must keep declarations light.
- **C FFI** is designed in Phase 0 (0.6). **Concurrency** is deferred to Phase 5 by decision (0.7).

## What is left to the builder

- Grammar sign-off (the grammar is drafted in 0.1; it awaits Phase 1 validation).
- The region inference algorithm for Phase 2+ features. The Phase 2 rules in 0.3 §5 are **proposals** with open decisions (D4–D11); Phase 1 is specified in 0.3 §3.
- Standard library design.
- Package manager (post-MVP).
