//! # TechScript LLVM Backend — Codegen Engine
//!
//! Generates LLVM IR code block by block from TechScript IR instructions.

#![cfg(feature = "llvm")]

use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::collections::HashMap;
use std::ffi::CString;
use techscript_ast::LiteralVal;
use techscript_ir::types::{BlockId, IRType, ValueId};
use techscript_ir::{BasicBlock, Function, Instruction, Module, Op, TerminatorKind, Value};
use techscript_syntax::TokenKind;

use crate::context::CodegenContext;
use crate::type_map::to_llvm_type;

pub struct CodegenEngine<'a> {
    ctx: &'a mut CodegenContext,
}

impl<'a> CodegenEngine<'a> {
    pub fn new(ctx: &'a mut CodegenContext) -> Self {
        Self { ctx }
    }

    pub unsafe fn compile_module(&mut self, ir_module: &Module) -> Result<(), String> {
        // 1. Declare globals
        for &(ref global_id, ref name, ref ty) in &ir_module.globals {
            let llvm_ty = to_llvm_type(self.ctx.context, ty);
            let global_var = LLVMAddGlobal(
                self.ctx.module,
                llvm_ty,
                CString::new(name.as_str()).unwrap().as_ptr(),
            );
            self.ctx.register_value(*global_id, global_var);
        }

        // 2. Pre-declare all functions (enables forward-calls)
        let mut func_ptrs = HashMap::new();
        for func in &ir_module.functions {
            let mut param_types: Vec<LLVMTypeRef> = func
                .params
                .iter()
                .map(|(_, _, ty)| to_llvm_type(self.ctx.context, ty))
                .collect();

            let ret_type = to_llvm_type(self.ctx.context, &func.return_type);
            let func_ty = LLVMFunctionType(
                ret_type,
                param_types.as_mut_ptr(),
                param_types.len() as u32,
                0,
            );
            let llvm_func = LLVMAddFunction(
                self.ctx.module,
                CString::new(func.name.as_str()).unwrap().as_ptr(),
                func_ty,
            );
            func_ptrs.insert(func.id, llvm_func);
        }

        // 3. Compile functions
        for func in &ir_module.functions {
            let llvm_func = *func_ptrs.get(&func.id).unwrap();
            self.compile_function(func, llvm_func)?;
        }

        Ok(())
    }

    unsafe fn compile_function(
        &mut self,
        func: &Function,
        llvm_func: LLVMValueRef,
    ) -> Result<(), String> {
        // Clear value register cache for local scope
        self.ctx.values.clear();
        self.ctx.blocks.clear();

        // Register function parameter variables
        for (idx, &(local_id, _, _)) in func.params.iter().enumerate() {
            let param_val = LLVMGetParam(llvm_func, idx as u32);
            self.ctx.register_value(local_id, param_val);
        }

        // Allocate basic blocks
        for block in &func.blocks {
            let llvm_block = LLVMAppendBasicBlockInContext(
                self.ctx.context,
                llvm_func,
                CString::new(block.label.as_str()).unwrap().as_ptr(),
            );
            self.ctx.register_block(block.id, llvm_block);
        }

        // Emit instructions inside blocks
        for block in &func.blocks {
            let llvm_block = self.ctx.get_block(block.id).unwrap();
            LLVMPositionBuilderAtEnd(self.ctx.builder, llvm_block);

            for inst in &block.instructions {
                self.compile_instruction(inst)?;
            }

            // Emit block terminator
            if let Some(terminator) = &block.terminator {
                match &terminator.kind {
                    TerminatorKind::Jump(dest) => {
                        let dest_block = self.ctx.get_block(*dest).unwrap();
                        LLVMBuildBr(self.ctx.builder, dest_block);
                    }
                    TerminatorKind::ConditionalJump {
                        cond,
                        then_block,
                        else_block,
                    } => {
                        let cond_val = self.codegen_val(cond)?;
                        let then_b = self.ctx.get_block(*then_block).unwrap();
                        let else_b = self.ctx.get_block(*else_block).unwrap();
                        LLVMBuildCondBr(self.ctx.builder, cond_val, then_b, else_b);
                    }
                    TerminatorKind::Return(val_opt) => {
                        if let Some(val) = val_opt {
                            let ret_val = self.codegen_val(val)?;
                            LLVMBuildRet(self.ctx.builder, ret_val);
                        } else {
                            LLVMBuildRetVoid(self.ctx.builder);
                        }
                    }
                    TerminatorKind::Unreachable => {
                        LLVMBuildUnreachable(self.ctx.builder);
                    }
                }
            }
        }

        Ok(())
    }

