//! Source locations: 1-based line and column for diagnostic display (0.5 §2.1).

/// A 1-based source position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    /// The 1-based line number.
    pub line: usize,
    /// The 1-based column number, counted in characters.
    pub column: usize,
}

/// Converts a byte `offset` in `source` to a 1-based [`Location`].
///
/// An offset past the end of `source` is clamped to its last byte, so a
/// diagnostic that points past the final newline still renders.
#[must_use]
pub fn locate(source: &str, offset: usize) -> Location {
    let offset = offset.min(source.len());
    let mut line = 1;
    let mut line_start = 0;
    for (index, byte) in source.bytes().enumerate() {
        if index >= offset {
            break;
        }
        if byte == b'\n' {
            line += 1;
            line_start = index + 1;
        }
    }
    Location {
        line,
        column: source[line_start..offset].chars().count() + 1,
    }
}
