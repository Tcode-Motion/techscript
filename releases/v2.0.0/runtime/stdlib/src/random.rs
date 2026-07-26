use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{
    error::RuntimeError,
    value::RuntimeValue,
};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

impl StdlibRegistry {
    pub fn register_random(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();

        exports.insert(
            "int".to_string(),
            Rc::new(StdFunction {
                name: "int".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let min = args[0].try_into_int()?;
                    let max = args[1].try_into_int()?;
                    let rand_num = min + (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as i64 % (max - min + 1));
                    Ok(RuntimeValue::Int(rand_num))
                },
            }),
        );

        exports.insert(
            "float".to_string(),
            Rc::new(StdFunction {
                name: "float".to_string(),
                arity: 2,
                callback: |_ctx, args| {
                    let min = args[0].try_into_float()?;
                    let max = args[1].try_into_float()?;
                    let nano = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().subsec_nanos() as f64;
                    let pct = nano / 1_000_000_000.0;
                    Ok(RuntimeValue::Float(min + pct * (max - min)))
                },
            }),
        );

        exports.insert(
            "choice".to_string(),
            Rc::new(StdFunction {
                name: "choice".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    if let RuntimeValue::List { items, .. } = &args[0] {
                        let borrow = items.borrow();
                        if !borrow.is_empty() {
                            let idx = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().subsec_nanos() as usize) % borrow.len();
                            return Ok(borrow[idx].clone());
                        }
                    }
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        self.register_module(
            "std.random",
            StdlibModule {
                name: "std.random".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
