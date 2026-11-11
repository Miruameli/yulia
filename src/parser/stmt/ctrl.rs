//! Control-flow statements (0.1 §3.3).

use crate::ast::{ElseBranch, Stmt, StmtKind};
use crate::diagnostics::code;
use crate::lexer::TokenKind;
use crate::parser::{Parser, expr, stmt, ty};

/// Parses `if expr block [ "else" ( if_stmt | block ) ]`.
pub(crate) fn parse_if(parser: &mut Parser<'_>, start: usize) -> Stmt {
    parser.bump();
    let cond = parser.without_struct_literals(|p| expr::parse_expr(p));
    let then = stmt::parse_block(parser);
    let else_branch = if parser.eat(TokenKind::Else) {
        Some(else_branch(parser))
    } else {
        None
    };
    Stmt {
        kind: StmtKind::If {
            cond,
            then,
            else_branch,
        },
        span: parser.span_since(start),
    }
}

/// Parses the `"else"` tail of an `if`.
fn else_branch(parser: &mut Parser<'_>) -> ElseBranch {
    if parser.at(TokenKind::If) {
        let start = parser.span().start;
        ElseBranch::If(Box::new(parse_if(parser, start)))
    } else {
        ElseBranch::Block(stmt::parse_block(parser))
    }
}

/// Parses `while expr block`.
pub(crate) fn parse_while(parser: &mut Parser<'_>, start: usize) -> Stmt {
    parser.bump();
    let cond = parser.without_struct_literals(|p| expr::parse_expr(p));
    let body = stmt::parse_block(parser);
    Stmt {
        kind: StmtKind::While { cond, body },
        span: parser.span_since(start),
    }
}

/// Parses `pin lvalue "as" IDENT block`.
pub(crate) fn parse_pin(parser: &mut Parser<'_>, start: usize) -> Stmt {
    parser.bump();
    let target = expr::parse_expr(parser);
    if !parser.eat_ident("as") {
        parser.error(code::EXPECTED_AS, "expected `as`");
    }
    let alias = parser.expect_ident();
    let body = stmt::parse_block(parser);
    Stmt {
        kind: StmtKind::Pin {
            target,
            alias,
            body,
        },
        span: parser.span_since(start),
    }
}

/// Parses both `for` forms, dispatching on `in` versus `:`.
pub(crate) fn parse_for(parser: &mut Parser<'_>, start: usize) -> Stmt {
    parser.bump();
    let var_start = parser.span().start;
    let var = parser.expect_ident();
    if parser.eat(TokenKind::In) {
        return for_range(parser, start, var);
    }
    for_c_style(parser, start, var, var_start)
}

/// Parses `in expr ".." expr block`.
fn for_range(parser: &mut Parser<'_>, start: usize, var: String) -> Stmt {
    let range_start = expr::parse_expr(parser);
    parser.expect(TokenKind::DotDot, code::EXPECTED_DOTDOT);
    let end = parser.without_struct_literals(|p| expr::parse_expr(p));
    let body = stmt::parse_block(parser);
    Stmt {
        kind: StmtKind::ForRange {
            var,
            start: range_start,
            end,
            body,
        },
        span: parser.span_since(start),
    }
}

/// Parses `":" type "=" expr ";" expr ";" assign_expr block`.
fn for_c_style(parser: &mut Parser<'_>, start: usize, var: String, var_start: usize) -> Stmt {
    parser.expect(TokenKind::Colon, code::EXPECTED_COLON);
    let declared = ty::parse_type(parser);
    parser.expect(TokenKind::Eq, code::EXPECTED_EQ);
    let init = expr::parse_expr(parser);
    parser.expect(TokenKind::Semi, code::EXPECTED_SEMICOLON);
    let cond = expr::parse_expr(parser);
    parser.expect(TokenKind::Semi, code::EXPECTED_SEMICOLON);
    let update = expr::parse_expr(parser);
    let body = stmt::parse_block(parser);
    let init_stmt = Stmt {
        kind: StmtKind::Let {
            name: var,
            ty: declared,
            init: Some(init),
        },
        span: parser.span_since(var_start),
    };
    Stmt {
        kind: StmtKind::ForCStyle {
            init: Box::new(init_stmt),
            cond,
            update,
            body,
        },
        span: parser.span_since(start),
    }
}
