use crate::error::VMError;
use crate::frame::{CallFrame, ExceptionHandler};
use crate::vm::VM;
use std::cell::RefCell;
use std::rc::Rc;
use techscript_ast::LiteralVal;
use techscript_bytecode::{Opcode, Operand};
use techscript_runtime::{collections, RuntimeValue};

impl VM {
    /// Inner instruction fetch-decode-execute loop.
    pub(crate) fn execute_loop(&mut self) -> Result<RuntimeValue, VMError> {
        while self.running {
            let (inst, ip) = {
                let frame = self.frames.last_mut().ok_or(VMError::StackUnderflow)?;
                let func = &self.module.functions[frame.function_idx as usize];

                if frame.ip >= func.chunk.instructions.len() {
                    // Implicit return from function
                    if self.frames.pop().is_none() {
                        self.running = false;
                        return Ok(RuntimeValue::Null);
                    }
                    continue;
                }

                let inst = &func.chunk.instructions[frame.ip];
                let current_ip = frame.ip;
                frame.ip += 1;
                (inst.clone(), current_ip)
            };

            // Diagnostics and tracing
            self.profiler.record_instruction();
            self.profiler.record_stack_height(self.stack.len());

            let current_func =
                &self.module.functions[self.frames.last().unwrap().function_idx as usize];
            self.debugger
                .trace_instruction(current_func, ip, &inst, &self.stack.get_dump());

            match inst.op {
                Opcode::NoOp => {}

                Opcode::LoadConst => {
                    if let Some(Operand::ConstantIndex(c_idx)) = inst.operands.first() {
                        let lit = current_func
                            .chunk
                            .constants
                            .get(*c_idx)
                            .ok_or(VMError::InvalidConstant(*c_idx))?;
                        let val = match lit {
                            LiteralVal::Int(n) => RuntimeValue::Int(*n),
                            LiteralVal::Float(f) => RuntimeValue::Float(*f),
                            LiteralVal::Str(s) => RuntimeValue::Str(s.clone()),
                            LiteralVal::Bool(b) => RuntimeValue::Bool(*b),
                            LiteralVal::None => RuntimeValue::Null,
                        };
                        self.stack.push(val)?;
                    } else {
                        return Err(VMError::InvalidOpcode);
                    }
                }

                Opcode::LoadLocal => {
                    if let Some(Operand::LocalIndex(l_idx)) = inst.operands.first() {
                        let bp = self.frames.last().unwrap().base_pointer;
                        let val = self.stack.get(bp + *l_idx as usize)?;
                        self.stack.push(val.clone())?;
                    } else {
                        return Err(VMError::InvalidOpcode);
                    }
                }

                Opcode::StoreLocal => {
                    if let Some(Operand::LocalIndex(l_idx)) = inst.operands.first() {
                        let bp = self.frames.last().unwrap().base_pointer;
                        let val = self.stack.pop()?;
                        self.stack.set(bp + *l_idx as usize, val)?;
                    } else {
                        return Err(VMError::InvalidOpcode);
                    }
                }

                Opcode::LoadGlobal => {
                    if let Some(Operand::GlobalIndex(g_idx)) = inst.operands.first() {
                        let name = match *g_idx {
                            999 => "say".to_string(),
                            998 => "range".to_string(),
                            997 => "ask".to_string(),
                            996 => "fstring_concat".to_string(),
                            idx => {
                                if idx as usize >= self.module.globals.len() {
                                    return Err(VMError::RuntimeException(format!(
                                        "Global index {} out of bounds",
                                        idx
                                    )));
                                }
                                self.module.globals[idx as usize].0.clone()
                            }
                        };
                        let val = if self.module.functions.iter().any(|f| f.name == name)
                            || self.native_bridge.has_function(&name)
                        {
                            RuntimeValue::Str(name)
                        } else {
                            self.globals
                                .get(&name)
                                .cloned()
                                .unwrap_or(RuntimeValue::Null)
                        };
                        self.stack.push(val)?;
                    } else {
                        return Err(VMError::InvalidOpcode);
                    }
                }

                Opcode::StoreGlobal => {
                    if let Some(Operand::GlobalIndex(g_idx)) = inst.operands.first() {
                        let name = match *g_idx {
                            999 => "say".to_string(),
                            998 => "range".to_string(),
                            997 => "ask".to_string(),
                            996 => "fstring_concat".to_string(),
                            idx => {
                                if idx as usize >= self.module.globals.len() {
                                    return Err(VMError::RuntimeException(format!(
                                        "Global index {} out of bounds",
                                        idx
                                    )));
                                }
                                self.module.globals[idx as usize].0.clone()
                            }
                        };
                        let val = self.stack.pop()?;
                        self.globals.insert(name, val);
                    } else {
                        return Err(VMError::InvalidOpcode);
                    }
                }

                Opcode::Pop => {
                    let _ = self.stack.pop()?;
                }

                Opcode::Dup => {
                    let val = self.stack.peek()?.clone();
                    self.stack.push(val)?;
                }

                Opcode::Add => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;
                    let res = match (left, right) {
                        (RuntimeValue::Int(a), RuntimeValue::Int(b)) => RuntimeValue::Int(a + b),
                        (RuntimeValue::Float(a), RuntimeValue::Float(b)) => {
                            RuntimeValue::Float(a + b)
                        }
                        (RuntimeValue::Str(a), RuntimeValue::Str(b)) => {
                            RuntimeValue::Str(format!("{}{}", a, b))
                        }
                        _ => {
                            return Err(VMError::TypeError {
                                expected: "Numeric/String".to_string(),
                                found: "mismatched".to_string(),
                            })
                        }
                    };
                    self.stack.push(res)?;
                }

                Opcode::Sub => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;
                    let res = match (left, right) {
                        (RuntimeValue::Int(a), RuntimeValue::Int(b)) => RuntimeValue::Int(a - b),
                        (RuntimeValue::Float(a), RuntimeValue::Float(b)) => {
                            RuntimeValue::Float(a - b)
                        }
                        _ => {
                            return Err(VMError::TypeError {
                                expected: "Numeric".to_string(),
                                found: "non-numeric".to_string(),
                            })
                        }
                    };
                    self.stack.push(res)?;
                }

                Opcode::Mul => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;
                    let res = match (left, right) {
                        (RuntimeValue::Int(a), RuntimeValue::Int(b)) => RuntimeValue::Int(a * b),
                        (RuntimeValue::Float(a), RuntimeValue::Float(b)) => {
                            RuntimeValue::Float(a * b)
                        }
                        _ => {
                            return Err(VMError::TypeError {
                                expected: "Numeric".to_string(),
                                found: "non-numeric".to_string(),
                            })
                        }
                    };
                    self.stack.push(res)?;
                }

