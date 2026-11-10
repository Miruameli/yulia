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

### Changed

- The toolchain implementation language is Rust with an LLVM backend
  (user decision, 2026-11-10). `docs/specs/2026-11-09-yulia-design.md`
  §Constraints, §Toolchain, and §Restrictions and fallback now state Rust,
  LLVM via inkwell, and the MPL-2.0 license.

### Removed

- The previous repository history and `docs/plans/` were lost when the `.git`
  directory disappeared on 2026-11-10. The specification under `docs/specs/`
  was unaffected and is the source of truth for the language.
