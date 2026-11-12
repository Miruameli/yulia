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

use crate::ast::{Block, DeclKind, ElseBranch, Program, Stmt, StmtKind};

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

impl Assignment {
    /// Attributes every binding of `program` to its enclosing function's region.
    #[must_use]
    pub fn new(program: &Program) -> Self {
        let mut assignment = Self::default();
        for decl in &program.decls {
            if let DeclKind::Fn(decl) = &decl.kind {
                let region = assignment.new_function(&decl.name);
                for param in &decl.params {
                    assignment.push_binding(&decl.name, &param.name, region);
                }
                assignment.walk_block(&decl.name, region, &decl.body);
            }
        }
        assignment
    }

    fn new_function(&mut self, name: &str) -> Region {
        let region = Region {
            index: self.functions.len(),
        };
        self.functions.push(FunctionRegion {
            name: name.to_string(),
            region,
        });
        region
    }

    fn push_binding(&mut self, function: &str, name: &str, region: Region) {
        self.bindings.push(BindingRegion {
            function: function.to_string(),
            name: name.to_string(),
            region,
        });
    }

    fn walk_block(&mut self, function: &str, region: Region, block: &Block) {
        for stmt in &block.stmts {
            self.walk_stmt(function, region, stmt);
        }
    }

    fn walk_stmt(&mut self, function: &str, region: Region, stmt: &Stmt) {
        match &stmt.kind {
            StmtKind::Let { name, .. } | StmtKind::Const { name, .. } => {
                self.push_binding(function, name, region);
            }
            StmtKind::Pin { alias, body, .. } => {
                // `pin` is outside the Phase 1 subset (D1), so a valid program
                // never reaches here; the alias is still a local of the
                // enclosing function.
                self.push_binding(function, alias, region);
                self.walk_block(function, region, body);
            }
            StmtKind::If {
                then, else_branch, ..
            } => {
                self.walk_block(function, region, then);
                match else_branch {
                    Some(ElseBranch::If(stmt)) => self.walk_stmt(function, region, stmt),
                    Some(ElseBranch::Block(block)) => {
                        self.walk_block(function, region, block);
                    }
                    None => {}
                }
            }
            StmtKind::While { body, .. } => self.walk_block(function, region, body),
            StmtKind::ForRange { var, body, .. } => {
                self.push_binding(function, var, region);
                self.walk_block(function, region, body);
            }
            StmtKind::ForCStyle { init, body, .. } => {
                self.walk_stmt(function, region, init);
                self.walk_block(function, region, body);
            }
            StmtKind::Return(_) | StmtKind::Break | StmtKind::Continue | StmtKind::Expr(_) => {}
        }
    }

    /// The region of `function`.
    #[must_use]
    pub fn function_region(&self, function: &str) -> Option<Region> {
        self.functions
            .iter()
            .find(|entry| entry.name == function)
            .map(|entry| entry.region)
    }

    /// The region of `name` as bound in `function`.
    #[must_use]
    pub fn binding_region(&self, function: &str, name: &str) -> Option<Region> {
        self.bindings
            .iter()
            .find(|entry| entry.function == function && entry.name == name)
            .map(|entry| entry.region)
    }

    /// The number of regions, which equals the number of functions.
    #[must_use]
    pub fn region_count(&self) -> usize {
        self.functions.len()
    }

    /// The names bound in `region`.
    #[must_use]
    pub fn bindings_in(&self, region: Region) -> Vec<&str> {
        self.bindings
            .iter()
            .filter(|entry| entry.region == region)
            .map(|entry| entry.name.as_str())
            .collect()
    }
}
