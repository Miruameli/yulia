//! Source spans: byte offsets into the source text.

/// A half-open byte range `[start, end)` into the source text.
///
/// Spans are byte offsets, not character offsets: the lexer works on bytes and
/// the diagnostics layer converts them to `line:col` for display (D2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    /// The first byte of the span (inclusive).
    pub start: usize,
    /// One past the last byte of the span (exclusive).
    pub end: usize,
}

impl Span {
    /// Creates a span from `start` (inclusive) to `end` (exclusive).
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Returns the length of the span in bytes.
    #[must_use]
    pub const fn len(self) -> usize {
        self.end - self.start
    }

    /// Returns `true` if the span covers no bytes.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }

    /// Returns `true` if `offset` falls inside the span.
    #[must_use]
    pub const fn contains(self, offset: usize) -> bool {
        offset >= self.start && offset < self.end
    }
}
