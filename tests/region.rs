//! Region inference tests (0.7 §3).
//!
//! Phase 1 is by value with no references, so the whole model reduces to one
//! rule: a binding lives in its enclosing function's region. These tests pin
//! that rule at every binding form the grammar has.

use yulia::lexer::Lexer;
use yulia::parser::Parser;
use yulia::region::Assignment;

/// Parses `source` and assigns its regions.
fn assign(source: &str) -> Assignment {
    let (tokens, lex) = Lexer::new(source).tokenize();
    assert!(lex.is_empty(), "unexpected lexer diagnostics: {lex:?}");
    let (program, parse) = Parser::new(&tokens).parse();
    assert!(parse.is_empty(), "unexpected parser diagnostics: {parse:?}");
    Assignment::new(&program)
}

#[test]
fn each_function_gets_its_own_region() {
    let assignment = assign(
        "module app.main;
fn f() -> void { }
fn g() -> void { }
fn main() -> void { }",
    );
    assert_eq!(assignment.region_count(), 3);
    let f = assignment.function_region("f").expect("f has a region");
    let g = assignment.function_region("g").expect("g has a region");
    let main = assignment
        .function_region("main")
        .expect("main has a region");
    assert_ne!(f, g);
    assert_ne!(f, main);
    assert_ne!(g, main);
}

#[test]
fn a_local_lives_in_its_function_region() {
    let assignment = assign("module app.main;\nfn main() -> void { let x: int = 1; }");
    let main = assignment
        .function_region("main")
        .expect("main has a region");
    assert_eq!(
        assignment.binding_region("main", "x"),
        Some(main),
        "`x` must live in main's region"
    );
}

#[test]
fn bindings_in_nested_blocks_stay_in_the_function_region() {
    let assignment = assign(
        "module app.main;
fn main() -> void {
    let outer: int = 0;
    if (outer == 0) { let then_branch: int = 1; } else { let else_branch: int = 2; }
    while (outer < 3) { let in_loop: int = 3; }
}",
    );
    let main = assignment
        .function_region("main")
        .expect("main has a region");
    for name in ["outer", "then_branch", "else_branch", "in_loop"] {
        assert_eq!(
            assignment.binding_region("main", name),
            Some(main),
            "{name}"
        );
    }
}

#[test]
fn loop_variables_live_in_the_function_region() {
    let assignment = assign(
        "module app.main;
fn main() -> void {
    for i in 0..3 { let in_range: int = i; }
    for j: int = 0; j < 3; j = j + 1 { let in_c_style: int = j; }
}",
    );
    let main = assignment
        .function_region("main")
        .expect("main has a region");
    for name in ["i", "j", "in_range", "in_c_style"] {
        assert_eq!(
            assignment.binding_region("main", name),
            Some(main),
            "{name}"
        );
    }
}

#[test]
fn parameters_live_in_the_function_region() {
    let assignment = assign(
        "module app.main;
fn add(a: int, b: int) -> int { return a + b; }
fn main() -> void { }",
    );
    let add = assignment.function_region("add").expect("add has a region");
    assert_eq!(assignment.binding_region("add", "a"), Some(add));
    assert_eq!(assignment.binding_region("add", "b"), Some(add));
}

#[test]
fn the_same_name_in_two_functions_gets_two_regions() {
    let assignment = assign(
        "module app.main;
fn f() -> void { let x: int = 1; }
fn g() -> void { let x: int = 2; }
fn main() -> void { }",
    );
    let f = assignment.function_region("f").expect("f has a region");
    let g = assignment.function_region("g").expect("g has a region");
    assert_eq!(assignment.binding_region("f", "x"), Some(f));
    assert_eq!(assignment.binding_region("g", "x"), Some(g));
    assert_ne!(f, g, "each function call gets a fresh region");
}

#[test]
fn a_call_runs_the_callee_in_its_own_region() {
    let assignment = assign(
        "module app.main;
fn g() -> void { let y: int = 1; }
fn main() -> void { g(); }",
    );
    let g = assignment.function_region("g").expect("g has a region");
    let main = assignment
        .function_region("main")
        .expect("main has a region");
    assert_eq!(assignment.binding_region("g", "y"), Some(g));
    assert_ne!(
        g, main,
        "the callee's locals are not in the caller's region"
    );
    assert_eq!(
        assignment.binding_region("main", "y"),
        None,
        "the callee's locals are not visible to the caller"
    );
}

#[test]
fn a_module_constant_belongs_to_no_function_region() {
    let assignment = assign(
        "module app.main;
const LIMIT: int = 3;
fn main() -> void { let x: int = LIMIT; }",
    );
    assert_eq!(assignment.region_count(), 1);
    assert_eq!(
        assignment.binding_region("main", "LIMIT"),
        None,
        "a module constant is not a value in any function's region"
    );
}
