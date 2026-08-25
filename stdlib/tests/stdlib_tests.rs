use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use techscript_runtime::{context::Capability, value::RuntimeValue, RuntimeConfig, RuntimeContext};
use techscript_stdlib::StdlibRegistry;

#[test]
fn test_math_module() {
    let registry = StdlibRegistry::new();
    let math = registry.get_module("std.math").unwrap();

    let mut ctx = RuntimeContext::new(RuntimeConfig::default());

    // test abs
    let abs = math.exports.get("abs").unwrap();
    let res = abs.call(&mut ctx, vec![RuntimeValue::Int(-42)]).unwrap();
    assert_eq!(res.as_int(), Some(42));

    let res = abs.call(&mut ctx, vec![RuntimeValue::Float(-3.5)]).unwrap();
    assert_eq!(res.as_float(), Some(3.5));

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
    let res = ceil.call(&mut ctx, vec![RuntimeValue::Float(2.1)]).unwrap();
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
    assert!((0.0..1.0).contains(&r1));
    assert!((0.0..1.0).contains(&r2));
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
        assert_eq!(entries_borrow.get("age").unwrap().as_int(), Some(25));
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
    assert_eq!(content.as_string(), Some("Hello TechScript 2.0 stdlib"));

    // Clean up temporary file
    std::fs::remove_file(temp_file).ok();
}

#[test]
fn test_math_limits_and_conversions() {
    let registry = StdlibRegistry::new();
    let math = registry.get_module("std.math").unwrap();
    let mut ctx = RuntimeContext::new(RuntimeConfig::default());

    // 1. Float limits & conversions
    let abs = math.exports.get("abs").unwrap();
    let nan_val = RuntimeValue::Float(f64::NAN);
    let res = abs.call(&mut ctx, vec![nan_val]).unwrap();
    assert!(res.as_float().unwrap().is_nan());

    let inf_val = RuntimeValue::Float(f64::INFINITY);
    let res = abs.call(&mut ctx, vec![inf_val]).unwrap();
    assert_eq!(res.as_float(), Some(f64::INFINITY));

    let neg_inf_val = RuntimeValue::Float(f64::NEG_INFINITY);
    let res = abs.call(&mut ctx, vec![neg_inf_val]).unwrap();
    assert_eq!(res.as_float(), Some(f64::INFINITY));

    let to_float = math.exports.get("to_float").unwrap();
    let res = to_float
        .call(&mut ctx, vec![RuntimeValue::Int(123)])
        .unwrap();
    assert_eq!(res.as_float(), Some(123.0));
}

#[test]
fn test_path_validations() {
    let registry = StdlibRegistry::new();
    let path = registry.get_module("std.path").unwrap();
    let mut ctx = RuntimeContext::new(RuntimeConfig::default());

    let join = path.exports.get("join").unwrap();
    let basename = path.exports.get("basename").unwrap();
    let extname = path.exports.get("extname").unwrap();

    // 1. Path joining
    let res = join
        .call(
            &mut ctx,
            vec![
                RuntimeValue::Str("foo".to_string()),
                RuntimeValue::Str("bar.txt".to_string()),
            ],
        )
        .unwrap();
    let path_str = res.as_string().unwrap();
    assert!(path_str.contains("foo") && path_str.contains("bar.txt"));

    // 2. Basename & extension extraction
    let res = basename
        .call(
            &mut ctx,
            vec![RuntimeValue::Str("foo/bar/baz.txs".to_string())],
        )
        .unwrap();
    assert_eq!(res.as_string(), Some("baz.txs"));

    let res = extname
        .call(
            &mut ctx,
            vec![RuntimeValue::Str("foo/bar/baz.txs".to_string())],
        )
        .unwrap();
    assert_eq!(res.as_string(), Some("txs"));
}

