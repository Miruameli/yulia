//! Smoke tests for the `yulia` driver.

use std::process::Command;

/// The `yulia` binary under test.
const YULIA: &str = env!("CARGO_BIN_EXE_yulia");

#[test]
fn help_prints_usage_and_succeeds() {
    let output = Command::new(YULIA)
        .arg("--help")
        .output()
        .expect("spawn yulia --help");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf-8 stdout");
    assert!(stdout.contains("Usage: yulia"));
}

#[test]
fn version_reports_the_crate_version() {
    let output = Command::new(YULIA)
        .arg("--version")
        .output()
        .expect("spawn yulia --version");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf-8 stdout");
    assert!(stdout.contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn unknown_option_fails() {
    let output = Command::new(YULIA)
        .arg("--definitely-not-a-flag")
        .output()
        .expect("spawn yulia");
    assert!(!output.status.success());
}

#[test]
fn missing_input_file_fails() {
    let output = Command::new(YULIA)
        .arg("no-such-file.yulia")
        .output()
        .expect("spawn yulia");
    assert!(!output.status.success());
}

/// A non-UTF-8 argument must be reported, not panicked on: the crate forbids
/// panicking at the user.
#[cfg(unix)]
#[test]
fn non_utf8_argument_does_not_panic() {
    use std::os::unix::ffi::OsStrExt;

    let output = Command::new(YULIA)
        .arg(std::ffi::OsStr::from_bytes(b"bad-\xff.yulia"))
        .output()
        .expect("spawn yulia");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.contains("panicked"), "stderr: {stderr}");
}
