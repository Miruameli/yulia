//! Operators and delimiters (0.1 §2.5, §2.7).

use crate::lexer::Lexer;
use crate::lexer::token::TokenKind;

impl Lexer<'_> {
    /// Lexes an operator or delimiter (0.1 §2.5).
    pub(in crate::lexer) fn lex_operator(&mut self) -> Option<TokenKind> {
        let kind = match self.peek()? {
            '+' => {
                self.bump();
                TokenKind::Plus
            }
            '-' => {
                self.bump();
                TokenKind::Minus
            }
            '*' => {
                self.bump();
                TokenKind::Star
            }
            '/' => {
                self.bump();
                TokenKind::Slash
            }
            '%' => {
                self.bump();
                TokenKind::Percent
            }
            '=' if self.peek_at(1) == Some('=') => {
                self.bump();
                self.bump();
                TokenKind::EqEq
            }
            '!' if self.peek_at(1) == Some('=') => {
                self.bump();
                self.bump();
                TokenKind::NotEq
            }
            '<' if self.peek_at(1) == Some('=') => {
                self.bump();
                self.bump();
                TokenKind::Le
            }
            '>' if self.peek_at(1) == Some('=') => {
                self.bump();
                self.bump();
                TokenKind::Ge
            }
            '&' if self.peek_at(1) == Some('&') => {
                self.bump();
                self.bump();
                TokenKind::AndAnd
            }
            '|' if self.peek_at(1) == Some('|') => {
                self.bump();
                self.bump();
                TokenKind::OrOr
            }
            '<' => {
                self.bump();
                TokenKind::Lt
            }
            '>' => {
                self.bump();
                TokenKind::Gt
            }
            '!' => {
                self.bump();
                TokenKind::Not
            }
            '=' => {
                self.bump();
                TokenKind::Eq
            }
            '(' => {
                self.bump();
                TokenKind::LParen
            }
            ')' => {
                self.bump();
                TokenKind::RParen
            }
            '{' => {
                self.bump();
                TokenKind::LBrace
            }
            '}' => {
                self.bump();
                TokenKind::RBrace
            }
            '[' => {
                self.bump();
                TokenKind::LBracket
            }
            ']' => {
                self.bump();
                TokenKind::RBracket
            }
            ';' => {
                self.bump();
                TokenKind::Semi
            }
            ',' => {
                self.bump();
                TokenKind::Comma
            }
            ':' => {
                self.bump();
                TokenKind::Colon
            }
            '.' if self.peek_at(1) == Some('.') => {
                self.bump();
                self.bump();
                TokenKind::DotDot
            }
            '.' => {
                self.bump();
                TokenKind::Dot
            }
            _ => return None,
        };
        Some(kind)
    }
}
