//! Field and index checking (0.2 §6.2).

use crate::ast::Expr;
use crate::diagnostics::code;
use crate::typeck::{Checker, Ty};

use super::check;

/// Checks a field access.
pub(crate) fn field(checker: &mut Checker<'_>, base: &Expr, name: &str) -> Ty {
    let base_ty = check(checker, base);
    let Ty::Struct(struct_name) = &base_ty else {
        if !base_ty.is_error() {
            checker.error(
                base.span,
                code::NOT_A_STRUCT,
                format!("`{}` has no fields", base_ty.name()),
            );
        }
        return Ty::Error;
    };
    let Some(sig) = checker.structs.get(struct_name) else {
        return Ty::Error;
    };
    let Some(field) = sig.fields.iter().find(|field| field.name == name) else {
        checker.error(
            base.span,
            code::UNKNOWN_FIELD,
            format!("`{}` has no field `{name}`", sig.name),
        );
        return Ty::Error;
    };
    field.ty.clone()
}

/// Checks an index expression.
pub(crate) fn index(checker: &mut Checker<'_>, base: &Expr, index: &Expr) -> Ty {
    let base_ty = check(checker, base);
    let index_ty = check(checker, index);
    if index_ty != Ty::Int && !index_ty.is_error() {
        checker.error(
            index.span,
            code::INDEX_NOT_INT,
            format!("array index must be `int`, found `{}`", index_ty.name()),
        );
    }
    if let Ty::Array(elem, _) = &base_ty {
        return (**elem).clone();
    }
    if !base_ty.is_error() {
        checker.error(
            base.span,
            code::NOT_AN_ARRAY,
            format!("`{}` cannot be indexed", base_ty.name()),
        );
    }
    Ty::Error
}
