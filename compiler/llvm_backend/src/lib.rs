//! # TechScript LLVM Backend Crate
//!
//! Entry point for LLVM native code compilation pipelines and ORC JIT.

pub mod codegen;
pub mod context;
pub mod type_map;

#[cfg(feature = "llvm")]
pub mod jit;

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
pub use llvm_sys::target_machine::LLVMCodeGenOptLevel;

#[cfg(not(feature = "llvm"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        Self::emit_to_file(ir_module, options, out_path, llvm_sys::target_machine::LLVMCodeGenFileType::LLVMObjectFile)
    }

    /// Compiles a TechScript IR Module to a native assembly file (`.s` or `.asm`) at the given output path.
    #[cfg(feature = "llvm")]
    pub fn emit_asm(
        ir_module: &Module,
        options: &LLVMBackendOptions,
        out_path: &Path,
    ) -> Result<(), LLVMCodegenError> {
        Self::emit_to_file(ir_module, options, out_path, llvm_sys::target_machine::LLVMCodeGenFileType::LLVMAssemblyFile)
    }

    /// Emits textual LLVM IR representation (`.ll`) at the given output path.
    #[cfg(feature = "llvm")]
    pub fn emit_llvm_ir(
        ir_module: &Module,
        out_path: &Path,
    ) -> Result<(), LLVMCodegenError> {
        use crate::codegen::CodegenEngine;
        use crate::context::CodegenContext;
        use std::ffi::CString;

        unsafe {
            let mut ctx = CodegenContext::new(&ir_module.name);
            let mut engine = CodegenEngine::new(&mut ctx);
            engine
                .compile_module(ir_module)
                .map_err(|e| LLVMCodegenError::CompilationError(e))?;

            let out_str = CString::new(out_path.to_string_lossy().to_string()).unwrap();
            let mut err_msg = std::ptr::null_mut();
            let status = llvm_sys::core::LLVMPrintModuleToFile(ctx.module, out_str.as_ptr(), &mut err_msg);
            if status != 0 {
                let err_str = std::ffi::CStr::from_ptr(err_msg).to_string_lossy().into_owned();
                libc::free(err_msg as *mut libc::c_void);
                return Err(LLVMCodegenError::FileError(err_str));
            }
            Ok(())
        }
    }

    #[cfg(feature = "llvm")]
    unsafe fn emit_to_file(
        ir_module: &Module,
        options: &LLVMBackendOptions,
        out_path: &Path,
        file_type: llvm_sys::target_machine::LLVMCodeGenFileType,
    ) -> Result<(), LLVMCodegenError> {
        use crate::codegen::CodegenEngine;
        use crate::context::CodegenContext;
        use llvm_sys::target::*;
        use llvm_sys::target_machine::*;
        use llvm_sys::transforms::pass_manager_builder::*;
        use std::ffi::{CStr, CString};

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

        // 3. Resolve Target Triple & Host CPU Features
        let triple_cstr = CString::new(options.target_triple.as_str()).unwrap();
        let mut target = std::ptr::null_mut();
        let mut err_msg = std::ptr::null_mut();

        if LLVMGetTargetFromTriple(triple_cstr.as_ptr(), &mut target, &mut err_msg) != 0 {
            let err_str = CStr::from_ptr(err_msg).to_string_lossy().into_owned();
            libc::free(err_msg as *mut libc::c_void);
            return Err(LLVMCodegenError::TargetError(err_str));
        }

        // Host CPU and Feature detection
        let host_cpu = LLVMGetHostCPUName();
        let host_features = LLVMGetHostCPUFeatures();

        // 4. Create Target Machine
        let target_machine = LLVMCreateTargetMachine(
            target,
            triple_cstr.as_ptr(),
            host_cpu,
            host_features,
            options.opt_level,
            LLVMRelocMode::LLVMRelocPIC,
            LLVMCodeModel::LLVMCodeModelDefault,
        );

        libc::free(host_cpu as *mut libc::c_void);
        libc::free(host_features as *mut libc::c_void);

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

        // 5. Setup Pass Manager Optimizations
        let opt_level_u32 = match options.opt_level {
            LLVMCodeGenOptLevel::LLVMCodeGenLevelNone => 0,
            LLVMCodeGenOptLevel::LLVMCodeGenLevelLess => 1,
            LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault => 2,
            LLVMCodeGenOptLevel::LLVMCodeGenLevelAggressive => 3,
        };

        let pm_builder = LLVMPassManagerBuilderCreate();
        LLVMPassManagerBuilderSetOptLevel(pm_builder, opt_level_u32);
        LLVMPassManagerBuilderSetSizeLevel(pm_builder, if opt_level_u32 == 2 { 1 } else { 0 }); // Os equivalent
        LLVMPassManagerBuilderUseInlinerWithThreshold(pm_builder, if opt_level_u32 > 0 { 275 } else { 0 });

        let mpm = llvm_sys::core::LLVMCreatePassManager();
        LLVMPassManagerBuilderPopulateModulePassManager(pm_builder, mpm);

        let fpm = llvm_sys::core::LLVMCreateFunctionPassManagerForModule(ctx.module);
        LLVMPassManagerBuilderPopulateFunctionPassManager(pm_builder, fpm);

        LLVMPassManagerBuilderDispose(pm_builder);

        // Run function-level optimizations
        llvm_sys::core::LLVMInitializeFunctionPassManager(fpm);
        let mut func = llvm_sys::core::LLVMGetFirstFunction(ctx.module);
        while !func.is_null() {
            llvm_sys::core::LLVMRunFunctionPassManager(fpm, func);
            func = llvm_sys::core::LLVMGetNextFunction(func);
        }
        llvm_sys::core::LLVMFinalizeFunctionPassManager(fpm);

        // Run module-level optimizations
        llvm_sys::core::LLVMRunPassManager(mpm, ctx.module);

        llvm_sys::core::LLVMDisposePassManager(fpm);
        llvm_sys::core::LLVMDisposePassManager(mpm);

        // 6. Emit target file (object or assembly)
        let out_str = CString::new(out_path.to_string_lossy().to_string()).unwrap();
        let mut emit_err = std::ptr::null_mut();

        let status = LLVMTargetMachineEmitToFile(
            target_machine,
            ctx.module,
            out_str.as_ptr() as *mut libc::c_char,
            file_type,
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

    #[cfg(not(feature = "llvm"))]
    pub fn emit_asm(
        _ir_module: &Module,
        _options: &LLVMBackendOptions,
        _out_path: &Path,
    ) -> Result<(), LLVMCodegenError> {
        Err(LLVMCodegenError::CompilationError(
            "LLVM backend was compiled without LLVM support.".to_string()
        ))
    }

    #[cfg(not(feature = "llvm"))]
    pub fn emit_llvm_ir(
        _ir_module: &Module,
        _out_path: &Path,
    ) -> Result<(), LLVMCodegenError> {
        Err(LLVMCodegenError::CompilationError(
            "LLVM backend was compiled without LLVM support.".to_string()
        ))
    }
}
