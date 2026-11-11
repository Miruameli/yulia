//! Lexical tokens (0.1 §2.5–§2.7).

mod kind;

use crate::diagnostics::Span;

pub use kind::TokenKind;

/// A single lexical token: its kind, its source text, and its span.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'a> {
    /// The lexical category.
    pub kind: TokenKind,
    /// The exact source text of the token.
    pub text: &'a str,
    /// The source range the token covers.
    pub span: Span,
}
