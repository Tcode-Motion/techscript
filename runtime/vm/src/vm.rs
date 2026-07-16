use crate::debugger::VMDebugger;
use crate::diagnostics::VMProfiler;
use crate::error::VMError;
use crate::frame::CallFrame;
use crate::heap::VMHeap;
use crate::native::NativeBridge;
use crate::stack::ValueStack;
use std::collections::HashMap;
use std::rc::Rc;
use techscript_bytecode::BytecodeModule;
use techscript_runtime::RuntimeValue;

/// TechScript virtual machine execution state.
pub struct VM {
    pub module: BytecodeModule,
    pub stack: ValueStack,
    pub frames: Vec<CallFrame>,
    pub globals: HashMap<String, RuntimeValue>,
    pub heap: VMHeap,
    pub native_bridge: NativeBridge,
    pub profiler: VMProfiler,
    pub debugger: VMDebugger,
    pub running: bool,
    pub ctx: techscript_runtime::RuntimeContext,
}

impl VM {
    /// Creates a VM state around a bytecode module.
    pub fn new(module: BytecodeModule) -> Self {
        let mut vm = Self {
            module,
            stack: ValueStack::new(1024),
            frames: Vec::with_capacity(512),
            globals: HashMap::new(),
            heap: VMHeap::new(),
            native_bridge: NativeBridge::new(),
            profiler: VMProfiler::new(),
            debugger: VMDebugger::new(),
            running: false,
            ctx: techscript_runtime::RuntimeContext::new(
                techscript_runtime::context::RuntimeConfig::default(),
            ),
        };
        vm.initialize_std();
        vm
    }

    fn initialize_std(&mut self) {
        let stdlib = techscript_stdlib::StdlibRegistry::new();
        // Register standard namespace map
        self.globals.insert("std".to_string(), stdlib.construct_std_namespace());
        // Register individual function exports globally to resolve individual symbol imports
        for module in stdlib.modules.values() {
            for (func_name, func) in &module.exports {
                self.globals.insert(
                    func_name.clone(),
                    RuntimeValue::Function(Rc::clone(func)),
                );
            }
        }
    }

    /// Evaluates bytecode from the entry module's entry function block.
    pub fn run(&mut self) -> Result<RuntimeValue, VMError> {
        self.running = true;

        // Push entry frame representing entry function (e.g. main)
        let mut main_idx = self.module.entry_idx;

        // Override entrypoint to user-defined main function if declared
        for (i, func) in self.module.functions.iter().enumerate() {
            if i > 0 && func.name == "main" {
                main_idx = i as u32;
                break;
            }
        }

        if (main_idx as usize) >= self.module.functions.len() {
            return Err(VMError::InvalidFunction(main_idx));
        }

        let main_frame = CallFrame::new(main_idx, 0);
        self.frames.push(main_frame);

        // Run dispatch cycle
        self.execute_loop()
    }
}
