//! Struct and constant declaration typing (0.2 §6.2).

use super::{accepts, rejects};

#[test]
fn a_struct_field_with_an_unknown_type_is_reported() {
    rejects(
        "module app.main;
struct P { x: Nope }
fn main() -> void { }",
        &["E216"],
    );
}

#[test]
fn duplicate_struct_is_reported() {
    rejects(
        "module app.main;
struct P { x: int }
struct P { y: int }
fn main() -> void { }",
        &["E215"],
    );
}

#[test]
fn duplicate_field_is_reported() {
    rejects(
        "module app.main;
struct P { x: int; x: bool }
fn main() -> void { }",
        &["E215"],
    );
}

#[test]
fn duplicate_constant_is_reported() {
    rejects(
        "module app.main;
const K: int = 1;
const K: int = 2;
fn main() -> void { }",
        &["E215"],
    );
}

#[test]
fn a_struct_may_reference_a_struct_declared_later() {
    accepts(
        "module app.main;
struct A { b: B }
struct B { x: int }
fn main() -> void {
    let b: B = B { x: 1 };
    let a: A = A { b: b };
}",
    );
}

#[test]
fn unknown_type_is_reported() {
    rejects(
        "module app.main;
fn main() -> void { let x: Nope = 1; }",
        &["E216"],
    );
}

#[test]
fn constant_initializer_must_match() {
    rejects(
        "module app.main;
const K: int = true;
fn main() -> void { }",
        &["E202"],
    );
}
