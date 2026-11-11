//! Statement parsing (0.1 §3.3).

mod ctrl;

use crate::ast::{Block, Expr, Stmt, StmtKind};
use crate::diagnostics::code;
use crate::lexer::TokenKind;
use crate::parser::{Parser, expr, ty};

/// Parses `block ::= "{" { statement } "}"`.
pub(crate) fn parse_block(parser: &mut Parser<'_>) -> Block {
    let start = parser.span().start;
    if !parser.eat(TokenKind::LBrace) {
        parser.error(code::EXPECTED_LBRACE, "expected `{`");
    }
    let mut stmts = Vec::new();
    while !parser.at(TokenKind::RBrace) && !parser.at(TokenKind::Eof) {
        let before = parser.pos;
        stmts.push(parse_stmt(parser));
        if parser.pos == before {
            parser.unexpected();
        }
    }
    parser.expect(TokenKind::RBrace, code::EXPECTED_RBRACE);
    Block {
        stmts,
        span: parser.span_since(start),
    }
}

/// Parses a statement.
pub(crate) fn parse_stmt(parser: &mut Parser<'_>) -> Stmt {
    let start = parser.span().start;
    match parser.peek() {
        TokenKind::Let => let_stmt(parser, start),
        TokenKind::Const => const_stmt(parser, start),
        TokenKind::If => ctrl::parse_if(parser, start),
        TokenKind::While => ctrl::parse_while(parser, start),
        TokenKind::For => ctrl::parse_for(parser, start),
        TokenKind::Pin => ctrl::parse_pin(parser, start),
        TokenKind::Return => return_stmt(parser, start),
        TokenKind::Break => simple(parser, start, StmtKind::Break),
        TokenKind::Continue => simple(parser, start, StmtKind::Continue),
        _ => expr_stmt(parser, start),
    }
}

/// Parses `let IDENT ":" type [ "=" expr ] ";"`.
fn let_stmt(parser: &mut Parser<'_>, start: usize) -> Stmt {
    parser.bump();
    let name = parser.expect_ident();
    parser.expect(TokenKind::Colon, code::EXPECTED_COLON);
    let declared = ty::parse_type(parser);
    let init = assignment(parser);
    parser.expect(TokenKind::Semi, code::EXPECTED_SEMICOLON);
    Stmt {
        kind: StmtKind::Let {
            name,
            ty: declared,
            init,
        },
        span: parser.span_since(start),
    }
}

/// Parses `const IDENT ":" type "=" expr ";"`.
fn const_stmt(parser: &mut Parser<'_>, start: usize) -> Stmt {
    parser.bump();
    let name = parser.expect_ident();
    parser.expect(TokenKind::Colon, code::EXPECTED_COLON);
    let declared = ty::parse_type(parser);
    parser.expect(TokenKind::Eq, code::EXPECTED_EQ);
    let value = expr::parse_expr(parser);
    parser.expect(TokenKind::Semi, code::EXPECTED_SEMICOLON);
    Stmt {
        kind: StmtKind::Const {
            name,
            ty: declared,
            value,
        },
        span: parser.span_since(start),
    }
}

/// Parses `"=" expr` when an initializer follows.
fn assignment(parser: &mut Parser<'_>) -> Option<Expr> {
    if parser.eat(TokenKind::Eq) {
        Some(expr::parse_expr(parser))
    } else {
        None
    }
}

/// Parses `return [ expr ] ";"`.
fn return_stmt(parser: &mut Parser<'_>, start: usize) -> Stmt {
    parser.bump();
    let value = if parser.at(TokenKind::Semi) {
        None
    } else {
        Some(expr::parse_expr(parser))
    };
    parser.expect(TokenKind::Semi, code::EXPECTED_SEMICOLON);
    Stmt {
        kind: StmtKind::Return(value),
        span: parser.span_since(start),
    }
}

/// Parses `break ";"` or `continue ";"`.
fn simple(parser: &mut Parser<'_>, start: usize, kind: StmtKind) -> Stmt {
    parser.bump();
    parser.expect(TokenKind::Semi, code::EXPECTED_SEMICOLON);
    Stmt {
        kind,
        span: parser.span_since(start),
    }
}

/// Parses `expr ";"`.
fn expr_stmt(parser: &mut Parser<'_>, start: usize) -> Stmt {
    let node = expr::parse_expr(parser);
    parser.expect(TokenKind::Semi, code::EXPECTED_SEMICOLON);
    Stmt {
        kind: StmtKind::Expr(node),
        span: parser.span_since(start),
    }
}
