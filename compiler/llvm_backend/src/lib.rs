//! # TechScript LLVM Backend Crate
//!
//! Entry point for LLVM native code compilation pipelines.

pub mod codegen;
pub mod context;
pub mod type_map;

use std::path::Path;
use techscript_ir::Module;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LLVMCodegenError {
    #[error("Target error: {0}")]
    TargetError(String),

    #[error("Linker or file error: {0}")]
    FileError(String),

    #[error("LLVM compilation error: {0}")]
    CompilationError(String),
}

#[cfg(feature = "llvm")]
use llvm_sys::target_machine::LLVMCodeGenOptLevel;

#[cfg(not(feature = "llvm"))]
#[derive(Debug, Clone, Copy)]
pub enum LLVMCodeGenOptLevel {
    LLVMCodeGenLevelNone,
    LLVMCodeGenLevelLess,
    LLVMCodeGenLevelDefault,
    LLVMCodeGenLevelAggressive,
}

#[derive(Debug, Clone)]
pub struct LLVMBackendOptions {
    pub target_triple: String,
    pub opt_level: LLVMCodeGenOptLevel,
    pub debug_symbols: bool,
}

pub struct LLVMBackend;

impl LLVMBackend {
    /// Compiles a TechScript IR Module to a native object file (`.o` or `.obj`) at the given output path.
    #[cfg(feature = "llvm")]
    pub fn compile(
        ir_module: &Module,
        options: &LLVMBackendOptions,
        out_path: &Path,
    ) -> Result<(), LLVMCodegenError> {
        use crate::codegen::CodegenEngine;
        use crate::context::CodegenContext;
        use llvm_sys::target::*;
        use llvm_sys::target_machine::*;
        use std::ffi::{CStr, CString};

        unsafe {
            // 1. Initialize LLVM targets
            LLVM_InitializeAllTargetInfos();
            LLVM_InitializeAllTargets();
            LLVM_InitializeAllTargetMCs();
            LLVM_InitializeAllAsmPrinters();
            LLVM_InitializeAllAsmParsers();

            // 2. Set up context and module
            let mut ctx = CodegenContext::new(&ir_module.name);
            let mut engine = CodegenEngine::new(&mut ctx);

            engine
                .compile_module(ir_module)
                .map_err(|e| LLVMCodegenError::CompilationError(e))?;

            // 3. Resolve Target Triple
            let triple_cstr = CString::new(options.target_triple.as_str()).unwrap();
            let mut target = std::ptr::null_mut();
            let mut err_msg = std::ptr::null_mut();

            if LLVMGetTargetFromTriple(triple_cstr.as_ptr(), &mut target, &mut err_msg) != 0 {
                let err_str = CStr::from_ptr(err_msg).to_string_lossy().into_owned();
                libc::free(err_msg as *mut libc::c_void);
                return Err(LLVMCodegenError::TargetError(err_str));
            }

            // 4. Create Target Machine
            let cpu = CString::new("generic").unwrap();
            let features = CString::new("").unwrap();
            let target_machine = LLVMCreateTargetMachine(
                target,
                triple_cstr.as_ptr(),
                cpu.as_ptr(),
                features.as_ptr(),
                options.opt_level,
                LLVMRelocMode::LLVMRelocPIC,
                LLVMCodeModel::LLVMCodeModelDefault,
            );

            if target_machine.is_null() {
                return Err(LLVMCodegenError::TargetError(
                    "Failed to create LLVMTargetMachineRef".to_string(),
                ));
            }

            // Set module target triple and data layout
            let layout = LLVMCreateTargetDataLayout(target_machine);
            let layout_str = LLVMCopyStringRepOfTargetData(layout);
            LLVMSetDataLayout(ctx.module, layout_str);
            LLVMSetTarget(ctx.module, triple_cstr.as_ptr());

            // 5. Emit object file
            let out_str = CString::new(out_path.to_string_lossy().to_string()).unwrap();
            let mut emit_err = std::ptr::null_mut();

            let status = LLVMTargetMachineEmitToFile(
                target_machine,
                ctx.module,
                out_str.as_ptr() as *mut libc::c_char,
                LLVMCodeGenFileType::LLVMObjectFile,
                &mut emit_err,
            );

            // Clean up target layouts and machines
            LLVMDisposeTargetString(layout_str);
            LLVMDisposeTargetData(layout);
            LLVMDisposeTargetMachine(target_machine);

            if status != 0 {
                let err_str = CStr::from_ptr(emit_err).to_string_lossy().into_owned();
                libc::free(emit_err as *mut libc::c_void);
                return Err(LLVMCodegenError::FileError(err_str));
            }

            Ok(())
        }
    }

    /// Stub fallback compile function when LLVM backend features are disabled at compile-time.
    #[cfg(not(feature = "llvm"))]
    pub fn compile(
        _ir_module: &Module,
        _options: &LLVMBackendOptions,
        _out_path: &Path,
    ) -> Result<(), LLVMCodegenError> {
        Err(LLVMCodegenError::CompilationError(
            "LLVM backend was compiled without LLVM support. Enable the 'llvm' feature and configure LLVM on the host.".to_string()
        ))
    }
}
