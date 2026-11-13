//! Binary operations and short-circuit `&&` / `||`.

use inkwell::IntPredicate;
use inkwell::values::BasicValueEnum;

use crate::ast::{BinaryOp, Expr};

use crate::codegen::{CodegenError, builder_error};

use super::super::Lower;

impl<'ctx> Lower<'ctx, '_> {
    /// Lowers a binary operation.
    pub(crate) fn binary(
        &mut self,
        op: BinaryOp,
        lhs: &Expr,
        rhs: &Expr,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        match op {
            BinaryOp::And | BinaryOp::Or => self.logical(op, lhs, rhs),
            BinaryOp::Add => {
                let left = self.expr(lhs)?.into_int_value();
                let right = self.expr(rhs)?.into_int_value();
                self.overflowing("llvm.sadd.with.overflow.i32", left, right, "add")
            }
            BinaryOp::Sub => {
                let left = self.expr(lhs)?.into_int_value();
                let right = self.expr(rhs)?.into_int_value();
                self.overflowing("llvm.ssub.with.overflow.i32", left, right, "sub")
            }
            BinaryOp::Mul => {
                let left = self.expr(lhs)?.into_int_value();
                let right = self.expr(rhs)?.into_int_value();
                self.overflowing("llvm.smul.with.overflow.i32", left, right, "mul")
            }
            BinaryOp::Div => {
                let left = self.expr(lhs)?.into_int_value();
                let right = self.expr(rhs)?.into_int_value();
                self.checked_division(left, right, "div")
            }
            BinaryOp::Rem => {
                let left = self.expr(lhs)?.into_int_value();
                let right = self.expr(rhs)?.into_int_value();
                self.checked_remainder(left, right, "rem")
            }
            BinaryOp::Eq
            | BinaryOp::Ne
            | BinaryOp::Lt
            | BinaryOp::Gt
            | BinaryOp::Le
            | BinaryOp::Ge => {
                let left = self.expr(lhs)?.into_int_value();
                let right = self.expr(rhs)?.into_int_value();
                let predicate = match op {
                    BinaryOp::Eq => IntPredicate::EQ,
                    BinaryOp::Ne => IntPredicate::NE,
                    BinaryOp::Lt => IntPredicate::SLT,
                    BinaryOp::Gt => IntPredicate::SGT,
                    BinaryOp::Le => IntPredicate::SLE,
                    _ => IntPredicate::SGE,
                };
                let name = match op {
                    BinaryOp::Eq => "eq",
                    BinaryOp::Ne => "ne",
                    BinaryOp::Lt => "lt",
                    BinaryOp::Gt => "gt",
                    BinaryOp::Le => "le",
                    _ => "ge",
                };
                let result = self
                    .builder
                    .build_int_compare(predicate, left, right, name)
                    .map_err(builder_error)?;
                Ok(result.into())
            }
        }
    }

    /// Lowers `&&` / `||` with short-circuit evaluation.
    pub(crate) fn logical(
        &mut self,
        op: BinaryOp,
        lhs: &Expr,
        rhs: &Expr,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        let function = self.function()?;
        let left = self.expr(lhs)?.into_int_value();
        let lhs_block = self
            .builder
            .get_insert_block()
            .ok_or_else(|| CodegenError::new("no block to branch from"))?;
        let rhs_block = self.context.append_basic_block(function, "logic.rhs");
        let merge = self.context.append_basic_block(function, "logic.merge");
        // `&&` skips the right operand on false; `||` on true.
        let condition = if matches!(op, BinaryOp::And) {
            self.builder
                .build_conditional_branch(left, rhs_block, merge)
        } else {
            self.builder
                .build_conditional_branch(left, merge, rhs_block)
        };
        condition.map_err(builder_error)?;

        self.builder.position_at_end(rhs_block);
        let right = self.expr(rhs)?.into_int_value();
        let rhs_end = self
            .builder
            .get_insert_block()
            .ok_or_else(|| CodegenError::new("no block to branch from"))?;
        self.builder
            .build_unconditional_branch(merge)
            .map_err(builder_error)?;

        self.builder.position_at_end(merge);
        let phi = self
            .builder
            .build_phi(self.context.bool_type(), "logic")
            .map_err(builder_error)?;
        let short = self
            .context
            .bool_type()
            .const_int(u64::from(matches!(op, BinaryOp::Or)), false);
        phi.add_incoming(&[(&short, lhs_block), (&right, rhs_end)]);
        Ok(phi.as_basic_value())
    }
}
