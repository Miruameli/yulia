# Yulia Language — Region Inference Specification (Draft 0.3)

**Date:** 2026-11-09
**Status:** Draft for review (Phase 0 deliverable 0.3)
**Depends on:** `docs/specs/2026-11-09-yulia-design.md`, `docs/specs/2026-11-09-yulia-design-review.md`
**Plan ref:** `docs/plans/2026-11-09-yulia-implementation-plan.md` §2, deliverable 0.3

---

## 1. Purpose

This document specifies how Yulia assigns values to regions and argues that the assignment is sound. It is the make-or-break deliverable identified in the design review (§2 of the review).

---

## 2. Model

### 2.1 Regions

A **region** is a scope-bound arena. A region is created on entry to a scope and reclaimed on exit. Reclamation is deterministic and O(1) (bump-pointer reset or freelist).

### 2.2 Values and references

Every value has a region. A reference (where the language has one) names a value in a region.

### 2.3 The inference problem

Given a program, assign each value to a region such that:

1. **Safety:** no reference outlives the region of its referent.
2. **Determinism:** reclamation happens at scope exit.
3. **Efficiency:** regions are as small as possible (slot reuse).

---

## 3. The Phase 1 subset is trivial — and that is the point

The Phase 1 subset (`int`, structs, fixed-size arrays, functions) has **no pointers or references**. All values are passed by value. Therefore:

- Each local variable's region is its enclosing function's region.
- Each function call creates a fresh region for the callee's locals; it is reclaimed on return.
- Return values are copied into the caller's region.
- No reference can escape, because there are no references.

