# Yulia Language — Escape Analysis Specification (Draft 0.3a)

**Date:** 2026-11-09
**Status:** Draft for review (Phase 0 deliverable 0.3a)
**Depends on:** `docs/specs/2026-11-09-yulia-region-inference.md` (0.3)
**Plan ref:** `docs/plans/2026-11-09-yulia-implementation-plan.md` §2, deliverable 0.3a

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
| **E0** | no-escape | The value never leaves its region. Stack-allocate; reclaim at scope exit. |
| **E1** | argument-escape | The value is passed to a function but not returned or stored. The callee's region must outlive the call. |
| **E2** | return-escape | The value is returned from a function. The caller's region must outlive the callee's. |
| **E3** | global-escape | The value is stored in a global, a C global, or a long-lived region. Promote to a global region. |

The levels form a lattice: E0 < E1 < E2 < E3. The analysis computes the least upper bound.

---

## 4. Analysis rules

### 4.1 Phase 1 (no references)

Trivially E0 for all values. No escape analysis is needed. This is why Phase 1 region inference is trivial (0.3 §3).

### 4.2 Phase 2

- **Function call:** if a value is passed as an argument, its level is at least E1.
- **Return:** if a value is returned, its level is at least E2.
- **Global store:** if a value is stored in a global, its level is E3.
- **C FFI:** if a value's address is passed to C, its level is E3 (unless pinned; see §4.3).
- **Transitivity:** if a value is stored in another value, the container's level is the max of its own and the stored value's level.

### 4.3 Pinning

A `pin` construct (0.3 §5.3) marks a value as intentionally long-lived. A pinned value is E3 by declaration, and the compiler does not reject its escape. The `pin` region is reclaimed when the pin goes out of scope. **Clarification:** pinning keeps the region alive for the duration of the C call (or the pin's scope), not indefinitely. A pin that outlives its scope is a compile error, not a leak.

---

## 5. Integration with region inference

1. Run escape analysis.
2. For each value, assign the smallest region that contains its escape level.
3. If a value's escape level exceeds its lexical region, promote it.
4. If promotion is impossible (e.g., E3 in a stack region), reject the program (R.1) or promote to the global region (R.2).

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
| Return a local reference | Reject (R.1) or promote (R.2) |
| Store a local in a global | Reject or promote |
| Pass a local to C and retain | Reject (R-FFI-1) |
| Pass a local to C without retaining | Accept (E1) — the value escapes as an argument but the region is reclaimed after the call returns. Sound only if C does not retain the pointer. |
| Pin a local and pass to C | Accept (E3 by declaration) |
---

## 8. Open decisions

1. Is E3 always a rejection, or is the global-region fallback (R.2) acceptable?
2. How is `pin` represented in the analysis?
3. Does escape analysis run before or after type checking?

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