#[test]
fn test_regex_operations() {
    let registry = StdlibRegistry::new();
    let regex = registry.get_module("std.regex").unwrap();
    let mut ctx = RuntimeContext::new(RuntimeConfig::default());

    let r_match = regex.exports.get("match").unwrap();
    let r_replace = regex.exports.get("replace").unwrap();

    // 1. Regex Match (substring fallback check)
    let res = r_match
        .call(
            &mut ctx,
            vec![
                RuntimeValue::Str("hello".to_string()),
                RuntimeValue::Str("hello world".to_string()),
            ],
        )
        .unwrap();
    assert_eq!(res, RuntimeValue::Bool(true));

    let res = r_match
        .call(
            &mut ctx,
            vec![
                RuntimeValue::Str("missing".to_string()),
                RuntimeValue::Str("hello world".to_string()),
            ],
        )
        .unwrap();
    assert_eq!(res, RuntimeValue::Bool(false));

    // 2. Regex Replace (replacement check)
    let res = r_replace
        .call(
            &mut ctx,
            vec![
                RuntimeValue::Str("world".to_string()),
                RuntimeValue::Str("hello world".to_string()),
                RuntimeValue::Str("TechScript".to_string()),
            ],
        )
        .unwrap();
    assert_eq!(res.as_string(), Some("hello TechScript"));
}

#[test]
fn test_http_module() {
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::thread;

    // Start a simple mock TCP server on a random port
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let mut buf = [0; 1024];
            if stream.read(&mut buf).is_ok() {
                let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\nContent-Type: text/plain\r\n\r\nHello Server!";
                stream.write_all(response.as_bytes()).ok();
            }
        }
    });

    let registry = StdlibRegistry::new();
    let http = registry.get_module("std.http").unwrap();

    let mut caps = HashSet::new();
    caps.insert(Capability::Network);
    let mut ctx = RuntimeContext::new(RuntimeConfig {
        strict_mode: false,
        max_recursion_depth: 1000,
        enable_assertions: true,
        capabilities: caps,
    });

    let get = http.exports.get("get").unwrap();
    let res = get
        .call(
            &mut ctx,
            vec![RuntimeValue::Str(format!("http://127.0.0.1:{}", port))],
        )
        .unwrap();

    if let RuntimeValue::Map { entries, .. } = res {
        let entries_borrow = entries.borrow();
        assert_eq!(entries_borrow.get("status").unwrap().as_int(), Some(200));
        assert_eq!(
            entries_borrow.get("body").unwrap().as_string(),
            Some("Hello Server!")
        );
    } else {
        panic!("get did not return a Map");
    }
}

#[test]
fn test_system_diagnostics_and_process_execution() {
    let registry = StdlibRegistry::new();
    let system = registry.get_module("std.system").unwrap();
    let process = registry.get_module("std.process").unwrap();

    let mut caps = HashSet::new();
    caps.insert(Capability::Process);
    let mut ctx = RuntimeContext::new(RuntimeConfig {
        strict_mode: false,
        max_recursion_depth: 1000,
        enable_assertions: true,
        capabilities: caps,
    });

    // 1. CPU, memory, and disk diagnostics
    let cpucount = system.exports.get("cpucount").unwrap();
    let res = cpucount.call(&mut ctx, vec![]).unwrap();
    assert!(res.as_int().unwrap() > 0);

    let memory = system.exports.get("memory").unwrap();
    let res = memory.call(&mut ctx, vec![]).unwrap();
    if let RuntimeValue::Map { entries, .. } = res {
        let entries_borrow = entries.borrow();
        assert!(entries_borrow.get("total").unwrap().as_int().unwrap() > 0);
        assert!(entries_borrow.get("free").unwrap().as_int().unwrap() > 0);
    } else {
        panic!("memory did not return a Map");
    }

    let disk = system.exports.get("disk").unwrap();
    let res = disk.call(&mut ctx, vec![]).unwrap();
    if let RuntimeValue::Map { entries, .. } = res {
        let entries_borrow = entries.borrow();
        assert!(entries_borrow.get("total").unwrap().as_int().unwrap() > 0);
        assert!(entries_borrow.get("free").unwrap().as_int().unwrap() > 0);
    } else {
        panic!("disk did not return a Map");
    }

    // 2. Process run under granted Capability::Process
    let run = process.exports.get("run").unwrap();
    let cmd = if cfg!(windows) { "cmd" } else { "echo" };
    let args = if cfg!(windows) {
        vec![
            RuntimeValue::Str("/c".to_string()),
            RuntimeValue::Str("echo hello".to_string()),
        ]
    } else {
        vec![RuntimeValue::Str("hello".to_string())]
    };

    let res = run
        .call(
            &mut ctx,
            vec![
                RuntimeValue::Str(cmd.to_string()),
                RuntimeValue::List {
                    items: Rc::new(RefCell::new(args)),
                    is_const: false,
                },
            ],
        )
        .unwrap();

    if let RuntimeValue::Map { entries, .. } = res {
        let entries_borrow = entries.borrow();
        assert_eq!(entries_borrow.get("code").unwrap().as_int(), Some(0));
        assert!(entries_borrow
            .get("stdout")
            .unwrap()
            .as_string()
            .unwrap()
            .contains("hello"));
    } else {
        panic!("run did not return a Map");
    }
}

