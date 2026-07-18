use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{
    context::{Capability, RuntimeContext},
    error::RuntimeError,
    value::RuntimeValue,
};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

impl StdlibRegistry {
    pub fn register_thread(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();

        exports.insert(
            "spawn".to_string(),
            Rc::new(StdFunction {
                name: "spawn".to_string(),
                arity: 1,
                callback: |ctx, args| {
                    let callback = args[0].clone();
                    if let RuntimeValue::Function(func) = callback {
                        let func_ptr = Box::into_raw(Box::new(func)) as usize;
                        let handle = std::thread::spawn(move || {
                            let func = unsafe { Box::from_raw(func_ptr as *mut Rc<dyn techscript_runtime::function::Callable>) };
                            let mut ctx = RuntimeContext::new(techscript_runtime::RuntimeConfig::default());
                            func.call(&mut ctx, vec![]).ok();
                        });
                        let handle_id = ctx.resources.borrow_mut().insert(handle);
                        return Ok(RuntimeValue::Int(handle_id as i64));
                    }
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        exports.insert(
            "join".to_string(),
            Rc::new(StdFunction {
                name: "join".to_string(),
                arity: 1,
                callback: |ctx, args| {
                    let handle_id = args[0].try_into_int()? as u32;
                    if let Some(handle) = ctx.resources.borrow_mut().remove::<std::thread::JoinHandle<()>>(handle_id) {
                        handle.join().ok();
                    }
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        self.register_module(
            "std.thread",
            StdlibModule {
                name: "std.thread".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: vec![Capability::Process],
            },
        );
    }
}
