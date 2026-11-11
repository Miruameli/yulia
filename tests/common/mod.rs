//! Shared helpers for the lexer integration tests.

use yulia::diagnostics::Diagnostic;
use yulia::lexer::{Lexer, TokenKind};

/// Tokenizes `source` and returns the token kinds and diagnostics.
pub(crate) fn kinds(source: &str) -> (Vec<TokenKind>, Vec<Diagnostic>) {
    let (tokens, diagnostics) = Lexer::new(source).tokenize();
    (
        tokens.into_iter().map(|token| token.kind).collect(),
        diagnostics,
    )
}

/// Asserts that no diagnostics were produced.
pub(crate) fn assert_clean(diags: &[Diagnostic]) {
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");
}
