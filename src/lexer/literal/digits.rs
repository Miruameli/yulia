//! Digit consumption shared by decimal and radix literals (0.1 §2.6).

use crate::diagnostics::diagnostic::code;
use crate::diagnostics::{Diagnostic, Span};
use crate::lexer::Lexer;

/// Re-exported so the sibling modules can build numeric diagnostics.
pub(in crate::lexer) use code::INVALID_NUMERIC_LITERAL;

impl Lexer<'_> {
    /// Consumes decimal digits with `_` separators (0.1 §2.6).
    ///
    /// Returns `true` if at least one digit was consumed.
    pub(in crate::lexer) fn consume_decimal(&mut self, diagnostics: &mut Vec<Diagnostic>) -> bool {
        let mut any = false;
        loop {
            match self.peek() {
                Some(c) if c.is_ascii_digit() => {
                    any = true;
                    self.bump();
                }
                Some('_') => {
                    if self.peek_at(1).is_some_and(|c| c.is_ascii_digit()) {
                        self.bump();
                    } else {
                        self.misplaced_underscore(diagnostics);
                    }
                }
                _ => break,
            }
        }
        any
    }

    /// Consumes digits in `radix` with `_` separators (0.1 §2.6).
    ///
    /// Returns `true` if at least one digit was consumed.
    pub(in crate::lexer) fn consume_digits(
        &mut self,
        radix: u32,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> bool {
        let mut any = false;
        loop {
            match self.peek() {
                Some(c) if c.is_digit(radix) => {
                    any = true;
                    self.bump();
                }
                Some('_') => {
                    if self.peek_at(1).is_some_and(|c| c.is_digit(radix)) {
                        self.bump();
                    } else {
                        self.misplaced_underscore(diagnostics);
                    }
                }
                _ => break,
            }
        }
        any
    }

    /// Reports an underscore that is not followed by a digit.
    fn misplaced_underscore(&mut self, diagnostics: &mut Vec<Diagnostic>) {
        diagnostics.push(Diagnostic::new(
            INVALID_NUMERIC_LITERAL,
            "underscore in a numeric literal must be followed by a digit",
            Span::new(self.pos, self.pos + 1),
        ));
        self.bump();
    }

    /// Returns `true` if an exponent (with at least one digit) follows the `e`.
    pub(in crate::lexer) fn exponent_follows(&self) -> bool {
        let mut chars = self.source[self.pos + 1..].chars();
        match chars.next() {
            Some('+' | '-') => chars.next().is_some_and(|c| c.is_ascii_digit()),
            Some(c) => c.is_ascii_digit(),
            None => false,
        }
    }
}
