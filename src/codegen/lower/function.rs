//! Function and top-level constant lowering.

use std::rc::Rc;

use crate::ast::{ConstDecl, DeclKind, FnDecl};
use crate::diagnostics::Span;
use crate::typeck::{FieldSig, FnSig, StructSig, Ty};

use crate::codegen::{CodegenError, array_len, builder_error};

use super::Lower;

impl Lower<'_, '_> {
    /// Lowers a top-level constant to a global.
    pub(crate) fn lower_const(&mut self, decl: &ConstDecl) -> Result<(), CodegenError> {
        let ty = self.annotation(&decl.ty.span);
        let llvm_ty = self.llvm_type(&ty)?;
        let value = self.expr(&decl.value)?;
        if value.get_type() != llvm_ty {
            return Err(CodegenError::new(format!(
                "constant `{}` has the wrong type",
                decl.name
            )));
        }
        let global = self.module.add_global(llvm_ty, None, &decl.name);
        global.set_initializer(&value);
        self.consts.insert(decl.name.clone(), value);
        Ok(())
    }

    /// Lowers one function body.
    pub(crate) fn lower_function(&mut self, decl: &FnDecl) -> Result<(), CodegenError> {
        let sig =
            self.funcs.get(&decl.name).cloned().ok_or_else(|| {
                CodegenError::new(format!("missing signature for `{}`", decl.name))
            })?;
        let function =
            self.fn_values.get(&decl.name).copied().ok_or_else(|| {
                CodegenError::new(format!("missing declaration for `{}`", decl.name))
            })?;
        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);
        self.func = Some(function);
        self.scopes.clear();
        self.breaks.clear();
        self.continues.clear();
        self.push_scope();
        for (index, param) in decl.params.iter().enumerate() {
            let value = function
                .get_nth_param(array_len(index)?)
                .ok_or_else(|| CodegenError::new(format!("missing parameter `{}`", param.name)))?;
            let ptr = self.alloca_entry(value.get_type(), &param.name)?;
            self.builder
                .build_store(ptr, value)
                .map_err(builder_error)?;
            self.declare(&param.name, ptr, value.get_type());
        }
        self.block(&decl.body)?;
        if !self.terminated() {
            match &sig.ret {
                Ty::Void => {
                    self.builder.build_return(None).map_err(builder_error)?;
                }
                ret => {
                    let zero = self.llvm_type(ret)?.const_zero();
                    self.builder
                        .build_return(Some(&zero))
                        .map_err(builder_error)?;
                }
            }
        }
        self.pop_scope();
        Ok(())
    }

    /// Collects struct and function signatures, resolving annotations.
    pub(crate) fn collect_signatures(&mut self) {
        for decl in &self.program.decls {
            match &decl.kind {
                DeclKind::Struct(decl) => {
                    let fields: Vec<FieldSig> = decl
                        .fields
                        .iter()
                        .map(|field| FieldSig {
                            name: field.name.clone(),
                            ty: self.annotation(&field.ty.span),
                        })
                        .collect();
                    self.structs.insert(
                        decl.name.clone(),
                        Rc::new(StructSig {
                            name: decl.name.clone(),
                            fields,
                        }),
                    );
                }
                DeclKind::Fn(decl) => {
                    let mut sig = self.signature(decl.params.iter(), &decl.ret.span);
                    // The C entry point is `i32 @main()` (0.1 §3.2).
                    if decl.name == "main" && sig.params.is_empty() {
                        sig.ret = Ty::Int;
                    }
                    self.funcs.insert(decl.name.clone(), sig);
                }
                DeclKind::Extern(decl) => {
                    let sig = self.signature(decl.params.iter(), &decl.ret.span);
                    self.funcs.insert(decl.name.clone(), sig);
                }
                DeclKind::Const(_) => {}
            }
        }
    }

    /// Builds a function signature from parameter annotations.
    pub(crate) fn signature<'p>(
        &self,
        params: impl Iterator<Item = &'p crate::ast::Param>,
        ret: &Span,
    ) -> FnSig {
        FnSig {
            params: params
                .map(|param| self.annotation(&param.ty.span))
                .collect(),
            ret: self.annotation(ret),
        }
    }
}
