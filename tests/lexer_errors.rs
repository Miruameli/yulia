//! Diagnostic tests: every lexical error path in 0.1 §2.

mod common;

use common::{assert_clean, kinds};
use yulia::diagnostics::code;
use yulia::lexer::TokenKind;

#[test]
fn unterminated_block_comment_is_reported() {
    let (kinds, diags) = kinds("/*");
    assert_eq!(kinds, vec![TokenKind::Eof]);
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].code, code::UNTERMINATED_BLOCK_COMMENT);
}

#[test]
fn unterminated_string_is_reported() {
    let (kinds, diags) = kinds("\"abc");
    assert_eq!(kinds, vec![TokenKind::StringLit, TokenKind::Eof]);
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].code, code::UNTERMINATED_STRING);
}

#[test]
fn unterminated_char_is_reported() {
    let (kinds, diags) = kinds("'a");
    assert_eq!(kinds, vec![TokenKind::CharLit, TokenKind::Eof]);
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].code, code::UNTERMINATED_CHAR);
}

#[test]
fn invalid_escape_is_reported() {
    let (kinds, diags) = kinds("'\\q'");
    assert_eq!(kinds, vec![TokenKind::CharLit, TokenKind::Eof]);
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].code, code::INVALID_ESCAPE);
}

#[test]
fn misplaced_underscore_is_reported() {
    let (kinds, diags) = kinds("1__0");
    assert_eq!(kinds, vec![TokenKind::IntLit, TokenKind::Eof]);
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].code, code::INVALID_NUMERIC_LITERAL);
}

#[test]
fn radix_literal_without_digits_is_reported() {
    let (kinds, diags) = kinds("0x");
    assert_eq!(kinds, vec![TokenKind::IntLit, TokenKind::Eof]);
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].code, code::INVALID_NUMERIC_LITERAL);
}

#[test]
fn empty_char_literal_is_reported() {
    let (kinds, diags) = kinds("''");
    assert_eq!(kinds, vec![TokenKind::CharLit, TokenKind::Eof]);
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].code, code::INVALID_CHAR_LITERAL);
}

#[test]
fn unexpected_character_is_reported_and_skipped() {
    let (kinds, diags) = kinds("@ 1");
    assert_eq!(kinds, vec![TokenKind::IntLit, TokenKind::Eof]);
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].code, code::UNEXPECTED_CHAR);
}

#[test]
fn lone_ampersand_is_unexpected() {
    let (kinds, diags) = kinds("&");
    assert_eq!(kinds, vec![TokenKind::Eof]);
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].code, code::UNEXPECTED_CHAR);
}

#[test]
fn a_valid_program_produces_no_diagnostics() {
    let source = "module main;\n\nfn add(a: int, b: int) -> int {\n    return a + b;\n}\n";
    let (kinds, diags) = kinds(source);
    assert_clean(&diags);
    assert_eq!(kinds[0], TokenKind::Module);
    assert_eq!(kinds[kinds.len() - 1], TokenKind::Eof);
}
