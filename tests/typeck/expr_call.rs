//! Call and assignment typing (0.2 §6.2).

use super::{main_program, rejects};

#[test]
fn call_argument_count_is_reported() {
    rejects(
        "module app.main;
fn add(a: int, b: int) -> int { return a + b; }
fn main() -> void {
    let x: int = add(1);
}",
        &["E204"],
    );
}

#[test]
fn call_argument_type_is_reported() {
    rejects(
        "module app.main;
fn add(a: int, b: int) -> int { return a + b; }
fn main() -> void {
    let x: int = add(1, true);
}",
        &["E202"],
    );
}

#[test]
fn call_to_an_undefined_function_is_reported() {
    rejects(&main_program("let x: int = nope();"), &["E201"]);
}

#[test]
fn assignment_type_mismatch_is_reported() {
    rejects(&main_program("let x: int = 1; x = true;"), &["E202"]);
}

#[test]
fn assignment_to_a_non_lvalue_is_reported() {
    rejects(&main_program("1 = 2;"), &["E209"]);
}

#[test]
fn undefined_name_is_reported() {
    rejects(&main_program("let x: int = y;"), &["E201"]);
}
