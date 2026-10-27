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
                    let _ = notify_rust::Notification::new()
                        .summary(&title)
                        .body(&body)
                        .show();
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
                    let _ = notify_rust::Notification::new()
                        .summary("Alert")
                        .body(&msg)
                        .show();
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
    fn test_notification_show_and_alert() {
        use techscript_runtime::context::{RuntimeConfig, RuntimeContext};

        let mut registry = StdlibRegistry {
            modules: std::collections::HashMap::new(),
        };
        registry.register_notification();

        let module = registry.get_module("std.notification").unwrap();
        let show = module.exports.get("show").unwrap();
        let alert = module.exports.get("alert").unwrap();

        let mut config = RuntimeConfig::default();
        config.capabilities.insert(Capability::Process);
        let mut ctx = RuntimeContext::new(config);

        let res_show = show.call(
            &mut ctx,
            vec![
                RuntimeValue::Str("Test Title".to_string()),
                RuntimeValue::Str("Test Body".to_string()),
            ],
        );
        assert!(res_show.is_ok());
        assert_eq!(res_show.unwrap(), RuntimeValue::Null);

        let res_alert = alert.call(
            &mut ctx,
            vec![RuntimeValue::Str("Test Alert Message".to_string())],
        );
        assert!(res_alert.is_ok());
        assert_eq!(res_alert.unwrap(), RuntimeValue::Null);
    }

    #[test]
    fn test_notification_capability_denied() {
        use techscript_runtime::context::{RuntimeConfig, RuntimeContext};

        let mut registry = StdlibRegistry {
            modules: std::collections::HashMap::new(),
        };
        registry.register_notification();

        let module = registry.get_module("std.notification").unwrap();
        let show = module.exports.get("show").unwrap();

        let mut config = RuntimeConfig::default();
        config.capabilities.remove(&Capability::Process);
        let mut ctx = RuntimeContext::new(config);

        let res_show = show.call(
            &mut ctx,
            vec![
                RuntimeValue::Str("Test Title".to_string()),
                RuntimeValue::Str("Test Body".to_string()),
            ],
        );
        assert!(res_show.is_err());
    }
}
