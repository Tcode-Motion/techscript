//! # TechScript VM Crate
//!
//! Stack-based bytecode execution machine for TechScript 2.0.

pub mod bytecode_loader;
pub mod debugger;
pub mod diagnostics;
pub mod error;
pub mod executor;
pub mod frame;
pub mod gc;
pub mod heap;
pub mod native;
pub mod stack;
pub mod vm;

pub use bytecode_loader::BytecodeLoader;
pub use debugger::VMDebugger;
pub use diagnostics::VMProfiler;
pub use error::VMError;
pub use frame::{CallFrame, ExceptionHandler};
pub use gc::{GarbageCollector, HeapObject};
pub use heap::VMHeap;
pub use native::NativeBridge;
pub use stack::ValueStack;
pub use vm::VM;

/// Direct execution helper evaluating compiled bytecode module to final result.
pub fn run(
    module: techscript_bytecode::BytecodeModule,
) -> Result<techscript_runtime::RuntimeValue, VMError> {
    let mut vm = VM::new(module);
    vm.run()
}
