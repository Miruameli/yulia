//! Keyword and identifier tests (0.1 §2.4, §2.5).

mod common;

use common::{KEYWORDS, assert_clean, kinds};
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
    for (word, expected) in KEYWORDS {
        let (kinds, diags) = kinds(word);
        assert_clean(&diags);
        assert_eq!(kinds, vec![*expected, TokenKind::Eof], "{word}");
    }
}

#[test]
fn keywords_are_case_sensitive() {
    let (kinds, diags) = kinds("Let TRUE");
    assert_eq!(
        kinds,
        vec![TokenKind::Ident, TokenKind::Ident, TokenKind::Eof]
    );
    assert_clean(&diags);
}
