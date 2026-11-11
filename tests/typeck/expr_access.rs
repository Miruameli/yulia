//! Field, index, struct literal, and array typing (0.2 §6.2).

use super::{accepts, main_program, rejects};

#[test]
fn field_access_yields_the_field_type() {
    accepts(
        "module app.main;
struct Point { x: int; y: int }
fn main() -> void {
    let p: Point = Point { x: 1, y: 2 };
    let x: int = p.x;
}",
    );
}

#[test]
fn unknown_field_is_reported() {
    rejects(
        "module app.main;
struct Point { x: int; y: int }
fn main() -> void {
    let p: Point = Point { x: 1, y: 2 };
    let z: int = p.z;
}",
        &["E206"],
    );
}

#[test]
fn field_on_a_non_struct_is_reported() {
    rejects(
        &main_program("let x: int = 1; let y: int = x.f;"),
        &["E220"],
    );
}

#[test]
fn index_yields_the_element_type() {
    accepts(&main_program(
        "let a: [int; 3] = [1, 2, 3]; let x: int = a[0];",
    ));
}

#[test]
fn index_requires_int() {
    rejects(
        &main_program("let a: [int; 3] = [1, 2, 3]; let x: int = a[true];"),
        &["E218"],
    );
}

#[test]
fn index_on_a_non_array_is_reported() {
    rejects(
        &main_program("let x: int = 1; let y: int = x[0];"),
        &["E219"],
    );
}

#[test]
fn struct_literal_field_type_mismatch() {
    rejects(
        "module app.main;
struct Point { x: int; y: int }
fn main() -> void {
    let p: Point = Point { x: true, y: 2 };
}",
        &["E202"],
    );
}

#[test]
fn struct_literal_missing_field_is_reported() {
    rejects(
        "module app.main;
struct Point { x: int; y: int }
fn main() -> void {
    let p: Point = Point { x: 1 };
}",
        &["E207"],
    );
}

#[test]
fn struct_literal_unknown_field_is_reported() {
    rejects(
        "module app.main;
struct Point { x: int; y: int }
fn main() -> void {
    let p: Point = Point { x: 1, y: 2, z: 3 };
}",
        &["E206"],
    );
}

#[test]
fn struct_literal_duplicate_field_is_reported() {
    rejects(
        "module app.main;
struct Point { x: int; y: int }
fn main() -> void {
    let p: Point = Point { x: 1, x: 2, y: 3 };
}",
        &["E208"],
    );
}

#[test]
fn array_literal_unifies_element_types() {
    accepts(&main_program("let a: [int; 3] = [1, 2, 3];"));
}

#[test]
fn array_literal_mixed_types_is_reported() {
    rejects(&main_program("let a: [int; 2] = [1, true];"), &["E217"]);
}

#[test]
fn array_repeat_accepts() {
    accepts(&main_program("let a: [int; 4] = [0; 4];"));
}
