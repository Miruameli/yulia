//! Literal tests: integers, floats, booleans, characters, and strings.

mod common;

use common::{assert_clean, kinds};
use yulia::lexer::{Lexer, TokenKind};

#[test]
fn integer_literals() {
    let (kinds, diags) = kinds("0 1 42 1_000 0xFF 0b1010 0o77");
    assert_eq!(
        kinds,
        vec![
            TokenKind::IntLit,
            TokenKind::IntLit,
            TokenKind::IntLit,
            TokenKind::IntLit,
            TokenKind::IntLit,
            TokenKind::IntLit,
            TokenKind::IntLit,
            TokenKind::Eof
        ]
    );
    assert_clean(&diags);
}

#[test]
fn float_literals() {
    let (kinds, diags) = kinds("1.5 1.5e3 1e3 1.5e-3 0.5");
    assert_eq!(
        kinds,
        vec![
            TokenKind::FloatLit,
            TokenKind::FloatLit,
            TokenKind::FloatLit,
            TokenKind::FloatLit,
            TokenKind::FloatLit,
            TokenKind::Eof
        ]
    );
    assert_clean(&diags);
}

#[test]
fn bool_literals() {
    let (kinds, diags) = kinds("true false");
    assert_eq!(
        kinds,
        vec![TokenKind::True, TokenKind::False, TokenKind::Eof]
    );
    assert_clean(&diags);
}

#[test]
fn char_literals_and_escapes() {
    let (kinds, diags) = kinds("'a' '\\n' '\\x41' '\\u{1F600}'");
    assert_eq!(
        kinds,
        vec![
            TokenKind::CharLit,
            TokenKind::CharLit,
            TokenKind::CharLit,
            TokenKind::CharLit,
            TokenKind::Eof
        ]
    );
    assert_clean(&diags);
}

#[test]
fn string_literals_and_escapes() {
    let (kinds, diags) = kinds("\"hello\" \"a\\nb\" \"\\u{1F600}\"");
    assert_eq!(
        kinds,
        vec![
            TokenKind::StringLit,
            TokenKind::StringLit,
            TokenKind::StringLit,
            TokenKind::Eof
        ]
    );
    assert_clean(&diags);
}

#[test]
fn underscore_separators_are_part_of_the_literal() {
    let (tokens, diags) = Lexer::new("1_000_000").tokenize();
    assert_clean(&diags);
    assert_eq!(tokens[0].kind, TokenKind::IntLit);
    assert_eq!(tokens[0].text, "1_000_000");
}
