# Yulia Phase 0 — Review Findings Log

**Date:** 2026-11-10
**Scope:** Phase 0 deliverables 0.1–0.9 (+ 0.3a)
**Reviewers:** A (0.5–0.9, 0.1 extensions), B (0.2–0.4, 0.7 additional)

This log persists review findings that previously existed only in coord messages, so deferred work survives context loss.

## A's review of 0.5–0.9 (F12–F24)

| ID | Severity | File | Finding | Status |
|---|---|---|---|---|
| F12 | minor | 0.5 §4.2/§4.4 | Region cleanup on panic vs Phase 1 abort inconsistency | ✅ addressed (0.5 §4.2 Phase 1 note) |
| F13 | minor | 0.5 §4.4 | setjmp/longjmp at FFI is Phase 2+ | ✅ addressed (0.5 §4.4 Phase 1 note) |
| F14 | moderate | 0.5 §8 | Growable collection in outer region needs a rule | ✅ addressed (0.5 §8 outer-region rule) |
| F15 | minor | 0.6 §4.4 | string → const char* buffer ownership | ✅ addressed (0.6 §4.4 buffer ownership) |
| F16 | minor | 0.6 §4.4 | int width cross-ref to 0.2 | ✅ addressed (0.6 §4.4) |
| F17 | minor | 0.6 §3.1 | panic-at-FFI cross-ref to 0.5 §4.4 | ✅ addressed (0.6 §3.1) |
| F18 | minor | 0.7 §4.1 | ownership transfer enforcement unspecified | ✅ resolved 2026-11-10 (D27) — split discipline; affine necessary but not sufficient (§4.2 supplies exclusivity) |
| F19 | minor | 0.7 §4.2 | immutable sharing region outliving threads | ✅ addressed (0.7 §4.2 Phase 5 requirement) |
| F20 | minor | 0.8 §4.1 | -march=native harms reproducibility | ✅ addressed (0.8 §4.1 fixed arch) |
| F21 | minor | 0.8 §3.2 | K2 linked list needs pointers/indices not in Phase 1 | ✅ addressed (0.8 §3.2 phase note) |
| F22 | minor | 0.8 §3.5 | K5 '2x faster' claim too strong | ✅ addressed (0.8 §3.5 softened to 1.5x + risk note) |
| F23 | minor | 0.9 §4.1 | 'whichever is longer' ambiguous | ✅ addressed (0.9 §4.1 clarified) |
| F24 | minor | 0.9 §3.2 | keyword reservation strategy | ✅ addressed (0.9 §3.3) |

## B's additional review of 0.7 (F40–F54)

| ID | Severity | File | Finding | Status |
|---|---|---|---|---|
| F40 | blocker | 0.7 §3.3/§6 | Grammar had no concurrency keywords → diagnostic unreachable | ✅ resolved (0.1 §2.5) |
| F41 | high | 0.7 §1 | Leaked coordination-channel reference in §1 syntax note | ✅ resolved |
| F42 | high | 0.7 §4.1 | Ownership model direction | ✅ resolved 2026-11-10 (D27) — split discipline (copy for value types, move/affine for handles); soundness = Phase 5.4 (D17) |
| F43 | high | 0.7 §4.2 | Immutable sharing lifetime | ✅ resolved |
| F44 | high | 0.7 §4.4 | Panic semantics incomplete | ✅ resolved |
| F45 | high | 0.7 §5 | MEM-SAFE-8 deadlock claim too strong | ✅ resolved |

| F46 | high | 0.7 §6 | Negative tests lack program forms | ✅ resolved |
| F47 | medium | 0.7 §3.1 | 'LIFO' misleading | ✅ resolved |
| F48 | medium | 0.7 §4.3 | Thread-local state model | ✅ resolved |
| F49 | medium | 0.7 §3.2 | async vs threads | ✅ resolved (async → Phase 6) |
| F50 | medium | 0.7 §4 | FFI + concurrency | ✅ resolved (§4.5) |
| F51 | medium | 0.7 §7/§8 | kill-criteria timing | ✅ resolved (Phase 4 feasibility study) |
| F52 | low | 0.7 §1 | title vs decision placement | ✅ resolved |
| F53 | low | 0.7 §5 | invariants as fact | ✅ resolved |
| F54 | low | 0.7 §8 | Rust prior art incorrect | ✅ resolved |

## Open items

