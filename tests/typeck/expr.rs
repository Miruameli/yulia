//! Operator typing (0.2 §6.2).

use super::{accepts, main_program, rejects};

#[test]
fn int_arithmetic_accepts() {
    accepts(&main_program("let x: int = 1 + 2 * 3 - 4 / 2 % 3;"));
}

#[test]
fn bool_operators_require_bool() {
    rejects(&main_program("let b: bool = 1 && 2;"), &["E202", "E202"]);
}

#[test]
fn equality_is_int_only() {
    rejects(
        &main_program("let b: bool = true == false;"),
        &["E202", "E202"],
    );
}

#[test]
fn unary_minus_requires_int() {
    rejects(&main_program("let x: int = -true;"), &["E202"]);
}

#[test]
fn not_requires_bool() {
    rejects(&main_program("let b: bool = !1;"), &["E202"]);
}
