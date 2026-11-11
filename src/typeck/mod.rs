//! The Phase 1 type checker (0.2 §6).
//!
//! Resolves every expression to a [`Ty`] and reports a diagnostic for each
//! ill-typed program. Constructs outside the Phase 1 subset (D1) are rejected
//! with [`code::OUT_OF_SUBSET`] rather than silently accepted.

mod decl;
mod expr;
mod scopes;
mod stmt;
mod ty;

use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::{Program, Type};
use crate::diagnostics::code;
use crate::diagnostics::{Code, Diagnostic, Span};

pub(crate) use ty::compatible;
pub use ty::{FieldSig, FnSig, ResolveError, StructSig, Ty};

/// Type checks a whole program.
#[derive(Debug)]
pub struct Checker<'a> {
    program: &'a Program,
    diagnostics: Vec<Diagnostic>,
    structs: HashMap<String, Rc<StructSig>>,
    fns: HashMap<String, FnSig>,
    consts: HashMap<String, Ty>,
    scopes: Vec<HashMap<String, VarState>>,
    ret: Ty,
    loops: usize,
}

/// A local binding and its definite-assignment state.
#[derive(Debug, Clone)]
pub(crate) struct VarState {
    pub(crate) ty: Ty,
    pub(crate) init: bool,
}

impl<'a> Checker<'a> {
    /// Creates a checker for `program`.
    #[must_use]
    pub fn new(program: &'a Program) -> Self {
        Self {
            program,
            diagnostics: Vec::new(),
            structs: HashMap::new(),
            fns: HashMap::new(),
            consts: HashMap::new(),
            scopes: Vec::new(),
            ret: Ty::Void,
            loops: 0,
        }
    }

    /// Checks the program and returns its diagnostics.
    pub fn check(mut self) -> Vec<Diagnostic> {
        let program = self.program;
        decl::collect(&mut self, program);
        decl::check(&mut self, program);
        self.diagnostics
    }

    /// Reports a diagnostic at `span`.
    pub(crate) fn error(&mut self, span: Span, code: Code, message: impl Into<String>) {
        self.diagnostics.push(Diagnostic::new(code, message, span));
    }

    /// Resolves a type annotation, reporting why it failed.
    pub(crate) fn resolve(&mut self, ty: &Type) -> Ty {
        match Ty::resolve(ty, &self.structs) {
            Ok(ty) => ty,
            Err(ResolveError::Unknown(name)) => {
                self.error(
                    ty.span,
                    code::UNKNOWN_TYPE,
                    format!("unknown type `{name}`"),
                );
                Ty::Error
            }
            Err(ResolveError::OutOfScope(what)) => {
                self.error(
                    ty.span,
                    code::OUT_OF_SUBSET,
                    format!("`{what}` is outside the Phase 1 subset"),
                );
                Ty::Error
            }
            Err(ResolveError::BadLength) => {
                self.error(
                    ty.span,
                    code::BAD_ARRAY_LENGTH,
                    "array length must be a positive integer",
                );
                Ty::Error
            }
        }
    }
}
