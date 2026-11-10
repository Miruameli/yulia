# Yulia Language — Concurrency Model (Draft 0.7)

**Date:** 2026-11-09
**Status:** Draft for review (Phase 0 deliverable 0.7). **Pending 0.3 stable:** written against a 0.3 still in review; claims that depend on 0.3 region inference must be re-verified when 0.3 is finalized.
**Depends on:** `docs/specs/2026-11-09-yulia-region-inference.md` (0.3), `docs/specs/2026-11-09-yulia-operational-semantics.md` (0.2)
**Plan ref:** `docs/plans/2026-11-09-yulia-implementation-plan.md` §2, deliverable 0.7

---

## 1. Purpose
**Decision: DEFERRED to Phase 5.** Concurrency is not in MVP (Phases 1–4); see §2.

**Syntax note:** Concurrency constructs are reserved keywords in the 0.1 grammar (§2.5) but are not part of the Phase 1 subset. Thread constructs (`thread::spawn`, `spawn`, `send`, `recv`, `mutex`) are Phase 5; async constructs (`async`, `await`, coroutines) are Phase 6 (§3.2). The grammar is extended with their full syntax when those phases begin.

## 2. Decision: DEFERRED to Phase 5

**Concurrency is explicitly deferred to Phase 5 (Hardening).** The boundary is documented in §3. No concurrency primitives are included in MVP (Phases 1–4).

**Rationale:**
1. **Memory model first:** The region-inference model (0.3) must be proven on the sequential subset before concurrency is added. Concurrency introduces shared regions, ownership transfer, and data races — all of which complicate the model.
2. **Simplicity:** The simplicity philosophy favors sequential correctness first. Concurrency is a cross-cutting concern that should not be bolted on.
3. **Risk reduction:** Deferring concurrency reduces the risk that the memory model is unsound. If concurrency is added later, it builds on a proven foundation.
4. **Prior art:** Rust deferred concurrency to later phases (threads were added after the borrow checker was proven). Go included concurrency from the start but had to retrofit the memory model.

---

## 3. Boundary (what is and is not allowed in MVP)

### 3.1 Allowed in MVP (Phases 1–4)

- **Single-threaded programs only.** No threads, no async/await, no parallelism.
- **Sequential execution.** Functions execute to completion before the next function is called.
- **No shared mutable state.** There is only one thread, so no data races are possible.
- **Deterministic reclamation.** Regions are reclaimed in reverse creation order within a thread.

### 3.2 Forbidden in MVP

- `spawn` / `thread::spawn` / any thread creation.
- `async` / `await` / coroutines. **Async is deferred to Phase 6** (separate from threads: coroutines require regions that survive suspension, a different problem from thread region stacks).
- Shared mutable references across threads.
- Atomic operations.
- Locks, mutexes, channels, or any synchronization primitive.
- Memory ordering annotations.

### 3.3 What happens if a program tries to use concurrency

A program that references a concurrency primitive (e.g., `thread::spawn`) is a compile error in Phases 1–4. The error message states: "Concurrency is not available in this phase. See Phase 5."

---

## 4. Phase 5 concurrency model (threads)

When concurrency is added in Phase 5, the following model is proposed (subject to revision):

### 4.1 Ownership transfer

Regions are not shared across threads. Instead, ownership of a region is **transferred** from one thread to another. After transfer, the original thread can no longer access the region.

**Assumed model: move semantics (affine).** A value may be moved to another thread; the source thread's access is invalidated. This is the minimum assumption that keeps 0.3 sound without lifetime annotations. Linear types (full consumption) and other variants would require a rewrite of the 0.3 region-inference rules; if Phase 5 chooses a different model, 0.3 §5.4 must be revisited.

**Input to 0.3:** region inference (0.3) is designed against move/affine semantics. Any other ownership model is a Phase 5 change that requires re-verification of 0.3.

### 4.2 No shared regions

Shared mutable regions are forbidden. If two threads need to access the same data, one of the following must be used:
- **Message passing:** data is moved (ownership transferred) via a channel.
- **Immutable sharing:** data is read-only and may be shared. The compiler proves the data's region outlives both threads by requiring the data to be owned by a region that is not tied to either thread's stack (e.g., a `static`-like or explicitly scoped region). Without lifetime annotations, this is expressed as an explicit region binding, not inferred. **This is a Phase 5 requirement on 0.3:** if 0.3 cannot express it, immutable sharing is removed from Phase 5.
- **Synchronization:** explicit locks (with documented lock ordering to prevent deadlocks).
### 4.3 Region per thread

Each thread has its own region stack. Regions are not shared. A thread may create regions, and they are reclaimed when the thread exits or when the region goes out of scope.

