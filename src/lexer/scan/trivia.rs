//! Whitespace, comments, and the character cursor.

use crate::diagnostics::code;
use crate::diagnostics::{Diagnostic, Span};
use crate::lexer::Lexer;

impl Lexer<'_> {
    /// Returns the character at the current position without consuming it.
    pub(in crate::lexer) fn peek(&self) -> Option<char> {
        self.source[self.pos..].chars().next()
    }

    /// Returns the character `offset` positions ahead without consuming it.
    pub(in crate::lexer) fn peek_at(&self, offset: usize) -> Option<char> {
        self.source[self.pos..].chars().nth(offset)
    }

    /// Consumes one character.
    pub(in crate::lexer) fn bump(&mut self) {
        if let Some(c) = self.peek() {
            self.pos += c.len_utf8();
        }
    }

    /// Skips whitespace and comments (0.1 §2.2, §2.3).
    pub(in crate::lexer) fn skip_trivia(&mut self, diagnostics: &mut Vec<Diagnostic>) {
        loop {
            match self.peek() {
                Some(c) if c.is_whitespace() => {
                    self.bump();
                }
                Some('/') if self.peek_at(1) == Some('/') => {
                    while let Some(c) = self.peek() {
                        if c == '\n' {
                            break;
                        }
                        self.bump();
                    }
                }
                Some('/') if self.peek_at(1) == Some('*') => {
                    let start = self.pos;
                    self.bump();
                    self.bump();
                    loop {
                        match self.peek() {
                            None => {
                                diagnostics.push(Diagnostic::new(
                                    code::UNTERMINATED_BLOCK_COMMENT,
                                    "unterminated block comment",
                                    Span::new(start, self.pos),
                                ));
                                break;
                            }
                            Some('*') if self.peek_at(1) == Some('/') => {
                                self.bump();
                                self.bump();
                                break;
                            }
                            Some(_) => {
                                self.bump();
                            }
                        }
                    }
                }
                _ => break,
            }
        }
    }
}
