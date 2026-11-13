//! The lowering state and the pass that drives it.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, StructType};
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};

use crate::ast::{DeclKind, Program};
use crate::diagnostics::Span;
use crate::region::Assignment;
use crate::typeck::{FnSig, StructSig, Ty, TypeMap};

use super::CodegenError;

mod expr;
mod function;
mod scope;
mod stmt;

/// A single module being lowered.
pub(super) struct Lower<'ctx, 'a> {
    pub(super) context: &'ctx Context,
    pub(super) module: Module<'ctx>,
    pub(super) builder: Builder<'ctx>,
    pub(super) program: &'a Program,
    pub(super) types: &'a TypeMap,
    pub(super) structs: HashMap<String, Rc<StructSig>>,
    pub(super) struct_types: HashMap<String, StructType<'ctx>>,
    pub(super) building: HashSet<String>,
    pub(super) funcs: HashMap<String, FnSig>,
    pub(super) fn_values: HashMap<String, FunctionValue<'ctx>>,
    pub(super) consts: HashMap<String, BasicValueEnum<'ctx>>,
    pub(super) func: Option<FunctionValue<'ctx>>,
    pub(super) scopes: Vec<HashMap<String, (PointerValue<'ctx>, BasicTypeEnum<'ctx>)>>,
    pub(super) breaks: Vec<BasicBlock<'ctx>>,
    pub(super) continues: Vec<BasicBlock<'ctx>>,
}

impl<'ctx, 'a> Lower<'ctx, 'a> {
    pub(crate) fn new(
        context: &'ctx Context,
        module: Module<'ctx>,
        types: &'a TypeMap,
        program: &'a Program,
        _assignment: &'a Assignment,
    ) -> Self {
        Self {
            context,
            module,
            builder: context.create_builder(),
            program,
            types,
            structs: HashMap::new(),
            struct_types: HashMap::new(),
            building: HashSet::new(),
            funcs: HashMap::new(),
            fn_values: HashMap::new(),
            consts: HashMap::new(),
            func: None,
            scopes: Vec::new(),
            breaks: Vec::new(),
            continues: Vec::new(),
        }
    }

    pub(crate) fn run(&mut self) -> Result<(), CodegenError> {
        self.collect_signatures();
        // Struct bodies first: function signatures and bodies may name them.
        let struct_names: Vec<String> = self.structs.keys().cloned().collect();
        for name in &struct_names {
            self.struct_type(name)?;
        }
        // Declare every function before any body so calls may go forward.
        let fn_names: Vec<String> = self.funcs.keys().cloned().collect();
        for name in &fn_names {
            let sig = self
                .funcs
                .get(name)
                .cloned()
                .ok_or_else(|| CodegenError::new(format!("missing signature for `{name}`")))?;
            let ty = self.fn_type(&sig)?;
            let value = self.module.add_function(name, ty, Some(Linkage::External));
            self.fn_values.insert(name.clone(), value);
        }
        for decl in &self.program.decls {
            if let DeclKind::Const(decl) = &decl.kind {
                self.lower_const(decl)?;
            }
        }
        for decl in &self.program.decls {
            if let DeclKind::Fn(decl) = &decl.kind {
                self.lower_function(decl)?;
            }
        }
        Ok(())
    }

    /// Returns the resolved type recorded at `span`.
    pub(crate) fn annotation(&self, span: &Span) -> Ty {
        self.types.ty(*span).cloned().unwrap_or(Ty::Error)
    }

    /// Returns the resolved type of `expr`.
    pub(crate) fn expr_ty(&self, expr: &crate::ast::Expr) -> Ty {
        self.types.ty(expr.span).cloned().unwrap_or(Ty::Error)
    }

    /// Builds the LLVM function type of `sig`.
    pub(crate) fn fn_type(
        &mut self,
        sig: &FnSig,
    ) -> Result<inkwell::types::FunctionType<'ctx>, CodegenError> {
        let mut params: Vec<BasicMetadataTypeEnum<'ctx>> = Vec::with_capacity(sig.params.len());
        for ty in &sig.params {
            params.push(self.llvm_type(ty)?.into());
        }
        match &sig.ret {
            Ty::Void => Ok(self.context.void_type().fn_type(&params, false)),
            ret => Ok(self.llvm_type(ret)?.fn_type(&params, false)),
        }
    }
}
