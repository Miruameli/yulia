//! Escape sequences (0.1 §2.6).

use crate::diagnostics::code;
use crate::diagnostics::{Diagnostic, Span};
use crate::lexer::Lexer;

impl Lexer<'_> {
    /// Lexes an escape sequence after its backslash (0.1 §2.6).
    pub(in crate::lexer) fn lex_escape(&mut self, diagnostics: &mut Vec<Diagnostic>) {
        let start = self.pos;
        match self.peek() {
            Some('n' | 't' | 'r' | '\\' | '\'' | '"' | '0') => {
                self.bump();
            }
            Some('x') => {
                self.bump();
                let digits_start = self.pos;
                for _ in 0..2 {
                    if self.peek().is_some_and(|c| c.is_ascii_hexdigit()) {
                        self.bump();
                    } else {
                        break;
                    }
                }
                if self.pos - digits_start != 2 {
                    diagnostics.push(Diagnostic::new(
                        code::INVALID_ESCAPE,
                        "\\x escape requires exactly two hexadecimal digits",
                        Span::new(start - 1, self.pos),
                    ));
                }
            }
            Some('u') => self.lex_unicode_escape(start, diagnostics),
            Some(c) => {
                diagnostics.push(Diagnostic::new(
                    code::INVALID_ESCAPE,
                    format!("invalid escape sequence '\\{c}'"),
                    Span::new(start - 1, self.pos + c.len_utf8()),
                ));
                self.bump();
            }
            None => diagnostics.push(Diagnostic::new(
                code::INVALID_ESCAPE,
                "escape sequence at end of input",
                Span::new(start - 1, self.pos),
            )),
        }
    }

    /// Lexes a `\u{...}` escape (0.1 §2.6).
    fn lex_unicode_escape(&mut self, start: usize, diagnostics: &mut Vec<Diagnostic>) {
        self.bump();
        if self.peek() != Some('{') {
            diagnostics.push(Diagnostic::new(
                code::INVALID_ESCAPE,
                "\\u escape requires '{' after \\u",
                Span::new(start - 1, self.pos),
            ));
            return;
        }
        self.bump();
        let digits_start = self.pos;
        while let Some(c) = self.peek() {
            if c.is_ascii_hexdigit() {
                self.bump();
            } else {
                break;
            }
        }
        if self.pos == digits_start {
            diagnostics.push(Diagnostic::new(
                code::INVALID_ESCAPE,
                "\\u{...} escape requires at least one hexadecimal digit",
                Span::new(start - 1, self.pos),
            ));
            return;
        }
        if self.peek() != Some('}') {
            diagnostics.push(Diagnostic::new(
                code::INVALID_ESCAPE,
                "\\u{...} escape is missing its closing '}'",
                Span::new(start - 1, self.pos),
            ));
            return;
        }
        self.bump();
    }
}
