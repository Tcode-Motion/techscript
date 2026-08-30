//! # TechScript LLVM Backend — Codegen Context
//!
//! Wraps the LLVM context, builder, module, and houses variable lookups.

#![cfg(feature = "llvm")]

use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::collections::HashMap;
use std::ffi::CString;
use techscript_ir::{BlockId, GlobalId, LocalId, ValueId};

pub struct CodegenContext {
    pub context: LLVMContextRef,
    pub module: LLVMModuleRef,
    pub builder: LLVMBuilderRef,
    pub values: HashMap<ValueId, LLVMValueRef>,
    pub globals: HashMap<GlobalId, LLVMValueRef>,
    pub locals: HashMap<LocalId, LLVMValueRef>,
    pub blocks: HashMap<BlockId, LLVMBasicBlockRef>,
}

impl CodegenContext {
    /// # Safety
    ///
    /// Caller must ensure LLVM context is valid.
    pub unsafe fn new(name: &str) -> Self {
        let context = LLVMContextCreate();
        let module =
            LLVMModuleCreateWithNameInContext(CString::new(name).unwrap().as_ptr(), context);
        let builder = LLVMCreateBuilderInContext(context);

        Self {
            context,
            module,
            builder,
            values: HashMap::new(),
            globals: HashMap::new(),
            locals: HashMap::new(),
            blocks: HashMap::new(),
        }
    }

    /// Looks up an LLVM value for the given ValueId.
    pub fn get_global(&self, id: GlobalId) -> Option<LLVMValueRef> {
        self.globals.get(&id).copied()
    }

    pub fn register_global(&mut self, id: GlobalId, val: LLVMValueRef) {
        self.globals.insert(id, val);
    }

    pub fn get_local(&self, id: LocalId) -> Option<LLVMValueRef> {
        self.locals.get(&id).copied()
    }

    pub fn register_local(&mut self, id: LocalId, val: LLVMValueRef) {
        self.locals.insert(id, val);
    }

    pub fn get_value(&self, id: ValueId) -> Option<LLVMValueRef> {
        self.values.get(&id).copied()
    }

    /// Registers a value mapping.
    pub fn register_value(&mut self, id: ValueId, val: LLVMValueRef) {
        self.values.insert(id, val);
    }

    /// Looks up an LLVM basic block for the given BlockId.
    pub fn get_block(&self, id: BlockId) -> Option<LLVMBasicBlockRef> {
        self.blocks.get(&id).copied()
    }

    /// Registers a basic block mapping.
    pub fn register_block(&mut self, id: BlockId, block: LLVMBasicBlockRef) {
        self.blocks.insert(id, block);
    }
}

impl Drop for CodegenContext {
    fn drop(&mut self) {
        unsafe {
            if !self.builder.is_null() {
                LLVMDisposeBuilder(self.builder);
            }
            if !self.module.is_null() {
                LLVMDisposeModule(self.module);
            }
            if !self.context.is_null() {
                LLVMContextDispose(self.context);
            }
        }
    }
}
