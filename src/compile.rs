//! The compiler pipeline: source text to a checked program.
//!
//! The stages run in order — lexer, parser, type checker, region inference —
//! and the pipeline stops at the first stage that reports a diagnostic: a later
//! stage cannot run on a program an earlier stage rejected.

use crate::ast::Program;
use crate::diagnostics::Diagnostic;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::region::Assignment;
use crate::typeck::{Checker, TypeMap};

/// A program that passed every Phase 1a stage.
#[derive(Debug)]
pub struct Compilation {
    /// The parsed program, handed to the code generator.
    pub program: Program,
    /// The resolved type of every expression, keyed by its span.
    pub types: TypeMap,
    /// The region assignment for the program.
    pub regions: Assignment,
}

/// Runs the Phase 1a pipeline over `source`.
///
/// Returns the diagnostics of the first failing stage, or the compiled program
/// of the accepted one.
pub fn compile(source: &str) -> Result<Compilation, Vec<Diagnostic>> {
    let (tokens, lexed) = Lexer::new(source).tokenize();
    if !lexed.is_empty() {
        return Err(lexed);
    }
    let (program, parsed) = Parser::new(&tokens).parse();
    if !parsed.is_empty() {
        return Err(parsed);
    }
    let mut checker = Checker::new(&program);
    let diagnostics = checker.check();
    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }
    Ok(Compilation {
        program,
        types: checker.types().clone(),
        regions: Assignment::new(&program),
    })
}
