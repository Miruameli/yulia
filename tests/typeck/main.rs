//! Type checker integration tests (0.2 §6).

mod decl;
mod decl_fn;
mod expr;
mod expr_access;
mod expr_call;
mod stmt;
mod subset;

use yulia::diagnostics::Diagnostic;
use yulia::lexer::Lexer;
use yulia::parser::Parser;
use yulia::typeck::Checker;

/// Parses and type checks a whole program.
pub(crate) fn check(source: &str) -> Vec<Diagnostic> {
    let (tokens, lex) = Lexer::new(source).tokenize();
    assert!(lex.is_empty(), "unexpected lexer diagnostics: {lex:?}");
    let (program, parse) = Parser::new(&tokens).parse();
    assert!(parse.is_empty(), "unexpected parser diagnostics: {parse:?}");
    Checker::new(&program).check()
}

/// Returns the diagnostic codes, in order.
pub(crate) fn codes(diagnostics: &[Diagnostic]) -> Vec<&'static str> {
    diagnostics.iter().map(|d| d.code.0).collect()
}

/// Asserts that `source` type checks with no diagnostics.
pub(crate) fn accepts(source: &str) {
    let diagnostics = check(source);
    assert!(
        diagnostics.is_empty(),
        "unexpected diagnostics: {diagnostics:?}"
    );
}

/// Asserts that `source` produces exactly `expected` codes, in order.
pub(crate) fn rejects(source: &str, expected: &[&str]) {
    let diagnostics = check(source);
    assert_eq!(codes(&diagnostics), expected, "in {source}");
}

/// Builds a program whose `main` runs `body`.
pub(crate) fn main_program(body: &str) -> String {
    format!("module app.main;\nfn main() -> void {{\n{body}\n}}\n")
}
