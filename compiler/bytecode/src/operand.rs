use serde::{Deserialize, Serialize};

/// Represents self-describing VM instruction operands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Operand {
    Register(u32),
    ConstantIndex(u32),
    FunctionIndex(u32),
    JumpOffset(i32),
    LocalIndex(u32),
    GlobalIndex(u32),
    Count(u32),
}
