//! Decoding of literal escapes (0.1 §2.6).
//!
//! The lexer keeps literal text verbatim so the parser can report the exact
//! source; this module turns that text into the value it denotes.

/// Decodes a character literal's source text, quotes included.
#[must_use]
pub(crate) fn decode_char(text: &str) -> char {
    decode_string(text).chars().next().unwrap_or('\0')
}

/// Decodes a string literal's source text, quotes included.
#[must_use]
pub(crate) fn decode_string(text: &str) -> String {
    let inner = unquote(text);
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        let Some(escape) = chars.next() else { break };
        match escape {
            'n' => out.push('\n'),
            't' => out.push('\t'),
            'r' => out.push('\r'),
            '0' => out.push('\0'),
            '\\' => out.push('\\'),
            '"' => out.push('"'),
            '\'' => out.push('\''),
            other => out.push(other),
        }
    }
    out
}

/// Strips the surrounding quotes, tolerating unterminated literals.
fn unquote(text: &str) -> &str {
    let end = text.len().saturating_sub(1);
    text.get(1..end).unwrap_or("")
}
