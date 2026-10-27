use crate::{StdFunction, StdlibModule, StdlibRegistry};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Write;
use std::rc::Rc;
use techscript_runtime::{error::RuntimeError, value::RuntimeValue};

fn parse_yaml(args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    let yaml = args[0].try_into_string()?;
    let mut map = IndexMap::new();
    for line in yaml.lines() {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        if let Some(pos) = line.find(':') {
            let k = line[..pos].trim().to_string();
            let v = line[pos + 1..].trim().to_string();
            map.insert(k, RuntimeValue::Str(v));
        }
    }
    Ok(RuntimeValue::Map {
        entries: Rc::new(RefCell::new(map)),
        is_const: false,
    })
}

fn stringify_yaml(args: &[RuntimeValue]) -> Result<RuntimeValue, RuntimeError> {
    let mut result = String::new();
    if let RuntimeValue::Map { entries, .. } = &args[0] {
        for (k, v) in entries.borrow().iter() {
            let val_str = v.try_into_string().unwrap_or_default();
            let _ = write!(result, "{}: {}\n", k, val_str);
        }
    }
    Ok(RuntimeValue::Str(result))
}

impl StdlibRegistry {
    pub fn register_yaml(&mut self) {
        let mut exports: HashMap<String, Rc<dyn techscript_runtime::function::Callable>> =
            HashMap::new();

        exports.insert(
            "parse".to_string(),
            Rc::new(StdFunction {
                name: "parse".to_string(),
                arity: 1,
                callback: |_ctx, args| parse_yaml(&args),
            }),
        );

        exports.insert(
            "stringify".to_string(),
            Rc::new(StdFunction {
                name: "stringify".to_string(),
                arity: 1,
                callback: |_ctx, args| stringify_yaml(&args),
            }),
        );

        self.register_module(
            "std.yaml",
            StdlibModule {
                name: "std.yaml".to_string(),
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

    #[test]
    fn test_parse_yaml() {
        let input = vec![RuntimeValue::Str(
            "foo: bar\n# comment\nbaz: qux".to_string(),
        )];
        let res = parse_yaml(&input).unwrap();
        if let RuntimeValue::Map { entries, .. } = res {
            let map = entries.borrow();
            assert_eq!(map.get("foo"), Some(&RuntimeValue::Str("bar".to_string())));
            assert_eq!(map.get("baz"), Some(&RuntimeValue::Str("qux".to_string())));
        } else {
            panic!("Expected map value");
        }
    }

    #[test]
    fn test_stringify_yaml() {
        let mut map = IndexMap::new();
        map.insert("key1".to_string(), RuntimeValue::Str("val1".to_string()));
        let val = RuntimeValue::Map {
            entries: Rc::new(RefCell::new(map)),
            is_const: false,
        };
        let res = stringify_yaml(&[val]).unwrap();
        assert_eq!(res, RuntimeValue::Str("key1: val1\n".to_string()));
    }
}
