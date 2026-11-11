//! Character and string literals (0.1 §2.6).

use crate::diagnostics::diagnostic::code;
use crate::diagnostics::{Diagnostic, Span};
use crate::lexer::Lexer;

impl Lexer<'_> {
    /// Lexes a character literal (0.1 §2.6).
    pub(in crate::lexer) fn lex_char(&mut self, diagnostics: &mut Vec<Diagnostic>) {
        let start = self.pos;
        self.bump();
        match self.peek() {
            None => self.unterminated_char(start, diagnostics),
            Some('\\') => {
                self.bump();
                self.lex_escape(diagnostics);
                if self.peek() == Some('\'') {
                    self.bump();
                } else {
                    self.unterminated_char(start, diagnostics);
                }
            }
            Some('\'') => {
                self.bump();
                diagnostics.push(Diagnostic::new(
                    code::INVALID_CHAR_LITERAL,
                    "character literal is empty",
                    Span::new(start, self.pos),
                ));
            }
            Some(_) => {
                self.bump();
                if self.peek() == Some('\'') {
                    self.bump();
                } else {
                    self.unterminated_char(start, diagnostics);
                }
            }
        }
    }

    /// Lexes a string literal (0.1 §2.6).
    pub(in crate::lexer) fn lex_string(&mut self, diagnostics: &mut Vec<Diagnostic>) {
        let start = self.pos;
        self.bump();
        loop {
            match self.peek() {
                None => {
                    diagnostics.push(Diagnostic::new(
                        code::UNTERMINATED_STRING,
                        "unterminated string literal",
                        Span::new(start, self.pos),
                    ));
                    break;
                }
                Some('"') => {
                    self.bump();
                    break;
                }
                Some('\\') => {
                    self.bump();
                    self.lex_escape(diagnostics);
                }
                Some(_) => {
                    self.bump();
                }
            }
        }
    }

    /// Reports an unterminated character literal.
    fn unterminated_char(&mut self, start: usize, diagnostics: &mut Vec<Diagnostic>) {
        diagnostics.push(Diagnostic::new(
            code::UNTERMINATED_CHAR,
            "unterminated character literal",
            Span::new(start, self.pos),
        ));
    }
}
