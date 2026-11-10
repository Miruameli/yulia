# Yulia Language — Design Document

**Date:** 2026-11-09
**Status:** Draft for review

## Intent

Yulia is a procedural programming language with explicit types and a custom modern syntax. Its purpose is to establish a **new paradigm for memory and performance**: memory that is automatic, deterministic, and invisible to the programmer, while delivering C-class performance and Python-class simplicity.

The language takes sophisticated ideas that already exist in other languages — region-based memory, dense data layout, compile-time inference — and makes them simpler. No lifetime annotations, no manual allocation, no garbage collector.

**One-sentence technical claim:** *Compiler-inferred region-based memory management with dense packing and slot reuse — no `malloc`/`free`, no GC, no lifetime annotations, deterministic reclamation.*

## Goals

- **Extreme performance** — C-class speed; no runtime overhead for memory management.
- **Extreme memory efficiency** — dense packing, slot reuse, no per-object metadata or headers.
- **Simplicity** — Python-like ergonomics; the programmer never manages memory.
- **Procedural paradigm** — functions, structs, modules; clear step-by-step control flow.
- **Explicit types** — the programmer writes types; no hidden inference.
- **Custom modern syntax** — designed for readability and simplicity, not copied from an existing language.
- **Full ecosystem** — compiler, LSP, formatter, build tooling.

## Non-goals

- Object-oriented programming (classes, inheritance, virtual dispatch).
- Dynamic typing / runtime type checking.
- Stop-the-world garbage collection.
- Rust-style lifetime annotations or a borrow checker.
- C++-level complexity and footguns.

## Anti-goals

- Becoming C++: decades of baggage, complexity, undefined behavior.
- Becoming Python: object overhead, GC pauses, interpreter speed.
- Exposing memory management to the programmer in any form.

## Constraints

- **Implementation language:** Rust.
- **Compilation target:** native machine code via LLVM IR.
- **Memory management:** deterministic; no GC.
- **Runtime:** no heavyweight runtime; libc only.
- **License:** MPL-2.0.

## Approaches considered

| Approach | Description | Trade-off |
|---|---|---|
| A. Static memory layout | Every value assigned a slot at compile time; slots reused via liveness analysis | Most extreme performance and efficiency; dynamic data structures become very hard |
| **B. Region-based memory *(chosen)*** | Memory managed in compiler-inferred regions; a region is reclaimed when it goes out of scope | Flexible enough for dynamic data; no GC; deterministic; compiler is more complex |
| C. Bytecode VM | Custom VM carrying the new memory model | Easiest to iterate and extend; performance not extreme |

**Chosen: B.** It matches the vision — no visible lifecycle, slot reuse, automatic management — while staying flexible enough for real programs. Approach A's static layout can be layered on later as an optimization for values the compiler can prove are short-lived.

## Memory model

The programmer's model: memory is a flat, ever-present substrate. Values are written densely, and slots are reused automatically. There is no allocation or deallocation step visible in the language.

Mechanically:

- **Region-based.** Memory is allocated in *regions*. A region is a scope-bound arena; its contents are reclaimed together when the region exits.
- **Compiler-inferred.** The compiler decides where each value lives and which region it belongs to. The programmer writes no annotations.
- **Dense packing.** Values are laid out compactly: no implicit padding, no per-object headers, struct-of-arrays where it helps cache locality.
- **Slot reuse.** Memory slots are recycled; there is no `malloc`/`free` churn at runtime.
- **Deterministic.** No GC pauses. Reclamation happens at region exit, at a point the programmer can reason about.

### Metaphor → mechanism

| Metaphor | Mechanism |
|---|---|
| The standard book | The process address space / region heap |
| A neatly torn sheet | A region (allocation unit) |
| Writing with very small letters | Dense packing, SoA, bit-level layout control |
| Lamp A replaced by lamp B | Slot reuse / freelist recycling, not repeated alloc+free |
| Everything managed from behind | Compiler-managed lifetime and ownership; invisible to the programmer |

## Restrictions and fallback

The memory-safety guarantee holds for a **decidable subset** of the language. Full region inference for arbitrary imperative programs is not generally decidable (see the design review §2); the language therefore bounds the claim and makes the boundary explicit.

- **R.1 (restrict).** A program that the compiler cannot prove safe is **rejected at compile time**. The diagnostic names the specific rule violated and, where applicable, points to the R.2 escape hatch. The language's real semantics is this safe subset, not the full grammar.
- **R.2 (warned escape hatch).** When a future feature cannot be made safe, the value is promoted to a **global region** (never reclaimed) and the compiler emits a **warning**. This is a documented, bounded leak — not a garbage collector, and not a lifetime annotation. It is an explicit opt-in deviation, never silent.
- **No stronger inference is claimed.** A novel inference algorithm that removes the escape hatch is not asserted. The claim "compiler-inferred region-based memory, no lifetime annotations" holds within the safe subset; R.2 is a warned deviation from it, not an annotation.

See `docs/specs/2026-11-09-yulia-region-inference.md` (0.3 §6), `docs/specs/2026-11-09-yulia-escape-analysis.md` (0.3a), and `docs/specs/2026-11-09-yulia-fallback-policy.md` (0.4).

## Syntax

- Custom modern syntax, designed from scratch for simplicity.
- Explicit types on all declarations.
- Procedural constructs: functions, structs, modules.
- Concrete grammar is specified in `docs/specs/2026-11-09-yulia-grammar-draft.md` (0.1).

## Toolchain

- **Compiler:** Rust → LLVM IR → native code (LLVM via inkwell; inkwell enters at the codegen phase).
- **LSP:** built from the start; a custom syntax needs tooling to be usable.
- **Formatter:** built from the start, driven by the same grammar.
- **Build system:** Cargo; LLVM via inkwell.

## MVP feature set

This is the **whole-MVP** feature set, not the Phase 1 subset. Phase 1 is specified in 0.3 §3 and delivers `int`, `bool`, structs, fixed-size arrays, functions, and control flow only — no `float`, no `char`, no `string`, no codegen.

- Primitive types: `int`, `float`, `bool`, `char`, `string`
- Functions / procedures
- Structs
- Arrays
- Modules
- Explicit control flow: `if`, `while`, `for`
- No classes, no GC, no lifetime annotations

## Risks

- **Custom syntax** raises adoption and tooling cost; the LSP and formatter must land early.
- **Automatic memory + extreme performance** is a strong claim; the region model must be genuinely deterministic, or the claim becomes a slogan.
- **Explicit types** may hurt ergonomics compared to Python; the syntax must keep declarations light.
- **Concurrency and C FFI** must be designed early, not retrofitted.

## What is left to the builder

- Exact grammar and lexical rules.
- The region inference algorithm for Phase 2+ features (Phase 1 is specified in 0.3 §3).
- Standard library design.
- Package manager (post-MVP).
