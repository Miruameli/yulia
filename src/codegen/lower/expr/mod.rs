//! Expression lowering: dispatch, literals, names, prefix operations,
//! assignment, and calls.

use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum, ValueKind};

use crate::ast::{Expr, ExprKind, UnaryOp};

use crate::codegen::{CodegenError, builder_error};

use super::super::Lower;

mod access;
mod aggregate;
mod binary;
mod checked;

impl<'ctx> Lower<'ctx, '_> {
    /// Lowers `expr` to an LLVM value.
    pub(crate) fn expr(&mut self, expr: &Expr) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        match &expr.kind {
            ExprKind::IntLit(text) => self.int_literal(text),
            ExprKind::BoolLit(value) => Ok(self
                .context
                .bool_type()
                .const_int(u64::from(*value), false)
                .into()),
            ExprKind::Ident(name) => self.ident(name),
            ExprKind::Unary { op, operand } => self.unary(*op, operand),
            ExprKind::Binary { op, lhs, rhs } => self.binary(*op, lhs, rhs),
            ExprKind::Assign { target, value } => self.assign(target, value),
            ExprKind::Call { callee, args } => self.call(callee, args),
            ExprKind::Field { base, name } => self.field(base, name),
            ExprKind::Index { base, index } => self.index(base, index),
            ExprKind::StructLiteral { name, fields } => self.struct_literal(name, fields),
            ExprKind::ArrayLiteral { elements } => self.array_literal(expr, elements),
            ExprKind::ArrayRepeat { value, count } => self.array_repeat(value, count),
            ExprKind::FloatLit(_)
            | ExprKind::CharLit(_)
            | ExprKind::StringLit(_)
            | ExprKind::Panic(_) => Err(CodegenError::new(
                "this expression is outside the Phase 1 subset",
            )),
        }
    }

    /// Lowers an integer literal, which the lexer keeps as written.
    pub(crate) fn int_literal(&self, text: &str) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        let cleaned: String = text.chars().filter(|c| *c != '_').collect();
        let (radix, digits) = if let Some(rest) = cleaned
            .strip_prefix("0x")
            .or_else(|| cleaned.strip_prefix("0X"))
        {
            (16, rest)
        } else if let Some(rest) = cleaned
            .strip_prefix("0b")
            .or_else(|| cleaned.strip_prefix("0B"))
        {
            (2, rest)
        } else if let Some(rest) = cleaned
            .strip_prefix("0o")
            .or_else(|| cleaned.strip_prefix("0O"))
        {
            (8, rest)
        } else {
            (10, cleaned.as_str())
        };
        let value = i32::from_str_radix(digits, radix)
            .map_err(|_| CodegenError::new(format!("`{text}` does not fit in `int`")))?;
        let bits = u32::from_ne_bytes(value.to_ne_bytes());
        Ok(self
            .context
            .i32_type()
            .const_int(u64::from(bits), true)
            .into())
    }

    /// Loads a name: a local, a parameter, or a constant.
    pub(crate) fn ident(&mut self, name: &str) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        if let Some(value) = self.consts.get(name).copied() {
            return Ok(value);
        }
        let (ptr, ty) = self
            .lookup(name)
            .ok_or_else(|| CodegenError::new(format!("`{name}` is not a value")))?;
        self.builder
            .build_load(ty, ptr, name)
            .map_err(builder_error)
    }

    /// Lowers a prefix operation.
    pub(crate) fn unary(
        &mut self,
        op: UnaryOp,
        operand: &Expr,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        let value = self.expr(operand)?.into_int_value();
        let result = match op {
            UnaryOp::Not => self.builder.build_not(value, "not"),
            UnaryOp::Neg => {
                let zero = value.get_type().const_zero();
                self.builder.build_int_sub(zero, value, "neg")
            }
        }
        .map_err(builder_error)?;
        Ok(result.into())
    }

    /// Lowers `target = value`, returning the stored value.
    pub(crate) fn assign(
        &mut self,
        target: &Expr,
        value: &Expr,
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        let (ptr, ty) = self.lvalue(target)?;
        let value = self.expr(value)?;
        if value.get_type() != ty {
            return Err(CodegenError::new(
                "the assigned value does not match the target type",
            ));
        }
        self.builder
            .build_store(ptr, value)
            .map_err(builder_error)?;
        Ok(value)
    }

    /// Lowers a call to a declared function.
    pub(crate) fn call(
        &mut self,
        callee: &Expr,
        args: &[Expr],
    ) -> Result<BasicValueEnum<'ctx>, CodegenError> {
        let name = match &callee.kind {
            ExprKind::Ident(name) => name.clone(),
            _ => return Err(CodegenError::new("the call target is not a function")),
        };
        let function = self
            .fn_values
            .get(&name)
            .copied()
            .ok_or_else(|| CodegenError::new(format!("`{name}` is not a function")))?;
        let mut values = Vec::with_capacity(args.len());
        for arg in args {
            values.push(self.expr(arg)?);
        }
        let metadata: Vec<BasicMetadataValueEnum<'ctx>> = values
            .iter()
            .copied()
            .map(BasicMetadataValueEnum::from)
            .collect();
        let call = self
            .builder
            .build_call(function, &metadata, &name)
            .map_err(builder_error)?;
        Ok(match call.try_as_basic_value() {
            ValueKind::Basic(value) => value,
            // A `void` call only appears as a statement; the checker rejects a
            // void value anywhere it would be used.
            ValueKind::Instruction(_) => self.context.i32_type().const_zero().into(),
        })
    }
}
