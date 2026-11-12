//! End-to-end tests over the illustrative programs of 0.1 §6.
//!
//! The Phase 1 subset is `int`, `bool`, structs, fixed-size arrays, functions,
//! and control flow (D1). Every illustrative program inside that subset must
//! compile; every one outside it must be rejected with the subset diagnostic.

use yulia::compile::compile;

/// Asserts that `source` compiles cleanly.
///
/// Test helper: a program that must compile but does not means the test failed.
#[allow(clippy::panic)]
fn assert_compiles(source: &str) {
    match compile(source) {
        Ok(_) => {}
        Err(diagnostics) => panic!(
            "expected the program to compile, got: {:?}",
            diagnostics
                .iter()
                .map(|d| format!("{}: {}", d.code.0, d.message))
                .collect::<Vec<_>>()
        ),
    }
}

/// Asserts that `source` is rejected and returns the first diagnostic.
///
/// Test helper: a program that must be rejected but compiles means the test failed.
#[allow(clippy::panic)]
fn assert_rejected(source: &str) -> String {
    match compile(source) {
        Ok(_) => panic!("expected the program to be rejected, but it compiled"),
        Err(diagnostics) => {
            assert!(
                !diagnostics.is_empty(),
                "a rejection must carry a diagnostic"
            );
            format!("{}: {}", diagnostics[0].code.0, diagnostics[0].message)
        }
    }
}

// --- Programs inside the Phase 1 subset -------------------------------------

#[test]
fn program_2_arithmetic_compiles() {
    assert_compiles(
        r"module main;

fn add(a: int, b: int) -> int {
    return a + b;
}

fn main() {
    let x: int = 10;
    let y: int = 20;
    let z: int = add(x, y);
    // z == 30
}",
    );
}

#[test]
fn program_3_functions_with_multiple_parameters_compiles() {
    assert_compiles(
        r"module main;

fn clamp(val: int, lo: int, hi: int) -> int {
    if val < lo {
        return lo;
    } else if val > hi {
        return hi;
    } else {
        return val;
    }
}

fn main() {
    let r: int = clamp(15, 0, 10);  // r == 10
}",
    );
}

#[test]
fn program_4_struct_definition_and_usage_compiles() {
    assert_compiles(
        r"module main;

struct Point {
    x: int;
    y: int;
}

fn distance_squared(p: Point) -> int {
    return p.x * p.x + p.y * p.y;
}

fn main() {
    let origin: Point = Point { x: 0, y: 0 };
    let p: Point = Point { x: 3, y: 4 };
    let d: int = distance_squared(p);  // d == 25
}",
    );
}

#[test]
fn program_5_arrays_compiles() {
    assert_compiles(
        r"module main;

fn sum(arr: [int; 4]) -> int {
    let total: int = 0;
    for i in 0..4 {
        total = total + arr[i];
    }
    return total;
}

fn main() {
    let data: [int; 4] = [1, 2, 3, 4];
    let s: int = sum(data);  // s == 10
}",
    );
}

#[test]
fn program_6_if_else_compiles() {
    assert_compiles(
        r"module main;

fn abs(x: int) -> int {
    if x < 0 {
        return -x;
    } else {
        return x;
    }
}

fn main() {
    let a: int = abs(-5);   // a == 5
    let b: int = abs(3);    // b == 3
}",
    );
}

#[test]
fn program_7_while_loop_compiles() {
    assert_compiles(
        r"module main;

fn factorial(n: int) -> int {
    let result: int = 1;
    let i: int = 2;
    while i <= n {
        result = result * i;
        i = i + 1;
    }
    return result;
}

fn main() {
    let f: int = factorial(5);  // f == 120
}",
    );
}

#[test]
fn program_8_for_loop_range_based_compiles() {
    assert_compiles(
        r"module main;

fn main() {
    let sum: int = 0;
    for i in 1..11 {
        sum = sum + i;
    }
    // sum == 55
}",
    );
}

#[test]
fn program_9_for_loop_c_style_compiles() {
    assert_compiles(
        r"module main;

fn main() {
    let sum: int = 0;
    for i: int = 0; i < 10; i = i + 1 {
        sum = sum + i;
    }
    // sum == 45
}",
    );
}

