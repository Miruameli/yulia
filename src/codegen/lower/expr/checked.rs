//! Arithmetic that reports a runtime error: overflow, division by zero.

use inkwell::IntPredicate;
use inkwell::values::{BasicValueEnum, IntValue, ValueKind};

use crate::codegen::{CodegenError, builder_error};

use super::super::Lower;

impl<'ctx> Lower<'ctx, '_> {
    /// Lowers `+`, `-`, `*` through the overflow-reporting intrinsics.
    pub(crate) fn overflowing(
        &mut self,
        intrinsic: &str,
        left: IntValue<'ctx>,
        right: IntValue<'ctx>,
        name: &str,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        let function = self.function()?;
        let Some(found) = inkwell::intrinsics::Intrinsic::find(intrinsic) else {
            return Err(CodegenError::new(format!(
                "missing LLVM intrinsic `{intrinsic}`"
            )));
        };
        let i32_ty = self.context.i32_type();
        let declaration = found
            .get_declaration(&self.module, &[i32_ty.into()])
            .ok_or_else(|| {
                CodegenError::new(format!("cannot declare LLVM intrinsic `{intrinsic}`"))
            })?;
        let call = self
            .builder
            .build_call(declaration, &[left.into(), right.into()], name)
            .map_err(builder_error)?;
        let ValueKind::Basic(pair) = call.try_as_basic_value() else {
            return Err(CodegenError::new(
                "the overflow intrinsic did not produce a value",
            ));
        };
        let sum = self
            .builder
            .build_extract_value(pair.into_struct_value(), 0, name)
            .map_err(builder_error)?;
        let flag = self
            .builder
            .build_extract_value(pair.into_struct_value(), 1, "overflow")
            .map_err(builder_error)?;
        let overflow = self
            .builder
            .build_int_compare(
                IntPredicate::EQ,
                flag.into_int_value(),
                self.context.bool_type().const_int(1, false),
                "overflowed",
            )
            .map_err(builder_error)?;
        let bad = self.context.append_basic_block(function, "overflow.bad");
        let ok = self.context.append_basic_block(function, "overflow.ok");
        self.builder
            .build_conditional_branch(overflow, bad, ok)
            .map_err(builder_error)?;
        self.builder.position_at_end(bad);
        self.runtime_error("R0103", "integer overflow");
        self.builder.position_at_end(ok);
        Ok(sum)
    }

    /// Lowers `/`, checking for a zero divisor and the `INT_MIN / -1` poison case.
    pub(crate) fn checked_division(
        &mut self,
        left: IntValue<'ctx>,
        right: IntValue<'ctx>,
        name: &str,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        self.checked_divisor(left, right, "div", "division by zero")?;
        let value = self
            .builder
            .build_int_signed_div(left, right, name)
            .map_err(builder_error)?;
        Ok(value.into())
    }

    /// Lowers `%`, checking for a zero divisor and the `INT_MIN % -1` poison case.
    pub(crate) fn checked_remainder(
        &mut self,
        left: IntValue<'ctx>,
        right: IntValue<'ctx>,
        name: &str,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        self.checked_divisor(left, right, "rem", "modulo by zero")?;
        let value = self
            .builder
            .build_int_signed_rem(left, right, name)
            .map_err(builder_error)?;
        Ok(value.into())
    }

    /// Emits the divisor guard shared by `/` and `%`, leaving the builder
    /// positioned at the block the operation itself is lowered into.
    ///
    /// Two cases abort: a zero divisor (R0102) and `INT_MIN` over `-1` (R0103).
    /// LLVM `sdiv`/`srem` yield **poison** for `INT_MIN` with a `-1` divisor, so
    /// without the second guard the optimizer could miscompile a well-typed
    /// program instead of producing the runtime error the language promises.
    fn checked_divisor(
        &mut self,
        left: IntValue<'ctx>,
        right: IntValue<'ctx>,
        stem: &str,
        zero_message: &str,
    ) -> Result<(), CodegenError> {
        let function = self.function()?;
        let i32_ty = self.context.i32_type();
        let zero = right.get_type().const_zero();
        let int_min = i32_ty.const_int(0x8000_0000, true);
        let minus_one = i32_ty.const_int(0xFFFF_FFFF, true);
        let is_zero = self
            .builder
            .build_int_compare(IntPredicate::EQ, right, zero, "iszero")
            .map_err(builder_error)?;
        let is_min = self
            .builder
            .build_int_compare(IntPredicate::EQ, left, int_min, "ismin")
            .map_err(builder_error)?;
        let is_minus_one = self
            .builder
            .build_int_compare(IntPredicate::EQ, right, minus_one, "isnegone")
            .map_err(builder_error)?;
        let overflows = self
            .builder
            .build_and(is_min, is_minus_one, "overflows")
            .map_err(builder_error)?;
        let zero_block = self
            .context
            .append_basic_block(function, &format!("{stem}.zero"));
        let check_block = self
            .context
            .append_basic_block(function, &format!("{stem}.check"));
        let overflow_block = self
            .context
            .append_basic_block(function, &format!("{stem}.overflow"));
        let ok = self
            .context
            .append_basic_block(function, &format!("{stem}.ok"));
        self.builder
            .build_conditional_branch(is_zero, zero_block, check_block)
            .map_err(builder_error)?;
        self.builder.position_at_end(check_block);
        self.builder
            .build_conditional_branch(overflows, overflow_block, ok)
            .map_err(builder_error)?;
        self.builder.position_at_end(zero_block);
        self.runtime_error("R0102", zero_message);
        self.builder.position_at_end(overflow_block);
        self.runtime_error("R0103", "integer overflow");
        self.builder.position_at_end(ok);
        Ok(())
    }
}
