//! Program, module, and import parsing (0.1 §3.1).

use crate::ast::{ImportDecl, ModuleDecl, Program};
use crate::diagnostics::diagnostic::code;
use crate::lexer::TokenKind;
use crate::parser::{Parser, decl};

/// Parses `program ::= module_decl { import_decl } { top_level_decl } [ ";" ]`.
pub(crate) fn parse_program(parser: &mut Parser<'_>) -> Program {
    let module = parse_module_decl(parser);
    let mut imports = Vec::new();
    while parser.at(TokenKind::Import) {
        imports.push(parse_import_decl(parser));
    }
    let mut decls = Vec::new();
    while !parser.at(TokenKind::Eof) {
        if parser.eat(TokenKind::Semi) {
            continue;
        }
        let before = parser.pos;
        decls.push(decl::parse_top_level_decl(parser));
        recover_decl(parser);
        if parser.pos == before {
            parser.unexpected();
        }
    }
    Program {
        module,
        imports,
        decls,
    }
}

/// Parses `module_decl ::= "module" module_path ";"`.
fn parse_module_decl(parser: &mut Parser<'_>) -> ModuleDecl {
    let start = parser.span().start;
    if !parser.eat(TokenKind::Module) {
        parser.error(code::EXPECTED_MODULE, "expected `module` declaration");
        if parser.at(TokenKind::Eof) {
            return ModuleDecl {
                path: Vec::new(),
                span: parser.span(),
            };
        }
    }
    let path = parse_module_path(parser);
    parser.expect(TokenKind::Semi, code::EXPECTED_SEMICOLON);
    ModuleDecl {
        path,
        span: parser.span_since(start),
    }
}

/// Parses `import_decl ::= "import" module_path [ "as" IDENT ] ";"`.
fn parse_import_decl(parser: &mut Parser<'_>) -> ImportDecl {
    let start = parser.span().start;
    parser.bump();
    let path = parse_module_path(parser);
    let alias = if parser.eat_ident("as") {
        Some(parser.expect_ident())
    } else {
        None
    };
    parser.expect(TokenKind::Semi, code::EXPECTED_SEMICOLON);
    ImportDecl {
        path,
        alias,
        span: parser.span_since(start),
    }
}

/// Parses `module_path ::= IDENT { "." IDENT }`.
fn parse_module_path(parser: &mut Parser<'_>) -> Vec<String> {
    let mut path = Vec::new();
    if parser.at(TokenKind::Ident) {
        path.push(parser.bump().text.to_string());
    } else {
        parser.error(code::EXPECTED_IDENT, "expected identifier");
        return path;
    }
    while parser.eat(TokenKind::Dot) {
        path.push(parser.expect_ident());
    }
    path
}

/// Skips tokens after a failed declaration until the next declaration.
fn recover_decl(parser: &mut Parser<'_>) {
    const STOPS: &[TokenKind] = &[
        TokenKind::Semi,
        TokenKind::Private,
        TokenKind::Struct,
        TokenKind::Fn,
        TokenKind::Extern,
        TokenKind::Const,
        TokenKind::Eof,
    ];
    parser.recover_until(STOPS);
    parser.eat(TokenKind::Semi);
}
