<div align="center">

<pre>
██╗   ██╗██╗   ██╗██╗     ██╗ █████╗
╚██╗ ██╔╝██║   ██║██║     ██║██╔══██╗
 ╚████╔╝ ██║   ██║██║     ██║███████║
  ╚██╔╝  ██║   ██║██║     ██║██╔══██║
   ██║   ╚██████╔╝███████╗██║██║  ██║
   ╚═╝    ╚═════╝ ╚══════╝╚═╝╚═╝  ╚═╝
</pre>

# Yulia

**A procedural language with explicit types and a compiler-inferred,
region-based memory model.** No tracing GC. No lifetime annotations.
No `malloc`/`free` for Yulia values.

[![CI](https://github.com/Miruameli/yulia/actions/workflows/ci.yml/badge.svg)](https://github.com/Miruameli/yulia/actions/workflows/ci.yml)
[![License: MPL-2.0](https://img.shields.io/badge/license-MPL--2.0-blue.svg)](LICENSE)
[![Rust 1.99.1](https://img.shields.io/badge/rust-1.99.1-ed4b32.svg)](rust-toolchain.toml)
[![Status](https://img.shields.io/badge/status-Phase%201a%20front--end-blue.svg)](docs/specs/)

</div>

## Table of contents

- [Why Yulia?](#why-yulia)
- [At a glance](#at-a-glance)
- [Features](#features)
- [A taste of Yulia](#a-taste-of-yulia)
- [The memory model](#the-memory-model)
- [Repository layout](#repository-layout)
- [Getting started](#getting-started)
- [Status and roadmap](#status-and-roadmap)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [License](#license)

---

## Why Yulia?

Systems programming forces a choice: manage memory by hand (C, C++) or pay for a
garbage collector (Go, Java, Python). Yulia removes that choice **for a decidable
safe subset** — the compiler infers where every value lives and reclaims it
deterministically when its region exits. The programmer writes no lifetime
annotations and no `malloc`/`free`; the memory model stays invisible until the
compiler cannot prove safety, at which point it says so instead of guessing.

The claim is deliberately bounded. Full region inference for arbitrary imperative
programs is not generally decidable, so the language states its boundary
explicitly ([R.1 / R.2](#the-memory-model)).

## At a glance

| | |
|---|---|
| **Paradigm** | Procedural — functions, structs, modules |
| **Types** | Explicit and static; no `auto`, no `var` |
| **Memory** | Compiler-inferred regions; deterministic reclamation for scoped regions |
| **Performance** | Targeting C-class speed (0.8 §2.2); not yet measured |
| **Garbage collector** | None |
| **Lifetime annotations** | None — `pin` is the sole explicit escape hatch |
| **Implementation** | Rust → LLVM IR (inkwell, Phase 1b) |
| **License** | MPL-2.0 |
| **Status** | Phase 1 — front-end in progress; Phase 0 complete and accepted |

## Features

- **Region-based memory, inferred.** Values live in function-bound regions that
  are reclaimed together when the region exits. No GC pauses, no per-object
  headers for plain structs.
- **Dense packing and slot reuse.** Values are laid out compactly; memory slots
  are recycled rather than churned through `malloc`/`free`.
- **Explicit types.** Every declaration carries its type. Region inference is
  memory inference, not type inference.
- **Familiar, unambiguous syntax.** C-style braces, `fn`, `struct`, `let`, and
  at most 3 tokens of lookahead — every program has exactly one parse tree.
- **Deterministic reclamation.** Values in scoped regions are reclaimed at
  region exit, at a point the programmer can reason about. R.2-promoted values
  are the documented exception.
- **A real toolchain plan.** Compiler, LSP, formatter, and build tooling are
  scheduled from the start.

## A taste of Yulia

```yulia
module main;

struct Point {
    x: int;
    y: int;
}

fn distance_squared(p: Point) -> int {
    return p.x * p.x + p.y * p.y;
}

fn main() {
    let origin: Point = Point { x: 0, y: 0 };
    let p: Point = Point { x: 3, y: 4 };
    let d: int = distance_squared(p);  // d == 25
}
```

No `&`, no `*`, no allocator, no lifetime. `p` is passed by value at the
language level; the compiler decides the layout and the region.

## The memory model

The design doc states the model as a set of metaphors, then maps each one to a
mechanism:

| Metaphor | Mechanism |
|---|---|
| The standard book | The process address space / region heap |
| A neatly torn sheet | A region (the allocation unit) |
| Writing with very small letters | Dense packing; AoS by default, SoA on request |
| Lamp A replaced by lamp B | Slot reuse — recycling, not repeated alloc + free |
| Everything managed from behind | Compiler-managed lifetime and ownership |

**The boundary is explicit.** The safety guarantee holds for a decidable subset:

- **R.1 — restrict.** A program the compiler cannot prove safe is rejected at
  compile time, with a diagnostic naming the rule.
- **R.2 — warned escape hatch.** When a future feature cannot be made safe, the
  value is promoted to a global region (never reclaimed) and the compiler emits
  a warning. This is a documented, unbounded leak — not a tracing GC — and it is
  not available in Phase 1–2.

## Repository layout

| Path | Purpose |
|---|---|
| `src/` | Compiler crate — Phase 1a front-end, Phase 1b codegen |
| `tests/` | Integration tests (`tests/driver.rs`) |
| `benchmarks/` | Benchmark harness; kernels land in Phase 4 (spec §4.3) |
| `docs/specs/` | The language specification — Phase 0 deliverables 0.1–0.9, 0.3a, design, design review, open decisions, review log |
| `docs/operations/` | Operations runbook — CI, resource limits, OOM response |
| `docs/adr/` | Architecture decision records |
| `.github/workflows/ci.yml` | CI: formatting, clippy, tests, build, dependency audit, spell check, codegraph |

## Getting started

The Rust toolchain is pinned in [`rust-toolchain.toml`](rust-toolchain.toml);
`rustup` installs it automatically on the first `cargo` invocation.

```sh
git clone https://github.com/Miruameli/yulia
cd yulia
make help        # list every target
```

| Target | What it does |
|---|---|
| `make fmt` | Format the Rust sources |
| `make lint` | Clippy, warnings denied |
| `make test` | Run the test suite |
| `make build` | Release build |
| `make benchmark` | Benchmark suite (spec §4.3) |
| `make ci` | The fast checks CI runs — format, lint, test, build |
| `make ci-full` | Everything CI runs, locally (needs `cargo-deny`, `typos-cli`) |

CI runs on every push to `main` and on every pull request. The compiler driver
(`src/main.rs`) currently supports `--help` and `--version`; the pipeline
(lexer, parser, type checker, region inference, LLVM codegen) is Phase 1 work.

## Status and roadmap

| Phase | Scope | Status |
|---|---|---|
| **0** | Language specification | Complete; gate accepted 2026-11-11 |
| **1a** | Front-end: lexer, parser, AST, type checker, diagnostics | In progress |
| **1b** | Codegen: LLVM IR via inkwell | Planned (after 1a) |
| **2** | Region inference for the wider feature set (strings, growable arrays, FFI) | Open decisions D4–D11 |
| **4** | Benchmark kernels K1–K5 + baseline data | Open (D12) |
| **5** | Concurrency (threads), SoA, generics, soundness argument | Open (D13–D17) |
| **6** | Async / await / coroutines | Open (D14) |

The Phase 0 gate was accepted on 2026-11-11: the specification passed strict
review and theoretical soundness, so Phase 1 (toolchain) is unblocked and in
progress. Formal proofs (D17) and the benchmark suite (D12, Phase 4) remain
open. The open decisions register is the single source of truth for what is
still undecided:
[`docs/specs/2026-11-10-yulia-open-decisions.md`](docs/specs/2026-11-10-yulia-open-decisions.md).

## Documentation

The specification lives in [`docs/specs/`](docs/specs/):

| Document | Contents |
|---|---|
| [`2026-11-09-yulia-design.md`](docs/specs/2026-11-09-yulia-design.md) | Intent, goals, memory model, restrictions |
| [`2026-11-09-yulia-design-review.md`](docs/specs/2026-11-09-yulia-design-review.md) | Independent design review — verdict and findings |
| [`2026-11-09-yulia-grammar-draft.md`](docs/specs/2026-11-09-yulia-grammar-draft.md) | Concrete grammar and lexical rules (0.1) |
| [`2026-11-09-yulia-operational-semantics.md`](docs/specs/2026-11-09-yulia-operational-semantics.md) | Operational semantics (0.2) |
| [`2026-11-09-yulia-region-inference.md`](docs/specs/2026-11-09-yulia-region-inference.md) | Region inference rules (0.3) |
| [`2026-11-09-yulia-escape-analysis.md`](docs/specs/2026-11-09-yulia-escape-analysis.md) | Escape analysis and the `pin` escape hatch (0.3a) |
| [`2026-11-09-yulia-fallback-policy.md`](docs/specs/2026-11-09-yulia-fallback-policy.md) | Fallback policy: R.1 restrict / R.2 warned escape hatch (0.4) |
| [`2026-11-09-yulia-error-panic-model.md`](docs/specs/2026-11-09-yulia-error-panic-model.md) | Errors, panics, and abort behavior (0.5) |
| [`2026-11-09-yulia-ffi-rules.md`](docs/specs/2026-11-09-yulia-ffi-rules.md) | C FFI rules (0.6) |
| [`2026-11-09-yulia-concurrency-model.md`](docs/specs/2026-11-09-yulia-concurrency-model.md) | Concurrency model (0.7) |
| [`2026-11-09-yulia-benchmark-methodology.md`](docs/specs/2026-11-09-yulia-benchmark-methodology.md) | Benchmark methodology (0.8) |
| [`2026-11-09-yulia-versioning-policy.md`](docs/specs/2026-11-09-yulia-versioning-policy.md) | Versioning and release policy (0.9) |
| [`2026-11-10-yulia-open-decisions.md`](docs/specs/2026-11-10-yulia-open-decisions.md) | Open decisions register |
| [`2026-11-10-yulia-review-log.md`](docs/specs/2026-11-10-yulia-review-log.md) | Review log |

Architecture decision records live in [`docs/adr/`](docs/adr/):

| Document | Contents |
|---|---|
| [`adr/0001-diagnostic-code-scheme.md`](docs/adr/0001-diagnostic-code-scheme.md) | Diagnostic code scheme (ADR 0001) |

Operations and runbooks live in [`docs/operations/`](docs/operations/):

| Document | Contents |
|---|---|
| [`operations/README.md`](docs/operations/README.md) | CI, resource limits, OOM response, release, secrets |

## Contributing

See [`CONTRIBUTING.md`](CONTRIBUTING.md) for the development workflow, and
[`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) for the community rules. Security
reports go through the process in [`SECURITY.md`](SECURITY.md).

## License

[Yulia is licensed under the MPL-2.0](LICENSE).
