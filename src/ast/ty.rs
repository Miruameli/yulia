//! Type syntax (0.1 §3.2).

use crate::diagnostics::Span;

/// A parsed type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    /// The type category.
    pub kind: TypeKind,
    /// The span of the whole type.
    pub span: Span,
}

/// The type categories of 0.1 §3.2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeKind {
    /// `int`.
    Int,
    /// `float`.
    Float,
    /// `bool`.
    Bool,
    /// `char`.
    Char,
    /// `string`.
    String,
    /// `void` — only valid as a function return type.
    Void,
    /// `[T; N]`.
    Array {
        /// The element type.
        elem: Box<Type>,
        /// The element count, as written in the source.
        len: String,
    },
    /// `CPtr` — the opaque C pointer (Phase 2).
    CPtr,
    /// A user-defined struct type.
    Named(String),
}
