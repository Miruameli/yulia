//! Parser integration tests (0.1 §3).

#[path = "../common/mod.rs"]
mod common;

mod expr;
mod program;
mod stmt;

use yulia::ast::{DeclKind, FnDecl, Program};
use yulia::diagnostics::Diagnostic;
use yulia::lexer::Lexer;
use yulia::parser::Parser;

/// Parses `source`, asserting that the lexer produced no diagnostics.
pub(crate) fn parse(source: &str) -> (Program, Vec<Diagnostic>) {
    let (tokens, lex_diags) = Lexer::new(source).tokenize();
    assert!(
        lex_diags.is_empty(),
        "unexpected lexer diagnostics: {lex_diags:?}"
    );
    Parser::new(&tokens).parse()
}

/// Returns the single function declaration of a program.
///
/// Test helper: a non-function declaration means the test itself is wrong.
#[allow(clippy::panic)]
pub(crate) fn only_fn(program: &Program) -> &FnDecl {
    let DeclKind::Fn(f) = &program.decls[0].kind else {
        panic!(
            "expected a function declaration, got {:?}",
            program.decls[0].kind
        );
    };
    f
}
