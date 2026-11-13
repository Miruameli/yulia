//! The range-based and C-style `for` loops.

use inkwell::IntPredicate;

use crate::ast::{Block, Expr, Stmt};

use crate::codegen::{CodegenError, builder_error};

use super::super::Lower;

impl<'ctx> Lower<'ctx, '_> {
    /// Lowers `for var in start..end`.
    pub(crate) fn for_range(
        &mut self,
        var: &str,
        start: &Expr,
        end: &Expr,
        body: &Block,
    ) -> Result<(), CodegenError> {
        let function = self.function()?;
        let i32_ty = self.context.i32_type();
        let start_value = self.expr(start)?.into_int_value();
        let end_value = self.expr(end)?.into_int_value();
        let ptr = self.alloca_entry(i32_ty.into(), var)?;
        self.builder
            .build_store(ptr, start_value)
            .map_err(builder_error)?;

        self.push_scope();
        self.declare(var, ptr, i32_ty.into());
        let cond_block = self.context.append_basic_block(function, "for.cond");
        let body_block = self.context.append_basic_block(function, "for.body");
        let step_block = self.context.append_basic_block(function, "for.step");
        let done = self.context.append_basic_block(function, "for.done");
        self.builder
            .build_unconditional_branch(cond_block)
            .map_err(builder_error)?;

        self.builder.position_at_end(cond_block);
        let current = self.load_loop_var(ptr, var)?;
        let condition = self
            .builder
            .build_int_compare(IntPredicate::SLT, current, end_value, "for.lt")
            .map_err(builder_error)?;
        self.builder
            .build_conditional_branch(condition, body_block, done)
            .map_err(builder_error)?;

        self.builder.position_at_end(body_block);
        self.breaks.push(done);
        self.continues.push(step_block);
        self.block(body)?;
        self.breaks.pop();
        self.continues.pop();
        if !self.terminated() {
            self.builder
                .build_unconditional_branch(step_block)
                .map_err(builder_error)?;
        }

        self.builder.position_at_end(step_block);
        let current = self.load_loop_var(ptr, var)?;
        let step = self
            .builder
            .build_int_add(current, i32_ty.const_int(1, false), "for.next")
            .map_err(builder_error)?;
        self.builder.build_store(ptr, step).map_err(builder_error)?;
        self.builder
            .build_unconditional_branch(cond_block)
            .map_err(builder_error)?;

        self.builder.position_at_end(done);
        self.pop_scope();
        Ok(())
    }

    /// Lowers the C-style `for`.
    pub(crate) fn for_c_style(
        &mut self,
        init: &Stmt,
        cond: &Expr,
        update: &Expr,
        body: &Block,
    ) -> Result<(), CodegenError> {
        let function = self.function()?;
        self.push_scope();
        self.stmt(init)?;
        let cond_block = self.context.append_basic_block(function, "c_for.cond");
        let body_block = self.context.append_basic_block(function, "c_for.body");
        let step_block = self.context.append_basic_block(function, "c_for.step");
        let done = self.context.append_basic_block(function, "c_for.done");
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
        self.continues.push(step_block);
        self.block(body)?;
        self.breaks.pop();
        self.continues.pop();
        if !self.terminated() {
            self.builder
                .build_unconditional_branch(step_block)
                .map_err(builder_error)?;
        }

        self.builder.position_at_end(step_block);
        self.expr(update)?;
        self.builder
            .build_unconditional_branch(cond_block)
            .map_err(builder_error)?;

        self.builder.position_at_end(done);
        self.pop_scope();
        Ok(())
    }

    /// Loads the induction variable of a `for` loop.
    pub(crate) fn load_loop_var(
        &mut self,
        ptr: inkwell::values::PointerValue<'ctx>,
        name: &str,
    ) -> Result<inkwell::values::IntValue<'ctx>, CodegenError> {
        let value = self
            .builder
            .build_load(self.context.i32_type(), ptr, name)
            .map_err(builder_error)?;
        Ok(value.into_int_value())
    }
}
