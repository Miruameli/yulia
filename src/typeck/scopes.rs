//! The checker's scope stack and definite-assignment state.

use std::collections::HashMap;

use crate::typeck::{Checker, VarState};

impl Checker<'_> {
    /// Looks up a local variable.
    pub(crate) fn local(&self, name: &str) -> Option<VarState> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).cloned())
    }

    /// Declares a local variable in the innermost scope.
    pub(crate) fn declare(&mut self, name: String, ty: crate::typeck::Ty, init: bool) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, VarState { ty, init });
        }
    }

    /// Marks a local variable as definitely assigned.
    pub(crate) fn mark_init(&mut self, name: &str) {
        if let Some(scope) = self
            .scopes
            .iter_mut()
            .rev()
            .find(|scope| scope.contains_key(name))
            && let Some(var) = scope.get_mut(name)
        {
            var.init = true;
        }
    }

    /// Enters a new scope.
    pub(crate) fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Leaves the innermost scope.
    pub(crate) fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    /// Returns a copy of the current scope stack.
    pub(crate) fn snapshot(&self) -> Vec<HashMap<String, VarState>> {
        self.scopes.clone()
    }

    /// Replaces the scope stack.
    pub(crate) fn restore(&mut self, scopes: Vec<HashMap<String, VarState>>) {
        self.scopes = scopes;
    }

    /// Merges the two branches of an `if`.
    ///
    /// A variable is definitely assigned after the `if` only when it is
    /// definitely assigned on both paths.
    pub(crate) fn merge_if(
        &mut self,
        base: &[HashMap<String, VarState>],
        left: &[HashMap<String, VarState>],
        right: &[HashMap<String, VarState>],
    ) {
        let mut merged = base.to_owned();
        for (depth, scope) in merged.iter_mut().enumerate() {
            for (name, var) in scope.iter_mut() {
                let a = left
                    .get(depth)
                    .and_then(|s| s.get(name))
                    .is_some_and(|v| v.init);
                let b = right
                    .get(depth)
                    .and_then(|s| s.get(name))
                    .is_some_and(|v| v.init);
                var.init = var.init || (a && b);
            }
        }
        self.scopes = merged;
    }
}
