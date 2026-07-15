use techscript_interpreter::Value;
use techscript_gc::{DummyGC, GarbageCollector};

#[test]
fn test_gc_allocator() {
    let mut gc = DummyGC::new();
    let ptr = gc.allocate(Value::Int(42));
    assert_eq!(ptr, 1);
}
