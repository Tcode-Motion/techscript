use serde::{Deserialize, Serialize};

/// Operations supported by the TechScript 2.0 Virtual Machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Opcode {
    // ── Stack Operations ────────────────────────────────────────────────────
    Pop,
    Dup,
    Swap,

    // ── Value Loading and Storing ───────────────────────────────────────────
    LoadConst,
    LoadLocal,
    StoreLocal,
    LoadGlobal,
    StoreGlobal,

    // ── Arithmetic Operations ───────────────────────────────────────────────
    Add,
    Sub,
    Mul,
    Div,
    IntDiv,
    Mod,
    Pow,

    // ── Unary Operations ────────────────────────────────────────────────────
    Neg,
    Not,

    // ── Logical Operations ──────────────────────────────────────────────────
    And,
    Or,

    // ── Comparison Operations ───────────────────────────────────────────────
    Equal,
    StrictEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // ── Control Flow ────────────────────────────────────────────────────────
    Jump,
    JumpIfTrue,
    JumpIfFalse,
    Call,
    Return,

    // ── Collections ─────────────────────────────────────────────────────────
    MakeList,
    MakeMap,
    IndexLoad,
    IndexStore,

    // ── OOP / Structures ────────────────────────────────────────────────────
    MakeStruct,
    MakeEnum,
    MakeModel,
    FieldLoad,
    FieldStore,

    // ── Closures ────────────────────────────────────────────────────────────
    Capture,
    LoadUpvalue,
    StoreUpvalue,
    CloseUpvalue,

    // ── Exceptions ──────────────────────────────────────────────────────────
    Throw,
    Try,
    EndTry,

    // ── Miscellaneous ───────────────────────────────────────────────────────
    NoOp,
}

impl Opcode {
    /// Returns the net stack effect (push - pop) of executing the opcode.
    pub fn stack_effect(&self) -> i32 {
        match self {
            Opcode::Pop => -1,
            Opcode::Dup => 1,
            Opcode::Swap => 0,

            Opcode::LoadConst => 1,
            Opcode::LoadLocal => 1,
            Opcode::StoreLocal => 0, // Leaves value on stack or pops depending on compiler semantics; here we assume store consumes nothing (or pops; let's treat as popping 1 value, i.e. -1)
            Opcode::LoadGlobal => 1,
            Opcode::StoreGlobal => -1,

            Opcode::Add
            | Opcode::Sub
            | Opcode::Mul
            | Opcode::Div
            | Opcode::IntDiv
            | Opcode::Mod
            | Opcode::Pow => -1, // Pops 2, pushes 1

            Opcode::Neg | Opcode::Not => 0, // Pops 1, pushes 1

            Opcode::And | Opcode::Or => -1,

            Opcode::Equal
            | Opcode::StrictEqual
            | Opcode::NotEqual
            | Opcode::Less
            | Opcode::LessEqual
            | Opcode::Greater
            | Opcode::GreaterEqual => -1,

            Opcode::Jump => 0,
            Opcode::JumpIfTrue | Opcode::JumpIfFalse => -1,
            Opcode::Call => 0, // Dynamic, resolved by call arguments count
            Opcode::Return => -1,

            Opcode::MakeList => 1,
            Opcode::MakeMap => 1,
            Opcode::IndexLoad => -1, // Pops base & index, pushes element -> -1
            Opcode::IndexStore => -3, // Pops base, index & value -> -3

            Opcode::MakeStruct | Opcode::MakeEnum | Opcode::MakeModel => 1,
            Opcode::FieldLoad => 0,   // Pops base, pushes field val -> 0
            Opcode::FieldStore => -2, // Pops base & value -> -2

            Opcode::Capture => 1,
            Opcode::LoadUpvalue => 1,
            Opcode::StoreUpvalue => -1,
            Opcode::CloseUpvalue => 0,

            Opcode::Throw => -1,
            Opcode::Try => 0,
            Opcode::EndTry => 0,

            Opcode::NoOp => 0,
        }
    }
}
