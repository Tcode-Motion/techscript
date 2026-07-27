use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_uuid(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "uuid_v4".to_string(),
            Rc::new(StdFunction {
                name: "uuid_v4".to_string(),
                arity: 0,
                callback: |_ctx, _args| {
                    use std::time::SystemTime;
                    let nano = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos();
                    Ok(RuntimeValue::Str(format!(
                        "123e4567-e89b-12d3-a456-{}",
                        nano
                    )))
                },
            }),
        );

        self.register_module(
            "std.uuid",
            StdlibModule {
                name: "std.uuid".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}