#[test]
fn test_csv_module() {
    let registry = StdlibRegistry::new();
    let csv = registry.get_module("std.csv").unwrap();
    let mut ctx = RuntimeContext::new(RuntimeConfig::default());

    let parse = csv.exports.get("parse").unwrap();
    let stringify = csv.exports.get("stringify").unwrap();

    let csv_str = "a,b\nc,d";
    let res = parse
        .call(&mut ctx, vec![RuntimeValue::Str(csv_str.to_string())])
        .unwrap();

    if let RuntimeValue::List { items, .. } = res {
        let list = items.borrow();
        assert_eq!(list.len(), 2);
        // Test stringify
        let back = stringify
            .call(
                &mut ctx,
                vec![RuntimeValue::List {
                    items: items.clone(),
                    is_const: false,
                }],
            )
            .unwrap();
        assert_eq!(back.as_string().unwrap(), "a,b\nc,d");
    } else {
        panic!("parse did not return a List");
    }
}

#[test]
fn test_xml_module() {
    let registry = StdlibRegistry::new();
    let xml = registry.get_module("std.xml").unwrap();
    let mut ctx = RuntimeContext::new(RuntimeConfig::default());

    let parse = xml.exports.get("parse").unwrap();
    let stringify = xml.exports.get("stringify").unwrap();

    let xml_str = "<user>Tanmoy</user>";
    let res = parse
        .call(&mut ctx, vec![RuntimeValue::Str(xml_str.to_string())])
        .unwrap();

    if let RuntimeValue::Map { entries, .. } = res {
        let entries_borrow = entries.borrow();
        assert_eq!(
            entries_borrow.get("user").unwrap().as_string(),
            Some("Tanmoy")
        );

        let back = stringify
            .call(
                &mut ctx,
                vec![RuntimeValue::Map {
                    entries: entries.clone(),
                    is_const: false,
                }],
            )
            .unwrap();
        assert_eq!(back.as_string().unwrap(), "<user>Tanmoy</user>");
    } else {
        panic!("parse did not return a Map");
    }
}

#[test]
fn test_yaml_module() {
    let registry = StdlibRegistry::new();
    let yaml = registry.get_module("std.yaml").unwrap();
    let mut ctx = RuntimeContext::new(RuntimeConfig::default());

    let parse = yaml.exports.get("parse").unwrap();
    let stringify = yaml.exports.get("stringify").unwrap();

    let yaml_str = "name: Tanmoy\nage: 25\n";
    let res = parse
        .call(&mut ctx, vec![RuntimeValue::Str(yaml_str.to_string())])
        .unwrap();

    if let RuntimeValue::Map { entries, .. } = res {
        let entries_borrow = entries.borrow();
        assert_eq!(
            entries_borrow.get("name").unwrap().as_string(),
            Some("Tanmoy")
        );

        let back = stringify
            .call(
                &mut ctx,
                vec![RuntimeValue::Map {
                    entries: entries.clone(),
                    is_const: false,
                }],
            )
            .unwrap();
        assert!(back.as_string().unwrap().contains("name: Tanmoy"));
    } else {
        panic!("parse did not return a Map");
    }
}

