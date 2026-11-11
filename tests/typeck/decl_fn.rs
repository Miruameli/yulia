//! Function declaration typing and Phase 1 subset gates (0.2 §6.2, D1).

use super::{accepts, rejects};

#[test]
fn duplicate_function_is_reported() {
    rejects(
        "module app.main;
fn f() -> void { }
fn f() -> void { }
fn main() -> void { }",
        &["E215"],
    );
}

#[test]
fn function_parameter_types_are_checked() {
    rejects(
        "module app.main;
fn f(a: Nope) -> void { }
fn main() -> void { }",
        &["E216"],
    );
}

#[test]
fn duplicate_parameter_is_reported() {
    rejects(
        "module app.main;
fn f(a: int, a: bool) -> void { }
fn main() -> void { }",
        &["E225"],
    );
}

#[test]
fn void_cannot_be_a_value_type() {
    rejects(
        "module app.main;
fn main() -> void { let x: void; }",
        &["E221"],
    );
}

#[test]
fn a_main_function_accepts() {
    accepts("module app.main;\nfn main() -> void { let x: int = 1; }");
}

#[test]
fn export_is_out_of_subset() {
    rejects(
        "module app.main;
export fn main() -> void { }",
        &["E214"],
    );
}

#[test]
fn extern_is_out_of_subset() {
    rejects(
        r#"module app.main;
extern "C" fn puts(s: CPtr) -> int;
fn main() -> void { }"#,
        &["E214"],
    );
}

#[test]
fn import_is_out_of_subset() {
    rejects(
        "module app.main;
import core.io as io;
fn main() -> void { }",
        &["E214"],
    );
}
