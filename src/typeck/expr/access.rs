//! Assignment and call checking (0.2 §6.2).

mod lookup;

use crate::ast::{Expr, ExprKind};
use crate::diagnostics::code;
use crate::typeck::{Checker, Ty};

use super::check;
use crate::typeck::compatible;

pub(crate) use lookup::{field, index};

/// Checks an assignment.
pub(crate) fn assign(checker: &mut Checker<'_>, target: &Expr, value: &Expr) -> Ty {
    let value_ty = check(checker, value);
    let target_ty = target_type(checker, target);
    if !compatible(&target_ty, &value_ty) {
        checker.error(
            value.span,
            code::TYPE_MISMATCH,
            format!(
                "cannot assign `{}` to `{}`",
                value_ty.name(),
                target_ty.name()
            ),
        );
    }
    target_ty
}

/// Returns the type of an assignment target, reporting an invalid one.
fn target_type(checker: &mut Checker<'_>, target: &Expr) -> Ty {
    match &target.kind {
        ExprKind::Ident(name) => {
            if checker.local(name).is_none() {
                checker.error(
                    target.span,
                    code::UNDEFINED_NAME,
                    format!("`{name}` is not defined"),
                );
                return Ty::Error;
            }
            checker.mark_init(name);
            checker.local(name).map_or(Ty::Error, |var| var.ty)
        }
        ExprKind::Field { .. } | ExprKind::Index { .. } => check(checker, target),
        _ => {
            checker.error(
                target.span,
                code::NOT_AN_LVALUE,
                "assignment target must be a variable, field, or element",
            );
            Ty::Error
        }
    }
}

/// Checks a call.
pub(crate) fn call(checker: &mut Checker<'_>, callee: &Expr, args: &[Expr]) -> Ty {
    let ExprKind::Ident(name) = &callee.kind else {
        checker.error(
            callee.span,
            code::NOT_A_FUNCTION,
            "call target must be a function name",
        );
        return Ty::Error;
    };
    let Some(sig) = checker.fns.get(name).cloned() else {
        checker.error(
            callee.span,
            code::UNDEFINED_NAME,
            format!("`{name}` is not a function"),
        );
        return Ty::Error;
    };
    if args.len() != sig.params.len() {
        checker.error(
            callee.span,
            code::ARGUMENT_COUNT,
            format!(
                "`{name}` takes {} argument(s), found {}",
                sig.params.len(),
                args.len()
            ),
        );
    }
    for (index, arg) in args.iter().enumerate() {
        let arg_ty = check(checker, arg);
        if let Some(param) = sig.params.get(index)
            && !compatible(param, &arg_ty)
        {
            checker.error(
                arg.span,
                code::TYPE_MISMATCH,
                format!(
                    "argument {} of `{name}` expects `{}`, found `{}`",
                    index + 1,
                    param.name(),
                    arg_ty.name()
                ),
            );
        }
    }
    sig.ret
}
