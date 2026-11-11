//! Function declarations (0.1 §3.1).

use crate::ast::{ExternDecl, FnDecl, Param, Type, TypeKind, Visibility};
use crate::diagnostics::diagnostic::code;
use crate::lexer::{TokenKind, decode_string};
use crate::parser::{Parser, stmt, ty};

/// Parses `fn IDENT "(" [ param_list ] ")" [ "->" return_type ] block`.
pub(crate) fn parse_fn(parser: &mut Parser<'_>, visibility: Visibility, start: usize) -> FnDecl {
    parser.bump();
    let name = parser.expect_ident();
    let params = parse_params(parser);
    let ret = return_type(parser);
    let body = stmt::parse_block(parser);
    FnDecl {
        name,
        visibility,
        params,
        ret,
        body,
        span: parser.span_since(start),
    }
}

/// Parses `extern STRING_LIT fn IDENT "(" [ param_list ] ")" [ "->" return_type ] ";"`.
pub(crate) fn parse_extern(parser: &mut Parser<'_>, start: usize) -> ExternDecl {
    parser.bump();
    let abi = match parser.expect(TokenKind::StringLit, code::EXPECTED_STRING_LIT) {
        Some(token) => decode_string(token.text),
        None => String::new(),
    };
    parser.expect(TokenKind::Fn, code::EXPECTED_KEYWORD);
    let name = parser.expect_ident();
    let params = parse_params(parser);
    let ret = return_type(parser);
    parser.expect(TokenKind::Semi, code::EXPECTED_SEMICOLON);
    ExternDecl {
        abi,
        name,
        params,
        ret,
        span: parser.span_since(start),
    }
}

/// Parses `"(" [ param { "," param } ] ")"`.
fn parse_params(parser: &mut Parser<'_>) -> Vec<Param> {
    let mut params = Vec::new();
    parser.expect(TokenKind::LParen, code::EXPECTED_LPAREN);
    if parser.at(TokenKind::RParen) {
        parser.bump();
        return params;
    }
    loop {
        params.push(parse_param(parser));
        if !parser.eat(TokenKind::Comma) {
            break;
        }
    }
    parser.expect(TokenKind::RParen, code::EXPECTED_RPAREN);
    params
}

/// Parses `IDENT ":" type`.
fn parse_param(parser: &mut Parser<'_>) -> Param {
    let start = parser.span().start;
    let name = parser.expect_ident();
    parser.expect(TokenKind::Colon, code::EXPECTED_COLON);
    let declared = ty::parse_type(parser);
    Param {
        name,
        ty: declared,
        span: parser.span_since(start),
    }
}

/// Parses `[ "->" return_type ]`, defaulting to `void`.
fn return_type(parser: &mut Parser<'_>) -> Type {
    if parser.eat(TokenKind::Arrow) {
        ty::parse_type(parser)
    } else {
        Type {
            kind: TypeKind::Void,
            span: parser.span(),
        }
    }
}
