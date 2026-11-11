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
    // Parser (E1xx).
    /// A `module` declaration is missing (E101).
    pub const EXPECTED_MODULE: Code = Code("E101");
    /// An identifier is missing (E102).
    pub const EXPECTED_IDENT: Code = Code("E102");
    /// A `;` is missing (E103).
    pub const EXPECTED_SEMICOLON: Code = Code("E103");
    /// A `{` is missing (E104).
    pub const EXPECTED_LBRACE: Code = Code("E104");
    /// A `}` is missing (E105).
    pub const EXPECTED_RBRACE: Code = Code("E105");
    /// A `(` is missing (E106).
    pub const EXPECTED_LPAREN: Code = Code("E106");
    /// A `)` is missing (E107).
    pub const EXPECTED_RPAREN: Code = Code("E107");
    /// A `[` is missing (E108).
    pub const EXPECTED_LBRACKET: Code = Code("E108");
    /// A `]` is missing (E109).
    pub const EXPECTED_RBRACKET: Code = Code("E109");
    /// A `:` is missing (E110).
    pub const EXPECTED_COLON: Code = Code("E110");
    /// A `,` is missing (E111).
    pub const EXPECTED_COMMA: Code = Code("E111");
    /// A `.` is missing (E112).
    pub const EXPECTED_DOT: Code = Code("E112");
    /// A `..` is missing (E113).
    pub const EXPECTED_DOTDOT: Code = Code("E113");
    /// A `->` is missing (E114).
    pub const EXPECTED_ARROW: Code = Code("E114");
    /// An `=` is missing (E115).
    pub const EXPECTED_EQ: Code = Code("E115");
    /// An expression is missing (E116).
    pub const EXPECTED_EXPRESSION: Code = Code("E116");
    /// A type is missing (E117).
    pub const EXPECTED_TYPE: Code = Code("E117");
    /// A statement is missing (E118).
    pub const EXPECTED_STATEMENT: Code = Code("E118");
    /// An integer literal is missing (E119).
    pub const EXPECTED_INT_LIT: Code = Code("E119");
    /// A string literal is missing (E120).
    pub const EXPECTED_STRING_LIT: Code = Code("E120");
    /// A token that cannot appear here (E121).
    pub const UNEXPECTED_TOKEN: Code = Code("E121");
    /// An `as` is missing (E122).
    pub const EXPECTED_AS: Code = Code("E122");
    /// An `in` is missing (E123).
    pub const EXPECTED_IN: Code = Code("E123");

    /// A keyword is missing (E124).
    pub const EXPECTED_KEYWORD: Code = Code("E124");
}
