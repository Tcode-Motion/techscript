// ── TechScript Virtual Machine ───────────────────────────────────────
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Write};
use std::rc::Rc;

use crate::error::{TechError, TechResult};
use crate::opcode::OpCode;
use crate::value::*;

struct CallFrame {
    function: Rc<Function>,
    ip: usize,
    slot_offset: usize,
    upvalues: Vec<Rc<RefCell<Value>>>,
}

struct TryHandler {
    catch_ip: usize,
    frame_depth: usize,
    stack_depth: usize,
}

pub struct VM {
    stack: Vec<Value>,
    pub globals: HashMap<String, Value>,
    frames: Vec<CallFrame>,
    #[allow(dead_code)]
    open_upvalues: Vec<(usize, Rc<RefCell<Value>>)>,
    try_handlers: Vec<TryHandler>,
    pub stdout_buffer: Option<std::sync::Arc<std::sync::Mutex<String>>>,
}

impl VM {
    pub fn new() -> Self {
        let mut vm = VM {
            stack: Vec::with_capacity(256),
            globals: HashMap::new(),
            frames: Vec::new(),
            open_upvalues: Vec::new(),
            try_handlers: Vec::new(),
            stdout_buffer: None,
        };
        crate::builtins::register_builtins(&mut vm.globals);
        vm
    }

    pub fn run(&mut self, function: Function) -> TechResult<()> {
        self.stack.clear();
        self.frames.clear();
        self.open_upvalues.clear();
        self.try_handlers.clear();

        let func = Rc::new(function);
        self.frames.push(CallFrame {
            function: func, ip: 0, slot_offset: 0,
            upvalues: Vec::new(),
        });
        self.stack.push(Value::None);
        self.execute()
    }

    fn read_byte(&mut self) -> u8 {
        let frame = self.frames.last_mut().unwrap();
        let b = frame.function.chunk.code[frame.ip];
        frame.ip += 1;
        b
    }

    fn read_u16(&mut self) -> u16 {
        let hi = self.read_byte() as u16;
        let lo = self.read_byte() as u16;
        (hi << 8) | lo
    }

    fn read_constant(&mut self) -> Value {
        let idx = self.read_u16() as usize;
        self.frames.last().unwrap().function.chunk.constants[idx].clone()
    }

    fn current_line(&self) -> usize {
        let f = self.frames.last().unwrap();
        if f.ip > 0 && f.ip - 1 < f.function.chunk.lines.len() {
            f.function.chunk.lines[f.ip - 1]
        } else { 0 }
    }

    fn runtime_error(&self, msg: impl Into<String>) -> TechError {
        TechError::runtime_at(msg, self.current_line())
    }

    fn push(&mut self, val: Value) { self.stack.push(val); }
    pub fn pop(&mut self) -> Value { self.stack.pop().unwrap_or(Value::None) }
    pub fn clear_stack(&mut self) { self.stack.clear(); }
    fn peek(&self, dist: usize) -> &Value { &self.stack[self.stack.len() - 1 - dist] }

    fn execute(&mut self) -> TechResult<()> {
        loop {
            if self.frames.is_empty() { return Ok(()); }
            let ip = self.frames.last().unwrap().ip;
            let code_len = self.frames.last().unwrap().function.chunk.code.len();
            if ip >= code_len {
                self.frames.pop();
                continue;
            }

            if let Err(e) = self.step() {
                if let Some(handler) = self.try_handlers.pop() {
                    self.frames.truncate(handler.frame_depth);
                    self.stack.truncate(handler.stack_depth);
                    self.frames.last_mut().unwrap().ip = handler.catch_ip;
                    self.push(Value::String(Rc::new(e.message)));
                } else {
                    return Err(e);
                }
            }
        }
    }

