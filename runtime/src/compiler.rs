// ── TechScript Bytecode Compiler ─────────────────────────────────────
// Walks the AST and emits bytecode instructions.

use std::collections::HashSet;
use std::rc::Rc;

use crate::ast::*;
use crate::chunk::Chunk;
use crate::error::{TechError, TechResult};
use crate::opcode::OpCode;
use crate::value::{Function, Value};

/// Tracks a local variable during compilation.
#[derive(Debug, Clone)]
struct Local {
    name: String,
    depth: i32,
    is_captured: bool,
}

/// Tracks an upvalue (closed-over variable).
#[derive(Debug, Clone, Copy)]
pub struct Upvalue {
    pub index: u8,
    pub is_local: bool,
}

/// Type of function being compiled.
#[derive(Debug, Clone, Copy, PartialEq)]
enum FunctionType {
    Script,
    Function,
    Method,
    Initializer,
}

/// The compiler state for a single function scope.
struct CompilerScope {
    function: Function,
    #[allow(dead_code)]
    fn_type: FunctionType,
    locals: Vec<Local>,
    upvalues: Vec<Upvalue>,
    scope_depth: i32,
}

impl CompilerScope {
    fn new(name: &str, fn_type: FunctionType) -> Self {
        let mut scope = CompilerScope {
            function: Function::new(name, 0),
            fn_type,
            locals: Vec::new(),
            upvalues: Vec::new(),
            scope_depth: 0,
        };
        // Reserve slot 0 for the function itself (or "self" for methods)
        let slot_name = if fn_type == FunctionType::Method || fn_type == FunctionType::Initializer {
            "self".to_string()
        } else {
            String::new()
        };
        scope.locals.push(Local { name: slot_name, depth: 0, is_captured: false });
        scope
    }
}

pub struct Compiler {
    scopes: Vec<CompilerScope>,
    /// Current class being compiled (name, has_superclass)
    class_stack: Vec<(String, bool)>,
    /// Stack of loop scopes for break/continue tracking
    loop_stack: Vec<LoopScope>,
    /// Globals defined at script scope (for bare-assignment checks)
    globals_defined: HashSet<String>,
    pub is_repl: bool,
}

