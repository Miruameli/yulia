//! Program, declaration, and recovery tests (0.1 §3.1).

use crate::common::assert_clean;
use crate::{only_fn, parse};
use yulia::ast::DeclKind;
use yulia::diagnostics::diagnostic::code;

#[test]
fn a_full_program_parses() {
    let (program, diags) = parse(
        r"
module app.main;
import core.io as io;
private struct Point { x: int; y: int }
fn add(a: int, b: int) -> int { return a + b; }
private const LIMIT: int = 10;
export fn main() { let p: Point = Point { x: 1, y: 2 }; }
",
    );
    assert_clean(&diags);
    assert_eq!(program.module.path.join("."), "app.main");
    assert_eq!(program.imports.len(), 1);
    assert_eq!(program.imports[0].alias.as_deref(), Some("io"));
    assert_eq!(program.decls.len(), 4);
}

#[test]
fn extern_declares_abi() {
    let (program, diags) = parse(r#"module m; extern "C" fn puts(s: CPtr) -> int;"#);
    assert_clean(&diags);
    let DeclKind::Extern(extern_decl) = &program.decls[0].kind else {
        panic!("expected an extern declaration");
    };
    assert_eq!(extern_decl.abi, "C");
    assert_eq!(extern_decl.name, "puts");
    let yulia::ast::TypeKind::CPtr = extern_decl.params[0].ty.kind else {
        panic!("expected a CPtr parameter");
    };
}

#[test]
fn missing_semicolon_is_reported() {
    let (program, diags) = parse("module m; fn f() { let x: int = 1 }");
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].code, code::EXPECTED_SEMICOLON);
    let f = only_fn(&program);
    assert_eq!(f.body.stmts.len(), 1);
}

#[test]
fn unexpected_token_recovers() {
    let (program, diags) = parse("module m; fn f() { let x: int = ; }");
    assert_eq!(diags.len(), 2);
    assert!(matches!(program.decls[0].kind, DeclKind::Fn(_)));
}

#[test]
fn empty_program_reports_missing_module() {
    let (program, diags) = parse("");
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].code, code::EXPECTED_MODULE);
    assert_eq!(program.decls.len(), 0);
}
