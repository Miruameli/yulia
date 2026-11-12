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
- `docs/operations/README.md`: the operations runbook — CI jobs and timeouts,
  resource limits, the OOM response procedure, the release process, and the
  secrets posture (SOP-DOC).
- `docs/adr/`: the ADR index and ADR 0001 (diagnostic code scheme) — the
  stage-based `E0xx`/`E1xx`/`E2xx`/`E3xx`/`R0xx` ranges and the `error[<code>]`
  format (SOP-DOC).
- `docs/runbook/README.md`: the developer runbook — setup, the daily loop,
  the spec-first workflow, the diagnostic code rule, and commit conventions
  (SOP-DOC).
- `docs/operations/threat-model.md` and `docs/operations/system-model.md`: the
  attacker model, threats, invariants under attack, and coordinated disclosure,
  and the compiler pipeline / crate layout / trust boundaries (SOP-DOC).
- Phase 1a region inference (`src/region.rs`): the single lexical pass of
  0.3 §3 — one region per function, every binding attributed to it, O(n)
  (0.3 §8). The pass emits no diagnostics; the hard part of the model is
  deferred to Phase 2 (0.3 §4).
- The front-end pipeline is wired into the driver (`src/compile.rs`): lex ->
  parse -> type check -> region inference, stopping at the first stage that
  reports a diagnostic. `src/main.rs` renders `file:line:col: error[code]:
  message` (0.5 §2.1) on stderr and exits 1; a clean compile is silent.
- End-to-end tests over the 24 illustrative programs of 0.1 §6
  (`tests/programs.rs`): every program inside the Phase 1 subset must compile,
  and every one outside it must be rejected with the subset diagnostic.
- Repository hygiene: GitHub Discussions enabled, issue templates
  (`.github/ISSUE_TEMPLATE/`), `.github/CODEOWNERS`, and the project labels
  `spec`, `security`, `tooling`, and `codegen`.

### Changed

- `KONTRAK.md` updated from AI Operating Contract v3.9 to v4.0
  (human-supplied). v4.0 adds INV-36/37/38 (GHOP / TOOL / TEMPLATES
  compliance), the expanded SOP-GHOP (§12), new SOP-TOOL (§13), new
  SOP-TEMPLATES (§14), §50 verification commands, and §59 examples.

- `src/region.rs` split into `src/region/mod.rs` (types and module doc) and
  `src/region/assign.rs` (the `Assignment` impl) to satisfy the SOP-MOD
  150 SLOC/file cap. Pure file split; no behaviour change.

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
- `Cargo.toml` package description qualified: it stated "C-class performance,
  Python-class simplicity" as fact, while `README.md` says the C-class target is
  not yet measured (0.8 §2.2). It now reads "targeting C-class performance".
- `docs/specs/2026-11-10-yulia-open-decisions.md` D28 records the CLI driver
  surface (no-args usage, `--help`, `--version`, unknown option, unreadable
  input), closing the spec gap flagged in review.
- The oversized `src/` modules were split under the 150 SLOC cap (SOP-MOD);
  `src/lexer/token/kind.rs` is a documented exception (ADR 0002).

### Fixed

- The parser rejected a reserved keyword as a module-path component, so the
  spec's own example `import std.string as str;` (0.1 §4.2) failed with E102
  instead of reaching the type checker's E214. `parse_module_path` now accepts
  `IDENT` or a keyword per component and reports E122 (missing `as`) when an
  import ends in a keyword without an alias.
- `benchmarks/README.md` kernel example: the sample invocation used
  `kernels/k1.yl`, which cannot resolve because the harness runs each kernel
  with the repository root as the working directory. It now uses
  `benchmarks/kernels/k1.yl`, and the working-directory rule is stated.
- `benchmarks/README.md`: the K1 row stated the target as "within 5% of C",
  which is the stretch expectation; the binding requirement is 10% (spec §2.2,
  §3.1). The row now says so.
- `docs/specs/2026-11-10-yulia-open-decisions.md` D1: the OUT list said
  "modules/imports", which was ambiguous — the `module <path>;` header is
  required by 0.1 §3.1 and is IN, while `import` is OUT. Clarified at B's
  request (B implemented the split).
