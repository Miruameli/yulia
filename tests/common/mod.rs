//! Shared helpers for the integration tests.

use yulia::diagnostics::Diagnostic;
use yulia::lexer::{Lexer, TokenKind};

/// Tokenizes `source` and returns the token kinds and diagnostics.
pub(crate) fn kinds(source: &str) -> (Vec<TokenKind>, Vec<Diagnostic>) {
    let (tokens, diagnostics) = Lexer::new(source).tokenize();
    (
        tokens.into_iter().map(|token| token.kind).collect(),
        diagnostics,
    )
}

/// Asserts that no diagnostics were produced.
pub(crate) fn assert_clean(diags: &[Diagnostic]) {
    assert!(diags.is_empty(), "unexpected diagnostics: {diags:?}");
}

/// The reserved keywords of 0.1 §2.5 and their token kinds.
///
/// Shared by every lexer test crate; only the keyword tests use it, so the
/// other crates would otherwise report it as dead code.
#[allow(dead_code)]
pub(crate) const KEYWORDS: &[(&str, TokenKind)] = &[
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