    fn step(&mut self) -> TechResult<()> {
        let byte = self.read_byte();
        let op: OpCode = OpCode::try_from(byte).map_err(|_| {
            self.runtime_error(format!("Invalid bytecode opcode: 0x{:02X}", byte))
        })?;

            // DEBUG TRACE
            // print!(">>> {:?} | Stack: [", op);
            // for v in &self.stack { print!("{}, ", v.display_string()); }
            // println!("]");
            if std::env::var("TRACE").is_ok() {
                print!(">>> {:?} | Stack: [", op);
                for v in &self.stack { print!("{}, ", v.display_string()); }
                println!("]");
            }

            match op {
                OpCode::Constant => { let v = self.read_constant(); self.push(v); }
                OpCode::None => self.push(Value::None),
                OpCode::True => self.push(Value::Bool(true)),
                OpCode::False => self.push(Value::Bool(false)),
                OpCode::Pop => { self.pop(); }
                OpCode::Dup => { let v = self.peek(0).clone(); self.push(v); }

                OpCode::GetGlobal => {
                    let name_val = self.read_constant();
                    let name = match &name_val { Value::String(s) => s.as_ref().clone(), _ => String::new() };
                    match self.globals.get(&name) {
                        Some(v) => self.push(v.clone()),
                        None => {
                            eprintln!("DEBUG: Registered globals are: {:?}", self.globals.keys().collect::<Vec<&String>>());
                            return Err(self.runtime_error(format!("Undefined variable: '{}'", name)));
                        }
                    }
                }
                OpCode::SetGlobal => {
                    let name_val = self.read_constant();
                    let name = match &name_val { Value::String(s) => s.as_ref().clone(), _ => String::new() };
                    let val = self.peek(0).clone();
                    self.globals.insert(name, val);
                }
                OpCode::DefineGlobal => {
                    let name_val = self.read_constant();
                    let name = match &name_val { Value::String(s) => s.as_ref().clone(), _ => String::new() };
                    let val = self.pop();
                    self.globals.insert(name, val);
                }
                OpCode::GetLocal => {
                    let slot = self.read_byte() as usize;
                    let off = self.frames.last().unwrap().slot_offset;
                    let val = self.stack[off + slot].clone();
                    self.push(val);
                }
                OpCode::SetLocal => {
                    let slot = self.read_byte() as usize;
                    let off = self.frames.last().unwrap().slot_offset;
                    let val = self.peek(0).clone();
                    self.stack[off + slot] = val;
                }
                OpCode::GetUpvalue => {
                    let slot = self.read_byte() as usize;
                    let val = self.frames.last().unwrap().upvalues[slot].borrow().clone();
                    self.push(val);
                }
                OpCode::SetUpvalue => {
                    let slot = self.read_byte() as usize;
                    let val = self.peek(0).clone();
                    *self.frames.last().unwrap().upvalues[slot].borrow_mut() = val;
                }

                // Arithmetic
                OpCode::Add => { let b = self.pop(); let a = self.pop(); self.push(self.op_add(a, b)?); }
                OpCode::Subtract => { let b = self.pop(); let a = self.pop(); self.push(self.op_sub(a, b)?); }
                OpCode::Multiply => { let b = self.pop(); let a = self.pop(); self.push(self.op_mul(a, b)?); }
                OpCode::Divide => { 
                    let b = self.pop(); let a = self.pop(); 
                    if let Value::Int(0) | Value::Float(0.0) = b { return Err(self.runtime_error("Division by zero")); }
                    self.push(self.op_div(a, b)?); 
                }
                OpCode::IntDivide => { 
                    let b = self.pop(); let a = self.pop(); 
                    if let Value::Int(0) | Value::Float(0.0) = b { return Err(self.runtime_error("Division by zero")); }
                    self.push(self.op_intdiv(a, b)?); 
                }
                OpCode::Modulo => { 
                    let b = self.pop(); let a = self.pop(); 
                    if let Value::Int(0) | Value::Float(0.0) = b { return Err(self.runtime_error("Division by zero")); }
                    self.push(self.op_mod(a, b)?); 
                }
                OpCode::Power => { let b = self.pop(); let a = self.pop(); self.push(self.op_pow(a, b)?); }
                OpCode::Negate => {
                    let v = self.pop();
                    match v {
                        Value::Int(i) => self.push(Value::Int(-i)),
                        Value::Float(f) => self.push(Value::Float(-f)),
                        _ => return Err(self.runtime_error("Cannot negate non-number")),
                    }
                }

                // Comparison
                OpCode::Equal => { let b = self.pop(); let a = self.pop(); self.push(Value::Bool(a.equals(&b))); }
                OpCode::NotEqual => { let b = self.pop(); let a = self.pop(); self.push(Value::Bool(!a.equals(&b))); }
                OpCode::In => {
                    let container = self.pop();
                    let item = self.pop();
                    let result = self.op_in(&item, &container)?;
                    self.push(Value::Bool(result));
                }
                OpCode::TypeOf => {
                    let val = self.pop();
                    self.push(Value::String(Rc::new(val.type_name().to_string())));
                }
                OpCode::Less => { let b = self.pop(); let a = self.pop(); self.push(self.op_cmp(&a, &b, std::cmp::Ordering::Less)?); }
                OpCode::Greater => { let b = self.pop(); let a = self.pop(); self.push(self.op_cmp(&a, &b, std::cmp::Ordering::Greater)?); }
                OpCode::LessEqual => {
                    let b = self.pop(); let a = self.pop();
                    let r = a.compare(&b).map(|o| o != std::cmp::Ordering::Greater).unwrap_or(false);
                    self.push(Value::Bool(r));
                }
                OpCode::GreaterEqual => {
                    let b = self.pop(); let a = self.pop();
                    let r = a.compare(&b).map(|o| o != std::cmp::Ordering::Less).unwrap_or(false);
                    self.push(Value::Bool(r));
                }
                OpCode::Not => { let v = self.pop(); self.push(Value::Bool(!v.is_truthy())); }
                OpCode::And | OpCode::Or => { /* handled via jumps in compiler */ }

                // I/O
                OpCode::Print => {
                    let count = self.read_byte() as usize;
                    let start = self.stack.len() - count;
                    let vals: Vec<String> = self.stack[start..].iter().map(|v| v.display_string()).collect();
                    self.stack.truncate(start);
                    let output = vals.join(" ");
                    if let Some(buf) = &self.stdout_buffer {
                        if let Ok(mut lock) = buf.lock() {
                            lock.push_str(&output);
                            lock.push('\n');
                        }
                    } else {
                        println!("{}", output);
                    }
                }
                OpCode::ReadInput => {
                    let prompt = self.pop();
                    print!("{}", prompt.display_string());
                    io::stdout().flush().ok();
                    if std::env::var("TECHSCRIPT_NON_INTERACTIVE").is_ok() {
                        println!("\n[Non-interactive test: stopping execution]");
                        crate::run::exit(0);
                    }
                    let mut input = String::new();
                    match io::stdin().read_line(&mut input) {
                        Ok(0) => {
                            // EOF encountered
                            println!();
                            crate::run::exit(0);
                        }
                        _ => {}
                    }
                    self.push(Value::String(Rc::new(input.trim_end_matches('\n').trim_end_matches('\r').to_string())));
                }

                // Control flow
                OpCode::Jump => {
                    let offset = self.read_u16() as usize;
                    self.frames.last_mut().unwrap().ip += offset;
                }
                OpCode::JumpIfFalse => {
                    let offset = self.read_u16() as usize;
                    if !self.peek(0).is_truthy() {
                        self.frames.last_mut().unwrap().ip += offset;
                    }
                }
                OpCode::Loop => {
                    let offset = self.read_u16() as usize;
                    self.frames.last_mut().unwrap().ip -= offset;
                }

                // Functions
                OpCode::Call => {
                    let arg_count = self.read_byte() as usize;
                    self.call_value(arg_count)?;
                }
                OpCode::Return => {
                    let result = self.pop();
                    let frame = self.frames.pop().unwrap();
                    if self.frames.is_empty() {
                        self.stack.truncate(frame.slot_offset);
                        self.push(result);
                        return Ok(());
                    }
                    self.stack.truncate(frame.slot_offset);
                    self.push(result);
                }
                OpCode::Closure => {
                    let func_val = self.read_constant();
                    if let Value::Function(func) = func_val {
                        let mut upvalues = Vec::new();
                        for _ in 0..func.upvalue_count {
                            let is_local = self.read_byte() == 1;
                            let index = self.read_byte() as usize;
                            if is_local {
                                let off = self.frames.last().unwrap().slot_offset;
                                let val = Rc::new(RefCell::new(self.stack[off + index].clone()));
                                upvalues.push(val);
                            } else {
                                let uv = self.frames.last().unwrap().upvalues[index].clone();
                                upvalues.push(uv);
                            }
                        }
                        let closure = Rc::new(ClosureObj {
                            function: Rc::clone(&func),
                            upvalues,
                        });
                        if std::env::var("TRACE").is_ok() {
                            println!("DEBUG: created closure {} with {} upvalues", func.name, closure.upvalues.len());
                        }
                        self.push(Value::Closure(closure));
                    }
                }
                OpCode::CloseUpvalue => { self.pop(); }

                // Collections
                OpCode::BuildList => {
                    let count = self.read_byte() as usize;
                    let start = self.stack.len() - count;
                    let items: Vec<Value> = self.stack[start..].to_vec();
                    self.stack.truncate(start);
                    self.push(Value::List(Rc::new(RefCell::new(items))));
                }
                OpCode::BuildMap => {
                    let count = self.read_byte() as usize;
                    let start = self.stack.len() - count * 2;
                    let mut map = HashMap::new();
                    for i in 0..count {
                        let key = self.stack[start + i * 2].display_string();
                        let val = self.stack[start + i * 2 + 1].clone();
                        map.insert(key, val);
                    }
                    self.stack.truncate(start);
                    self.push(Value::Map(Rc::new(RefCell::new(map))));
                }
                OpCode::Index => {
                    let idx = self.pop();
                    let obj = self.pop();
                    self.push(self.op_index(&obj, &idx)?);
                }
                OpCode::SetIndex => {
                    let val = self.pop();
                    let idx = self.pop();
                    let obj = self.pop();
                    self.op_set_index(&obj, &idx, val)?;
                }

                // Classes
                OpCode::Class => {
                    let name_val = self.read_constant();
                    let name = match &name_val { Value::String(s) => s.as_ref().clone(), _ => String::new() };
                    let class = ClassObj { name, methods: HashMap::new(), parent: None };
                    self.push(Value::Class(Rc::new(RefCell::new(class))));
                }
                OpCode::GetProperty => {
                    let name_val = self.read_constant();
                    let name = match &name_val { Value::String(s) => s.as_ref().clone(), _ => String::new() };
                    let obj = self.pop();
                    self.push(self.get_property(&obj, &name)?);
                }
                OpCode::SetProperty => {
                    let name_val = self.read_constant();
                    let name = match &name_val { Value::String(s) => s.as_ref().clone(), _ => String::new() };
                    let val = self.pop();
                    let obj = self.pop();
                    if let Value::Instance(inst) = &obj {
                        inst.borrow_mut().fields.insert(name, val.clone());
                        self.push(val);
                    } else {
                        return Err(self.runtime_error("Only instances have settable fields"));
                    }
                }
                OpCode::Method => {
                    let name_val = self.read_constant();
                    let name = match &name_val { Value::String(s) => s.as_ref().clone(), _ => String::new() };
                    let method = self.pop();
                    let class = self.peek(0).clone();
                    if let Value::Class(c) = &class {
                        c.borrow_mut().methods.insert(name, method);
                    }
                }
                OpCode::Invoke => {
                    let name_val = self.read_constant();
                    let name = match &name_val {
                        Value::String(s) => s.as_ref().clone(),
                        _ => String::new(),
                    };
                    let arg_count = self.read_byte() as usize;
                    let receiver_idx = self.stack.len() - 1 - arg_count;
                    let receiver = self.stack[receiver_idx].clone();
                    let method = self.get_property(&receiver, &name)?;
                    self.stack[receiver_idx] = method;
                    self.call_value(arg_count)?;
                }
                OpCode::Inherit => {
                    let subclass = self.pop();
                    let superclass = self.pop();
                    if let (Value::Class(sub), Value::Class(sup)) = (&subclass, &superclass) {
                        let methods = sup.borrow().methods.clone();
                        for (name, method) in methods {
                            sub.borrow_mut().methods.insert(name, method);
                        }
                        sub.borrow_mut().parent = Some(sup.clone());
                    }
                }

                // Modules
                OpCode::Import => {
                    let name_val = self.read_constant();
                    let name = match &name_val {
                        Value::String(s) => s.as_ref().clone(),
                        _ => String::new(),
                    };
                    crate::modules::load_module(&name, &mut self.globals)
                        .map_err(|msg| self.runtime_error(msg))?;
                }

                // Iteration
                OpCode::GetIter => {
                    let val = self.pop();
                    let items = match &val {
                        Value::List(l) => l.borrow().clone(),
                        Value::Range(start, end, inclusive) => {
                            let e = if *inclusive { *end + 1 } else { *end };
                            (*start..e).map(Value::Int).collect()
                        }
                        _ => return Err(self.runtime_error(format!("Cannot iterate over {}", val.type_name()))),
                    };
                    self.push(Value::Iterator(Rc::new(RefCell::new(IterState::new(items)))));
                }
                OpCode::IterNext => {
                    let offset = self.read_u16() as usize;
                    let iter = self.peek(0).clone();
                    if let Value::Iterator(state) = &iter {
                        match state.borrow_mut().next() {
                            Some(val) => self.push(val),
                            None => { 
                                self.push(Value::None); 
                                self.frames.last_mut().unwrap().ip += offset; 
                            }
                        }
                    }
                }

                // Range
                OpCode::BuildRange => {
                    let end = self.pop(); let start = self.pop();
                    match (&start, &end) {
                        (Value::Int(s), Value::Int(e)) => self.push(Value::Range(*s, *e, false)),
                        _ => return Err(self.runtime_error("Range bounds must be integers")),
                    }
                }
                OpCode::BuildRangeInclusive => {
                    let end = self.pop(); let start = self.pop();
                    match (&start, &end) {
                        (Value::Int(s), Value::Int(e)) => self.push(Value::Range(*s, *e, true)),
                        _ => return Err(self.runtime_error("Range bounds must be integers")),
                    }
                }

                // F-string
                OpCode::FormatString => {
                    let count = self.read_byte() as usize;
                    let start = self.stack.len() - count;
                    let parts: Vec<String> = self.stack[start..].iter().map(|v| v.display_string()).collect();
                    self.stack.truncate(start);
                    self.push(Value::String(Rc::new(parts.concat())));
                }
                OpCode::SetupTry => {
                    let offset = self.read_u16() as usize;
                    let catch_ip = self.frames.last().unwrap().ip + offset;
                    self.try_handlers.push(TryHandler {
                        catch_ip,
                        frame_depth: self.frames.len(),
                        stack_depth: self.stack.len(),
                    });
                }
                OpCode::PopTry => {
                    self.try_handlers.pop();
                }
                OpCode::Throw => {
                    let val = self.pop();
                    return Err(self.runtime_error(val.display_string()));
                }
            }
        Ok(())
    }

