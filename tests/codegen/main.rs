//! End-to-end codegen tests: compile a Yulia program to an executable and run it.
//!
//! Phase 1 has no I/O in the language (imports and strings are outside the D1
//! subset), so a successful program produces no output; these tests assert the
//! exit status and the runtime-error contract (R0101–R0103). The `stdout`
//! helper is kept for the Phase 2 stdlib.

mod harness;

use harness::{assert_aborted, compile, stderr, stdout};

/// A program whose `main` returns normally.
const SUCCESS: &str = r"module app.main;

fn add(a: int, b: int) -> int {
    return a + b;
}

fn main() -> void {
    let x: int = add(2, 3);
}
";

#[test]
fn a_program_that_returns_normally_exits_zero() {
    let (exe, compiled) = compile("success", SUCCESS).expect("spawn yulia");
    assert!(
        compiled.status.success(),
        "compile failed: {}",
        stderr(&compiled)
    );
    let run = exe.run().expect("run the executable");
    assert!(run.status.success(), "run failed: {}", stderr(&run));
    assert_eq!(run.status.code(), Some(0));
    assert_eq!(stdout(&run), "");
}

#[test]
fn array_index_out_of_bounds_aborts_with_r0101() {
    let (exe, compiled) = compile(
        "r0101",
        r"module app.main;

fn main() -> void {
    let a: [int; 3] = [1, 2, 3];
    let i: int = 5;
    let x: int = a[i];
}
",
    )
    .expect("spawn yulia");
    assert!(
        compiled.status.success(),
        "compile failed: {}",
        stderr(&compiled)
    );
    let run = exe.run().expect("run the executable");
    assert_aborted(&run);
    let err = stderr(&run);
    assert!(err.contains("runtime error[R0101]"), "stderr: {err}");
}

#[test]
fn division_by_zero_aborts_with_r0102() {
    let (exe, compiled) = compile(
        "r0102",
        r"module app.main;

fn main() -> void {
    let a: int = 1;
    let b: int = 0;
    let c: int = a / b;
}
",
    )
    .expect("spawn yulia");
    assert!(
        compiled.status.success(),
        "compile failed: {}",
        stderr(&compiled)
    );
    let run = exe.run().expect("run the executable");
    assert_aborted(&run);
    let err = stderr(&run);
    assert!(err.contains("runtime error[R0102]"), "stderr: {err}");
}

#[test]
fn integer_overflow_aborts_with_r0103() {
    let (exe, compiled) = compile(
        "r0103",
        r"module app.main;

fn main() -> void {
    let a: int = 2147483647;
    let b: int = a + 1;
}
",
    )
    .expect("spawn yulia");
    assert!(
        compiled.status.success(),
        "compile failed: {}",
        stderr(&compiled)
    );
    let run = exe.run().expect("run the executable");
    assert_aborted(&run);
    let err = stderr(&run);
    assert!(err.contains("runtime error[R0103]"), "stderr: {err}");
}
