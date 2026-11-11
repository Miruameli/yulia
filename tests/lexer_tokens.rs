//! Token-stream tests: keywords, identifiers, operators, comments, spans.

mod common;

use common::{assert_clean, kinds};
use yulia::diagnostics::Span;
use yulia::lexer::{Lexer, TokenKind};

#[test]
fn empty_source_yields_eof() {
    let (kinds, diags) = kinds("");
    assert_eq!(kinds, vec![TokenKind::Eof]);
    assert_clean(&diags);
}

#[test]
fn whitespace_only_yields_eof() {
    let (kinds, diags) = kinds(" \t\r\n");
    assert_eq!(kinds, vec![TokenKind::Eof]);
    assert_clean(&diags);
}

#[test]
fn operators_and_delimiters() {
    let (kinds, diags) = kinds("+ - * / % == != < > <= >= && || ! = ( ) { } [ ] ; , : . ..");
    let expected = vec![
        TokenKind::Plus,
        TokenKind::Minus,
        TokenKind::Star,
        TokenKind::Slash,
        TokenKind::Percent,
        TokenKind::EqEq,
        TokenKind::NotEq,
        TokenKind::Lt,
        TokenKind::Gt,
        TokenKind::Le,
        TokenKind::Ge,
        TokenKind::AndAnd,
        TokenKind::OrOr,
        TokenKind::Not,
        TokenKind::Eq,
        TokenKind::LParen,
        TokenKind::RParen,
        TokenKind::LBrace,
        TokenKind::RBrace,
        TokenKind::LBracket,
        TokenKind::RBracket,
        TokenKind::Semi,
        TokenKind::Comma,
        TokenKind::Colon,
        TokenKind::Dot,
        TokenKind::DotDot,
        TokenKind::Eof,
    ];
    assert_eq!(kinds, expected);
    assert_clean(&diags);
}

#[test]
fn comments_are_trivia() {
    let (kinds, diags) = kinds("// line comment\n/* block */ 1");
    assert_eq!(kinds, vec![TokenKind::IntLit, TokenKind::Eof]);
    assert_clean(&diags);
}

#[test]
fn block_comments_do_not_nest() {
    let (kinds, diags) = kinds("/* a /* b */ 1");
    assert_eq!(kinds, vec![TokenKind::IntLit, TokenKind::Eof]);
    assert_clean(&diags);
}

#[test]
fn range_is_distinct_from_member_access() {
    let (kinds, diags) = kinds("a..b a.b");
    assert_eq!(
        kinds,
        vec![
            TokenKind::Ident,
            TokenKind::DotDot,
            TokenKind::Ident,
            TokenKind::Ident,
            TokenKind::Dot,
            TokenKind::Ident,
            TokenKind::Eof
        ]
    );
    assert_clean(&diags);
}

#[test]
fn token_spans_cover_the_source_text() {
    let (tokens, diags) = Lexer::new("let x").tokenize();
    assert_clean(&diags);
    let texts: Vec<&str> = tokens.iter().map(|token| token.text).collect();
    assert_eq!(texts, vec!["let", "x", ""]);
    assert_eq!(tokens[0].span, Span::new(0, 3));
    assert_eq!(tokens[1].span, Span::new(4, 5));
}

#[test]
fn unicode_identifiers_are_accepted() {
    let (kinds, diags) = kinds("café");
    assert_eq!(kinds, vec![TokenKind::Ident, TokenKind::Eof]);
    assert_clean(&diags);
}
