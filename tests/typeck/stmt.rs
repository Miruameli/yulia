//! Statement typing (0.2 §6.2).

use super::{accepts, main_program, rejects};

#[test]
fn let_with_a_matching_type_accepts() {
    accepts(&main_program("let x: int = 1;"));
}

#[test]
fn let_with_a_mismatched_type_is_reported() {
    rejects(&main_program("let x: int = true;"), &["E202"]);
}

#[test]
fn read_before_assignment_is_reported() {
    rejects(&main_program("let x: int; let y: int = x;"), &["E222"]);
}

#[test]
fn assignment_marks_the_variable_initialized() {
    accepts(&main_program("let x: int; x = 1; let y: int = x;"));
}

#[test]
fn if_condition_must_be_bool() {
    rejects(&main_program("if (1) { }"), &["E210"]);
}

#[test]
fn while_condition_must_be_bool() {
    rejects(&main_program("while (1) { }"), &["E210"]);
}

#[test]
fn for_range_bounds_must_be_int() {
    rejects(&main_program("for i in true..3 { }"), &["E202"]);
    rejects(&main_program("for i in 0..true { }"), &["E202"]);
}

#[test]
fn for_c_style_condition_must_be_bool() {
    rejects(&main_program("for i: int = 0; 1; i = i + 1 { }"), &["E210"]);
}

#[test]
fn for_c_style_update_must_match_the_variable() {
    rejects(
        &main_program("for i: int = 0; i < 3; i = true { }"),
        &["E202"],
    );
}

#[test]
fn return_type_must_match() {
    rejects(
        "module app.main;
fn f() -> int { return true; }
fn main() -> void { }",
        &["E211"],
    );
}

#[test]
fn bare_return_requires_void() {
    rejects(
        "module app.main;
fn f() -> int { return; }
fn main() -> void { }",
        &["E211"],
    );
}

#[test]
fn value_return_in_void_is_reported() {
    rejects(
        "module app.main;
fn f() -> void { return 1; }
fn main() -> void { }",
        &["E211"],
    );
}

#[test]
fn break_outside_a_loop_is_reported() {
    rejects(&main_program("break;"), &["E212"]);
}

#[test]
fn continue_outside_a_loop_is_reported() {
    rejects(&main_program("continue;"), &["E213"]);
}

#[test]
fn break_and_continue_inside_a_loop_accept() {
    accepts(&main_program(
        "while (true) { break; } for i in 0..1 { continue; }",
    ));
}

#[test]
fn both_if_branches_initializing_accepts() {
    accepts(&main_program(
        "let x: int; if (true) { x = 1; } else { x = 2; } let y: int = x;",
    ));
}

#[test]
fn if_without_else_leaves_the_variable_uninitialized() {
    rejects(
        &main_program("let x: int; if (true) { x = 1; } let y: int = x;"),
        &["E222"],
    );
}

#[test]
fn an_inner_scope_may_shadow_an_outer_binding() {
    accepts(&main_program(
        "let x: int = 1; if (true) { let x: bool = true; } let y: int = x;",
    ));
}

#[test]
fn a_name_from_an_closed_scope_is_undefined() {
    rejects(
        &main_program("if (true) { let x: int = 1; } let y: int = x;"),
        &["E201"],
    );
}
