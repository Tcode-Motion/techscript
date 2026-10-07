use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{
    context::Capability,
    error::{RuntimeError, RuntimeErrorKind},
    value::RuntimeValue,
};

impl StdlibRegistry {
    pub fn register_notification(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "show".to_string(),
            Rc::new(StdFunction {
                name: "show".to_string(),
                arity: 2,
                callback: |ctx, args| {
                    if !ctx.config.capabilities.contains(&Capability::Process) {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(
                                "Security policy violation: Process capability is denied"
                                    .to_string(),
                            ),
                            None,
                            None,
                        ));
                    }
                    let title = args[0].to_string();
                    let body = args[1].to_string();
                    #[cfg(target_os = "windows")]
                    {
                        use std::process::Command;
                        use base64::Engine;
                        let b64_title = base64::prelude::BASE64_STANDARD.encode(title.as_bytes());
                        let b64_body = base64::prelude::BASE64_STANDARD.encode(body.as_bytes());
                        let script_text = format!(
                            "[System.Windows.MessageBox]::Show(\
                                [System.Text.Encoding]::UTF8.GetString([System.Convert]::FromBase64String('{}')), \
                                [System.Text.Encoding]::UTF8.GetString([System.Convert]::FromBase64String('{}'))\
                            )",
                            b64_body, b64_title
                        );
                        let mut script_utf16: Vec<u8> = Vec::with_capacity(script_text.len() * 2);
                        for c in script_text.encode_utf16() {
                            script_utf16.push((c & 0xFF) as u8);
                            script_utf16.push((c >> 8) as u8);
                        }
                        let encoded_cmd = base64::prelude::BASE64_STANDARD.encode(&script_utf16);
                        let _ = Command::new("powershell")
                            .args(["-NoProfile", "-EncodedCommand", &encoded_cmd])
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
            }),
        );

        exports.insert(
            "alert".to_string(),
            Rc::new(StdFunction {
                name: "alert".to_string(),
                arity: 1,
                callback: |ctx, args| {
                    if !ctx.config.capabilities.contains(&Capability::Process) {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::InvalidOperation(
                                "Security policy violation: Process capability is denied"
                                    .to_string(),
                            ),
                            None,
                            None,
                        ));
                    }
                    let msg = args[0].to_string();
                    #[cfg(target_os = "windows")]
                    {
                        let _ = std::process::Command::new("msg").args(["*", &msg]).spawn();
                    }
                    #[cfg(not(target_os = "windows"))]
                    {
                        let _ = std::process::Command::new("notify-send")
                            .args(["Alert", &msg])
                            .spawn();
                    }
                    Ok(RuntimeValue::Null)
                },
            }),
        );

        self.register_module(
            "std.notification",
            StdlibModule {
                name: "std.notification".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: vec![Capability::Process],
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_notification() {
        let mut registry = StdlibRegistry {
            modules: std::collections::HashMap::new(),
        };
        registry.register_notification();

        assert!(registry.has_module("std.notification"));

        let module = registry.get_module("std.notification").unwrap();
        assert_eq!(module.name, "std.notification");

        let show = module.exports.get("show").unwrap();
        assert_eq!(show.name(), "show");
        assert_eq!(show.arity(), 2);

        let alert = module.exports.get("alert").unwrap();
        assert_eq!(alert.name(), "alert");
        assert_eq!(alert.arity(), 1);
    }
}
