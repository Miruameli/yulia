# Yulia Language — Benchmark Methodology (Draft 0.8)

**Date:** 2026-11-09
**Status:** Draft for review (Phase 0 deliverable 0.8)
**Depends on:** `docs/specs/2026-11-09-yulia-design.md`
**Plan ref:** `docs/plans/2026-11-09-yulia-implementation-plan.md` §2, deliverable 0.8

---

## 1. Purpose

This document defines the benchmark methodology for Yulia: baseline, target, kernels, and measurement protocol. It ensures that performance claims are verifiable and reproducible.

---

## 2. Baseline and target

### 2.1 Baseline

The baseline is **C** (compiled with `gcc -O2` or `clang -O2`). C is the gold standard for systems programming performance. Yulia's goal is to match or exceed C on the defined kernels.

### 2.2 Target

**Yulia must achieve C-class performance (within 10% of C) on all defined kernels.** This is a hard requirement, not an aspiration.

### 2.3 Comparison languages

| Language | Role | Why |
|---|---|---|
| **C** | Baseline | Gold standard for systems programming |
| **Rust** | Competitor | Safe systems language with zero-cost abstractions |
| **Zig** | Competitor | Simple systems language with manual memory management |
| **Go** | Competitor | Simple language with GC (shows GC overhead) |

---

## 3. Kernels

### 3.1 K1: Matrix multiply (compute-bound)

**Description:** Multiply two N×N matrices of `float` (64-bit). N = 1024.

**Why:** Tests raw compute throughput and cache efficiency.

**Expected:** Yulia should match C within 5% (SIMD auto-vectorization should be equivalent).

### 3.2 K2: Linked list traversal (memory-bound)

**Description:** Traverse a linked list of 1,000,000 nodes, summing a field. Nodes are heap-allocated (region-allocated in Yulia).

**Why:** Tests memory access patterns and region reclamation overhead.

**Phase note:** Linked lists require pointers/indices, which are not in the Phase 1 subset. K2 is a Phase 4 benchmark (see §6 acceptance); the data structure is provided by the benchmark harness, not the language.

**Expected:** Yulia should match C within 10%. Region bump-pointer allocation should be faster than `malloc`.

### 3.3 K3: String processing (allocation-heavy)

**Description:** Read a 10 MB file, split into lines, count words, write result. Tests string allocation and copying.

**Why:** Tests allocation throughput and string handling.

**Expected:** Yulia should match C within 15% (strings are copied; C may use zero-copy tricks).

### 3.4 K4: Region allocation throughput (allocation-bound)

**Description:** Allocate 1,000,000 small structs (16 bytes each) in a region, then reclaim the region. Measure allocation + reclamation time.

**Why:** Tests the core claim: region bump-pointer allocation is faster than `malloc`/`free`.

**Expected:** Yulia should be **faster** than C (malloc/free) by at least 2x. This is Yulia's key differentiator.

### 3.5 K5: Compile time (tooling)

**Description:** Compile a 10,000-line Yulia program. Measure wall-clock time.

**Why:** Tests compiler performance. Fast compilation is a Yulia goal.

**Expected:** Yulia should compile faster than Rust (rustc). Target: at least 1.5x faster (softened from 2x; see risk note). Comparison with C (gcc) is informational.

**Risk note:** The 1.5x target is aspirational. Rust's compile times are dominated by borrow checking and monomorphization, which Yulia may avoid (simpler region inference). However, Yulia's region inference is not free. If the target is missed, the kill-criteria is: escalate before Phase 4.

---

## 4. Measurement protocol

### 4.1 Environment

- **Hardware:** Dedicated benchmark machine (no other processes). Record CPU model, RAM, storage.
- **OS:** Linux (Ubuntu LTS). Record kernel version.
- **Compiler:** Yulia compiler built in release mode. C compiled with `gcc -O2 -march=x86-64-v3` (fixed architecture for reproducibility; `-march=native` is prohibited because it varies by machine and breaks reproducibility).
- **Warm-up:** 3 warm-up runs (discarded).
- **Runs:** 10 measured runs per kernel.
- **Reporting:** Median and standard deviation. No cherry-picking.

### 4.2 Metrics

| Metric | Unit | How measured |
|---|---|---|
| Wall-clock time | ms | `time` or `hyperfine` |
| Peak RSS | MB | `/usr/bin/time -v` |
| Allocations | count | Instrumentation (Yulia only) |
| Region reclamation time | ms | Instrumentation (Yulia only) |

### 4.3 Reproducibility

- All benchmark code is in the repository (`benchmarks/` directory).
- All compiler versions and flags are recorded.
- Results are published with the release.
- Anyone can reproduce by running `make benchmark`.

### 4.4 Regression detection

- Every PR that touches the compiler or runtime runs the benchmark suite.
- A regression of >5% on any kernel blocks the PR.
- Regressions are tracked in the issue tracker.

---

## 5. Acceptance

- [x] Baseline (C) and target (C-class, within 10%) defined.
- [x] Comparison languages listed (C, Rust, Zig, Go).
- [x] 5 kernels defined (K1–K5).
- [x] Measurement protocol specified (environment, metrics, reproducibility, regression detection).
- [ ] Benchmarks implemented (Phase 4).
- [ ] Baseline results recorded (Phase 4).

---

## 6. Deliberation (SOP-THINK)

| Field | Content |
|---|---|
| **Goal** | Define benchmark methodology for Yulia. |
| **Assumptions** | C is the appropriate baseline; 10% tolerance is achievable. |
| **Evidence** | Design doc §3 (performance goals); standard benchmarking practice (hyperfine, criterion). |
| **Alternatives** | (a) C baseline; (b) Rust baseline; (c) no baseline (relative only). |
| **Chosen** | (a). C is the gold standard for systems programming. |
| **Risks** | C may be unachievable for some kernels; 10% tolerance may be too tight. |
| **Mitigations** | K4 (region allocation) is expected to beat C; K3 (string processing) has 15% tolerance. |
| **Confidence** | Medium. |
| **Kill-criteria** | If no kernel achieves C-class performance after Phase 4, escalate. |
| **Verification plan** | Peer review; Phase 4 benchmark results. |