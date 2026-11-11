//! Function and constant body checking (0.2 §6).

use crate::ast::{ConstDecl, FnDecl, Param, Type, Visibility};
use crate::diagnostics::code;
use crate::typeck::{Checker, FnSig, Ty};

use crate::typeck::compatible;
use crate::typeck::expr;
use crate::typeck::stmt;

/// Checks a function body.
pub(crate) fn check_fn(checker: &mut Checker<'_>, decl: &FnDecl) {
    if decl.visibility == Visibility::Export {
        checker.error(
            decl.span,
            code::OUT_OF_SUBSET,
            "`export` is outside the Phase 1 subset",
        );
    }
    // The signature was resolved during collection; reuse it so a bad type is
    // reported once rather than once per pass.
    let Some(sig) = checker.fns.get(&decl.name).cloned() else {
        return;
    };
    checker.ret = sig.ret;
    checker.scopes.clear();
    checker.push_scope();
    for (param, ty) in decl.params.iter().zip(&sig.params) {
        if checker.local(&param.name).is_some() {
            checker.error(
                param.span,
                code::DUPLICATE_PARAM,
                format!("`{}` is bound more than once", param.name),
            );
            continue;
        }
        checker.declare(param.name.clone(), ty.clone(), true);
    }
    stmt::block(checker, &decl.body);
    checker.pop_scope();
    checker.ret = Ty::Void;
}

/// Checks a constant initializer.
pub(crate) fn check_const(checker: &mut Checker<'_>, decl: &ConstDecl) {
    let declared = checker.resolve(&decl.ty);
    let value_ty = expr::check(checker, &decl.value);
    if !compatible(&declared, &value_ty) {
        checker.error(
            decl.value.span,
            code::TYPE_MISMATCH,
            format!(
                "cannot initialize `{}` with `{}`",
                declared.name(),
                value_ty.name()
            ),
        );
    }
}

/// Builds a function signature from its parameters and return type.
pub(crate) fn signature(checker: &mut Checker<'_>, params: &[Param], ret: &Type) -> FnSig {
    FnSig {
        params: params
            .iter()
            .map(|param| checker.resolve(&param.ty))
            .collect(),
        ret: checker.resolve(ret),
    }
}
