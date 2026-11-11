//! Compile-time diagnostics: stable codes, messages, and spans.

use crate::diagnostics::span::Span;

/// A stable diagnostic code (D2).
///
/// Codes are part of the compiler's public contract: once a code ships it must
/// not be reused for a different condition. The ranges are reserved by stage:
/// `E0xx` lexer, `E1xx` parser, `E2xx` type checker, `E3xx` region inference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Code(pub &'static str);

/// A single compile-time diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    /// The stable code identifying the diagnostic class.
    pub code: Code,
    /// The human-readable message.
    pub message: String,
    /// The source range the diagnostic refers to.
    pub span: Span,
}

impl Diagnostic {
    /// Creates a diagnostic from a code, a message, and a span.
    #[must_use]
    pub fn new(code: Code, message: impl Into<String>, span: Span) -> Self {
        Self {
            code,
            message: message.into(),
            span,
        }
    }
}

/// Stable diagnostic codes, grouped by compiler stage.
pub mod code {
    use super::Code;

    /// A character that cannot start a token (E001).
    pub const UNEXPECTED_CHAR: Code = Code("E001");
    /// A block comment that is not closed before end of input (E002).
    pub const UNTERMINATED_BLOCK_COMMENT: Code = Code("E002");
    /// A string literal that is not closed before end of input (E003).
    pub const UNTERMINATED_STRING: Code = Code("E003");
    /// A character literal that is not closed before end of input (E004).
    pub const UNTERMINATED_CHAR: Code = Code("E004");
    /// An escape sequence that is not part of the grammar (E005).
    pub const INVALID_ESCAPE: Code = Code("E005");
    /// A numeric literal with misplaced underscores or no digits (E006).
    pub const INVALID_NUMERIC_LITERAL: Code = Code("E006");
    /// A character literal that is empty or holds more than one character (E007).
    pub const INVALID_CHAR_LITERAL: Code = Code("E007");
}
