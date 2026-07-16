use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use techscript_runtime::{
    context::Capability,
    value::RuntimeValue,
    RuntimeConfig, RuntimeContext,
};
use techscript_stdlib::StdlibRegistry;

#[test]
fn test_math_module() {
    let registry = StdlibRegistry::new();
    let math = registry.get_module("std.math").unwrap();

    let mut ctx = RuntimeContext::new(RuntimeConfig::default());

    // test abs
    let abs = math.exports.get("abs").unwrap();
    let res = abs
        .call(&mut ctx, vec![RuntimeValue::Int(-42)])
        .unwrap();
    assert_eq!(res.as_int(), Some(42));

    let res = abs
        .call(&mut ctx, vec![RuntimeValue::Float(-3.14)])
        .unwrap();
    assert_eq!(res.as_float(), Some(3.14));

    // test sqrt
    let sqrt = math.exports.get("sqrt").unwrap();
    let res = sqrt
        .call(&mut ctx, vec![RuntimeValue::Float(16.0)])
        .unwrap();
    assert_eq!(res.as_float(), Some(4.0));

    // test pow
    let pow = math.exports.get("pow").unwrap();
    let res = pow
        .call(
            &mut ctx,
            vec![RuntimeValue::Float(2.0), RuntimeValue::Float(3.0)],
        )
        .unwrap();
    assert_eq!(res.as_float(), Some(8.0));

    // test floor, ceil, round
    let floor = math.exports.get("floor").unwrap();
    let res = floor
        .call(&mut ctx, vec![RuntimeValue::Float(2.7)])
        .unwrap();
    assert_eq!(res.as_float(), Some(2.0));

    let ceil = math.exports.get("ceil").unwrap();
    let res = ceil
        .call(&mut ctx, vec![RuntimeValue::Float(2.1)])
        .unwrap();
    assert_eq!(res.as_float(), Some(3.0));

    let round = math.exports.get("round").unwrap();
    let res = round
        .call(&mut ctx, vec![RuntimeValue::Float(2.5)])
        .unwrap();
    assert_eq!(res.as_float(), Some(3.0));

    // test random
    let random = math.exports.get("random").unwrap();
    let r1 = random.call(&mut ctx, vec![]).unwrap().as_float().unwrap();
    let r2 = random.call(&mut ctx, vec![]).unwrap().as_float().unwrap();
    assert!(r1 >= 0.0 && r1 < 1.0);
    assert!(r2 >= 0.0 && r2 < 1.0);
    assert_ne!(r1, r2); // pseudo-random sequence should advance
}

#[test]
fn test_strings_module() {
    let registry = StdlibRegistry::new();
    let strings = registry.get_module("std.strings").unwrap();

    let mut ctx = RuntimeContext::new(RuntimeConfig::default());

    // test trim
    let trim = strings.exports.get("trim").unwrap();
    let res = trim
        .call(&mut ctx, vec![RuntimeValue::Str("   hello   ".to_string())])
        .unwrap();
    assert_eq!(res.as_string(), Some("hello"));

    // test replace
    let replace = strings.exports.get("replace").unwrap();
    let res = replace
        .call(
            &mut ctx,
            vec![
                RuntimeValue::Str("foo bar".to_string()),
                RuntimeValue::Str("foo".to_string()),
                RuntimeValue::Str("baz".to_string()),
            ],
        )
        .unwrap();
    assert_eq!(res.as_string(), Some("baz bar"));

    // test split
    let split = strings.exports.get("split").unwrap();
    let res = split
        .call(
            &mut ctx,
            vec![
                RuntimeValue::Str("a,b,c".to_string()),
                RuntimeValue::Str(",".to_string()),
            ],
        )
        .unwrap();
    if let RuntimeValue::List { items, .. } = res {
        let list = items.borrow();
        assert_eq!(list.len(), 3);
        assert_eq!(list[0].as_string(), Some("a"));
        assert_eq!(list[1].as_string(), Some("b"));
        assert_eq!(list[2].as_string(), Some("c"));
    } else {
        panic!("split did not return a List");
    }
}

#[test]
fn test_collections_module() {
    let registry = StdlibRegistry::new();
    let collections = registry.get_module("std.collections").unwrap();

    let mut ctx = RuntimeContext::new(RuntimeConfig::default());

    // test push, pop, sort
    let push = collections.exports.get("push").unwrap();
    let pop = collections.exports.get("pop").unwrap();
    let sort = collections.exports.get("sort").unwrap();

    let list = RuntimeValue::List {
        items: Rc::new(RefCell::new(vec![
            RuntimeValue::Int(3),
            RuntimeValue::Int(1),
        ])),
        is_const: false,
    };

    push.call(&mut ctx, vec![list.clone(), RuntimeValue::Int(2)])
        .unwrap();
    sort.call(&mut ctx, vec![list.clone()]).unwrap();

    if let RuntimeValue::List { items, .. } = &list {
        let list_borrow = items.borrow();
        assert_eq!(list_borrow.len(), 3);
        assert_eq!(list_borrow[0].as_int(), Some(1));
        assert_eq!(list_borrow[1].as_int(), Some(2));
        assert_eq!(list_borrow[2].as_int(), Some(3));
    } else {
        panic!("not a list");
    }

    let popped = pop.call(&mut ctx, vec![list.clone()]).unwrap();
    assert_eq!(popped.as_int(), Some(3));
}

