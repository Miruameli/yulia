# Developer runbook

How to work on the Yulia compiler: setup, the daily loop, and the spec workflow.

## Setup

1. Install the pinned toolchain — `rustup` reads `rust-toolchain.toml` and
   installs the pinned Rust automatically on the first `cargo` invocation.
2. Install LLVM 20 — the Phase 1b backend links through inkwell, which needs
   `llvm-config`. CI installs it with `.github/actions/install-llvm20/`;
   locally, `sudo apt-get install llvm-20-dev` (Debian/Ubuntu) and export
   `LLVM_SYS_20_PREFIX` if it is not on the default path.
3. `cargo build` — the crate is `yulia` (see `Cargo.toml`).
4. `cargo test` — unit tests plus the integration tests in `tests/`.

`make` wraps the common targets; `make help` lists them.

## Daily loop

| Task | Command |
|---|---|
| Format | `cargo fmt --all` |
| Lint | `cargo clippy --workspace --all-targets --all-features -- -D warnings` |
| Test | `cargo test --workspace --all-features -- --test-threads=2` |
| Full CI locally | `make ci` |

CI runs the same seven jobs — see [`docs/operations/README.md`](../operations/README.md).

## Adding or changing a language feature

1. **Spec first.** A language change requires a specification change
   (`CONTRIBUTING.md` §9). Update the relevant file under `docs/specs/`.
2. Record the outcome: a design decision goes in
   `docs/specs/2026-11-10-yulia-open-decisions.md`; an architecture decision
   goes in an ADR under `docs/adr/`.
3. Implement, with tests that cover the failure modes — not only the happy path.
4. Update `CHANGELOG.md` in the same change.

## Diagnostics

Every user-facing diagnostic carries a stable code: `error[<code>]` for compile
errors, `runtime error[<code>]` for runtime errors. The Phase 1 catalog is
`docs/specs/2026-11-09-yulia-error-panic-model.md` §2.2, and the scheme is
[`docs/adr/0001-diagnostic-code-scheme.md`](../adr/0001-diagnostic-code-scheme.md).
Add a new code to the catalog and to `src/diagnostics/` in the same change —
never one without the other.

## Compiling and running a program

```sh
cargo build --release
./target/release/yulia hello.yl          # check only; silent on success
./target/release/yulia hello.yl -o hello # write an object file and link it
./target/release/yulia hello.yl -o hello.o --emit obj
./target/release/yulia hello.yl -o hello.ll --emit ir
```

`-o/--output` selects the artifact; without it the driver only checks the
program. `--emit` picks the kind (`exe` default, `obj`, `ir`).

## Runtime errors

A program that violates a runtime invariant prints
`<file>:<line>:<col>: runtime error[<code>]: <message>` on stderr and aborts
(`abort()`, SIGABRT, exit 134). Phase 1 codes: R0101 array index out of bounds,
R0102 division by zero, R0103 integer overflow. The catalog is
[`docs/specs/2026-11-09-yulia-error-panic-model.md`](../specs/2026-11-09-yulia-error-panic-model.md)
§2.2.

## Commit conventions

Conventional Commits: subject ≤72 characters, imperative mood. The body
explains WHY and the impact. Reference the issue in the footer.

## Resources

Heavy commands are bounded (see
[`docs/operations/README.md`](../operations/README.md) §Resource limits):
`CARGO_BUILD_JOBS=2`, `--test-threads=2`, and `timeout -k 5` on the Makefile
targets.

File defects and feature requests in
[issues](https://github.com/Miruameli/yulia/issues) using a template; take
questions and design discussion to
[Discussions](https://github.com/Miruameli/yulia/discussions).
