# Yulia Language — Operational Semantics and Memory Model (Draft 0.2)

**Date:** 2026-11-09
**Status:** Draft for review (Phase 0 deliverable 0.2)
**Depends on:** `docs/specs/2026-11-09-yulia-grammar-draft.md` (0.1), `docs/specs/2026-11-09-yulia-region-inference.md` (0.3)
**Plan ref:** `docs/plans/2026-11-09-yulia-implementation-plan.md` §2, deliverable 0.2

---

## 1. Purpose

This document defines the operational semantics and memory model of Yulia. It is the formal foundation for the compiler, the reference interpreter, and the soundness proof.

---

## 2. Approach

We use **small-step operational semantics** (SOS) with an explicit store and a region stack. The semantics is defined for the Phase 1 subset first, then extended to Phase 2.

---

## 3. Semantic domains

```
Expr     -- expressions
Val      -- values: int | float | bool | char | string | struct | array
Var      -- variable names
Region   -- region identifiers
Loc      -- memory locations
Store    -- mapping from Loc to Val
Regions  -- stack of active regions
```

---

## 4. Phase 1 semantics

### 4.1 Values

```
Val ::= INT_LIT | FLOAT_LIT | BOOL_LIT | CHAR_LIT | STRING_LIT
      | StructVal | ArrayVal
StructVal ::= "struct" IDENT "{" field_values "}"
ArrayVal  ::= "array" "[" Val { "," Val } "]"
```

### 4.2 Environments

```
Env ::= Var -> Loc
```

An environment maps variable names to locations. Each function call creates a fresh environment.

### 4.3 Stores and regions

```
Store    ::= Loc -> Val
Regions  ::= [ Region ]
```

The store maps locations to values. The region stack tracks active regions. A region is pushed on function entry and popped on return.

### 4.4 Reduction rules

**Variable lookup:**

```
(Var x, env, store, regions) -> (store(env(x)), env, store, regions)
```

**Arithmetic:**

```
(INT_LIT a + INT_LIT b, env, store, regions) -> (INT_LIT (a + b), env, store, regions)
```

**Function call:**

```
(f(v1, ..., vn), env, store, regions) ->
  let args = [store(env(v1)), ..., store(env(vn))]
  let new_env = fresh_env(f.params, args)
  let new_region = fresh_region()
  (f.body, new_env, store, regions ++ [new_region])
```

**Return:**

```
(return e, env, store, regions) -> (e_val, env, store, regions)
```

The caller copies the return value into its own region. For Phase 1 (by-value), this is a deep copy for structs and arrays (element-wise). For Phase 2, strings and growable arrays follow the rules in 0.3 §5.

**Struct field access:**

```
(s.f, env, store, regions) -> (v, env, store, regions)
  where store(env(s)) = StructVal { f: v, ... }
```

**Array indexing:**

```
(a[i], env, store, regions) -> (v, env, store, regions)
  where store(env(a)) = ArrayVal [v0, ..., vn] and 0 <= i < n

Note: the bounds check `0 <= i < n` is a runtime check in Phase 1. Compile-time bounds checking (when provable) is a Phase 2 optimization (MEM-SAFE-2).

---

## 5. Memory model

### 5.1 Regions

A region is a scope-bound arena. A region is created on function entry and reclaimed on return. Reclamation is deterministic and O(1).

### 5.2 Allocation

A value is allocated in the region of its lexical scope. Allocation is a bump-pointer increment.

### 5.3 Reclamation

On function return, the region is popped and its memory is reclaimed. No per-object destructors are needed for Phase 1 (no references).

### 5.4 Safety invariant

**MEM-SAFE-1:** No reference outlives its region.

In Phase 1, this holds trivially: there are no references.

---

## 6. Type safety

**Theorem (Progress).** If `e` is well-typed, then `e` is either a value or can take a step.

**Theorem (Preservation).** If `e` is well-typed and `e -> e'`, then `e'` is well-typed.

*Proof sketch.* By induction on the reduction rules. Each rule preserves the type of the expression.

---

## 7. Open decisions

1. Panic/unwind semantics: how are regions reclaimed on unwind? (See Phase 0 deliverable 0.5.)
2. Phase 2: how do strings and growable arrays extend the store?
3. Concurrency: how do regions interact with threads?

---

## 8. Acceptance

- [x] Semantic domains defined.
- [x] Phase 1 reduction rules specified.
- [x] Memory model defined.
- [x] Safety invariant stated.
- [x] Type safety theorems sketched.
- [ ] Full formal proof (Phase 5.4).

---

## 9. Deliberation (SOP-THINK)

| Field | Content |
|---|---|
| **Goal** | Define the operational semantics and memory model. |
| **Assumptions** | Small-step SOS is sufficient; the Phase 1 subset is the target. |
| **Evidence** | Standard SOS literature (Plotkin 1981; Felleisen & Hieb 1992); 0.3 region model. |
| **Alternatives** | (a) Big-step semantics; (b) small-step SOS; (c) denotational semantics. |
| **Chosen** | (b). |
| **Risks** | The semantics may not scale to Phase 2; the proof sketch is incomplete. |
| **Mitigations** | Phase 5.4 formal proof; reference interpreter. |
| **Confidence** | Medium. |
| **Kill-criteria** | If the semantics cannot express a Phase 2 feature, escalate before implementation. |
| **Verification plan** | Peer review; reference interpreter; formal proof (Phase 5.4). |
