# Yulia Language — C FFI Memory-Safety Rules (Draft 0.6)

**Date:** 2026-11-09
**Status:** Draft for review (Phase 0 deliverable 0.6)
**Depends on:** `docs/specs/2026-11-09-yulia-region-inference.md` (0.3), `docs/specs/2026-11-09-yulia-escape-analysis.md` (0.3a), `docs/specs/2026-11-09-yulia-error-panic-model.md` (0.5)
**Plan ref:** `docs/plans/2026-11-09-yulia-implementation-plan.md` §2, deliverable 0.6

---

## 1. Purpose

This document defines the memory-safety rules for C FFI (foreign function interface) in Yulia. It also resolves the SoA-vs-ABI tension from the design document (tension #3).

**Syntax note:** `extern "C"`, `export`, `pin`, and `CPtr` are reserved keywords/types in the 0.1 grammar (§2.5, §10.2–10.5) and are Phase 2 features. `#[layout(soa)]` attributes are deferred to Phase 5 (0.1 §10.6).

Yulia calls C functions via `extern "C"` declarations. C functions may be called from Yulia, and Yulia functions may be called from C (via exported symbols).

### 2.1 Calling C from Yulia

```yulia
extern "C" fn c_function(arg: int) -> int;
```

The C function is linked at build time. Arguments are passed according to the C ABI.

### 2.2 Calling Yulia from C

Yulia functions marked `export` are callable from C:

```yulia
export fn yulia_function(x: int) -> int { ... }
```

---

## 3. Pointer lifetime rules

### 3.1 Yulia → C

When a Yulia value's address is passed to C:

| Rule | Description |
|---|---|
| **R-FFI-1** | A C function may not retain a pointer to a Yulia region value beyond the call. |
| **R-FFI-2** | If C needs to retain a pointer, the value must be `pin`ned (0.3a §4.3). A pinned value's region is kept alive for the duration of the pin. |
| **R-FFI-3** | A C function may read or write the pointed-to value during the call. After the call returns, the pointer is invalid. |
| **R-FFI-4** | Passing a pointer to a growable collection's internal buffer is forbidden (the buffer may reallocate). |

**Panic at FFI boundary:** See 0.5 §4.4 for panic handling during C FFI calls. In Phase 1, panic calls `abort()` (0.5 §4.4); in Phase 2+, the runtime catches panics via a setjmp/longjmp boundary at the FFI boundary and reclaims Yulia regions.

### 3.2 C → Yulia

When C returns a pointer to Yulia:

| Rule | Description |
|---|---|
| **R-FFI-5** | A C-returned pointer must be wrapped in a `CPtr<T>` type. |
| **R-FFI-6** | `CPtr<T>` is opaque: Yulia cannot dereference it directly. |
| **R-FFI-7** | To access the pointed-to value, the programmer must call a Yulia function that wraps the C dereference. |
| **R-FFI-8** | `CPtr<T>` values are never reclaimed by Yulia's region system. The programmer must explicitly free them via `c_free(ptr)` (or the appropriate C deallocator). |

**Rationale:** C-allocated memory is outside Yulia's region system. Yulia cannot safely reclaim it. The programmer is responsible for C memory management.

### 3.3 Pinning

A `pin` construct keeps a Yulia region alive for an extended duration:

```yulia
fn process(data: [int; 256]) {
    pin data as pinned {
        c_process(pinned.ptr, 256);
        // pinned.ptr is valid here
    }
    // pinned.ptr is now invalid
}
```

**Rules:**
- A pinned value is E3 by declaration (0.3a §4.3).
- The pin's region is reclaimed when the pin goes out of scope.
- A pin that outlives its scope is a compile error.
- Pinning does not transfer ownership to C; C must not retain the pointer beyond the pin's scope.

---

## 4. SoA vs. ABI decision (design doc tension #3)

### 4.1 The tension

The design document (§7, tension #3) identifies a conflict:
- **Structure of Arrays (SoA):** Yulia may store struct fields in separate arrays for cache efficiency.
- **C ABI compatibility:** C expects Array of Structures (AoS) layout for FFI.

### 4.2 Decision

**Yulia uses AoS (Array of Structures) as the default layout.** SoA is available as an explicit optimization annotation, not the default.

**Rationale:**
1. **FFI compatibility:** AoS matches the C ABI. No conversion is needed for FFI calls.
2. **Simplicity:** One default layout reduces cognitive load. SoA is an advanced optimization.
3. **Performance:** AoS is cache-friendly for most workloads. SoA is better for SIMD-heavy kernels, which are a minority of use cases.

### 4.3 SoA annotation

When SoA is desired, the programmer annotates the struct or array:

```yulia
#[layout(soa)]
struct Particle { x: float; y: float; z: float; }
```

**Rules:**
- SoA structs are **not** FFI-compatible. Passing a SoA struct to C is a compile error.
- SoA arrays are stored as separate arrays per field. Access is transparent (the language handles indexing).
- SoA is a Phase 5 optimization (5.2 "Static layout layer"). The annotation is accepted in Phase 2 but has no effect until Phase 5.

### 4.4 ABI stability

Yulia's C ABI for exported functions follows the platform's C calling convention:
- `int` → `int32_t` (Yulia's `int` is 32-bit; fixed width for ABI determinism). See 0.2 §3 (semantic domains) for the type definition.
- `float` → `double` (Yulia's `float` is 64-bit)
- `bool` → `int` (0 or 1)
- `char` → `char` (UTF-8 byte)
- `string` → `const char*` (null-terminated, UTF-8). **Buffer ownership:** Yulia allocates a null-terminated UTF-8 copy of the string in the caller's region. The pointer is valid for the duration of the C call only (R-FFI-1). C must not retain the pointer. If C needs a longer-lived buffer, the programmer must allocate it explicitly via `c_malloc` and copy the string content. See 0.3 Q3 (string ownership model) for details on string representation.
- Structs → C struct (AoS layout, packed according to platform ABI)
- Arrays → pointer + length (two arguments: `T* ptr, size_t len`)
- `void` return → `void`

**Commitment:** The ABI is stable within a major version. Breaking ABI changes require a major version bump (SemVer).

---

## 5. Negative tests

| Test | Expected |
|---|---|
| Pass non-pinned Yulia pointer to C that retains it | Reject (R-FFI-1) |
| Pass pinned Yulia pointer to C that retains it beyond pin scope | Reject (R-FFI-2) |
| Pass growable collection buffer to C | Reject (R-FFI-4) |
| Dereference CPtr without wrapper | Reject (R-FFI-6) |
| Leak CPtr (no free) | Compile-time warning (not error; C memory is programmer's responsibility) |
| Pass SoA struct to C | Reject (§4.3) |

---

## 6. Acceptance

- [x] Pointer lifetime rules defined (Yulia → C and C → Yulia).
- [x] Pinning semantics specified.
- [x] SoA vs. ABI decision recorded (AoS default, SoA as annotated optimization).
- [x] ABI stability commitment documented.
- [x] Negative tests specified.
- [ ] Formal proof (Phase 5.4).

---

## 7. Deliberation (SOP-THINK)

| Field | Content |
|---|---|
| **Goal** | Define C FFI memory-safety rules and resolve SoA/ABI tension. |
| **Assumptions** | C memory is outside Yulia's region system; Yulia cannot reclaim C-allocated memory. |
| **Evidence** | 0.3 §5.3 (pinning); 0.3a §4.2–4.3; design doc §7 tension #3; Rust FFI rules; Cyclone regions. |
| **Alternatives** | (a) AoS default + SoA annotation; (b) SoA default + AoS FFI conversion; (c) AoS only (no SoA). |
| **Chosen** | (a). AoS default for FFI compatibility; SoA as explicit optimization. |
| **Risks** | SoA annotation may be underused if the default is AoS; performance claims may suffer. |
| **Mitigations** | SoA is Phase 5 optimization; annotation accepted in Phase 2 but no-op until Phase 5. |
| **Confidence** | High on pointer rules; medium on SoA timeline. |
| **Kill-criteria** | If SoA proves essential for MVP performance, escalate before Phase 2. |
| **Verification plan** | Peer review; negative tests; FFI integration tests. |