    // ─── Helpers ─────────────────────────────────────────────────

    fn call_value(&mut self, arg_count: usize) -> TechResult<()> {
        let callee_idx = self.stack.len() - 1 - arg_count;
        let callee = self.stack[callee_idx].clone();
        match callee {
            Value::Closure(closure) => {
                if std::env::var("TRACE").is_ok() {
                    println!("DEBUG: calling closure {} with {} upvalues", closure.function.name, closure.upvalues.len());
                }
                self.frames.push(CallFrame {
                    function: Rc::clone(&closure.function),
                    ip: 0,
                    slot_offset: callee_idx,
                    upvalues: closure.upvalues.clone(),
                });
                Ok(())
            }
            Value::NativeFunction(native) => {
                let start = self.stack.len() - arg_count;
                let args: Vec<Value> = self.stack[start..].to_vec();
                self.stack.truncate(callee_idx);
                match (native.func)(&args) {
                    Ok(result) => { self.push(result); Ok(()) }
                    Err(msg) => Err(self.runtime_error(msg)),
                }
            }
            Value::Class(class) => {
                let inst = InstanceObj { class: class.clone(), fields: HashMap::new() };
                let instance = Value::Instance(Rc::new(RefCell::new(inst)));
                self.stack[callee_idx] = instance.clone();
                // Call init if it exists
                let init = class.borrow().methods.get("init").cloned();
                if let Some(Value::Closure(init_c)) = init {
                    self.frames.push(CallFrame {
                        function: Rc::clone(&init_c.function),
                        ip: 0,
                        slot_offset: callee_idx,
                        upvalues: init_c.upvalues.clone(),
                    });
                } else {
                    // No init — just leave instance, pop args
                    let start = self.stack.len() - arg_count;
                    self.stack.truncate(start);
                }
                Ok(())
            }
            Value::BoundMethod(inst, c) => {
                self.stack[callee_idx] = Value::Instance(inst);
                self.frames.push(CallFrame {
                    function: Rc::clone(&c.function),
                    ip: 0,
                    slot_offset: callee_idx,
                    upvalues: c.upvalues.clone(),
                });
                Ok(())
            }
            Value::ListMethod(list, method) => {
                let start = self.stack.len() - arg_count;
                let args: Vec<Value> = self.stack[start..].to_vec();
                self.stack.truncate(callee_idx);
                let result = self.invoke_list_method(&list, &method, &args)?;
                self.push(result);
                Ok(())
            }
            _ => Err(self.runtime_error(format!("Cannot call {}", callee.type_name()))),
        }
    }

