//! The Yulia abstract syntax tree (0.1 §3).
//!
//! The AST is the parser's output and the type checker's input. Every node
//! carries the [`Span`] it was parsed from so diagnostics can point at the
//! exact source text.
//!
//! # Status
//!
//! Phase 1a: expressions, statements, declarations, and types are defined.

mod decl;
mod expr;
mod stmt;
mod ty;

pub use decl::{
    ConstDecl, Decl, DeclKind, ExternDecl, Field, FnDecl, Param, StructDecl, Visibility,
};
pub use expr::{BinaryOp, Expr, ExprKind, FieldInit, UnaryOp};
pub use stmt::{Block, ElseBranch, Stmt, StmtKind};
pub use ty::{Type, TypeKind};

use crate::diagnostics::Span;

/// A parsed source file (0.1 §3.1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    /// The module declaration that opens every file.
    pub module: ModuleDecl,
    /// The imports that follow it.
    pub imports: Vec<ImportDecl>,
    /// The top-level declarations.
    pub decls: Vec<Decl>,
}

/// The `module path;` declaration (0.1 §3.1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleDecl {
    /// The dot-separated module path.
    pub path: Vec<String>,
    /// The span of the whole declaration.
    pub span: Span,
}

/// The `import path [as alias];` declaration (0.1 §3.1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportDecl {
    /// The dot-separated module path.
    pub path: Vec<String>,
    /// The optional alias.
    pub alias: Option<String>,
    /// The span of the whole declaration.
    pub span: Span,
}
