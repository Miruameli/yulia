# Yulia Language — Grammar Standard (Draft 0.1)

**Date:** 2026-11-09
**Status:** Draft for review (Phase 0 deliverable 0.1)
**Depends on:** `docs/specs/2026-11-09-yulia-design.md`, `docs/specs/2026-11-09-yulia-design-review.md`
**Plan ref:** unavailable — `docs/plans/` was lost on 2026-11-10 (see decision log #82).

---

## 1. Overview and design rationale

This document defines the concrete grammar and lexical rules for Yulia. It is the **standard** that the lexer, parser, LSP, and formatter all consume. No implementation may begin until this document is reviewed and signed off.

### 1.1 Design goals for the syntax

1. **Simplicity.** The syntax should be learnable in an afternoon. No context-sensitive keywords, no significant whitespace, no implicit type conversion.
2. **Explicit types.** Every declaration carries its type. No type inference, no `auto`, no `var`.
3. **Procedural.** Functions, structs, modules. No classes, no inheritance, no virtual dispatch.
4. **Unambiguous with small lookahead.** Every program has exactly one parse tree. The grammar requires at most 3 tokens of lookahead (worst case: `for`-loop disambiguation). Struct literals require 2 tokens. All other constructs are LL(1).
5. **Memory-invisible.** The programmer never writes `&`, `*`, `malloc`, `free`, or lifetime annotations. References and region management are compiler-internal. Values are passed by value at the language level; the compiler may optimize to references internally.
6. **Custom but familiar.** The syntax borrows proven constructs (C-style braces, `fn` for functions, `struct` for aggregates) but is not a copy of any single language.

### 1.2 Notation

This grammar uses **EBNF** (ISO/IEC 14977) with the following conventions:

- `::=` — production rule
- `|` — alternation
- `[ ... ]` — optional (0 or 1)
- `{ ... }` — repetition (0 or more)
- `( ... )` — grouping
- `"..."` — terminal (literal token)
- `(* ... *)` — comment
- Uppercase names (e.g., `IDENT`) — lexical tokens
- Lowercase names (e.g., `expr`) — non-terminals

---

## 2. Lexical rules

### 2.1 Character set

Source files are UTF-8. The lexical grammar operates on Unicode code points. Identifiers and string literals may contain Unicode. Keywords and operators are ASCII.

### 2.2 Whitespace and newlines

Whitespace (space `U+0020`, tab `U+0009`, carriage return `U+000D`, newline `U+000A`) separates tokens but is otherwise insignificant. Newlines do not terminate statements; semicolons (`;`) do. A newline is treated as whitespace.

**Rationale:** Significant whitespace (Python-style) makes formatter and LSP implementation harder and creates ambiguity with multi-line expressions. Semicolons are explicit and unambiguous.

### 2.3 Comments

```
line_comment    ::= "//" { any_char_except_newline } newline
block_comment   ::= "/*" { any_char } "*/"
```

Block comments do not nest. A `/*` inside a block comment does not start a new comment; the first `*/` closes it.

### 2.4 Identifiers

```
IDENT           ::= (letter | "_") { letter | digit | "_" }
letter          ::= "a".."z" | "A".."Z" | unicode_letter
digit           ::= "0".."9"
```

Identifiers are case-sensitive. By convention:
- Type names start with an uppercase letter (`Point`, `Color`).
- Function names, variable names, and module names start with a lowercase letter (`main`, `add`, `x`).
- Constants (if any) use `SCREAMING_SNAKE_CASE`.

### 2.5 Keywords (reserved)

The following identifiers are reserved and cannot be used as variable, function, or type names:

```
module  import  private  struct  fn  let  const  if  else  while  for  in
return  break  continue  true  false  int  float  bool  char  string  void
extern  export  pin  panic  thread  spawn  async  await  send  recv  mutex  defer  yield  match  trait  impl
```

### 2.6 Literals
```
INT_LIT         ::= digit { digit } { "_" digit { digit } }
                   | "0x" hex_digit { hex_digit } { "_" hex_digit { hex_digit } }
                   | "0b" bin_digit { bin_digit } { "_" bin_digit { bin_digit } }
                   | "0o" oct_digit { oct_digit } { "_" oct_digit { oct_digit } }

FLOAT_LIT       ::= digit { digit } "." digit { digit } [ "e" [ "+" | "-" ] digit { digit } ]
                   | digit { digit } "e" [ "+" | "-" ] digit { digit }

BOOL_LIT        ::= "true" | "false"

CHAR_LIT        ::= "'" ( char_escape | any_char_except_quote_and_backslash ) "'"

STRING_LIT      ::= '"' { string_char } '"'
string_char     ::= char_escape | any_char_except_quote_and_backslash
char_escape     ::= "\\" ( "n" | "t" | "r" | "\\" | "'" | '"' | "0" | "x" hex_digit hex_digit | "u{" hex_digit { hex_digit } "}" )
```
(* Underscore separators are part of the INT_LIT production in §2.6.
   Consecutive underscores and leading/trailing underscores are lexer errors. *)

### 2.7 Operators and delimiters

```
// Arithmetic
"+"  "-"  "*"  "/"  "%"

// Comparison
"=="  "!="  "<"  ">"  "<="  ">="

// Logical
"&&"  "||"  "!"

// Assignment
"="

// Delimiters
"("  ")"  "{"  "}"  "["  "]"  ";"  ","  ":"  "."  ".."
```

### 2.8 Operator precedence and associativity

From lowest to highest precedence:

| Precedence | Operators | Associativity |
|---|---|---|
| 1 | `=` | Right |
| 2 | `||` | Left |
| 3 | `&&` | Left |
| 4 | `==` `!=` | Left |
| 5 | `<` `>` `<=` `>=` | Left |
| 6 | `+` `-` | Left |
| 7 | `*` `/` `%` | Left |
| 8 | `!` `-` (unary) | Right (prefix) |
| 9 | `.` `[]` `()` (call) | Left (postfix) |
**Note:** `..` is a delimiter used only in `for`-loop range expressions (`for_stmt`).
It is NOT a general expression operator and does not appear in the precedence table above.

## 3. Grammar (EBNF)

### 3.1 Top-level

```
program         ::= module_decl { import_decl } { top_level_decl } [ ";" ]

module_decl     ::= "module" module_path ";"

module_path     ::= IDENT { "." IDENT }

import_decl     ::= "import" module_path [ "as" IDENT ] ";"

top_level_decl  ::= struct_decl
                  | fn_decl
                  | extern_decl
                  | const_decl
struct_decl     ::= [ "private" ] "struct" IDENT "{" { struct_field } "}"
struct_field    ::= IDENT ":" type ";"

fn_decl         ::= [ "private" | "export" ] "fn" IDENT "(" [ param_list ] ")" [ "->" return_type ] block

extern_decl     ::= "extern" STRING_LIT "fn" IDENT "(" [ param_list ] ")" [ "->" return_type ] ";"

param_list      ::= param { "," param }
param           ::= IDENT ":" type

const_decl      ::= [ "private" ] "const" IDENT ":" type "=" expr ";"
```

### 3.2 Types

```
type            ::= primitive_type
                  | array_type
                  | "CPtr"                     (* opaque C pointer (Phase 2) *)
                  | IDENT                      (* user-defined struct type *)

return_type     ::= primitive_type | "void" | IDENT | array_type
array_type      ::= "[" type ";" INT_LIT "]"
```

**Note:** `void` is only valid as a function return type. All other types must be concrete.

### 3.3 Statements

```
block           ::= "{" { statement } "}"

statement       ::= let_stmt
                  | const_stmt
                  | pin_stmt
                  | if_stmt
                  | while_stmt
                  | for_stmt
                  | return_stmt
                  | break_stmt
                  | continue_stmt
                  | expr_stmt

let_stmt        ::= "let" IDENT ":" type [ "=" expr ] ";"

const_stmt      ::= "const" IDENT ":" type "=" expr ";"
pin_stmt        ::= "pin" lvalue "as" IDENT block

(* lvalue defines valid assignment targets for semantic analysis.
   Assignment is an expression (assign_expr), used via expr_stmt. *)
lvalue          ::= IDENT
                  | lvalue "." IDENT
                  | lvalue "[" expr "]"

if_stmt         ::= "if" expr block [ "else" ( if_stmt | block ) ]

while_stmt      ::= "while" expr block

for_stmt        ::= "for" IDENT "in" expr ".." expr block
                  | "for" IDENT ":" type "=" expr ";" expr ";" assign_expr block

return_stmt     ::= "return" [ expr ] ";"

break_stmt      ::= "break" ";"

continue_stmt   ::= "continue" ";"

expr_stmt       ::= expr ";"
```

**Note on `let`:** A `let` with an initializer (`let x: int = 5;`) initializes the variable. A `let` without an initializer (`let x: int;`) declares an uninitialized variable; reading it before assignment is a compile error (MEM-SAFE-4).

**Semantic restrictions (not expressible in the grammar):**
- `break` and `continue` are only valid inside a `while` or `for` loop. Using them elsewhere is a compile error.
- `return` with a value is only valid in a function with a non-`void` return type. `return` without a value is only valid in a `void` function. Arity mismatch is a compile error.
- `return` outside a function is a compile error.

**Note on condition disambiguation:** `if a { }` is ambiguous — `a { }` could read as a struct literal, leaving the `if` without a body. The grammar resolves it by position: in an `if`/`while` condition or a `for` range bound, a `{` begins the branch/loop body, not a struct literal. A struct literal in one of those positions must be parenthesized: `if (p == Point { x: 1, y: 2 }) { ... }`. Struct literals remain available everywhere else, including inside parentheses, array elements, and argument lists.

### 3.4 Expressions

```
expr            ::= assign_expr

assign_expr     ::= or_expr [ "=" assign_expr ]

or_expr         ::= and_expr { "||" and_expr }

and_expr        ::= eq_expr { "&&" eq_expr }

eq_expr         ::= cmp_expr { ( "==" | "!=" ) cmp_expr }

cmp_expr        ::= add_expr { ( "<" | ">" | "<=" | ">=" ) add_expr }

add_expr        ::= mul_expr { ( "+" | "-" ) mul_expr }

mul_expr        ::= unary_expr { ( "*" | "/" | "%" ) unary_expr }

unary_expr      ::= ( "!" | "-" ) unary_expr
                  | postfix_expr

postfix_expr    ::= primary_expr { postfix_op }

postfix_op      ::= "." IDENT
                  | "[" expr "]"
                  | "(" [ arg_list ] ")"

primary_expr    ::= INT_LIT
                  | FLOAT_LIT
                  | BOOL_LIT
                  | CHAR_LIT
                  | STRING_LIT
                  | IDENT
                  | "(" expr ")"
                  | struct_literal
                  | array_literal
                  | panic_expr

struct_literal  ::= IDENT "{" [ field_init_list ] "}"

field_init_list ::= field_init { "," field_init } [ "," ]

field_init      ::= IDENT ":" expr

array_literal   ::= "[" [ expr_list ] "]"
                  | "[" expr ";" INT_LIT "]"   (* repeat: [0; 256] fills 256 slots with 0 *)

arg_list        ::= expr { "," expr }

expr_list       ::= expr { "," expr }
```

**Note on struct literals:** A struct literal `Point { x: 1, y: 2 }` requires all fields to be initialized. Missing fields are a compile error.

**Note on array literals:** `[1, 2, 3]` infers its size from the number of elements. The type is `[int; 3]`. An empty array `[]` requires an explicit type annotation. The repeat form `[e; N]` has type `[T; N]` where `T` is the type of `e` (e.g., `[0; 256]` has type `[int; 256]`).

### 3.5 Assign expression in for-loops

```
assign_expr     ::= ... (as above, used in for-loop update clause)
```

The for-loop update clause uses `assign_expr` to allow `i = i + 1` style updates.

---

## 4. Module grammar

### 4.1 Module declaration

Every source file begins with a module declaration (defined in §3.1):

```
module_decl     ::= "module" module_path ";"   (* see §3.1 *)
```

The module path determines the module's identity in the module graph. A file at `src/math/vec.yl` declares `module math.vec`.

### 4.2 Imports

Imports use the production defined in §3.1:

```
import_decl     ::= "import" module_path [ "as" IDENT ] ";"   (* see §3.1 *)
```

Examples:
```
import std.io;                    // imports std.io; names accessed as io.println_str
import std.math as m;             // imports std.math; names accessed as m.sqrt
import std.collections.array;      // imports std.collections.array
```

Imported names are accessed via the module's last path component (or the alias). There is no `from ... import ...` form; all imports are module-level.

**Alias mandatory rule:** If the last component of the module path is a reserved keyword (e.g., `string`, `int`), an alias MUST be provided. Example: `import std.string as str;` (not `import std.string;`, which would make `string` both a keyword and a module name, causing a parse error).
### 4.3 Visibility and exports

All top-level declarations (`struct`, `fn`, `const`) are **public by default**. The `private` keyword restricts a declaration to the current module. The `export` keyword marks a function as callable from C (FFI); it is mutually exclusive with `private`:

```
private fn helper(x: int) -> int { ... }   // not visible outside this module
fn public_fn(x: int) -> int { ... }          // visible to importers
export fn c_abi(x: int) -> int { ... }       // callable from C (Phase 2)
```

There is no `pub` keyword; visibility is opt-out (private) rather than opt-in (public). `export` is an FFI concern, not a visibility modifier (see 0.6 §2.2).

### 4.4 Module resolution

- The compiler resolves module paths to source files using a search path (configured by the build system).
- Circular imports are a compile error.
- A module's public API is the set of its public declarations.
- Name conflicts between imported modules are resolved by qualification (`module_name.name`).

### 4.5 Entry point

A program's entry point is the `main` function in the `main` module (or the module specified by the build system):

```
module main;

fn main() {
    // entry point
}
```

The `main` function takes no arguments and returns `void` in Phase 1. Command-line arguments are provided via a standard library function in Phase 2.

---

## 5. Ambiguity resolution

### 5.1 Dangling else

An `else` binds to the nearest unmatched `if`. This is the standard resolution and is unambiguous given the grammar's structure (`if_stmt` allows `else (if_stmt | block)`).

### 5.2 Struct literal vs. block

A struct literal `Foo { ... }` can appear in expression position. In statement position, `{` begins a block. The parser resolves this by context: `expr_stmt` expects an expression, and `struct_literal` is a valid expression. If an identifier followed by `{` appears where a statement is expected, it is parsed as an expression statement (struct literal), not a block. Empty struct literals `Foo {}` are valid.

### 5.3 Array literal vs. block

An array literal `[...]` uses square brackets, distinct from braces. No ambiguity.

### 5.4 Type vs. expression in `let`

`let x: int = 5;` — the `:` separates the name from the type. The type is parsed first, then `=` introduces the initializer. This is unambiguous.

### 5.5 Range `..` vs. member access

`..` is a distinct token. `a..b` is a range expression used in `for` loops. `a.b` is member access. No ambiguity.

---

## 6. Illustrative programs

### Program 1: Hello world

```yulia
module main;

import std.io;

fn main() {
    io.println_str("Hello, world!");
}
```

### Program 2: Arithmetic

```yulia
module main;

fn add(a: int, b: int) -> int {
    return a + b;
}

fn main() {
    let x: int = 10;
    let y: int = 20;
    let z: int = add(x, y);
    // z == 30
}
```

### Program 3: Functions with multiple parameters

```yulia
module main;

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
}
```

### Program 4: Struct definition and usage

```yulia
module main;

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
}
```

### Program 5: Arrays

```yulia
module main;

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
}
```

### Program 6: If/else

```yulia
module main;

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
}
```

### Program 7: While loop

```yulia
module main;

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
}
```

### Program 8: For loop (range-based)

```yulia
module main;

fn main() {
    let sum: int = 0;
    for i in 1..11 {
        sum = sum + i;
    }
    // sum == 55
}
```

### Program 9: For loop (C-style)

```yulia
module main;

fn main() {
    let sum: int = 0;
    for i: int = 0; i < 10; i = i + 1 {
        sum = sum + i;
    }
    // sum == 45
}
```

### Program 10: Nested structs

```yulia
module main;

struct Vec2 {
    x: float;
    y: float;
}

struct Rect {
    top_left: Vec2;
    bottom_right: Vec2;
}

fn area(r: Rect) -> float {
    let w: float = r.bottom_right.x - r.top_left.x;
    let h: float = r.bottom_right.y - r.top_left.y;
    return w * h;
}

fn main() {
    let r: Rect = Rect {
        top_left: Vec2 { x: 0.0, y: 0.0 },
        bottom_right: Vec2 { x: 3.0, y: 4.0 },
    };
    let a: float = area(r);  // a == 12.0
}
```

### Program 11: Recursion

```yulia
module main;

fn fib(n: int) -> int {
    if n <= 1 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

fn main() {
    let f: int = fib(10);  // f == 55
}
```

### Program 12: Multiple functions and constants

```yulia
module main;

const PI: float = 3.14159265;
const MAX_SIZE: int = 1024;

fn circle_area(r: float) -> float {
    return PI * r * r;
}

fn main() {
    let a: float = circle_area(2.0);  // a ≈ 12.566
}
```

### Program 13: String operations

```yulia
module main;

import std.string as str;

fn main() {
    let s: string = "Hello";
    let t: string = "World";
    let combined: string = str.concat(s, " ", t);
    let len: int = str.length(combined);  // len == 11
}
```

### Program 14: Boolean logic

```yulia
module main;

fn is_even(n: int) -> bool {
    return n % 2 == 0;
}

fn main() {
    let a: bool = is_even(4);   // true
    let b: bool = is_even(7);   // false
    let c: bool = a && !b;      // true
    let d: bool = a || b;       // true
}
```

### Program 15: Nested control flow

```yulia
module main;

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
}
```

### Program 16: Array indexing and mutation

```yulia
module main;

fn main() {
    let arr: [int; 3] = [10, 20, 30];
    arr[1] = 99;
    // arr == [10, 99, 30]
    let x: int = arr[0] + arr[2];  // x == 40
}
```

### Program 17: Private helper

```yulia
module math;

private fn square(x: int) -> int {
    return x * x;
}

fn hypotenuse(a: int, b: int) -> int {
    return square(a) + square(b);
}
```

### Program 18: Module import and usage

```yulia
module main;

import math;
import std.io as io;

fn main() {
    let h: int = math.hypotenuse(3, 4);  // h == 25
    io.println_int(h);  // stdlib provides type-specific print functions
    // (overloaded println is a Phase 2 stdlib design decision)
}
```
### Program 19: Char and escape sequences

```yulia
module main;

fn main() {
    let c: char = 'A';
    let newline: char = '\n';
    let unicode: char = '\u{1F600}';
    let msg: string = "Line 1\nLine 2\tTabbed";
    let quote: string = "She said \"hello\"";
}
```

### Program 20: Empty and minimal programs

```yulia
module empty;
```

```yulia
module main;

fn main() {
}
```

### Program 21: Complex expressions

```yulia
module main;

fn main() {
    let x: int = 2 + 3 * 4;        // x == 14
    let y: int = (2 + 3) * 4;      // y == 20
    let z: bool = 1 < 2 && 3 > 2;  // z == true
    let w: int = -x + y;           // w == 6
}
```

### Program 22: Struct with array field

```yulia
module main;

struct Buffer {
    data: [int; 256];
    len: int;
}

fn main() {
    let buf: Buffer = Buffer {
        data: [0; 256],
        len: 0,
    };
}
```

### Program 23: Break and continue

```yulia
module main;

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
}
```

### Program 24: Hex, binary, octal literals

```yulia
module main;

fn main() {
    let hex: int = 0xFF;        // 255
    let bin: int = 0b1010;      // 10
    let oct: int = 0o77;        // 63
    let big: int = 1_000_000;   // 1000000
    let flt: float = 1.5e3;     // 1500.0
}
```

---

## 7. Grammar properties

### 7.1 Ambiguity

The grammar is designed to be unambiguous. Key resolution rules:

1. **Dangling else:** binds to nearest `if` (§5.1).
2. **Struct literal in statement position:** parsed as expression statement (§5.2).
3. **Operator precedence:** fully specified (§2.8).
4. **No implicit conversions:** `int` and `float` are distinct; mixing them is a type error, not a coercion.

### 7.2 Lookahead and LL(k) compatibility

The grammar is **LL(3) in the worst case**, not LL(1). The maximum lookahead is 3 tokens:

- **3 tokens:** `for`-loop disambiguation (`for`, IDENT, then `in` vs `:`).
- **2 tokens:** struct literal vs. block (`IDENT`, `{`).
- **1 token:** all other constructs (`let`, `const`, `if`, `while`, `return`, `break`, `continue`, expression statements).

The grammar is LL(1) for all constructs except the two cases above. The parser implementation should use a 3-token lookahead buffer, which is trivially efficient.

- `let` and `const` are distinguished by keyword.
- `if` / `while` / `for` / `return` / `break` / `continue` are distinguished by keyword.
- Struct literals require the type name (IDENT) followed by `{`, which is distinguishable from a block by the preceding IDENT (2-token lookahead).
- The `for` loop has two forms; the parser distinguishes them by checking for `in` vs `:` after the first IDENT (3-token lookahead).

### 7.3 Reserved for future phases

The following constructs are **not** in this grammar but are reserved for later phases:

- **Pointers/references:** No `&`, `*`, or reference types. Memory management is compiler-internal.
- **Growable collections:** No `Vec`, `List`, or dynamic arrays. Only fixed-size arrays `[T; N]`.
- **Generics/templates:** No `<T>` syntax. Types are monomorphic.
- **Traits/interfaces:** No `trait` or `impl` blocks.
- **Closures/lambdas:** No anonymous functions.
- **Macros:** No macro system.
- **Unsafe blocks:** No `unsafe` keyword (reserved for Phase 2+).
- **Concurrency:** `thread`, `spawn`, `async`, `await`, `send`, `recv`, `mutex` are reserved keywords (§2.5) but have no grammar productions yet. Their syntax is defined in Phase 5 (see 0.7).
- **Future keywords:** `defer`, `yield`, `match`, `trait`, `impl` are reserved (§2.5) but have no grammar productions yet. They are activated in later phases; a program using them as an identifier is a compile error (0.9 §3.3).
- **Growable strings:** `string` is a primitive type; its mutability and growth semantics are defined in Phase 2 (0.2 semantics).

---

## 8. Open questions for review

See `docs/specs/2026-11-10-yulia-open-decisions.md` for owner + decision gate. Each item below is marked **resolved** (with the deciding document) or **open** (with its register id).

1. **String mutability:** Is `string` immutable (value type, copy-on-write) or mutable (in-place)? This affects the memory model and must be decided in 0.2. — **open**, D4 (Phase 2 gate).
2. **Array bounds checking:** Compile-time (when provable) + runtime trap, or always runtime? — **resolved**: runtime check in Phase 1; compile-time where provable is a Phase 2 optimization (0.2 §113, MEM-SAFE-2).
3. **Integer overflow:** Wraparound (C-style) or trap (Rust-style)? — **resolved**: trapping (D2).
4. **Default initialization:** Are variables zero-initialized by default, or must they be explicitly initialized? — **resolved**: no uninitialized reads; reading a `let` before assignment is a compile error (MEM-SAFE-4; see the note on `let` in §3.3).
5. **Module path separator:** Uses `.` (e.g., `std.io`). Is this consistent with file system paths? The build system must define the mapping. — **open**, decide with the build system (Phase 2).
6. **Stdlib function overloading:** Program 18 uses `io.println_int` because no overloading is defined. Should the stdlib use type-suffixed names (`println_int`, `println_str`) or support overloading? — **open**, D18 (post-MVP).
7. **Escape analysis integration:** The grammar has no reference/pointer syntax (by design). Escape analysis (0.3a) operates on compiler-internal references. The grammar does not need to change for escape analysis, but the type checker must track region membership. — **open**, D11 (Phase 2 gate).
---

## 9. Acceptance criteria for 0.1

- [x] EBNF grammar defined for all language constructs.
- [x] Lexical rules defined (identifiers, literals, operators, comments).
- [x] Module grammar included (declaration, imports, visibility, resolution, entry point).
- [x] ≥20 illustrative programs provided (24 programs included).
- [x] Ambiguity resolution documented.
- [x] Operator precedence table defined.
- [x] Open questions listed for 0.2.

## 10. Phase 2 extensions (reserved)

The following constructs are used by the Phase 2 deliverables (0.5 error/panic model, 0.6 FFI rules) and are reserved in the grammar. They are not part of the Phase 1 subset.

### 10.1 `panic` (built-in)

`panic` is a built-in function, not a user-definable name. It is reserved (§2.5).

```
panic_expr     ::= "panic" "(" [ expr ] ")"
```

`panic` is an expression (it never returns). See 0.5 §3.

### 10.2 `extern "C" fn` (FFI declaration)

```
extern_decl    ::= "extern" STRING_LIT "fn" IDENT "(" [ param_list ] ")" [ "->" return_type ] ";"
```

The string literal names the ABI (only `"C"` is supported in Phase 2). See 0.6 §2.1.

### 10.3 `export` (visibility modifier)

```
fn_decl        ::= [ "private" | "export" ] "fn" ...
```

`export` marks a function as callable from C. It is mutually exclusive with `private`. See 0.6 §2.2.

### 10.4 `pin` (pinning statement)

```
pin_stmt       ::= "pin" lvalue "as" IDENT block
```

The pinned value's region is kept alive for the duration of the block. See 0.6 §3.3 and 0.3a §4.3.

### 10.5 `CPtr` (opaque C pointer type)

```
type           ::= ... | "CPtr"
```

`CPtr` is opaque: it cannot be dereferenced directly. It is non-generic in Phase 2; generics are a Phase 5 extension. See 0.6 §3.2.

### 10.6 Deferred to Phase 5

- `#[layout(soa)]` attribute syntax (0.6 §4.3).
- Concurrency constructs: `thread::spawn`, `async`, `await` (0.7).
- Generics (`CPtr<T>`, `Result<T, E>`).

### 10.7 Multi-module programs and imports

`import_decl` and multi-module name resolution are Phase 2 (D1). The
`module <path>;` header is required in every program (§3.1, §4.5); `import`
is rejected in Phase 1.
---

*This document is a draft. It requires review sign-off before implementation begins (Phase 0 exit criteria).*
