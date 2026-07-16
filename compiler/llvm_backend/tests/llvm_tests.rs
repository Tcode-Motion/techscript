use techscript_ir::types::IRType;
use techscript_ir::{Function, Module};
use techscript_llvm_backend::{LLVMBackend, LLVMBackendOptions, LLVMCodeGenOptLevel};

#[cfg(feature = "llvm")]
use techscript_llvm_backend::context::CodegenContext;
#[cfg(feature = "llvm")]
use techscript_llvm_backend::type_map::to_llvm_type;

#[test]
fn test_context_creation() {
    #[cfg(feature = "llvm")]
    unsafe {
        let ctx = CodegenContext::new("test_module");
        assert!(!ctx.context.is_null());
        assert!(!ctx.module.is_null());
        assert!(!ctx.builder.is_null());
    }
}

#[test]
fn test_type_mappings() {
    #[cfg(feature = "llvm")]
    unsafe {
        let ctx = CodegenContext::new("test_types");

        let int_ty = to_llvm_type(ctx.context, &IRType::Int64);
        assert!(!int_ty.is_null());

        let float_ty = to_llvm_type(ctx.context, &IRType::Float64);
        assert!(!float_ty.is_null());

        let bool_ty = to_llvm_type(ctx.context, &IRType::Bool);
        assert!(!bool_ty.is_null());
    }
}

#[test]
fn test_compile_module() {
    let mut module = Module::new("empty_test".to_string());

    // Add simple main function
    let main_id = techscript_ir::types::FunctionId(0);
    let main_func = Function::new(main_id, "main".to_string(), IRType::Void);
    module.functions.push(main_func);

    let temp_dir = std::env::temp_dir();
    let out_file = temp_dir.join("empty_test.o");

    let options = LLVMBackendOptions {
        target_triple: "x86_64-pc-windows-msvc".to_string(),
        opt_level: LLVMCodeGenOptLevel::LLVMCodeGenLevelNone,
        debug_symbols: false,
    };

    let res = LLVMBackend::compile(&module, &options, &out_file);

    #[cfg(not(feature = "llvm"))]
    {
        assert!(res.is_err());
        assert!(res
            .unwrap_err()
            .to_string()
            .contains("without LLVM support"));
    }

    #[cfg(feature = "llvm")]
    {
        println!("LLVM Compilation result: {:?}", res);
    }
}
