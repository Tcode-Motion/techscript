//! # TechScript LLVM Backend — ORC JIT Engine
//!
//! Exposes the LLVM ORC JIT compilation and dynamic execution environment.

#![cfg(feature = "llvm")]

use llvm_sys::orc2::lljit::*;
use llvm_sys::orc2::*;
use std::collections::HashMap;
use std::ffi::CString;
use std::ptr;

use crate::codegen::CodegenEngine;
use crate::context::CodegenContext;
use crate::LLVMBackendOptions;

pub struct LLVMJitEngine {
    jit: LLVMOrcLLJITRef,
    ts_ctx: LLVMOrcThreadSafeContextRef,
    cache: HashMap<String, u64>,
}

impl LLVMJitEngine {
    /// Creates a new LLVMJitEngine instance.
    /// # Safety
    ///
    /// Caller must ensure LLVM context is valid.
    pub unsafe fn new() -> Result<Self, String> {
        let mut jit = ptr::null_mut();
        let builder = LLVMOrcCreateLLJITBuilder();
        let err = LLVMOrcCreateLLJIT(&mut jit, builder);
        if !err.is_null() {
            return Err("Failed to create LLVMOrcLLJITRef".to_string());
        }

        let ts_ctx = LLVMOrcCreateNewThreadSafeContext();

        Ok(Self {
            jit,
            ts_ctx,
            cache: HashMap::new(),
        })
    }

    /// Compiles a TechScript IR Module to JIT memory.
    /// # Safety
    ///
    /// Caller must ensure LLVM context is valid.
    pub unsafe fn compile(
        &mut self,
        ir_module: &techscript_ir::Module,
        _options: &LLVMBackendOptions,
    ) -> Result<(), String> {
        // 1. Build LLVM IR Module
        let mut ctx = CodegenContext::new(&ir_module.name);
        let mut engine = CodegenEngine::new(&mut ctx);
        engine.compile_module(ir_module)?;

        // 2. Set target triple and data layout matching LLJIT
        let jd = LLVMOrcLLJITGetMainJITDylib(self.jit);
        let layout_str = LLVMOrcLLJITGetDataLayoutStr(self.jit);
        llvm_sys::core::LLVMSetDataLayout(ctx.module, layout_str);
        // Do not dispose layout_str directly as LLVMOrcLLJITGetDataLayoutStr returns a borrowed const char*
        // string tied to the DataLayout of the JIT instance.

        // 3. Set host target triple
        let _host_triple = LLVMOrcLLJITGetExecutionSession(self.jit); // session triple fallback
                                                                      // We can just keep the default LLVM target triple

        // 4. Wrap Module in ThreadSafeModule
        let tsm = LLVMOrcCreateNewThreadSafeModule(ctx.module, self.ts_ctx);

        // Relinquish ownership of ctx.module because LLVMOrcCreateThreadSafeModule takes it
        ctx.module = ptr::null_mut();

        // 5. Add Module to JIT Dylib
        let err = LLVMOrcLLJITAddLLVMIRModule(self.jit, jd, tsm);
        if !err.is_null() {
            LLVMOrcDisposeThreadSafeModule(tsm);
            return Err("Failed to add LLVM IR module to JIT".to_string());
        }

        Ok(())
    }

    /// Looks up a function symbol by name.
    /// # Safety
    ///
    /// Caller must ensure LLVM context is valid.
    pub unsafe fn lookup(&mut self, name: &str) -> Result<u64, String> {
        if let Some(&addr) = self.cache.get(name) {
            return Ok(addr);
        }

        let name_cstr = CString::new(name).unwrap();
        let mut addr = 0;
        let err = LLVMOrcLLJITLookup(self.jit, &mut addr, name_cstr.as_ptr());
        if !err.is_null() {
            return Err(format!("JIT function symbol '{}' not found", name));
        }

        self.cache.insert(name.to_string(), addr);
        Ok(addr)
    }

    /// Executes the JIT-compiled main function and returns its result (if integer).
    /// # Safety
    ///
    /// Caller must ensure LLVM context is valid.
    pub unsafe fn execute(&mut self, func_name: &str) -> Result<i64, String> {
        let addr = self.lookup(func_name)?;
        let func: extern "C" fn() -> i64 = std::mem::transmute(addr);
        Ok(func())
    }

    /// Clears the function cache and reloads the engine (for hot reload support).
    /// # Safety
    ///
    /// Caller must ensure LLVM context is valid.
    pub unsafe fn hot_reload(&mut self) {
        self.cache.clear();
    }
}

impl Drop for LLVMJitEngine {
    fn drop(&mut self) {
        unsafe {
            LLVMOrcDisposeLLJIT(self.jit);
            LLVMOrcDisposeThreadSafeContext(self.ts_ctx);
        }
    }
}
