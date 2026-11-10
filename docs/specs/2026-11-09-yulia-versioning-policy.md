# Yulia Language — Versioning and Compatibility Policy (Draft 0.9)

**Date:** 2026-11-09
**Status:** Draft for review (Phase 0 deliverable 0.9)
**Depends on:** `docs/specs/2026-11-09-yulia-design.md`, `docs/specs/2026-11-09-yulia-ffi-rules.md` (0.6)
**Plan ref:** `docs/plans/2026-11-09-yulia-implementation-plan.md` §2, deliverable 0.9

---

## 1. Purpose

This document defines Yulia's versioning and compatibility policy: SemVer rules, deprecation policy, and ABI stability commitment.

---

## 2. Versioning scheme

Yulia follows **Semantic Versioning (SemVer)**: `MAJOR.MINOR.PATCH`.

| Component | When incremented |
|---|---|
| **MAJOR** | Breaking changes to the language, standard library, or ABI |
| **MINOR** | New features, backward-compatible additions |
| **PATCH** | Bug fixes, performance improvements, documentation |

**Pre-release versions:** Yulia does **not** ship pre-releases (alpha, beta, rc, canary, nightly, preview). Only stable releases ship (SOP-REL). Development versions are identified by git commit hash, not by a version number.

---

## 3. Language stability

### 3.1 What is "the language"

The language is defined by:
- The grammar (0.1).
- The operational semantics (0.2).
- The memory model (0.3).
- The standard library (Phase 2+).

### 3.2 Stability guarantees

| Version | Guarantee |
|---|---|
| **0.x** | No stability guarantees. The language may change in any way between minor versions. |
| **1.0+** | Breaking changes require a major version bump. Minor versions are backward-compatible. |
| **2.0+** | Same as 1.0, but with a stronger commitment to ABI stability. |



A change is breaking if it:
- Removes or renames a keyword, operator, or built-in function.
- Changes the type of a standard library function.
- Changes the memory layout of a type (affects FFI).
- Changes the ABI (function calling convention).
- Changes the semantics of an existing construct (e.g., integer overflow behavior).
- Removes a standard library module.

- A change is **not** breaking if it:
- **Adds a new keyword, operator, or built-in function.** *Caveat:* adding a new keyword is **breaking** if it conflicts with existing identifiers (a program using the new keyword as an identifier will fail to compile). To avoid this, new keywords SHOULD be added as context-sensitive (recognized only in specific syntactic positions) unless a major version bump is acceptable. This exception is reserved explicitly and must be documented per keyword.
- Adds a new standard library module or function.
- Improves performance without changing semantics.
- Fixes a bug (even if some programs relied on the buggy behavior, this is a bug fix, not a breaking change).

### 3.3 Reserved keywords

**Keyword reservation strategy:** To minimize conflicts, Yulia reserves a set of "future keywords" that are always parsed as keywords, even if not yet used. The canonical reserved-keyword list is **0.1 §2.5**; it currently includes `extern`, `export`, `pin`, `panic`, `thread`, `spawn`, `async`, `await`, `send`, `recv`, `mutex`, `defer`, `yield`, `match`, `trait`, `impl`. A program using any of these as an identifier is a compile error, even if the feature is not yet implemented. This prevents future breaking changes when the keyword is activated. The list may be extended (with a MAJOR version bump if the extension conflicts with existing programs).

---

## 4. Deprecation policy

### 4.1 Deprecation process

1. A feature is marked as deprecated in a **MINOR** release. A deprecation warning is emitted at compile time.
2. The feature remains functional for at least **2 MINOR** releases (or 1 MAJOR release, whichever spans more calendar time). "Whichever is longer" means: if a MAJOR release occurs before 2 MINOR releases have elapsed, the feature survives until that MAJOR release. If 2 MINOR releases occur first, the feature may be removed in the next MAJOR release after that. In practice: minimum deprecation window is 2 MINOR releases OR 1 MAJOR release, whichever is later.
3. The feature is removed in the next **MAJOR** release.

