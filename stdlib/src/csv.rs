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