                Opcode::Div => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;
                    let res = match (left, right) {
                        (RuntimeValue::Int(_), RuntimeValue::Int(0)) => {
                            return Err(VMError::DivisionByZero)
                        }
                        (RuntimeValue::Float(_), RuntimeValue::Float(0.0)) => {
                            return Err(VMError::DivisionByZero)
                        }
                        (RuntimeValue::Int(a), RuntimeValue::Int(b)) => RuntimeValue::Int(a / b),
                        (RuntimeValue::Float(a), RuntimeValue::Float(b)) => {
                            RuntimeValue::Float(a / b)
                        }
                        _ => {
                            return Err(VMError::TypeError {
                                expected: "Numeric".to_string(),
                                found: "non-numeric".to_string(),
                            })
                        }
                    };
                    self.stack.push(res)?;
                }

                Opcode::Mod => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;
                    let res = match (left, right) {
                        (RuntimeValue::Int(_), RuntimeValue::Int(0)) => {
                            return Err(VMError::DivisionByZero)
                        }
                        (RuntimeValue::Int(a), RuntimeValue::Int(b)) => RuntimeValue::Int(a % b),
                        _ => {
                            return Err(VMError::TypeError {
                                expected: "Int".to_string(),
                                found: "non-integer".to_string(),
                            })
                        }
                    };
                    self.stack.push(res)?;
                }

                Opcode::Neg => {
                    let val = self.stack.pop()?;
                    let res = match val {
                        RuntimeValue::Int(n) => RuntimeValue::Int(-n),
                        RuntimeValue::Float(f) => RuntimeValue::Float(-f),
                        _ => {
                            return Err(VMError::TypeError {
                                expected: "Numeric".to_string(),
                                found: "non-numeric".to_string(),
                            })
                        }
                    };
                    self.stack.push(res)?;
                }

                Opcode::Not => {
                    let val = self.stack.pop()?;
                    self.stack.push(RuntimeValue::Bool(!val.is_truthy()))?;
                }

                Opcode::Await => {
                    let val = self.stack.pop()?;
                    if let RuntimeValue::Map { entries, .. } = &val {
                        let mut is_future = false;
                        {
                            let entries_borrow = entries.borrow();
                            if entries_borrow.contains_key("state") && entries_borrow.contains_key("value") {
                                is_future = true;
                            }
                        }
                        if is_future {
                            loop {
                                let state = {
                                    let entries_borrow = entries.borrow();
                                    entries_borrow.get("state").cloned().unwrap_or(RuntimeValue::Null)
                                };
                                if let RuntimeValue::Str(s) = &state {
                                    if s == "pending" {
                                        techscript_stdlib::async_runtime::tick();
                                        std::thread::sleep(std::time::Duration::from_millis(1));
                                        continue;
                                    } else if s == "resolved" {
                                        let resolved_val = entries.borrow().get("value").cloned().unwrap_or(RuntimeValue::Null);
                                        self.stack.push(resolved_val)?;
                                        break;
                                    } else if s == "rejected" {
                                        let err_val = entries.borrow().get("value").cloned().unwrap_or(RuntimeValue::Null);
                                        return Err(VMError::RuntimeException(format!("Awaited future was rejected: {:?}", err_val)));
                                    }
                                }
                                self.stack.push(val.clone())?;
                                break;
                            }
                        } else {
                            self.stack.push(val)?;
                        }
                    } else {
                        self.stack.push(val)?;
                    }
                }

                Opcode::Equal => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;
                    let res = match (left, right) {
                        (RuntimeValue::Int(a), RuntimeValue::Int(b)) => a == b,
                        (RuntimeValue::Float(a), RuntimeValue::Float(b)) => a == b,
                        (RuntimeValue::Str(a), RuntimeValue::Str(b)) => a == b,
                        (RuntimeValue::Bool(a), RuntimeValue::Bool(b)) => a == b,
                        (RuntimeValue::Null, RuntimeValue::Null) => true,
                        _ => false,
                    };
                    self.stack.push(RuntimeValue::Bool(res))?;
                }

                Opcode::StrictEqual => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;
                    let res = match (left, right) {
                        (RuntimeValue::Int(a), RuntimeValue::Int(b)) => a == b,
                        (RuntimeValue::Float(a), RuntimeValue::Float(b)) => a == b,
                        (RuntimeValue::Str(a), RuntimeValue::Str(b)) => a == b,
                        (RuntimeValue::Bool(a), RuntimeValue::Bool(b)) => a == b,
                        (RuntimeValue::Null, RuntimeValue::Null) => true,
                        _ => false,
                    };
                    self.stack.push(RuntimeValue::Bool(res))?;
                }

                Opcode::NotEqual => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;
                    let res = match (left, right) {
                        (RuntimeValue::Int(a), RuntimeValue::Int(b)) => a != b,
                        (RuntimeValue::Float(a), RuntimeValue::Float(b)) => a != b,
                        (RuntimeValue::Str(a), RuntimeValue::Str(b)) => a != b,
                        (RuntimeValue::Bool(a), RuntimeValue::Bool(b)) => a != b,
                        (RuntimeValue::Null, RuntimeValue::Null) => false,
                        _ => true,
                    };
                    self.stack.push(RuntimeValue::Bool(res))?;
                }

                Opcode::Less => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;
                    let res = match (left, right) {
                        (RuntimeValue::Int(a), RuntimeValue::Int(b)) => a < b,
                        (RuntimeValue::Float(a), RuntimeValue::Float(b)) => a < b,
                        _ => {
                            return Err(VMError::TypeError {
                                expected: "Numeric".to_string(),
                                found: "non-numeric".to_string(),
                            })
                        }
                    };
                    self.stack.push(RuntimeValue::Bool(res))?;
                }

                Opcode::LessEqual => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;
                    let res = match (left, right) {
                        (RuntimeValue::Int(a), RuntimeValue::Int(b)) => a <= b,
                        (RuntimeValue::Float(a), RuntimeValue::Float(b)) => a <= b,
                        _ => {
                            return Err(VMError::TypeError {
                                expected: "Numeric".to_string(),
                                found: "non-numeric".to_string(),
                            })
                        }
                    };
                    self.stack.push(RuntimeValue::Bool(res))?;
                }

                Opcode::Greater => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;
                    let res = match (left, right) {
                        (RuntimeValue::Int(a), RuntimeValue::Int(b)) => a > b,
                        (RuntimeValue::Float(a), RuntimeValue::Float(b)) => a > b,
                        _ => {
                            return Err(VMError::TypeError {
                                expected: "Numeric".to_string(),
                                found: "non-numeric".to_string(),
                            })
                        }
                    };
                    self.stack.push(RuntimeValue::Bool(res))?;
                }

                Opcode::GreaterEqual => {
                    let right = self.stack.pop()?;
                    let left = self.stack.pop()?;
                    let res = match (left, right) {
                        (RuntimeValue::Int(a), RuntimeValue::Int(b)) => a >= b,
                        (RuntimeValue::Float(a), RuntimeValue::Float(b)) => a >= b,
                        _ => {
                            return Err(VMError::TypeError {
                                expected: "Numeric".to_string(),
                                found: "non-numeric".to_string(),
                            })
                        }
                    };
                    self.stack.push(RuntimeValue::Bool(res))?;
                }

                Opcode::Jump => {
                    if let Some(Operand::JumpOffset(offset)) = inst.operands.first() {
                        let frame = self.frames.last_mut().ok_or(VMError::StackUnderflow)?;
                        frame.ip = ((frame.ip as i32 - 1) + offset) as usize;
                    } else {
                        return Err(VMError::InvalidOpcode);
                    }
                }

                Opcode::JumpIfTrue => {
                    if let Some(Operand::JumpOffset(offset)) = inst.operands.first() {
                        let cond = self.stack.pop()?;
                        if cond.is_truthy() {
                            let frame = self.frames.last_mut().ok_or(VMError::StackUnderflow)?;
                            frame.ip = ((frame.ip as i32 - 1) + offset) as usize;
                        }
                    } else {
                        return Err(VMError::InvalidOpcode);
                    }
                }

                Opcode::JumpIfFalse => {
                    if let Some(Operand::JumpOffset(offset)) = inst.operands.first() {
                        let cond = self.stack.pop()?;
                        if !cond.is_truthy() {
                            let frame = self.frames.last_mut().ok_or(VMError::StackUnderflow)?;
                            frame.ip = ((frame.ip as i32 - 1) + offset) as usize;
                        }
                    } else {
                        return Err(VMError::InvalidOpcode);
                    }
                }

                Opcode::Call => {
                    if let Some(Operand::Count(arg_count)) = inst.operands.first() {
                        let mut args = Vec::new();
                        for _ in 0..*arg_count {
                            args.push(self.stack.pop()?);
                        }
                        args.reverse();

                        let callee = self.stack.pop()?;
                        match callee {
                            RuntimeValue::Str(name) if self.native_bridge.has_function(&name) => {
                                let ret = self.native_bridge.call(&name, &args)?;
                                self.stack.push(ret)?;
                            }
                            RuntimeValue::Str(name) => {
                                // Try finding in module's compiled functions
                                let target_idx = self
                                    .module
                                    .functions
                                    .iter()
                                    .position(|f| f.name == name)
                                    .ok_or(VMError::InvalidFunction(0))?;

                                if self.frames.len() >= 512 {
                                    return Err(VMError::CallStackOverflow);
                                }

                                let bp = self.stack.len();
                                for arg in args {
                                    self.stack.push(arg)?;
                                }

                                let new_frame = CallFrame::new(target_idx as u32, bp);
                                self.frames.push(new_frame);
                                self.profiler.record_call();
                            }
                            RuntimeValue::Function(func) => {
                                if func.name() == "spawn_async" {
                                    if let Some(RuntimeValue::Str(func_name)) = args.first().cloned() {
                                        let target_idx = self
                                            .module
                                            .functions
                                            .iter()
                                            .position(|f| f.name == func_name)
                                            .ok_or(VMError::InvalidFunction(0))?;
                                        
                                        let mut sub_vm = VM::new(self.module.clone());
                                        sub_vm.ctx.config.capabilities = self.ctx.config.capabilities.clone();
                                        sub_vm.frames.push(CallFrame::new(target_idx as u32, 0));
                                        sub_vm.running = true;
                                        let val = sub_vm.execute_loop()?;
                                        
                                        let mut fut_map = indexmap::IndexMap::new();
                                        fut_map.insert("state".to_string(), RuntimeValue::Str("resolved".to_string()));
                                        fut_map.insert("value".to_string(), val);
                                        let future = RuntimeValue::Map {
                                            entries: Rc::new(RefCell::new(fut_map)),
                                            is_const: false,
                                        };
                                        self.stack.push(future)?;
                                        continue;
                                    }
                                }
                                let ret = match func.call(&mut self.ctx, args) {
                                    Ok(val) => val,
                                    Err(err) => {
                                        let msg = err.to_string();
                                        let mut handler_found = false;
                                        while let Some(frame) = self.frames.last_mut() {
                                            if let Some(handler) = frame.handlers.pop() {
                                                self.stack.truncate(handler.stack_depth);
                                                self.stack.push(RuntimeValue::Str(msg.clone()))?;
                                                frame.ip = handler.catch_ip;
                                                handler_found = true;
                                                break;
                                            }
                                            self.frames.pop();
                                        }
                                        if handler_found {
                                            continue;
                                        } else {
                                            self.running = false;
                                            return Err(VMError::RuntimeException(msg));
                                        }
                                    }
                                };
                                self.stack.push(ret)?;
                            }
                            _ => {
                                return Err(VMError::TypeError {
                                    expected: "Callable".to_string(),
                                    found: "non-callable".to_string(),
                                })
                            }
                        }
                    } else {
                        return Err(VMError::InvalidOpcode);
                    }
                }

                Opcode::Return => {
                    let ret = self.stack.pop().unwrap_or(RuntimeValue::Null);
                    if let Some(frame) = self.frames.last() {
                        if self.frames.len() == 1 {
                            let func = &self.module.functions[frame.function_idx as usize];
                            for (slot, name) in &func.debug_symbols.local_names {
                                if let Ok(val) = self.stack.get(frame.base_pointer + *slot as usize)
                                {
                                    self.globals.insert(name.clone(), val.clone());
                                }
                            }
                        }
                    }
                    if let Some(frame) = self.frames.pop() {
                        self.stack.truncate(frame.base_pointer);
                        if self.frames.is_empty() {
                            self.running = false;
                            return Ok(ret);
                        }
                        self.stack.push(ret)?;
                    } else {
                        return Err(VMError::StackUnderflow);
                    }
                }

                Opcode::Try => {
                    if let Some(Operand::JumpOffset(offset)) = inst.operands.first() {
                        let frame = self.frames.last_mut().ok_or(VMError::StackUnderflow)?;
                        let catch_ip = ((frame.ip as i32 - 1) + offset) as usize;
                        frame.handlers.push(ExceptionHandler {
                            catch_ip,
                            stack_depth: self.stack.len(),
                        });
                    } else {
                        return Err(VMError::InvalidOpcode);
                    }
                }

                Opcode::EndTry => {
                    let frame = self.frames.last_mut().ok_or(VMError::StackUnderflow)?;
                    frame.handlers.pop();
                }

                Opcode::Throw => {
                    let msg_val = self
                        .stack
                        .pop()
                        .unwrap_or(RuntimeValue::Str("RuntimeException".to_string()));
                    let msg = match msg_val {
                        RuntimeValue::Str(s) => s,
                        other => other.to_string(),
                    };

                    // Search for a try-catch handler in the frame stack
                    let mut handler_found = false;
                    while let Some(frame) = self.frames.last_mut() {
                        if let Some(handler) = frame.handlers.pop() {
                            self.stack.truncate(handler.stack_depth);
                            self.stack.push(RuntimeValue::Str(msg.clone()))?;
                            frame.ip = handler.catch_ip;
                            handler_found = true;
                            break;
                        }
                        self.frames.pop();
                    }

                    if !handler_found {
                        self.running = false;
                        return Err(VMError::RuntimeException(msg));
                    }
                }

                Opcode::MakeList => {
                    if let Some(Operand::Count(n)) = inst.operands.first() {
                        let mut items = Vec::new();
                        for _ in 0..*n {
                            items.push(self.stack.pop()?);
                        }
                        items.reverse();
                        self.stack.push(RuntimeValue::List {
                            items: Rc::new(RefCell::new(items)),
                            is_const: false,
                        })?;
                    } else {
                        return Err(VMError::InvalidOpcode);
                    }
                }

                Opcode::MakeMap => {
                    if let Some(Operand::Count(n)) = inst.operands.first() {
                        let mut entries = indexmap::IndexMap::new();
                        for _ in 0..*n {
                            let val = self.stack.pop()?;
                            let key_val = self.stack.pop()?;
                            let key = match key_val {
                                RuntimeValue::Str(s) => s,
                                other => other.to_string(),
                            };
                            entries.insert(key, val);
                        }
                        self.stack.push(RuntimeValue::Map {
                            entries: Rc::new(RefCell::new(entries)),
                            is_const: false,
                        })?;
                    } else {
                        return Err(VMError::InvalidOpcode);
                    }
                }

                Opcode::IndexLoad => {
                    let index = self.stack.pop()?;
                    let base = self.stack.pop()?;
                    match index {
                        RuntimeValue::Int(idx) => {
                            let res = collections::list_get(&base, idx)
                                .map_err(|e| VMError::RuntimeException(e.to_string()))?;
                            self.stack.push(res)?;
                        }
                        other => {
                            let key = other.to_string();
                            let res = collections::map_get(&base, &key)
                                .map_err(|e| VMError::RuntimeException(e.to_string()))?;
                            self.stack.push(res)?;
                        }
                    }
                }

                Opcode::IndexStore => {
                    let value = self.stack.pop()?;
                    let index = self.stack.pop()?;
                    let base = self.stack.pop()?;
                    match index {
                        RuntimeValue::Int(idx) => {
                            collections::list_set(&base, idx, value.clone())
                                .map_err(|e| VMError::RuntimeException(e.to_string()))?;
                        }
                        other => {
                            let key = other.to_string();
                            collections::map_set(&base, key, value.clone())
                                .map_err(|e| VMError::RuntimeException(e.to_string()))?;
                        }
                    }
                    self.stack.push(value)?;
                }

                Opcode::FieldLoad => {
                    if let Some(Operand::ConstantIndex(c_idx)) = inst.operands.first() {
                        let lit = current_func
                            .chunk
                            .constants
                            .get(*c_idx)
                            .ok_or(VMError::InvalidConstant(*c_idx))?;
                        let name = match lit {
                            LiteralVal::Str(s) => s,
                            _ => {
                                return Err(VMError::TypeError {
                                    expected: "String".to_string(),
                                    found: "non-string".to_string(),
                                })
                            }
                        };

                        let base = self.stack.pop()?;
                        let res = collections::map_get(&base, name)
                            .map_err(|e| VMError::RuntimeException(e.to_string()))?;
                        self.stack.push(res)?;
                    } else {
                        return Err(VMError::InvalidOpcode);
                    }
                }

                Opcode::FieldStore => {
                    if let Some(Operand::ConstantIndex(c_idx)) = inst.operands.first() {
                        let lit = current_func
                            .chunk
                            .constants
                            .get(*c_idx)
                            .ok_or(VMError::InvalidConstant(*c_idx))?;
                        let name = match lit {
                            LiteralVal::Str(s) => s,
                            _ => {
                                return Err(VMError::TypeError {
                                    expected: "String".to_string(),
                                    found: "non-string".to_string(),
                                })
                            }
                        };

                        let value = self.stack.pop()?;
                        let base = self.stack.pop()?;
                        collections::map_set(&base, name.clone(), value.clone())
                            .map_err(|e| VMError::RuntimeException(e.to_string()))?;
                        self.stack.push(value)?;
                    } else {
                        return Err(VMError::InvalidOpcode);
                    }
                }

                _ => return Err(VMError::InvalidOpcode),
            }
        }

        Ok(RuntimeValue::Null)
    }
}
