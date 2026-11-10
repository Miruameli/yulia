# Yulia Language — Open Decisions Register

**Date:** 2026-11-10
**Status:** Living document — updated as decisions are made
**Owner:** spec owner (session B)

This register consolidates every open decision in the Phase 0 specifications, with an owner and the phase gate by which it must be decided. It is the single source of truth for "what is still open"; the per-spec "Open decisions" sections are pointers to it.

## How to use

- **Owner:** who drives the decision to closure.
- **Decide by:** the phase gate before which the decision must be made. A decision not made by its gate blocks that gate.
- **Status:** `open` or `resolved` (with the decision and date).

## Phase 1 gate — must decide before Phase 1 implementation

| # | Decision | Owner | Decide by | Status |
|---|---|---|---|---|
| D1 | Phase 1 subset boundary (exact type/construct set) | spec owner (B) | Phase 1 gate | revised 2026-11-10 (see Resolved) |
| D2 | Error and diagnostic model (messages, exit codes, abort behavior) | spec owner (B) | Phase 1 gate | resolved 2026-11-10 (see Resolved) |
| D3 | Region inference rules for the by-value subset | spec owner (B) | Phase 1 gate | resolved 2026-11-10 (see Resolved) |

## Phase 2 gate

| # | Decision | Owner | Decide by | Status |
|---|---|---|---|---|
| D4 | String ownership model (0.3 §5.1 Q3: copy / copy-on-write / region-owned). **Split discipline (D27):** plain immutable value types use copy semantics; move and copy coincide, so affine is vacuous for them. | spec owner (B) | Phase 2 gate | open |
| D5 | Growable array sharing rule (0.3 §5.2: invalidate / indirection / no sharing). **Split discipline (D27):** resource handles use move/affine; moving the handle is cheap and source invalidation is expected. | spec owner (B) | Phase 2 gate | open |
| D6 | C FFI pinning rule (0.3 §5.3) | spec owner (B) | Phase 2 gate | scoped 2026-11-10 — `pin` defined as a region-lifetime extension (0.3a §4.3); rule adoption remains at the Phase 2 gate |
| D7 | Recoverable errors: `Result` type shape (0.5 §2) | spec owner (B) | Phase 2 gate | open |
| D8 | Escape lattice coverage for growable collections (0.3a) | spec owner (B) | Phase 2 gate | open — lattice defined (0.3a §3); growable-collection coverage still open |
| D9 | E3 handling: reject vs R.2 promote (0.3a §8) | spec owner (B) | Phase 2 gate | scoped 2026-11-10 — E3 promotes to the global region; reject if promotion is impossible (0.3a §5) |
| D10 | `pin` representation in the analysis (0.3a §8) | spec owner (B) | Phase 2 gate | scoped 2026-11-10 — pin is a region-lifetime extension, not a lattice level (0.3a §4.3) |
| D11 | Escape analysis order vs type checking (0.3a §8) | spec owner (B) | Phase 2 gate | open |
| D27 | move/affine ownership assumption (0.7 §4.1) | spec owner (B) | Phase 2 gate | resolved 2026-11-10 (see Resolved) |

## Phase 4 gate

| # | Decision | Owner | Decide by | Status |
|---|---|---|---|---|
| D12 | Benchmark kernels K1–K5 implementation + baseline data (0.8) | spec owner (B) + infra (A) | Phase 4 gate | open |

## Phase 5 gate

| # | Decision | Owner | Decide by | Status |
|---|---|---|---|---|
| D13 | Concurrency model: threads (0.7) | spec owner (B) | Phase 5 gate | open |
| D14 | Async / await / coroutines (0.7 §3.2) | spec owner (B) | Phase 6 gate | open |
| D15 | SoA layout layer (0.6 §4.3) | spec owner (B) | Phase 5 gate | open |
| D16 | Generics (`CPtr<T>`, `Result<T, E>`) (0.1 §10.5) | spec owner (B) | Phase 5 gate | open |
| D17 | Soundness argument — pen-and-paper with explicit invariants (not machine-checked) | spec owner (B) + independent reviewer | Phase 5.4 | open |

## Post-MVP

| # | Decision | Owner | Decide by | Status |
|---|---|---|---|---|
| D18 | Standard library design | spec owner (B) | post-MVP | open |
| D19 | Package manager | spec owner (B) | post-MVP | open |

## Resolved

| # | Decision | Resolution | Date |
|---|---|---|---|
| D1 | Phase 1 subset boundary | IN: `int`, `bool`, structs, fixed-size arrays `[T; N]`, functions incl. `main`, `if`/`else`, `while`, `for`/`in`, `return`, `break`, `continue`, `let`/`const`. OUT: `float`, `char`, `string`, `panic()`, `extern "C"`, `export`, `pin`, `CPtr`, references, growable arrays, modules/imports, concurrency, generics, SoA, stdlib. Deliverables: lexer, parser, AST, type checker, region inference, diagnostics, tests, **plus incremental codegen** (LLVM/inkwell) for the subset — staged: 1a front-end, 1b codegen backend. Revised by user directive 2026-11-10 ("percepat codegen jadi phase 1, tapi bertahap pengembangannya"). | 2026-11-10 |
| D2 | Error and diagnostic model | Compile error -> stderr `file:line:col: error: <msg>`, exit 1. Runtime error (bounds check, div-by-zero, trapping overflow) -> stderr + `abort()` (SIGABRT/134), no region cleanup; **exercisable from Phase 1b (codegen)**. No `panic()` builtin in Phase 1. Stable error code per diagnostic. | 2026-11-10 |
| D3 | Region inference rules for the by-value subset | Resolved by 0.3 §3: single lexical pass, O(n), Lemma 1 (trivially sound). No new rules needed. | 2026-11-10 |
| D20 | Fallback policy (R.1 / R.2 / R.3) | R.1 restrict + R.2 warned escape hatch (user sign-off) | 2026-11-10 |
| D21 | Implementation language / backend | Rust + LLVM (inkwell enters in Phase 1b, codegen) | 2026-11-10 |
| D22 | License | MPL-2.0 | 2026-11-10 |
| D23 | R.2 acceptability | Acceptable as an explicit, warned escape hatch only | 2026-11-10 |
| D24 | Mechanical doc staleness (C++ → Rust, dangling plan refs) | Fixed in `docs/specs/` | 2026-11-10 |
| D25 | G1 claim integrity | R.1 restriction stated explicitly in the design doc | 2026-11-10 |
| D26 | Phase 3 / codegen phase naming (G8) | Resolved by user directive 2026-11-10: codegen is a Phase 1 deliverable (staged 1a front-end / 1b codegen). 'Phase 3' is not a phase; the plan is 1, 2, 4, 5, 6 with codegen inside Phase 1. | 2026-11-10 |
| D27 | move/affine ownership assumption (0.7 §4.1) | Resolved by user decision 2026-11-10: **split discipline** — copy semantics for plain immutable value types (`string` §5.1 (a)); move/affine for resource handles (growable arrays §5.2, `CPtr`, channels). 0.7 §4.1 corrected: affine is necessary but not sufficient (§4.2 supplies exclusivity); move/affine is a **design input**, soundness = Phase 5.4 (D17). 0.3 §5.4/§7 updated. | 2026-11-10 |
