//! Diagnostics: source spans, stable error codes, and compile-time reports.
//!
//! The compiler never panics at the user (D2). Every rejected program produces
//! one or more [`Diagnostic`] values, each carrying a stable [`Code`], a
//! human-readable message, and the [`Span`] it refers to. Diagnostics are
//! accumulated so a single run can report many problems.
//!
//! The authoritative language specification lives in `docs/specs/`.

pub mod code;
pub mod diagnostic;
pub mod location;
pub mod span;

pub use diagnostic::{Code, Diagnostic};
pub use location::{Location, locate};
pub use span::Span;