    unsafe fn compile_instruction(&mut self, inst: &Instruction) -> Result<(), String> {
        let llvm_val = match &inst.op {
            Op::Constant(lit) => self.codegen_literal(lit)?,
            Op::Load(val) => {
                let addr = self.codegen_val(val)?;
                let name = CString::new("load_tmp").unwrap();
                LLVMBuildLoad2(
                    self.ctx.builder,
                    to_llvm_type(self.ctx.context, &inst.ty),
                    addr,
                    name.as_ptr(),
                )
            }
            Op::Store { target, value } => {
                let dest = self.codegen_val(target)?;
                let src = self.codegen_val(value)?;
                LLVMBuildStore(self.ctx.builder, src, dest);
                return Ok(());
            }
            Op::Move { target, value } => {
                let src = self.codegen_val(value)?;
                self.ctx.register_value(*target, src);
                return Ok(());
            }
            Op::BinaryOp { op, left, right } => {
                let l_val = self.codegen_val(left)?;
                let r_val = self.codegen_val(right)?;
                let name = CString::new("bin_op_tmp").unwrap();
                match op {
                    TokenKind::Plus => LLVMBuildAdd(self.ctx.builder, l_val, r_val, name.as_ptr()),
                    TokenKind::Minus => LLVMBuildSub(self.ctx.builder, l_val, r_val, name.as_ptr()),
                    TokenKind::Star => LLVMBuildMul(self.ctx.builder, l_val, r_val, name.as_ptr()),
                    TokenKind::Slash => {
                        if inst.ty == IRType::Float64 {
                            LLVMBuildFDiv(self.ctx.builder, l_val, r_val, name.as_ptr())
                        } else {
                            LLVMBuildSDiv(self.ctx.builder, l_val, r_val, name.as_ptr())
                        }
                    }
                    _ => return Err(format!("Unsupported LLVM binary op: {:?}", op)),
                }
            }
            Op::UnaryOp { op, right } => {
                let r_val = self.codegen_val(right)?;
                let name = CString::new("unary_op_tmp").unwrap();
                match op {
                    TokenKind::Minus => LLVMBuildNeg(self.ctx.builder, r_val, name.as_ptr()),
                    _ => return Err(format!("Unsupported LLVM unary op: {:?}", op)),
                }
            }
            Op::Compare { op, left, right } => {
                let l_val = self.codegen_val(left)?;
                let r_val = self.codegen_val(right)?;
                let name = CString::new("cmp_tmp").unwrap();
                let is_float = left.runtime_type() == IRType::Float64
                    || right.runtime_type() == IRType::Float64;
                if is_float {
                    let pred = match op {
                        TokenKind::Equal => llvm_sys::LLVMRealPredicate::LLVMRealOEQ,
                        TokenKind::LessThan => llvm_sys::LLVMRealPredicate::LLVMRealOLT,
                        TokenKind::GreaterThan => llvm_sys::LLVMRealPredicate::LLVMRealOGT,
                        _ => llvm_sys::LLVMRealPredicate::LLVMRealOEQ,
                    };
                    LLVMBuildFCmp(self.ctx.builder, pred, l_val, r_val, name.as_ptr())
                } else {
                    let pred = match op {
                        TokenKind::Equal => llvm_sys::LLVMIntPredicate::LLVMIntEQ,
                        TokenKind::LessThan => llvm_sys::LLVMIntPredicate::LLVMIntSLT,
                        TokenKind::GreaterThan => llvm_sys::LLVMIntPredicate::LLVMIntSGT,
                        _ => llvm_sys::LLVMIntPredicate::LLVMIntEQ,
                    };
                    LLVMBuildICmp(self.ctx.builder, pred, l_val, r_val, name.as_ptr())
                }
            }
            Op::Allocate(ty) => {
                let llvm_ty = to_llvm_type(self.ctx.context, ty);
                let name = CString::new("alloc_tmp").unwrap();
                LLVMBuildAlloca(self.ctx.builder, llvm_ty, name.as_ptr())
            }
            Op::Call { callee, args } => {
                let func_ptr = self.codegen_val(callee)?;
                let mut llvm_args: Vec<LLVMValueRef> = args
                    .iter()
                    .map(|a| self.codegen_val(a))
                    .collect::<Result<_, _>>()?;

                let name = CString::new("call_tmp").unwrap();
                let func_ty = to_llvm_type(self.ctx.context, &callee.runtime_type());
                LLVMBuildCall2(
                    self.ctx.builder,
                    func_ty,
                    func_ptr,
                    llvm_args.as_mut_ptr(),
                    llvm_args.len() as u32,
                    name.as_ptr(),
                )
            }
            Op::NoOp => return Ok(()),
            other => return Err(format!("LLVM Codegen unhandled op: {:?}", other)),
        };

        if let Some(res_id) = inst.result {
            self.ctx.register_value(res_id, llvm_val);
        }

        Ok(())
    }

