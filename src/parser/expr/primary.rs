//! Primary expressions (0.1 §3.4).

use crate::ast::{Expr, ExprKind, FieldInit};
use crate::diagnostics::code;
use crate::lexer::{Token, TokenKind, decode_char, decode_string};
use crate::parser::Parser;

/// Parses a primary expression.
pub(crate) fn parse_primary(parser: &mut Parser<'_>) -> Expr {
    let token = parser.bump();
    match token.kind {
        TokenKind::IntLit => literal(token, ExprKind::IntLit(token.text.to_string())),
        TokenKind::FloatLit => literal(token, ExprKind::FloatLit(token.text.to_string())),
        TokenKind::True => literal(token, ExprKind::BoolLit(true)),
        TokenKind::False => literal(token, ExprKind::BoolLit(false)),
        TokenKind::CharLit => literal(token, ExprKind::CharLit(decode_char(token.text))),
        TokenKind::StringLit => literal(token, ExprKind::StringLit(decode_string(token.text))),
        TokenKind::Ident => ident(parser, token),
        TokenKind::LParen => parenthesized(parser),
        TokenKind::LBracket => array(parser, token),
        TokenKind::Panic => panic_expr(parser, token),
        _ => {
            parser.error_at(token.span, code::EXPECTED_EXPRESSION, "expected expression");
            literal(token, ExprKind::Ident(String::new()))
        }
    }
}

/// Builds an expression from a single token.
fn literal(token: Token<'_>, kind: ExprKind) -> Expr {
    Expr {
        kind,
        span: token.span,
    }
}

/// Parses `IDENT`, or a struct literal when `{` follows.
fn ident(parser: &mut Parser<'_>, token: Token<'_>) -> Expr {
    if parser.struct_literals && parser.at(TokenKind::LBrace) {
        return struct_literal(parser, token);
    }
    literal(token, ExprKind::Ident(token.text.to_string()))
}

/// Parses `"(" expr ")"`.
fn parenthesized(parser: &mut Parser<'_>) -> Expr {
    let inner = parser.in_struct_scope(|p| super::parse_expr(p));
    parser.expect(TokenKind::RParen, code::EXPECTED_RPAREN);
    inner
}

/// Parses `[ expr { "," expr } ]` or `[ value ; count ]`.
fn array(parser: &mut Parser<'_>, token: Token<'_>) -> Expr {
    parser.in_struct_scope(|parser| array_literal(parser, token))
}

/// Parses the bracketed body of an array literal.
fn array_literal(parser: &mut Parser<'_>, token: Token<'_>) -> Expr {
    let start = token.span.start;
    if parser.eat(TokenKind::RBracket) {
        return Expr {
            kind: ExprKind::ArrayLiteral {
                elements: Vec::new(),
            },
            span: parser.span_since(start),
        };
    }
    let first = super::parse_expr(parser);
    if parser.eat(TokenKind::Semi) {
        return repeat(parser, start, first);
    }
    let mut elements = vec![first];
    while parser.eat(TokenKind::Comma) {
        if parser.at(TokenKind::RBracket) {
            break;
        }
        elements.push(super::parse_expr(parser));
    }
    parser.expect(TokenKind::RBracket, code::EXPECTED_RBRACKET);
    Expr {
        kind: ExprKind::ArrayLiteral { elements },
        span: parser.span_since(start),
    }
}

/// Parses the `; count ]` tail of `[ value ; count ]`.
fn repeat(parser: &mut Parser<'_>, start: usize, value: Expr) -> Expr {
    let count = match parser.expect(TokenKind::IntLit, code::EXPECTED_INT_LIT) {
        Some(token) => token.text.to_string(),
        None => String::new(),
    };
    parser.expect(TokenKind::RBracket, code::EXPECTED_RBRACKET);
    Expr {
        kind: ExprKind::ArrayRepeat {
            value: Box::new(value),
            count,
        },
        span: parser.span_since(start),
    }
}

/// Parses `Name { field: value { , field: value } }`.
fn struct_literal(parser: &mut Parser<'_>, token: Token<'_>) -> Expr {
    let start = token.span.start;
    let name = token.text.to_string();
    parser.bump();
    let mut fields = Vec::new();
    while !parser.at(TokenKind::RBrace) && !parser.at(TokenKind::Eof) {
        let field_start = parser.span().start;
        let field_name = parser.expect_ident();
        parser.expect(TokenKind::Colon, code::EXPECTED_COLON);
        let value = super::parse_expr(parser);
        fields.push(FieldInit {
            name: field_name,
            value,
            span: parser.span_since(field_start),
        });
        if !parser.eat(TokenKind::Comma) {
            break;
        }
    }
    parser.expect(TokenKind::RBrace, code::EXPECTED_RBRACE);
    Expr {
        kind: ExprKind::StructLiteral { name, fields },
        span: parser.span_since(start),
    }
}

/// Parses `panic "(" [ expr ] ")"`.
fn panic_expr(parser: &mut Parser<'_>, token: Token<'_>) -> Expr {
    let start = token.span.start;
    let message = if parser.at(TokenKind::LParen) {
        parser.bump();
        let inner = if parser.at(TokenKind::RParen) {
            None
        } else {
            Some(Box::new(super::parse_expr(parser)))
        };
        parser.expect(TokenKind::RParen, code::EXPECTED_RPAREN);
        inner
    } else {
        parser.error(code::EXPECTED_LPAREN, "expected `(`");
        None
    };
    Expr {
        kind: ExprKind::Panic(message),
        span: parser.span_since(start),
    }
}
