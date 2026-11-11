//! Lexical tokens (0.1 §2.5–§2.7).

use crate::diagnostics::Span;

/// The lexical category of a token.
///
/// Keywords are reserved (0.1 §2.5) and are lexed as their own kinds even when
/// the current phase does not use them, so a later phase needs no re-lexing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // Keywords (0.1 §2.5).
    /// `module`
    Module,
    /// `import`
    Import,
    /// `private`
    Private,
    /// `struct`
    Struct,
    /// `fn`
    Fn,
    /// `let`
    Let,
    /// `const`
    Const,
    /// `if`
    If,
    /// `else`
    Else,
    /// `while`
    While,
    /// `for`
    For,
    /// `in`
    In,
    /// `return`
    Return,
    /// `break`
    Break,
    /// `continue`
    Continue,
    /// `true`
    True,
    /// `false`
    False,
    /// `int`
    Int,
    /// `float`
    Float,
    /// `bool`
    Bool,
    /// `char`
    Char,
    /// `string`
    String,
    /// `void`
    Void,
    /// `extern`
    Extern,
    /// `export`
    Export,
    /// `pin`
    Pin,
    /// `panic`
    Panic,
    /// `thread`
    Thread,
    /// `spawn`
    Spawn,
    /// `async`
    Async,
    /// `await`
    Await,
    /// `send`
    Send,
    /// `recv`
    Recv,
    /// `mutex`
    Mutex,
    /// `defer`
    Defer,
    /// `yield`
    Yield,
    /// `match`
    Match,
    /// `trait`
    Trait,
    /// `impl`
    Impl,
    // Literals (0.1 §2.6).
    /// An integer literal.
    IntLit,
    /// A floating-point literal.
    FloatLit,
    /// A boolean literal.
    BoolLit,
    /// A character literal.
    CharLit,
    /// A string literal.
    StringLit,
    /// An identifier.
    Ident,
    // Operators (0.1 §2.7).
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `==`
    EqEq,
    /// `!=`
    NotEq,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `<=`
    Le,
    /// `>=`
    Ge,
    /// `&&`
    AndAnd,
    /// `||`
    OrOr,
    /// `!`
    Not,
    /// `=`
    Eq,
    // Delimiters (0.1 §2.7).
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `[`
    LBracket,
    /// `]`
    RBracket,
    /// `;`
    Semi,
    /// `,`
    Comma,
    /// `:`
    Colon,
    /// `.`
    Dot,
    /// `..`
    DotDot,
    /// `->`
    Arrow,
    /// End of input.
    Eof,
}

/// A single lexical token: its kind, its source text, and its span.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'a> {
    /// The lexical category.
    pub kind: TokenKind,
    /// The exact source text of the token.
    pub text: &'a str,
    /// The source range the token covers.
    pub span: Span,
}
