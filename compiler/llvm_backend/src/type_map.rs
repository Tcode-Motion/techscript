//! # TechScript LLVM Backend — Type Mapping
//!
//! Maps TechScript IR types (`IRType`) to LLVM representation types (`LLVMTypeRef`).

#![cfg(feature = "llvm")]

use llvm_sys::core::*;
use llvm_sys::prelude::*;
use techscript_ir::types::IRType;

/// Maps a TechScript IRType to its corresponding LLVMTypeRef.
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

        IRType::Struct(name) => {
            let struct_ty = LLVMStructCreateNamed(
                context,
                std::ffi::CString::new(name.as_str()).unwrap().as_ptr(),
            );
            // Struct body would be set dynamically or treated as opaque pointer
            LLVMPointerType(struct_ty, 0)
        }
        IRType::Enum(name) => {
            let struct_ty = LLVMStructCreateNamed(
                context,
                std::ffi::CString::new(name.as_str()).unwrap().as_ptr(),
            );
            LLVMPointerType(struct_ty, 0)
        }
        IRType::Model(name) => {
            let struct_ty = LLVMStructCreateNamed(
                context,
                std::ffi::CString::new(name.as_str()).unwrap().as_ptr(),
            );
            LLVMPointerType(struct_ty, 0)
        }

        IRType::Any => {
            // Dynamically-typed values are boxed as opaque pointers
            LLVMPointerType(LLVMInt8TypeInContext(context), 0)
        }
    }
}
