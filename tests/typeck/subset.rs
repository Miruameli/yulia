//! Phase 1 subset enforcement (D1).

use super::{accepts, main_program, rejects};

#[test]
fn float_is_out_of_subset() {
    rejects(&main_program("let x: float = 1.0;"), &["E214", "E214"]);
}

#[test]
fn char_is_out_of_subset() {
    rejects(&main_program("let c: char = 'a';"), &["E214", "E214"]);
}

#[test]
fn string_is_out_of_subset() {
    rejects(&main_program("let s: string = \"hi\";"), &["E214", "E214"]);
}

#[test]
fn panic_is_out_of_subset() {
    rejects(&main_program("panic(\"no\");"), &["E214"]);
}

#[test]
fn pin_is_out_of_subset() {
    rejects(&main_program("pin x as y { }"), &["E214"]);
}

#[test]
fn cptr_is_out_of_subset() {
    rejects(
        "module app.main;
fn main() -> void { let p: CPtr; }",
        &["E214"],
    );
}

#[test]
fn the_phase1_subset_accepts_a_full_program() {
    accepts(
        "module app.main;
struct Point { x: int; y: int }
const ORIGIN_X: int = 0;
fn add(a: int, b: int) -> int { return a + b; }
fn main() -> void {
    let p: Point = Point { x: ORIGIN_X, y: 2 };
    let xs: [int; 3] = [0; 3];
    let total: int = add(p.x, p.y);
    let i: int = 0;
    while (i < 3) {
        xs[i] = total;
        i = i + 1;
    }
    for j in 0..3 {
        if (j == 1) { continue; } else { break; }
    }
    for k: int = 0; k < 3; k = k + 1 {
        total = total + k;
    }
    let flag: bool = total > 0 && xs[0] == 0;
    if (flag) { return; }
}",
    );
}
