# 2. Token kind enum size exception

- **Date:** 2026-11-11
- **Status:** Accepted
- **Deciders:** A (spec), B (implementation)
- **Related:** `docs/specs/2026-11-09-yulia-grammar-draft.md` §2.5–§2.7,
  `KONTRAK.md` §46 SOP-MOD

## Context

SOP-MOD caps a file at 150 SLOC. `src/lexer/token/kind.rs` holds the
`TokenKind` enum, which is the closed set of lexical categories defined by the
grammar: 39 keywords (§2.5), 6 literal forms (§2.6), 15 operators and 13
delimiters (§2.7), plus `Eof` — 73 variants. The crate denies `missing_docs`, so
every variant carries a one-line doc comment giving its exact source spelling.
The enum is therefore 159 lines: 146 for the documented variants (73 variants
plus one doc line each), 3 for the derive/header/close, and 10 for the module
doc, the type doc, the four section comments, and the blank line.

Every other oversized file was split without changing its API. This one cannot
be: the variants are one closed set, and the lexer, parser, and tests all match
on `TokenKind` directly.

## Decision

`TokenKind` is a documented exception to the 150 SLOC cap. The file stays whole
and the enum stays flat.

## Consequences

- `src/lexer/token/kind.rs` is the only file above the cap; it is recorded here
  rather than split.
- Adding a keyword, literal, operator, or delimiter to the grammar adds one
  documented variant (2 lines) to this file. A grammar revision that pushes the
  enum past roughly 200 lines should revisit this decision.
- The crate's `missing_docs` denial is what makes the per-variant docs
  mandatory; removing it was rejected because the spelling of each token is
  part of the lexical contract (§2.5–§2.7).

## Alternatives considered

- **Group the variants into nested enums** (`TokenKind::Keyword(Keyword)`,
  `TokenKind::Operator(Operator)`, …). Rejected: it fragments one closed set and
  rewrites every match site in the lexer, parser, and tests for no semantic gain.
- **Drop the per-variant docs.** Rejected: `missing_docs` is denied crate-wide,
  and the docs are the only place the exact spelling of each token is recorded.
- **Compress each doc onto the variant line.** Rejected: rustfmt reformats it,
  and it hurts readability.
