//! Type syntax (0.1 §3.2).

use crate::ast::{Type, TypeKind};
use crate::diagnostics::code;
use crate::lexer::TokenKind;
use crate::parser::Parser;

/// Parses `type ::= primitive_type | array_type | "CPtr" | IDENT`.
pub(crate) fn parse_type(parser: &mut Parser<'_>) -> Type {
    let start = parser.span().start;
    let kind = match parser.peek() {
        TokenKind::Int => {
            parser.bump();
            TypeKind::Int
        }
        TokenKind::Float => {
            parser.bump();
            TypeKind::Float
        }
        TokenKind::Bool => {
            parser.bump();
            TypeKind::Bool
        }
        TokenKind::Char => {
            parser.bump();
            TypeKind::Char
        }
        TokenKind::String => {
            parser.bump();
            TypeKind::String
        }
        TokenKind::Void => {
            parser.bump();
            TypeKind::Void
        }
        TokenKind::LBracket => array(parser),
        TokenKind::Ident => named(parser),
        _ => {
            parser.error(code::EXPECTED_TYPE, "expected type");
            parser.bump();
            TypeKind::Named(String::new())
        }
    };
    Type {
        kind,
        span: parser.span_since(start),
    }
}

/// Parses `array_type ::= "[" type ";" INT_LIT "]"`.
fn array(parser: &mut Parser<'_>) -> TypeKind {
    parser.bump();
    let elem = parse_type(parser);
    parser.expect(TokenKind::Semi, code::EXPECTED_SEMICOLON);
    let len = match parser.expect(TokenKind::IntLit, code::EXPECTED_INT_LIT) {
        Some(token) => token.text.to_string(),
        None => String::new(),
    };
    parser.expect(TokenKind::RBracket, code::EXPECTED_RBRACKET);
    TypeKind::Array {
        elem: Box::new(elem),
        len,
    }
}

/// Parses `IDENT`, mapping `CPtr` to its own kind.
fn named(parser: &mut Parser<'_>) -> TypeKind {
    let name = parser.bump().text.to_string();
    if name == "CPtr" {
        TypeKind::CPtr
    } else {
        TypeKind::Named(name)
    }
}
