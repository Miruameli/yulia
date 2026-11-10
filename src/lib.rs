//! Yulia compiler library.
//!
//! Yulia is a procedural programming language with explicit types and a
//! compiler-inferred, region-based memory model: no `malloc`/`free`, no garbage
//! collector, no lifetime annotations, and deterministic reclamation.
//!
//! The compiler is a pipeline — source text flows through a lexer, a parser, a
//! type checker, a region-inference pass, and finally a code generator. Each
//! stage owns its own module and its own error type; diagnostics are
//! accumulated rather than thrown away so a single run can report many problems.
//!
//! The authoritative language specification lives in `docs/specs/`.
//!
//! # Status
//!
//! Phase 1 (lexer, parser, types, regions, codegen) is being implemented. See
//! `CONTRIBUTING.md` for the build, lint, and review workflow.

/// The compiler version, taken from `Cargo.toml` at compile time.
///
/// Yulia follows `SemVer` (`docs/specs/2026-11-09-yulia-versioning-policy.md`);
/// 0.x carries no stability guarantee.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The crate's MPL-2.0 license identifier, as recorded in `Cargo.toml`.
pub const LICENSE: &str = env!("CARGO_PKG_LICENSE");
