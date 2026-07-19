use serde::{Deserialize, Serialize};

/// Strongly typed intermediate representation types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IRType {
    Int64,
    Float64,
    Bool,
    String,
    List,
    Map,
    Struct(String),
    Enum(String),
    Model(String),
    DslBlock(String),
    Pointer,
    Void,
    Any,
}

impl std::fmt::Display for IRType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IRType::Int64 => write!(f, "int64"),
            IRType::Float64 => write!(f, "float64"),
            IRType::Bool => write!(f, "bool"),
            IRType::String => write!(f, "string"),
            IRType::List => write!(f, "list"),
            IRType::Map => write!(f, "map"),
            IRType::Struct(name) => write!(f, "struct {}", name),
            IRType::Enum(name) => write!(f, "enum {}", name),
            IRType::Model(name) => write!(f, "model {}", name),
            IRType::DslBlock(name) => write!(f, "dsl_block {}", name),
            IRType::Pointer => write!(f, "ptr"),
            IRType::Void => write!(f, "void"),
            IRType::Any => write!(f, "any"),
        }
    }
}

/// Type-safe identifier for temporary IR values / virtual registers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ValueId(pub u32);

/// Type-safe identifier for local function variable scopes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LocalId(pub u32);

/// Type-safe identifier for global program variables.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct GlobalId(pub u32);

/// Type-safe identifier for basic blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BlockId(pub u32);

/// Type-safe identifier for functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FunctionId(pub u32);

/// Type-safe identifier for instructions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct InstructionId(pub u32);

/// Type-safe identifier for DSL blocks in the module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DslBlockId(pub u32);