#[test]
fn test_toml_module() {
    let registry = StdlibRegistry::new();
    let toml = registry.get_module("std.toml").unwrap();
    let mut ctx = RuntimeContext::new(RuntimeConfig::default());

    let parse = toml.exports.get("parse").unwrap();
    let stringify = toml.exports.get("stringify").unwrap();

    let toml_str = "title = \"TOML Example\"\n[owner]\nname = \"Tanmoy\"\n";
    let res = parse
        .call(&mut ctx, vec![RuntimeValue::Str(toml_str.to_string())])
        .unwrap();

    if let RuntimeValue::Map { entries, .. } = res {
        let entries_borrow = entries.borrow();
        assert_eq!(
            entries_borrow.get("title").unwrap().as_string(),
            Some("TOML Example")
        );

        let back = stringify
            .call(
                &mut ctx,
                vec![RuntimeValue::Map {
                    entries: entries.clone(),
                    is_const: false,
                }],
            )
            .unwrap();
        assert!(back
            .as_string()
            .unwrap()
            .contains("title = \"TOML Example\""));
    } else {
        panic!("parse did not return a Map");
    }
}

#[test]
fn test_database_module() {
    let registry = StdlibRegistry::new();
    let db = registry.get_module("std.database").unwrap();

    // Grant FileSystem capability
    let mut caps = HashSet::new();
    caps.insert(Capability::FileSystem);
    let mut ctx = RuntimeContext::new(RuntimeConfig {
        strict_mode: false,
        max_recursion_depth: 1000,
        enable_assertions: true,
        capabilities: caps,
    });

    let connect = db.exports.get("connect").unwrap();
    let query = db.exports.get("query").unwrap();
    let execute = db.exports.get("execute").unwrap();
    let close = db.exports.get("close").unwrap();

    // 1. Connect in-memory
    let conn_handle = connect
        .call(&mut ctx, vec![RuntimeValue::Str(":memory:".to_string())])
        .unwrap();
    assert!(conn_handle.as_int().is_some());

    // 2. Create table
    let res = execute
        .call(
            &mut ctx,
            vec![
                conn_handle.clone(),
                RuntimeValue::Str(
                    "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)".to_string(),
                ),
            ],
        )
        .unwrap();
    assert_eq!(res.as_int(), Some(0));

    // 3. Insert user
    let res = execute
        .call(
            &mut ctx,
            vec![
                conn_handle.clone(),
                RuntimeValue::Str("INSERT INTO users (id, name) VALUES (?, ?)".to_string()),
                RuntimeValue::List {
                    items: Rc::new(RefCell::new(vec![
                        RuntimeValue::Int(42),
                        RuntimeValue::Str("Tanmoy".to_string()),
                    ])),
                    is_const: false,
                },
            ],
        )
        .unwrap();
    assert_eq!(res.as_int(), Some(1));

    // 4. Query user
    let res = query
        .call(
            &mut ctx,
            vec![
                conn_handle.clone(),
                RuntimeValue::Str("SELECT id, name FROM users WHERE id = ?".to_string()),
                RuntimeValue::List {
                    items: Rc::new(RefCell::new(vec![RuntimeValue::Int(42)])),
                    is_const: false,
                },
            ],
        )
        .unwrap();

    if let RuntimeValue::List { items, .. } = res {
        let list = items.borrow();
        assert_eq!(list.len(), 1);
        if let RuntimeValue::Map { entries, .. } = &list[0] {
            let entries_borrow = entries.borrow();
            assert_eq!(entries_borrow.get("id").unwrap().as_int(), Some(42));
            assert_eq!(
                entries_borrow.get("name").unwrap().as_string(),
                Some("Tanmoy")
            );
        } else {
            panic!("row is not a Map");
        }
    } else {
        panic!("query did not return a List");
    }

    // 5. Close connection
    let closed = close.call(&mut ctx, vec![conn_handle]).unwrap();
    assert_eq!(closed, RuntimeValue::Bool(true));
}

