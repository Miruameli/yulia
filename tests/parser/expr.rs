//! Expression tests (0.1 §3.4).

use crate::common::assert_clean;
use crate::{only_fn, parse};
use yulia::ast::{BinaryOp, ExprKind, StmtKind};

#[test]
fn operator_precedence_is_respected() {
    let (program, diags) = parse("module m; fn f() -> int { return 1 + 2 * 3; }");
    assert_clean(&diags);
    let f = only_fn(&program);
    let StmtKind::Return(Some(expr)) = &f.body.stmts[0].kind else {
        panic!("expected a return statement");
    };
    let ExprKind::Binary { op, rhs, .. } = &expr.kind else {
        panic!("expected a binary operation");
    };
    assert_eq!(*op, BinaryOp::Add);
    assert!(matches!(
        rhs.kind,
        ExprKind::Binary {
            op: BinaryOp::Mul,
            ..
        }
    ));
}

#[test]
fn assignment_is_right_associative() {
    let (program, diags) = parse("module m; fn f() { a = b = c; }");
    assert_clean(&diags);
    let f = only_fn(&program);
    let StmtKind::Expr(outer) = &f.body.stmts[0].kind else {
        panic!("expected an expression statement");
    };
    let ExprKind::Assign { value, .. } = &outer.kind else {
        panic!("expected an assignment");
    };
    assert!(matches!(value.kind, ExprKind::Assign { .. }));
}

#[test]
fn postfix_binds_tightest() {
    let (program, diags) = parse("module m; fn f() { a.b.c[0](1); }");
    assert_clean(&diags);
    let f = only_fn(&program);
    let StmtKind::Expr(expr) = &f.body.stmts[0].kind else {
        panic!("expected an expression statement");
    };
    let ExprKind::Call { callee, args } = &expr.kind else {
        panic!("expected a call");
    };
    assert_eq!(args.len(), 1);
    let ExprKind::Index { base, .. } = &callee.kind else {
        panic!("expected an index expression");
    };
    let ExprKind::Field { name, .. } = &base.kind else {
        panic!("expected a field access");
    };
    assert_eq!(name, "c");
}

#[test]
fn struct_literal_parses() {
    let (program, diags) = parse("module m; fn f() { let p: P = P { x: 1, y: 2 }; }");
    assert_clean(&diags);
    let f = only_fn(&program);
    let StmtKind::Let {
        init: Some(expr), ..
    } = &f.body.stmts[0].kind
    else {
        panic!("expected a let statement");
    };
    let ExprKind::StructLiteral { name, fields } = &expr.kind else {
        panic!("expected a struct literal");
    };
    assert_eq!(name, "P");
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name, "x");
}

#[test]
fn array_literals_parse() {
    let (program, diags) =
        parse("module m; fn f() { let a: [int; 3] = [1, 2, 3]; let b: [int; 4] = [0; 4]; }");
    assert_clean(&diags);
    let f = only_fn(&program);
    let StmtKind::Let {
        init: Some(first), ..
    } = &f.body.stmts[0].kind
    else {
        panic!("expected a let statement");
    };
    assert!(matches!(first.kind, ExprKind::ArrayLiteral { .. }));
    let StmtKind::Let {
        init: Some(second), ..
    } = &f.body.stmts[1].kind
    else {
        panic!("expected a let statement");
    };
    let ExprKind::ArrayRepeat { count, .. } = &second.kind else {
        panic!("expected a repeat expression");
    };
    assert_eq!(count, "4");
}

#[test]
fn panic_parses() {
    let (program, diags) = parse(r#"module m; fn f() { panic("boom"); }"#);
    assert_clean(&diags);
    let f = only_fn(&program);
    let StmtKind::Expr(expr) = &f.body.stmts[0].kind else {
        panic!("expected an expression statement");
    };
    let ExprKind::Panic(Some(message)) = &expr.kind else {
        panic!("expected a panic with a message");
    };
    assert!(matches!(message.kind, ExprKind::StringLit(_)));
}

#[test]
fn struct_literal_inside_parens_parses() {
    let (program, diags) = parse("module m; fn f() { if (P { x: 1 }) { } }");
    assert_clean(&diags);
    let f = only_fn(&program);
    let StmtKind::If { cond, .. } = &f.body.stmts[0].kind else {
        panic!("expected an if statement");
    };
    let ExprKind::StructLiteral { name, .. } = &cond.kind else {
        panic!("expected a struct literal condition");
    };
    assert_eq!(name, "P");
}
