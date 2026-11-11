//! Stable diagnostic codes, grouped by compiler stage.
//!
//! The catalog is `docs/specs/2026-11-09-yulia-error-panic-model.md` §2.2:
//! `E0xx` lexer, `E1xx` parser, `E2xx` type checker, `E3xx` region inference
//! (reserved, Phase 2+), `R0xx` runtime. Codes are stable across releases
//! (0.9 §4); the message text may change.

use super::Code;

/// A character that cannot start a token (E001).
pub const UNEXPECTED_CHAR: Code = Code("E001");
/// A block comment that is not closed before end of input (E002).
pub const UNTERMINATED_BLOCK_COMMENT: Code = Code("E002");
/// A string literal that is not closed before end of input (E003).
pub const UNTERMINATED_STRING: Code = Code("E003");
/// A character literal that is not closed before end of input (E004).
pub const UNTERMINATED_CHAR: Code = Code("E004");
/// An escape sequence that is not part of the grammar (E005).
pub const INVALID_ESCAPE: Code = Code("E005");
/// A numeric literal with misplaced underscores or no digits (E006).
pub const INVALID_NUMERIC_LITERAL: Code = Code("E006");
/// A character literal that is empty or holds more than one character (E007).
pub const INVALID_CHAR_LITERAL: Code = Code("E007");
// Parser (E1xx).
/// A `module` declaration is missing (E101).
pub const EXPECTED_MODULE: Code = Code("E101");
/// An identifier is missing (E102).
pub const EXPECTED_IDENT: Code = Code("E102");
/// A `;` is missing (E103).
pub const EXPECTED_SEMICOLON: Code = Code("E103");
/// A `{` is missing (E104).
pub const EXPECTED_LBRACE: Code = Code("E104");
/// A `}` is missing (E105).
pub const EXPECTED_RBRACE: Code = Code("E105");
/// A `(` is missing (E106).
pub const EXPECTED_LPAREN: Code = Code("E106");
/// A `)` is missing (E107).
pub const EXPECTED_RPAREN: Code = Code("E107");
/// A `[` is missing (E108).
pub const EXPECTED_LBRACKET: Code = Code("E108");
/// A `]` is missing (E109).
pub const EXPECTED_RBRACKET: Code = Code("E109");
/// A `:` is missing (E110).
pub const EXPECTED_COLON: Code = Code("E110");
/// A `,` is missing (E111).
pub const EXPECTED_COMMA: Code = Code("E111");
/// A `.` is missing (E112).
pub const EXPECTED_DOT: Code = Code("E112");
/// A `..` is missing (E113).
pub const EXPECTED_DOTDOT: Code = Code("E113");
/// A `->` is missing (E114).
pub const EXPECTED_ARROW: Code = Code("E114");
/// An `=` is missing (E115).
pub const EXPECTED_EQ: Code = Code("E115");
/// An expression is missing (E116).
pub const EXPECTED_EXPRESSION: Code = Code("E116");
/// A type is missing (E117).
pub const EXPECTED_TYPE: Code = Code("E117");
/// A statement is missing (E118).
pub const EXPECTED_STATEMENT: Code = Code("E118");
/// An integer literal is missing (E119).
pub const EXPECTED_INT_LIT: Code = Code("E119");
/// A string literal is missing (E120).
pub const EXPECTED_STRING_LIT: Code = Code("E120");
/// A token that cannot appear here (E121).
pub const UNEXPECTED_TOKEN: Code = Code("E121");
/// An `as` is missing (E122).
pub const EXPECTED_AS: Code = Code("E122");
/// An `in` is missing (E123).
pub const EXPECTED_IN: Code = Code("E123");
/// A keyword is missing (E124).
pub const EXPECTED_KEYWORD: Code = Code("E124");
// Type checker (E2xx).
/// A name is not defined (E201).
pub const UNDEFINED_NAME: Code = Code("E201");
/// Two types do not match (E202).
pub const TYPE_MISMATCH: Code = Code("E202");
/// A call target is not a function (E203).
pub const NOT_A_FUNCTION: Code = Code("E203");
/// A call has the wrong number of arguments (E204).
pub const ARGUMENT_COUNT: Code = Code("E204");
/// A struct type is not declared (E205).
pub const UNKNOWN_STRUCT: Code = Code("E205");
/// A struct has no such field (E206).
pub const UNKNOWN_FIELD: Code = Code("E206");
/// A struct literal omits a field (E207).
pub const MISSING_FIELD: Code = Code("E207");
/// A struct literal initializes a field twice (E208).
pub const DUPLICATE_FIELD: Code = Code("E208");
/// An assignment target is not an lvalue (E209).
pub const NOT_AN_LVALUE: Code = Code("E209");
/// A condition is not `bool` (E210).
pub const CONDITION_NOT_BOOL: Code = Code("E210");
/// A `return` value does not match the return type (E211).
pub const RETURN_TYPE_MISMATCH: Code = Code("E211");
/// A `break` outside any loop (E212).
pub const BREAK_OUTSIDE_LOOP: Code = Code("E212");
/// A `continue` outside any loop (E213).
pub const CONTINUE_OUTSIDE_LOOP: Code = Code("E213");
/// A construct outside the Phase 1 subset (E214).
pub const OUT_OF_SUBSET: Code = Code("E214");
/// A name is defined twice (E215).
pub const DUPLICATE_DEFINITION: Code = Code("E215");
/// A type annotation names no known type (E216).
pub const UNKNOWN_TYPE: Code = Code("E216");
/// Array elements have different types (E217).
pub const ARRAY_ELEMENT_MISMATCH: Code = Code("E217");
/// An array index is not `int` (E218).
pub const INDEX_NOT_INT: Code = Code("E218");
/// An index is applied to a non-array (E219).
pub const NOT_AN_ARRAY: Code = Code("E219");
/// A field is accessed on a non-struct (E220).
pub const NOT_A_STRUCT: Code = Code("E220");
/// A `void` value is used in an expression (E221).
pub const VOID_VALUE: Code = Code("E221");
/// A variable is read before it is definitely assigned (E222).
pub const UNINITIALIZED_READ: Code = Code("E222");
/// A struct contains itself by value (E223).
pub const RECURSIVE_STRUCT: Code = Code("E223");
/// An array length is not a positive integer (E224).
pub const BAD_ARRAY_LENGTH: Code = Code("E224");
/// A parameter name is duplicated (E225).
pub const DUPLICATE_PARAM: Code = Code("E225");
// Runtime (R0xx, Phase 1b).
/// An array index is out of bounds (R0101).
pub const ARRAY_OUT_OF_BOUNDS: Code = Code("R0101");
/// A division by zero (R0102).
pub const DIVISION_BY_ZERO: Code = Code("R0102");
/// An integer overflow trap (R0103).
pub const INTEGER_OVERFLOW: Code = Code("R0103");