- `docs/specs/2026-11-09-yulia-grammar-draft.md` §10.7: `import_decl` and
  multi-module resolution are now listed as Phase 2 (D1); the `module <path>;`
  header is required and `import` is rejected in Phase 1. `README.md` and
  `CONTRIBUTING.md` §9 now name `docs/adr/` and `docs/operations/`.
- `docs/specs/2026-11-09-yulia-error-panic-model.md` §2: removed
  "assertion failure" from the Phase 1 runtime-error triggers — no Phase 1
  construct produces one (0.1 §2.5/§3.3/§10, D1, D2). Recorded as review-log
  finding F56.
- `docs/specs/2026-11-09-yulia-benchmark-methodology.md` §2.2: the target
  summary now matches the per-kernel sections §3.1–§3.5 (K1 within 5%, K4 at
  least 2× faster than C, K5 at least 1.5× faster than rustc) and the stale
  "(§3.3)" reference is corrected. Recorded as review-log finding F57.
- `docs/specs/2026-11-09-yulia-benchmark-methodology.md` §3.1: the K1 entry now
  states that the binding requirement is 10% (§2.2) and 5% is the stretch
  expectation, closing the F57 open question (user delegation, 2026-11-11).
- `docs/specs/2026-11-09-yulia-grammar-draft.md` §3.3: added a "Note on
  condition disambiguation" — `if a { }` is ambiguous (a struct literal could
  swallow the body), so struct-literal parsing is disabled in `if`/`while`
  conditions and `for` range bounds; such literals must be parenthesized.
  Recorded as review-log finding F58 (B's parser implementation, 2026-11-11).
- `docs/specs/2026-11-09-yulia-grammar-draft.md` §8: the "Open questions" list
  now marks each item resolved (with the deciding document) or open (with its
  register id) — array bounds checking, integer overflow, and default
  initialization were already decided but still listed as open. Recorded as
  review-log finding F59.
- `docs/specs/2026-11-09-yulia-operational-semantics.md` §7,
  `docs/specs/2026-11-09-yulia-region-inference.md` §10, and
  `docs/specs/2026-11-09-yulia-escape-analysis.md` §8: the "Open decisions"
  lists now mark each item resolved/scoped (with the deciding document) or
  open (with its register id), matching the 0.1 §8 convention. Recorded as
  review-log finding F60.
- `docs/specs/2026-11-09-yulia-operational-semantics.md` §6.2: the typing
  rules are now exhaustive for the Phase 1 subset (all binary/unary operators,
  assignment, `[e; N]`, and the statement/declaration forms) instead of the
  previous 9 "selected" rules. Recorded as review-log finding F61.
- `README.md`: the status badge read "Phase 0 specification" while the status
  table and prose already report Phase 0 complete and Phase 1a in progress.
  The badge now reads "Phase 1a front-end".
- `docs/specs/2026-11-09-yulia-error-panic-model.md` §2: added §2.1
  (diagnostic format: `error[<code>]` / `runtime error[<code>]`; exit 1 /
  abort 134) and §2.2 (the Phase 1 diagnostic catalog — E001–E007 lexer,
  E101–E124 parser, E201–E225 type checker, R0101–R0103 runtime — aligned with
  `src/diagnostics/diagnostic.rs`). `design.md` §R.1, `0.4` §9, and the D2 row
  in the open decisions register now point at the catalog. Recorded as
  review-log finding F62.
- `.editorconfig`: `*.{yml,yaml,json}` now use 2-space indentation, matching
  `.github/workflows/ci.yml`; TOML keeps 4 spaces (`deny.toml`). The previous
  rule lumped YAML in with TOML at 4 spaces.
- `benchmarks/run.sh`: a failing warm-up run now reports the kernel and exits,
  instead of dying silently under `set -e`.

### Removed

- The previous repository history and `docs/plans/` were lost when the `.git`
  directory disappeared on 2026-11-10. The specification under `docs/specs/`
  was unaffected and is the source of truth for the language.
