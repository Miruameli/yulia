//! Phase 1b LLVM code generation (0.2 §6.3).
//!
//! Lowers a checked [`Program`] to an LLVM [`Module`] through the inkwell
//! bindings. Every expression's type is read from the [`TypeMap`] the checker
//! produced, so the generator never re-derives a type.

mod lower;
mod ty;

use std::fmt;
use std::path::Path;

use inkwell::OptimizationLevel;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};

use crate::ast::Program;
use crate::region::Assignment;
use crate::typeck::TypeMap;

use lower::Lower;

/// A code generation failure.
#[derive(Debug)]
pub struct CodegenError {
    message: String,
}

impl CodegenError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for CodegenError {}

/// Converts an element count to LLVM's `u32` array length.
fn array_len(len: usize) -> Result<u32, CodegenError> {
    u32::try_from(len)
        .map_err(|_| CodegenError::new(format!("array length {len} does not fit in u32")))
}

/// Turns an inkwell builder failure into a [`CodegenError`].
pub(crate) fn builder_error(error: impl fmt::Display) -> CodegenError {
    CodegenError::new(format!("LLVM could not build an instruction: {error}"))
}

/// Lowers a checked program to an LLVM module.
///
/// `assignment` is the region assignment of 0.3 §3. Phase 1b lowers every
/// region to a stack slot, so the assignment is informational today; it is
/// accepted to keep the backend signature stable for Phase 2 region lowering.
pub fn generate<'ctx>(
    context: &'ctx Context,
    program: &Program,
    types: &TypeMap,
    assignment: &Assignment,
) -> Result<Module<'ctx>, CodegenError> {
    let module = context.create_module(&program.module.path.join("."));
    let mut lower = Lower::new(context, module, types, program, assignment);
    lower.run()?;
    Ok(lower.module)
}

/// Writes the module's LLVM IR to `path`.
pub fn write_ir(module: &Module<'_>, path: &Path) -> Result<(), CodegenError> {
    module.print_to_file(path).map_err(|message| {
        CodegenError::new(format!(
            "cannot write IR to `{}`: {message}",
            path.display()
        ))
    })
}

/// Writes the module's native object file to `path`.
pub fn write_object(module: &Module<'_>, path: &Path) -> Result<(), CodegenError> {
    Target::initialize_native(&InitializationConfig::default()).map_err(|message| {
        CodegenError::new(format!("cannot initialize the native target: {message}"))
    })?;
    let triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&triple).map_err(|message| {
        CodegenError::new(format!("cannot resolve the native target: {message}"))
    })?;
    let machine = target
        .create_target_machine(
            &triple,
            "",
            "",
            OptimizationLevel::Default,
            RelocMode::Default,
            CodeModel::Default,
        )
        .ok_or_else(|| CodegenError::new("no target machine for the native triple".to_string()))?;
    machine
        .write_to_file(module, FileType::Object, path)
        .map_err(|message| {
            CodegenError::new(format!(
                "cannot write object to `{}`: {message}",
                path.display()
            ))
        })
}

#[cfg(test)]
mod tests;
