# Threat model

The attacker model, attack surface, and invariants for the Yulia compiler
repository. Written against the system model in
[`system-model.md`](system-model.md). Disclosure process:
[`SECURITY.md`](../../SECURITY.md).

## Attacker model

- **Hostile source.** Anyone can write a `.yulia` file. A program is untrusted
  input to the compiler, not code the compiler runs.
- **Supply chain.** A crate dependency, a GitHub Action, or the Rust toolchain
  can be compromised or malicious.
- **CI injection.** A pull request can attempt to modify workflow files, exfiltrate
  the `GITHUB_TOKEN`, or run arbitrary steps.
- **Maintainer account.** A compromised maintainer account can push directly
  (branch protection is admin-bypassable — see the operations runbook).

## Attack surface

| Surface | Entry point |
|---|---|
| Lexer, parser, type checker, region inference | Untrusted `.yulia` source |
| Codegen (Phase 1b) | Well-typed AST |
| `cargo` dependencies | `Cargo.toml` / `Cargo.lock` |
| CI | `.github/workflows/ci.yml`, action refs |
| Repository | Git history, tags, releases |

## Threats

| ID | Threat | Impact |
|---|---|---|
| T1 | Hostile input crashes the compiler (panic, stack overflow, OOM, integer overflow in the compiler) | Denial of service; a crash is a defect, not a diagnostic |
| T2 | Hostile input reaches undefined behavior in the compiler | Memory safety violation in the compiler itself |
| T3 | Compromised or malicious crate dependency | Arbitrary code execution at build time |
| T4 | CI workflow injection from a pull request | Token exfiltration, malicious artifacts |
| T5 | Over-privileged `GITHUB_TOKEN` | Repository or org compromise |
| T6 | Secret leakage through logs, diagnostics, or artifacts | Credential exposure |
| T7 | Miscompilation — well-typed program generates wrong code | Correctness and safety of compiled programs |
| T8 | Resource exhaustion by a hostile program (unbounded allocation in the compiler) | Host OOM; peer-impacting (SOP-RESLIM) |

## Invariants under attack

1. **The compiler never executes the program it compiles.** A `.yulia` file is
   data end to end.
2. **Hostile input produces a diagnostic, not a crash.** Every rejection path
   is a typed diagnostic with a stable code; no `unwrap`/`expect`/`panic`
   outside tests.
3. **No `unsafe` in the compiler.** Phase 1a is safe Rust; codegen (Phase 1b)
   is the only place `unsafe` is expected, and it is bounded by inkwell's
   invariants.
4. **Resource use is bounded.** Recursion depth, input size, and allocation
   are bounded; a hostile program cannot OOM the host (SOP-RESLIM).
5. **No secret is written to logs, diagnostics, or artifacts.**
6. **Dependencies are pinned and audited.** `Cargo.lock` is committed;
   `cargo deny` runs in CI; actions are pinned by commit SHA.

## Test strategy

At least one negative test per invariant:

| Invariant | Negative test |
|---|---|
| 1, 2 | Lexer/parser/type-checker tests feed malformed and adversarial input and assert a diagnostic, not a crash |
| 3 | Clippy `-D warnings`; no `unsafe` outside codegen |
| 4 | Bounded-input tests; resource limits in CI (`timeout-minutes`, `--test-threads=2`) |
| 5 | No secret material in the repository; CI logs are the only output |
| 6 | `cargo deny` in CI; pinned action SHAs |

## Defense in depth

- Pinned toolchain (`rust-toolchain.toml`) and committed `Cargo.lock`.
- `cargo deny` (licenses, advisories, bans) and `typos` in CI.
- Clippy with `-D warnings`; rustfmt enforced.
- Actions pinned by commit SHA; `GITHUB_TOKEN` scoped to `contents: read`.
- No deployment secrets in the repository.
- Branch protection with required status checks (admin-bypassable — recorded).

## Detection

- CI failures on `main` (`gh run list --branch main`).
- Dependabot alerts, secret scanning, and code scanning
  (`gh api repos/:owner/:repo/dependabot/alerts`).
- Dependency audit job in CI.

## Residual risk

- A bug in codegen's `unsafe` (Phase 1b) could violate invariant 3; it is
  bounded by inkwell's API and covered by differential tests.
- A compromised dependency that passes `cargo deny` and has no known advisory.
- A compromised maintainer account (admin bypass).

## Coordinated disclosure

Report privately per [`SECURITY.md`](../../SECURITY.md). Do not open a public
issue for an unmitigated vulnerability. Internal first: reproduce, quantify,
propose mitigation, then disclose.
