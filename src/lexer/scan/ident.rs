//! Identifiers and keywords (0.1 §2.4, §2.5).

use crate::lexer::Lexer;
use crate::lexer::token::TokenKind;

/// The reserved keywords (0.1 §2.5), mapped to their token kinds.
const KEYWORDS: &[(&str, TokenKind)] = &[
    ("module", TokenKind::Module),
    ("import", TokenKind::Import),
    ("private", TokenKind::Private),
    ("struct", TokenKind::Struct),
    ("fn", TokenKind::Fn),
    ("let", TokenKind::Let),
    ("const", TokenKind::Const),
    ("if", TokenKind::If),
    ("else", TokenKind::Else),
    ("while", TokenKind::While),
    ("for", TokenKind::For),
    ("in", TokenKind::In),
    ("return", TokenKind::Return),
    ("break", TokenKind::Break),
    ("continue", TokenKind::Continue),
    ("true", TokenKind::True),
    ("false", TokenKind::False),
    ("int", TokenKind::Int),
    ("float", TokenKind::Float),
    ("bool", TokenKind::Bool),
    ("char", TokenKind::Char),
    ("string", TokenKind::String),
    ("void", TokenKind::Void),
    ("extern", TokenKind::Extern),
    ("export", TokenKind::Export),
    ("pin", TokenKind::Pin),
    ("panic", TokenKind::Panic),
    ("thread", TokenKind::Thread),
    ("spawn", TokenKind::Spawn),
    ("async", TokenKind::Async),
    ("await", TokenKind::Await),
    ("send", TokenKind::Send),
    ("recv", TokenKind::Recv),
    ("mutex", TokenKind::Mutex),
    ("defer", TokenKind::Defer),
    ("yield", TokenKind::Yield),
    ("match", TokenKind::Match),
    ("trait", TokenKind::Trait),
    ("impl", TokenKind::Impl),
];

impl Lexer<'_> {
    /// Lexes an identifier or a keyword (0.1 §2.4).
    pub(in crate::lexer) fn lex_ident(&mut self) -> TokenKind {
        let start = self.pos;
        while let Some(c) = self.peek() {
            if is_ident_continue(c) {
                self.bump();
            } else {
                break;
            }
        }
        keyword(&self.source[start..self.pos]).unwrap_or(TokenKind::Ident)
    }
}

/// Looks up a reserved keyword.
fn keyword(text: &str) -> Option<TokenKind> {
    KEYWORDS
        .iter()
        .find(|(word, _)| *word == text)
        .map(|(_, kind)| *kind)
}

/// Returns `true` if `text` is a reserved keyword (0.1 §2.5).
///
/// The parser uses this to accept a keyword as a module-path component, which
/// 0.1 §4.2 permits (`import std.string as str;`).
#[must_use]
pub fn is_keyword_text(text: &str) -> bool {
    keyword(text).is_some()
}

/// Returns `true` if `c` can start an identifier (0.1 §2.4).
pub(in crate::lexer) fn is_ident_start(c: char) -> bool {
    c == '_' || c.is_alphabetic()
}

/// Returns `true` if `c` can continue an identifier (0.1 §2.4).
fn is_ident_continue(c: char) -> bool {
    c == '_' || c.is_alphabetic() || c.is_ascii_digit()
}