### 4.2 Deprecation notice

A deprecation warning includes:
- The deprecated feature.
- The replacement (if any).
- The version in which the feature will be removed.
- A migration guide (if the change is non-trivial).

### 4.3 Undeprecated features

A deprecated feature may be undeprecated if the replacement proves inadequate. Undeprecation requires a MAJOR version bump and a recorded decision.

---

## 5. ABI stability

### 5.1 ABI definition

The ABI (Application Binary Interface) is defined in 0.6 §4.4. It specifies how Yulia types map to C types and how functions are called.

### 5.2 ABI stability commitment

| Version | ABI guarantee |
|---|---|
| **0.x** | No ABI stability. The ABI may change between any versions. |
| **1.0+** | ABI is stable within a major version. Breaking ABI changes require a major version bump. |
| **2.0+** | ABI is stable across major versions (backward-compatible at the binary level). |

### 5.3 ABI versioning

The ABI version is tracked separately from the language version:
- ABI version = `MAJOR.MINOR` (no patch).
- A language PATCH release never changes the ABI.
- A language MINOR release may add to the ABI (new types, new functions) but does not break existing ABI.
- A language MAJOR release may break the ABI.

---

## 6. Standard library versioning

### 6.1 Stdlib follows language versioning

The standard library is versioned with the language. A `1.2.0` compiler ships with the `1.2.0` standard library.

### 6.2 Stdlib additions

New standard library modules or functions may be added in MINOR releases. They are never removed in MINOR releases (only deprecated, per §4).

### 6.3 Stdlib stability

- **0.x:** No stability guarantees.
- **1.0+:** Standard library APIs are stable within a major version. Breaking changes require a major version bump.

---

## 7. Compatibility matrix

| Compiler version | Can compile | Can link against |
|---|---|---|
| 0.x | 0.x programs (same minor) | 0.x libraries (same minor) |
| 1.x | 1.x programs (any minor) | 1.x libraries (same major) |
| 2.x | 2.x programs (any minor) | 2.x libraries (same major) |

Cross-major-version compilation is not supported. A 2.0 compiler cannot compile 1.x programs without migration.

---

## 8. Migration

### 8.1 Migration guides

Every MAJOR release ships with a migration guide documenting:
- Breaking changes.
- Deprecated features removed.
- New features.
- Step-by-step migration instructions.
- Automated migration tools (if available).

### 8.2 Migration tooling

An automated migration tool (`yulia-migrate`) is provided for MAJOR version upgrades. It handles:
- Keyword renames.
- Standard library API changes (where mechanical).
- Deprecation replacements.

Manual migration is required for semantic changes.

---

## 9. Acceptance

- [x] SemVer rules defined.
- [x] Language stability guarantees documented (0.x, 1.0+, 2.0+).
- [x] Breaking change definition documented.
- [x] Deprecation policy defined (2 MINOR releases, then removal in MAJOR).
- [x] ABI stability commitment documented.
- [x] Standard library versioning defined.
- [x] Compatibility matrix documented.
- [x] Migration policy defined.

---

## 10. Deliberation (SOP-THINK)

| Field | Content |
|---|---|
| **Goal** | Define versioning and compatibility policy for Yulia. |
| **Assumptions** | SemVer is the appropriate scheme; 0.x has no stability guarantees. |
| **Evidence** | SemVer spec; Rust (edition-based); Go (module versioning); Zig (no SemVer, master-based). |
| **Alternatives** | (a) SemVer; (b) CalVer; (c) Go-style module versioning. |
| **Chosen** | (a). SemVer is the industry standard and well-understood. |
| **Risks** | SemVer may be too rigid for a language in early development (0.x). |
| **Mitigations** | 0.x explicitly has no stability guarantees, allowing rapid iteration. |
| **Confidence** | High. |
| **Kill-criteria** | If SemVer proves inadequate, escalate before 1.0. |
| **Verification plan** | Peer review; community feedback. |