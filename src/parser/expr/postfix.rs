//! Postfix expressions (0.1 §3.4).

use crate::ast::{Expr, ExprKind};
use crate::diagnostics::code;
use crate::lexer::TokenKind;
use crate::parser::Parser;

/// Parses `postfix_expr ::= primary_expr { "." IDENT | "[" expr "]" | "(" args ")" }`.
pub(crate) fn parse_postfix(parser: &mut Parser<'_>) -> Expr {
    let mut base = super::primary::parse_primary(parser);
    loop {
        match parser.peek() {
            TokenKind::Dot => {
                parser.bump();
                let name = parser.expect_ident();
                base = field(parser, base, name);
            }
            TokenKind::LBracket => {
                parser.bump();
                let index = super::parse_expr(parser);
                parser.expect(TokenKind::RBracket, code::EXPECTED_RBRACKET);
                base = indexed(parser, base, index);
            }
            TokenKind::LParen => {
                let args = parse_args(parser);
                base = call(parser, base, args);
            }
            _ => return base,
        }
    }
}

/// Builds a `base.name` field access.
fn field(parser: &mut Parser<'_>, base: Expr, name: String) -> Expr {
    let span = parser.span_since(base.span.start);
    Expr {
        kind: ExprKind::Field {
            base: Box::new(base),
            name,
        },
        span,
    }
}

/// Builds a `base[index]` indexing expression.
fn indexed(parser: &mut Parser<'_>, base: Expr, index: Expr) -> Expr {
    let span = parser.span_since(base.span.start);
    Expr {
        kind: ExprKind::Index {
            base: Box::new(base),
            index: Box::new(index),
        },
        span,
    }
}

/// Builds a `callee(args)` call.
fn call(parser: &mut Parser<'_>, callee: Expr, args: Vec<Expr>) -> Expr {
    let span = parser.span_since(callee.span.start);
    Expr {
        kind: ExprKind::Call {
            callee: Box::new(callee),
            args,
        },
        span,
    }
}

/// Parses `( [ expr { "," expr } ] )`.
fn parse_args(parser: &mut Parser<'_>) -> Vec<Expr> {
    parser.bump();
    let mut args = Vec::new();
    if parser.at(TokenKind::RParen) {
        parser.bump();
        return args;
    }
    loop {
        args.push(super::parse_expr(parser));
        if !parser.eat(TokenKind::Comma) {
            break;
        }
    }
    parser.expect(TokenKind::RParen, code::EXPECTED_RPAREN);
    args
}
