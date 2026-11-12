//! Region inference (0.3 §3).
//!
//! Phase 1 has no pointers or references: every value is passed by value, so
//! each local's region is its enclosing function's region, and each call gets a
//! fresh region that is reclaimed on return. The pass is therefore a single
//! lexical walk that names one region per function and attributes every binding
//! to it.
//!
//! Rule 3.1's interprocedural mutation proof — the pass-by-value optimization —
//! is a separate obligation and is not part of this walk. The hard part of the
//! model is deferred to Phase 2 (0.3 §4).

mod assign;

/// A function-bound allocation arena (0.3 §2.1).
///
/// A region is created on function entry and reclaimed on return, so a region
/// is identified by the function that owns it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Region {
    index: usize,
}

impl Region {
    /// The region's position in the program's region list.
    #[must_use]
    pub fn index(self) -> usize {
        self.index
    }
}

/// The region assignment for one program.
#[derive(Debug, Default)]
pub struct Assignment {
    functions: Vec<FunctionRegion>,
    bindings: Vec<BindingRegion>,
}

/// A function and the region its locals live in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionRegion {
    /// The function name.
    pub name: String,
    /// The region created on entry to the function.
    pub region: Region,
}

/// A binding and the region its value lives in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingRegion {
    /// The function the binding belongs to.
    pub function: String,
    /// The bound name.
    pub name: String,
    /// The region the value lives in.
    pub region: Region,
}
