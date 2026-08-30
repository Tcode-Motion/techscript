//! # TechScript LLVM Backend — Type Mapping
//!
//! Maps TechScript IR types (`IRType`) to LLVM representation types (`LLVMTypeRef`).

#![cfg(feature = "llvm")]

use llvm_sys::core::*;
use llvm_sys::prelude::*;
use techscript_ir::types::IRType;

/// Maps a TechScript IRType to its corresponding LLVMTypeRef.
/// # Safety
///
/// Caller must ensure LLVM context is valid.
pub unsafe fn to_llvm_type(context: LLVMContextRef, ty: &IRType) -> LLVMTypeRef {
    match ty {
        IRType::Void => LLVMVoidTypeInContext(context),
        IRType::Int64 => LLVMInt64TypeInContext(context),
        IRType::Float64 => LLVMDoubleTypeInContext(context),
        IRType::Bool => LLVMInt1TypeInContext(context),

        // Strings, structures, maps, lists, and closures are managed as pointers in heap
        IRType::String => LLVMPointerType(LLVMInt8TypeInContext(context), 0),
        IRType::List => LLVMPointerType(LLVMInt8TypeInContext(context), 0),
        IRType::Map => LLVMPointerType(LLVMInt8TypeInContext(context), 0),
        IRType::Pointer => LLVMPointerType(LLVMInt8TypeInContext(context), 0),

        IRType::Struct(_) => LLVMPointerType(LLVMInt8TypeInContext(context), 0),
        IRType::Enum(_) => LLVMPointerType(LLVMInt8TypeInContext(context), 0),
        IRType::Model(_) => LLVMPointerType(LLVMInt8TypeInContext(context), 0),
        IRType::DslBlock(_) => LLVMPointerType(LLVMInt8TypeInContext(context), 0),
        IRType::Any => LLVMPointerType(LLVMInt8TypeInContext(context), 0),
    }
}
