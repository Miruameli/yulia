//! Control-flow statement checking (0.2 §6.2).

mod for_loops;

use crate::ast::{Block, ElseBranch, Expr, StmtKind};
use crate::diagnostics::Span;
use crate::diagnostics::code;
use crate::typeck::compatible;
use crate::typeck::{Checker, Ty};

use crate::typeck::expr;
use crate::typeck::stmt;

pub(crate) use for_loops::{for_c_style, for_range};

/// Checks `pin lvalue as name { .. }`.
pub(crate) fn pin(checker: &mut Checker<'_>, span: Span, body: &Block) {
    checker.error(
        span,
        code::OUT_OF_SUBSET,
        "`pin` is outside the Phase 1 subset",
    );
    stmt::block(checker, body);
}

/// Checks `if cond { .. } [else ..]`.
pub(crate) fn if_stmt(
    checker: &mut Checker<'_>,
    cond: &Expr,
    then: &Block,
    else_branch: Option<&ElseBranch>,
) {
    let cond_ty = expr::check(checker, cond);
    if cond_ty != Ty::Bool && !cond_ty.is_error() {
        checker.error(
            cond.span,
            code::CONDITION_NOT_BOOL,
            format!("`if` condition must be `bool`, found `{}`", cond_ty.name()),
        );
    }
    let base = checker.snapshot();
    stmt::block(checker, then);
    let left = checker.snapshot();
    checker.restore(base.clone());
    match else_branch {
        Some(ElseBranch::If(stmt)) => {
            if let StmtKind::If {
                cond,
                then,
                else_branch,
            } = &stmt.kind
            {
                if_stmt(checker, cond, then, else_branch.as_ref());
            }
        }
        Some(ElseBranch::Block(block)) => stmt::block(checker, block),
        None => {}
    }
    let right = checker.snapshot();
    checker.merge_if(&base, &left, &right);
}

/// Checks `while cond { .. }`.
pub(crate) fn while_stmt(checker: &mut Checker<'_>, cond: &Expr, body: &Block) {
    let cond_ty = expr::check(checker, cond);
    if cond_ty != Ty::Bool && !cond_ty.is_error() {
        checker.error(
            cond.span,
            code::CONDITION_NOT_BOOL,
            format!(
                "`while` condition must be `bool`, found `{}`",
                cond_ty.name()
            ),
        );
    }
    let base = checker.snapshot();
    checker.loops += 1;
    stmt::block(checker, body);
    checker.loops -= 1;
    checker.restore(base);
}

/// Checks `return [expr];`.
pub(crate) fn return_stmt(checker: &mut Checker<'_>, span: Span, expr: Option<&Expr>) {
    let Some(expr) = expr else {
        if checker.ret != Ty::Void && !checker.ret.is_error() {
            checker.error(
                span,
                code::RETURN_TYPE_MISMATCH,
                format!("`return` must produce a `{}`", checker.ret.name()),
            );
        }
        return;
    };
    let value_ty = expr::check(checker, expr);
    if !compatible(&checker.ret, &value_ty) {
        checker.error(
            expr.span,
            code::RETURN_TYPE_MISMATCH,
            format!(
                "expected `{}`, found `{}`",
                checker.ret.name(),
                value_ty.name()
            ),
        );
    }
}

/// Checks `break;`.
pub(crate) fn break_stmt(checker: &mut Checker<'_>, span: Span) {
    if checker.loops == 0 {
        checker.error(
            span,
            code::BREAK_OUTSIDE_LOOP,
            "`break` is outside any loop",
        );
    }
}

/// Checks `continue;`.
pub(crate) fn continue_stmt(checker: &mut Checker<'_>, span: Span) {
    if checker.loops == 0 {
        checker.error(
            span,
            code::CONTINUE_OUTSIDE_LOOP,
            "`continue` is outside any loop",
        );
    }
}