    fn get_property(&self, obj: &Value, name: &str) -> TechResult<Value> {
        match obj {
            Value::Instance(inst) => {
                let inst_ref = inst.borrow();
                if let Some(val) = inst_ref.fields.get(name) { return Ok(val.clone()); }
                let class = inst_ref.class.clone();
                if let Some(method) = class.borrow().methods.get(name) {
                    if let Value::Closure(c) = method {
                        return Ok(Value::BoundMethod(inst.clone(), c.clone()));
                    }
                }
                Err(self.runtime_error(format!("Undefined property '{}'", name)))
            }
            Value::String(s) => self.string_property(s, name),
            Value::List(l) => self.list_property(l, name),
            Value::Map(m) => self.map_property(m, name),
            _ => Err(self.runtime_error(format!("Cannot access property '{}' on {}", name, obj.type_name()))),
        }
    }

    fn string_property(&self, s: &str, name: &str) -> TechResult<Value> {
        match name {
            "length" => Ok(Value::Int(s.len() as i64)),
            "upper" | "lower" | "trim" => {
                let owned = s.to_string();
                let method = name.to_string();
                Ok(Value::NativeFunction(Rc::new(NativeFnObj {
                    name: method.clone(),
                    func: Box::new(move |_args| {
                        let out = match method.as_str() {
                            "upper" => owned.to_uppercase(),
                            "lower" => owned.to_lowercase(),
                            "trim" => owned.trim().to_string(),
                            _ => owned.clone(),
                        };
                        Ok(Value::String(Rc::new(out)))
                    }),
                })))
            }
            _ => Err(self.runtime_error(format!("Unknown string method: {}", name))),
        }
    }

