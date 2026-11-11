//! The token loop: `tokenize` and `next_token`.

use crate::diagnostics::diagnostic::code;
use crate::diagnostics::{Diagnostic, Span};
use crate::lexer::Lexer;
use crate::lexer::token::{Token, TokenKind};

mod ident;
mod operator;
mod trivia;

impl<'a> Lexer<'a> {
    /// Creates a lexer over `source`.
    #[must_use]
    pub fn new(source: &'a str) -> Self {
        Self { source, pos: 0 }
    }

    /// Tokenizes the whole source, returning the tokens and any diagnostics.
    ///
    /// The token stream always ends with [`TokenKind::Eof`].
    pub fn tokenize(mut self) -> (Vec<Token<'a>>, Vec<Diagnostic>) {
        let mut tokens = Vec::new();
        let mut diagnostics = Vec::new();
        while let Some(token) = self.next_token(&mut diagnostics) {
            let is_eof = token.kind == TokenKind::Eof;
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        (tokens, diagnostics)
    }

    /// Lexes the next token, or [`TokenKind::Eof`] at end of input.
    fn next_token(&mut self, diagnostics: &mut Vec<Diagnostic>) -> Option<Token<'a>> {
        loop {
            self.skip_trivia(diagnostics);
            let start = self.pos;
            let Some(c) = self.peek() else {
                return Some(Token {
                    kind: TokenKind::Eof,
                    text: "",
                    span: Span::new(start, start),
                });
            };
            let kind = if ident::is_ident_start(c) {
                self.lex_ident()
            } else if c.is_ascii_digit() {
                self.lex_number(diagnostics)
            } else if c == '"' {
                self.lex_string(diagnostics);
                TokenKind::StringLit
            } else if c == '\'' {
                self.lex_char(diagnostics);
                TokenKind::CharLit
            } else if let Some(kind) = self.lex_operator() {
                kind
            } else {
                diagnostics.push(Diagnostic::new(
                    code::UNEXPECTED_CHAR,
                    format!("unexpected character {c:?}"),
                    Span::new(start, start + c.len_utf8()),
                ));
                self.bump();
                continue;
            };
            let text = &self.source[start..self.pos];
            return Some(Token {
                kind,
                text,
                span: Span::new(start, self.pos),
            });
        }
    }
}