#[test]
fn test_async_and_channels() {
    let registry = StdlibRegistry::new();
    let async_mod = registry.get_module("std.async").unwrap();
    let channel_mod = registry.get_module("std.channel").unwrap();

    let mut ctx = RuntimeContext::new(RuntimeConfig::default());

    let spawn_async = async_mod.exports.get("spawn_async").unwrap();
    let make_channel = channel_mod.exports.get("make_channel").unwrap();
    let send_channel = channel_mod.exports.get("send_channel").unwrap();
    let recv_channel = channel_mod.exports.get("recv_channel").unwrap();

    // 1. Create a channel
    let chan = make_channel.call(&mut ctx, vec![]).unwrap();

    // Send a value
    send_channel
        .call(&mut ctx, vec![chan.clone(), RuntimeValue::Int(100)])
        .unwrap();

    // Receive a value
    let val = recv_channel.call(&mut ctx, vec![chan]).unwrap();
    assert_eq!(val.as_int(), Some(100));

    // 2. Spawn task and tick the async runtime cooperatively
    let callback = Rc::new(techscript_stdlib::StdFunction {
        name: "cb".to_string(),
        arity: 0,
        callback: |_ctx, _args| Ok(RuntimeValue::Str("Async Work Done".to_string())),
    });

    let future = spawn_async
        .call(&mut ctx, vec![RuntimeValue::Function(callback)])
        .unwrap();

    // Check initial state
    if let RuntimeValue::Map { entries, .. } = &future {
        assert_eq!(
            entries.borrow().get("state").unwrap().as_string(),
            Some("pending")
        );
    }

    // Tick the async runtime
    techscript_stdlib::async_runtime::tick();

    // Check completed state
    if let RuntimeValue::Map { entries, .. } = &future {
        assert_eq!(
            entries.borrow().get("state").unwrap().as_string(),
            Some("resolved")
        );
        assert_eq!(
            entries.borrow().get("value").unwrap().as_string(),
            Some("Async Work Done")
        );
    }
}

#[test]
fn test_crypto_hash_and_compression() {
    let registry = StdlibRegistry::new();
    let crypto = registry.get_module("std.crypto").unwrap();
    let hash = registry.get_module("std.hash").unwrap();
    let compress = registry.get_module("std.compress").unwrap();

    let mut config_unprivileged = RuntimeConfig::default();
    config_unprivileged
        .capabilities
        .remove(&Capability::FileSystem);
    let mut ctx_unprivileged = RuntimeContext::new(config_unprivileged);

    let mut config_fs = RuntimeConfig::default();
    config_fs.capabilities.insert(Capability::FileSystem);
    let mut ctx_fs = RuntimeContext::new(config_fs);

    // 1. Test hash operations
    let md5_fn = hash.exports.get("md5").unwrap();
    let val = md5_fn
        .call(
            &mut ctx_unprivileged,
            vec![RuntimeValue::Str("hello".to_string())],
        )
        .unwrap();
    assert_eq!(val.as_string(), Some("5d41402abc4b2a76b9719d911017c592"));

    let sha256_fn = hash.exports.get("sha256").unwrap();
    let val = sha256_fn
        .call(
            &mut ctx_unprivileged,
            vec![RuntimeValue::Str("hello".to_string())],
        )
        .unwrap();
    assert_eq!(
        val.as_string(),
        Some("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824")
    );

    let crc32_fn = hash.exports.get("crc32").unwrap();
    let val = crc32_fn
        .call(
            &mut ctx_unprivileged,
            vec![RuntimeValue::Str("hello".to_string())],
        )
        .unwrap();
    assert!(val.as_int().is_some());

    // 2. Test crypto operations (AES-GCM & Bcrypt)
    let aes_enc = crypto.exports.get("aes_encrypt").unwrap();
    let aes_dec = crypto.exports.get("aes_decrypt").unwrap();

    let key = RuntimeValue::Str("my_secret_key_123".to_string());
    let plain = RuntimeValue::Str("hello crypto world".to_string());

    let encrypted = aes_enc
        .call(&mut ctx_unprivileged, vec![key.clone(), plain])
        .unwrap();
    let decrypted = aes_dec
        .call(&mut ctx_unprivileged, vec![key, encrypted])
        .unwrap();
    assert_eq!(decrypted.as_string(), Some("hello crypto world"));

    let bcrypt_hash = crypto.exports.get("bcrypt_hash").unwrap();
    let bcrypt_verify = crypto.exports.get("bcrypt_verify").unwrap();

    let pass = RuntimeValue::Str("super_secure_pass".to_string());
    let hashed = bcrypt_hash
        .call(
            &mut ctx_unprivileged,
            vec![pass.clone(), RuntimeValue::Int(4)],
        )
        .unwrap();
    assert!(hashed.as_string().is_some());

    let is_valid = bcrypt_verify
        .call(&mut ctx_unprivileged, vec![pass, hashed.clone()])
        .unwrap();
    assert_eq!(is_valid.as_bool(), Some(true));

    // 3. Test compression capabilities & operations
    let temp_dir = std::env::temp_dir().join("techscript_archive_test");
    std::fs::create_dir_all(&temp_dir).ok();

    let test_file = temp_dir.join("test.txt");
    std::fs::write(&test_file, "archiving content here").ok();

    let zip_fn = compress.exports.get("zip").unwrap();
    let unzip_fn = compress.exports.get("unzip").unwrap();

    let archive = temp_dir.join("archive.zip");

    // Unprivileged context should fail
    let res = zip_fn.call(
        &mut ctx_unprivileged,
        vec![
            RuntimeValue::Str(temp_dir.to_string_lossy().to_string()),
            RuntimeValue::Str(archive.to_string_lossy().to_string()),
        ],
    );
    assert!(res.is_err());

    // Privileged context should succeed
    let res = zip_fn.call(
        &mut ctx_fs,
        vec![
            RuntimeValue::Str(temp_dir.to_string_lossy().to_string()),
            RuntimeValue::Str(archive.to_string_lossy().to_string()),
        ],
    );
    assert!(res.is_ok());

    let extract_dir = temp_dir.join("extracted");
    let res = unzip_fn.call(
        &mut ctx_fs,
        vec![
            RuntimeValue::Str(archive.to_string_lossy().to_string()),
            RuntimeValue::Str(extract_dir.to_string_lossy().to_string()),
        ],
    );
    assert!(res.is_ok());

    let extracted_file = extract_dir.join("test.txt");
    assert!(extracted_file.exists());
    let content = std::fs::read_to_string(extracted_file).unwrap();
    assert_eq!(content, "archiving content here");

    // Clean up
    std::fs::remove_dir_all(&temp_dir).ok();
}

