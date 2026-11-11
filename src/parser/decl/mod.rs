//! Top-level declaration parsing (0.1 §3.1).

mod func;

use crate::ast::{ConstDecl, Decl, DeclKind, Field, StructDecl, Visibility};
use crate::diagnostics::diagnostic::code;
use crate::lexer::TokenKind;
use crate::parser::{Parser, expr, ty};

/// Parses a top-level declaration.
pub(crate) fn parse_top_level_decl(parser: &mut Parser<'_>) -> Decl {
    let start = parser.span().start;
    let visibility = parse_visibility(parser);
    let kind = match parser.peek() {
        TokenKind::Struct => DeclKind::Struct(parse_struct(parser, visibility, start)),
        TokenKind::Fn => DeclKind::Fn(func::parse_fn(parser, visibility, start)),
        TokenKind::Extern => DeclKind::Extern(func::parse_extern(parser, start)),
        TokenKind::Const => DeclKind::Const(parse_const(parser, visibility, start)),
        _ => {
            parser.unexpected();
            DeclKind::Struct(StructDecl {
                name: String::new(),
                visibility,
                fields: Vec::new(),
                span: parser.span(),
            })
        }
    };
    Decl {
        kind,
        span: parser.span_since(start),
    }
}

/// Parses the optional `private` or `export` modifier.
fn parse_visibility(parser: &mut Parser<'_>) -> Visibility {
    match parser.peek() {
        TokenKind::Private => {
            parser.bump();
            Visibility::Private
        }
        TokenKind::Export => {
            parser.bump();
            Visibility::Export
        }
        _ => Visibility::Public,
    }
}

/// Parses `struct IDENT "{" { struct_field } "}"`.
fn parse_struct(parser: &mut Parser<'_>, visibility: Visibility, start: usize) -> StructDecl {
    parser.bump();
    let name = parser.expect_ident();
    parser.expect(TokenKind::LBrace, code::EXPECTED_LBRACE);
    let mut fields = Vec::new();
    while !parser.at(TokenKind::RBrace) && !parser.at(TokenKind::Eof) {
        fields.push(parse_field(parser));
        if !parser.eat(TokenKind::Semi) {
            break;
        }
    }
    parser.expect(TokenKind::RBrace, code::EXPECTED_RBRACE);
    StructDecl {
        name,
        visibility,
        fields,
        span: parser.span_since(start),
    }
}

/// Parses `IDENT ":" type ";"`.
fn parse_field(parser: &mut Parser<'_>) -> Field {
    let start = parser.span().start;
    let name = parser.expect_ident();
    parser.expect(TokenKind::Colon, code::EXPECTED_COLON);
    let declared = ty::parse_type(parser);
    Field {
        name,
        ty: declared,
        span: parser.span_since(start),
    }
}

/// Parses `const IDENT ":" type "=" expr ";"`.
fn parse_const(parser: &mut Parser<'_>, visibility: Visibility, start: usize) -> ConstDecl {
    parser.bump();
    let name = parser.expect_ident();
    parser.expect(TokenKind::Colon, code::EXPECTED_COLON);
    let declared = ty::parse_type(parser);
    parser.expect(TokenKind::Eq, code::EXPECTED_EQ);
    let value = expr::parse_expr(parser);
    parser.expect(TokenKind::Semi, code::EXPECTED_SEMICOLON);
    ConstDecl {
        name,
        visibility,
        ty: declared,
        value,
        span: parser.span_since(start),
    }
}
