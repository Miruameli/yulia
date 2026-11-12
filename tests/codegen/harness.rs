//! Shared codegen-test harness: compiles a Yulia program to an executable.

use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The `yulia` binary under test.
pub(crate) const YULIA: &str = env!("CARGO_BIN_EXE_yulia");

/// Returns the output's stdout as text.
pub(crate) fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// Returns the output's stderr as text.
pub(crate) fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// Compiles `source` to an executable.
///
/// Returns the executable and the compiler's output. The executable removes
/// itself when the returned guard drops.
pub(crate) fn compile(name: &str, source: &str) -> io::Result<(TempExe, Output)> {
    let stem = format!("yulia-codegen-test-{}-{name}", std::process::id());
    let input = std::env::temp_dir().join(format!("{stem}.yl"));
    let output = std::env::temp_dir().join(&stem);
    std::fs::write(&input, source.as_bytes())?;
    let result = Command::new(YULIA)
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .output();
    let _ = std::fs::remove_file(&input);
    Ok((TempExe(output), result?))
}

/// A compiled executable that removes itself when dropped.
pub(crate) struct TempExe(pub(crate) PathBuf);

impl TempExe {
    /// Runs the executable.
    pub(crate) fn run(&self) -> io::Result<Output> {
        Command::new(&self.0).output()
    }

    /// The executable's path.
    pub(crate) fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempExe {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

/// Asserts the program aborted with SIGABRT (the runtime-error contract:
/// `abort()` after the diagnostic, so the shell reports 134).
pub(crate) fn assert_aborted(output: &Output) {
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        assert_eq!(
            output.status.signal(),
            Some(6),
            "expected SIGABRT; stderr: {}",
            stderr(output)
        );
    }
    #[cfg(not(unix))]
    {
        assert_eq!(output.status.code(), Some(134), "expected exit 134");
    }
}