- **F55+ (deeper grammar review):** explicit **no findings** (B, 2026-11-10).
- **Phase 1 language/backend:** ✅ RESOLVED by user decision (2026-11-10): **Rust + LLVM backend** (inkwell enters in Phase 1b, codegen). License: **MPL-2.0**. Version pins verified 2026-11-10 (LLVM 23.1.x / Rust 1.99.x) — see Version-pin verification below. The Phase 1 plan document itself was lost on 2026-11-10.
- **G7 (four-way Phase 1 subset/codegen conflict):** ✅ RESOLVED 2026-11-10 by D1 — Phase 1 = `int`/`bool`/structs/fixed-size arrays/functions/control flow, no `float`. **Superseded by user directive 2026-11-10: codegen is now a Phase 1 deliverable (staged 1a front-end / 1b codegen); K1 stays Phase 4 (it is `float`, Phase 2+).** See `2026-11-10-yulia-open-decisions.md` D1 (revised).
- **G8 (Phase 3 / codegen phase naming):** ✅ RESOLVED 2026-11-10 by user directive — codegen lands in Phase 1 (staged 1a/1b). 'Phase 3' is not a phase; the plan is 1, 2, 4, 5, 6 with codegen inside Phase 1. Recorded as **D26** (resolved).
- **F56 (0.5 §2 runtime-error taxonomy):** the Phase 1 runtime-error row listed "assertion failure", but no Phase 1 construct produces one (0.1 §2.5 keywords, §3.3 statements and §10 Phase 2 extensions have no `assert`; D1's IN/OUT lists and D2 omit it). ✅ addressed 2026-11-11 (A) — "assertion failure" removed from `0.5:21`.
- **F57 (0.8 §2.2 target summary):** §2.2 summarised the targets as "within 10% of C on K1, K2, K4, and K5, and within 15% on K3 (§3.3)", which matched neither the per-kernel sections (§3.1 K1 within 5%; §3.4 K4 at least 2× faster than C; §3.5 K5 at least 1.5× faster than rustc) nor `benchmarks/README.md`'s kernel table, and the "(§3.3)" reference pointed at K3 alone. ✅ addressed 2026-11-11 (A) — §2.2 restated to match §3.1–§3.5. ✅ closed 2026-11-11 (A) — user delegated ('terserah'); §2.2 keeps 10% as the BINDING requirement for K1, §3.1's 5% is the STRETCH expectation, and §3.1 now cross-references §2.2.
- **F58 (0.1 §3.3/§3.4 condition disambiguation):** `if a { }` is ambiguous — `a { }` could read as a struct literal, leaving the `if` without a body. ✅ resolved 2026-11-11 (B's parser implementation, A's spec note) — struct-literal parsing is disabled in `if`/`while` conditions and `for` range bounds, so a `{` there begins the branch/loop body; a struct literal in one of those positions must be parenthesized. Recorded in 0.1 §3.3 ("Note on condition disambiguation").
- **F59 (0.1 §8 open questions):** the section listed seven questions as open, but three were already decided — array bounds checking (0.2 §113: runtime check in Phase 1, compile-time where provable is a Phase 2 optimization), integer overflow (D2: trapping), and default initialization (MEM-SAFE-4: reading a `let` before assignment is a compile error). ✅ resolved 2026-11-11 (A) — each item is now marked **resolved** (with the deciding document) or **open** (with its register id: D4, D11, D18).
- **F60 (0.2 §7 / 0.3 §10 / 0.3a §8 open decisions):** the per-spec "Open decisions" sections listed items without status, several of which were already decided or scoped — 0.2 §7.1 panic/unwind (resolved by 0.5 §4), 0.3 §10.5 R.2 acceptability (resolved, D20), 0.3a §8.1 E3 handling (scoped, D9), 0.3a §8.2 `pin` representation (scoped, D10). ✅ resolved 2026-11-11 (A) — each item is now marked **resolved/scoped** (with the deciding document) or **open** (with its register id), matching the 0.1 §8 convention (F59).
- **F61 (0.2 §6.2 typing rules):** the section listed only "selected" rules (9 rules), leaving the type checker under-specified for implementation — the arithmetic/comparison/logical/unary operators, assignment, the array repeat form, and every statement/declaration form were missing. ✅ resolved 2026-11-11 (A) — the rule set is now exhaustive for the Phase 1 subset (0.1 §3.2 types, §3.3 statements, §3.4 operators): all binary/unary operators, assignment, `[e; N]`, and the statement/declaration forms (`let`/`const`, `if`/`while`, both `for` forms, `return`, `break`/`continue`).
- **F62 (diagnostic code catalog):** D2 promised a "stable error code per diagnostic" and the format `file:line:col: error: <msg>`, but no spec enumerated the codes — `design.md:82` said the naming rule was "pending, 0.4 §6" and `0.4 §9` carried an unchecked "Diagnostic naming rule agreed" box. ✅ resolved 2026-11-11 (A) — 0.5 §2.1 defines the format (`error[<code>]` / `runtime error[<code>]`; exit 1 / abort 134) and 0.5 §2.2 is the Phase 1 catalog (E0101–E0603, R0101–R0103); `design.md:82` and `0.4 §9` now point at 0.5 §2.1–2.2, and D2's row records the catalog.

## Version-pin verification (INV-2 / SOP-NET, retrieved 2026-11-10)

The Phase 1 plan (lost 2026-11-10; `docs/plans/2026-11-10-yulia-phase1.md`) originally pinned "LLVM 18" and "Rust 1.75+" without sources. Verified against primary sources:

| Claim | Verified value (2026-11-10) | Source |
|---|---|---|
| LLVM stable | 23.1.5 (Nov 2026); 24.0.0 in progress | https://releases.llvm.org/ ; https://discourse.llvm.org/t/llvm-23-1-5-released/92716 |
| Rust stable | 1.99.1 (2026-10-15); beta 1.100.0 (12 Nov 2026) | https://releases.rs/ ; https://doc.rust-lang.org/stable/releases.html |
| inkwell supported LLVM | LLVM 12–23 via `llvmM-N` feature flags (e.g. `llvm18-1`, `llvm23-1`) | https://lib.rs/crates/inkwell |

Action: plan lines 9 and 15 updated to LLVM 23.1.x / Rust 1.99.x with sources and retrieval date. LLVM 18 remains a valid inkwell-supported floor but is not current.

## Soundness audit (2026-11-10, session A)

Independent audit of the memory-safety argument across 0.2/0.3/0.3a/0.4/0.5/0.6/0.7/0.8. **Verdict: the Phase 1 half holds; the Phase 2 half did not hold as written.** All findings are now fixed or explicitly scoped.

**Blockers (6).**

| ID | File | Finding | Resolution |
|---|---|---|---|
| B1 | 0.3 §7 | Theorem claims Phase 1–2; Lemma 2's premise undecided | Theorem scoped: Phase 1 unconditional, Phase 2 conditional on D4–D11 |
| B2 | 0.3 §7 / 0.4 §5 | Lemma 2 invokes R.2, which 0.4 excludes from Phase 1–2 | Lemma 2 uses R.1 only |
| B3 | 0.3a §7 / 0.6 §3.1 | R-FFI-1 unenforceable (C retention undetectable) | C FFI declared a trust boundary outside the Theorem |
| B4 | 0.2 §4.4 | SOS never reclaims (Return does not pop) | Return pops the region and invalidates its locations |
| B5 | 0.2 §6 | Progress false (out-of-bounds index) | runtime-error rule added; Progress restated |
| B6 | 0.2 §6 | Preservation unprovable (no type system) | minimal type system added |

**High (14).** B7–B20: escape-lattice contradictions (E1 direction, C FFI levels, E3 vs pin, region assignment rule), region granularity (scope vs function), Lemma 1 vs Rule 3.1, O(n) vs the interprocedural Rule 3.1 proof, alias analysis, 0.6:120 cross-ref, 0.5 "borrow checking", 0.7 lifetime-annotation exception, 0.7 exclusivity vs immutable sharing, 0.5 atomicity/rollback. All fixed.

**Medium/Low (17).** B21–B37: panic propagation scope, pin example syntax, array ABI, CPtr leak, benchmark targets, negative-test phase labels, 0.4 contradictions, Val domain, confidence, complexity, reclamation, SoA Phase 2, Rust prior art, illustrative syntax, `-march`. All fixed.

**Net effect on the claim.** The design's core claim is unchanged in substance but is now honestly bounded: *compiler-inferred region-based memory for a decidable safe subset* — Phase 1 established; Phase 2 conditional on the Phase 2 gate (D4–D11); C FFI a trust boundary; concurrency Phase 5.

**Residual fixes (2026-11-10, session A, follow-up pass).** Two cross-reference defects survived the first pass and were fixed after a later review: (a) 0.2 §4.1 cited 0.6 §4.4 for the `int` width while 0.6 §4.4 cited 0.2 §4.1 — 0.2 now owns the definition and the back-pointer is dropped; (b) 0.6 §7 said the SoA annotation is "accepted in Phase 2 but no-op", contradicting 0.6 §4.3 ("rejected in Phase 2") — §7 now matches §4.3.
