//! Expression parsing (0.1 §3.4).
//!
//! Precedence climbing over the levels of 0.1 §2.8, from `||` down to the
//! postfix operators.

mod postfix;
mod primary;

use crate::ast::{BinaryOp, Expr, ExprKind, UnaryOp};
use crate::diagnostics::Span;
use crate::lexer::TokenKind;
use crate::parser::Parser;

/// Parses `expr ::= assign_expr`.
pub(crate) fn parse_expr(parser: &mut Parser<'_>) -> Expr {
    parse_assign(parser)
}

/// Parses `assign_expr ::= or_expr [ "=" assign_expr ]`.
fn parse_assign(parser: &mut Parser<'_>) -> Expr {
    let target = parse_or(parser);
    if !parser.at(TokenKind::Eq) {
        return target;
    }
    parser.bump();
    let value = parse_assign(parser);
    let span = Span::new(target.span.start, value.span.end);
    Expr {
        kind: ExprKind::Assign {
            target: Box::new(target),
            value: Box::new(value),
        },
        span,
    }
}

/// Parses `or_expr ::= and_expr { "||" and_expr }`.
fn parse_or(parser: &mut Parser<'_>) -> Expr {
    let mut lhs = parse_and(parser);
    while parser.at(TokenKind::OrOr) {
        parser.bump();
        let rhs = parse_and(parser);
        lhs = binary(lhs, BinaryOp::Or, rhs);
    }
    lhs
}

/// Parses `and_expr ::= eq_expr { "&&" eq_expr }`.
fn parse_and(parser: &mut Parser<'_>) -> Expr {
    let mut lhs = parse_eq(parser);
    while parser.at(TokenKind::AndAnd) {
        parser.bump();
        let rhs = parse_eq(parser);
        lhs = binary(lhs, BinaryOp::And, rhs);
    }
    lhs
}

/// Parses `eq_expr ::= cmp_expr { ("==" | "!=") cmp_expr }`.
fn parse_eq(parser: &mut Parser<'_>) -> Expr {
    let mut lhs = parse_cmp(parser);
    loop {
        let op = match parser.peek() {
            TokenKind::EqEq => BinaryOp::Eq,
            TokenKind::NotEq => BinaryOp::Ne,
            _ => break,
        };
        parser.bump();
        let rhs = parse_cmp(parser);
        lhs = binary(lhs, op, rhs);
    }
    lhs
}

/// Parses `cmp_expr ::= add_expr { ("<" | ">" | "<=" | ">=") add_expr }`.
fn parse_cmp(parser: &mut Parser<'_>) -> Expr {
    let mut lhs = parse_add(parser);
    loop {
        let op = match parser.peek() {
            TokenKind::Lt => BinaryOp::Lt,
            TokenKind::Gt => BinaryOp::Gt,
            TokenKind::Le => BinaryOp::Le,
            TokenKind::Ge => BinaryOp::Ge,
            _ => break,
        };
        parser.bump();
        let rhs = parse_add(parser);
        lhs = binary(lhs, op, rhs);
    }
    lhs
}

/// Parses `add_expr ::= mul_expr { ("+" | "-") mul_expr }`.
fn parse_add(parser: &mut Parser<'_>) -> Expr {
    let mut lhs = parse_mul(parser);
    loop {
        let op = match parser.peek() {
            TokenKind::Plus => BinaryOp::Add,
            TokenKind::Minus => BinaryOp::Sub,
            _ => break,
        };
        parser.bump();
        let rhs = parse_mul(parser);
        lhs = binary(lhs, op, rhs);
    }
    lhs
}

/// Parses `mul_expr ::= unary_expr { ("*" | "/" | "%") unary_expr }`.
fn parse_mul(parser: &mut Parser<'_>) -> Expr {
    let mut lhs = parse_unary(parser);
    loop {
        let op = match parser.peek() {
            TokenKind::Star => BinaryOp::Mul,
            TokenKind::Slash => BinaryOp::Div,
            TokenKind::Percent => BinaryOp::Rem,
            _ => break,
        };
        parser.bump();
        let rhs = parse_unary(parser);
        lhs = binary(lhs, op, rhs);
    }
    lhs
}

/// Parses `unary_expr ::= ("!" | "-") unary_expr | postfix_expr`.
fn parse_unary(parser: &mut Parser<'_>) -> Expr {
    let op = match parser.peek() {
        TokenKind::Not => UnaryOp::Not,
        TokenKind::Minus => UnaryOp::Neg,
        _ => return postfix::parse_postfix(parser),
    };
    let start = parser.span().start;
    parser.bump();
    let operand = parse_unary(parser);
    let span = Span::new(start, operand.span.end);
    Expr {
        kind: ExprKind::Unary {
            op,
            operand: Box::new(operand),
        },
        span,
    }
}

/// Builds a binary expression spanning `lhs` through `rhs`.
fn binary(lhs: Expr, op: BinaryOp, rhs: Expr) -> Expr {
    let span = Span::new(lhs.span.start, rhs.span.end);
    Expr {
        kind: ExprKind::Binary {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        },
        span,
    }
}