**Thread-local state:** module-level `const` is immutable and shared read-only (no region). Mutable module-level state is not allowed in Phase 5 (it would be shared mutable state). A new thread starts with an empty region stack; it inherits no regions from its parent. Thread initialization is explicit (the spawned closure captures values by move).

### 4.4 Panic in threads

A panic in one thread does not affect other threads. The panicking thread's regions are reclaimed (0.5 §4.2). Other threads continue running.

- **Read-only shared regions:** if a read-only region is shared with the panicking thread, it is NOT reclaimed (it outlives both threads by definition, §4.2). No leak.
- **Join after panic:** a joining thread receives an error value (`Result::Err`) describing the panic. The error type is defined in Phase 5.
- **Child threads:** a panicking thread's child threads are orphaned (they continue running); they are not automatically cancelled. This is documented as a known sharp edge.
- **Cross-reference:** see 0.5 §4.2 for the reclamation rules and 0.5 §4.4 for the FFI boundary.

### 4.5 FFI and concurrency

Concurrency across the FFI boundary is **undefined in MVP and in Phase 5** unless explicitly specified. In particular:
- A C callback invoked from a C-owned thread does NOT enter a Yulia region; it is outside the region system.
- If a C callback calls back into Yulia and panics, the panic crosses the FFI boundary (0.5 §4.4) — the behavior is undefined.
- A C thread that outlives the Yulia program is not tracked.

This is documented here and cross-referenced from 0.6 (FFI rules). Phase 5 must either define this or keep it explicitly undefined.

---

## 5. Interaction with the memory model

The sequential memory model (0.3) is sound because there are no references that escape their region. In a concurrent setting, the following additional invariants are needed:

| Invariant | Description |
|---|---|
| MEM-SAFE-6 | No data race on shared mutable state (forbidden by design: no shared mutable regions). |
| MEM-SAFE-7 | No use-after-free across threads (ownership transfer prevents this). |
| MEM-SAFE-8 | No deadlock. Deadlock is not prevented by the language; use a documented lock-ordering convention. |

These invariants are Phase 5 deliverables. They are not checked in Phases 1–4. They are properties to be proven in Phase 5.4, not claims of this document.

---

## 6. Negative tests (Phase 5)

Each test is a program that MUST be rejected. The concurrency grammar is required (Phase 5); the tests are specified here so the Phase 5 grammar and diagnostics are testable.

| Test | Program form | Expected |
|---|---|---|
| Use `thread::spawn` in Phase 1–4 | `fn main() { thread::spawn(f); }` | Reject (compile error: concurrency not available) |
| Share a mutable region across threads | `let r = region::new(); thread::spawn(move \|\| r.write(1)); thread::spawn(move \|\| r.write(2));` | Reject (Phase 5) |
| Access a moved value from the source thread | `let v = 1; thread::spawn(move \|\| v); print(v);` | Reject (Phase 5: use after move) |
| Deadlock via circular lock acquisition | `let a = mutex::new(); let b = mutex::new(); two threads lock a then b, and b then a` | Not prevented; documented convention required |

---

## 7. Acceptance

- [x] Concurrency explicitly deferred to Phase 5.
- [x] Boundary documented (what is allowed/forbidden in MVP).
- [x] Phase 5 model previewed (ownership transfer, no shared regions).
- [x] Interaction with memory model documented.
- [x] Negative tests specified (Phase 5).
- [ ] Formal proof (Phase 5.4).
- [ ] Concurrency extension feasibility study run at end of Phase 4 (paper exercise against the Phase 4 memory model).

---

## 8. Deliberation (SOP-THINK)

| Field | Content |
|---|---|
| **Goal** | Decide whether to define a concurrency model in Phase 0 or defer it. |
| **Assumptions** | The sequential memory model must be proven before concurrency is added. |
| **Evidence** | 0.3 §5.4 (concurrency deferred); design doc §7; Rust (std::thread shipped in 1.0, 2015; the NLL borrow checker came later, 2018 — so concurrency was not "after" the borrow checker); Go (concurrency first, memory model retrofitted). |
| **Alternatives** | (a) Define full concurrency model now; (b) defer with documented boundary; (c) define a minimal model (channels only). |
| **Chosen** | (b). Defer with documented boundary. |
| **Risks** | Deferral may lead to a model that doesn't fit the existing design; Phase 5 may be too late to add concurrency. |
| **Mitigations** | Phase 5 preview in §4; boundary is clear; memory model invariants are extensible. |
| **Confidence** | High. |
| **Kill-criteria** | If the sequential model cannot be extended to concurrency, escalate before Phase 5. |
| **Verification plan** | Peer review; Phase 5 prototype. |