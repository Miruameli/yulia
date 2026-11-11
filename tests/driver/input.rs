//! Input-path tests for the `yulia` driver: every failure path that reads a file.

use std::ffi::OsStr;

use crate::harness::{TempFile, run, stderr};

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
