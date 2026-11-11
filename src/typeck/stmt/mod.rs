//! Statement type checking (0.2 §6.2).

mod ctrl;

use crate::ast::{Expr, Stmt, StmtKind, Type};
use crate::diagnostics::code;
use crate::typeck::{Checker, Ty};

use super::expr;
use crate::typeck::compatible;

/// Checks a block in a fresh scope.
pub(crate) fn block(checker: &mut Checker<'_>, block: &crate::ast::Block) {
    checker.push_scope();
    for stmt in &block.stmts {
        check(checker, stmt);
    }
    checker.pop_scope();
}

/// Checks a statement.
pub(crate) fn check(checker: &mut Checker<'_>, stmt: &Stmt) {
    match &stmt.kind {
        StmtKind::Let { name, ty, init } => declare_local(checker, ty, name, init.as_ref()),
        StmtKind::Const { name, ty, value } => declare_local(checker, ty, name, Some(value)),
        StmtKind::Pin { body, .. } => ctrl::pin(checker, stmt.span, body),
        StmtKind::If {
            cond,
            then,
            else_branch,
        } => ctrl::if_stmt(checker, cond, then, else_branch.as_ref()),
        StmtKind::While { cond, body } => ctrl::while_stmt(checker, cond, body),
        StmtKind::ForRange {
            var,
            start,
            end,
            body,
        } => {
            ctrl::for_range(checker, var, start, end, body);
        }
        StmtKind::ForCStyle {
            init,
            cond,
            update,
            body,
        } => {
            ctrl::for_c_style(checker, init, cond, update, body);
        }
        StmtKind::Return(expr) => ctrl::return_stmt(checker, stmt.span, expr.as_ref()),
        StmtKind::Break => ctrl::break_stmt(checker, stmt.span),
        StmtKind::Continue => ctrl::continue_stmt(checker, stmt.span),
        StmtKind::Expr(expr) => {
            expr::check(checker, expr);
        }
    }
}

/// Declares a `let` or `const` binding after checking its initializer.
fn declare_local(checker: &mut Checker<'_>, ty: &Type, name: &str, init: Option<&Expr>) {
    let declared = checker.resolve(ty);
    if let Some(expr) = init {
        let init_ty = expr::check(checker, expr);
        if !compatible(&declared, &init_ty) {
            checker.error(
                expr.span,
                code::TYPE_MISMATCH,
                format!(
                    "cannot initialize `{}` with `{}`",
                    declared.name(),
                    init_ty.name()
                ),
            );
        }
    }
    if declared == Ty::Void {
        checker.error(ty.span, code::VOID_VALUE, "`void` cannot be a value type");
    }
    checker.declare(name.to_string(), declared, init.is_some());
}
