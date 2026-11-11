//! Token-stream navigation and error recovery (0.1 §3).

use crate::diagnostics::code;
use crate::diagnostics::{Code, Span};
use crate::lexer::{Token, TokenKind};

use super::Parser;

impl<'a> Parser<'a> {
    /// Runs `f` with struct-literal parsing enabled, restoring the previous mode.
    pub(crate) fn in_struct_scope<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> T {
        let previous = self.struct_literals;
        self.struct_literals = true;
        let result = f(self);
        self.struct_literals = previous;
        result
    }

    /// Runs `f` with struct-literal parsing disabled, restoring the previous mode.
    pub(crate) fn without_struct_literals<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> T {
        let previous = self.struct_literals;
        self.struct_literals = false;
        let result = f(self);
        self.struct_literals = previous;
        result
    }

    /// Returns the span of the current token.
    pub(crate) fn span(&self) -> Span {
        self.tokens[self.pos].span
    }

    /// Returns the span from `start` to the end of the previous token.
    pub(crate) fn span_since(&self, start: usize) -> Span {
        Span::new(start, self.prev_end())
    }

    /// Returns the end offset of the previous token.
    fn prev_end(&self) -> usize {
        self.tokens[self.pos.saturating_sub(1)].span.end
    }

    /// Consumes and returns the current token.
    pub(crate) fn bump(&mut self) -> Token<'a> {
        let token = self.tokens[self.pos];
        if self.pos + 1 < self.tokens.len() {
            self.pos += 1;
        }
        token
    }

    /// Returns `true` if the current token is `kind`.
    pub(crate) fn at(&self, kind: TokenKind) -> bool {
        self.peek() == kind
    }

    /// Consumes the current token if it is `kind`.
    pub(crate) fn eat(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            self.bump();
            true
        } else {
            false
        }
    }

    /// Consumes the current token if it is `kind`, otherwise reports `code`.
    pub(crate) fn expect(&mut self, kind: TokenKind, code: Code) -> Option<Token<'a>> {
        if self.at(kind) {
            Some(self.bump())
        } else {
            self.error(code, format!("expected `{}`", kind.name()));
            None
        }
    }

    /// Consumes an identifier, reporting [`code::EXPECTED_IDENT`] when absent.
    pub(crate) fn expect_ident(&mut self) -> String {
        match self.expect(TokenKind::Ident, code::EXPECTED_IDENT) {
            Some(token) => token.text.to_string(),
            None => String::new(),
        }
    }

    /// Skips tokens until one of `stops` or the end of input.
    pub(crate) fn recover_until(&mut self, stops: &[TokenKind]) {
        while !self.at(TokenKind::Eof) && !stops.contains(&self.peek()) {
            self.bump();
        }
    }
}
