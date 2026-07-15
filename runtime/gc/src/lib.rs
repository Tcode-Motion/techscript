//! # TechScript Garbage Collector Crate
//!
//! Specification and interface definitions for the dynamic allocation collector.
//! To be fully implemented in v2.1 as a generational mark-and-sweep garbage collector.

use techscript_interpreter::Value;

/// Tracing Garbage Collector interface traits.
pub trait GarbageCollector {
    /// Allocate a value on the managed heap.
    fn allocate(&mut self, value: Value) -> usize;
    /// Traces active references and reclaims unreferenced cells.
    fn collect(&mut self, roots: &[usize]);
}

/// Skeletal tracking allocator.
#[derive(Default)]
pub struct DummyGC {
    allocated_count: usize,
}

impl DummyGC {
    pub fn new() -> Self {
        Self { allocated_count: 0 }
    }
}

impl GarbageCollector for DummyGC {
    fn allocate(&mut self, _value: Value) -> usize {
        self.allocated_count += 1;
        self.allocated_count
    }

    fn collect(&mut self, _roots: &[usize]) {
        // No-op in v2.0
    }
}
