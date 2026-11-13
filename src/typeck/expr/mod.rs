//! Expression type checking (0.2 §6.2).

mod access;
mod literal;
mod operator;

use crate::ast::{Expr, ExprKind};
use crate::diagnostics::code;
use crate::typeck::{Checker, Ty};

use operator::{binary, unary};

/// Checks `expr`, returning its type.
pub(crate) fn check(checker: &mut Checker<'_>, expr: &Expr) -> Ty {
    let ty = match &expr.kind {
        ExprKind::IntLit(_) => Ty::Int,
        ExprKind::BoolLit(_) => Ty::Bool,
        ExprKind::FloatLit(_) => out_of_subset(checker, expr, "float"),
        ExprKind::CharLit(_) => out_of_subset(checker, expr, "char"),
        ExprKind::StringLit(_) => out_of_subset(checker, expr, "string"),
        ExprKind::Panic(_) => out_of_subset(checker, expr, "panic"),
        ExprKind::Ident(name) => ident(checker, expr, name),
        ExprKind::Unary { op, operand } => unary(checker, *op, operand),
        ExprKind::Binary { op, lhs, rhs } => binary(checker, *op, lhs, rhs),
        ExprKind::Assign { target, value } => access::assign(checker, target, value),
        ExprKind::Call { callee, args } => access::call(checker, callee, args),
        ExprKind::Field { base, name } => access::field(checker, base, name),
        ExprKind::Index { base, index } => access::index(checker, base, index),
        ExprKind::StructLiteral { name, fields } => {
            literal::struct_literal(checker, expr, name, fields)
        }
        ExprKind::ArrayLiteral { elements } => literal::array_literal(checker, elements),
        ExprKind::ArrayRepeat { value, count } => {
            literal::array_repeat(checker, expr, value, count)
        }
    };
    checker.record(expr.span, ty.clone());
    ty
}

/// Reports a construct that parses but is outside the Phase 1 subset.
fn out_of_subset(checker: &mut Checker<'_>, expr: &Expr, what: &str) -> Ty {
    checker.error(
        expr.span,
        code::OUT_OF_SUBSET,
        format!("`{what}` is outside the Phase 1 subset"),
    );
    Ty::Error
}

/// Checks a name: a local, a constant, or a function used as a value.
fn ident(checker: &mut Checker<'_>, expr: &Expr, name: &str) -> Ty {
    if let Some(var) = checker.local(name) {
        if !var.init {
            checker.error(
                expr.span,
                code::UNINITIALIZED_READ,
                format!("`{name}` may be uninitialized here"),
            );
        }
        return var.ty;
    }
    if let Some(ty) = checker.consts.get(name).cloned() {
        return ty;
    }
    if checker.fns.contains_key(name) {
        checker.error(
            expr.span,
            code::NOT_A_FUNCTION,
            format!("`{name}` is a function, not a value"),
        );
    } else {
        checker.error(
            expr.span,
            code::UNDEFINED_NAME,
            format!("`{name}` is not defined"),
        );
    }
    Ty::Error
}
