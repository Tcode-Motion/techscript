use crate::gc::HeapObject;
use std::rc::Rc;

/// Allocator heap registry tracking dynamic references.
#[derive(Default)]
pub struct VMHeap {
    objects: Vec<Rc<dyn HeapObject>>,
}

impl VMHeap {
    /// Creates a VMHeap structure.
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    /// Registers a heap-allocated object, returning its dynamic index key.
    pub fn register(&mut self, obj: Rc<dyn HeapObject>) -> u32 {
        let idx = self.objects.len() as u32;
        self.objects.push(obj);
        idx
    }

    /// Clears and purges unreached heap references.
    pub fn gc_sweep(&mut self) {
        // Placeholder sweep
    }
}
