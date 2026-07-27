use crate::types::{BlockId, IRType, InstructionId, LocalId, ValueId};
use crate::value::Value;
use serde::{Deserialize, Serialize};
use techscript_ast::LiteralVal;
use techscript_common::Span;
use techscript_syntax::TokenKind;

/// Strongly typed metadata carrying optimizer and codegen hints.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct InstructionMetadata {
    pub inline_hint: bool,
    pub optimization_hint: Option<String>,
    pub llvm_attributes: Vec<String>,
}

/// A sequential instruction node within a basic block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instruction {
    pub id: InstructionId,
    pub op: Op,
    pub result: Option<ValueId>,
    pub ty: IRType,
    pub span: Span,
    pub metadata: InstructionMetadata,
}

/// IR Instruction operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Op {
    /// Loads a constant literal value.
    Constant(LiteralVal),
    /// Loads from local/global variable memory space.
    Load(Value),
    /// Stores an operand value into local/global memory.
    Store { target: Value, value: Value },
    /// Copies/moves a value to a temporary register.
    Move { target: ValueId, value: Value },
    /// Standard arithmetic/bitwise binary operations.
    BinaryOp {
        op: TokenKind,
        left: Value,
        right: Value,
    },
    /// Unary operations.
    UnaryOp { op: TokenKind, right: Value },
    /// Logical comparison operations.
    Compare {
        op: TokenKind,
        left: Value,
        right: Value,
    },
    /// SSA Phi Node mapping previous block paths to a value.
    Phi(Vec<(BlockId, Value)>),
    /// Function/Callable invocation.
    Call { callee: Value, args: Vec<Value> },
    /// Allocates memory slot for local variables.
    Allocate(IRType),
    /// Structural field read.
    FieldLoad { base: Value, field: String },
    /// Structural field write mutation.
    FieldStore {
        base: Value,
        field: String,
        value: Value,
    },
    /// Collection index read.
    IndexLoad { base: Value, index: Value },
    /// Collection index write mutation.
    IndexStore {
        base: Value,
        index: Value,
        value: Value,
    },
    /// Allocates struct instance.
    MakeStruct {
        name: String,
        fields: Vec<(String, Value)>,
    },
    /// Allocates enum variant.
    MakeEnum {
        name: String,
        variant: String,
        value: Option<Box<Value>>,
    },
    /// Allocates model instance.
    MakeModel {
        name: String,
        fields: Vec<(String, Value)>,
    },
    /// Constructs a List.
    MakeList(Vec<Value>),
    /// Constructs a Map.
    MakeMap(Vec<(Value, Value)>),
    /// Conversions.
    Cast { value: Value, target_type: IRType },
    /// Setup try/catch handler
    Try {
        catch_block: BlockId,
        catch_var: LocalId,
    },
    /// Pop try/catch handler
    EndTry,
    /// Construct a DSL block value with its kind, args, properties, and children refs.
    MakeDslBlock {
        kind: String,
        args: Vec<Value>,
        properties: Vec<(String, Option<Value>)>,
        children: Vec<Value>,
    },
    /// No operation.
    NoOp,
}

/// Control-flow terminator ending a basic block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Terminator {
    pub kind: TerminatorKind,
    pub span: Span,
}

/// Terminator target variants.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerminatorKind {
    /// Unconditional jump to basic block target.
    Jump(BlockId),
    /// Conditional branch split.
    ConditionalJump {
        cond: Value,
        then_block: BlockId,
        else_block: BlockId,
    },
    /// Returns control back to parent caller.
    Return(Option<Value>),
    /// Signals unreachable static program paths.
    Unreachable,
    /// Throw an exception with a value.
    Throw(Value),
}
