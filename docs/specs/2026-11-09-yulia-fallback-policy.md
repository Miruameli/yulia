# Yulia Language — Fallback Policy Decision (Draft 0.4)

**Date:** 2026-11-09
**Status:** Draft for decision (Phase 0 deliverable 0.4)
**Depends on:** `docs/specs/2026-11-09-yulia-region-inference.md` (0.3), `docs/specs/2026-11-09-yulia-design-review.md` §2.3
**Plan ref:** unavailable — `docs/plans/` was lost on 2026-11-10 (see decision log #82).

---

## 1. Purpose

This document presents the fallback policy decision (R.1/R.2/R.3) for Yulia. It is a **product-level decision** and requires sign-off before implementation.

---

## 2. The decision

When region inference cannot prove that a value's reference stays within its region, what does the compiler do?

---

## 3. Options

| Option | Description | Cost | Reversibility |
|---|---|---|---|
| **R.1 Restrict** | Reject the program at compile time. The language defines a safe subset. | Reduced expressiveness; must be documented as the language's real semantics. | High — can be relaxed later. |
| **R.2 Fallback** | Promote the value to a global region (never reclaimed) and emit a warning. | Unbounded memory growth; de facto leak. | Medium — can be tightened later. |
| **R.3 Novel** | Claim a new inference algorithm that beats prior art. | Unproven; must be published/proven before it is the foundation. | Low — hard to change once shipped. |

---

## 4. Criteria

| Criterion | Weight | R.1 | R.2 | R.3 |
|---|---|---|---|---|
| Soundness | Must | ✅ | ✅ (with warning) | Unproven |
| Expressiveness | High | Medium | High | High |
| Predictability | High | High | Low (silent leak) | Unknown |
| Implementation cost | Medium | Low | Low | High |
| Reversibility | Medium | High | Medium | Low |

---

## 5. Recommendation

**R.1 for Phase 1–2, with R.2 as an explicit, warned escape hatch for future features.**

Rationale:
- R.1 is the only option that is sound, predictable, and reversible.
- R.2 is acceptable only as an explicit, warned escape hatch — never silent.
- R.3 is not claimed; no novel algorithm is asserted.

---

## 6. Consequences

- The language's real semantics is the safe subset, not the full grammar.
- Programs outside the subset are rejected with a clear diagnostic. The diagnostic MUST name the specific rule violated and suggest the R.2 escape hatch if applicable.
- The escape hatch (R.2) is documented and warned.

---

## 7. Decision required

This is a product-level decision. It is escalated to the user per SOP-HUMAN.

---

## 8. Deliberation (SOP-THINK)

| Field | Content |
|---|---|
| **Goal** | Choose the fallback policy for region inference. |
| **Assumptions** | No novel algorithm is claimed; the safe subset is acceptable. |
| **Evidence** | 0.3 §6; design review §2.3; prior art (Cyclone, Rust, Koka). |
| **Alternatives** | R.1, R.2, R.3. |
| **Chosen** | R.1 + R.2 escape hatch. |
| **Risks** | The safe subset may be too restrictive; the escape hatch may be abused. |
| **Mitigations** | Document the subset; warn on the escape hatch; revisit in Phase 5. |
| **Confidence** | Medium-high. |
| **Kill-criteria** | If the safe subset cannot express the MVP, escalate. |
| **Verification plan** | Peer review; user sign-off. |

---

## 9. Acceptance / sign-off

- [x] R.1+R.2 recommendation signed off by user (SOP-HUMAN, Q1). Decision: R.1 restrict + R.2 warned escape hatch. Approved 2026-11-10. Escalation Q1-2026-11-09-01 resolved.
- [ ] Diagnostic naming rule agreed (§6).
- [x] Fallback policy incorporated into 0.3 (region inference spec) — cross-reference verified. Updated 2026-11-10 after Q1 sign-off.
- [ ] Escape hatch warning text reviewed for user-facing clarity.
- [ ] Formal proof (Phase 5.4).
