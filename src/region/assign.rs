//! The region inference walk (0.3 §3).
//!
//! One region per function; every binding of a function is attributed to it.
//! The walk is a single lexical pass over the AST.

use crate::ast::{Block, DeclKind, ElseBranch, Program, Stmt, StmtKind};

use super::{Assignment, BindingRegion, FunctionRegion, Region};

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
