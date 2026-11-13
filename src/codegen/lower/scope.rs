//! Stack slots, name scopes, and the runtime-error path.

use std::collections::HashMap;

use inkwell::AddressSpace;
use inkwell::module::Linkage;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{FunctionValue, PointerValue};

use crate::codegen::{CodegenError, builder_error};

use super::Lower;

impl<'ctx> Lower<'ctx, '_> {
    /// Allocates `ty` in the function's entry block (0.3 §3).
    ///
    /// The alloca is inserted before the entry block's first instruction so it
    /// stays valid even when the declaration is reached from a later block.
    pub(crate) fn alloca_entry(
        &mut self,
        ty: BasicTypeEnum<'ctx>,
        name: &str,
    ) -> Result<PointerValue<'ctx>, CodegenError> {
        let function = self
            .func
            .ok_or_else(|| CodegenError::new("no function is being lowered"))?;
        let entry = function
            .get_first_basic_block()
            .ok_or_else(|| CodegenError::new("function has no entry block"))?;
        let current = self.builder.get_insert_block();
        match entry.get_first_instruction() {
            Some(first) => self.builder.position_at(entry, &first),
            None => self.builder.position_at_end(entry),
        }
        let ptr = self.builder.build_alloca(ty, name).map_err(builder_error)?;
        if let Some(block) = current {
            self.builder.position_at_end(block);
        }
        Ok(ptr)
    }

    /// Binds `name` to `ptr` in the innermost scope.
    pub(crate) fn declare(&mut self, name: &str, ptr: PointerValue<'ctx>, ty: BasicTypeEnum<'ctx>) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), (ptr, ty));
        }
    }

    /// Looks up `name` from the innermost scope outwards.
    pub(crate) fn lookup(&self, name: &str) -> Option<(PointerValue<'ctx>, BasicTypeEnum<'ctx>)> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).copied())
    }

    pub(crate) fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub(crate) fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    /// Returns `true` when the builder sits in a terminated block.
    pub(crate) fn terminated(&self) -> bool {
        self.builder
            .get_insert_block()
            .is_some_and(|block| block.get_terminator().is_some())
    }

    /// Emits a runtime error: the message on stderr, then `abort()` (0.5 §2.1).
    ///
    /// `generate` receives no source text, so the message carries the stable
    /// code but not the `<file>:<line>:<col>` prefix of 0.5 §2.1.
    pub(crate) fn runtime_error(&mut self, code: &str, message: &str) {
        let text = format!("runtime error[{code}]: {message}\n");
        if let Ok(rope) = self.builder.build_global_string_ptr(&text, ".rt") {
            let ptr = rope.as_pointer_value();
            let write = self.declare_write();
            let fd = self.context.i32_type().const_int(2, false);
            let len = self.context.i64_type().const_int(text.len() as u64, false);
            let _ = self
                .builder
                .build_call(write, &[fd.into(), ptr.into(), len.into()], "write");
        }
        let abort = self.declare_abort();
        let _ = self.builder.build_call(abort, &[], "abort");
        let _ = self.builder.build_unreachable();
    }

    /// Returns the `write(2, ..)` declaration, creating it on first use.
    pub(crate) fn declare_write(&mut self) -> FunctionValue<'ctx> {
        if let Some(function) = self.module.get_function("write") {
            return function;
        }
        let i32_ty = self.context.i32_type();
        let i64_ty = self.context.i64_type();
        let ptr_ty = self.context.ptr_type(AddressSpace::default());
        let ty = i64_ty.fn_type(&[i32_ty.into(), ptr_ty.into(), i64_ty.into()], false);
        self.module
            .add_function("write", ty, Some(Linkage::External))
    }

    /// Returns the `abort()` declaration, creating it on first use.
    pub(crate) fn declare_abort(&mut self) -> FunctionValue<'ctx> {
        if let Some(function) = self.module.get_function("abort") {
            return function;
        }
        let ty = self.context.void_type().fn_type(&[], false);
        self.module
            .add_function("abort", ty, Some(Linkage::External))
    }
}