#[test]
fn test_json_module() {
    let registry = StdlibRegistry::new();
    let json = registry.get_module("std.json").unwrap();

    let mut ctx = RuntimeContext::new(RuntimeConfig::default());

    let stringify = json.exports.get("stringify").unwrap();
    let parse = json.exports.get("parse").unwrap();

    // Serialize a nested structure
    let mut map = indexmap::IndexMap::new();
    map.insert("name".to_string(), RuntimeValue::Str("Tanmoy".to_string()));
    map.insert("age".to_string(), RuntimeValue::Int(25));
    let original = RuntimeValue::Map {
        entries: Rc::new(RefCell::new(map)),
        is_const: false,
    };

    let serialized = stringify.call(&mut ctx, vec![original]).unwrap();
    assert_eq!(
        serialized.as_string(),
        Some("{\"name\":\"Tanmoy\",\"age\":25}")
    );

    let deserialized = parse.call(&mut ctx, vec![serialized]).unwrap();
    if let RuntimeValue::Map { entries, .. } = deserialized {
        let entries_borrow = entries.borrow();
        assert_eq!(
            entries_borrow.get("name").unwrap().as_string(),
            Some("Tanmoy")
        );
        assert_eq!(
            entries_borrow.get("age").unwrap().as_int(),
            Some(25)
        );
    } else {
        panic!("JSON parse result was not a Map");
    }
}

#[test]
fn test_sys_module_sandboxing() {
    let registry = StdlibRegistry::new();
    let fs = registry.get_module("std.fs").unwrap();
    let env = registry.get_module("std.env").unwrap();
    let process = registry.get_module("std.process").unwrap();

    // Create a context with NO capabilities
    let config = RuntimeConfig {
        strict_mode: false,
        max_recursion_depth: 1000,
        enable_assertions: true,
        capabilities: HashSet::new(), // Deny all capabilities!
    };
    let mut ctx = RuntimeContext::new(config);

    // 1. Verify filesystem operations are denied
    let read_file = fs.exports.get("read_file").unwrap();
    let res = read_file.call(&mut ctx, vec![RuntimeValue::Str("test.txt".to_string())]);
    assert!(res.is_err());
    assert!(res
        .unwrap_err()
        .to_string()
        .contains("Security policy violation"));

    // 2. Verify environment variable operations are denied
    let env_get = env.exports.get("get").unwrap();
    let res = env_get.call(&mut ctx, vec![RuntimeValue::Str("PATH".to_string())]);
    assert!(res.is_err());
    assert!(res
        .unwrap_err()
        .to_string()
        .contains("Security policy violation"));

    // 3. Verify process execution is denied
    let proc_run = process.exports.get("run").unwrap();
    let res = proc_run.call(
        &mut ctx,
        vec![
            RuntimeValue::Str("echo".to_string()),
            RuntimeValue::List {
                items: Rc::new(RefCell::new(vec![RuntimeValue::Str("hello".to_string())])),
                is_const: false,
            },
        ],
    );
    assert!(res.is_err());
    assert!(res
        .unwrap_err()
        .to_string()
        .contains("Security policy violation"));
}

#[test]
fn test_sys_module_granted() {
    let registry = StdlibRegistry::new();
    let fs = registry.get_module("std.fs").unwrap();

    // Create a temporary file fixture
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join("techscript_test.txt");
    let temp_file_str = temp_file.to_string_lossy().to_string();

    // Create a context with FileSystem capability GRANTED
    let mut caps = HashSet::new();
    caps.insert(Capability::FileSystem);
    let config = RuntimeConfig {
        strict_mode: false,
        max_recursion_depth: 1000,
        enable_assertions: true,
        capabilities: caps,
    };
    let mut ctx = RuntimeContext::new(config);

    let write_file = fs.exports.get("write_file").unwrap();
    let read_file = fs.exports.get("read_file").unwrap();
    let exists = fs.exports.get("exists").unwrap();

    // Write file
    write_file
        .call(
            &mut ctx,
            vec![
                RuntimeValue::Str(temp_file_str.clone()),
                RuntimeValue::Str("Hello TechScript 2.0 stdlib".to_string()),
            ],
        )
        .unwrap();

    // Verify exists
    let file_exists = exists
        .call(&mut ctx, vec![RuntimeValue::Str(temp_file_str.clone())])
        .unwrap();
    assert_eq!(file_exists, RuntimeValue::Bool(true));

    // Read file
    let content = read_file
        .call(&mut ctx, vec![RuntimeValue::Str(temp_file_str.clone())])
        .unwrap();
    assert_eq!(
        content.as_string(),
        Some("Hello TechScript 2.0 stdlib")
    );

    // Clean up temporary file
    std::fs::remove_file(temp_file).ok();
}
