//! The Yulia parser (0.1 §3).
//!
//! A recursive-descent parser over the lexer's token stream. It recovers from
//! syntax errors: a bad token is reported and the parser skips to the next
//! synchronization point so one run can surface many problems.
//!
//! # Status
//!
//! Phase 1a: the full grammar of 0.1 §3 is implemented.

mod decl;
mod expr;
mod navigation;
mod program;
mod stmt;
mod ty;

use crate::ast::Program;
use crate::diagnostics::code;
use crate::diagnostics::{Code, Diagnostic, Span};
use crate::lexer::{Token, TokenKind};

/// Parses a token stream into a [`Program`] (0.1 §3).
#[derive(Debug)]
pub struct Parser<'a> {
    /// The token stream, ending with [`TokenKind::Eof`].
    tokens: &'a [Token<'a>],
    /// The index of the current token.
    pub(crate) pos: usize,
    /// The diagnostics accumulated so far.
    diagnostics: Vec<Diagnostic>,
    /// Whether `IDENT { .. }` is read as a struct literal.
    ///
    /// Disabled while parsing an `if`/`while` condition or a `for` range end,
    /// where a following `{` opens the loop body rather than a literal.
    pub(crate) struct_literals: bool,
}

impl<'a> Parser<'a> {
    /// Creates a parser over `tokens`.
    #[must_use]
    pub fn new(tokens: &'a [Token<'a>]) -> Self {
        Self {
            tokens,
            pos: 0,
            diagnostics: Vec::new(),
            struct_literals: true,
        }
    }

    /// Parses the whole stream, returning the program and any diagnostics.
    pub fn parse(mut self) -> (Program, Vec<Diagnostic>) {
        let ast = program::parse_program(&mut self);
        (ast, self.diagnostics)
    }

    /// Reports the current token as unexpected and skips it.
    pub(crate) fn unexpected(&mut self) {
        self.error(
            code::UNEXPECTED_TOKEN,
            format!("unexpected `{}`", self.peek().name()),
        );
        self.bump();
    }

    /// Returns the kind of the current token.
    pub(crate) fn peek(&self) -> TokenKind {
        self.tokens[self.pos].kind
    }

    /// Returns the source text of the current token.
    pub(crate) fn text(&self) -> &'a str {
        self.tokens[self.pos].text
    }

    /// Returns `true` if the current token is the identifier `text`.
    pub(crate) fn at_ident(&self, text: &str) -> bool {
        self.at(TokenKind::Ident) && self.text() == text
    }

    /// Consumes the current token if it is the identifier `text`.
    pub(crate) fn eat_ident(&mut self, text: &str) -> bool {
        if self.at_ident(text) {
            self.bump();
            true
        } else {
            false
        }
    }

    /// Records a diagnostic at the current token.
    pub(crate) fn error(&mut self, code: Code, message: impl Into<String>) {
        let span = self.span();
        self.error_at(span, code, message);
    }

    /// Records a diagnostic at `span`.
    pub(crate) fn error_at(&mut self, span: Span, code: Code, message: impl Into<String>) {
        self.diagnostics.push(Diagnostic::new(code, message, span));
    }
}
