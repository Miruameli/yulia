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

/// The resolved type of every expression and type annotation (0.2 §6).
///
/// The code generator consumes this map so it never re-derives a type: each
/// expression's [`Ty`] is recorded as the checker resolves it, and each type
/// annotation is recorded under its own [`Span`].
#[derive(Debug, Clone, Default)]
pub struct TypeMap {
    types: HashMap<Span, Ty>,
}

impl TypeMap {
    /// Returns the resolved type recorded at `span`.
    #[must_use]
    pub fn ty(&self, span: Span) -> Option<&Ty> {
        self.types.get(&span)
    }

    /// Records `ty` at `span`.
    pub(crate) fn insert(&mut self, span: Span, ty: Ty) {
        self.types.insert(span, ty);
    }
}

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
    types: TypeMap,
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
            types: TypeMap::default(),
        }
    }

    /// Checks the program and returns its diagnostics.
    ///
    /// The checker is borrowed rather than consumed so the caller can read the
    /// resolved [`TypeMap`] afterwards via [`Checker::types`].
    pub fn check(&mut self) -> Vec<Diagnostic> {
        let program = self.program;
        decl::collect(self, program);
        decl::check(self, program);
        std::mem::take(&mut self.diagnostics)
    }

    /// Returns the resolved type of every expression and type annotation.
    #[must_use]
    pub fn types(&self) -> &TypeMap {
        &self.types
    }

    /// Reports a diagnostic at `span`.
    pub(crate) fn error(&mut self, span: Span, code: Code, message: impl Into<String>) {
        self.diagnostics.push(Diagnostic::new(code, message, span));
    }

    /// Records the resolved type of `span`.
    pub(crate) fn record(&mut self, span: Span, ty: Ty) {
        self.types.insert(span, ty);
    }

    /// Resolves a type annotation, reporting why it failed.
    pub(crate) fn resolve(&mut self, ty: &Type) -> Ty {
        let resolved = match Ty::resolve(ty, &self.structs) {
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
        };
        self.types.insert(ty.span, resolved.clone());
        resolved
    }
}
