//! Statement tests (0.1 §3.3).

use crate::common::assert_clean;
use crate::{only_fn, parse};
use yulia::ast::{ElseBranch, StmtKind};

#[test]
fn for_loops_parse() {
    let (program, diags) =
        parse("module m; fn f() { for i in 0..10 { } for j: int = 0; j < 3; j = j + 1 { } }");
    assert_clean(&diags);
    let f = only_fn(&program);
    assert!(matches!(f.body.stmts[0].kind, StmtKind::ForRange { .. }));
    assert!(matches!(f.body.stmts[1].kind, StmtKind::ForCStyle { .. }));
}

#[test]
fn if_else_chain_parses() {
    let (program, diags) = parse("module m; fn f() { if a { } else if b { } else { } }");
    assert_clean(&diags);
    let f = only_fn(&program);
    let StmtKind::If {
        else_branch: Some(ElseBranch::If(inner)),
        ..
    } = &f.body.stmts[0].kind
    else {
        panic!("expected an if statement");
    };
    let StmtKind::If {
        else_branch: Some(ElseBranch::Block(_)),
        ..
    } = &inner.kind
    else {
        panic!("expected a nested if statement");
    };
}

#[test]
fn pin_parses() {
    let (program, diags) = parse("module m; fn f() { pin x as y { } }");
    assert_clean(&diags);
    let f = only_fn(&program);
    let StmtKind::Pin { alias, .. } = &f.body.stmts[0].kind else {
        panic!("expected a pin statement");
    };
    assert_eq!(alias, "y");
}

#[test]
fn while_condition_does_not_swallow_the_body() {
    let (program, diags) = parse("module m; fn f() { while a { } }");
    assert_clean(&diags);
    let f = only_fn(&program);
    assert!(matches!(f.body.stmts[0].kind, StmtKind::While { .. }));
}
