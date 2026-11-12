//! Shared driver-test harness: spawns the `yulia` binary and builds temp files.

use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The `yulia` binary under test.
pub(crate) const YULIA: &str = env!("CARGO_BIN_EXE_yulia");

/// The usage text printed for `--help` and for no arguments.
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

/// Runs the driver with `args`.
///
/// The `expect` lives in each test rather than here: `clippy.toml` sets
/// `allow-expect-in-tests`, which only covers code inside a `#[test]` function.
pub(crate) fn run<S: AsRef<OsStr>>(args: &[S]) -> io::Result<Output> {
    Command::new(YULIA).args(args).output()
}

/// Returns the output's stdout as text.
pub(crate) fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// Returns the output's stderr as text.
pub(crate) fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// A temporary file that removes itself when dropped, so a failing assertion
/// cannot leak files into the temp directory.
pub(crate) struct TempFile(PathBuf);

impl TempFile {
    /// Writes `contents` to a uniquely named file in the temp directory.
    ///
    /// `name` must differ between tests: tests run in parallel threads of one
    /// process, so the process id alone does not disambiguate them.
    pub(crate) fn new(name: &str, contents: &[u8]) -> io::Result<Self> {
        let path =
            std::env::temp_dir().join(format!("yulia-driver-test-{}-{name}", std::process::id()));
        std::fs::write(&path, contents)?;
        Ok(Self(path))
    }

    /// The file's path.
    pub(crate) fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}
