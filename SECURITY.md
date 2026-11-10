# Security Policy

## Supported Versions

Yulia does not ship pre-releases: only stable versions are released
([versioning policy](docs/specs/2026-11-09-yulia-versioning-policy.md) §2).
Security fixes are applied to the latest stable release and, when feasible, to
the previous stable release.

| Version | Supported |
|---|---|
| latest stable | yes |
| previous stable | best effort |
| development builds | no |

## Reporting a Vulnerability

Report suspected vulnerabilities privately. Do **not** open a public issue for
a security report before a fix is available.

- **Preferred:** open a private security advisory at
  <https://github.com/Miruameli/yulia/security/advisories/new>.
- **Fallback:** email the maintainers at
  miruamelizabethelrathvonalwood@gmail.com.

Please include:

1. A description of the issue and its impact.
2. Steps to reproduce, or a proof of concept.
3. Affected version(s) and platform(s).
4. Any suggested mitigation.

You can expect an acknowledgement within a reasonable period and a status
update as the report is triaged. If you have not received a response, contact
the maintainers again rather than disclosing publicly.

## Scope

In scope:

- The compiler driver, lexer, parser, type checker, region inference, and
  codegen.
- The standard library once it exists.
- Build, test, benchmark, and release tooling in this repository.

Out of scope:

- Third-party dependencies (report these to their own maintainers; see below).
- Issues already disclosed publicly.
- Theoretical concerns with no demonstrable impact.

## Dependency Policy

The compiler is intentionally dependency-free in Phase 1a (Phase 1b adds
inkwell for codegen). Dependencies are added deliberately and reviewed before
introduction:

- Every dependency is pinned and covered by a lockfile.
- `cargo deny` checks licenses, advisories, and duplicate versions in CI.
- Advisories affecting pinned versions are treated as high-priority work.

## Build and Release Integrity

- Releases are cut from a clean checkout of the default branch after all
  quality gates pass.
- Release artifacts are accompanied by SHA-256 checksums and a software bill of
  materials (SBOM).
- Pre-releases (alpha, beta, rc, nightly, canary) are never published.

## Disclosure

Coordinated disclosure: the maintainers are notified first, a fix is prepared,
and the issue is disclosed only after a fix is available or after an agreed
period. The reporter is credited unless they prefer anonymity.
