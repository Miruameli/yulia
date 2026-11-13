//! `if` / `else` and `while` lowering.

use crate::ast::{Block, ElseBranch, Expr};

use crate::codegen::{CodegenError, builder_error};

use super::super::Lower;

impl Lower<'_, '_> {
    /// Lowers `if` / `else if` / `else`.
    pub(crate) fn if_stmt(
        &mut self,
        cond: &Expr,
        then: &Block,
        else_branch: Option<&ElseBranch>,
    ) -> Result<(), CodegenError> {
        let function = self.function()?;
        let condition = self.expr(cond)?.into_int_value();
        let then_block = self.context.append_basic_block(function, "if.then");
        let else_block = self.context.append_basic_block(function, "if.else");
        let merge = self.context.append_basic_block(function, "if.merge");
        self.builder
            .build_conditional_branch(condition, then_block, else_block)
            .map_err(builder_error)?;

        self.builder.position_at_end(then_block);
        self.block(then)?;
        if !self.terminated() {
            self.builder
                .build_unconditional_branch(merge)
                .map_err(builder_error)?;
        }

        self.builder.position_at_end(else_block);
        match else_branch {
            Some(ElseBranch::Block(block)) => self.block(block)?,
            Some(ElseBranch::If(stmt)) => self.stmt(stmt)?,
            None => {}
        }
        if !self.terminated() {
            self.builder
                .build_unconditional_branch(merge)
                .map_err(builder_error)?;
        }

        self.builder.position_at_end(merge);
        Ok(())
    }

    /// Lowers `while`.
    pub(crate) fn while_stmt(&mut self, cond: &Expr, body: &Block) -> Result<(), CodegenError> {
        let function = self.function()?;
        let cond_block = self.context.append_basic_block(function, "while.cond");
        let body_block = self.context.append_basic_block(function, "while.body");
        let done = self.context.append_basic_block(function, "while.done");
        self.builder
            .build_unconditional_branch(cond_block)
            .map_err(builder_error)?;

        self.builder.position_at_end(cond_block);
        let condition = self.expr(cond)?.into_int_value();
        self.builder
            .build_conditional_branch(condition, body_block, done)
            .map_err(builder_error)?;

        self.builder.position_at_end(body_block);
        self.breaks.push(done);
        self.continues.push(cond_block);
        self.block(body)?;
        self.breaks.pop();
        self.continues.pop();
        if !self.terminated() {
            self.builder
                .build_unconditional_branch(cond_block)
                .map_err(builder_error)?;
        }

        self.builder.position_at_end(done);
        Ok(())
    }
}
