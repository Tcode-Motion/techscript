use indexmap::IndexMap;
use std::cell::RefCell;
use std::rc::Rc;
use techscript_runtime::{
    list_get, list_set, map_get, map_set, Environment, RuntimeConfig, RuntimeContext,
    RuntimeErrorKind, RuntimeType, RuntimeValue, StructInstance,
};

#[test]
fn test_value_construction_and_truthiness() {
    let null_val = RuntimeValue::Null;
    let true_val = RuntimeValue::Bool(true);
    let false_val = RuntimeValue::Bool(false);
    let int_val = RuntimeValue::Int(42);
    let float_val = RuntimeValue::Float(1.5);
    let str_val = RuntimeValue::Str("hello".to_string());

    assert_eq!(null_val.runtime_type(), RuntimeType::Null);
    assert_eq!(true_val.runtime_type(), RuntimeType::Bool);
    assert_eq!(int_val.runtime_type(), RuntimeType::Int);
    assert_eq!(float_val.runtime_type(), RuntimeType::Float);
    assert_eq!(str_val.runtime_type(), RuntimeType::String);

    assert!(!null_val.is_truthy());
    assert!(true_val.is_truthy());
    assert!(!false_val.is_truthy());
    assert!(int_val.is_truthy());
    assert!(float_val.is_truthy());
    assert!(str_val.is_truthy());
}

#[test]
fn test_value_conversions() {
    let int_val = RuntimeValue::Int(42);
    let float_val = RuntimeValue::Float(1.5);
    let str_val = RuntimeValue::Str("123".to_string());
    let true_val = RuntimeValue::Bool(true);

    assert_eq!(int_val.try_into_int().unwrap(), 42);
    assert_eq!(float_val.try_into_int().unwrap(), 1);
    assert_eq!(str_val.try_into_int().unwrap(), 123);
    assert_eq!(true_val.try_into_int().unwrap(), 1);

    assert_eq!(float_val.try_into_float().unwrap(), 1.5);
    assert_eq!(int_val.try_into_float().unwrap(), 42.0);
    assert_eq!(str_val.try_into_float().unwrap(), 123.0);

    assert!(true_val.try_into_bool().unwrap());
    assert!(int_val.try_into_bool().unwrap());

    assert_eq!(int_val.try_into_string().unwrap(), "42");
    assert_eq!(str_val.try_into_string().unwrap(), "123");
}

#[test]
fn test_environment_scoping() {
    let global = Rc::new(RefCell::new(Environment::new(None)));
    global
        .borrow_mut()
        .define("x".to_string(), RuntimeValue::Int(10), false);
    global
        .borrow_mut()
        .define("y".to_string(), RuntimeValue::Int(20), true); // const y

    let child = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&global)))));
    child
        .borrow_mut()
        .define("x".to_string(), RuntimeValue::Int(30), false); // shadows x

    // Lookup checks
    assert_eq!(child.borrow().lookup("x").unwrap(), RuntimeValue::Int(30));
    assert_eq!(child.borrow().lookup("y").unwrap(), RuntimeValue::Int(20));

    // Assignment checks
    child
        .borrow_mut()
        .assign("x", RuntimeValue::Int(40))
        .unwrap();
    assert_eq!(child.borrow().lookup("x").unwrap(), RuntimeValue::Int(40));
    assert_eq!(global.borrow().lookup("x").unwrap(), RuntimeValue::Int(10)); // outer scope remains unchanged

    // Assigning to const y should error
    let assign_const_res = child.borrow_mut().assign("y", RuntimeValue::Int(50));
    assert!(assign_const_res.is_err());
    assert!(matches!(
        assign_const_res.unwrap_err().kind,
        RuntimeErrorKind::InvalidOperation(_)
    ));
}

#[test]
fn test_runtime_object_identity() {
    let s1 = Rc::new(RefCell::new(StructInstance::new(
        "Point".to_string(),
        IndexMap::new(),
        false,
    )));
    let s2 = Rc::new(RefCell::new(StructInstance::new(
        "Point".to_string(),
        IndexMap::new(),
        false,
    )));
    let s1_clone = Rc::clone(&s1);

    let val1 = RuntimeValue::StructInstance(s1);
    let val2 = RuntimeValue::StructInstance(s2);
    let val1_clone = RuntimeValue::StructInstance(s1_clone);

    // Dynamic equality (both are Point with empty fields, so they are structurally equal)
    assert_eq!(val1, val2);

    // Reference identity (val1 and val2 have different ObjectId)
    assert!(!val1.physical_eq(&val2));
    assert!(val1.physical_eq(&val1_clone));
}

#[test]
fn test_list_and_map_mutations() {
    let list_val = RuntimeValue::List {
        items: Rc::new(RefCell::new(vec![
            RuntimeValue::Int(1),
            RuntimeValue::Int(2),
        ])),
        is_const: false,
    };

    assert_eq!(list_get(&list_val, 0).unwrap(), RuntimeValue::Int(1));
    assert_eq!(list_get(&list_val, -1).unwrap(), RuntimeValue::Int(2)); // negative indexing

    list_set(&list_val, 1, RuntimeValue::Int(3)).unwrap();
    assert_eq!(list_get(&list_val, 1).unwrap(), RuntimeValue::Int(3));

    // Const collections
    let const_list = RuntimeValue::List {
        items: Rc::new(RefCell::new(vec![RuntimeValue::Int(1)])),
        is_const: true,
    };
    let mutate_const_res = list_set(&const_list, 0, RuntimeValue::Int(2));
    assert!(mutate_const_res.is_err());

    let map_val = RuntimeValue::Map {
        entries: Rc::new(RefCell::new(IndexMap::new())),
        is_const: false,
    };
    map_set(
        &map_val,
        "key".to_string(),
        RuntimeValue::Str("value".to_string()),
    )
    .unwrap();
    assert_eq!(
        map_get(&map_val, "key").unwrap(),
        RuntimeValue::Str("value".to_string())
    );
}

#[test]
fn test_native_functions_dispatch() {
    let config = RuntimeConfig::default();
    let mut ctx = RuntimeContext::new(config);

    let len_func = ctx.registry.lookup("len").unwrap();
    assert_eq!(len_func.name(), "len");
    assert_eq!(len_func.arity(), 1);

    let list_val = RuntimeValue::List {
        items: Rc::new(RefCell::new(vec![
            RuntimeValue::Int(10),
            RuntimeValue::Int(20),
        ])),
        is_const: false,
    };
    let res = len_func.call(&mut ctx, vec![list_val]).unwrap();
    assert_eq!(res, RuntimeValue::Int(2));
}

#[test]
fn test_assertion_errors() {
    let config = RuntimeConfig {
        strict_mode: false,
        max_recursion_depth: 1000,
        enable_assertions: true,
        capabilities: std::collections::HashSet::new(),
    };
    let mut ctx = RuntimeContext::new(config);

    let assert_func = ctx.registry.lookup("assert").unwrap();
    let res = assert_func.call(
        &mut ctx,
        vec![
            RuntimeValue::Bool(false),
            RuntimeValue::Str("failed".to_string()),
        ],
    );
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert!(matches!(err.kind, RuntimeErrorKind::AssertionFailed(_)));
    assert_eq!(err.message, "Assertion failed: failed");
}
