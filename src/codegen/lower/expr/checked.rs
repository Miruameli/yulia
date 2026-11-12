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

    /// Lowers `/`, checking for a zero divisor.
    pub(crate) fn checked_division(
        &mut self,
        left: IntValue<'ctx>,
        right: IntValue<'ctx>,
        name: &str,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        let function = self.function()?;
        let zero = right.get_type().const_zero();
        let is_zero = self
            .builder
            .build_int_compare(IntPredicate::EQ, right, zero, "iszero")
            .map_err(builder_error)?;
        let ok = self.context.append_basic_block(function, "div.ok");
        let bad = self.context.append_basic_block(function, "div.bad");
        self.builder
            .build_conditional_branch(is_zero, bad, ok)
            .map_err(builder_error)?;
        self.builder.position_at_end(bad);
        self.runtime_error("R0102", "division by zero");
        self.builder.position_at_end(ok);
        let value = self
            .builder
            .build_int_signed_div(left, right, name)
            .map_err(builder_error)?;
        Ok(value.into())
    }

    /// Lowers `%`, checking for a zero divisor.
    pub(crate) fn checked_remainder(
        &mut self,
        left: IntValue<'ctx>,
        right: IntValue<'ctx>,
        name: &str,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        let function = self.function()?;
        let zero = right.get_type().const_zero();
        let is_zero = self
            .builder
            .build_int_compare(IntPredicate::EQ, right, zero, "iszero")
            .map_err(builder_error)?;
        let ok = self.context.append_basic_block(function, "rem.ok");
        let bad = self.context.append_basic_block(function, "rem.bad");
        self.builder
            .build_conditional_branch(is_zero, bad, ok)
            .map_err(builder_error)?;
        self.builder.position_at_end(bad);
        self.runtime_error("R0102", "modulo by zero");
        self.builder.position_at_end(ok);
        let value = self
            .builder
            .build_int_signed_rem(left, right, name)
            .map_err(builder_error)?;
        Ok(value.into())
    }
}
