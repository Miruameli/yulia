//! `yulia` — the Yulia compiler driver.
//!
//! Phase 1 driver: it parses command-line arguments and reports the compiler
//! version. Compilation proper is wired in as the pipeline lands.

use std::ffi::{OsStr, OsString};
use std::process::ExitCode;

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
            compile(input)
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
fn compile(input: &OsStr) -> ExitCode {
    let Ok(source) = std::fs::read_to_string(input) else {
        eprintln!("yulia: cannot read '{}'", input.to_string_lossy());
        return ExitCode::FAILURE;
    };
    if source.trim().is_empty() {
        eprintln!("yulia: empty input: '{}'", input.to_string_lossy());
        return ExitCode::FAILURE;
    }
    // The pipeline is not wired yet; Phase 1 lands it module by module.
    eprintln!("yulia: compilation is not implemented yet");
    ExitCode::FAILURE
}
