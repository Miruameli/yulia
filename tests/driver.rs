//! Tests for the `yulia` driver.
//!
//! Every branch of `src/main.rs` is exercised: the flag paths, the input-path
//! path, and every failure path. The driver's behaviour is pinned here so a
//! later change cannot silently alter the CLI contract.

use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The `yulia` binary under test.
const YULIA: &str = env!("CARGO_BIN_EXE_yulia");

/// The usage text printed for `--help` and for no arguments.
const USAGE: &str = "\
Usage: yulia [OPTIONS] [INPUT]

Arguments:
  [INPUT]  Yulia source file to compile

Options:
  -h, --help       Print help
  -V, --version    Print version
";

/// Runs the driver with `args`.
///
/// The `expect` lives in each test rather than here: `clippy.toml` sets
/// `allow-expect-in-tests`, which only covers code inside a `#[test]` function.
fn run<S: AsRef<OsStr>>(args: &[S]) -> io::Result<Output> {
    Command::new(YULIA).args(args).output()
}

/// Returns the output's stdout as text.
fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// Returns the output's stderr as text.
fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// A temporary file that removes itself when dropped, so a failing assertion
/// cannot leak files into the temp directory.
struct TempFile(PathBuf);

impl TempFile {
    /// Writes `contents` to a uniquely named file in the temp directory.
    ///
    /// `name` must differ between tests: tests run in parallel threads of one
    /// process, so the process id alone does not disambiguate them.
    fn new(name: &str, contents: &[u8]) -> io::Result<Self> {
        let path =
            std::env::temp_dir().join(format!("yulia-driver-test-{}-{name}", std::process::id()));
        std::fs::write(&path, contents)?;
        Ok(Self(path))
    }

    /// The file's path.
    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

#[test]
fn no_arguments_prints_usage_and_succeeds() {
    let output = run(&[] as &[&OsStr]).expect("spawn yulia");
    assert!(output.status.success());
    assert_eq!(stdout(&output), USAGE);
}

#[test]
fn help_flag_prints_usage_and_succeeds() {
    let output = run(&["--help"]).expect("spawn yulia");
    assert!(output.status.success());
    assert_eq!(stdout(&output), USAGE);
}

#[test]
fn short_help_flag_matches_long() {
    let short = run(&["-h"]).expect("spawn yulia");
    let long = run(&["--help"]).expect("spawn yulia");
    assert_eq!(stdout(&short), stdout(&long));
}

#[test]
fn version_flag_reports_the_crate_version() {
    let output = run(&["--version"]).expect("spawn yulia");
    assert!(output.status.success());
    assert_eq!(
        stdout(&output),
        format!("yulia {} (license: MPL-2.0)\n", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn short_version_flag_matches_long() {
    let short = run(&["-V"]).expect("spawn yulia");
    let long = run(&["--version"]).expect("spawn yulia");
    assert_eq!(stdout(&short), stdout(&long));
}

#[test]
fn unknown_option_fails_with_a_hint() {
    let output = run(&["--definitely-not-a-flag"]).expect("spawn yulia");
    assert!(!output.status.success());
    let err = stderr(&output);
    assert!(err.contains("unknown option"), "stderr: {err}");
    assert!(err.contains("try 'yulia --help'"), "stderr: {err}");
}

#[test]
fn missing_input_file_fails() {
    let output = run(&["no-such-file.yulia"]).expect("spawn yulia");
    assert!(!output.status.success());
    let err = stderr(&output);
    assert!(err.contains("cannot read"), "stderr: {err}");
    assert!(err.contains("no-such-file.yulia"), "stderr: {err}");
}

#[test]
fn empty_input_file_fails() {
    let file = TempFile::new("empty", b"").expect("write temp file");
    let output = run(&[file.path().as_os_str()]).expect("spawn yulia");
    assert!(!output.status.success());
    let err = stderr(&output);
    assert!(err.contains("empty input"), "stderr: {err}");
}

#[test]
fn non_utf8_input_file_fails() {
    let file = TempFile::new("binary", &[0xff, 0xfe, 0x00]).expect("write temp file");
    let output = run(&[file.path().as_os_str()]).expect("spawn yulia");
    assert!(!output.status.success());
    let err = stderr(&output);
    assert!(err.contains("cannot read"), "stderr: {err}");
}

#[test]
fn extra_argument_fails() {
    let file = TempFile::new("extra", b"fn main() {}").expect("write temp file");
    let output = run(&[file.path().as_os_str(), OsStr::new("unexpected")]).expect("spawn yulia");
    assert!(!output.status.success());
    let err = stderr(&output);
    assert!(err.contains("unexpected argument"), "stderr: {err}");
    assert!(err.contains("unexpected"), "stderr: {err}");
}

/// Pins the pre-Phase-1 stub: a readable input currently fails with a
/// "not implemented" diagnostic instead of panicking. Replace this test when
/// the pipeline lands (Phase 1a) — it must then assert real compilation.
#[test]
fn readable_input_fails_gracefully_without_panicking() {
    let file = TempFile::new("valid", b"fn main() {}").expect("write temp file");
    let output = run(&[file.path().as_os_str()]).expect("spawn yulia");
    assert!(!output.status.success());
    let err = stderr(&output);
    assert!(err.contains("not implemented"), "stderr: {err}");
    assert!(!err.contains("panicked"), "stderr: {err}");
}

/// A non-UTF-8 argument must be reported, not panicked on: the crate forbids
/// panicking at the user.
#[cfg(unix)]
#[test]
fn non_utf8_argument_does_not_panic() {
    use std::os::unix::ffi::OsStrExt;

    let output = run(&[OsStr::from_bytes(b"bad-\xff.yulia")]).expect("spawn yulia");
    assert!(!output.status.success());
    let err = stderr(&output);
    assert!(!err.contains("panicked"), "stderr: {err}");
}

/// A non-UTF-8 file name must be reported, not panicked on.
#[cfg(unix)]
#[test]
fn non_utf8_filename_does_not_panic() {
    use std::os::unix::ffi::OsStrExt;

    let output = run(&[OsStr::from_bytes(b"missing-\xff.yulia")]).expect("spawn yulia");
    assert!(!output.status.success());
    let err = stderr(&output);
    assert!(err.contains("cannot read"), "stderr: {err}");
    assert!(!err.contains("panicked"), "stderr: {err}");
}
