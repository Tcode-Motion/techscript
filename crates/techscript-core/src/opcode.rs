// ── TechScript Bytecode Opcodes ──────────────────────────────────────

/// Bytecode instructions for the TechScript VM.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum OpCode {
    /// Push a constant from the constant pool.
    Constant,
    /// Push none.
    None,
    /// Push true.
    True,
    /// Push false.
    False,
    /// Pop and discard the top of stack.
    Pop,

    // ── Variables ────────────────────────────────────────────────────
    /// Get a global variable by name (index into constant pool).
    GetGlobal,
    /// Set a global variable by name.
    SetGlobal,
    /// Define a global variable.
    DefineGlobal,
    /// Get a local variable by slot index.
    GetLocal,
    /// Set a local variable by slot index.
    SetLocal,
    /// Get an upvalue (closure variable).
    GetUpvalue,
    /// Set an upvalue.
    SetUpvalue,

    // ── Arithmetic ──────────────────────────────────────────────────
    Add,
    Subtract,
    Multiply,
    Divide,
    IntDivide,
    Modulo,
    Power,
    Negate,

    // ── Comparison ──────────────────────────────────────────────────
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Not,

    // ── Logical ─────────────────────────────────────────────────────
    And,
    Or,
    /// Collection containment: `x in list/map/string`
    In,
    /// Return type name string of top-of-stack value
    TypeOf,

    // ── Output / Input ──────────────────────────────────────────────
    /// `say` — print N values from stack.
    Print,
    /// `ask` — read input from stdin.
    ReadInput,

    // ── Control flow ────────────────────────────────────────────────
    /// Unconditional jump forward.
    Jump,
    /// Jump forward if top of stack is falsy.
    JumpIfFalse,
    /// Jump backward (loops).
    Loop,

    // ── Functions ───────────────────────────────────────────────────
    /// Call a function with N arguments.
    Call,
    /// Return from the current function.
    Return,
    /// Create a closure.
    Closure,
    /// Close an upvalue.
    CloseUpvalue,

    // ── Collections ─────────────────────────────────────────────────
    /// Build a list from N elements on stack.
    BuildList,
    /// Build a map from N key-value pairs on stack.
    BuildMap,
    /// Index access `obj[index]`.
    Index,
    /// Index set `obj[index] = value`.
    SetIndex,

    // ── Classes & Objects ───────────────────────────────────────────
    /// Define a class.
    Class,
    /// Get a property `obj.member`.
    GetProperty,
    /// Set a property `obj.member = val`.
    SetProperty,
    /// Define a method on a class.
    Method,
    /// Invoke a method (optimised get+call).
    Invoke,
    /// Set up class inheritance.
    Inherit,

    // ── Modules ─────────────────────────────────────────────────────
    /// Import a module.
    Import,

    // ── Iteration ───────────────────────────────────────────────────
    /// Create an iterator from an iterable.
    GetIter,
    /// Advance iterator; push next value or jump if done.
    IterNext,

    // ── Range ───────────────────────────────────────────────────────
    /// Build a range object from start/end.
    BuildRange,
    /// Build an inclusive range.
    BuildRangeInclusive,

    // ── Exceptions ──────────────────────────────────────────────────
    /// Setup a try block (takes 16-bit offset to catch block).
    SetupTry,
    /// Pop a try block.
    PopTry,
    /// Throw an exception.
    Throw,

    // ── String interpolation ────────────────────────────────────────
    /// Format an f-string with N parts.
    FormatString,

    // ── Misc ────────────────────────────────────────────────────────
    /// Duplicate the top of the stack
    Dup,
    /// Await a task (currently evaluates argument sync)
    Await,
    /// Spawn a task into background queue
    Spawn,
}

impl TryFrom<u8> for OpCode {
    type Error = ();

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0  => Ok(OpCode::Constant),
            1  => Ok(OpCode::None),
            2  => Ok(OpCode::True),
            3  => Ok(OpCode::False),
            4  => Ok(OpCode::Pop),
            5  => Ok(OpCode::GetGlobal),
            6  => Ok(OpCode::SetGlobal),
            7  => Ok(OpCode::DefineGlobal),
            8  => Ok(OpCode::GetLocal),
            9  => Ok(OpCode::SetLocal),
            10 => Ok(OpCode::GetUpvalue),
            11 => Ok(OpCode::SetUpvalue),
            12 => Ok(OpCode::Add),
            13 => Ok(OpCode::Subtract),
            14 => Ok(OpCode::Multiply),
            15 => Ok(OpCode::Divide),
            16 => Ok(OpCode::IntDivide),
            17 => Ok(OpCode::Modulo),
            18 => Ok(OpCode::Power),
            19 => Ok(OpCode::Negate),
            20 => Ok(OpCode::Equal),
            21 => Ok(OpCode::NotEqual),
            22 => Ok(OpCode::Less),
            23 => Ok(OpCode::Greater),
            24 => Ok(OpCode::LessEqual),
            25 => Ok(OpCode::GreaterEqual),
            26 => Ok(OpCode::Not),
            27 => Ok(OpCode::And),
            28 => Ok(OpCode::Or),
            29 => Ok(OpCode::In),
            30 => Ok(OpCode::TypeOf),
            31 => Ok(OpCode::Print),
            32 => Ok(OpCode::ReadInput),
            33 => Ok(OpCode::Jump),
            34 => Ok(OpCode::JumpIfFalse),
            35 => Ok(OpCode::Loop),
            36 => Ok(OpCode::Call),
            37 => Ok(OpCode::Return),
            38 => Ok(OpCode::Closure),
            39 => Ok(OpCode::CloseUpvalue),
            40 => Ok(OpCode::BuildList),
            41 => Ok(OpCode::BuildMap),
            42 => Ok(OpCode::Index),
            43 => Ok(OpCode::SetIndex),
            44 => Ok(OpCode::Class),
            45 => Ok(OpCode::GetProperty),
            46 => Ok(OpCode::SetProperty),
            47 => Ok(OpCode::Method),
            48 => Ok(OpCode::Invoke),
            49 => Ok(OpCode::Inherit),
            50 => Ok(OpCode::Import),
            51 => Ok(OpCode::GetIter),
            52 => Ok(OpCode::IterNext),
            53 => Ok(OpCode::BuildRange),
            54 => Ok(OpCode::BuildRangeInclusive),
            55 => Ok(OpCode::SetupTry),
            56 => Ok(OpCode::PopTry),
            57 => Ok(OpCode::Throw),
            58 => Ok(OpCode::FormatString),
            59 => Ok(OpCode::Dup),
            60 => Ok(OpCode::Await),
            61 => Ok(OpCode::Spawn),
            _  => Err(()),
        }
    }
}
