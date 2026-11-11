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
- CI runners: `ubuntu-latest` (4 vCPU / 7 GB).

## OOM response

Symptoms: exit 137, "Killed", "Cannot allocate memory", or a job cancelled
with no test output.

1. **Stop.** Do not retry the same command.
2. **Diagnose.** Check the kernel log for the OOM killer and the job log for
   the last command that ran.
3. **Reduce.** Lower parallelism (`CARGO_BUILD_JOBS=1`, `--test-threads=1`),
   split the work, or run it on a larger runner.
4. **Guard.** Add a timeout or a memory bound so the failure stays bounded.
5. **Notify peers** if the host is shared (`coord_send`).

## Release

Releases are cut from a clean checkout of `main` after all quality gates pass
(SECURITY.md). The versioning policy is
`docs/specs/2026-11-09-yulia-versioning-policy.md`; no pre-releases are
published.

## Secrets

The repository holds no deployment secrets. CI uses only the implicit
`GITHUB_TOKEN` (`contents: read`).