**Rule 3.1 (pass-by-value optimization).** The compiler MAY optimize pass-by-value to pass-by-reference only when it can prove the callee does not mutate the parameter, or that the caller does not observe the parameter after the call. Otherwise it MUST copy. Without this rule, an internal reference would break by-value semantics (e.g., a callee mutating a caller's struct).

**Consequence:** region inference for Phase 1 is a single lexical pass, O(n), and is trivially sound. The hard part of the design is deferred to Phase 2.

This is a deliberate sequencing decision: prove the toolchain on a subset where the memory model is trivial, then extend the model carefully.

---

## 4. Where the difficulty actually lives

The design's difficulty is concentrated in Phase 2 features:

| Feature | Why it breaks the trivial model |
|---|---|
| `string` | Needs ownership; is it copied, borrowed, or interned? |
| Growable arrays | Reallocation moves storage; references into the old storage dangle. |
| C FFI | C pointers can outlive a region. |
| Concurrency | Regions shared across threads need ownership transfer. |

Each of these requires an explicit rule. The rules are the real content of this specification.

---

## 5. Proposed rules for Phase 2

### 5.1 Strings

**Decision needed (open question Q3).** Options:

- (a) Immutable value type, copied on assignment. Simple, safe, but O(n) copies.
- (b) Copy-on-write with reference counting. Needs RC, which is a runtime cost.
- (c) Region-owned mutable string. Fast, but growth reallocation is a problem.

**Recommendation:** (a) for MVP-1, (c) for MVP-2 after the reallocation rule is defined.

### 5.2 Growable arrays

**Rule:** a growable array is a handle (pointer to a region-allocated buffer). Growth reallocates the buffer; existing handles are updated or invalidated. The language MUST choose:

- (a) Handles are invalidated on growth (programmer must re-acquire). **Poor UX; consider removing or restricting to internal buffers only.**
- (b) Handles are indices into a stable indirection table.
- (c) Growth is forbidden after the array is shared.

**Recommendation:** (c) for MVP-1 (no sharing of growable arrays), then (b).

### 5.3 C FFI

**Rule:** a C function may not retain a pointer to a Yulia region value beyond the call. Enforced by:

- (a) A linear/affine type for "borrowed C pointers".
- (b) An explicit `pin` construct that keeps a region alive.
- (c) Documentation only (unsafe).

**Recommendation:** (b) `pin`, with (a) as a later refinement.

### 5.4 Concurrency

**Deferred** to Phase 5 (plan §8). The boundary: no shared regions across threads until the ownership-transfer model is defined.

---

## 6. Fallback policy (R.1/R.2/R.3)

The design review identified three options. This specification recommends:

- **R.1 (restrict):** enforce the rules in §5 at compile time. A program that violates them is rejected.
- **R.2 (fallback):** if a future feature cannot be made safe, promote the value to a global region (never reclaimed) and emit a warning. This is a leak, bounded by program semantics, and documented.
- **R.3 (novel algorithm):** not claimed.

**Decision:** R.1 for Phase 1–2; R.2 as an explicit, warned escape hatch for future features. No GC.

---

## 7. Soundness argument

**Lemma 1 (Phase 1).** Under by-value semantics with no references, every value's region contains its entire lifetime.

*Proof.* A value is created in its lexical scope and destroyed at scope exit. No reference to it exists outside the scope. ∎

**Lemma 2 (Phase 2, under §5 rules).** Under the string/array/FFI rules, no reference outlives its referent's region.

*Proof sketch.* Each rule either forbids the escaping reference (R.1) or promotes the value to a region that outlives the reference (R.2). ∎

**Theorem (memory safety).** Yulia programs do not access dangling pointers.

*Proof.* By Lemma 1 (Phase 1) and Lemma 2 (Phase 2). ∎

**Caveat:** the proof is a sketch. A formal proof is Phase 5 deliverable 5.4.

---

## 8. Complexity

| Phase | Complexity | Notes |
|---|---|---|
| Phase 1 | O(n) | Single lexical pass |
| Phase 2 | O(n · k) | k = number of region promotions |
| Full | Open | Depends on the concurrency model |

---

## 9. Negative tests

| Invariant | Test | Expected |
|---|---|---|
| MEM-SAFE-1 | Return a reference to a region-local value | Reject |
| MEM-SAFE-1 | Store a region pointer in a C global | Reject |
| MEM-SAFE-5 | Construct a cycle in a long-lived region | Documented leak or reject |

---

## 10. Open decisions

1. String ownership model (Q3).
2. Growable array sharing rule.
3. C FFI pinning rule.
4. Concurrency model.
5. Whether R.2 (global-region fallback) is acceptable. (Resolved by 0.4: user signed off on R.1 restrict + R.2 warned escape hatch on 2026-11-10. R.2 acceptable as explicit, warned escape hatch only.)

---

## 11. Acceptance

- [x] Region model defined.
- [x] Phase 1 inference specified and shown trivial.
- [x] Phase 2 rules proposed.
- [x] Fallback policy recommended.
- [x] Soundness argument sketched.
- [x] Complexity stated.
- [x] Negative tests specified.
- [ ] Formal proof (Phase 5.4).

---

## 12. Deliberation (SOP-THINK)

| Field | Content |
|---|---|
| **Goal** | Specify region inference and argue soundness. |
| **Assumptions** | The Phase 1 subset is by-value with no references; Phase 2 adds strings, growable arrays, FFI. |
| **Evidence** | Design review §2; plan §2; Tofte & Talpin (1997); Cyclone (2004). |
| **Alternatives** | (a) Full inference for the whole language now; (b) trivial Phase 1 + explicit Phase 2 rules; (c) GC fallback. |
| **Chosen** | (b). |
| **Risks** | Phase 2 rules may prove insufficient; the soundness argument is a sketch. |
| **Mitigations** | Negative tests; formal proof in Phase 5.4; R.2 fallback. |
| **Confidence** | High on Phase 1; medium on Phase 2 rules. |
| **Kill-criteria** | If a Phase 2 rule cannot be made sound, escalate before implementation. |
| **Verification plan** | Peer review; negative tests; formal proof (Phase 5.4). |
