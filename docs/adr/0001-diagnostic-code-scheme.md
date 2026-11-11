# 1. Diagnostic code scheme

- **Date:** 2026-11-11
- **Status:** Accepted
- **Deciders:** A (spec), B (implementation)
- **Related:** `docs/specs/2026-11-09-yulia-error-panic-model.md` §2,
  `docs/specs/2026-11-10-yulia-open-decisions.md` D2

## Context

D2 promised a "stable error code per diagnostic" and the format
`file:line:col: error: <msg>`, but no spec enumerated the codes. The
implementation (`src/diagnostics/diagnostic.rs`) already reserved ranges by
compiler stage: `E0xx` lexer, `E1xx` parser, `E2xx` type checker, `E3xx` region
inference.

## Decision

Codes are grouped by compiler stage:

| Range | Stage |
|---|---|
| `E0xx` | lexer |
| `E1xx` | parser |
| `E2xx` | type checker |
| `E3xx` | region inference (reserved, Phase 2+) |
| `R0xx` | runtime |

The format embeds the code: `error[<code>]` for compile-time diagnostics and
`runtime error[<code>]` for runtime traps. Once a code ships it is stable.

## Consequences

- The spec (0.5 §2.2) and the implementation agree; the catalog is the single
  source of truth.
- Adding a diagnostic requires a code from the stage's range, and the spec is
  updated in the same change.
- `E3xx` is reserved for region inference, which lands in Phase 2+.

## Alternatives considered

- A coarse `E01xx`–`E06xx` scheme grouped by phase (lexer, parser, name
  resolution, type checker, control flow, program structure). Rejected: it did
  not match the implementation, which was already more granular and stage-based.
