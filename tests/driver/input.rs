//! Input-path tests for the `yulia` driver: every path that reads a file.

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

#[test]
fn a_valid_program_compiles() {
    let file = TempFile::new(
        "valid",
        b"module app.main;\nfn main() -> void { let x: int = 1; }",
    )
    .expect("write temp file");
    let output = run(&[file.path().as_os_str()]).expect("spawn yulia");
    assert!(
        output.status.success(),
        "a valid program must compile; stderr: {}",
        stderr(&output)
    );
    assert_eq!(stderr(&output), "", "success is silent");
}

#[test]
fn a_type_error_is_reported_with_its_location() {
    let file = TempFile::new(
        "type-error",
        b"module app.main;\nfn main() -> void { let x: int = true; }",
    )
    .expect("write temp file");
    let output = run(&[file.path().as_os_str()]).expect("spawn yulia");
    assert!(!output.status.success());
    let err = stderr(&output);
    // `true` starts at line 2, column 34 of the file.
    let name = file.path().to_string_lossy();
    assert!(
        err.contains(&format!("{name}:2:34: error[E202]")),
        "stderr: {err}"
    );
}

#[test]
fn a_lexer_error_is_reported() {
    let file = TempFile::new("lex-error", b"module app.main;\nfn main() -> void { @ }")
        .expect("write temp file");
    let output = run(&[file.path().as_os_str()]).expect("spawn yulia");
    assert!(!output.status.success());
    let err = stderr(&output);
    assert!(err.contains("error[E001]"), "stderr: {err}");
    assert!(err.contains(":2:"), "stderr: {err}");
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

/// A rejected program must not leave an artifact behind, even when `-o` was
/// given: the driver stops at the first failing stage.
#[test]
fn a_type_error_writes_no_artifact() {
    let file = TempFile::new(
        "type-error-artifact",
        b"module app.main;\nfn main() -> void { let x: int = true; }",
    )
    .expect("write temp file");
    let artifact = std::env::temp_dir().join(format!(
        "yulia-driver-test-{}-no-artifact",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&artifact);
    let output = run(&[
        file.path().as_os_str(),
        OsStr::new("-o"),
        artifact.as_os_str(),
    ])
    .expect("spawn yulia");
    assert!(!output.status.success());
    assert!(
        !artifact.exists(),
        "a rejected program must not write an artifact"
    );
    let _ = std::fs::remove_file(&artifact);
}
