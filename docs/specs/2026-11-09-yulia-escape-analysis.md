# Yulia Language — Escape Analysis Specification (Draft 0.3a)

**Date:** 2026-11-09
**Status:** Draft for review (Phase 0 deliverable 0.3a)
**Depends on:** `docs/specs/2026-11-09-yulia-region-inference.md` (0.3)
**Plan ref:** unavailable — `docs/plans/` was lost on 2026-11-10 (see decision log #82).

---

## 1. Purpose

This document specifies escape analysis for Yulia: how the compiler decides whether a value can escape its region, and what happens when it does. It is a companion to the region-inference specification (0.3).

---

## 2. Why escape analysis is needed

Region inference (0.3) assigns each value to a region. The assignment is sound only if no reference outlives its region. Escape analysis decides whether a value's reference can leave its region.

---

## 3. The escape lattice

For each value, the analysis computes an escape level:

| Level | Name | Meaning |
|---|---|---|
| **E0** | no-escape | The value never leaves its region. Reclaimed at function return. |
| **E1** | argument-escape | The value is passed to a function. The **caller's** region must outlive the call; lexical nesting satisfies this. |
| **E2** | return-escape | The value is returned from a function. The caller's region must outlive the callee's; lexical nesting satisfies this. |
| **E3** | global-escape | The value is stored in a global or a long-lived region. Promoted to a global region (never reclaimed). |

The levels form a lattice: E0 < E1 < E2 < E3. The analysis computes the least upper bound.

A `pin` is **not** a lattice level: it is a region-lifetime extension (§4.3).

---

## 4. Analysis rules

### 4.1 Phase 1 (no references)

Trivially E0 for all values. No escape analysis is needed. This is why Phase 1 region inference is trivial (0.3 §3).

### 4.2 Phase 2

- **Function call:** if a value is passed as an argument, its level is at least E1.
- **Return:** if a value is returned, its level is at least E2.
- **Global store:** if a value is stored in a global, its level is E3.
- **C FFI:** if a value's address is passed to C, the value must be pinned (§4.3). A pinned value's region is extended to the pin's scope. Passing an unpinned pointer is rejected (R.1).
- **Transitivity:** if a value is stored in another value, the container's level is the max of its own and the stored value's level.

### 4.3 Pinning

A `pin` construct (0.3 §5.3) **extends a value's region to the pin's scope**. A pinned value is not promoted to the global region; its region is reclaimed when the pin goes out of scope. A pin that outlives its scope is a compile error, not a leak. Pinning is what makes the C FFI rule (§4.2) sound: the pin's scope encloses the call, so the value's region outlives it.

---

## 5. Integration with region inference

1. Run escape analysis.
2. Allocate each value in its enclosing function's region.
3. E1 and E2 are satisfied by lexical nesting: the caller's region outlives the callee's.
4. E3 requires promotion to the global region (never reclaimed).
5. If an E3 value cannot be promoted, reject the program (R.1).

---

## 6. Complexity

| Phase | Complexity | Notes |
|---|---|---|
| Phase 1 | O(1) | No analysis |
| Phase 2 | O(n · d) | d = maximum value depth (for transitivity) |

---

## 7. Negative tests

| Test | Expected |
|---|---|
| Return a local reference | Reject (R.1) — ungrammatical in Phase 1 |
| Store a local in a global | Reject or promote (E3) |
| Pass an unpinned local to C | Reject (R.1) |
| Pass a pinned local to C | Accept — the pin's scope encloses the call |
| Retain a pinned pointer beyond the pin's scope | Reject (R-FFI-2) |
---

## 8. Open decisions

See `docs/specs/2026-11-10-yulia-open-decisions.md` for owner + decision gate. Each item is marked **resolved/scoped** (with the deciding document) or **open** (with its register id).

1. Is E3 always a rejection, or is the global-region fallback (R.2) acceptable? — **scoped** 2026-11-10: E3 promotes to the global region; reject if promotion is impossible (D9, 0.3a §5).
2. How is `pin` represented in the analysis? — **scoped** 2026-11-10: `pin` is a region-lifetime extension, not a lattice level (D10, 0.3a §4.3).
3. Does escape analysis run before or after type checking? — **open**, D11 (Phase 2 gate).

---

## 9. Acceptance

- [x] Escape lattice defined.
- [x] Analysis rules specified.
- [x] Integration with region inference specified.
- [x] Complexity stated.
- [x] Negative tests specified.
- [ ] Formal proof (Phase 5.4).

---

## 10. Deliberation (SOP-THINK)

| Field | Content |
|---|---|
| **Goal** | Specify escape analysis and its integration with region inference. |
| **Assumptions** | The escape lattice E0–E3 is sufficient; `pin` is the only explicit escape hatch. |
| **Evidence** | 0.3 §4–5; standard escape-analysis literature (Choi et al. 1999; Bladen et al. 2020). |
| **Alternatives** | (a) No escape analysis (rely on GC); (b) full inference with escape analysis; (c) explicit annotations. |
| **Chosen** | (b) for Phase 2, with (a) trivial for Phase 1. |
| **Risks** | The lattice may be insufficient for growable collections; transitivity may be expensive. |
| **Mitigations** | Negative tests; complexity bound; formal proof in Phase 5.4. |
| **Confidence** | Medium. |
| **Kill-criteria** | If the lattice cannot express a required escape pattern, escalate before implementation. |
| **Verification plan** | Peer review; negative tests; formal proof (Phase 5.4). |