#[test]
fn test_graphics_canvas_drawing() {
    let registry = StdlibRegistry::new();
    let graphics = registry.get_module("std.graphics").unwrap();

    let mut config_unprivileged = RuntimeConfig::default();
    config_unprivileged
        .capabilities
        .remove(&Capability::FileSystem);
    let mut ctx_unprivileged = RuntimeContext::new(config_unprivileged);

    let mut config_fs = RuntimeConfig::default();
    config_fs.capabilities.insert(Capability::FileSystem);
    let mut ctx_fs = RuntimeContext::new(config_fs);

    let create_canvas_fn = graphics.exports.get("create_canvas").unwrap();
    let draw_rect_fn = graphics.exports.get("draw_rect").unwrap();
    let draw_circle_fn = graphics.exports.get("draw_circle").unwrap();
    let draw_line_fn = graphics.exports.get("draw_line").unwrap();
    let save_png_fn = graphics.exports.get("save_png").unwrap();

    // 1. Create a 100x100 canvas
    let canvas_handle_val = create_canvas_fn
        .call(
            &mut ctx_unprivileged,
            vec![RuntimeValue::Int(100), RuntimeValue::Int(100)],
        )
        .unwrap();
    let handle = canvas_handle_val.as_int().unwrap();
    assert!(handle > 0);

    // 2. Draw some shapes
    // Draw red rectangle
    draw_rect_fn
        .call(
            &mut ctx_unprivileged,
            vec![
                RuntimeValue::Int(handle),
                RuntimeValue::Int(10),                    // x
                RuntimeValue::Int(10),                    // y
                RuntimeValue::Int(50),                    // w
                RuntimeValue::Int(30),                    // h
                RuntimeValue::Str("#ff0000".to_string()), // color
            ],
        )
        .unwrap();

    // Draw green circle
    draw_circle_fn
        .call(
            &mut ctx_unprivileged,
            vec![
                RuntimeValue::Int(handle),
                RuntimeValue::Int(50), // cx
                RuntimeValue::Int(50), // cy
                RuntimeValue::Int(20), // r
                RuntimeValue::Str("#00ff00".to_string()),
            ],
        )
        .unwrap();

    // Draw blue line
    draw_line_fn
        .call(
            &mut ctx_unprivileged,
            vec![
                RuntimeValue::Int(handle),
                RuntimeValue::Int(0),  // x1
                RuntimeValue::Int(0),  // y1
                RuntimeValue::Int(99), // x2
                RuntimeValue::Int(99), // y2
                RuntimeValue::Str("#0000ff".to_string()),
            ],
        )
        .unwrap();

    // 3. Save to PNG file (requires FileSystem capability)
    let temp_file = std::env::temp_dir().join("test_canvas.png");
    let temp_file_str = temp_file.to_string_lossy().to_string();

    // Unprivileged should fail (due to missing FileSystem capability)
    let res = save_png_fn.call(
        &mut ctx_unprivileged,
        vec![
            RuntimeValue::Int(handle),
            RuntimeValue::Str(temp_file_str.clone()),
        ],
    );
    assert!(res.is_err(), "Expected security error, got: {:?}", res);

    // Privileged should succeed (creating and saving canvas under ctx_fs)
    let canvas_fs_val = create_canvas_fn
        .call(
            &mut ctx_fs,
            vec![RuntimeValue::Int(10), RuntimeValue::Int(10)],
        )
        .unwrap();
    let handle_fs = canvas_fs_val.as_int().unwrap();

    let res = save_png_fn.call(
        &mut ctx_fs,
        vec![
            RuntimeValue::Int(handle_fs),
            RuntimeValue::Str(temp_file_str.clone()),
        ],
    );
    assert!(res.is_ok(), "Expected success, got error: {:?}", res.err());

    assert!(temp_file.exists());
    std::fs::remove_file(temp_file).ok();
}

