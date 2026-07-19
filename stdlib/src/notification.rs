use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};
use crate::{StdFunction, StdlibModule, StdlibRegistry};

impl StdlibRegistry {
    pub fn register_notification(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> = HashMap::new();

        exports.insert("show".to_string(), Rc::new(StdFunction {
            name: "show".to_string(),
            arity: 2,
            callback: |_ctx, args| {
                let title = args[0].to_string();
                let body = args[1].to_string();
                #[cfg(target_os = "windows")]
                {
                    use std::process::Command;
                    let _ = Command::new("powershell")
                        .args(["-Command", &format!("[System.Windows.MessageBox]::Show('{}','{}')", body.replace("'", "''"), title.replace("'", "''"))])
                        .spawn();
                }
                #[cfg(not(target_os = "windows"))]
                {
                    let _ = std::process::Command::new("notify-send")
                        .args([&title, &body])
                        .spawn();
                }
                Ok(RuntimeValue::Null)
            },
        }));

        exports.insert("alert".to_string(), Rc::new(StdFunction {
            name: "alert".to_string(),
            arity: 1,
            callback: |_ctx, args| {
                let msg = args[0].to_string();
                #[cfg(target_os = "windows")]
                {
                    let _ = std::process::Command::new("msg")
                        .args(["*", &msg])
                        .spawn();
                }
                #[cfg(not(target_os = "windows"))]
                {
                    let _ = std::process::Command::new("notify-send")
                        .args(["Alert", &msg])
                        .spawn();
                }
                Ok(RuntimeValue::Null)
            },
        }));

        self.register_module("std.notification", StdlibModule {
            name: "std.notification".to_string(),
            version: "1.0.0".to_string(),
            exports,
            required_capabilities: Vec::new(),
        });
    }
}
