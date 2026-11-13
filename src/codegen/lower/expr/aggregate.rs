//! Struct and array literals.

use inkwell::types::BasicType;
use inkwell::values::BasicValueEnum;

use crate::ast::{Expr, FieldInit};
use crate::typeck::Ty;

use crate::codegen::{CodegenError, array_len, builder_error};

use super::super::Lower;

impl<'ctx> Lower<'ctx, '_> {
    /// Lowers a struct literal.
    pub(crate) fn struct_literal(
        &mut self,
        name: &str,
        fields: &[FieldInit],
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        let struct_ty = self.struct_type(name)?;
        let sig = self
            .structs
            .get(name)
            .cloned()
            .ok_or_else(|| CodegenError::new(format!("unknown struct `{name}`")))?;
        let mut value = struct_ty.const_zero();
        for field in &sig.fields {
            let init = fields
                .iter()
                .find(|init| init.name == field.name)
                .ok_or_else(|| {
                    CodegenError::new(format!(
                        "`{name} {{ .. }}` is missing the field `{}`",
                        field.name
                    ))
                })?;
            let index = self.field_index(name, &field.name)?;
            let init_value = self.expr(&init.value)?;
            value = self
                .builder
                .build_insert_value(value, init_value, index, &field.name)
                .map_err(builder_error)?
                .into_struct_value();
        }
        Ok(value.into())
    }

    /// Lowers an array literal.
    pub(crate) fn array_literal(
        &mut self,
        expr: &Expr,
        elements: &[Expr],
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        let (elem_ty, len) = match self.expr_ty(expr) {
            Ty::Array(elem, len) => ((*elem).clone(), len),
            _ => {
                return Err(CodegenError::new(
                    "array literal has an unknown element type",
                ));
            }
        };
        if len == 0 || elements.is_empty() {
            return Err(CodegenError::new("arrays of length zero have no LLVM type"));
        }
        if elements.len() != len {
            return Err(CodegenError::new(
                "the array literal length does not match its type",
            ));
        }
        let llvm_elem = self.llvm_type(&elem_ty)?;
        let array_ty = llvm_elem.array_type(array_len(len)?);
        let mut value = array_ty.const_zero();
        for (index, element) in elements.iter().enumerate() {
            let element_value = self.expr(element)?;
            value = self
                .builder
                .build_insert_value(value, element_value, array_len(index)?, "elem")
                .map_err(builder_error)?
                .into_array_value();
        }
        Ok(value.into())
    }

    /// Lowers a `[value; count]` literal.
    pub(crate) fn array_repeat(
        &mut self,
        value: &Expr,
        count: &str,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        let len: usize = count
            .parse()
            .map_err(|_| CodegenError::new(format!("`{count}` is not a valid array length")))?;
        if len == 0 {
            return Err(CodegenError::new("arrays of length zero have no LLVM type"));
        }
        let elem_ty = self.expr_ty(value);
        let llvm_elem = self.llvm_type(&elem_ty)?;
        let repeated = self.expr(value)?;
        let array_ty = llvm_elem.array_type(array_len(len)?);
        let mut array = array_ty.const_zero();
        for index in 0..array_len(len)? {
            array = self
                .builder
                .build_insert_value(array, repeated, index, "repeat")
                .map_err(builder_error)?
                .into_array_value();
        }
        Ok(array.into())
    }
}