    fn list_property(&self, list: &Rc<RefCell<Vec<Value>>>, name: &str) -> TechResult<Value> {
        match name {
            "length" => Ok(Value::Int(list.borrow().len() as i64)),
            "first" => Ok(list.borrow().first().cloned().unwrap_or(Value::None)),
            "last" => Ok(list.borrow().last().cloned().unwrap_or(Value::None)),
            "append" | "sort" | "reverse" | "map" | "filter" | "reduce" | "remove" => {
                Ok(Value::ListMethod(list.clone(), name.to_string()))
            }
            _ => Err(self.runtime_error(format!("Unknown list method: {}", name))),
        }
    }

    fn invoke_list_method(
        &mut self,
        list: &Rc<RefCell<Vec<Value>>>,
        name: &str,
        args: &[Value],
    ) -> TechResult<Value> {
        match name {
            "append" => {
                if let Some(v) = args.first() {
                    list.borrow_mut().push(v.clone());
                }
                Ok(Value::None)
            }
            "sort" => {
                list.borrow_mut().sort_by(|a, b| {
                    match (a.as_f64(), b.as_f64()) {
                        (Some(x), Some(y)) => x.partial_cmp(&y).unwrap_or(std::cmp::Ordering::Equal),
                        _ => a.display_string().cmp(&b.display_string()),
                    }
                });
                Ok(Value::List(list.clone()))
            }
            "reverse" => {
                list.borrow_mut().reverse();
                Ok(Value::List(list.clone()))
            }
            "remove" => {
                if let Some(v) = args.first() {
                    list.borrow_mut().retain(|x| !x.equals(v));
                }
                Ok(Value::List(list.clone()))
            }
            "map" => {
                let cb = args.first().ok_or_else(|| self.runtime_error("map() needs a function"))?;
                let closure = match cb {
                    Value::Closure(c) => c.clone(),
                    _ => return Err(self.runtime_error("map() callback must be a function")),
                };
                let mut out = Vec::new();
                for item in list.borrow().iter().cloned() {
                    out.push(self.call_closure_sync(closure.clone(), vec![item])?);
                }
                Ok(Value::List(Rc::new(RefCell::new(out))))
            }
            "filter" => {
                let cb = args.first().ok_or_else(|| self.runtime_error("filter() needs a function"))?;
                let closure = match cb {
                    Value::Closure(c) => c.clone(),
                    _ => return Err(self.runtime_error("filter() callback must be a function")),
                };
                let mut out = Vec::new();
                for item in list.borrow().iter().cloned() {
                    let keep = self.call_closure_sync(closure.clone(), vec![item.clone()])?;
                    if keep.is_truthy() {
                        out.push(item);
                    }
                }
                Ok(Value::List(Rc::new(RefCell::new(out))))
            }
            "reduce" => {
                let cb = args.first().ok_or_else(|| self.runtime_error("reduce() needs a function"))?;
                let closure = match cb {
                    Value::Closure(c) => c.clone(),
                    _ => return Err(self.runtime_error("reduce() callback must be a function")),
                };
                let mut acc = args.get(1).cloned().unwrap_or(Value::None);
                for item in list.borrow().iter().cloned() {
                    acc = self.call_closure_sync(closure.clone(), vec![acc, item])?;
                }
                Ok(acc)
            }
            _ => Err(self.runtime_error(format!("Unknown list method: {}", name))),
        }
    }

