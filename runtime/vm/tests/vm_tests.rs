use techscript_interpreter::Value;
use techscript_vm::{Instruction, OpCode, VM};

#[test]
fn test_vm_execution() {
    let mut vm = VM::new();
    let instructions = vec![Instruction {
        op: OpCode::LoadConst,
        operand: Some(0),
    }];
    let val = vm.execute(&instructions).expect("execution should run");
    assert_eq!(val, Value::None);
}
