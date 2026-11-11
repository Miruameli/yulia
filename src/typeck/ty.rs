//! The Phase 1 type representation (0.2 §6.1).
//!
//! `T ::= int | bool | StructT | ArrayT(T, N)`. The Phase 1 subset has no
//! `float`, `char`, `string`, or pointers, so those parse but do not resolve.

use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::{Type, TypeKind};

/// A resolved type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    /// `int`.
    Int,
    /// `bool`.
    Bool,
    /// `void` — only valid as a function return type.
    Void,
    /// `[T; N]`.
    Array(Box<Ty>, usize),
    /// A user-defined struct, by name.
    Struct(String),
    /// A type that could not be determined; suppresses cascading errors.
    Error,
}

/// Why a type annotation did not resolve.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolveError {
    /// The name is not a declared struct.
    Unknown(String),
    /// The type is valid but outside the Phase 1 subset.
    OutOfScope(&'static str),
    /// The array length is not a positive integer.
    BadLength,
}

/// A struct signature: its name and fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructSig {
    /// The struct name.
    pub name: String,
    /// The fields, in declaration order.
    pub fields: Vec<FieldSig>,
}

/// A `name: type` struct field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldSig {
    /// The field name.
    pub name: String,
    /// The field type.
    pub ty: Ty,
}

/// A function signature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnSig {
    /// The parameter types.
    pub params: Vec<Ty>,
    /// The return type.
    pub ret: Ty,
}

impl Ty {
    /// Resolves a parsed type annotation against the declared structs.
    pub fn resolve(
        ty: &Type,
        structs: &HashMap<String, Rc<StructSig>>,
    ) -> Result<Ty, ResolveError> {
        match &ty.kind {
            TypeKind::Int => Ok(Ty::Int),
            TypeKind::Bool => Ok(Ty::Bool),
            TypeKind::Void => Ok(Ty::Void),
            TypeKind::Array { elem, len } => {
                let elem = Self::resolve(elem, structs)?;
                let len = len.parse::<usize>().map_err(|_| ResolveError::BadLength)?;
                Ok(Ty::Array(Box::new(elem), len))
            }
            TypeKind::Named(name) => match structs.get(name) {
                Some(_) => Ok(Ty::Struct(name.clone())),
                None => Err(ResolveError::Unknown(name.clone())),
            },
            TypeKind::Float => Err(ResolveError::OutOfScope("float")),
            TypeKind::Char => Err(ResolveError::OutOfScope("char")),
            TypeKind::String => Err(ResolveError::OutOfScope("string")),
            TypeKind::CPtr => Err(ResolveError::OutOfScope("CPtr")),
        }
    }

    /// Returns `true` for [`Ty::Error`].
    #[must_use]
    pub fn is_error(&self) -> bool {
        matches!(self, Ty::Error)
    }

    /// Returns the type's source spelling, for diagnostics.
    #[must_use]
    pub fn name(&self) -> String {
        match self {
            Ty::Int => "int".to_string(),
            Ty::Bool => "bool".to_string(),
            Ty::Void => "void".to_string(),
            Ty::Array(elem, len) => format!("[{}; {len}]", elem.name()),
            Ty::Struct(name) => name.clone(),
            Ty::Error => "<error>".to_string(),
        }
    }
}

/// Returns `true` when a value of type `actual` can be used where `expected`
/// is required. [`Ty::Error`] is compatible with everything so one bad type
/// does not cascade.
pub(crate) fn compatible(expected: &Ty, actual: &Ty) -> bool {
    if expected.is_error() || actual.is_error() {
        return true;
    }
    match (expected, actual) {
        (Ty::Array(expected, n), Ty::Array(actual, m)) => n == m && compatible(expected, actual),
        _ => expected == actual,
    }
}
