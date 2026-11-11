//! Struct and array literal checking (0.2 §6.2).

use crate::ast::{Expr, FieldInit};
use crate::diagnostics::code;
use crate::typeck::{Checker, StructSig, Ty};

use super::check;
use crate::typeck::compatible;

/// Checks a struct literal.
pub(crate) fn struct_literal(
    checker: &mut Checker<'_>,
    expr: &Expr,
    name: &str,
    fields: &[FieldInit],
) -> Ty {
    let Some(sig) = checker.structs.get(name).cloned() else {
        checker.error(
            expr.span,
            code::UNKNOWN_STRUCT,
            format!("`{name}` is not a declared struct"),
        );
        return Ty::Error;
    };
    for declared in &sig.fields {
        let matches: Vec<&FieldInit> = fields
            .iter()
            .filter(|field| field.name == declared.name)
            .collect();
        if matches.is_empty() {
            checker.error(
                expr.span,
                code::MISSING_FIELD,
                format!("`{}` is missing field `{}`", sig.name, declared.name),
            );
            continue;
        }
        if matches.len() > 1 {
            checker.error(
                matches[1].span,
                code::DUPLICATE_FIELD,
                format!("field `{}` is initialized more than once", declared.name),
            );
        }
        let value_ty = check(checker, &matches[0].value);
        if !compatible(&declared.ty, &value_ty) {
            checker.error(
                matches[0].value.span,
                code::TYPE_MISMATCH,
                format!(
                    "field `{}` expects `{}`, found `{}`",
                    declared.name,
                    declared.ty.name(),
                    value_ty.name()
                ),
            );
        }
    }
    unknown_fields(checker, &sig, fields);
    Ty::Struct(name.to_string())
}

/// Reports initializers for fields the struct does not declare.
fn unknown_fields(checker: &mut Checker<'_>, sig: &StructSig, fields: &[FieldInit]) {
    for field in fields {
        if !sig
            .fields
            .iter()
            .any(|declared| declared.name == field.name)
        {
            checker.error(
                field.span,
                code::UNKNOWN_FIELD,
                format!("`{}` has no field `{}`", sig.name, field.name),
            );
        }
    }
}

/// Checks an array literal.
pub(crate) fn array_literal(checker: &mut Checker<'_>, elements: &[Expr]) -> Ty {
    let mut elem_ty = Ty::Error;
    for (index, element) in elements.iter().enumerate() {
        let ty = check(checker, element);
        if index == 0 {
            elem_ty = ty;
        } else if !compatible(&elem_ty, &ty) {
            checker.error(
                element.span,
                code::ARRAY_ELEMENT_MISMATCH,
                format!(
                    "array elements must share a type, expected `{}`, found `{}`",
                    elem_ty.name(),
                    ty.name()
                ),
            );
        }
    }
    Ty::Array(Box::new(elem_ty), elements.len())
}

/// Checks a repeat array literal.
pub(crate) fn array_repeat(
    checker: &mut Checker<'_>,
    expr: &Expr,
    value: &Expr,
    count: &str,
) -> Ty {
    let value_ty = check(checker, value);
    let Ok(len) = count.parse::<usize>() else {
        checker.error(
            expr.span,
            code::BAD_ARRAY_LENGTH,
            "array length must be a positive integer",
        );
        return Ty::Array(Box::new(value_ty), 0);
    };
    Ty::Array(Box::new(value_ty), len)
}
