//! Top-level declaration syntax (0.1 §3.1).

use crate::ast::expr::Expr;
use crate::ast::stmt::Block;
use crate::ast::ty::Type;
use crate::diagnostics::Span;

/// A top-level declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decl {
    /// The declaration category.
    pub kind: DeclKind,
    /// The span of the whole declaration.
    pub span: Span,
}

/// The top-level declaration categories of 0.1 §3.1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclKind {
    /// `struct Name { .. }`
    Struct(StructDecl),
    /// `fn name(..) -> T { .. }`
    Fn(FnDecl),
    /// `extern "C" fn name(..) -> T;`
    Extern(ExternDecl),
    /// `const NAME: T = value;`
    Const(ConstDecl),
}

/// The visibility of a declaration (0.1 §4.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Visibility {
    /// Public by default.
    #[default]
    Public,
    /// `private` — module-local.
    Private,
    /// `export` — callable from C.
    Export,
}

/// A `struct` declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructDecl {
    /// The struct name.
    pub name: String,
    /// The visibility.
    pub visibility: Visibility,
    /// The fields.
    pub fields: Vec<Field>,
    /// The span of the whole declaration.
    pub span: Span,
}

/// A `name: type;` struct field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    /// The field name.
    pub name: String,
    /// The field type.
    pub ty: Type,
    /// The span of the whole field.
    pub span: Span,
}

/// A function declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnDecl {
    /// The function name.
    pub name: String,
    /// The visibility.
    pub visibility: Visibility,
    /// The parameters.
    pub params: Vec<Param>,
    /// The return type (`void` when omitted).
    pub ret: Type,
    /// The function body.
    pub body: Block,
    /// The span of the whole declaration.
    pub span: Span,
}

/// An `extern` function declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternDecl {
    /// The ABI string, e.g. `"C"`.
    pub abi: String,
    /// The function name.
    pub name: String,
    /// The parameters.
    pub params: Vec<Param>,
    /// The return type (`void` when omitted).
    pub ret: Type,
    /// The span of the whole declaration.
    pub span: Span,
}

/// A `name: type` parameter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    /// The parameter name.
    pub name: String,
    /// The parameter type.
    pub ty: Type,
    /// The span of the whole parameter.
    pub span: Span,
}

/// A `const` declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstDecl {
    /// The constant name.
    pub name: String,
    /// The visibility.
    pub visibility: Visibility,
    /// The declared type.
    pub ty: Type,
    /// The value.
    pub value: Expr,
    /// The span of the whole declaration.
    pub span: Span,
}
