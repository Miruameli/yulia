//! Precedence-climbing binary levels (0.1 §2.8, §3.4).

use crate::ast::{BinaryOp, Expr};
use crate::lexer::TokenKind;
use crate::parser::Parser;

use super::{binary, parse_unary};

/// Parses `or_expr ::= and_expr { "||" and_expr }`.
pub(crate) fn parse_or(parser: &mut Parser<'_>) -> Expr {
    let mut lhs = parse_and(parser);
    while parser.at(TokenKind::OrOr) {
        parser.bump();
        let rhs = parse_and(parser);
        lhs = binary(lhs, BinaryOp::Or, rhs);
    }
    lhs
}

/// Parses `and_expr ::= eq_expr { "&&" eq_expr }`.
pub(crate) fn parse_and(parser: &mut Parser<'_>) -> Expr {
    let mut lhs = parse_eq(parser);
    while parser.at(TokenKind::AndAnd) {
        parser.bump();
        let rhs = parse_eq(parser);
        lhs = binary(lhs, BinaryOp::And, rhs);
    }
    lhs
}

/// Parses `eq_expr ::= cmp_expr { ("==" | "!=") cmp_expr }`.
pub(crate) fn parse_eq(parser: &mut Parser<'_>) -> Expr {
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
pub(crate) fn parse_cmp(parser: &mut Parser<'_>) -> Expr {
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
pub(crate) fn parse_add(parser: &mut Parser<'_>) -> Expr {
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
pub(crate) fn parse_mul(parser: &mut Parser<'_>) -> Expr {
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
