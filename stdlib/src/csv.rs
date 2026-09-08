use crate::{StdFunction, StdlibModule, StdlibRegistry};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

impl StdlibRegistry {
    pub fn register_csv(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "parse".to_string(),
            Rc::new(StdFunction {
                name: "parse".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let csv = args[0].try_into_string()?;
                    let mut list = Vec::new();
                    for line in csv.lines() {
                        let parts: Vec<RuntimeValue> = line
                            .split(',')
                            .map(|s| RuntimeValue::Str(s.to_string()))
                            .collect();
                        list.push(RuntimeValue::List {
                            items: Rc::new(RefCell::new(parts)),
                            is_const: false,
                        });
                    }
                    Ok(RuntimeValue::List {
                        items: Rc::new(RefCell::new(list)),
                        is_const: false,
                    })
                },
            }),
        );

        exports.insert(
            "stringify".to_string(),
            Rc::new(StdFunction {
                name: "stringify".to_string(),
                arity: 1,
                callback: |_ctx, args| {
                    let mut lines = Vec::new();
                    if let RuntimeValue::List { items, .. } = &args[0] {
                        for row in items.borrow().iter() {
                            if let RuntimeValue::List {
                                items: row_items, ..
                            } = row
                            {
                                let parts: Vec<String> = row_items
                                    .borrow()
                                    .iter()
                                    .map(|item| item.try_into_string().unwrap_or_default())
                                    .collect();
                                lines.push(parts.join(","));
                            }
                        }
                    }
                    Ok(RuntimeValue::Str(lines.join("\n")))
                },
            }),
        );

        self.register_module(
            "std.csv",
            StdlibModule {
                name: "std.csv".to_string(),
                version: "1.0.0".to_string(),
                exports,
                required_capabilities: Vec::new(),
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    use techscript_runtime::context::{RuntimeConfig, RuntimeContext};
    use techscript_runtime::value::RuntimeValue;

    #[test]
    fn test_csv_register() {
        let mut registry = StdlibRegistry::default();
        registry.register_csv();

        let module = registry
            .modules
            .get("std.csv")
            .expect("Module std.csv should be registered");
        assert_eq!(module.name, "std.csv");
        assert!(module.exports.contains_key("parse"));
        assert!(module.exports.contains_key("stringify"));
    }

    #[test]
    fn test_csv_parse() {
        let mut registry = StdlibRegistry::default();
        registry.register_csv();

        let module = registry.modules.get("std.csv").unwrap();
        let parse_fn = module.exports.get("parse").unwrap();

        let mut ctx = RuntimeContext::new(RuntimeConfig::default());
        let args = vec![RuntimeValue::Str("a,b,c\n1,2,3".to_string())];

        let result = parse_fn.call(&mut ctx, args).unwrap();

        if let RuntimeValue::List { items, .. } = result {
            let rows = items.borrow();
            assert_eq!(rows.len(), 2);

            // row 1
            if let RuntimeValue::List {
                items: r1_items, ..
            } = &rows[0]
            {
                let r1 = r1_items.borrow();
                assert_eq!(r1[0].try_into_string().unwrap(), "a");
                assert_eq!(r1[1].try_into_string().unwrap(), "b");
                assert_eq!(r1[2].try_into_string().unwrap(), "c");
            } else {
                panic!("Row 1 is not a list");
            }

            // row 2
            if let RuntimeValue::List {
                items: r2_items, ..
            } = &rows[1]
            {
                let r2 = r2_items.borrow();
                assert_eq!(r2[0].try_into_string().unwrap(), "1");
                assert_eq!(r2[1].try_into_string().unwrap(), "2");
                assert_eq!(r2[2].try_into_string().unwrap(), "3");
            } else {
                panic!("Row 2 is not a list");
            }
        } else {
            panic!("Result is not a list");
        }
    }

    #[test]
    fn test_csv_stringify() {
        let mut registry = StdlibRegistry::default();
        registry.register_csv();

        let module = registry.modules.get("std.csv").unwrap();
        let stringify_fn = module.exports.get("stringify").unwrap();

        let mut ctx = RuntimeContext::new(RuntimeConfig::default());

        let r1 = vec![
            RuntimeValue::Str("a".to_string()),
            RuntimeValue::Str("b".to_string()),
            RuntimeValue::Str("c".to_string()),
        ];
        let row1 = RuntimeValue::List {
            items: Rc::new(RefCell::new(r1)),
            is_const: false,
        };
        let r2 = vec![
            RuntimeValue::Int(1),
            RuntimeValue::Int(2),
            RuntimeValue::Int(3),
        ];
        let row2 = RuntimeValue::List {
            items: Rc::new(RefCell::new(r2)),
            is_const: false,
        };

        let args = vec![RuntimeValue::List {
            items: Rc::new(RefCell::new(vec![row1, row2])),
            is_const: false,
        }];

        let result = stringify_fn.call(&mut ctx, args).unwrap();

        assert_eq!(result.try_into_string().unwrap(), "a,b,c\n1,2,3");
    }
}
