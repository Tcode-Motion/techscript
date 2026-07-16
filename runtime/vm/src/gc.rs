/// Trait representing markable objects in GC root sweep scans.
pub trait HeapObject {
    /// Marks the object as reached.
    fn mark(&self);
}

/// Garbage collection executor interface.
pub trait GarbageCollector {
    /// Triggers an immediate collect sweep.
    fn collect(&mut self);
}
