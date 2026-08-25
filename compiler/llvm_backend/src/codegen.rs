//! # TechScript LLVM Backend — Codegen Engine
//!
//! Generates LLVM IR code block by block from TechScript IR instructions.

#![cfg(feature = "llvm")]

use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::collections::HashMap;
use std::ffi::CString;
use techscript_ast::LiteralVal;
use techscript_ir::types::{BlockId, GlobalId, IRType, ValueId};
use techscript_ir::{Function, Instruction, Module, Op, TerminatorKind, Value};
use techscript_syntax::TokenKind;

use crate::context::CodegenContext;
use crate::type_map::to_llvm_type;

pub struct CodegenEngine<'a> {
    ctx: &'a mut CodegenContext,
    temp_to_global: HashMap<ValueId, GlobalId>,
    phi_resolutions: Vec<(LLVMValueRef, Vec<(BlockId, Value)>)>,
    global_names: HashMap<GlobalId, String>,
}

impl<'a> CodegenEngine<'a> {
    pub fn new(ctx: &'a mut CodegenContext) -> Self {
        Self {
            ctx,
            temp_to_global: HashMap::new(),
            phi_resolutions: Vec::new(),
            global_names: HashMap::new(),
        }
    }

    pub unsafe fn compile_module(&mut self, ir_module: &Module) -> Result<(), String> {
        self.global_names.clear();

        // 1. Declare globals
        for &(ref global_id, ref name, ref ty) in &ir_module.globals {
            let llvm_ty = to_llvm_type(self.ctx.context, ty);
            let global_var = LLVMAddGlobal(
                self.ctx.module,
                llvm_ty,
                CString::new(name.as_str()).unwrap().as_ptr(),
            );
            self.ctx.register_global(*global_id, global_var);
            self.global_names.insert(*global_id, name.clone());
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
        self.temp_to_global.clear();
        self.phi_resolutions.clear();

        // Register function parameter variables
        for (idx, &(local_id, _, _)) in func.params.iter().enumerate() {
            let param_val = LLVMGetParam(llvm_func, idx as u32);
            self.ctx.register_local(local_id, param_val);
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
<
                    }
                    TerminatorKind::ConditionalJump {
                        cond,
                        then_block,
                        else_block,
                    } => {
                        let cond_val = self.codegen_val(cond)?;
                        // Coerce condition to boolean if it's dynamic
                        let cond_bool = if LLVMGetTypeKind(LLVMTypeOf(cond_val))
                            == llvm_sys::LLVMTypeKind::LLVMIntegerTypeKind
                            && LLVMGetIntTypeWidth(LLVMTypeOf(cond_val)) == 1
                        {
                            cond_val
                        } else {
                            let boxed = self.box_val(cond_val)?;
                            let fn_val = self.get_or_declare_runtime_fn(
                                "ts_cast",
                                LLVMPointerType(LLVMInt8TypeInContext(self.ctx.context), 0),
                                &[
                                    LLVMPointerType(LLVMInt8TypeInContext(self.ctx.context), 0),
                                    LLVMInt32TypeInContext(self.ctx.context),
                                ],
                            );
                            let cast_val = LLVMBuildCall2(
                                self.ctx.builder,
                                LLVMTypeOf(fn_val),
                                fn_val,
                                [
                                    boxed,
                                    LLVMConstInt(LLVMInt32TypeInContext(self.ctx.context), 1, 0),
                                ]
                                .as_mut_ptr(),
                                2,
                                CString::new("cast_bool").unwrap().as_ptr(),
                            );
                            // Now read boolean field (we can call ts_eq to compare with true, or unbox)
                            // A simple way is to check if it matches a boxed true val:
                            let fn_val2 = self.get_or_declare_runtime_fn(
                                "ts_eq",
                                LLVMInt1TypeInContext(self.ctx.context),
                                &[
                                    LLVMPointerType(LLVMInt8TypeInContext(self.ctx.context), 0),
                                    LLVMPointerType(LLVMInt8TypeInContext(self.ctx.context), 0),
                                ],
                            );
                            let true_box = self.box_val(LLVMConstInt(
                                LLVMInt1TypeInContext(self.ctx.context),
                                1,
                                0,
                            ))?;
                            LLVMBuildCall2(
                                self.ctx.builder,
                                LLVMTypeOf(fn_val2),
                                fn_val2,
                                [cast_val, true_box].as_mut_ptr(),
                                2,
                                CString::new("is_true").unwrap().as_ptr(),
                            )
                        };
                        let then_b = self.ctx.get_block(*then_block).unwrap();
                        let else_b = self.ctx.get_block(*else_block).unwrap();
                        LLVMBuildCondBr(self.ctx.builder, cond_bool, then_b, else_b);
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

        // 4. Resolve Phi nodes
        for &(phi_val, ref incoming) in &self.phi_resolutions {
            let mut incoming_vals: Vec<LLVMValueRef> = Vec::new();
            let mut incoming_blocks: Vec<LLVMBasicBlockRef> = Vec::new();
            for &(block_id, ref val) in incoming {
                if let Some(block) = self.ctx.get_block(block_id) {
                    if let Ok(v) = self.codegen_val(val) {
                        incoming_vals.push(v);
                        incoming_blocks.push(block);
                    }
                }
            }
            LLVMAddIncoming(
                phi_val,
                incoming_vals.as_mut_ptr(),
                incoming_blocks.as_mut_ptr(),
                incoming_vals.len() as u32,
            );
        }

        Ok(())
    }

    unsafe fn compile_instruction(&mut self, inst: &Instruction) -> Result<(), String> {
        let context = self.ctx.context;
        let i8_ptr_ty = LLVMPointerType(LLVMInt8TypeInContext(context), 0);
        let i64_ty = LLVMInt64TypeInContext(context);
        let i1_ty = LLVMInt1TypeInContext(context);
        let i32_ty = LLVMInt32TypeInContext(context);
        let double_ty = LLVMDoubleTypeInContext(context);

        let llvm_val = match &inst.op {
            Op::Constant(lit) => {
                if inst.ty == IRType::Any {
                    let unboxed = self.codegen_literal(lit)?;
                    self.box_val(unboxed)?
                } else {
                    self.codegen_literal(lit)?
                }
            }
            Op::Load(val) => {
                let addr = self.codegen_val(val)?;
                let name = CString::new("load_tmp").unwrap();
                let load_res = LLVMBuildLoad2(
                    self.ctx.builder,
                    to_llvm_type(self.ctx.context, &inst.ty),
                    addr,
                    name.as_ptr(),
                );
                if let Value::Global(global_id) = val {
                    if let Some(res_id) = inst.result {
                        self.temp_to_global.insert(res_id, *global_id);
                    }
                }
                load_res
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

                let l_kind = LLVMGetTypeKind(LLVMTypeOf(l_val));
                let r_kind = LLVMGetTypeKind(LLVMTypeOf(r_val));

                let is_int = l_kind == llvm_sys::LLVMTypeKind::LLVMIntegerTypeKind
                    && r_kind == llvm_sys::LLVMTypeKind::LLVMIntegerTypeKind;
                let is_float = l_kind == llvm_sys::LLVMTypeKind::LLVMDoubleTypeKind
                    || r_kind == llvm_sys::LLVMTypeKind::LLVMDoubleTypeKind;

                match op {
                    TokenKind::Plus => {
                        if is_int {
                            LLVMBuildAdd(self.ctx.builder, l_val, r_val, name.as_ptr())
                        } else if is_float {
                            let lf = self.coerce_to_float(l_val)?;
                            let rf = self.coerce_to_float(r_val)?;
                            LLVMBuildFAdd(self.ctx.builder, lf, rf, name.as_ptr())
                        } else {
                            let fn_val = self.get_or_declare_runtime_fn(
                                "ts_add",
                                i8_ptr_ty,
                                &[i8_ptr_ty, i8_ptr_ty],
                            );
                            let mut args = [self.box_val(l_val)?, self.box_val(r_val)?];
                            LLVMBuildCall2(
                                self.ctx.builder,
                                LLVMTypeOf(fn_val),
                                fn_val,
                                args.as_mut_ptr(),
                                2,
                                name.as_ptr(),
                            )
                        }
                    }
                    TokenKind::Minus => {
                        if is_int {
                            LLVMBuildSub(self.ctx.builder, l_val, r_val, name.as_ptr())
                        } else if is_float {
                            let lf = self.coerce_to_float(l_val)?;
                            let rf = self.coerce_to_float(r_val)?;
                            LLVMBuildFSub(self.ctx.builder, lf, rf, name.as_ptr())
                        } else {
                            let fn_val = self.get_or_declare_runtime_fn(
                                "ts_sub",
                                i8_ptr_ty,
                                &[i8_ptr_ty, i8_ptr_ty],
                            );
                            let mut args = [self.box_val(l_val)?, self.box_val(r_val)?];
                            LLVMBuildCall2(
                                self.ctx.builder,
                                LLVMTypeOf(fn_val),
                                fn_val,
                                args.as_mut_ptr(),
                                2,
                                name.as_ptr(),
                            )
                        }
                    }
                    TokenKind::Star => {
                        if is_int {
                            LLVMBuildMul(self.ctx.builder, l_val, r_val, name.as_ptr())
                        } else if is_float {
                            let lf = self.coerce_to_float(l_val)?;
                            let rf = self.coerce_to_float(r_val)?;
                            LLVMBuildFMul(self.ctx.builder, lf, rf, name.as_ptr())
                        } else {
                            let fn_val = self.get_or_declare_runtime_fn(
                                "ts_mul",
                                i8_ptr_ty,
                                &[i8_ptr_ty, i8_ptr_ty],
                            );
                            let mut args = [self.box_val(l_val)?, self.box_val(r_val)?];
                            LLVMBuildCall2(
                                self.ctx.builder,
                                LLVMTypeOf(fn_val),
                                fn_val,
                                args.as_mut_ptr(),
                                2,
                                name.as_ptr(),
                            )
                        }
                    }
                    TokenKind::Slash => {
                        if is_int {
                            LLVMBuildSDiv(self.ctx.builder, l_val, r_val, name.as_ptr())
                        } else if is_float {
                            let lf = self.coerce_to_float(l_val)?;
                            let rf = self.coerce_to_float(r_val)?;
                            LLVMBuildFDiv(self.ctx.builder, lf, rf, name.as_ptr())
                        } else {
                            let fn_val = self.get_or_declare_runtime_fn(
                                "ts_div",
                                i8_ptr_ty,
                                &[i8_ptr_ty, i8_ptr_ty],
                            );
                            let mut args = [self.box_val(l_val)?, self.box_val(r_val)?];
                            LLVMBuildCall2(
                                self.ctx.builder,
                                LLVMTypeOf(fn_val),
                                fn_val,
                                args.as_mut_ptr(),
                                2,
                                name.as_ptr(),
                            )
                        }
                    }
                    TokenKind::Percent => {
                        if is_int {
                            LLVMBuildSRem(self.ctx.builder, l_val, r_val, name.as_ptr())
                        } else {
                            let fn_val = self.get_or_declare_runtime_fn(
                                "ts_mod",
                                i8_ptr_ty,
                                &[i8_ptr_ty, i8_ptr_ty],
                            );
                            let mut args = [self.box_val(l_val)?, self.box_val(r_val)?];
                            LLVMBuildCall2(
                                self.ctx.builder,
                                LLVMTypeOf(fn_val),
                                fn_val,
                                args.as_mut_ptr(),
                                2,
                                name.as_ptr(),
                            )
                        }
                    }
                    TokenKind::DoubleStar => {
                        let fn_val = self.get_or_declare_runtime_fn(
                            "ts_pow",
                            i8_ptr_ty,
                            &[i8_ptr_ty, i8_ptr_ty],
                        );
                        let mut args = [self.box_val(l_val)?, self.box_val(r_val)?];
                        LLVMBuildCall2(
                            self.ctx.builder,
                            LLVMTypeOf(fn_val),
                            fn_val,
                            args.as_mut_ptr(),
                            2,
                            name.as_ptr(),
                        )
                    }
                    TokenKind::And => {
                        if l_kind == llvm_sys::LLVMTypeKind::LLVMIntegerTypeKind
                            && LLVMGetIntTypeWidth(LLVMTypeOf(l_val)) == 1
                        {
                            LLVMBuildAnd(self.ctx.builder, l_val, r_val, name.as_ptr())
                        } else {
                            let fn_val = self.get_or_declare_runtime_fn(
                                "ts_cast",
                                i8_ptr_ty,
                                &[i8_ptr_ty, i32_ty],
                            );
                            let l_b = LLVMBuildCall2(
                                self.ctx.builder,
                                LLVMTypeOf(fn_val),
                                fn_val,
                                [self.box_val(l_val)?, LLVMConstInt(i32_ty, 1, 0)].as_mut_ptr(),
                                2,
                                CString::new("l_bool").unwrap().as_ptr(),
                            );
                            let r_b = LLVMBuildCall2(
                                self.ctx.builder,
                                LLVMTypeOf(fn_val),
                                fn_val,
                                [self.box_val(r_val)?, LLVMConstInt(i32_ty, 1, 0)].as_mut_ptr(),
                                2,
                                CString::new("r_bool").unwrap().as_ptr(),
                            );
                            let fn_val2 = self.get_or_declare_runtime_fn(
                                "ts_eq",
                                i1_ty,
                                &[i8_ptr_ty, i8_ptr_ty],
                            );
                            let true_box = self.box_val(LLVMConstInt(i1_ty, 1, 0))?;
                            let l_bool = LLVMBuildCall2(
                                self.ctx.builder,
                                LLVMTypeOf(fn_val2),
                                fn_val2,
                                [l_b, true_box].as_mut_ptr(),
                                2,
                                CString::new("l_bool_un").unwrap().as_ptr(),
                            );
                            let r_bool = LLVMBuildCall2(
                                self.ctx.builder,
                                LLVMTypeOf(fn_val2),
                                fn_val2,
                                [r_b, true_box].as_mut_ptr(),
                                2,
                                CString::new("r_bool_un").unwrap().as_ptr(),
                            );
                            let and_res =
                                LLVMBuildAnd(self.ctx.builder, l_bool, r_bool, name.as_ptr());
                            self.box_val(and_res)?
                        }
                    }
                    TokenKind::Or => {
                        if l_kind == llvm_sys::LLVMTypeKind::LLVMIntegerTypeKind
                            && LLVMGetIntTypeWidth(LLVMTypeOf(l_val)) == 1
                        {
                            LLVMBuildOr(self.ctx.builder, l_val, r_val, name.as_ptr())
                        } else {
                            let fn_val = self.get_or_declare_runtime_fn(
                                "ts_cast",
                                i8_ptr_ty,
                                &[i8_ptr_ty, i32_ty],
                            );
                            let l_b = LLVMBuildCall2(
                                self.ctx.builder,
                                LLVMTypeOf(fn_val),
                                fn_val,
                                [self.box_val(l_val)?, LLVMConstInt(i32_ty, 1, 0)].as_mut_ptr(),
                                2,
                                CString::new("l_bool").unwrap().as_ptr(),
                            );
                            let r_b = LLVMBuildCall2(
                                self.ctx.builder,
                                LLVMTypeOf(fn_val),
                                fn_val,
                                [self.box_val(r_val)?, LLVMConstInt(i32_ty, 1, 0)].as_mut_ptr(),
                                2,
                                CString::new("r_bool").unwrap().as_ptr(),
                            );
                            let fn_val2 = self.get_or_declare_runtime_fn(
                                "ts_eq",
                                i1_ty,
                                &[i8_ptr_ty, i8_ptr_ty],
                            );
                            let true_box = self.box_val(LLVMConstInt(i1_ty, 1, 0))?;
                            let l_bool = LLVMBuildCall2(
                                self.ctx.builder,
                                LLVMTypeOf(fn_val2),
                                fn_val2,
                                [l_b, true_box].as_mut_ptr(),
                                2,
                                CString::new("l_bool_un").unwrap().as_ptr(),
                            );
                            let r_bool = LLVMBuildCall2(
                                self.ctx.builder,
                                LLVMTypeOf(fn_val2),
                                fn_val2,
                                [r_b, true_box].as_mut_ptr(),
                                2,
                                CString::new("r_bool_un").unwrap().as_ptr(),
                            );
                            let or_res =
                                LLVMBuildOr(self.ctx.builder, l_bool, r_bool, name.as_ptr());
                            self.box_val(or_res)?
                        }
                    }
                    _ => return Err(format!("Unsupported LLVM binary op: {:?}", op)),
                }
            }
            Op::UnaryOp { op, right } => {
                let r_val = self.codegen_val(right)?;
                let name = CString::new("unary_op_tmp").unwrap();
                let r_kind = LLVMGetTypeKind(LLVMTypeOf(r_val));

                match op {
                    TokenKind::Minus => {
                        if r_kind == llvm_sys::LLVMTypeKind::LLVMIntegerTypeKind {
                            LLVMBuildNeg(self.ctx.builder, r_val, name.as_ptr())
                        } else if r_kind == llvm_sys::LLVMTypeKind::LLVMDoubleTypeKind {
                            LLVMBuildFNeg(self.ctx.builder, r_val, name.as_ptr())
                        } else {
                            let fn_val = self.get_or_declare_runtime_fn(
                                "ts_sub",
                                i8_ptr_ty,
                                &[i8_ptr_ty, i8_ptr_ty],
                            );
                            let zero = self.box_val(LLVMConstInt(i64_ty, 0, 1))?;
                            let boxed = self.box_val(r_val)?;
                            let mut args = [zero, boxed];
                            LLVMBuildCall2(
                                self.ctx.builder,
                                LLVMTypeOf(fn_val),
                                fn_val,
                                args.as_mut_ptr(),
                                2,
                                name.as_ptr(),
                            )
                        }
                    }
                    TokenKind::Not => {
                        if r_kind == llvm_sys::LLVMTypeKind::LLVMIntegerTypeKind
                            && LLVMGetIntTypeWidth(LLVMTypeOf(r_val)) == 1
                        {
                            LLVMBuildNot(self.ctx.builder, r_val, name.as_ptr())
                        } else {
                            let fn_val = self.get_or_declare_runtime_fn(
                                "ts_cast",
                                i8_ptr_ty,
                                &[i8_ptr_ty, i32_ty],
                            );
                            let boxed = self.box_val(r_val)?;
                            let cast_bool = LLVMBuildCall2(
                                self.ctx.builder,
                                LLVMTypeOf(fn_val),
                                fn_val,
                                [boxed, LLVMConstInt(i32_ty, 1, 0)].as_mut_ptr(),
                                2,
                                CString::new("cast_bool").unwrap().as_ptr(),
                            );
                            let fn_val2 = self.get_or_declare_runtime_fn(
                                "ts_eq",
                                i1_ty,
                                &[i8_ptr_ty, i8_ptr_ty],
                            );
                            let true_box = self.box_val(LLVMConstInt(i1_ty, 1, 0))?;
                            let is_true = LLVMBuildCall2(
                                self.ctx.builder,
                                LLVMTypeOf(fn_val2),
                                fn_val2,
                                [cast_bool, true_box].as_mut_ptr(),
                                2,
                                CString::new("is_true").unwrap().as_ptr(),
                            );
                            let not_res = LLVMBuildNot(self.ctx.builder, is_true, name.as_ptr());
                            self.box_val(not_res)?
                        }
                    }
                    TokenKind::Await => {
                        let i8_ptr_ty = LLVMPointerType(LLVMInt8TypeInContext(self.ctx.context), 0);
                        let fn_val =
                            self.get_or_declare_runtime_fn("ts_await", i8_ptr_ty, &[i8_ptr_ty]);
                        let boxed = self.box_val(r_val)?;
                        let mut args = [boxed];
                        LLVMBuildCall2(
                            self.ctx.builder,
                            LLVMTypeOf(fn_val),
                            fn_val,
                            args.as_mut_ptr(),
                            1,
                            name.as_ptr(),
                        )
                    }
                    _ => return Err(format!("Unsupported LLVM unary op: {:?}", op)),
                }
            }
            Op::Compare { op, left, right } => {
                let l_val = self.codegen_val(left)?;
                let r_val = self.codegen_val(right)?;
                let name = CString::new("cmp_tmp").unwrap();

                let l_kind = LLVMGetTypeKind(LLVMTypeOf(l_val));
                let r_kind = LLVMGetTypeKind(LLVMTypeOf(r_val));

                let is_int = l_kind == llvm_sys::LLVMTypeKind::LLVMIntegerTypeKind
                    && r_kind == llvm_sys::LLVMTypeKind::LLVMIntegerTypeKind;
                let is_float = l_kind == llvm_sys::LLVMTypeKind::LLVMDoubleTypeKind
                    || r_kind == llvm_sys::LLVMTypeKind::LLVMDoubleTypeKind;

                if is_int {
                    let pred = match op {
                        TokenKind::EqualEqual | TokenKind::TripleEqual => {
                            llvm_sys::LLVMIntPredicate::LLVMIntEQ
                        }
                        TokenKind::BangEqual | TokenKind::BangEqualEqual => {
                            llvm_sys::LLVMIntPredicate::LLVMIntNE
                        }
                        TokenKind::Less => llvm_sys::LLVMIntPredicate::LLVMIntSLT,
                        TokenKind::LessEqual => llvm_sys::LLVMIntPredicate::LLVMIntSLE,
                        TokenKind::Greater => llvm_sys::LLVMIntPredicate::LLVMIntSGT,
                        TokenKind::GreaterEqual => llvm_sys::LLVMIntPredicate::LLVMIntSGE,
                        _ => llvm_sys::LLVMIntPredicate::LLVMIntEQ,
                    };
                    LLVMBuildICmp(self.ctx.builder, pred, l_val, r_val, name.as_ptr())
                } else if is_float {
                    let pred = match op {
                        TokenKind::EqualEqual | TokenKind::TripleEqual => {
                            llvm_sys::LLVMRealPredicate::LLVMRealOEQ
                        }
                        TokenKind::BangEqual | TokenKind::BangEqualEqual => {
                            llvm_sys::LLVMRealPredicate::LLVMRealUNE
                        }
                        TokenKind::Less => llvm_sys::LLVMRealPredicate::LLVMRealOLT,
                        TokenKind::LessEqual => llvm_sys::LLVMRealPredicate::LLVMRealOLE,
                        TokenKind::Greater => llvm_sys::LLVMRealPredicate::LLVMRealOGT,
                        TokenKind::GreaterEqual => llvm_sys::LLVMRealPredicate::LLVMRealOGE,
                        _ => llvm_sys::LLVMRealPredicate::LLVMRealOEQ,
                    };
                    let lf = self.coerce_to_float(l_val)?;
                    let rf = self.coerce_to_float(r_val)?;
                    LLVMBuildFCmp(self.ctx.builder, pred, lf, rf, name.as_ptr())
                } else {
                    let (fn_name, ret_ty) = match op {
                        TokenKind::EqualEqual | TokenKind::TripleEqual => ("ts_eq", i1_ty),
                        TokenKind::BangEqual | TokenKind::BangEqualEqual => ("ts_ne", i1_ty),
                        TokenKind::Less => ("ts_lt", i1_ty),
                        TokenKind::LessEqual => ("ts_le", i1_ty),
                        TokenKind::Greater => ("ts_gt", i1_ty),
                        TokenKind::GreaterEqual => ("ts_ge", i1_ty),
                        _ => ("ts_eq", i1_ty),
                    };
                    let fn_val =
                        self.get_or_declare_runtime_fn(fn_name, ret_ty, &[i8_ptr_ty, i8_ptr_ty]);
                    let mut args = [self.box_val(l_val)?, self.box_val(r_val)?];
                    LLVMBuildCall2(
                        self.ctx.builder,
                        LLVMTypeOf(fn_val),
                        fn_val,
                        args.as_mut_ptr(),
                        2,
                        name.as_ptr(),
                    )
                }
            }
            Op::Allocate(ty) => {
                let llvm_ty = to_llvm_type(self.ctx.context, ty);
                let name = CString::new("alloc_tmp").unwrap();
                LLVMBuildAlloca(self.ctx.builder, llvm_ty, name.as_ptr())
            }
            Op::Call { callee, args } => {
                let mut llvm_args: Vec<LLVMValueRef> = args
                    .iter()
                    .map(|a| self.codegen_val(a))
                    .collect::<Result<_, _>>()?;

                let name = CString::new("call_tmp").unwrap();

                // Handle direct calls to global/user/standard functions
                let mut resolved_func = None;
                if let Value::Global(global_id) = callee {
                    let name = self.global_names.get(global_id).cloned();
                    if let Some(n) = name {
                        resolved_func = self.resolve_function_by_name(&n);
                    }
                } else if let Value::Temp(temp_id) = callee {
                    let mut name = None;
                    if let Some(global_id) = self.temp_to_global.get(temp_id) {
                        name = self.global_names.get(global_id).cloned();
                    }
                    if let Some(n) = name {
                        resolved_func = self.resolve_function_by_name(&n);
                    }
                }

                if let Some((llvm_func, is_runtime)) = resolved_func {
                    let fn_ty = LLVMGlobalGetValueType(llvm_func);
                    if is_runtime {
                        // All runtime helpers consume/return boxed TsValue (i8*)
                        let mut boxed_args: Vec<LLVMValueRef> = Vec::new();
                        for a in llvm_args {
                            boxed_args.push(self.box_val(a)?);
                        }
                        LLVMBuildCall2(
                            self.ctx.builder,
                            fn_ty,
                            llvm_func,
                            boxed_args.as_mut_ptr(),
                            boxed_args.len() as u32,
                            name.as_ptr(),
                        )
                    } else {
                        // Standard user function call
                        LLVMBuildCall2(
                            self.ctx.builder,
                            fn_ty,
                            llvm_func,
                            llvm_args.as_mut_ptr(),
                            llvm_args.len() as u32,
                            name.as_ptr(),
                        )
                    }
                } else {
                    // Fallback to dynamic call
                    let func_ptr = self.codegen_val(callee)?;
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
            }
            Op::Phi(incoming) => {
                let llvm_ty = to_llvm_type(self.ctx.context, &inst.ty);
                let phi_name = CString::new("phi_tmp").unwrap();
                let phi_val = LLVMBuildPhi(self.ctx.builder, llvm_ty, phi_name.as_ptr());
                self.phi_resolutions.push((phi_val, incoming.clone()));
                phi_val
            }
            Op::FieldLoad { base, field } => {
                let base_val = self.codegen_val(base)?;
                let boxed_base = self.box_val(base_val)?;
                let field_cstr = CString::new(field.as_str()).unwrap();
                let field_val = LLVMBuildGlobalStringPtr(
                    self.ctx.builder,
                    field_cstr.as_ptr(),
                    CString::new("field_name").unwrap().as_ptr(),
                );
                let fn_val = self.get_or_declare_runtime_fn(
                    "ts_struct_get",
                    i8_ptr_ty,
                    &[i8_ptr_ty, i8_ptr_ty],
                );
                LLVMBuildCall2(
                    self.ctx.builder,
                    LLVMTypeOf(fn_val),
                    fn_val,
                    [boxed_base, field_val].as_mut_ptr(),
                    2,
                    CString::new("field_load").unwrap().as_ptr(),
                )
            }
            Op::FieldStore { base, field, value } => {
                let base_val = self.codegen_val(base)?;
                let val_val = self.codegen_val(value)?;
                let boxed_base = self.box_val(base_val)?;
                let boxed_val = self.box_val(val_val)?;
                let field_cstr = CString::new(field.as_str()).unwrap();
                let field_val = LLVMBuildGlobalStringPtr(
                    self.ctx.builder,
                    field_cstr.as_ptr(),
                    CString::new("field_name").unwrap().as_ptr(),
                );
                let fn_val = self.get_or_declare_runtime_fn(
                    "ts_struct_set",
                    LLVMVoidTypeInContext(context),
                    &[i8_ptr_ty, i8_ptr_ty, i8_ptr_ty],
                );
                LLVMBuildCall2(
                    self.ctx.builder,
                    LLVMTypeOf(fn_val),
                    fn_val,
                    [boxed_base, field_val, boxed_val].as_mut_ptr(),
                    3,
                    CString::new("").unwrap().as_ptr(),
                );
                return Ok(());
            }
            Op::IndexLoad { base, index } => {
                let base_val = self.codegen_val(base)?;
                let index_val = self.codegen_val(index)?;
                let boxed_base = self.box_val(base_val)?;
                let boxed_index = self.box_val(index_val)?;
                let fn_val = self.get_or_declare_runtime_fn(
                    "ts_index_get",
                    i8_ptr_ty,
                    &[i8_ptr_ty, i8_ptr_ty],
                );
                LLVMBuildCall2(
                    self.ctx.builder,
                    LLVMTypeOf(fn_val),
                    fn_val,
                    [boxed_base, boxed_index].as_mut_ptr(),
                    2,
                    CString::new("index_load").unwrap().as_ptr(),
                )
            }
            Op::IndexStore { base, index, value } => {
                let base_val = self.codegen_val(base)?;
                let index_val = self.codegen_val(index)?;
                let val_val = self.codegen_val(value)?;
                let boxed_base = self.box_val(base_val)?;
                let boxed_index = self.box_val(index_val)?;
                let boxed_val = self.box_val(val_val)?;
                let fn_val = self.get_or_declare_runtime_fn(
                    "ts_index_set",
                    LLVMVoidTypeInContext(context),
                    &[i8_ptr_ty, i8_ptr_ty, i8_ptr_ty],
                );
                LLVMBuildCall2(
                    self.ctx.builder,
                    LLVMTypeOf(fn_val),
                    fn_val,
                    [boxed_base, boxed_index, boxed_val].as_mut_ptr(),
                    3,
                    CString::new("").unwrap().as_ptr(),
                );
                return Ok(());
            }
            Op::MakeStruct { name, fields } => {
                let name_cstr = CString::new(name.as_str()).unwrap();
                let name_ptr = LLVMBuildGlobalStringPtr(
                    self.ctx.builder,
                    name_cstr.as_ptr(),
                    CString::new("struct_name").unwrap().as_ptr(),
                );
                let fn_val =
                    self.get_or_declare_runtime_fn("ts_alloc_struct", i8_ptr_ty, &[i8_ptr_ty]);
                let struct_val = LLVMBuildCall2(
                    self.ctx.builder,
                    LLVMTypeOf(fn_val),
                    fn_val,
                    [name_ptr].as_mut_ptr(),
                    1,
                    CString::new("struct_alloc").unwrap().as_ptr(),
                );

                for (field_name, field_val) in fields {
                    let val_val = self.codegen_val(field_val)?;
                    let boxed_val = self.box_val(val_val)?;
                    let f_name_cstr = CString::new(field_name.as_str()).unwrap();
                    let f_name_ptr = LLVMBuildGlobalStringPtr(
                        self.ctx.builder,
                        f_name_cstr.as_ptr(),
                        CString::new("field_name").unwrap().as_ptr(),
                    );
                    let fn_set = self.get_or_declare_runtime_fn(
                        "ts_struct_set",
                        LLVMVoidTypeInContext(context),
                        &[i8_ptr_ty, i8_ptr_ty, i8_ptr_ty],
                    );
                    LLVMBuildCall2(
                        self.ctx.builder,
                        LLVMTypeOf(fn_set),
                        fn_set,
                        [struct_val, f_name_ptr, boxed_val].as_mut_ptr(),
                        3,
                        CString::new("").unwrap().as_ptr(),
                    );
                }
                struct_val
            }
            Op::MakeModel { name, fields } => {
                let name_cstr = CString::new(name.as_str()).unwrap();
                let name_ptr = LLVMBuildGlobalStringPtr(
                    self.ctx.builder,
                    name_cstr.as_ptr(),
                    CString::new("model_name").unwrap().as_ptr(),
                );
                let fn_val =
                    self.get_or_declare_runtime_fn("ts_alloc_model", i8_ptr_ty, &[i8_ptr_ty]);
                let model_val = LLVMBuildCall2(
                    self.ctx.builder,
                    LLVMTypeOf(fn_val),
                    fn_val,
                    [name_ptr].as_mut_ptr(),
                    1,
                    CString::new("model_alloc").unwrap().as_ptr(),
                );

                for (field_name, field_val) in fields {
                    let val_val = self.codegen_val(field_val)?;
                    let boxed_val = self.box_val(val_val)?;
                    let f_name_cstr = CString::new(field_name.as_str()).unwrap();
                    let f_name_ptr = LLVMBuildGlobalStringPtr(
                        self.ctx.builder,
                        f_name_cstr.as_ptr(),
                        CString::new("field_name").unwrap().as_ptr(),
                    );
                    let fn_set = self.get_or_declare_runtime_fn(
                        "ts_struct_set",
                        LLVMVoidTypeInContext(context),
                        &[i8_ptr_ty, i8_ptr_ty, i8_ptr_ty],
                    );
                    LLVMBuildCall2(
                        self.ctx.builder,
                        LLVMTypeOf(fn_set),
                        fn_set,
                        [model_val, f_name_ptr, boxed_val].as_mut_ptr(),
                        3,
                        CString::new("").unwrap().as_ptr(),
                    );
                }
                model_val
            }
            Op::MakeEnum {
                name,
                variant,
                value,
            } => {
                let name_cstr = CString::new(name.as_str()).unwrap();
                let name_ptr = LLVMBuildGlobalStringPtr(
                    self.ctx.builder,
                    name_cstr.as_ptr(),
                    CString::new("enum_name").unwrap().as_ptr(),
                );
                let var_cstr = CString::new(variant.as_str()).unwrap();
                let var_ptr = LLVMBuildGlobalStringPtr(
                    self.ctx.builder,
                    var_cstr.as_ptr(),
                    CString::new("enum_variant").unwrap().as_ptr(),
                );
                let payload_ptr = if let Some(v) = value {
                    let val_val = self.codegen_val(v)?;
                    self.box_val(val_val)?
                } else {
                    LLVMConstNull(i8_ptr_ty)
                };
                let fn_val = self.get_or_declare_runtime_fn(
                    "ts_alloc_enum",
                    i8_ptr_ty,
                    &[i8_ptr_ty, i8_ptr_ty, i8_ptr_ty],
                );
                LLVMBuildCall2(
                    self.ctx.builder,
                    LLVMTypeOf(fn_val),
                    fn_val,
                    [name_ptr, var_ptr, payload_ptr].as_mut_ptr(),
                    3,
                    CString::new("enum_alloc").unwrap().as_ptr(),
                )
            }
            Op::MakeList(elements) => {
                let fn_val = self.get_or_declare_runtime_fn("ts_alloc_list", i8_ptr_ty, &[]);
                let list_val = LLVMBuildCall2(
                    self.ctx.builder,
                    LLVMTypeOf(fn_val),
                    fn_val,
                    [].as_mut_ptr(),
                    0,
                    CString::new("list_alloc").unwrap().as_ptr(),
                );

                for item in elements {
                    let val_val = self.codegen_val(item)?;
                    let boxed_val = self.box_val(val_val)?;
                    let fn_push = self.get_or_declare_runtime_fn(
                        "ts_list_push",
                        LLVMVoidTypeInContext(context),
                        &[i8_ptr_ty, i8_ptr_ty],
                    );
                    LLVMBuildCall2(
                        self.ctx.builder,
                        LLVMTypeOf(fn_push),
                        fn_push,
                        [list_val, boxed_val].as_mut_ptr(),
                        2,
                        CString::new("").unwrap().as_ptr(),
                    );
                }
                list_val
            }
            Op::MakeMap(entries) => {
                let fn_val = self.get_or_declare_runtime_fn("ts_alloc_map", i8_ptr_ty, &[]);
                let map_val = LLVMBuildCall2(
                    self.ctx.builder,
                    LLVMTypeOf(fn_val),
                    fn_val,
                    [].as_mut_ptr(),
                    0,
                    CString::new("map_alloc").unwrap().as_ptr(),
                );

                for (k, v) in entries {
                    let k_val = self.codegen_val(k)?;
                    let v_val = self.codegen_val(v)?;
                    let boxed_k = self.box_val(k_val)?;
                    let boxed_v = self.box_val(v_val)?;
                    let fn_set = self.get_or_declare_runtime_fn(
                        "ts_map_set",
                        LLVMVoidTypeInContext(context),
                        &[i8_ptr_ty, i8_ptr_ty, i8_ptr_ty],
                    );
                    LLVMBuildCall2(
                        self.ctx.builder,
                        LLVMTypeOf(fn_set),
                        fn_set,
                        [map_val, boxed_k, boxed_v].as_mut_ptr(),
                        3,
                        CString::new("").unwrap().as_ptr(),
                    );
                }
                map_val
            }
            Op::Cast { value, target_type } => {
                let _double_ty = double_ty; // avoid unused warning
                let val_val = self.codegen_val(value)?;
                let boxed_val = self.box_val(val_val)?;
                let tag = match target_type {
                    IRType::Bool => 1,
                    IRType::Int64 => 2,
                    IRType::Float64 => 3,
                    IRType::String => 4,
                    _ => 0,
                };
                let fn_val =
                    self.get_or_declare_runtime_fn("ts_cast", i8_ptr_ty, &[i8_ptr_ty, i32_ty]);
                LLVMBuildCall2(
                    self.ctx.builder,
                    LLVMTypeOf(fn_val),
                    fn_val,
                    [boxed_val, LLVMConstInt(i32_ty, tag, 0)].as_mut_ptr(),
                    2,
                    CString::new("cast").unwrap().as_ptr(),
                )
            }
            Op::Try { catch_block, catch_var } => {
                let i8_ptr_ty = LLVMPointerType(LLVMInt8TypeInContext(context), 0);
                let fn_push = self.get_or_declare_runtime_fn("ts_try_push", i8_ptr_ty, &[]);
                let buf_ptr = LLVMBuildCall2(
                    self.ctx.builder,
                    LLVMTypeOf(fn_push),
                    fn_push,
                    [].as_mut_ptr(),
                    0,
                    CString::new("jmp_buf").unwrap().as_ptr(),
                );

                let setjmp_fn = self.get_or_declare_runtime_fn(
                    #[cfg(target_os = "windows")]
                    "_setjmp",
                    #[cfg(not(target_os = "windows"))]
                    "setjmp",
                    i32_ty,
                    &[i8_ptr_ty],
                );

                let res = LLVMBuildCall2(
                    self.ctx.builder,
                    LLVMTypeOf(setjmp_fn),
                    setjmp_fn,
                    [buf_ptr].as_mut_ptr(),
                    1,
                    CString::new("setjmp_res").unwrap().as_ptr(),
                );

                let zero = LLVMConstInt(i32_ty, 0, 0);
                let is_exception = LLVMBuildICmp(
                    self.ctx.builder,
                    llvm_sys::LLVMIntPredicate::LLVMIntNE,
                    res,
                    zero,
                    CString::new("is_exception").unwrap().as_ptr(),
                );

                // We need to split the block here because setjmp acts as a conditional branch point
                let current_block = LLVMGetInsertBlock(self.ctx.builder);
                let func = LLVMGetBasicBlockParent(current_block);

                let cont_block = LLVMAppendBasicBlockInContext(
                    context,
                    func,
                    CString::new("try_continue").unwrap().as_ptr(),
                );

                let catch_target = self.ctx.get_block(*catch_block).unwrap();

                // We create a dispatch block for the exception path
                let dispatch_block = LLVMAppendBasicBlockInContext(
                    context,
                    func,
                    CString::new("try_dispatch").unwrap().as_ptr(),
                );

                LLVMBuildCondBr(self.ctx.builder, is_exception, dispatch_block, cont_block);

                // exception path
                LLVMPositionBuilderAtEnd(self.ctx.builder, dispatch_block);

                // Clean up the jmp_buf since we arrived here via longjmp and ts_try_pop wasn't called
                let fn_free_buf = self.get_or_declare_runtime_fn("ts_try_free", LLVMVoidTypeInContext(context), &[i8_ptr_ty]);
                LLVMBuildCall2(
                    self.ctx.builder,
                    LLVMTypeOf(fn_free_buf),
                    fn_free_buf,
                    [buf_ptr].as_mut_ptr(),
                    1,
                    CString::new("").unwrap().as_ptr(),
                );

                let fn_get_ex = self.get_or_declare_runtime_fn("ts_get_exception", i8_ptr_ty, &[]);
                let ex_val = LLVMBuildCall2(
                    self.ctx.builder,
                    LLVMTypeOf(fn_get_ex),
                    fn_get_ex,
                    [].as_mut_ptr(),
                    0,
                    CString::new("ex_val").unwrap().as_ptr(),
                );

                if let Some(target) = self.ctx.get_local(*catch_var) {
                    LLVMBuildStore(self.ctx.builder, ex_val, target);
                }
                LLVMBuildBr(self.ctx.builder, catch_target);

                // continue path
                LLVMPositionBuilderAtEnd(self.ctx.builder, cont_block);

                return Ok(());
            }
            Op::EndTry => {
                let fn_pop = self.get_or_declare_runtime_fn("ts_try_pop", LLVMVoidTypeInContext(context), &[]);
                LLVMBuildCall2(
                    self.ctx.builder,
                    LLVMTypeOf(fn_pop),
                    fn_pop,
                    [].as_mut_ptr(),
                    0,
                    CString::new("").unwrap().as_ptr(),
                );
                return Ok(());
            }
            Op::MakeDslBlock { .. } | Op::NoOp => return Ok(()),
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
                .get_local(*id)
                .ok_or_else(|| format!("LocalId {:?} not found", id)),
            Value::Global(id) => self
                .ctx
                .get_global(*id)
                .ok_or_else(|| format!("GlobalId {:?} not found", id)),
            Value::Const(lit) => self.codegen_literal(lit),
            Value::Null | Value::DslBlock { .. } => Ok(LLVMConstNull(LLVMPointerType(
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
            LiteralVal::None => Ok(LLVMConstNull(LLVMPointerType(
                LLVMInt8TypeInContext(self.ctx.context),
                0,
            ))),
        }
    }

    unsafe fn get_or_declare_runtime_fn(
        &mut self,
        name: &str,
        ret_ty: LLVMTypeRef,
        arg_tys: &[LLVMTypeRef],
    ) -> LLVMValueRef {
        let name_cstr = CString::new(name).unwrap();
        let existing = LLVMGetNamedFunction(self.ctx.module, name_cstr.as_ptr());
        if !existing.is_null() {
            return existing;
        }
        let mut args = arg_tys.to_vec();
        let func_ty = LLVMFunctionType(ret_ty, args.as_mut_ptr(), args.len() as u32, 0);
        LLVMAddFunction(self.ctx.module, name_cstr.as_ptr(), func_ty)
    }

    unsafe fn box_val(&mut self, val: LLVMValueRef) -> Result<LLVMValueRef, String> {
        let context = self.ctx.context;
        let i8_ptr_ty = LLVMPointerType(LLVMInt8TypeInContext(context), 0);
        let ty = LLVMTypeOf(val);
        let kind = LLVMGetTypeKind(ty);
        match kind {
            llvm_sys::LLVMTypeKind::LLVMPointerTypeKind => Ok(val),
            llvm_sys::LLVMTypeKind::LLVMIntegerTypeKind => {
                let width = LLVMGetIntTypeWidth(ty);
                if width == 1 {
                    let fn_val = self.get_or_declare_runtime_fn(
                        "ts_alloc_bool",
                        i8_ptr_ty,
                        &[LLVMInt1TypeInContext(context)],
                    );
                    let mut args = [val];
                    Ok(LLVMBuildCall2(
                        self.ctx.builder,
                        LLVMTypeOf(fn_val),
                        fn_val,
                        args.as_mut_ptr(),
                        1,
                        CString::new("box_bool").unwrap().as_ptr(),
                    ))
                } else {
                    let fn_val = self.get_or_declare_runtime_fn(
                        "ts_alloc_int",
                        i8_ptr_ty,
                        &[LLVMInt64TypeInContext(context)],
                    );
                    let val_i64 = if width != 64 {
                        LLVMBuildSExt(
                            self.ctx.builder,
                            val,
                            LLVMInt64TypeInContext(context),
                            CString::new("sext").unwrap().as_ptr(),
                        )
                    } else {
                        val
                    };
                    let mut args = [val_i64];
                    Ok(LLVMBuildCall2(
                        self.ctx.builder,
                        LLVMTypeOf(fn_val),
                        fn_val,
                        args.as_mut_ptr(),
                        1,
                        CString::new("box_int").unwrap().as_ptr(),
                    ))
                }
            }
            llvm_sys::LLVMTypeKind::LLVMDoubleTypeKind => {
                let fn_val = self.get_or_declare_runtime_fn(
                    "ts_alloc_float",
                    i8_ptr_ty,
                    &[LLVMDoubleTypeInContext(context)],
                );
                let mut args = [val];
                Ok(LLVMBuildCall2(
                    self.ctx.builder,
                    LLVMTypeOf(fn_val),
                    fn_val,
                    args.as_mut_ptr(),
                    1,
                    CString::new("box_float").unwrap().as_ptr(),
                ))
            }
            _ => Err(format!("Cannot box value of type kind {:?}", kind)),
        }
    }

    unsafe fn coerce_to_float(&mut self, val: LLVMValueRef) -> Result<LLVMValueRef, String> {
        let context = self.ctx.context;
        let ty = LLVMTypeOf(val);
        let kind = LLVMGetTypeKind(ty);
        match kind {
            llvm_sys::LLVMTypeKind::LLVMDoubleTypeKind => Ok(val),
            llvm_sys::LLVMTypeKind::LLVMIntegerTypeKind => Ok(LLVMBuildSIToFP(
                self.ctx.builder,
                val,
                LLVMDoubleTypeInContext(context),
                CString::new("sitofp").unwrap().as_ptr(),
            )),
            _ => Err("Cannot coerce to float".to_string()),
        }
    }

    unsafe fn resolve_function_by_name(&mut self, name: &str) -> Option<(LLVMValueRef, bool)> {
        let name_cstr = CString::new(name).unwrap();

        // 1. Check user-defined functions
        let user_func = LLVMGetNamedFunction(self.ctx.module, name_cstr.as_ptr());
        if !user_func.is_null() && !name.starts_with("ts_") {
            return Some((user_func, false));
        }

        // 2. Check standard built-ins mapping
        let mapped_name = match name {
            "say" => Some("ts_say"),
            "ask" => Some("ts_ask"),
            "len" => Some("ts_len"),
            "range" => Some("ts_range"),
            "abs" => Some("ts_math_abs"),
            "sin" => Some("ts_math_sin"),
            "cos" => Some("ts_math_cos"),
            "tan" => Some("ts_math_tan"),
            "log" => Some("ts_math_log"),
            "exp" => Some("ts_math_exp"),
            "sqrt" => Some("ts_math_sqrt"),
            _ => None,
        };

        if let Some(m_name) = mapped_name {
            let context = self.ctx.context;
            let i8_ptr_ty = LLVMPointerType(LLVMInt8TypeInContext(context), 0);
            let ret_ty = if m_name == "ts_say" {
                LLVMVoidTypeInContext(context)
            } else {
                i8_ptr_ty
            };
            let arg_tys = if m_name == "ts_range" {
                vec![i8_ptr_ty, i8_ptr_ty]
            } else {
                vec![i8_ptr_ty]
            };
            let fn_val = self.get_or_declare_runtime_fn(m_name, ret_ty, &arg_tys);
            return Some((fn_val, true));
        }

        None
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
                LiteralVal::None => IRType::Any,
            },
            _ => IRType::Any,
        }
    }
}
