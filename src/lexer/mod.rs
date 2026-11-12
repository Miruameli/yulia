//! The Yulia lexer (0.1 §2).
//!
//! The lexer turns source text into a token stream plus diagnostics. It is the
//! first stage of the compiler pipeline and owns only lexical concerns: it does
//! not parse, type-check, or infer regions.
//!
//! # Status
//!
//! Phase 1a: the full lexical grammar is implemented, including keywords
//! reserved for later phases.

mod kind_name;

mod literal;
mod scan;
mod token;

pub(crate) use literal::{decode_char, decode_string};
pub use scan::is_keyword_text;
pub use token::{Token, TokenKind};

/// Tokenizes Yulia source text (0.1 §2).
///
/// The lexer recovers from lexical errors: an unexpected character is reported
/// and skipped so one run can surface many problems.
#[derive(Debug)]
pub struct Lexer<'a> {
    /// The full source text.
    source: &'a str,
    /// The current byte offset.
    pos: usize,
}
