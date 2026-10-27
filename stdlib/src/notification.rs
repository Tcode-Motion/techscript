pub(crate) fn build_powershell_notification_args(title: &str, body: &str) -> Vec<String> {
    use base64::Engine;

    let b64_title = base64::engine::general_purpose::STANDARD.encode(
        title
            .encode_utf16()
            .flat_map(|c| c.to_le_bytes())
            .collect::<Vec<u8>>(),
    );
    let b64_body = base64::engine::general_purpose::STANDARD.encode(
        body.encode_utf16()
            .flat_map(|c| c.to_le_bytes())
            .collect::<Vec<u8>>(),
    );

    let script = format!(
        "Add-Type -AssemblyName PresentationFramework; [System.Windows.MessageBox]::Show([System.Text.Encoding]::Unicode.GetString([System.Convert]::FromBase64String('{}')), [System.Text.Encoding]::Unicode.GetString([System.Convert]::FromBase64String('{}')))",
        b64_body, b64_title
    );
    let b64_script = base64::engine::general_purpose::STANDARD.encode(
        script
            .encode_utf16()
            .flat_map(|c| c.to_le_bytes())
            .collect::<Vec<u8>>(),
    );

    vec![
        "-NoProfile".to_string(),
        "-NonInteractive".to_string(),
        "-EncodedCommand".to_string(),
        b64_script,
    ]
}

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

                        let args = build_powershell_notification_args(&title, &body);
                        let _ = Command::new("powershell").args(&args).spawn();
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

    #[test]
    fn test_powershell_notification_args_security() {
        let title = "Test Title'; Remove-Item -Path C:\\ -Recurse -Force; '";
        let body = "Test Body\n\"'$$\x00";

        let args = build_powershell_notification_args(title, body);

        assert_eq!(args[0], "-NoProfile");
        assert_eq!(args[1], "-NonInteractive");
        assert_eq!(args[2], "-EncodedCommand");

        use base64::Engine;
        let script_bytes = base64::engine::general_purpose::STANDARD
            .decode(&args[3])
            .unwrap();
        let u16_chars: Vec<u16> = script_bytes
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();
        let script = String::from_utf16(&u16_chars).unwrap();

        assert!(script.contains("Add-Type -AssemblyName PresentationFramework"));
        assert!(script.contains("[System.Windows.MessageBox]::Show"));

        assert!(!script.contains("Remove-Item"));
    }
}
