//! Field access, indexing, assignment targets, and value addresses.

use inkwell::AddressSpace;
use inkwell::IntPredicate;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};

use crate::ast::{Expr, ExprKind};
use crate::typeck::Ty;

use crate::codegen::{CodegenError, builder_error};

use super::super::Lower;

impl<'ctx> Lower<'ctx, '_> {
    /// Loads a field.
    pub(crate) fn field(
        &mut self,
        base: &Expr,
        name: &str,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        let (ptr, ty) = self.field_pointer(base, name)?;
        self.builder
            .build_load(ty, ptr, name)
            .map_err(builder_error)
    }

    /// Loads an array element.
    pub(crate) fn index(
        &mut self,
        base: &Expr,
        index: &Expr,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        let (ptr, ty) = self.index_pointer(base, index)?;
        self.builder
            .build_load(ty, ptr, "index")
            .map_err(builder_error)
    }

    /// Returns a pointer to a field, reading through an rvalue base if needed.
    pub(crate) fn field_pointer(
        &mut self,
        base: &Expr,
        name: &str,
    ) -> Result<(PointerValue<'ctx>, BasicTypeEnum<'ctx>), CodegenError> {
        let Ty::Struct(struct_name) = self.expr_ty(base) else {
            return Err(CodegenError::new("field access on a non-struct"));
        };
        let (base_ptr, _) = self.pointer_to(base)?;
        let struct_ty = self.struct_type(&struct_name)?;
        let index = self.field_index(&struct_name, name)?;
        let ptr = self
            .builder
            .build_struct_gep(struct_ty, base_ptr, index, name)
            .map_err(builder_error)?;
        let field_ty = self.field_type(&struct_name, index)?;
        let llvm_ty = self.llvm_type(&field_ty)?;
        Ok((ptr, llvm_ty))
    }

    /// Returns a pointer to an array element, checking the bounds.
    pub(crate) fn index_pointer(
        &mut self,
        base: &Expr,
        index: &Expr,
    ) -> Result<(PointerValue<'ctx>, BasicTypeEnum<'ctx>), CodegenError> {
        let (elem_ty, len) = match self.expr_ty(base) {
            Ty::Array(elem, len) => ((*elem).clone(), len),
            _ => return Err(CodegenError::new("index on a non-array")),
        };
        let llvm_elem = self.llvm_type(&elem_ty)?;
        let (base_ptr, base_llvm) = self.pointer_to(base)?;
        if !matches!(base_llvm, BasicTypeEnum::ArrayType(_)) {
            return Err(CodegenError::new("index target is not an array"));
        }
        let index_value = self.expr(index)?.into_int_value();
        let function = self.function()?;
        let len_value = self.context.i32_type().const_int(len as u64, false);
        // Unsigned comparison: a negative index wraps to a huge value and is
        // therefore rejected as out of bounds.
        let in_bounds = self
            .builder
            .build_int_compare(IntPredicate::ULT, index_value, len_value, "inbounds")
            .map_err(builder_error)?;
        let ok = self.context.append_basic_block(function, "index.ok");
        let bad = self.context.append_basic_block(function, "index.bad");
        self.builder
            .build_conditional_branch(in_bounds, ok, bad)
            .map_err(builder_error)?;
        self.builder.position_at_end(bad);
        self.runtime_error("R0101", "array index out of bounds");
        self.builder.position_at_end(ok);

        let i64_ty = self.context.i64_type();
        let base_int = self
            .builder
            .build_ptr_to_int(base_ptr, i64_ty, "base")
            .map_err(builder_error)?;
        let index_int = self
            .builder
            .build_int_z_extend(index_value, i64_ty, "index")
            .map_err(builder_error)?;
        let size = llvm_elem
            .size_of()
            .ok_or_else(|| CodegenError::new("the element type has no size"))?;
        let offset = self
            .builder
            .build_int_mul(index_int, size, "offset")
            .map_err(builder_error)?;
        let elem_int = self
            .builder
            .build_int_add(base_int, offset, "elem")
            .map_err(builder_error)?;
        let ptr_ty = self.context.ptr_type(AddressSpace::default());
        let ptr = self
            .builder
            .build_int_to_ptr(elem_int, ptr_ty, "elemptr")
            .map_err(builder_error)?;
        Ok((ptr, llvm_elem))
    }

    /// Returns the address of an assignable expression.
    pub(crate) fn lvalue(
        &mut self,
        expr: &Expr,
    ) -> Result<(PointerValue<'ctx>, BasicTypeEnum<'ctx>), CodegenError> {
        match &expr.kind {
            ExprKind::Ident(name) => self
                .lookup(name)
                .ok_or_else(|| CodegenError::new(format!("`{name}` is not assignable"))),
            ExprKind::Field { base, name } => self.field_pointer(base, name),
            ExprKind::Index { base, index } => self.index_pointer(base, index),
            _ => Err(CodegenError::new("the assignment target is not an lvalue")),
        }
    }

    /// Returns a pointer to a value, spilling an rvalue base to the stack.
    pub(crate) fn pointer_to(
        &mut self,
        expr: &Expr,
    ) -> Result<(PointerValue<'ctx>, BasicTypeEnum<'ctx>), CodegenError> {
        match &expr.kind {
            ExprKind::Ident(_) | ExprKind::Field { .. } | ExprKind::Index { .. } => {
                self.lvalue(expr)
            }
            _ => {
                let ty = self.expr_ty(expr);
                let llvm_ty = self.llvm_type(&ty)?;
                let value = self.expr(expr)?;
                let ptr = self.alloca_entry(llvm_ty, "temp")?;
                self.builder
                    .build_store(ptr, value)
                    .map_err(builder_error)?;
                Ok((ptr, llvm_ty))
            }
        }
    }

    /// Returns the function currently being lowered.
    pub(crate) fn function(&self) -> Result<FunctionValue<'ctx>, CodegenError> {
        self.func
            .ok_or_else(|| CodegenError::new("no function is being lowered"))
    }
}
