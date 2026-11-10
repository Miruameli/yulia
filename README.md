# Yulia

[![CI](https://github.com/Miruameli/yulia/actions/workflows/ci.yml/badge.svg)](https://github.com/Miruameli/yulia/actions/workflows/ci.yml)

Yulia is a procedural programming language with explicit types and a
compiler-inferred, region-based memory model: no `malloc`/`free`, no garbage
collector, no lifetime annotations, and deterministic reclamation.

**Status:** Phase 0 (language specification) is complete. Phase 1 (toolchain)
is starting. The specification lives in [`docs/specs/`](docs/specs/); the
development workflow is in [`CONTRIBUTING.md`](CONTRIBUTING.md).

## Layout

| Path | Purpose |
|---|---|
| `src/` | Compiler crate (Phase 1 pipeline modules) |
| `benchmarks/` | Benchmark suite and harness (spec §4.3) |
| `docs/specs/` | Language specification — Phase 0 deliverables 0.1–0.9 |
| `docs/specs/2026-11-10-yulia-open-decisions.md` | Open decisions register (owner + decision gate) |

## Getting started

The Rust toolchain is pinned in [`rust-toolchain.toml`](rust-toolchain.toml);
`rustup` installs it automatically on the first `cargo` invocation.

```sh
make help        # list every target
make fmt         # format
make lint        # clippy, warnings denied
make test        # test suite
make benchmark   # benchmark suite (spec §4.3)
make codegraph   # rebuild the codegraph index
```

**CI status:** the workflow in `.github/workflows/ci.yml` runs on every push to
`main` and on every pull request. `make ci` runs the same fast checks locally
(format, clippy, tests, build); `make ci-full` additionally runs the dependency
audit, spell check, and codegraph index, and requires `cargo-deny` and
`typos-cli` to be installed.

## Compiler status

The driver (`src/main.rs`) currently supports `--help` and `--version`. The
compilation pipeline — lexer, parser, type checker, region inference, LLVM
codegen — is Phase 1 work.

## License

[MPL-2.0](LICENSE).
