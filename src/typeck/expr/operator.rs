//! Operator type checking (0.2 §6.2).

use crate::ast::{BinaryOp, Expr, UnaryOp};
use crate::diagnostics::code;
use crate::typeck::{Checker, Ty};

use super::check;

/// Checks a prefix operation.
pub(crate) fn unary(checker: &mut Checker<'_>, op: UnaryOp, operand: &Expr) -> Ty {
    let ty = check(checker, operand);
    let (expected, name) = match op {
        UnaryOp::Not => (Ty::Bool, "!"),
        UnaryOp::Neg => (Ty::Int, "-"),
    };
    if ty != expected && !ty.is_error() {
        checker.error(
            operand.span,
            code::TYPE_MISMATCH,
            format!("`{name}` requires a `{}` operand", expected.name()),
        );
    }
    expected
}

/// Checks a binary operation.
pub(crate) fn binary(checker: &mut Checker<'_>, op: BinaryOp, lhs: &Expr, rhs: &Expr) -> Ty {
    let left = check(checker, lhs);
    let right = check(checker, rhs);
    if matches!(op, BinaryOp::Eq | BinaryOp::Ne) {
        return equality(checker, op, lhs, rhs, left, right);
    }
    let (expected, result) = match op {
        BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Rem => {
            (Ty::Int, Ty::Int)
        }
        BinaryOp::Lt | BinaryOp::Gt | BinaryOp::Le | BinaryOp::Ge | BinaryOp::Eq | BinaryOp::Ne => {
            (Ty::Int, Ty::Bool)
        }
        BinaryOp::And | BinaryOp::Or => (Ty::Bool, Ty::Bool),
    };
    for (span, ty) in [(lhs.span, left), (rhs.span, right)] {
        if ty != expected && !ty.is_error() {
            checker.error(
                span,
                code::TYPE_MISMATCH,
                format!("`{}` requires a `{}` operand", op_name(op), expected.name()),
            );
        }
    }
    result
}

/// Checks `==` and `!=`, which the Phase 1 rules restrict to `int`.
fn equality(
    checker: &mut Checker<'_>,
    op: BinaryOp,
    lhs: &Expr,
    rhs: &Expr,
    left: Ty,
    right: Ty,
) -> Ty {
    for (span, ty) in [(lhs.span, left), (rhs.span, right)] {
        if ty != Ty::Int && !ty.is_error() {
            checker.error(
                span,
                code::TYPE_MISMATCH,
                format!("`{}` requires `int` operands", op_name(op)),
            );
        }
    }
    Ty::Bool
}

/// Returns the source spelling of a binary operator.
fn op_name(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::Add => "+",
        BinaryOp::Sub => "-",
        BinaryOp::Mul => "*",
        BinaryOp::Div => "/",
        BinaryOp::Rem => "%",
        BinaryOp::Eq => "==",
        BinaryOp::Ne => "!=",
        BinaryOp::Lt => "<",
        BinaryOp::Gt => ">",
        BinaryOp::Le => "<=",
        BinaryOp::Ge => ">=",
        BinaryOp::And => "&&",
        BinaryOp::Or => "||",
    }
}
