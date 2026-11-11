//! Expression syntax (0.1 §3.4).

use crate::diagnostics::Span;

/// A parsed expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expr {
    /// The expression category.
    pub kind: ExprKind,
    /// The span of the whole expression.
    pub span: Span,
}

/// The expression categories of 0.1 §3.4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprKind {
    /// An integer literal, kept as written so the type checker can parse it.
    IntLit(String),
    /// A float literal, kept as written.
    FloatLit(String),
    /// `true` or `false`.
    BoolLit(bool),
    /// A character literal, decoded from its escapes.
    CharLit(char),
    /// A string literal, decoded from its escapes.
    StringLit(String),
    /// A name: a variable, a constant, or a function.
    Ident(String),
    /// `!e` or `-e`.
    Unary {
        /// The operator.
        op: UnaryOp,
        /// The operand.
        operand: Box<Expr>,
    },
    /// A binary operation.
    Binary {
        /// The operator.
        op: BinaryOp,
        /// The left operand.
        lhs: Box<Expr>,
        /// The right operand.
        rhs: Box<Expr>,
    },
    /// `target = value`.
    Assign {
        /// The assignment target.
        target: Box<Expr>,
        /// The assigned value.
        value: Box<Expr>,
    },
    /// `callee(args)`.
    Call {
        /// The called expression.
        callee: Box<Expr>,
        /// The arguments.
        args: Vec<Expr>,
    },
    /// `base.name`.
    Field {
        /// The base expression.
        base: Box<Expr>,
        /// The field name.
        name: String,
    },
    /// `base[index]`.
    Index {
        /// The base expression.
        base: Box<Expr>,
        /// The index expression.
        index: Box<Expr>,
    },
    /// `Name { field: value, .. }`.
    StructLiteral {
        /// The struct type name.
        name: String,
        /// The field initializers.
        fields: Vec<FieldInit>,
    },
    /// `[a, b, c]`.
    ArrayLiteral {
        /// The elements.
        elements: Vec<Expr>,
    },
    /// `[value; count]`.
    ArrayRepeat {
        /// The repeated value.
        value: Box<Expr>,
        /// The element count, as written in the source.
        count: String,
    },
    /// `panic("message")` or `panic()`.
    Panic(Option<Box<Expr>>),
}

/// A `name: value` entry in a struct literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldInit {
    /// The field name.
    pub name: String,
    /// The initializer.
    pub value: Expr,
    /// The span of the whole entry.
    pub span: Span,
}

/// The prefix operators (0.1 §2.8).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    /// `!`
    Not,
    /// `-`
    Neg,
}

/// The binary operators (0.1 §2.8).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    /// `+`
    Add,
    /// `-`
    Sub,
    /// `*`
    Mul,
    /// `/`
    Div,
    /// `%`
    Rem,
    /// `==`
    Eq,
    /// `!=`
    Ne,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `<=`
    Le,
    /// `>=`
    Ge,
    /// `&&`
    And,
    /// `||`
    Or,
}
