//! Literal scanning: numbers, characters, and strings (0.1 §2.6).

mod decode;

mod digits;
mod escape;
mod text;
pub(crate) use decode::{decode_char, decode_string};

use crate::diagnostics::Diagnostic;
use crate::lexer::Lexer;
use crate::lexer::token::TokenKind;

impl Lexer<'_> {
    /// Lexes a numeric literal, returning [`TokenKind::IntLit`] or
    /// [`TokenKind::FloatLit`].
    pub(in crate::lexer) fn lex_number(&mut self, diagnostics: &mut Vec<Diagnostic>) -> TokenKind {
        let start = self.pos;
        let mut is_float = false;

        if self.peek() == Some('0') {
            let radix = match self.peek_at(1) {
                Some('x') => Some(16),
                Some('b') => Some(2),
                Some('o') => Some(8),
                _ => None,
            };
            if let Some(radix) = radix {
                self.bump();
                self.bump();
                if !self.consume_digits(radix, diagnostics) {
                    diagnostics.push(Diagnostic::new(
                        digits::INVALID_NUMERIC_LITERAL,
                        "radix-prefixed literal has no digits",
                        crate::diagnostics::Span::new(start, self.pos),
                    ));
                }
                return TokenKind::IntLit;
            }
        }

        self.consume_decimal(diagnostics);

        if self.peek() == Some('.') && self.peek_at(1).is_some_and(|c| c.is_ascii_digit()) {
            is_float = true;
            self.bump();
            self.consume_decimal(diagnostics);
        }

        if self.peek() == Some('e') && self.exponent_follows() {
            is_float = true;
            self.bump();
            if matches!(self.peek(), Some('+' | '-')) {
                self.bump();
            }
            self.consume_decimal(diagnostics);
        }

        if is_float {
            TokenKind::FloatLit
        } else {
            TokenKind::IntLit
        }
    }
}
