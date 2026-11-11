# Yulia Language — Region Inference Specification (Draft 0.3)

**Date:** 2026-11-09
**Status:** Draft for review (Phase 0 deliverable 0.3)
**Depends on:** `docs/specs/2026-11-09-yulia-design.md`, `docs/specs/2026-11-09-yulia-design-review.md`
**Plan ref:** unavailable — `docs/plans/` was lost on 2026-11-10 (see decision log #82).

---

## 1. Purpose

This document specifies how Yulia assigns values to regions and argues that the assignment is sound. It is the make-or-break deliverable identified in the design review (§2 of the review).

---

## 2. Model

### 2.1 Regions

A **region** is a function-bound arena. A region is created on function entry and reclaimed on return. Reclamation is deterministic and O(1) (bump-pointer reset or freelist).

### 2.2 Values and references

Every value has a region. A reference (where the language has one) names a value in a region.

### 2.3 The inference problem

Given a program, assign each value to a region such that:

1. **Safety:** no reference outlives the region of its referent.
2. **Determinism:** reclamation happens at function return.
3. **Efficiency:** regions are as small as possible (slot reuse).

---

## 3. The Phase 1 subset is trivial — and that is the point

The Phase 1 subset (`int`, `bool`, structs, fixed-size arrays, functions, control flow) has **no pointers or references**. All values are passed by value. Therefore:

- Each local variable's region is its enclosing function's region.
- Each function call creates a fresh region for the callee's locals; it is reclaimed on return.
- Return values are copied into the caller's region.
- No reference can escape, because there are no references.

**Rule 3.1 (pass-by-value optimization).** The compiler MAY optimize pass-by-value to pass-by-reference only when it can prove the callee does not mutate the parameter, or that the caller does not observe the parameter after the call. Otherwise it MUST copy. Without this rule, an internal reference would break by-value semantics (e.g., a callee mutating a caller's struct). This proof is **interprocedural**; it is separate from the O(n) region assignment.

**Consequence:** region *assignment* for Phase 1 is a single lexical pass, O(n), and is trivially sound. Rule 3.1's mutation proof is a separate, interprocedural obligation. The hard part of the design is deferred to Phase 2.

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

**Deferred** to Phase 5 (0.7). The boundary: no shared regions across threads until the ownership-transfer model is defined. The model is a **design input** (0.7 §4.1: move/affine + no shared regions); the concurrency rule and its soundness are Phase 5.4 deliverables (D17), not settled lemmas.

---

## 6. Fallback policy (R.1/R.2/R.3)

The design review identified three options. This specification recommends:

- **R.1 (restrict):** enforce the rules in §5 at compile time. A program that violates them is rejected.
- **R.2 (fallback):** if a future feature cannot be made safe, promote the value to a global region (never reclaimed) and emit a warning. This is an **unbounded** leak (0.4 §3), documented and warned.
- **R.3 (novel algorithm):** not claimed.

**Decision:** R.1 for Phase 1–2; R.2 as an explicit, warned escape hatch for future features. No GC.

---

## 7. Soundness argument

**Lemma 1 (Phase 1).** At the language level, Phase 1 has no reference type (0.1 §7.3), so no reference to a value can be expressed outside the value's scope. Every value's region therefore contains its entire lifetime.

*Proof.* Phase 1 has no `&`, `*`, or reference types (0.1 §7.3). Values are passed by value; return values are copied into the caller's region (§3). No reference to a value can be formed, so none can outlive the value's region. ∎

**Lemma 2 (Phase 2, conditional).** *If* the §5 rules are decided (D4–D11) and enforced with R.1, then no reference outlives its referent's region.

*Proof sketch.* Under R.1, a program that cannot be proven safe is rejected; the remaining programs contain no escaping reference. R.2 is reserved for future features (0.4 §5) and is not invoked for Phase 2. ∎

**Theorem (memory safety).** Yulia programs in the safe subset do not access dangling pointers.

*Proof.* By Lemma 1 (Phase 1, unconditional) and Lemma 2 (Phase 2, conditional on D4–D11). ∎

**Scope.**
- **Phase 1: established.** No reference type exists, so no dangling pointer is expressible.
- **Phase 2: conditional** on the Phase 2 gate (D4–D11) and the chosen rules.
- **C FFI: a trust boundary outside the Theorem.** R-FFI-1 is a documented contract on C code, not a compiler-enforced rule. The compiler rejects passing a non-pinned pointer, but it cannot detect retention by C (0.3a §7).
- **Concurrency (Phase 5): deferred** (§5.4); its soundness is a Phase 5.4 deliverable (D17).

**Caveat:** the proof is a sketch. A formal proof is Phase 5 deliverable 5.4 (D17).

---

## 8. Complexity

| Phase | Complexity | Notes |
|---|---|---|
| Phase 1 | O(n) | Single lexical pass for region assignment |
| Phase 2 | O(n · k) | k = region promotions; escape analysis O(n · d), d = max value depth (0.3a §6) |
| Full | Open | Depends on the concurrency model |

---

## 9. Negative tests

| Invariant | Test | Phase | Expected |
|---|---|---|---|
| MEM-SAFE-1 | Return a reference to a region-local value | 2 (ungrammatical in 1) | Reject |
| MEM-SAFE-1 | Store a region pointer in a C global | 2 | Reject |
| MEM-SAFE-5 | Construct a cycle in a long-lived region | 2+ | Documented leak or reject |

---

## 10. Open decisions

See `docs/specs/2026-11-10-yulia-open-decisions.md` for owner + decision gate. Each item is marked **resolved** (with the deciding document) or **open** (with its register id).

1. String ownership model (Q3). — **open**, D4 (Phase 2 gate).
2. Growable array sharing rule. — **open**, D5 (Phase 2 gate).
3. C FFI pinning rule. — **scoped** 2026-11-10: `pin` is a region-lifetime extension (D6, 0.3a §4.3); rule adoption remains at the Phase 2 gate.
4. Concurrency model. — **open**, D13 (Phase 5 gate).
5. Whether R.2 (global-region fallback) is acceptable. — **resolved** 2026-11-10: user signed off on R.1 restrict + R.2 warned escape hatch (D20); R.2 acceptable as an explicit, warned escape hatch only.

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
| **Evidence** | Design review §2; Tofte & Talpin (1997); Cyclone (2004). |
| **Alternatives** | (a) Full inference for the whole language now; (b) trivial Phase 1 + explicit Phase 2 rules; (c) GC fallback. |
| **Chosen** | (b). |
| **Risks** | Phase 2 rules may prove insufficient; the soundness argument is a sketch. |
| **Mitigations** | Negative tests; formal proof in Phase 5.4; R.2 fallback. |
| **Confidence** | High on Phase 1; medium on Phase 2 rules (conditional on D4–D11). |
| **Kill-criteria** | If a Phase 2 rule cannot be made sound, escalate before implementation. |
| **Verification plan** | Peer review; negative tests; formal proof (Phase 5.4). |