    fn call_closure_sync(&mut self, closure: Rc<ClosureObj>, args: Vec<Value>) -> TechResult<Value> {
        let frame_depth = self.frames.len();
        let argc = args.len();
        self.push(Value::Closure(closure));
        for arg in args {
            self.push(arg);
        }
        self.call_value(argc)?;
        loop {
            if self.frames.len() <= frame_depth {
                return Ok(self.pop());
            }
            if let Err(e) = self.step() {
                if let Some(handler) = self.try_handlers.pop() {
                    self.frames.truncate(handler.frame_depth);
                    self.stack.truncate(handler.stack_depth);
                    self.frames.last_mut().unwrap().ip = handler.catch_ip;
                    self.push(Value::String(Rc::new(e.message)));
                } else {
                    return Err(e);
                }
            }
        }
    }

    fn map_property(&self, map: &Rc<RefCell<HashMap<String, Value>>>, name: &str) -> TechResult<Value> {
        let m = map.borrow();
        if let Some(val) = m.get(name) { return Ok(val.clone()); }
        match name {
            "keys" => Ok(Value::List(Rc::new(RefCell::new(m.keys().map(|k| Value::String(Rc::new(k.clone()))).collect())))),
            "values" => Ok(Value::List(Rc::new(RefCell::new(m.values().cloned().collect())))),
            _ => Err(self.runtime_error(format!("Unknown map key/method: {}", name))),
        }
    }

