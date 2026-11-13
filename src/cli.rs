//! The `yulia` command-line surface: usage text and argument parsing.
//!
//! Parsing is hand-written rather than pulled from an argument crate: the
//! driver has four flags, and Phase 1a is std-only on purpose (`Cargo.toml`).

use std::ffi::OsString;

/// The usage text printed by `--help` and by a bare invocation.
pub(crate) const USAGE: &str = "\
Usage: yulia [OPTIONS] [INPUT]

Arguments:
  [INPUT]  Yulia source file to compile

Options:
  -h, --help           Print help
  -V, --version        Print version
  -o, --output <FILE>  Write the compiled artifact to FILE
      --emit <KIND>    Artifact kind: exe (default), obj, or ir
";

/// The artifact kind selected by `--emit`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum EmitKind {
    /// Write an object file.
    Obj,
    /// Write LLVM IR.
    Ir,
    /// Link an executable.
    #[default]
    Exe,
}

/// The parsed command line.
#[derive(Debug)]
pub(crate) struct Options {
    /// The input source file.
    pub(crate) input: Option<OsString>,
    /// The artifact to write; `None` keeps compile-check-only behaviour.
    pub(crate) output: Option<OsString>,
    /// The artifact kind.
    pub(crate) emit: EmitKind,
}

/// The outcome of parsing the command line.
#[derive(Debug)]
pub(crate) enum Parsed {
    /// `--help` was given.
    Help,
    /// `--version` was given.
    Version,
    /// The driver should run.
    Run(Options),
}

/// Parses the command line, reporting the first problem as a message.
///
/// `args` excludes the program name. Arguments are read as `OsString`, so a
/// non-UTF-8 path still reaches the filesystem instead of panicking; only the
/// `--emit` value is interpreted as text.
pub(crate) fn parse_args(args: &[OsString]) -> Result<Parsed, String> {
    let mut options = Options {
        input: None,
        output: None,
        emit: EmitKind::default(),
    };
    let mut index = 0;
    while index < args.len() {
        let arg = args[index].to_string_lossy();
        match arg.as_ref() {
            "-h" | "--help" => return Ok(Parsed::Help),
            "-V" | "--version" => return Ok(Parsed::Version),
            "-o" | "--output" => {
                index += 1;
                let value = args
                    .get(index)
                    .ok_or_else(|| format!("missing value for {arg}"))?;
                options.output = Some(value.clone());
            }
            "--emit" => {
                index += 1;
                let value = args
                    .get(index)
                    .ok_or_else(|| "missing value for --emit".to_string())?;
                options.emit = parse_emit(&value.to_string_lossy())?;
            }
            _ if arg.starts_with('-') => return Err(format!("unknown option: {arg}")),
            _ => {
                if options.input.is_some() {
                    return Err(format!("unexpected argument: {arg}"));
                }
                options.input = Some(args[index].clone());
            }
        }
        index += 1;
    }
    Ok(Parsed::Run(options))
}

/// Parses the `--emit` value.
fn parse_emit(value: &str) -> Result<EmitKind, String> {
    match value {
        "exe" => Ok(EmitKind::Exe),
        "obj" => Ok(EmitKind::Obj),
        "ir" => Ok(EmitKind::Ir),
        other => Err(format!(
            "unknown emit kind: {other} (expected exe, obj, or ir)"
        )),
    }
}
