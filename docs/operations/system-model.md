# System model

The Yulia compiler as a pipeline, and the repository around it. This is the
model the threat model ([`threat-model.md`](threat-model.md)) and the
operations runbook ([`README.md`](README.md)) are written against.

## Compiler pipeline

```
source (.yulia)
  → lexer      tokens + lexer diagnostics (E0xx)
  → parser     AST + parser diagnostics (E1xx)
  → typeck     type-checked AST + type diagnostics (E2xx)
  → region     region inference (Phase 1a: by-value subset; E3xx reserved)
  → codegen    LLVM IR → object → executable (Phase 1b, inkwell)
```

Each stage consumes the previous stage's output and emits diagnostics with a
stable code (`docs/specs/2026-11-09-yulia-error-panic-model.md` §2). A stage
never executes the program it is compiling.

Phase 1a is the front end (lexer, parser, AST, type checker, region inference,
diagnostics, tests). Phase 1b adds the codegen backend. The subset boundary is
D1 in `docs/specs/2026-11-10-yulia-open-decisions.md`.

## Crate layout

| Path | Responsibility |
|---|---|
| `src/lexer/` | Tokeniser and token kinds |
| `src/parser/` | Recursive-descent parser and AST construction |
| `src/ast/` | AST node types |
| `src/typeck/` | Type checker and name resolution |
| `src/diagnostics/` | Diagnostic type and the stable code catalog |
| `src/main.rs` | CLI driver |
| `tests/` | Integration tests |

## Driver

`src/main.rs` is the CLI. It reads a source file, runs the pipeline, prints
diagnostics to stderr, and exits 1 on a compile error. Runtime errors are
raised by the generated program (Phase 1b), not by the driver.

## Tests

Unit tests live next to their module; integration tests live in `tests/`.
`tests/driver.rs` exercises the CLI. CI runs the full suite with
`--test-threads=2` (SOP-RESLIM).

## CI

`.github/workflows/ci.yml` runs seven jobs on every push to `main` and on every
pull request: Formatting, Clippy, Tests, Build, Dependency audit, Spell check,
Codegraph index. Every job and step carries a `timeout-minutes`. See
[`README.md`](README.md) §CI.

## Trust boundaries

1. **Untrusted source → compiler.** A `.yulia` file is untrusted input. The
   compiler must reject it with a diagnostic, never crash or invoke undefined
   behavior.
2. **Repository → CI runner.** Workflow files and actions are trusted; `pull
   request` triggers run with read-only token.
3. **Compiler → generated code.** Codegen must not introduce undefined behavior
   for a well-typed program.
