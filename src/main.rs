//! `yulia` — the Yulia compiler driver.
//!
//! Phase 1b driver: it parses command-line arguments, runs the front-end
//! pipeline (lex, parse, type check, region inference) and, when an output
//! path is given, lowers the accepted program through the LLVM code generator
//! (`src/codegen`). With no output path it stays a compile-check-only tool.

mod cli;

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use inkwell::context::Context;
use yulia::codegen;
use yulia::compile::Compilation;
use yulia::diagnostics::{Diagnostic, locate};
use yulia::{LICENSE, VERSION};

use crate::cli::{EmitKind, Options, Parsed, USAGE, parse_args};

fn main() -> ExitCode {
    // `args_os`, not `args`: a non-UTF-8 argument must be reported, not
    // panicked on.
    let args: Vec<OsString> = std::env::args_os().skip(1).collect();
    match parse_args(&args) {
        Ok(Parsed::Help) => {
            print!("{USAGE}");
            ExitCode::SUCCESS
        }
        Ok(Parsed::Version) => {
            println!("yulia {VERSION} (license: {LICENSE})");
            ExitCode::SUCCESS
        }
        Ok(Parsed::Run(options)) => run(options),
        Err(message) => {
            eprintln!("yulia: {message}");
            eprintln!("try 'yulia --help'");
            ExitCode::FAILURE
        }
    }
}

/// Compiles the input file, then writes the requested artifact.
fn run(options: Options) -> ExitCode {
    let Some(input) = options.input else {
        print!("{USAGE}");
        return ExitCode::SUCCESS;
    };
    // The original `OsStr` is kept for the input path so a non-UTF-8 filename
    // still reaches the filesystem intact.
    let Ok(source) = std::fs::read_to_string(&input) else {
        eprintln!("yulia: cannot read '{}'", input.to_string_lossy());
        return ExitCode::FAILURE;
    };
    if source.trim().is_empty() {
        eprintln!("yulia: empty input: '{}'", input.to_string_lossy());
        return ExitCode::FAILURE;
    }
    let compilation = match yulia::compile::compile(&source) {
        Ok(compilation) => compilation,
        Err(diagnostics) => {
            report(&source, &input.to_string_lossy(), &diagnostics);
            return ExitCode::FAILURE;
        }
    };
    let Some(output) = options.output else {
        return ExitCode::SUCCESS;
    };
    emit(&compilation, Path::new(&output), options.emit)
}

/// Prints every diagnostic as `file:line:col: error[code]: message`.
fn report(source: &str, path: &str, diagnostics: &[Diagnostic]) {
    for diagnostic in diagnostics {
        let location = locate(source, diagnostic.span.start);
        eprintln!(
            "{}:{}:{}: error[{}]: {}",
            path, location.line, location.column, diagnostic.code.0, diagnostic.message
        );
    }
}

/// Lowers `compilation` and writes the artifact selected by `emit`.
fn emit(compilation: &Compilation, output: &Path, emit: EmitKind) -> ExitCode {
    let context = Context::create();
    let module = match codegen::generate(
        &context,
        &compilation.program,
        &compilation.types,
        &compilation.regions,
    ) {
        Ok(module) => module,
        Err(error) => {
            eprintln!("yulia: codegen failed: {error}");
            return ExitCode::FAILURE;
        }
    };
    let result = match emit {
        EmitKind::Ir => codegen::write_ir(&module, output).map_err(|error| error.to_string()),
        EmitKind::Obj => codegen::write_object(&module, output).map_err(|error| error.to_string()),
        EmitKind::Exe => {
            let object = object_path();
            let result = match codegen::write_object(&module, &object) {
                Ok(()) => link(&object, output),
                Err(error) => Err(error.to_string()),
            };
            let _ = std::fs::remove_file(&object);
            result
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("yulia: {message}");
            ExitCode::FAILURE
        }
    }
}

/// Links an object file into an executable with `cc`.
fn link(object: &Path, output: &Path) -> Result<(), String> {
    let status = Command::new("cc")
        .arg(object)
        .arg("-o")
        .arg(output)
        .status()
        .map_err(|error| format!("cannot run the linker 'cc': {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("linker failed: {status}"))
    }
}

/// A unique temporary object path for the link step.
fn object_path() -> PathBuf {
    std::env::temp_dir().join(format!("yulia-{}.o", std::process::id()))
}
