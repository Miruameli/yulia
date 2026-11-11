//! `for` loop checking (0.2 §6.2).

use crate::ast::{Block, Expr, Stmt, StmtKind};
use crate::diagnostics::code;
use crate::typeck::compatible;
use crate::typeck::{Checker, Ty};

use crate::typeck::expr;
use crate::typeck::stmt;

/// Checks `for var in start..end { .. }`.
pub(crate) fn for_range(
    checker: &mut Checker<'_>,
    var: &str,
    start: &Expr,
    end: &Expr,
    body: &Block,
) {
    let start_ty = expr::check(checker, start);
    let end_ty = expr::check(checker, end);
    for (span, ty) in [(start.span, start_ty), (end.span, end_ty)] {
        if ty != Ty::Int && !ty.is_error() {
            checker.error(
                span,
                code::TYPE_MISMATCH,
                format!("`for` range bounds must be `int`, found `{}`", ty.name()),
            );
        }
    }
    let base = checker.snapshot();
    checker.push_scope();
    checker.declare(var.to_string(), Ty::Int, true);
    checker.loops += 1;
    stmt::block(checker, body);
    checker.loops -= 1;
    checker.pop_scope();
    checker.restore(base);
}

/// Checks `for var: type = init; cond; update { .. }`.
pub(crate) fn for_c_style(
    checker: &mut Checker<'_>,
    init: &Stmt,
    cond: &Expr,
    update: &Expr,
    body: &Block,
) {
    let base = checker.snapshot();
    checker.push_scope();
    let var = loop_var(init);
    stmt::check(checker, init);
    let cond_ty = expr::check(checker, cond);
    if cond_ty != Ty::Bool && !cond_ty.is_error() {
        checker.error(
            cond.span,
            code::CONDITION_NOT_BOOL,
            format!("`for` condition must be `bool`, found `{}`", cond_ty.name()),
        );
    }
    let update_ty = expr::check(checker, update);
    if let Some(var) = var {
        let var_ty = checker.local(&var).map_or(Ty::Error, |binding| binding.ty);
        if !compatible(&var_ty, &update_ty) {
            checker.error(
                update.span,
                code::TYPE_MISMATCH,
                format!(
                    "`for` update must produce `{}`, found `{}`",
                    var_ty.name(),
                    update_ty.name()
                ),
            );
        }
    }
    checker.loops += 1;
    stmt::block(checker, body);
    checker.loops -= 1;
    checker.pop_scope();
    checker.restore(base);
}

/// Returns the name bound by a C-style `for` initializer.
fn loop_var(init: &Stmt) -> Option<String> {
    match &init.kind {
        StmtKind::Let { name, .. } | StmtKind::Const { name, .. } => Some(name.clone()),
        _ => None,
    }
}