    fn op_index(&self, obj: &Value, idx: &Value) -> TechResult<Value> {
        match (obj, idx) {
            (Value::List(l), Value::Int(i)) => {
                let list = l.borrow();
                let index = if *i < 0 { (list.len() as i64 + *i) as usize } else { *i as usize };
                Ok(list.get(index).cloned().unwrap_or(Value::None))
            }
            (Value::Map(m), Value::String(k)) => Ok(m.borrow().get(k.as_ref()).cloned().unwrap_or(Value::None)),
            (Value::String(s), Value::Int(i)) => {
                let index = if *i < 0 { (s.len() as i64 + *i) as usize } else { *i as usize };
                Ok(s.chars().nth(index).map(|c| Value::String(Rc::new(c.to_string()))).unwrap_or(Value::None))
            }
            _ => Err(self.runtime_error("Invalid index operation")),
        }
    }

    fn op_set_index(&self, obj: &Value, idx: &Value, val: Value) -> TechResult<()> {
        match (obj, idx) {
            (Value::List(l), Value::Int(i)) => { l.borrow_mut()[*i as usize] = val; Ok(()) }
            (Value::Map(m), Value::String(k)) => { m.borrow_mut().insert(k.as_ref().clone(), val); Ok(()) }
            _ => Err(self.runtime_error("Invalid index set")),
        }
    }

