# Contributing to Yulia

Standards for the Yulia compiler repository: building, formatting, linting,
testing, reviewing, and committing.

## 1. Prerequisites

| Tool | Required | Notes |
|---|---|---|
| Rust toolchain | yes | Pinned in `rust-toolchain.toml`; `rustup` installs it on the first `cargo` call |
| LLVM 20 | yes | Phase 1b links through inkwell, which needs `llvm-config`. Install `llvm-20-dev` and `libpolly-20-dev` from apt.llvm.org (see `.github/actions/install-llvm20/action.yml`; llvm-sys links statically and needs `libPolly.a`) and export `LLVM_SYS_201_PREFIX` when `llvm-config` is not on `PATH` |
| `codegraph` | yes | Repository knowledge graph; needed before non-trivial code changes |
| `cargo-nextest` | no | Faster test runner (`make test-nextest`) |
| `cargo-deny` | no | Dependency licence/advisory audit (`cargo deny check`) |
| `cargo-llvm-cov` | no | Coverage (`make coverage`) |
| `typos-cli` | no | Spell checking (`typos`) |

## 2. Quick start

```sh
make help        # list every target
make fmt         # format
make lint        # clippy, warnings denied
make test        # test suite
make ci          # fast local checks (no extra tools)
```

## 3. Common tasks

| Target | Purpose |
|---|---|
| `make fmt` | Format all Rust code |
| `make fmt-check` | Verify formatting without writing |
| `make lint` | `clippy --workspace --all-targets --all-features -- -D warnings` |
| `make test` | `cargo test --workspace --all-features` |
| `make test-nextest` | Same suite through cargo-nextest |
| `make benchmark` | Run the benchmark suite (spec §4.3) |
| `make coverage` | LLVM coverage report into `coverage/` |
| `make codegraph` | Rebuild the codegraph index from scratch |
| `make codegraph-sync` | Incrementally update the codegraph index |
| `make doc` | Build rustdoc |
| `make ci` | `fmt-check` + `lint` + `test` + `build` |
| `make ci-full` | `ci` + `deny` + `typos` + `codegraph` (needs `cargo-deny`, `typos-cli`) |

## 4. Code standards

### 4.1 Formatting

- `cargo fmt` before every commit; `rustfmt.toml` is the single source of truth.
- 100-column limit, 4-space indent, LF line endings, one final newline.
- Only stable rustfmt options are configured. Nightly-only options
  (`imports_granularity`, `group_imports`, `wrap_comments`, …) are
  deliberately **not** used so that CI and contributor machines agree.

### 4.2 Lints

Configured in `Cargo.toml` (`[lints]`) and `clippy.toml`.

- `clippy::all` is **denied**; `clippy::pedantic` is warn-level and CI denies
  warnings, so pedantic lints are effectively errors too.
- `unsafe_code` is **forbidden**.
- `unwrap_used`, `expect_used`, `panic`, `todo`, `unimplemented`, and
  `dbg_macro` are **denied** outside tests (`clippy.toml` re-allows them in
  test code).
- `missing_docs` is warn-level: every public item is documented.

### 4.3 Naming

- Modules and functions: `snake_case`. Types and traits: `PascalCase`.
  Constants: `SCREAMING_SNAKE_CASE`.
- Folder names are descriptive and kebab-case. Never `utils/`, `helpers/`,
  `common/`, `misc/`, `temp/`, `old/`, `new/`, or `backup/`.

### 4.4 Documentation and comments

- Doc comments on every public item; comments explain **why**, not what.
- Maximum 100 columns. No commented-out code.
- `TODO`/`FIXME`/`HACK` must reference a tracked issue.

### 4.5 Errors

- The compiler reports errors; it does not panic at the user.
- Recoverable failures return `Result`; diagnostics carry file, line, column,
  and a stable code (`docs/specs/2026-11-09-yulia-error-panic-model.md` §2.1–2.2).
- Runtime errors abort with a non-zero exit code.

### 4.6 Module limits

- At most 150 SLOC per file, 5 direct files per folder, 5 subfolders per folder.
- No circular dependencies; no layer skipping.

## 5. Testing

- Every logic change ships with tests. Tests cover error paths and boundaries,
  not just the happy path.
- At least one **negative test per invariant**
  (`docs/specs/2026-11-09-yulia-design-review.md` §6.3).
- Use `proptest` for the lexer and parser: tokenisation and parsing are
  adversarial surfaces. It is not yet a dependency — the Phase 1a runtime is
  std-only (`Cargo.toml`) — so add it to `[dev-dependencies]` when the first
  property test lands.
- `make test` locally; the full suite also runs in CI on every push and pull request.

## 6. Benchmarks

- `make benchmark` runs the suite in `benchmarks/`.
- Methodology: `docs/specs/2026-11-09-yulia-benchmark-methodology.md`.
- A regression greater than 5% on any kernel blocks the change (spec §4.4).
- `-march=native` is prohibited: it breaks reproducibility.

