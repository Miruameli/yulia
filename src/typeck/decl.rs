//! Declaration checking: signatures first, then bodies (0.2 §6).

mod body;

use std::collections::HashSet;
use std::rc::Rc;

use crate::ast::{DeclKind, Program, StructDecl};
use crate::diagnostics::code;
use crate::typeck::{Checker, FieldSig, StructSig};

use body::{check_const, check_fn, signature};

/// Collects struct, function, and constant signatures.
pub(crate) fn collect(checker: &mut Checker<'_>, program: &Program) {
    // Pass 1: reserve every struct name so fields can name each other.
    let mut duplicates = HashSet::new();
    for decl in &program.decls {
        if let DeclKind::Struct(decl) = &decl.kind {
            if checker.structs.contains_key(&decl.name) {
                duplicates.insert(decl.name.clone());
                checker.error(
                    decl.span,
                    code::DUPLICATE_DEFINITION,
                    format!("`{}` is defined more than once", decl.name),
                );
                continue;
            }
            checker.structs.insert(
                decl.name.clone(),
                Rc::new(StructSig {
                    name: decl.name.clone(),
                    fields: Vec::new(),
                }),
            );
        }
    }
    for import in &program.imports {
        checker.error(
            import.span,
            code::OUT_OF_SUBSET,
            "`import` is outside the Phase 1 subset",
        );
    }
    // Pass 2: fill in the fields now that every name is known.
    for decl in &program.decls {
        if let DeclKind::Struct(decl) = &decl.kind
            && !duplicates.contains(&decl.name)
        {
            fill_struct(checker, decl);
        }
    }
    // Pass 3: functions and constants.
    for decl in &program.decls {
        match &decl.kind {
            DeclKind::Fn(decl) => {
                let sig = signature(checker, &decl.params, &decl.ret);
                if checker.fns.insert(decl.name.clone(), sig).is_some() {
                    checker.error(
                        decl.span,
                        code::DUPLICATE_DEFINITION,
                        format!("`{}` is defined more than once", decl.name),
                    );
                }
            }
            DeclKind::Extern(decl) => {
                checker.error(
                    decl.span,
                    code::OUT_OF_SUBSET,
                    "`extern` is outside the Phase 1 subset",
                );
            }
            DeclKind::Const(decl) => {
                let ty = checker.resolve(&decl.ty);
                if checker.consts.insert(decl.name.clone(), ty).is_some() {
                    checker.error(
                        decl.span,
                        code::DUPLICATE_DEFINITION,
                        format!("`{}` is defined more than once", decl.name),
                    );
                }
            }
            DeclKind::Struct(_) => {}
        }
    }
}

/// Fills in the fields of a reserved struct.
fn fill_struct(checker: &mut Checker<'_>, decl: &StructDecl) {
    let mut fields = Vec::new();
    for field in &decl.fields {
        if fields
            .iter()
            .any(|known: &FieldSig| known.name == field.name)
        {
            checker.error(
                field.span,
                code::DUPLICATE_DEFINITION,
                format!("field `{}` is declared more than once", field.name),
            );
            continue;
        }
        fields.push(FieldSig {
            name: field.name.clone(),
            ty: checker.resolve(&field.ty),
        });
    }
    checker.structs.insert(
        decl.name.clone(),
        Rc::new(StructSig {
            name: decl.name.clone(),
            fields,
        }),
    );
}

/// Checks every declaration body.
pub(crate) fn check(checker: &mut Checker<'_>, program: &Program) {
    for decl in &program.decls {
        match &decl.kind {
            DeclKind::Fn(decl) => check_fn(checker, decl),
            DeclKind::Const(decl) => check_const(checker, decl),
            DeclKind::Extern(_) | DeclKind::Struct(_) => {}
        }
    }
}