    // Arithmetic helpers
    fn op_add(&self, a: Value, b: Value) -> TechResult<Value> {
        match (&a, &b) {
            (Value::Int(x), Value::Int(y)) => Ok(Value::Int(x + y)),
            (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x + y)),
            (Value::Int(x), Value::Float(y)) => Ok(Value::Float(*x as f64 + y)),
            (Value::Float(x), Value::Int(y)) => Ok(Value::Float(x + *y as f64)),
            (Value::String(x), Value::String(y)) => Ok(Value::String(Rc::new(format!("{}{}", x, y)))),
            (Value::List(x), Value::List(y)) => {
                let mut new = x.borrow().clone(); new.extend(y.borrow().iter().cloned());
                Ok(Value::List(Rc::new(RefCell::new(new))))
            }
            _ => Err(self.runtime_error(format!("Cannot add {} and {}", a.type_name(), b.type_name()))),
        }
    }
    fn op_sub(&self, a: Value, b: Value) -> TechResult<Value> {
        match (&a, &b) {
            (Value::Int(x), Value::Int(y)) => Ok(Value::Int(x - y)),
            (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x - y)),
            (Value::Int(x), Value::Float(y)) => Ok(Value::Float(*x as f64 - y)),
            (Value::Float(x), Value::Int(y)) => Ok(Value::Float(x - *y as f64)),
            _ => Err(self.runtime_error(format!("Cannot subtract {} and {}", a.type_name(), b.type_name()))),
        }
    }
    fn op_mul(&self, a: Value, b: Value) -> TechResult<Value> {
        match (&a, &b) {
            (Value::Int(x), Value::Int(y)) => Ok(Value::Int(x * y)),
            (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x * y)),
            (Value::Int(x), Value::Float(y)) => Ok(Value::Float(*x as f64 * y)),
            (Value::Float(x), Value::Int(y)) => Ok(Value::Float(x * *y as f64)),
            (Value::String(s), Value::Int(n)) => Ok(Value::String(Rc::new(s.repeat(*n as usize)))),
            _ => Err(self.runtime_error(format!("Cannot multiply {} and {}", a.type_name(), b.type_name()))),
        }
    }
    fn op_div(&self, a: Value, b: Value) -> TechResult<Value> {
        match (&a, &b) {
            (_, Value::Int(0)) => Err(self.runtime_error("Division by zero")),
            (_, Value::Float(f)) if *f == 0.0 => Err(self.runtime_error("Division by zero")),
            (Value::Int(x), Value::Int(y)) => Ok(Value::Float(*x as f64 / *y as f64)),
            (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x / y)),
            (Value::Int(x), Value::Float(y)) => Ok(Value::Float(*x as f64 / y)),
            (Value::Float(x), Value::Int(y)) => Ok(Value::Float(x / *y as f64)),
            _ => Err(self.runtime_error(format!("Cannot divide {} by {}", a.type_name(), b.type_name()))),
        }
    }
    fn op_intdiv(&self, a: Value, b: Value) -> TechResult<Value> {
        match (&a, &b) {
            (_, Value::Int(0)) => Err(self.runtime_error("Division by zero")),
            (Value::Int(x), Value::Int(y)) => Ok(Value::Int(x / y)),
            _ => self.op_div(a, b),
        }
    }
    fn op_mod(&self, a: Value, b: Value) -> TechResult<Value> {
        match (&a, &b) {
            (Value::Int(x), Value::Int(y)) => Ok(Value::Int(x % y)),
            (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x % y)),
            _ => Err(self.runtime_error("Modulo requires numbers")),
        }
    }
    fn op_pow(&self, a: Value, b: Value) -> TechResult<Value> {
        match (&a, &b) {
            (Value::Int(x), Value::Int(y)) if *y >= 0 => Ok(Value::Int(x.pow(*y as u32))),
            _ => {
                let x = a.as_f64().ok_or_else(|| self.runtime_error("Power requires numbers"))?;
                let y = b.as_f64().ok_or_else(|| self.runtime_error("Power requires numbers"))?;
                Ok(Value::Float(x.powf(y)))
            }
        }
    }
    fn op_cmp(&self, a: &Value, b: &Value, expected: std::cmp::Ordering) -> TechResult<Value> {
        match a.compare(b) {
            Some(ord) => Ok(Value::Bool(ord == expected)),
            None => Err(self.runtime_error(format!("Cannot compare {} and {}", a.type_name(), b.type_name()))),
        }
    }

    fn op_in(&self, item: &Value, container: &Value) -> TechResult<bool> {
        match container {
            Value::List(l) => Ok(l.borrow().iter().any(|v| v.equals(item))),
            Value::Map(m) => {
                let key = item.display_string();
                Ok(m.borrow().contains_key(&key))
            }
            Value::String(s) => {
                let sub = item.display_string();
                Ok(s.contains(sub.as_str()))
            }
            Value::Range(start, end, inclusive) => {
                if let Value::Int(n) = item {
                    let e = if *inclusive { *end } else { *end - 1 };
                    Ok(*n >= *start && *n <= e)
                } else {
                    Ok(false)
                }
            }
            _ => Err(self.runtime_error(format!("Cannot use 'in' with {}", container.type_name()))),
        }
    }
}
