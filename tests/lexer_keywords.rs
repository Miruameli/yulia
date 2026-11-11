//! Keyword and identifier tests (0.1 §2.4, §2.5).

mod common;

use common::{assert_clean, kinds};
use yulia::lexer::TokenKind;

#[test]
fn identifiers_and_keywords() {
    let (kinds, diags) = kinds("let x = true");
    assert_eq!(
        kinds,
        vec![
            TokenKind::Let,
            TokenKind::Ident,
            TokenKind::Eq,
            TokenKind::True,
            TokenKind::Eof
        ]
    );
    assert_clean(&diags);
}

#[test]
fn every_keyword_is_reserved() {
    let source = "module import private struct fn let const if else while for in \
                  return break continue true false int float bool char string void \
                  extern export pin panic thread spawn async await send recv mutex \
                  defer yield match trait impl";
    let (kinds, diags) = kinds(source);
    assert_clean(&diags);
    let expected = vec![
        TokenKind::Module,
        TokenKind::Import,
        TokenKind::Private,
        TokenKind::Struct,
        TokenKind::Fn,
        TokenKind::Let,
        TokenKind::Const,
        TokenKind::If,
        TokenKind::Else,
        TokenKind::While,
        TokenKind::For,
        TokenKind::In,
        TokenKind::Return,
        TokenKind::Break,
        TokenKind::Continue,
        TokenKind::True,
        TokenKind::False,
        TokenKind::Int,
        TokenKind::Float,
        TokenKind::Bool,
        TokenKind::Char,
        TokenKind::String,
        TokenKind::Void,
        TokenKind::Extern,
        TokenKind::Export,
        TokenKind::Pin,
        TokenKind::Panic,
        TokenKind::Thread,
        TokenKind::Spawn,
        TokenKind::Async,
        TokenKind::Await,
        TokenKind::Send,
        TokenKind::Recv,
        TokenKind::Mutex,
        TokenKind::Defer,
        TokenKind::Yield,
        TokenKind::Match,
        TokenKind::Trait,
        TokenKind::Impl,
        TokenKind::Eof,
    ];
    assert_eq!(kinds, expected);
}
