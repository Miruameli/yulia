# Changelog

All notable changes to this project are documented in this file. The format is
based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the
project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Yulia does not ship pre-releases: only stable versions are released, and
development builds are identified by git commit hash
(`docs/specs/2026-11-09-yulia-versioning-policy.md` §2).

## [Unreleased]

### Added

- Repository infrastructure for long-term development:
  - Pinned Rust toolchain (`rust-toolchain.toml`), `rustfmt.toml`,
    `clippy.toml`, and a crate-level lint policy in `Cargo.toml`.
  - `Makefile` with `fmt`, `fmt-check`, `lint`, `test`, `test-nextest`,
    `benchmark`, `coverage`, `codegraph`, `doc`, `build`, `deny`, `typos`,
    `ci`, and `ci-full` targets.
  - CI workflow covering formatting, clippy, tests, build, dependency audit,
    spell checking, and the codegraph index.
  - `benchmarks/` with the harness required by the benchmark methodology
    (spec §4.3).
  - `CONTRIBUTING.md`, `CHANGELOG.md`, `README.md`, and an MPL-2.0 `LICENSE`.
  - `CODE_OF_CONDUCT.md` (Contributor Covenant 2.1) and `SECURITY.md`.
  - `deny.toml` (cargo-deny), `.typos.toml`, `.editorconfig`, `.gitattributes`,
    and `.config/nextest.toml`.
- Minimal compiler crate skeleton (`src/lib.rs`, `src/main.rs`) with a working
  `--help` / `--version` driver.
- Driver test coverage: every branch of `src/main.rs` is exercised
  (`tests/driver.rs`), plus a licence-invariant unit test in `src/lib.rs`.

### Changed

- The toolchain implementation language is Rust with an LLVM backend
  (user decision, 2026-11-10). `docs/specs/2026-11-09-yulia-design.md`
  §Constraints, §Toolchain, and §Restrictions and fallback now state Rust,
  LLVM via inkwell, and the MPL-2.0 license.
- The repository has a GitHub remote (`Miruameli/yulia`, public). CI runs on
  every push to `main` and on pull requests; the stale "no git remote" notes in
  `README.md`, `CONTRIBUTING.md`, and `.github/workflows/ci.yml` are corrected,
  and `Cargo.toml` now carries the `repository` metadata.
- Remaining stale "once a remote is configured" notes corrected: `CONTRIBUTING.md`
  §5 and `SECURITY.md` (reporting now points at the repository's private advisory
  form); `README.md` records that the Phase 0 exit sign-off is held.
- CI fixes: the codegraph job runs `codegraph init --yes` on a fresh checkout
  (`.codegraph/` is not committed), and a spell-check false positive caused by
  an inline-code split in `docs/specs/2026-11-09-yulia-ffi-rules.md` (R-FFI-2)
  is resolved by rewording.
- Phase 0 consistency gate (user directive, 2026-11-10): Phase 0 must be made
  internally consistent and pass strict review and theoretical soundness before
  Phase 1 begins. Phase 1 (toolchain) is ON HOLD until the user accepts the
  Phase 0 gate. `README.md` status, `CONTRIBUTING.md` §5, and `SECURITY.md`
  dependency policy now distinguish Phase 1a (std-only) from Phase 1b (adds
  inkwell/LLVM for codegen).
- `README.md` headline claims aligned with the spec audit findings: the language
  is now described as a "decidable safe subset" with "no implicit malloc/free,
  no tracing GC, no lifetime annotations, and deterministic reclamation for
  scoped regions"; the status line, benchmark harness line, and spec deliverable
  list are corrected.
- `README.md` rewritten as a project-facing README: banner, badges, table of
  contents, "Why Yulia?", a feature list, a worked code example, the memory-model
  metaphor table, repository layout, getting-started, a phase roadmap, and a
  specification index.
- Security alerting enabled on the repository: Dependabot alerts and CodeQL code
  scanning (default setup) are now active, ahead of the first third-party
  dependency (inkwell, Phase 1b).
- `benchmarks/README.md` kernel table corrected: K1 is a Phase 4 benchmark
  (spec 0.8 §4.4), not Phase 1; the phase note now records that K1 requires
  `float` (Phase 2+).
- `.config/nextest.toml` bounds test parallelism to `min(2, nproc/2)`
  (SOP-RESLIM.2 / INV-35); `test-threads = "num-cpus"` used every core.
- `Makefile` and CI bound cargo's parallel build jobs to `min(2, nproc/2)`
  (`CARGO_BUILD_JOBS=2`) and the test harness to 2 threads (SOP-RESLIM.2 / INV-35).
  The CI `test` job now passes `--test-threads=2` as well: it called `cargo test`
  directly, so the Makefile's bound did not apply to CI.
- Phase 0 gate accepted (user, 2026-11-11): the specification passed strict
  review and theoretical soundness, so Phase 1 (toolchain) is unblocked and in
  progress. `README.md` status and roadmap updated; the earlier "on hold"
  wording above is superseded.

### Fixed

- `benchmarks/README.md` kernel example: the sample invocation used
  `kernels/k1.yl`, which cannot resolve because the harness runs each kernel
  with the repository root as the working directory. It now uses
  `benchmarks/kernels/k1.yl`, and the working-directory rule is stated.
- `docs/specs/2026-11-09-yulia-error-panic-model.md` §2: removed
  "assertion failure" from the Phase 1 runtime-error triggers — no Phase 1
  construct produces one (0.1 §2.5/§3.3/§10, D1, D2). Recorded as review-log
  finding F56.

### Removed

- The previous repository history and `docs/plans/` were lost when the `.git`
  directory disappeared on 2026-11-10. The specification under `docs/specs/`
  was unaffected and is the source of truth for the language.
