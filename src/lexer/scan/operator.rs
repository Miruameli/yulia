//! Operators and delimiters (0.1 §2.5, §2.7).

use crate::lexer::Lexer;
use crate::lexer::token::TokenKind;

/// Two-character operators and delimiters.
///
/// Checked before [`ONE_CHAR`] so that `==` wins over `=`.
const TWO_CHAR: &[(&str, TokenKind)] = &[
    ("==", TokenKind::EqEq),
    ("!=", TokenKind::NotEq),
    ("<=", TokenKind::Le),
    (">=", TokenKind::Ge),
    ("&&", TokenKind::AndAnd),
    ("||", TokenKind::OrOr),
    ("..", TokenKind::DotDot),
];

/// Single-character operators and delimiters.
const ONE_CHAR: &[(char, TokenKind)] = &[
    ('+', TokenKind::Plus),
    ('-', TokenKind::Minus),
    ('*', TokenKind::Star),
    ('/', TokenKind::Slash),
    ('%', TokenKind::Percent),
    ('<', TokenKind::Lt),
    ('>', TokenKind::Gt),
    ('!', TokenKind::Not),
    ('=', TokenKind::Eq),
    ('(', TokenKind::LParen),
    (')', TokenKind::RParen),
    ('{', TokenKind::LBrace),
    ('}', TokenKind::RBrace),
    ('[', TokenKind::LBracket),
    (']', TokenKind::RBracket),
    (';', TokenKind::Semi),
    (',', TokenKind::Comma),
    (':', TokenKind::Colon),
    ('.', TokenKind::Dot),
];

impl Lexer<'_> {
    /// Lexes an operator or delimiter (0.1 §2.5).
    pub(in crate::lexer) fn lex_operator(&mut self) -> Option<TokenKind> {
        if let Some(kind) = two_char(&self.source[self.pos..]) {
            self.bump();
            self.bump();
            return Some(kind);
        }
        let kind = one_char(self.peek()?)?;
        self.bump();
        Some(kind)
    }
}

/// Looks up a two-character operator at the start of `rest`.
fn two_char(rest: &str) -> Option<TokenKind> {
    let pair = rest.get(..2)?;
    TWO_CHAR
        .iter()
        .find(|(text, _)| *text == pair)
        .map(|(_, kind)| *kind)
}

/// Looks up a single-character operator.
fn one_char(c: char) -> Option<TokenKind> {
    ONE_CHAR
        .iter()
        .find(|(text, _)| *text == c)
        .map(|(_, kind)| *kind)
}