    unsafe fn codegen_val(&self, val: &Value) -> Result<LLVMValueRef, String> {
        match val {
            Value::Temp(id) => self
                .ctx
                .get_value(*id)
                .ok_or_else(|| format!("ValueId {:?} not found", id)),
            Value::Local(id) => self
                .ctx
                .get_value(*id)
                .ok_or_else(|| format!("LocalId {:?} not found", id)),
            Value::Global(id) => self
                .ctx
                .get_value(*id)
                .ok_or_else(|| format!("GlobalId {:?} not found", id)),
            Value::Const(lit) => self.codegen_literal(lit),
            Value::Null => Ok(LLVMConstNull(LLVMPointerType(
                LLVMInt8TypeInContext(self.ctx.context),
                0,
            ))),
        }
    }

    unsafe fn codegen_literal(&self, lit: &LiteralVal) -> Result<LLVMValueRef, String> {
        match lit {
            LiteralVal::Int(i) => Ok(LLVMConstInt(
                LLVMInt64TypeInContext(self.ctx.context),
                *i as u64,
                1,
            )),
            LiteralVal::Float(f) => {
                Ok(LLVMConstReal(LLVMDoubleTypeInContext(self.ctx.context), *f))
            }
            LiteralVal::Bool(b) => Ok(LLVMConstInt(
                LLVMInt1TypeInContext(self.ctx.context),
                if *b { 1 } else { 0 },
                0,
            )),
            LiteralVal::Str(s) => {
                let cstr = CString::new(s.as_str()).unwrap();
                let name = CString::new("str_lit").unwrap();
                Ok(LLVMBuildGlobalStringPtr(
                    self.ctx.builder,
                    cstr.as_ptr(),
                    name.as_ptr(),
                ))
            }
            LiteralVal::Null => Ok(LLVMConstNull(LLVMPointerType(
                LLVMInt8TypeInContext(self.ctx.context),
                0,
            ))),
        }
    }
}

trait TypeInfo {
    fn runtime_type(&self) -> IRType;
}

impl TypeInfo for Value {
    fn runtime_type(&self) -> IRType {
        match self {
            Value::Const(lit) => match lit {
                LiteralVal::Int(_) => IRType::Int64,
                LiteralVal::Float(_) => IRType::Float64,
                LiteralVal::Bool(_) => IRType::Bool,
                LiteralVal::Str(_) => IRType::String,
                LiteralVal::Null => IRType::Any, // Opaque
            },
            _ => IRType::Any, // Opaque or dynamic
        }
    }
}
