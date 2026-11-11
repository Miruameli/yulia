//! Statement syntax (0.1 §3.3).

use crate::ast::expr::Expr;
use crate::ast::ty::Type;
use crate::diagnostics::Span;

/// A `{ .. }` block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    /// The statements in the block.
    pub stmts: Vec<Stmt>,
    /// The span of the whole block, braces included.
    pub span: Span,
}

/// A parsed statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stmt {
    /// The statement category.
    pub kind: StmtKind,
    /// The span of the whole statement.
    pub span: Span,
}

/// The statement categories of 0.1 §3.3.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StmtKind {
    /// `let name: type [= init];`
    Let {
        /// The bound name.
        name: String,
        /// The declared type.
        ty: Type,
        /// The optional initializer.
        init: Option<Expr>,
    },
    /// `const name: type = value;`
    Const {
        /// The bound name.
        name: String,
        /// The declared type.
        ty: Type,
        /// The value.
        value: Expr,
    },
    /// `pin lvalue as name { .. }`
    Pin {
        /// The pinned target.
        target: Expr,
        /// The alias bound inside the block.
        alias: String,
        /// The pinned block.
        body: Block,
    },
    /// `if cond { .. } [else ..]`
    If {
        /// The condition.
        cond: Expr,
        /// The then-branch.
        then: Block,
        /// The optional else-branch.
        else_branch: Option<ElseBranch>,
    },
    /// `while cond { .. }`
    While {
        /// The condition.
        cond: Expr,
        /// The loop body.
        body: Block,
    },
    /// `for var in start..end { .. }`
    ForRange {
        /// The loop variable.
        var: String,
        /// The range start.
        start: Expr,
        /// The range end.
        end: Expr,
        /// The loop body.
        body: Block,
    },
    /// `for var: type = init; cond; update { .. }`
    ForCStyle {
        /// The initialization statement.
        init: Box<Stmt>,
        /// The loop condition.
        cond: Expr,
        /// The update expression.
        update: Expr,
        /// The loop body.
        body: Block,
    },
    /// `return [expr];`
    Return(Option<Expr>),
    /// `break;`
    Break,
    /// `continue;`
    Continue,
    /// `expr;`
    Expr(Expr),
}

/// The else-branch of an `if` (0.1 §3.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElseBranch {
    /// `else if ..`
    If(Box<Stmt>),
    /// `else { .. }`
    Block(Block),
}
