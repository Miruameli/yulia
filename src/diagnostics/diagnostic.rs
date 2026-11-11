//! Compile-time diagnostics: stable codes, messages, and spans.

use crate::diagnostics::span::Span;

/// A stable diagnostic code (D2).
///
/// Codes are part of the compiler's public contract: once a code ships it must
/// not be reused for a different condition. The catalog is
/// `docs/specs/2026-11-09-yulia-error-panic-model.md` §2.2.
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
