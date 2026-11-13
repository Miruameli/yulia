# Operations

Runbook for the Yulia repository: CI, resource limits, and incident response.

## CI

`.github/workflows/ci.yml` runs on every push to `main` and on every pull
request. Seven jobs:

| Job | What it does | Timeout |
|---|---|---|
| Formatting | `cargo fmt --all -- --check` | 10 min |
| Clippy | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | 10 min |
| Tests | `cargo test --workspace --all-features -- --test-threads=2` | 10 min |
| Build | `cargo build --workspace --all-features --release` | 15 min |
| Dependency audit | `cargo deny --config deny.toml check` | 20 min |
| Spell check | `typos` | 20 min |
| Codegraph index | `codegraph init --yes` | 15 min |

Every job and every step carries an explicit `timeout-minutes`. `concurrency`
cancels superseded runs on the same ref.

## Resource limits

SOP-RESLIM bounds every heavy command:

- `CARGO_BUILD_JOBS=2` (Makefile and CI) — `min(2, nproc/2)`.
- Test harness: `--test-threads=2`.
- Makefile targets wrap heavy work in `timeout -k 5 <seconds>` (`make test`
  900 s, `make lint` 600 s, `make benchmark` 1800 s).
- CI runners: `ubuntu-24.04` (4 vCPU / 7 GB).

## OOM response

Symptoms: exit 137, "Killed", "Cannot allocate memory", or a job cancelled
with no test output.

1. **Stop.** Do not retry the same command.
2. **Diagnose.** Check the kernel log for the OOM killer and the job log for
   the last command that ran.
3. **Reduce.** Lower parallelism (`CARGO_BUILD_JOBS=1`, `--test-threads=1`),
   split the work, or run it on a larger runner.
4. **Guard.** Add a timeout or a memory bound so the failure stays bounded.
5. **Notify peers** if the host is shared (`pi_messenger` `send`).

## Release

Releases are cut from a clean checkout of `main` after all quality gates pass
(SECURITY.md). The versioning policy is
`docs/specs/2026-11-09-yulia-versioning-policy.md`; no pre-releases are
published.

## Secrets

The repository holds no deployment secrets. CI uses only the implicit
`GITHUB_TOKEN` (`contents: read`).

## Repository settings

- **Branch protection (`main`):** seven required status checks (Formatting,
  Clippy, Tests, Build, Dependency audit, Spell check, Codegraph index),
  `strict=true` (the branch must be up to date), and `enforce_admins=true` —
  admins go through a pull request like everyone else. Verify with
  `gh api repos/Miruameli/yulia/branches/main/protection` (SOP-GH.1).
  `required_approving_review_count` is **0**: the repository has a single
  maintainer account, so a required review would deadlock every pull request
  (an author cannot approve their own). The seven required checks are the gate;
  review happens through `pi_messenger` (`send`/`broadcast`) and the pull request thread.
- **Merge options:** squash-only (`allow_merge_commit` and `allow_rebase_merge`
  are disabled) with `delete_branch_on_merge=true`. Pull requests are squashed
  into `main` with a Conventional Commit message (SOP-GHOP.2).
- **Dependabot:** `.github/dependabot.yml` schedules weekly version updates for
  Cargo and GitHub Actions; security updates are enabled at the repository level
  (SOP-GHOP.9). Verify with `gh api repos/Miruameli/yulia/dependabot/alerts`.
- **Discussions:** enabled — usage questions, design discussion, and ideas
  belong there, not in issues.
- **Issue templates:** `.github/ISSUE_TEMPLATE/` (bug report, feature request,
  documentation); `blank_issues_enabled: false`.
- **CODEOWNERS:** `.github/CODEOWNERS` — `@Miruameli` owns the repository; the
  spec and docs paths are called out explicitly.
- **Labels:** the GitHub defaults plus `spec`, `security`, `tooling`, and
  `codegen`.

## Observability

The repository has no deployed service, so there is no runtime metrics
surface. Build and test health is observed through GitHub Actions:
`gh run list --branch main --limit 20` (SOP-GH.1). A red run on `main` is the
signal; the run log is the diagnostic.

## Rollback

- **Commits:** `git revert <sha>` on `main`, then push. History is never
  rewritten on `main`.
- **Releases:** a bad tag is deleted and re-cut from the previous green
  commit; no pre-releases are published (0.9 §3).
- **CI:** a broken workflow is reverted the same way; `concurrency` cancels
  superseded runs.

## Deployment and backup

Not applicable: the project ships a compiler crate and its documentation, not
a deployed service. The git repository (with GitHub as the remote) is the
backup; there is no state to restore.
