//! End-to-end lowering tests: source text in, LLVM IR out.
//!
//! These drive the public [`crate::codegen::generate`] surface through the real
//! front end (lexer, parser, type checker, region inference) rather than a
//! hand-built AST, so they also cover the [`crate::typeck::TypeMap`] hand-off.

use inkwell::context::Context;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::region::Assignment;
use crate::typeck::Checker;

/// Runs the front end over `source` and returns the generated IR.
fn lower(source: &str) -> String {
    let (tokens, lexed) = Lexer::new(source).tokenize();
    assert!(lexed.is_empty(), "lexing failed: {lexed:?}");
    let (program, parsed) = Parser::new(&tokens).parse();
    assert!(parsed.is_empty(), "parsing failed: {parsed:?}");
    let mut checker = Checker::new(&program);
    let diagnostics = checker.check();
    assert!(
        diagnostics.is_empty(),
        "type checking failed: {diagnostics:?}"
    );
    let types = checker.types().clone();
    let regions = Assignment::new(&program);
    let context = Context::create();
    let module =
        super::generate(&context, &program, &types, &regions).expect("code generation failed");
    module.print_to_string().to_string()
}

#[test]
fn main_is_the_i32_entry_point() {
    let ir = lower("module app.main;\nfn main() -> int { return 0; }");
    assert!(ir.contains("define i32 @main()"), "{ir}");
}

#[test]
fn struct_literals_lower_to_a_named_struct() {
    let ir = lower(
        "module app.main;\n\
         struct Point { x: int; y: int }\n\
         fn main() -> int {\n\
         let p: Point = Point { x: 1, y: 2 };\n\
         return p.x;\n\
         }",
    );
    assert!(ir.contains("%Point = type { i32, i32 }"), "{ir}");
}

#[test]
fn array_index_reports_r0101() {
    let ir = lower(
        "module app.main;\n\
         fn main() -> int {\n\
         let a: [int; 2] = [1, 2];\n\
         let i: int = 0;\n\
         let x: int = a[i];\n\
         return x;\n\
         }",
    );
    assert!(
        ir.contains("runtime error[R0101]: array index out of bounds"),
        "{ir}"
    );
    assert!(ir.contains("declare void @abort()"), "{ir}");
}

#[test]
fn division_reports_r0102() {
    let ir = lower(
        "module app.main;\n\
         fn main() -> int {\n\
         let a: int = 1;\n\
         let b: int = 0;\n\
         let x: int = a / b;\n\
         return x;\n\
         }",
    );
    assert!(
        ir.contains("runtime error[R0102]: division by zero"),
        "{ir}"
    );
}

#[test]
fn addition_reports_r0103() {
    let ir = lower(
        "module app.main;\n\
         fn main() -> int {\n\
         let a: int = 1;\n\
         let b: int = 2;\n\
         let x: int = a + b;\n\
         return x;\n\
         }",
    );
    assert!(
        ir.contains("runtime error[R0103]: integer overflow"),
        "{ir}"
    );
}

#[test]
fn int_min_division_reports_r0103() {
    // `sdiv i32 -2147483648, -1` is poison in LLVM, so the divisor guard must
    // catch it before the division instead of letting the optimizer fold it.
    let ir = lower(
        "module app.main;\n\
         fn main() -> int {\n\
         let a: int = -2147483647 - 1;\n\
         let b: int = a / -1;\n\
         return b;\n\
         }",
    );
    assert!(ir.contains("div.overflow"), "{ir}");
    assert!(
        ir.contains("runtime error[R0103]: integer overflow"),
        "{ir}"
    );
}

#[test]
fn int_min_remainder_reports_r0103() {
    // `srem i32 -2147483648, -1` is poison for the same reason.
    let ir = lower(
        "module app.main;\n\
         fn main() -> int {\n\
         let a: int = -2147483647 - 1;\n\
         let b: int = a % -1;\n\
         return b;\n\
         }",
    );
    assert!(ir.contains("rem.overflow"), "{ir}");
    assert!(
        ir.contains("runtime error[R0103]: integer overflow"),
        "{ir}"
    );
}