## 7. Commits

[Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>          # subject <= 72 chars, imperative

<body>                              # WHY and WHAT, not a diff dump

<footer>                            # issue references
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`,
`ci`, `chore`, `revert`.

Never commit secrets, credentials, or generated artifacts.

### 7.1 Pull request workflow

`main` is protected: seven required status checks, `strict=true`, and
`enforce_admins=true`. Direct pushes to `main` are rejected.

```sh
git checkout -b <branch>
git add <explicit paths>          # never `git add -A`
git commit -m '<type>(<scope>): <subject>'
git push -u origin <branch>
gh pr create --base main --head <branch> --title '...' --body '...'
# wait for the seven checks, then:
gh pr merge <n> --squash --delete-branch
```

Fill the pull request template (Summary, Type of change, Checklist,
Verification). `required_approving_review_count` is 0 because the repository has
a single maintainer account; the seven checks are the gate, and review happens in
the pull request thread and over `pi_messenger` (`send`/`broadcast`).

## 8. Review

Every change receives **two independent review passes** before it is merged.

1. **Correctness** — does it do what it claims, including error paths?
2. **Standards** — formatting, lints, tests, docs, security.

### 8.1 Review checklist

- [ ] `make ci` passes locally.
- [ ] Formatting and lints are clean; no new warnings.
- [ ] Tests cover the change, including failure modes.
- [ ] Every public item is documented.
- [ ] No `unwrap`/`expect`/`panic` outside tests; no new `unsafe`.
- [ ] `CHANGELOG.md` and docs updated in the same change.
- [ ] No secrets or generated files committed.
- [ ] Specification references are current.

### 8.2 Filing issues and discussions

- Defects and feature requests go to
  [issues](https://github.com/Miruameli/yulia/issues); use the template that
  matches (bug report, feature request, documentation).
- Usage questions, design discussion, and ideas go to
  [Discussions](https://github.com/Miruameli/yulia/discussions).
- Security reports go through [SECURITY.md](SECURITY.md), not a public issue.
- Every `TODO`/`FIXME`/`HACK` must reference a tracked issue (§4.4).

## 9. Specification documents

- Location: `docs/specs/` (language specification), `docs/adr/` (architecture
  decision records), `docs/operations/` (operations runbook), `docs/runbook/`
  (developer runbook).
- Naming: `YYYY-MM-DD-yulia-<slug>.md`.
- Required header fields: `Date`, `Status`, `Depends on`, `Plan ref`.
- Every spec ends with an `## N. Acceptance` checklist and, for deliverables
  0.2–0.9, an `## N. Deliberation (SOP-THINK)` table with the fields Goal,
  Assumptions, Evidence, Alternatives, Chosen, Risks, Mitigations,
  Confidence, Kill-criteria, and Verification plan.
- **Language changes require a specification change first.**

## 10. Versioning and releases

- [Semantic Versioning](https://semver.org/); no pre-releases. Development
  builds are identified by git commit hash.
- See `docs/specs/2026-11-09-yulia-versioning-policy.md`.

## 11. Continuous integration

`.github/workflows/ci.yml` runs formatting, clippy, tests, build, dependency
audit, spell checking, and the codegraph index on every push and pull request.
CodeQL code scanning (GitHub's default setup) runs separately on every push and
pull request; Dependabot alerts are enabled for dependency advisories.

> The remote is `https://github.com/Miruameli/yulia`, so GitHub Actions runs on
> every push to `main` and on every pull request. `make ci` runs the same fast
> checks locally.

## 12. Resource limits (SOP-RESLIM)

Every heavy command runs with an explicit bound. The contract (v3.9, INV-35 /
SOP-RESLIM) makes resource discipline mandatory; resource exhaustion is a
defect.

| Surface | Bound |
|---|---|
| `make test`, `make test-nextest` | 900 s suite |
| `make build`, `make lint`, `make doc`, `make codegraph` | 600 s |
| `make coverage` | 900 s |
| `make deny`, `make typos` | 300 s |
| `make benchmark` | 1800 s suite; 300 s per kernel run (`KERNEL_TIMEOUT`) |
| CI (`.github/workflows/ci.yml`) | `timeout-minutes` on every job and every step |

- The Makefile wraps heavy targets in `$(TIMEOUT)` (`timeout -k 5 <seconds>`).
  Requires GNU coreutils — on macOS, `brew install coreutils` and pass
  `TIMEOUT=gtimeout`. Disable per invocation with `make <target> TIMEOUT=`.
- `benchmarks/run.sh` bounds each kernel run and fails the run if a kernel
  exceeds `KERNEL_TIMEOUT` (default 300 s).
- Before running anything heavier than the above (a long build, a large test
  run, a network fetch), set a timeout and coordinate with peers on a shared
  host (`pi_messenger` `send`). Network calls: 30 s.