/// Tracks current loop for break/continue jump patching.
struct LoopScope {
    /// Bytecode index where the loop starts (for `continue` backward jump)
    loop_start: usize,
    /// Indices of `break` jump instructions to be patched after the loop
    break_jumps: Vec<usize>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            scopes: vec![CompilerScope::new("<script>", FunctionType::Script)],
            class_stack: Vec::new(),
            loop_stack: Vec::new(),
            globals_defined: HashSet::new(),
            is_repl: false,
        }
    }

    fn current(&mut self) -> &mut CompilerScope {
        self.scopes.last_mut().unwrap()
    }

    fn chunk(&mut self) -> &mut Chunk {
        &mut self.current().function.chunk
    }

    fn emit(&mut self, op: OpCode, line: usize) {
        self.chunk().write_op(op, line);
    }

    fn emit_byte(&mut self, byte: u8, line: usize) {
        self.chunk().write(byte, line);
    }

    fn emit_constant(&mut self, val: Value, line: usize) {
        self.chunk().write_constant(val, line);
    }

    fn emit_jump(&mut self, op: OpCode, line: usize) -> usize {
        self.chunk().emit_jump(op, line)
    }

    fn patch_jump(&mut self, offset: usize) {
        self.chunk().patch_jump(offset);
    }

    fn emit_loop(&mut self, loop_start: usize, line: usize) {
        self.chunk().emit_loop(loop_start, line);
    }

    fn make_constant(&mut self, val: Value) -> usize {
        self.chunk().add_constant(val)
    }

    fn begin_scope(&mut self) {
        self.current().scope_depth += 1;
    }

    fn end_scope(&mut self, line: usize) {
        self.current().scope_depth -= 1;
        let depth = self.current().scope_depth;
        while !self.current().locals.is_empty()
            && self.current().locals.last().unwrap().depth > depth
        {
            let local = self.current().locals.pop().unwrap();
            if local.is_captured {
                self.emit(OpCode::CloseUpvalue, line);
            } else {
                self.emit(OpCode::Pop, line);
            }
        }
    }

    fn add_local(&mut self, name: &str) {
        let depth = self.current().scope_depth;
        self.current().locals.push(Local {
            name: name.to_string(),
            depth,
            is_captured: false,
        });
    }

    fn resolve_local(&self, scope_idx: usize, name: &str) -> Option<usize> {
        let scope = &self.scopes[scope_idx];
        for (i, local) in scope.locals.iter().enumerate().rev() {
            if local.name == name {
                return Some(i);
            }
        }
        None
    }

    fn resolve_upvalue(&mut self, scope_idx: usize, name: &str) -> Option<usize> {
        if scope_idx == 0 {
            return None;
        }

        // Check the immediately enclosing scope's locals
        if let Some(local_idx) = self.resolve_local(scope_idx - 1, name) {
            self.scopes[scope_idx - 1].locals[local_idx].is_captured = true;
            return Some(self.add_upvalue(scope_idx, local_idx as u8, true));
        }

        // Recurse to find it as an upvalue in the parent
        if let Some(upvalue_idx) = self.resolve_upvalue(scope_idx - 1, name) {
            return Some(self.add_upvalue(scope_idx, upvalue_idx as u8, false));
        }

        None
    }

    fn add_upvalue(&mut self, scope_idx: usize, index: u8, is_local: bool) -> usize {
        let scope = &mut self.scopes[scope_idx];
        // Check if already tracked
        for (i, uv) in scope.upvalues.iter().enumerate() {
            if uv.index == index && uv.is_local == is_local {
                return i;
            }
        }
        scope.upvalues.push(Upvalue { index, is_local });
        scope.function.upvalue_count = scope.upvalues.len();
        scope.upvalues.len() - 1
    }

    fn named_variable(&mut self, name: &str, get: bool, line: usize) {
        let scope_idx = self.scopes.len() - 1;

        if let Some(slot) = self.resolve_local(scope_idx, name) {
            if get {
                self.emit(OpCode::GetLocal, line);
                self.emit_byte(slot as u8, line);
            } else {
                self.emit(OpCode::SetLocal, line);
                self.emit_byte(slot as u8, line);
            }
        } else if let Some(idx) = self.resolve_upvalue(scope_idx, name) {
            if get {
                self.emit(OpCode::GetUpvalue, line);
                self.emit_byte(idx as u8, line);
            } else {
                self.emit(OpCode::SetUpvalue, line);
                self.emit_byte(idx as u8, line);
            }
        } else {
            let name_const = self.make_constant(Value::String(Rc::new(name.to_string())));
            if get {
                self.emit(OpCode::GetGlobal, line);
            } else {
                self.emit(OpCode::SetGlobal, line);
            }
            self.emit_byte((name_const >> 8) as u8, line);
            self.emit_byte((name_const & 0xFF) as u8, line);
        }
    }

    // ─── Public Entry Point ──────────────────────────────────────────

    pub fn compile(mut self, program: &Program) -> TechResult<Function> {
        let body_len = program.body.len();
        for (i, stmt) in program.body.iter().enumerate() {
            let is_last = i == body_len - 1;
            if self.is_repl && is_last {
                if let Stmt::Expression { expression } = stmt {
                    self.compile_expr(expression, 1)?;
                    self.emit(OpCode::Return, 0);
                    let scope = self.scopes.pop().unwrap();
                    return Ok(scope.function);
                }
            }
            self.compile_stmt(stmt, 1)?;
        }
        self.emit(OpCode::None, 0);
        self.emit(OpCode::Return, 0);

        let scope = self.scopes.pop().unwrap();
        Ok(scope.function)
    }

    // ─── Statements ─────────────────────────────────────────────────

    fn compile_stmt(&mut self, stmt: &Stmt, line: usize) -> TechResult<()> {
        match stmt {
            Stmt::Say { values } => {
                for val in values {
                    self.compile_expr(val, line)?;
                }
                self.emit(OpCode::Print, line);
                self.emit_byte(values.len() as u8, line);
            }

            Stmt::Set { name, value } | Stmt::Const { name, value } => {
                self.compile_expr(value, line)?;
                if self.current().scope_depth > 0 {
                    self.add_local(name);
                } else {
                    self.globals_defined.insert(name.clone());
                    let idx = self.make_constant(Value::String(Rc::new(name.clone())));
                    self.emit(OpCode::DefineGlobal, line);
                    self.emit_byte((idx >> 8) as u8, line);
                    self.emit_byte((idx & 0xFF) as u8, line);
                }
            }

            Stmt::Assign { target, op, value } => {
                match target {
                    Expr::Identifier(name) => {
                        if op == "="
                            && self.scopes.len() == 1
                            && self.current().scope_depth == 0
                            && self.resolve_local(0, name).is_none()
                            && self.resolve_upvalue(0, name).is_none()
                            && !self.globals_defined.contains(name)
                        {
                            return Err(TechError::compile(
                                format!(
                                    "Undefined variable '{}'. Use `make` or `const` for first declaration.",
                                    name
                                ),
                                line,
                                0,
                            ));
                        }
                        // Compound assignment
                        if op != "=" {
                            self.named_variable(name, true, line);
                        }
                        self.compile_expr(value, line)?;
                        match op.as_str() {
                            "+=" => self.emit(OpCode::Add, line),
                            "-=" => self.emit(OpCode::Subtract, line),
                            "*=" => self.emit(OpCode::Multiply, line),
                            "/=" => self.emit(OpCode::Divide, line),
                            "=" => {}
                            _ => return Err(TechError::compile(format!("Unknown assignment operator: {}", op), line, 0)),
                        }
                        self.named_variable(name, false, line);
                    }
                    Expr::Member { obj, member } => {
                        self.compile_expr(obj, line)?;
                        self.compile_expr(value, line)?;
                        let name_idx = self.make_constant(Value::String(Rc::new(member.clone())));
                        self.emit(OpCode::SetProperty, line);
                        self.emit_byte((name_idx >> 8) as u8, line);
                        self.emit_byte((name_idx & 0xFF) as u8, line);
                    }
                    Expr::Index { obj, index } => {
                        self.compile_expr(obj, line)?;
                        self.compile_expr(index, line)?;
                        self.compile_expr(value, line)?;
                        self.emit(OpCode::SetIndex, line);
                    }
                    _ => return Err(TechError::compile("Invalid assignment target", line, 0)),
                }
            }

            Stmt::Expression { expression } => {
                self.compile_expr(expression, line)?;
                self.emit(OpCode::Pop, line);
            }

            Stmt::If { condition, body, elif_clauses, else_body } => {
                self.compile_expr(condition, line)?;
                let then_jump = self.emit_jump(OpCode::JumpIfFalse, line);
                self.emit(OpCode::Pop, line); // pop condition

                for s in body {
                    self.compile_stmt(s, line)?;
                }

                let mut end_jumps = vec![self.emit_jump(OpCode::Jump, line)];
                self.patch_jump(then_jump);
                self.emit(OpCode::Pop, line); // pop condition

                for (elif_cond, elif_body) in elif_clauses {
                    self.compile_expr(elif_cond, line)?;
                    let elif_jump = self.emit_jump(OpCode::JumpIfFalse, line);
                    self.emit(OpCode::Pop, line);
                    for s in elif_body {
                        self.compile_stmt(s, line)?;
                    }
                    end_jumps.push(self.emit_jump(OpCode::Jump, line));
                    self.patch_jump(elif_jump);
                    self.emit(OpCode::Pop, line);
                }

                if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        self.compile_stmt(s, line)?;
                    }
                }

                for j in end_jumps {
                    self.patch_jump(j);
                }
            }

            Stmt::Unless { condition, body } => {
                self.compile_expr(condition, line)?;
                self.emit(OpCode::Not, line);
                let jump = self.emit_jump(OpCode::JumpIfFalse, line);
                self.emit(OpCode::Pop, line);
                for s in body {
                    self.compile_stmt(s, line)?;
                }
                let end_jump = self.emit_jump(OpCode::Jump, line);
                self.patch_jump(jump);
                self.emit(OpCode::Pop, line);
                self.patch_jump(end_jump);
            }

            Stmt::While { condition, body } => {
                let loop_start = self.chunk().len();
                self.loop_stack.push(LoopScope { loop_start, break_jumps: Vec::new() });

                self.compile_expr(condition, line)?;
                let exit_jump = self.emit_jump(OpCode::JumpIfFalse, line);
                self.emit(OpCode::Pop, line);

                for s in body {
                    self.compile_stmt(s, line)?;
                }

                self.emit_loop(loop_start, line);
                self.patch_jump(exit_jump);
                self.emit(OpCode::Pop, line);

                let scope = self.loop_stack.pop().unwrap();
                for bj in scope.break_jumps { self.patch_jump(bj); }
            }

            Stmt::Until { condition, body } => {
                // until X { ... }  →  repeat not X { ... }
                let loop_start = self.chunk().len();
                self.loop_stack.push(LoopScope { loop_start, break_jumps: Vec::new() });

                self.compile_expr(condition, line)?;
                self.emit(OpCode::Not, line);
                let exit_jump = self.emit_jump(OpCode::JumpIfFalse, line);
                self.emit(OpCode::Pop, line);

                for s in body {
                    self.compile_stmt(s, line)?;
                }

                self.emit_loop(loop_start, line);
                self.patch_jump(exit_jump);
                self.emit(OpCode::Pop, line);

                let scope = self.loop_stack.pop().unwrap();
                for bj in scope.break_jumps { self.patch_jump(bj); }
            }

            Stmt::For { var_name, iterable, body } => {
                self.begin_scope();

                // Compile iterable and create iterator
                self.compile_expr(iterable, line)?;
                self.emit(OpCode::GetIter, line);
                self.add_local("__iter__");

                let loop_start = self.chunk().len();
                self.loop_stack.push(LoopScope { loop_start, break_jumps: Vec::new() });

                // Get next value or jump
                let exit_jump = self.emit_jump(OpCode::IterNext, line);

                // Add loop variable
                self.add_local(var_name);

                for s in body {
                    self.compile_stmt(s, line)?;
                }

                // Pop loop variable
                self.emit(OpCode::Pop, line);

                self.emit_loop(loop_start, line);
                self.patch_jump(exit_jump);

                let scope = self.loop_stack.pop().unwrap();
                for bj in scope.break_jumps { self.patch_jump(bj); }

                self.end_scope(line);
            }

            Stmt::Fn { name, params, body } => {
                self.compile_function(name, params, body, FunctionType::Function, line)?;
                if self.current().scope_depth > 0 {
                    self.add_local(name);
                } else {
                    let idx = self.make_constant(Value::String(Rc::new(name.clone())));
                    self.emit(OpCode::DefineGlobal, line);
                    self.emit_byte((idx >> 8) as u8, line);
                    self.emit_byte((idx & 0xFF) as u8, line);
                }
            }

            Stmt::Class { name, parent, body } => {
                let name_const = self.make_constant(Value::String(Rc::new(name.clone())));
                self.emit(OpCode::Class, line);
                self.emit_byte((name_const >> 8) as u8, line);
                self.emit_byte((name_const & 0xFF) as u8, line);

                if self.current().scope_depth > 0 {
                    self.add_local(name);
                } else {
                    let idx = self.make_constant(Value::String(Rc::new(name.clone())));
                    self.emit(OpCode::DefineGlobal, line);
                    self.emit_byte((idx >> 8) as u8, line);
                    self.emit_byte((idx & 0xFF) as u8, line);
                }

                let has_parent = parent.is_some();
                if let Some(parent_name) = parent {
                    self.named_variable(parent_name, true, line);
                    self.named_variable(name, true, line);
                    self.emit(OpCode::Inherit, line);
                }

                self.class_stack.push((name.clone(), has_parent));

                self.named_variable(name, true, line);

                for stmt in body {
                    if let Stmt::Fn { name: method_name, params, body: method_body } = stmt {
                        let fn_type = if method_name == "init" {
                            FunctionType::Initializer
                        } else {
                            FunctionType::Method
                        };
                        self.compile_function(method_name, params, method_body, fn_type, line)?;
                        let method_idx = self.make_constant(Value::String(Rc::new(method_name.clone())));
                        self.emit(OpCode::Method, line);
                        self.emit_byte((method_idx >> 8) as u8, line);
                        self.emit_byte((method_idx & 0xFF) as u8, line);
                    }
                }

                self.emit(OpCode::Pop, line); // pop class

                self.class_stack.pop();
            }

            Stmt::Return { value } => {
                if let Some(val) = value {
                    self.compile_expr(val, line)?;
                } else {
                    self.emit(OpCode::None, line);
                }
                self.emit(OpCode::Return, line);
            }

            Stmt::Break => {
                if self.loop_stack.is_empty() {
                    return Err(TechError::compile("'break' used outside of loop", line, 0));
                }
                // Emit a forward jump placeholder; store it for patching after the loop
                let break_jump = self.emit_jump(OpCode::Jump, line);
                self.loop_stack.last_mut().unwrap().break_jumps.push(break_jump);
            }

            Stmt::Skip => {
                if let Some(scope) = self.loop_stack.last() {
                    let start = scope.loop_start;
                    self.emit_loop(start, line);
                }
            }

            Stmt::Pass => {
                // no-op
            }

            Stmt::Try { body, catch_var, catch_body, finally_body } => {
                let setup_jump = self.emit_jump(OpCode::SetupTry, line);

                for s in body {
                    self.compile_stmt(s, line)?;
                }

                self.emit(OpCode::PopTry, line); // Success path, pop try handler
                let success_jump = self.emit_jump(OpCode::Jump, line);

                // Catch block (VM will jump here on error, pushing the error value)
                self.patch_jump(setup_jump);

                if let Some(var_name) = catch_var {
                    self.begin_scope();
                    self.add_local(var_name); // Error value pushed by VM is mapped to var_name
                    for s in catch_body {
                        self.compile_stmt(s, line)?;
                    }
                    self.end_scope(line);
                } else {
                    self.emit(OpCode::Pop, line); // No variable specified, discard the error value
                    for s in catch_body {
                        self.compile_stmt(s, line)?;
                    }
                }

                self.patch_jump(success_jump);

                if let Some(finally_stmts) = finally_body {
                    for s in finally_stmts {
                        self.compile_stmt(s, line)?;
                    }
                }
            }

            Stmt::Throw { value } => {
                self.compile_expr(value, line)?;
                self.emit(OpCode::Throw, line);
            }

            Stmt::Match { subject, cases } => {
                self.compile_expr(subject, line)?;
                let mut end_jumps = Vec::new();

                for (pattern, case_body) in cases {
                    self.emit(OpCode::Dup, line);
                    self.compile_expr(pattern, line)?;
                    self.emit(OpCode::Equal, line);
                    let skip_jump = self.emit_jump(OpCode::JumpIfFalse, line);
                    self.emit(OpCode::Pop, line); // pop comparison result
                    self.emit(OpCode::Pop, line); // pop subject duplicate

                    for s in case_body {
                        self.compile_stmt(s, line)?;
                    }
                    end_jumps.push(self.emit_jump(OpCode::Jump, line));

                    self.patch_jump(skip_jump);
                    self.emit(OpCode::Pop, line); // pop comparison result
                }

                self.emit(OpCode::Pop, line); // pop original subject

                for j in end_jumps {
                    self.patch_jump(j);
                }
            }

            Stmt::Import { module, .. } => {
                let name_idx = self.make_constant(Value::String(Rc::new(module.clone())));
                self.emit(OpCode::Import, line);
                self.emit_byte((name_idx >> 8) as u8, line);
                self.emit_byte((name_idx & 0xFF) as u8, line);
            }

            Stmt::FromImport { module, .. } => {
                let name_idx = self.make_constant(Value::String(Rc::new(module.clone())));
                self.emit(OpCode::Import, line);
                self.emit_byte((name_idx >> 8) as u8, line);
                self.emit_byte((name_idx & 0xFF) as u8, line);
            }

            Stmt::Del { name } => {
                self.emit(OpCode::None, line);
                self.named_variable(name, false, line);
            }

            Stmt::Defer { expression } => {
                self.compile_expr(expression, line)?;
                self.emit(OpCode::Pop, line);
            }

            Stmt::Guard { condition, else_body } => {
                self.compile_expr(condition, line)?;
                let ok_jump = self.emit_jump(OpCode::JumpIfFalse, line);
                self.emit(OpCode::Pop, line);
                let skip = self.emit_jump(OpCode::Jump, line);

                self.patch_jump(ok_jump);
                self.emit(OpCode::Pop, line);
                for s in else_body {
                    self.compile_stmt(s, line)?;
                }

                self.patch_jump(skip);
            }

            Stmt::With { expression, var_name, body } => {
                self.begin_scope();
                self.compile_expr(expression, line)?;
                self.add_local(var_name);
                for s in body {
                    self.compile_stmt(s, line)?;
                }
                self.end_scope(line);
            }

            Stmt::Export { declaration } => {
                self.compile_stmt(declaration, line)?;
            }

            Stmt::State { name, value } => {
                self.emit_call_global("__web_state", line);
                self.emit_string_const(&name, line);
                self.compile_expr(value, line)?;
                self.emit(OpCode::Call, line);
                self.emit_byte(2, line);
                self.emit(OpCode::Pop, line);
            }

            Stmt::Component { name, body } => {
                let html = self.compile_render_body(body, line)?;
                self.emit_call_global("__web_component", line);
                self.emit_string_const(&name, line);
                self.emit_string_const(&html, line);
                self.emit(OpCode::Call, line);
                self.emit_byte(2, line);
                self.emit(OpCode::Pop, line);
            }

            Stmt::Page { name, body } => {
                let html = self.compile_render_body(body, line)?;
                self.emit_call_global("__web_page", line);
                self.emit_string_const(&name, line);
                self.emit_string_const(&html, line);
                self.emit(OpCode::Call, line);
                self.emit_byte(2, line);
                self.emit(OpCode::Pop, line);
            }

            Stmt::Api { name, routes } => {
                for (method, path, route_body) in routes {
                    let response = self.compile_route_response(route_body);
                    self.emit_call_global("__web_route", line);
                    self.emit_string_const(method, line);
                    self.emit_string_const(path, line);
                    self.emit_string_const(&response, line);
                    self.emit(OpCode::Call, line);
                    self.emit_byte(3, line);
                    self.emit(OpCode::Pop, line);
                }
                let _ = name;
            }

            Stmt::Render { tag, body } => {
                let inner = self.compile_render_body(body, line)?;
                self.emit_call_global("__web_render", line);
                self.emit_string_const(&tag, line);
                self.emit_string_const(&inner, line);
                self.emit(OpCode::Call, line);
                self.emit_byte(2, line);
            }

            Stmt::Window { title, body } => {
                self.emit_call_global("__gui_window", line);
                self.emit_string_const(&title, line);
                self.emit(OpCode::Call, line);
                self.emit_byte(1, line);
                self.emit(OpCode::Pop, line);
                for s in body {
                    self.compile_stmt(s, line)?;
                }
                self.emit_call_global("__gui_run", line);
                self.emit(OpCode::Call, line);
                self.emit_byte(0, line);
                self.emit(OpCode::Pop, line);
            }

            Stmt::Button { label, body } => {
                let handler = self.compile_button_handler(body);
                self.emit_call_global("__gui_button", line);
                self.emit_string_const(&label, line);
                self.emit_string_const(&handler, line);
                self.emit(OpCode::Call, line);
                self.emit_byte(2, line);
                self.emit(OpCode::Pop, line);
            }

            Stmt::Input { name, placeholder } => {
                self.emit_call_global("__gui_input", line);
                self.emit_string_const(&name, line);
                self.emit_string_const(&placeholder, line);
                self.emit(OpCode::Call, line);
                self.emit_byte(2, line);
                self.emit(OpCode::Pop, line);
            }

            Stmt::Label { text } => {
                self.emit_call_global("__gui_label", line);
                self.emit_string_const(&text, line);
                self.emit(OpCode::Call, line);
                self.emit_byte(1, line);
                self.emit(OpCode::Pop, line);
            }

            Stmt::Scene { name, body } => {
                self.emit_call_global("__3d_scene", line);
                self.emit_string_const(&name, line);
                self.emit(OpCode::Call, line);
                self.emit_byte(1, line);
                self.emit(OpCode::Pop, line);
                for s in body {
                    self.compile_stmt(s, line)?;
                }
                self.emit_call_global("__3d_run", line);
                self.emit(OpCode::Call, line);
                self.emit_byte(0, line);
                self.emit(OpCode::Pop, line);
            }

            Stmt::Timeline { name, body } => {
                self.emit_call_global("__anime_timeline", line);
                self.emit_string_const(&name, line);
                self.emit(OpCode::Call, line);
                self.emit_byte(1, line);
                self.emit(OpCode::Pop, line);
                for s in body {
                    self.compile_stmt(s, line)?;
                }
                self.emit_call_global("__anime_run", line);
                self.emit(OpCode::Call, line);
                self.emit_byte(0, line);
                self.emit(OpCode::Pop, line);
            }

            Stmt::AnimeMove {
                target,
                coords,
                duration,
                ease,
            } => {
                self.emit_call_global("__anime_move", line);
                self.emit_string_const(target, line);
                self.emit_string_const(&self.format_list_expr(coords), line);
                self.compile_expr(duration, line)?;
                self.emit_string_const(ease, line);
                self.emit(OpCode::Call, line);
                self.emit_byte(4, line);
                self.emit(OpCode::Pop, line);
            }

            Stmt::AnimeFade {
                target,
                opacity,
                duration,
            } => {
                self.emit_call_global("__anime_fade", line);
                self.emit_string_const(target, line);
                self.compile_expr(opacity, line)?;
                self.compile_expr(duration, line)?;
                self.emit(OpCode::Call, line);
                self.emit_byte(3, line);
                self.emit(OpCode::Pop, line);
            }

            Stmt::Camera { coords } => {
                self.emit_call_global("__3d_camera", line);
                for i in 0..3 {
                    if let Some(c) = coords.get(i) {
                        self.compile_expr(c, line)?;
                    } else {
                        let default = if i == 2 { 5 } else { 0 };
                        let idx = self.make_constant(Value::Int(default));
                        self.emit(OpCode::Constant, line);
                        self.emit_byte((idx >> 8) as u8, line);
                        self.emit_byte((idx & 0xFF) as u8, line);
                    }
                }
                self.emit(OpCode::Call, line);
                self.emit_byte(3, line);
                self.emit(OpCode::Pop, line);
            }

            Stmt::Light { kind: _ } => {
                self.emit_call_global("__3d_light", line);
                self.emit(OpCode::Call, line);
                self.emit_byte(0, line);
                self.emit(OpCode::Pop, line);
            }

            Stmt::Mesh { shape, color } => {
                self.emit_call_global("__3d_mesh", line);
                self.emit_string_const(&shape, line);
                self.emit_string_const(&color, line);
                self.emit(OpCode::Call, line);
                self.emit_byte(2, line);
                self.emit(OpCode::Pop, line);
            }
        }
        Ok(())
    }

    fn emit_call_global(&mut self, name: &str, line: usize) {
        self.named_variable(name, true, line);
    }

    fn emit_string_const(&mut self, s: &str, line: usize) {
        let idx = self.make_constant(Value::String(Rc::new(s.to_string())));
        self.emit(OpCode::Constant, line);
        self.emit_byte((idx >> 8) as u8, line);
        self.emit_byte((idx & 0xFF) as u8, line);
    }

    fn compile_render_body(&mut self, body: &[Stmt], line: usize) -> TechResult<String> {
        let mut html = String::new();
        for stmt in body {
            match stmt {
                Stmt::Render { tag, body: inner } => {
                    let inner_html = self.compile_render_body(inner, line)?;
                    html.push_str(&format!("<{}>{}</{}>", tag, inner_html, tag));
                }
                Stmt::Say { values } => {
                    for v in values {
                        if let Expr::String(s) = v {
                            html.push_str(s);
                        }
                    }
                }
                Stmt::Expression { expression: Expr::String(s) } => {
                    html.push_str(s);
                }
                _ => {}
            }
        }
        Ok(html)
    }

    // ─── Function Compilation ────────────────────────────────────────

    fn compile_function(
        &mut self,
        name: &str,
        params: &[Param],
        body: &[Stmt],
        fn_type: FunctionType,
        line: usize,
    ) -> TechResult<()> {
        let mut new_scope = CompilerScope::new(name, fn_type);
        new_scope.function.arity = params.len();
        new_scope.scope_depth = self.current().scope_depth + 1;

        self.scopes.push(new_scope);

        self.begin_scope();

        // Add parameters as locals (slot 0 is reserved implicit `self` for methods)
        for param in params {
            if (fn_type == FunctionType::Method || fn_type == FunctionType::Initializer)
                && param.name == "self"
            {
                continue;
            }
            self.add_local(&param.name);
        }

        // Compile body
        for s in body {
            self.compile_stmt(s, line)?;
        }

        // Implicit return
        if fn_type == FunctionType::Initializer {
            self.emit(OpCode::GetLocal, line);
            self.emit_byte(0, line); // return self
        } else {
            self.emit(OpCode::None, line);
        }
        self.emit(OpCode::Return, line);

        let scope = self.scopes.pop().unwrap();
        let func = Rc::new(scope.function);
        let upvalues = scope.upvalues;

        // Emit closure instruction in parent scope
        let fn_const = self.make_constant(Value::Function(func));
        self.emit(OpCode::Closure, line);
        self.emit_byte((fn_const >> 8) as u8, line);
        self.emit_byte((fn_const & 0xFF) as u8, line);

        // Emit upvalue info
        for uv in &upvalues {
            self.emit_byte(if uv.is_local { 1 } else { 0 }, line);
            self.emit_byte(uv.index, line);
        }

        Ok(())
    }

    // ─── Expressions ────────────────────────────────────────────────

    fn compile_expr(&mut self, expr: &Expr, line: usize) -> TechResult<()> {
        match expr {
            Expr::NumberInt(v) => {
                self.emit_constant(Value::Int(*v), line);
            }
            Expr::NumberFloat(v) => {
                self.emit_constant(Value::Float(*v), line);
            }
            Expr::String(s) => {
                self.emit_constant(Value::String(Rc::new(s.clone())), line);
            }
            Expr::FString(raw) => {
                // Parse f-string parts: split on { and }
                self.compile_fstring(raw, line)?;
            }
            Expr::Bool(true) => self.emit(OpCode::True, line),
            Expr::Bool(false) => self.emit(OpCode::False, line),
            Expr::None => self.emit(OpCode::None, line),

            Expr::Identifier(name) => {
                self.named_variable(name, true, line);
            }

            Expr::BinaryOp { left, op, right } => {
                // Short-circuit for and/or
                if op == "and" {
                    self.compile_expr(left, line)?;
                    let end_jump = self.emit_jump(OpCode::JumpIfFalse, line);
                    self.emit(OpCode::Pop, line);
                    self.compile_expr(right, line)?;
                    self.patch_jump(end_jump);
                    return Ok(());
                }
                if op == "or" {
                    self.compile_expr(left, line)?;
                    let else_jump = self.emit_jump(OpCode::JumpIfFalse, line);
                    let end_jump = self.emit_jump(OpCode::Jump, line);
                    self.patch_jump(else_jump);
                    self.emit(OpCode::Pop, line);
                    self.compile_expr(right, line)?;
                    self.patch_jump(end_jump);
                    return Ok(());
                }

                if op == "|>" {
                    self.compile_expr(right, line)?;
                    self.compile_expr(left, line)?;
                    self.emit(OpCode::Call, line);
                    self.emit_byte(1, line);
                    return Ok(());
                }

                self.compile_expr(left, line)?;
                self.compile_expr(right, line)?;
                match op.as_str() {
                    "+" => self.emit(OpCode::Add, line),
                    "-" => self.emit(OpCode::Subtract, line),
                    "*" => self.emit(OpCode::Multiply, line),
                    "/" => self.emit(OpCode::Divide, line),
                    "//" => self.emit(OpCode::IntDivide, line),
                    "%" => self.emit(OpCode::Modulo, line),
                    "**" => self.emit(OpCode::Power, line),
                    "==" => self.emit(OpCode::Equal, line),
                    "!=" => self.emit(OpCode::NotEqual, line),
                    "<" => self.emit(OpCode::Less, line),
                    ">" => self.emit(OpCode::Greater, line),
                    "<=" => self.emit(OpCode::LessEqual, line),
                    ">=" => self.emit(OpCode::GreaterEqual, line),
                    "is" => self.emit(OpCode::Equal, line),
                    "in" => self.emit(OpCode::In, line),
                    _ => return Err(TechError::compile(format!("Unknown binary operator: {}", op), line, 0)),
                }
            }

            Expr::UnaryOp { op, operand } => {
                match op.as_str() {
                    "typeof" => {
                        // Compile operand, then emit TypeOf
                        self.compile_expr(operand, line)?;
                        self.emit(OpCode::TypeOf, line);
                    }
                    _ => {
                        self.compile_expr(operand, line)?;
                        match op.as_str() {
                            "-" => self.emit(OpCode::Negate, line),
                            "not" => self.emit(OpCode::Not, line),
                            _ => return Err(TechError::compile(format!("Unknown unary operator: {}", op), line, 0)),
                        }
                    }
                }
            }

            Expr::Call { callee, args } => {
                self.compile_expr(callee, line)?;
                for arg in args {
                    self.compile_expr(arg, line)?;
                }
                self.emit(OpCode::Call, line);
                self.emit_byte(args.len() as u8, line);
            }

            Expr::Member { obj, member } => {
                self.compile_expr(obj, line)?;
                let name_idx = self.make_constant(Value::String(Rc::new(member.clone())));
                self.emit(OpCode::GetProperty, line);
                self.emit_byte((name_idx >> 8) as u8, line);
                self.emit_byte((name_idx & 0xFF) as u8, line);
            }

            Expr::Index { obj, index } => {
                self.compile_expr(obj, line)?;
                self.compile_expr(index, line)?;
                self.emit(OpCode::Index, line);
            }

            Expr::List(elements) => {
                for elem in elements {
                    self.compile_expr(elem, line)?;
                }
                self.emit(OpCode::BuildList, line);
                self.emit_byte(elements.len() as u8, line);
            }

            Expr::Map(entries) => {
                for (key, val) in entries {
                    self.compile_expr(key, line)?;
                    self.compile_expr(val, line)?;
                }
                self.emit(OpCode::BuildMap, line);
                self.emit_byte(entries.len() as u8, line);
            }

            Expr::Lambda { params, body } => {
                // Compile as an anonymous function
                let lambda_stmts = vec![Stmt::Return { value: Some(*body.clone()) }];
                self.compile_function("<lambda>", params, &lambda_stmts, FunctionType::Function, line)?;
            }

            Expr::Range { start, end, inclusive } => {
                self.compile_expr(start, line)?;
                self.compile_expr(end, line)?;
                if *inclusive {
                    self.emit(OpCode::BuildRangeInclusive, line);
                } else {
                    self.emit(OpCode::BuildRange, line);
                }
            }

            Expr::Ternary { true_val, condition, false_val } => {
                self.compile_expr(condition, line)?;
                let else_jump = self.emit_jump(OpCode::JumpIfFalse, line);
                self.emit(OpCode::Pop, line); // pop condition
                self.compile_expr(true_val, line)?;
                let end_jump = self.emit_jump(OpCode::Jump, line);
                self.patch_jump(else_jump);
                self.emit(OpCode::Pop, line); // pop condition
                self.compile_expr(false_val, line)?;
                self.patch_jump(end_jump);
            }

            Expr::Ask { prompt } => {
                self.compile_expr(prompt, line)?;
                self.emit(OpCode::ReadInput, line);
            }
        }
        Ok(())
    }

    /// Compile an f-string by splitting on `{` and `}` boundaries.
    fn compile_fstring(&mut self, raw: &str, line: usize) -> TechResult<()> {
        let mut parts: Vec<(bool, String)> = Vec::new(); // (is_expr, content)
        let chars: Vec<char> = raw.chars().collect();
        let mut i = 0;
        let mut current_str = String::new();

        while i < chars.len() {
            if chars[i] == '{' {
                if !current_str.is_empty() {
                    parts.push((false, current_str.clone()));
                    current_str.clear();
                }
                i += 1;
                let mut expr_str = String::new();
                let mut depth = 1;
                while i < chars.len() && depth > 0 {
                    if chars[i] == '{' { depth += 1; }
                    if chars[i] == '}' { depth -= 1; }
                    if depth > 0 {
                        expr_str.push(chars[i]);
                    }
                    i += 1;
                }
                parts.push((true, expr_str));
            } else {
                current_str.push(chars[i]);
                i += 1;
            }
        }
        if !current_str.is_empty() {
            parts.push((false, current_str));
        }

        let part_count = parts.len();
        for (is_expr, content) in &parts {
            if *is_expr {
                // Parse and compile the expression
                let tokens = crate::lexer::Lexer::new(content, "<fstring>").tokenize()
                    .map_err(|e| TechError::compile(format!("F-string expression error: {}", e.message), line, 0))?;
                let expr_prog = crate::parser::Parser::new(tokens, "<fstring>").parse()
                    .map_err(|e| TechError::compile(format!("F-string expression error: {}", e.message), line, 0))?;

                if let Some(stmt) = expr_prog.body.first() {
                    if let Stmt::Expression { expression } = stmt {
                        self.compile_expr(expression, line)?;
                    } else {
                        self.emit(OpCode::None, line);
                    }
                } else {
                    self.emit(OpCode::None, line);
                }
            } else {
                self.emit_constant(Value::String(Rc::new(content.clone())), line);
            }
        }

        self.emit(OpCode::FormatString, line);
        self.emit_byte(part_count as u8, line);
        Ok(())
    }

    /// Build a plain-text HTTP response body from route handler `say` statements.
    fn compile_route_response(&self, body: &[Stmt]) -> String {
        let mut parts = Vec::new();
        for stmt in body {
            if let Stmt::Say { values } = stmt {
                for v in values {
                    if let Expr::String(s) = v {
                        parts.push(s.clone());
                    }
                }
            }
        }
        if parts.is_empty() {
            "{\"ok\":true}".to_string()
        } else {
            format!("{{\"ok\":true,\"message\":\"{}\"}}", parts.join(" ").replace('"', "\\\""))
        }
    }

    fn format_list_expr(&self, exprs: &[Expr]) -> String {
        let parts: Vec<String> = exprs
            .iter()
            .map(|e| match e {
                Expr::NumberInt(i) => i.to_string(),
                Expr::NumberFloat(f) => f.to_string(),
                Expr::String(s) => s.clone(),
                _ => "0".into(),
            })
            .collect();
        format!("[{}]", parts.join(", "))
    }

    /// Encode button click actions as `say` messages separated by `|`.
    fn compile_button_handler(&self, body: &[Stmt]) -> String {
        let mut parts = Vec::new();
        for stmt in body {
            if let Stmt::Say { values } = stmt {
                for v in values {
                    if let Expr::String(s) = v {
                        parts.push(s.clone());
                    }
                }
            }
        }
        parts.join("|")
    }
}