#[test]
fn test_ai_generate_text() {
    let registry = StdlibRegistry::new();
    let ai = registry.get_module("std.ai").unwrap();

    let mut config_unprivileged = RuntimeConfig::default();
    config_unprivileged
        .capabilities
        .remove(&Capability::Environment);
    config_unprivileged
        .capabilities
        .remove(&Capability::Network);
    let mut ctx_unprivileged = RuntimeContext::new(config_unprivileged);

    let mut config_privileged = RuntimeConfig::default();
    config_privileged
        .capabilities
        .insert(Capability::Environment);
    config_privileged.capabilities.insert(Capability::Network);
    let mut ctx_privileged = RuntimeContext::new(config_privileged);

    let generate_text_fn = ai.exports.get("generate_text").unwrap();

    // 1. Unprivileged context should fail with security policy violation
    let res = generate_text_fn.call(
        &mut ctx_unprivileged,
        vec![
            RuntimeValue::Str("openai".to_string()),
            RuntimeValue::Str("What is 2+2?".to_string()),
            RuntimeValue::Map {
                entries: Rc::new(RefCell::new(indexmap::IndexMap::new())),
                is_const: false,
            },
        ],
    );
    assert!(res.is_err());

    // 2. Privileged context should succeed (with mock fallback or real API calls)
    let res = generate_text_fn.call(
        &mut ctx_privileged,
        vec![
            RuntimeValue::Str("openai".to_string()),
            RuntimeValue::Str("What is 2+2?".to_string()),
            RuntimeValue::Map {
                entries: Rc::new(RefCell::new(indexmap::IndexMap::new())),
                is_const: false,
            },
        ],
    );
    assert!(res.is_ok());
    let val = res.unwrap();
    assert!(val.as_string().unwrap().contains("Prompt: What is 2+2?"));
}

#[test]
fn test_tar_dir_error_path() {
    let temp_dir = std::env::temp_dir().join("techscript_tar_error_test");
    std::fs::create_dir_all(&temp_dir).ok();

    let dest_file = temp_dir.join("archive.tar");
    let result = techscript_stdlib::compress::tar_dir(
        "/non/existent/path/for/sure",
        &dest_file.to_string_lossy(),
    );
    assert!(result.is_err());

    std::fs::remove_dir_all(&temp_dir).ok();
}
