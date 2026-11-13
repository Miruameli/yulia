//! LLVM type mapping (0.6 §4.4, 0.1 §3.2).
//!
//! `int` is `i32`, `bool` is `i1`, structs are C-layout structs (`AoS`,
//! unpacked), and `[T; N]` is an LLVM array of `N` elements.
use inkwell::types::{BasicType, BasicTypeEnum, StructType};

use crate::typeck::Ty;

use super::{CodegenError, Lower, array_len};

impl<'ctx> Lower<'ctx, '_> {
    /// Maps a resolved [`Ty`] to its LLVM type.
    pub(crate) fn llvm_type(&mut self, ty: &Ty) -> Result<BasicTypeEnum<'ctx>, CodegenError> {
        match ty {
            Ty::Int => Ok(self.context.i32_type().into()),
            Ty::Bool => Ok(self.context.bool_type().into()),
            Ty::Void => Err(CodegenError::new("`void` is not a value type")),
            Ty::Array(elem, len) => {
                if *len == 0 {
                    return Err(CodegenError::new("arrays of length zero have no LLVM type"));
                }
                let elem = self.llvm_type(elem)?;
                Ok(elem.array_type(array_len(*len)?).into())
            }
            Ty::Struct(name) => Ok(self.struct_type(name)?.into()),
            Ty::Error => Err(CodegenError::new("cannot lower a value of unknown type")),
        }
    }

    /// Returns the named struct type, building its body on first use.
    pub(crate) fn struct_type(&mut self, name: &str) -> Result<StructType<'ctx>, CodegenError> {
        if let Some(ty) = self.struct_types.get(name) {
            return Ok(*ty);
        }
        if self.building.contains(name) {
            return Err(CodegenError::new(format!(
                "`{name}` is recursive and has no size"
            )));
        }
        let sig = self
            .structs
            .get(name)
            .cloned()
            .ok_or_else(|| CodegenError::new(format!("unknown struct `{name}`")))?;
        let ty = self.context.opaque_struct_type(name);
        self.struct_types.insert(name.to_string(), ty);
        self.building.insert(name.to_string());
        let mut fields = Vec::with_capacity(sig.fields.len());
        for field in &sig.fields {
            fields.push(self.llvm_type(&field.ty)?);
        }
        self.building.remove(name);
        ty.set_body(&fields, false);
        Ok(ty)
    }

    /// Returns the index of field `name` in `struct_name`.
    pub(crate) fn field_index(&self, struct_name: &str, name: &str) -> Result<u32, CodegenError> {
        let sig = self
            .structs
            .get(struct_name)
            .ok_or_else(|| CodegenError::new(format!("unknown struct `{struct_name}`")))?;
        let index = sig
            .fields
            .iter()
            .position(|field| field.name == name)
            .ok_or_else(|| CodegenError::new(format!("`{struct_name}` has no field `{name}`")))?;
        array_len(index)
    }

    /// Returns the declared type of field `index` in `struct_name`.
    pub(crate) fn field_type(&self, struct_name: &str, index: u32) -> Result<Ty, CodegenError> {
        let sig = self
            .structs
            .get(struct_name)
            .ok_or_else(|| CodegenError::new(format!("unknown struct `{struct_name}`")))?;
        sig.fields
            .get(index as usize)
            .map(|field| field.ty.clone())
            .ok_or_else(|| {
                CodegenError::new(format!("`{struct_name}` has no field at index {index}"))
            })
    }
}
