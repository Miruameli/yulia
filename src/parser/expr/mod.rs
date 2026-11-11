//! Expression parsing (0.1 §3.4).
//!
//! Precedence climbing over the levels of 0.1 §2.8, from `||` down to the
//! postfix operators.

mod postfix;
mod precedence;
mod primary;

use crate::ast::{BinaryOp, Expr, ExprKind, UnaryOp};
use crate::diagnostics::Span;
use crate::lexer::TokenKind;
use crate::parser::Parser;

use precedence::parse_or;

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

/// Parses `unary_expr ::= ("!" | "-") unary_expr | postfix_expr`.
pub(crate) fn parse_unary(parser: &mut Parser<'_>) -> Expr {
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
pub(crate) fn binary(lhs: Expr, op: BinaryOp, rhs: Expr) -> Expr {
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
