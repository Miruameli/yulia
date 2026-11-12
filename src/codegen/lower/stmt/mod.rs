//! Statement lowering: blocks, declarations, returns, and loop exits.

use inkwell::basic_block::BasicBlock;

use crate::ast::{Block, Expr, Stmt, StmtKind, Type};

use crate::codegen::{CodegenError, builder_error};

use super::super::Lower;

mod control;
mod for_stmt;

impl<'ctx> Lower<'ctx, '_> {
    /// Lowers a block, opening a fresh scope.
    pub(crate) fn block(&mut self, block: &Block) -> Result<(), CodegenError> {
        self.push_scope();
        for stmt in &block.stmts {
            // A terminated block makes the rest unreachable; the checker
            // rejects such programs, so this only keeps the IR well-formed.
            if self.terminated() {
                break;
            }
            self.stmt(stmt)?;
        }
        self.pop_scope();
        Ok(())
    }

    /// Lowers one statement.
    pub(crate) fn stmt(&mut self, stmt: &Stmt) -> Result<(), CodegenError> {
        match &stmt.kind {
            StmtKind::Let { name, ty, init } => self.let_stmt(name, ty, init.as_ref()),
            StmtKind::Const { name, ty, value } => self.let_stmt(name, ty, Some(value)),
            StmtKind::Pin { body, .. } => self.block(body),
            StmtKind::If {
                cond,
                then,
                else_branch,
            } => self.if_stmt(cond, then, else_branch.as_ref()),
            StmtKind::While { cond, body } => self.while_stmt(cond, body),
            StmtKind::ForRange {
                var,
                start,
                end,
                body,
            } => self.for_range(var, start, end, body),
            StmtKind::ForCStyle {
                init,
                cond,
                update,
                body,
            } => self.for_c_style(init, cond, update, body),
            StmtKind::Return(expr) => self.return_stmt(expr.as_ref()),
            StmtKind::Break => self.branch(self.breaks.last().copied(), "`break` outside a loop"),
            StmtKind::Continue => {
                self.branch(self.continues.last().copied(), "`continue` outside a loop")
            }
            StmtKind::Expr(expr) => {
                self.expr(expr)?;
                Ok(())
            }
        }
    }

    /// Lowers `let` / local `const`, allocating one slot in the entry block.
    pub(crate) fn let_stmt(
        &mut self,
        name: &str,
        ty: &Type,
        init: Option<&Expr>,
    ) -> Result<(), CodegenError> {
        let resolved = self.annotation(&ty.span);
        let llvm_ty = self.llvm_type(&resolved)?;
        let ptr = self.alloca_entry(llvm_ty, name)?;
        if let Some(init) = init {
            let value = self.expr(init)?;
            if value.get_type() != llvm_ty {
                return Err(CodegenError::new(format!(
                    "cannot initialize `{name}` with this value"
                )));
            }
            self.builder
                .build_store(ptr, value)
                .map_err(builder_error)?;
        }
        self.declare(name, ptr, llvm_ty);
        Ok(())
    }

    /// Lowers `return`.
    pub(crate) fn return_stmt(&mut self, expr: Option<&Expr>) -> Result<(), CodegenError> {
        match expr {
            Some(expr) => {
                let value = self.expr(expr)?;
                self.builder
                    .build_return(Some(&value))
                    .map_err(builder_error)?;
            }
            None => {
                self.builder.build_return(None).map_err(builder_error)?;
            }
        }
        Ok(())
    }

    /// Branches to a loop exit, or reports a stray `break` / `continue`.
    pub(crate) fn branch(
        &mut self,
        target: Option<BasicBlock<'ctx>>,
        message: &str,
    ) -> Result<(), CodegenError> {
        let target = target.ok_or_else(|| CodegenError::new(message))?;
        self.builder
            .build_unconditional_branch(target)
            .map_err(builder_error)?;
        Ok(())
    }
}
