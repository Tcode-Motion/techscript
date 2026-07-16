use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Precedence binding power levels or concrete types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Type {
    Int,
    Float,
    Str,
    Bool,
    None,
    Any,
    Struct(String),
    Enum(String),
    Model(String),
    Generic(String),
    List(TypeId),
    Map(TypeId, TypeId),
    Function { params: Vec<TypeId>, ret_ty: TypeId },
    Unknown,
}

/// Newtype wrapper representing a unique interned type reference index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TypeId(pub usize);

/// Cache pool mapping type structures to individual unique index IDs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeInterner {
    types: Vec<Type>,
    #[serde(skip)]
    map: HashMap<Type, TypeId>,
}

impl Default for TypeInterner {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeInterner {
    /// Creates and pre-populates a type interner pool with primitives.
    pub fn new() -> Self {
        let mut interner = Self {
            types: Vec::new(),
            map: HashMap::new(),
        };
        interner.intern(Type::Int);
        interner.intern(Type::Float);
        interner.intern(Type::Str);
        interner.intern(Type::Bool);
        interner.intern(Type::None);
        interner.intern(Type::Any);
        interner.intern(Type::Unknown);
        interner
    }

    /// Registers a type structure returning its TypeId.
    pub fn intern(&mut self, ty: Type) -> TypeId {
        if self.map.is_empty() && !self.types.is_empty() {
            // Rebuild map after deserialization
            for (idx, t) in self.types.iter().enumerate() {
                self.map.insert(t.clone(), TypeId(idx));
            }
        }

        if let Some(&id) = self.map.get(&ty) {
            return id;
        }
        let id = TypeId(self.types.len());
        self.types.push(ty.clone());
        self.map.insert(ty, id);
        id
    }

    /// Retrieves type structure reference using its index ID.
    pub fn get(&self, id: TypeId) -> &Type {
        &self.types[id.0]
    }

    pub fn int(&self) -> TypeId {
        TypeId(0)
    }
    pub fn float(&self) -> TypeId {
        TypeId(1)
    }
    pub fn string(&self) -> TypeId {
        TypeId(2)
    }
    pub fn bool(&self) -> TypeId {
        TypeId(3)
    }
    pub fn none(&self) -> TypeId {
        TypeId(4)
    }
    pub fn any(&self) -> TypeId {
        TypeId(5)
    }
    pub fn unknown(&self) -> TypeId {
        TypeId(6)
    }
}
