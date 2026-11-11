//! Identifiers and keywords (0.1 §2.4, §2.5).

use crate::lexer::Lexer;
use crate::lexer::token::TokenKind;

impl Lexer<'_> {
    /// Lexes an identifier or a keyword (0.1 §2.4).
    pub(in crate::lexer) fn lex_ident(&mut self) -> TokenKind {
        let start = self.pos;
        while let Some(c) = self.peek() {
            if is_ident_continue(c) {
                self.bump();
            } else {
                break;
            }
        }
        let text = &self.source[start..self.pos];
        match text {
            "module" => TokenKind::Module,
            "import" => TokenKind::Import,
            "private" => TokenKind::Private,
            "struct" => TokenKind::Struct,
            "fn" => TokenKind::Fn,
            "let" => TokenKind::Let,
            "const" => TokenKind::Const,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "in" => TokenKind::In,
            "return" => TokenKind::Return,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "int" => TokenKind::Int,
            "float" => TokenKind::Float,
            "bool" => TokenKind::Bool,
            "char" => TokenKind::Char,
            "string" => TokenKind::String,
            "void" => TokenKind::Void,
            "extern" => TokenKind::Extern,
            "export" => TokenKind::Export,
            "pin" => TokenKind::Pin,
            "panic" => TokenKind::Panic,
            "thread" => TokenKind::Thread,
            "spawn" => TokenKind::Spawn,
            "async" => TokenKind::Async,
            "await" => TokenKind::Await,
            "send" => TokenKind::Send,
            "recv" => TokenKind::Recv,
            "mutex" => TokenKind::Mutex,
            "defer" => TokenKind::Defer,
            "yield" => TokenKind::Yield,
            "match" => TokenKind::Match,
            "trait" => TokenKind::Trait,
            "impl" => TokenKind::Impl,
            _ => TokenKind::Ident,
        }
    }
}

/// Returns `true` if `c` can start an identifier (0.1 §2.4).
pub(in crate::lexer) fn is_ident_start(c: char) -> bool {
    c == '_' || c.is_alphabetic()
}

/// Returns `true` if `c` can continue an identifier (0.1 §2.4).
fn is_ident_continue(c: char) -> bool {
    c == '_' || c.is_alphabetic() || c.is_ascii_digit()
}
