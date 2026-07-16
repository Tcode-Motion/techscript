use crate::value::RuntimeValue;
use indexmap::IndexMap;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_OBJECT_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectId(pub u64);

impl ObjectId {
    /// Generates a globally unique incrementing object identifier.
    pub fn next() -> Self {
        Self(NEXT_OBJECT_ID.fetch_add(1, Ordering::SeqCst))
    }
}

/// Runtime struct instance.
#[derive(Debug, Clone)]
pub struct StructInstance {
    pub id: ObjectId,
    pub name: String,
    pub fields: IndexMap<String, RuntimeValue>,
    pub is_const: bool,
}

impl StructInstance {
    pub fn new(name: String, fields: IndexMap<String, RuntimeValue>, is_const: bool) -> Self {
        Self {
            id: ObjectId::next(),
            name,
            fields,
            is_const,
        }
    }
}

/// Runtime class/model instance.
#[derive(Debug, Clone)]
pub struct ModelInstance {
    pub id: ObjectId,
    pub name: String,
    pub fields: IndexMap<String, RuntimeValue>,
}

impl ModelInstance {
    pub fn new(name: String, fields: IndexMap<String, RuntimeValue>) -> Self {
        Self {
            id: ObjectId::next(),
            name,
            fields,
        }
    }
}
