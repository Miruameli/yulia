//! `yulia` — the Yulia compiler driver.
//!
//! Phase 1a driver: it parses command-line arguments and runs the front-end
//! pipeline (lex, parse, type check, region inference) over the input file,
//! reporting diagnostics to stderr. Code generation lands in Phase 1b.

use std::ffi::{OsStr, OsString};
use std::process::ExitCode;

use yulia::diagnostics::locate;
use yulia::{LICENSE, VERSION};

const USAGE: &str = "\
Usage: yulia [OPTIONS] [INPUT]

Arguments:
  [INPUT]  Yulia source file to compile

Options:
  -h, --help       Print help
  -V, --version    Print version
";

fn main() -> ExitCode {
    // `args_os`, not `args`: a non-UTF-8 argument must be reported, not
    // panicked on.
    let args: Vec<OsString> = std::env::args_os().skip(1).collect();

    let Some(input) = args.first() else {
        print!("{USAGE}");
        return ExitCode::SUCCESS;
    };

    // Flags are matched on the lossy text; the original `OsStr` is kept for the
    // input path so a non-UTF-8 filename still reaches the filesystem intact.
    match input.to_string_lossy().as_ref() {
        "-h" | "--help" => {
            print!("{USAGE}");
            ExitCode::SUCCESS
        }
        "-V" | "--version" => {
            println!("yulia {VERSION} (license: {LICENSE})");
            ExitCode::SUCCESS
        }
        flag if !flag.starts_with('-') => {
            if let Some(extra) = args.get(1) {
                eprintln!("yulia: unexpected argument: {}", extra.to_string_lossy());
                return ExitCode::FAILURE;
            }
            compile_file(input)
        }
        flag => {
            eprintln!("yulia: unknown option: {flag}");
            eprintln!("try 'yulia --help'");
            ExitCode::FAILURE
        }
    }
}

/// Compiles the file at `input`, reporting diagnostics to stderr.
///
/// Returns `ExitCode::FAILURE` when the input cannot be read or is rejected.
fn compile_file(input: &OsStr) -> ExitCode {
    let Ok(source) = std::fs::read_to_string(input) else {
        eprintln!("yulia: cannot read '{}'", input.to_string_lossy());
        return ExitCode::FAILURE;
    };
    if source.trim().is_empty() {
        eprintln!("yulia: empty input: '{}'", input.to_string_lossy());
        return ExitCode::FAILURE;
    }
    match yulia::compile::compile(&source) {
        Ok(_) => ExitCode::SUCCESS,
        Err(diagnostics) => {
            let path = input.to_string_lossy();
            for diagnostic in &diagnostics {
                let location = locate(&source, diagnostic.span.start);
                eprintln!(
                    "{}:{}:{}: error[{}]: {}",
                    path, location.line, location.column, diagnostic.code.0, diagnostic.message
                );
            }
            ExitCode::FAILURE
        }
    }
}