#[test]
fn program_11_recursion_compiles() {
    assert_compiles(
        r"module main;

fn fib(n: int) -> int {
    if n <= 1 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

fn main() {
    let f: int = fib(10);  // f == 55
}",
    );
}

#[test]
fn program_14_boolean_logic_compiles() {
    assert_compiles(
        r"module main;

fn is_even(n: int) -> bool {
    return n % 2 == 0;
}

fn main() {
    let a: bool = is_even(4);   // true
    let b: bool = is_even(7);   // false
    let c: bool = a && !b;      // true
    let d: bool = a || b;       // true
}",
    );
}

#[test]
fn program_15_nested_control_flow_compiles() {
    assert_compiles(
        r"module main;

fn classify(n: int) -> int {
    if n < 0 {
        if n < -10 {
            return -2;
        } else {
            return -1;
        }
    } else if n == 0 {
        return 0;
    } else {
        if n > 10 {
            return 2;
        } else {
            return 1;
        }
    }
}

fn main() {
    let r: int = classify(15);  // r == 2
}",
    );
}

#[test]
fn program_16_array_indexing_and_mutation_compiles() {
    assert_compiles(
        r"module main;

fn main() {
    let arr: [int; 3] = [10, 20, 30];
    arr[1] = 99;
    // arr == [10, 99, 30]
    let x: int = arr[0] + arr[2];  // x == 40
}",
    );
}

#[test]
fn program_17_private_helper_compiles() {
    assert_compiles(
        r"module math;

private fn square(x: int) -> int {
    return x * x;
}

fn hypotenuse(a: int, b: int) -> int {
    return square(a) + square(b);
}",
    );
}

#[test]
fn program_20_empty_and_minimal_programs_compile() {
    assert_compiles("module empty;");
    assert_compiles("module main;\n\nfn main() {\n}");
}

#[test]
fn program_21_complex_expressions_compiles() {
    assert_compiles(
        r"module main;

fn main() {
    let x: int = 2 + 3 * 4;        // x == 14
    let y: int = (2 + 3) * 4;      // y == 20
    let z: bool = 1 < 2 && 3 > 2;  // z == true
    let w: int = -x + y;           // w == 6
}",
    );
}

#[test]
fn program_22_struct_with_array_field_compiles() {
    assert_compiles(
        r"module main;

struct Buffer {
    data: [int; 256];
    len: int;
}

fn main() {
    let buf: Buffer = Buffer {
        data: [0; 256],
        len: 0,
    };
}",
    );
}

#[test]
fn program_23_break_and_continue_compiles() {
    assert_compiles(
        r"module main;

fn find_first(arr: [int; 5], target: int) -> int {
    for i in 0..5 {
        if arr[i] == target {
            return i;
        }
    }
    return -1;
}

fn sum_positives(arr: [int; 5]) -> int {
    let total: int = 0;
    for i in 0..5 {
        if arr[i] <= 0 {
            continue;
        }
        total = total + arr[i];
    }
    return total;
}",
    );
}

// --- Programs outside the Phase 1 subset ------------------------------------

#[test]
fn program_1_is_rejected_for_its_import_and_string() {
    let diagnostic = assert_rejected(
        r#"module main;

import std.io;

fn main() {
    io.println_str("Hello, world!");
}"#,
    );
    assert!(
        diagnostic.contains("import") || diagnostic.contains("string"),
        "the rejection must name the out-of-subset construct: {diagnostic}"
    );
}

#[test]
fn program_10_is_rejected_for_float() {
    let diagnostic = assert_rejected(
        r"module main;

struct Vec2 {
    x: float;
    y: float;
}

fn main() {
    let v: Vec2 = Vec2 { x: 0.0, y: 0.0 };
}",
    );
    assert!(
        diagnostic.contains("float"),
        "the rejection must name float: {diagnostic}"
    );
}

#[test]
fn program_12_is_rejected_for_float() {
    let diagnostic = assert_rejected(
        r"module main;

const PI: float = 3.14159265;

fn main() {
    let a: float = PI;
}",
    );
    assert!(
        diagnostic.contains("float"),
        "the rejection must name float: {diagnostic}"
    );
}

#[test]
fn program_13_is_rejected_for_its_import() {
    let diagnostic = assert_rejected(
        r#"module main;

import std.string as str;

fn main() {
    let s: string = "Hello";
}"#,
    );
    assert!(
        diagnostic.contains("import"),
        "the rejection must name the import: {diagnostic}"
    );
}

#[test]
fn program_18_is_rejected_for_its_import() {
    let diagnostic = assert_rejected(
        r"module main;

import math;

fn main() {
    let h: int = math.hypotenuse(3, 4);
}",
    );
    assert!(
        diagnostic.contains("import"),
        "the rejection must name the import: {diagnostic}"
    );
}

#[test]
fn program_19_is_rejected_for_char_and_string() {
    let diagnostic = assert_rejected(
        r"module main;

fn main() {
    let c: char = 'A';
}",
    );
    assert!(
        diagnostic.contains("char"),
        "the rejection must name char: {diagnostic}"
    );
}

#[test]
fn program_24_is_rejected_for_float() {
    let diagnostic = assert_rejected(
        r"module main;

fn main() {
    let hex: int = 0xFF;
    let flt: float = 1.5e3;
}",
    );
    assert!(
        diagnostic.contains("float"),
        "the rejection must name float: {diagnostic}"
    );
}
