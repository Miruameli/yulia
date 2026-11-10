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
| F18 | minor | 0.7 §4.1 | ownership transfer enforcement unspecified | ♻️ re-opened 2026-11-10 (D27) — move/affine assumption under review |
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
| F42 | high | 0.7 §4.1 | Ownership model direction | ♻️ re-opened 2026-11-10 (D27) — move/affine assumption under review |
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

## Version-pin verification (INV-2 / SOP-NET, retrieved 2026-11-10)

The Phase 1 plan (lost 2026-11-10; `docs/plans/2026-11-10-yulia-phase1.md`) originally pinned "LLVM 18" and "Rust 1.75+" without sources. Verified against primary sources:

| Claim | Verified value (2026-11-10) | Source |
|---|---|---|
| LLVM stable | 23.1.5 (Nov 2026); 24.0.0 in progress | https://releases.llvm.org/ ; https://discourse.llvm.org/t/llvm-23-1-5-released/92716 |
| Rust stable | 1.99.1 (2026-10-15); beta 1.100.0 (12 Nov 2026) | https://releases.rs/ ; https://doc.rust-lang.org/stable/releases.html |
| inkwell supported LLVM | LLVM 12–23 via `llvmM-N` feature flags (e.g. `llvm18-1`, `llvm23-1`) | https://lib.rs/crates/inkwell |

Action: plan lines 9 and 15 updated to LLVM 23.1.x / Rust 1.99.x with sources and retrieval date. LLVM 18 remains a valid inkwell-supported floor but is not current.
