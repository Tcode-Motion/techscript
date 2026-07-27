use crate::builder::IRBuilder;
use crate::instruction::{Op, TerminatorKind};
use crate::module::{DslBlockIR, Module};
use crate::types::{BlockId, DslBlockId, IRType, LocalId};
use crate::value::Value;
use std::collections::HashMap;
use techscript_ast::{
    AssignmentExpr, BinaryExpr, Block, DSLBlock, DSLChild, DSLProperty, Expression, FuncDecl,
    LiteralVal, ModelDecl, Pattern, Program, Statement, StructDecl, UnaryExpr, VarDecl,
};
use techscript_common::Span;
use techscript_errors::Diagnostic;

/// Mapping context holding current symbol registers and loop targets.
#[derive(Debug, Clone)]
pub enum SymbolBinding {
    Local(LocalId, IRType),
    Global(crate::types::GlobalId, IRType),
}

/// The final lowering results containing the IR Module and any diagnostics.
pub struct LoweringResult {
    pub module: Module,
    pub diagnostics: Vec<Diagnostic>,
}

/// Dynamic context orchestrating AST lowering to intermediate blocks.
pub struct LoweringContext {
    builder: IRBuilder,
    symbol_map: HashMap<String, SymbolBinding>,
    break_stack: Vec<BlockId>,
    continue_stack: Vec<BlockId>,
    diagnostics: Vec<Diagnostic>,
}

impl Default for LoweringContext {
    fn default() -> Self {
        Self::new()
    }
}

impl LoweringContext {
    /// Creates a new LoweringContext.
    pub fn new() -> Self {
        Self {
            builder: IRBuilder::new(),
            symbol_map: HashMap::new(),
            break_stack: Vec::new(),
            continue_stack: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    /// Translates an AST Program into an IR Module.
    pub fn lower(mut self, program: &Program, name: &str) -> LoweringResult {
        // First pass: declare functions and models as global symbols or constructor slots
        for stmt in &program.statements {
            match stmt {
                Statement::FuncDecl(decl) => {
                    let ty = IRType::Any;
                    let global_id = self
                        .builder
                        .declare_global(decl.name.name.clone(), ty.clone());
                    self.symbol_map
                        .insert(decl.name.name.clone(), SymbolBinding::Global(global_id, ty));
                }
                Statement::ModelDecl(decl) => {
                    let ty = IRType::Model(decl.name.name.clone());
                    let global_id = self
                        .builder
                        .declare_global(decl.name.name.clone(), ty.clone());
                    self.symbol_map
                        .insert(decl.name.name.clone(), SymbolBinding::Global(global_id, ty));
                }
                Statement::StructDecl(decl) => {
                    let ty = IRType::Struct(decl.name.name.clone());
                    let global_id = self
                        .builder
                        .declare_global(decl.name.name.clone(), ty.clone());
                    self.symbol_map
                        .insert(decl.name.name.clone(), SymbolBinding::Global(global_id, ty));
                }
                _ => {}
            }
        }

        // Lower statements sequentially inside main
        let _main_id = self
            .builder
            .start_function("main".to_string(), IRType::Void);
        let entry_id = self.builder.new_block("entry".to_string());
        self.builder.enter_block(entry_id, "entry".to_string());

        for stmt in &program.statements {
            if !matches!(stmt, Statement::FuncDecl(_)) {
                self.lower_statement(stmt);
            }
        }

        // If there is an AST main function (explicit or synthetic), append its body to this main function
        if let Some(main_decl) = program.statements.iter().find_map(|s| {
            if let Statement::FuncDecl(decl) = s {
                if decl.name.name == "main" {
                    return Some(decl);
                }
            }
            None
        }) {
            for param in &main_decl.params {
                let ty = self.map_type_spec(&param.type_ann);
                let local_id = self
                    .builder
                    .add_parameter(param.name.name.clone(), ty.clone());
                self.symbol_map
                    .insert(param.name.name.clone(), SymbolBinding::Local(local_id, ty));
            }
            self.lower_block(&main_decl.body);
        }

        // Emit final unreachable/return terminator if main has no terminator
        self.builder
            .emit_terminator(TerminatorKind::Return(None), program.span);
        self.builder.seal_function();

        // Lower function declarations separately at the module level (skipping main)
        for stmt in &program.statements {
            if let Statement::FuncDecl(decl) = stmt {
                if decl.name.name != "main" {
                    self.lower_func_decl(decl);
                }
            }
        }

        LoweringResult {
            module: self.builder.build(name.to_string()),
            diagnostics: self.diagnostics,
        }
    }

    fn map_type_spec(&self, spec: &Option<techscript_ast::TypeSpec>) -> IRType {
        if let Some(ref s) = spec {
            match s.name.name.as_str() {
                "int" | "int64" | "Int" => IRType::Int64,
                "float" | "float64" | "Float" => IRType::Float64,
                "bool" | "Bool" => IRType::Bool,
                "string" | "String" => IRType::String,
                "list" | "List" => IRType::List,
                "map" | "Map" => IRType::Map,
                "void" | "Void" => IRType::Void,
                other => IRType::Struct(other.to_string()),
            }
        } else {
            IRType::Any
        }
    }

    fn lower_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::VarDecl(decl) => self.lower_var_decl(decl),
            Statement::ConstDecl(decl) => self.lower_const_decl(decl),
            Statement::Block(block) => self.lower_block(block),
            Statement::Expression(expr_stmt) => {
                let _ = self.lower_expression(&expr_stmt.expression);
            }
            Statement::If(if_stmt) => {
                let cond_val = self.lower_expression(&if_stmt.condition);
                let then_block = self.builder.new_block("if_then".to_string());
                let else_block = self.builder.new_block("if_else".to_string());
                let merge_block = self.builder.new_block("if_merge".to_string());

                self.builder.emit_terminator(
                    TerminatorKind::ConditionalJump {
                        cond: cond_val,
                        then_block,
                        else_block,
                    },
                    if_stmt.span,
                );

                // Then path
                self.builder.enter_block(then_block, "if_then".to_string());
                self.lower_block(&if_stmt.body);
                self.builder
                    .emit_terminator(TerminatorKind::Jump(merge_block), if_stmt.span);

                // Else path (with nested else-ifs mapped recursively)
                self.builder.enter_block(else_block, "if_else".to_string());
                if let Some(ref else_body) = if_stmt.else_body {
                    self.lower_block(else_body);
                }
                self.builder
                    .emit_terminator(TerminatorKind::Jump(merge_block), if_stmt.span);

                // Merge path
                self.builder
                    .enter_block(merge_block, "if_merge".to_string());
            }
            Statement::While(while_stmt) => {
                let cond_block = self.builder.new_block("while_cond".to_string());
                let body_block = self.builder.new_block("while_body".to_string());
                let exit_block = self.builder.new_block("while_exit".to_string());

                self.builder
                    .emit_terminator(TerminatorKind::Jump(cond_block), while_stmt.span);

                // Cond path
                self.builder
                    .enter_block(cond_block, "while_cond".to_string());
                let cond_val = self.lower_expression(&while_stmt.condition);
                self.builder.emit_terminator(
                    TerminatorKind::ConditionalJump {
                        cond: cond_val,
                        then_block: body_block,
                        else_block: exit_block,
                    },
                    while_stmt.span,
                );

                // Body path
                self.break_stack.push(exit_block);
                self.continue_stack.push(cond_block);

                self.builder
                    .enter_block(body_block, "while_body".to_string());
                self.lower_block(&while_stmt.body);
                self.builder
                    .emit_terminator(TerminatorKind::Jump(cond_block), while_stmt.span);

                self.break_stack.pop();
                self.continue_stack.pop();

                // Exit path
                self.builder
                    .enter_block(exit_block, "while_exit".to_string());
            }
            Statement::For(for_stmt) => {
                let _init_block = self.builder.new_block("for_init".to_string());
                let cond_block = self.builder.new_block("for_cond".to_string());
                let body_block = self.builder.new_block("for_body".to_string());
                let inc_block = self.builder.new_block("for_inc".to_string());
                let exit_block = self.builder.new_block("for_exit".to_string());

                // Init path: evaluate iterable, store list and index=0
                let iter_val = self.lower_expression(&for_stmt.iterable);
                let iter_local = self.builder.allocate_local(IRType::Any);
                let idx_local = self.builder.allocate_local(IRType::Any);
                self.builder.emit_effect(
                    Op::Store {
                        target: Value::Local(iter_local),
                        value: iter_val,
                    },
                    for_stmt.span,
                );
                let zero_idx = self.builder.emit_instruction(
                    Op::Constant(LiteralVal::Int(0)),
                    IRType::Int64,
                    for_stmt.span,
                );
                self.builder.emit_effect(
                    Op::Store {
                        target: Value::Local(idx_local),
                        value: Value::Temp(zero_idx),
                    },
                    for_stmt.span,
                );

                self.builder
                    .emit_terminator(TerminatorKind::Jump(cond_block), for_stmt.span);

                // Cond path: load index, load len(list), compare index < len(list)
                self.builder.enter_block(cond_block, "for_cond".to_string());
                let idx_load = self.builder.emit_instruction(
                    Op::Load(Value::Local(idx_local)),
                    IRType::Int64,
                    for_stmt.span,
                );
                let iter_load = self.builder.emit_instruction(
                    Op::Load(Value::Local(iter_local)),
                    IRType::Any,
                    for_stmt.span,
                );
                let len_global = self.builder.declare_global("len".to_string(), IRType::Any);
                let len_call = self.builder.emit_instruction(
                    Op::Call {
                        callee: Value::Global(len_global),
                        args: vec![Value::Temp(iter_load)],
                    },
                    IRType::Int64,
                    for_stmt.span,
                );
                let cond_val = self.builder.emit_instruction(
                    Op::Compare {
                        op: techscript_syntax::TokenKind::Less,
                        left: Value::Temp(idx_load),
                        right: Value::Temp(len_call),
                    },
                    IRType::Bool,
                    for_stmt.span,
                );
                self.builder.emit_terminator(
                    TerminatorKind::ConditionalJump {
                        cond: Value::Temp(cond_val),
                        then_block: body_block,
                        else_block: exit_block,
                    },
                    for_stmt.span,
                );

                // Body path: load list[idx] into loop variable
                self.break_stack.push(exit_block);
                self.continue_stack.push(inc_block);

                self.builder.enter_block(body_block, "for_body".to_string());
                let iter_load_body = self.builder.emit_instruction(
                    Op::Load(Value::Local(iter_local)),
                    IRType::Any,
                    for_stmt.span,
                );
                let idx_load_body = self.builder.emit_instruction(
                    Op::Load(Value::Local(idx_local)),
                    IRType::Int64,
                    for_stmt.span,
                );
                let elem = self.builder.emit_instruction(
                    Op::IndexLoad {
                        base: Value::Temp(iter_load_body),
                        index: Value::Temp(idx_load_body),
                    },
                    IRType::Any,
                    for_stmt.span,
                );
                let loop_param = self.builder.allocate_local(IRType::Any);
                self.symbol_map.insert(
                    for_stmt.item.name.clone(),
                    SymbolBinding::Local(loop_param, IRType::Any),
                );
                self.builder.emit_effect(
                    Op::Store {
                        target: Value::Local(loop_param),
                        value: Value::Temp(elem),
                    },
                    for_stmt.span,
                );

                self.lower_block(&for_stmt.body);
                self.builder
                    .emit_terminator(TerminatorKind::Jump(inc_block), for_stmt.span);

                // Increment path: idx = idx + 1
                self.builder.enter_block(inc_block, "for_inc".to_string());
                let old_idx = self.builder.emit_instruction(
                    Op::Load(Value::Local(idx_local)),
                    IRType::Int64,
                    for_stmt.span,
                );
                let one = self.builder.emit_instruction(
                    Op::Constant(LiteralVal::Int(1)),
                    IRType::Int64,
                    for_stmt.span,
                );
                let new_idx = self.builder.emit_instruction(
                    Op::BinaryOp {
                        op: techscript_syntax::TokenKind::Plus,
                        left: Value::Temp(old_idx),
                        right: Value::Temp(one),
                    },
                    IRType::Int64,
                    for_stmt.span,
                );
                self.builder.emit_effect(
                    Op::Store {
                        target: Value::Local(idx_local),
                        value: Value::Temp(new_idx),
                    },
                    for_stmt.span,
                );
                self.builder
                    .emit_terminator(TerminatorKind::Jump(cond_block), for_stmt.span);

                self.break_stack.pop();
                self.continue_stack.pop();

                // Exit path
                self.builder.enter_block(exit_block, "for_exit".to_string());
            }
            Statement::Repeat(repeat_stmt) => {
                let cond_block = self.builder.new_block("repeat_cond".to_string());
                let body_block = self.builder.new_block("repeat_body".to_string());
                let exit_block = self.builder.new_block("repeat_exit".to_string());

                self.builder
                    .emit_terminator(TerminatorKind::Jump(cond_block), repeat_stmt.span);

                // Cond: evaluate condition each iteration
                self.builder
                    .enter_block(cond_block, "repeat_cond".to_string());
                let cond_val = self.lower_expression(&repeat_stmt.count);
                self.builder.emit_terminator(
                    TerminatorKind::ConditionalJump {
                        cond: cond_val,
                        then_block: body_block,
                        else_block: exit_block,
                    },
                    repeat_stmt.span,
                );

                // Body
                self.break_stack.push(exit_block);
                self.continue_stack.push(cond_block);

                self.builder
                    .enter_block(body_block, "repeat_body".to_string());
                self.lower_block(&repeat_stmt.body);
                self.builder
                    .emit_terminator(TerminatorKind::Jump(cond_block), repeat_stmt.span);

                self.break_stack.pop();
                self.continue_stack.pop();

                self.builder
                    .enter_block(exit_block, "repeat_exit".to_string());
            }
            Statement::Break(break_stmt) => {
                if let Some(target) = self.break_stack.last().cloned() {
                    self.builder
                        .emit_terminator(TerminatorKind::Jump(target), break_stmt.span);
                }
            }
            Statement::Continue(continue_stmt) => {
                if let Some(target) = self.continue_stack.last().cloned() {
                    self.builder
                        .emit_terminator(TerminatorKind::Jump(target), continue_stmt.span);
                }
            }
            Statement::Return(ret_stmt) => {
                let val = ret_stmt
                    .value
                    .as_ref()
                    .map(|expr| self.lower_expression(expr));
                self.builder
                    .emit_terminator(TerminatorKind::Return(val), ret_stmt.span);
            }
            Statement::Throw(throw_stmt) => {
                let val = self.lower_expression(&throw_stmt.value);
                self.builder
                    .emit_terminator(TerminatorKind::Throw(val), throw_stmt.span);
            }
            Statement::Try(try_stmt) => {
                // Lower try blocks as consecutive basic blocks
                let try_block = self.builder.new_block("try_body".to_string());
                let try_next = self.builder.new_block("try_next".to_string());
                let catch_block = self.builder.new_block("catch_body".to_string());
                let merge_block = self.builder.new_block("try_merge".to_string());

                let catch_var = self.builder.allocate_local(IRType::String);
                self.symbol_map.insert(
                    try_stmt.catch_var.name.clone(),
                    SymbolBinding::Local(catch_var, IRType::String),
                );

                self.builder.emit_effect(
                    Op::Try {
                        catch_block,
                        catch_var,
                    },
                    try_stmt.span,
                );

                self.builder
                    .emit_terminator(TerminatorKind::Jump(try_block), try_stmt.span);

                // Try Body
                self.builder.enter_block(try_block, "try_body".to_string());
                self.lower_block(&try_stmt.body);
                if !self.builder.has_terminator() {
                    self.builder
                        .emit_terminator(TerminatorKind::Jump(try_next), try_stmt.span);
                }

                // Try next (normal completion path — runs EndTry then jumps to merge)
                self.builder.enter_block(try_next, "try_next".to_string());
                self.builder.emit_effect(Op::EndTry, try_stmt.span);
                self.builder
                    .emit_terminator(TerminatorKind::Jump(merge_block), try_stmt.span);

                // Catch Body
                self.builder
                    .enter_block(catch_block, "catch_body".to_string());
                self.lower_block(&try_stmt.catch_body);
                self.builder
                    .emit_terminator(TerminatorKind::Jump(merge_block), try_stmt.span);

                // Merge
                self.builder
                    .enter_block(merge_block, "try_merge".to_string());
            }
            Statement::Say(say_stmt) => {
                let val = self.lower_expression(&say_stmt.value);
                self.builder.emit_effect(
                    Op::Call {
                        callee: Value::Global(crate::types::GlobalId(999)), // native "say" global slot
                        args: vec![val],
                    },
                    say_stmt.span,
                );
            }
            Statement::FuncDecl(decl) => {
                self.lower_func_decl(decl);
            }
            Statement::StructDecl(decl) => {
                self.lower_struct_decl(decl);
            }
            Statement::ModelDecl(decl) => {
                self.lower_model_decl(decl);
            }
            Statement::ExportDecl(decl) => {
                self.lower_statement(&decl.declaration);
            }
            Statement::Import(import_stmt) => {
                let path_str = import_stmt
                    .path
                    .iter()
                    .map(|ident| ident.name.clone())
                    .collect::<Vec<_>>()
                    .join("/");
                self.builder.declare_import(path_str);

                if let Some(symbols) = &import_stmt.symbols {
                    if import_stmt.path.len() > 1
                        && symbols.len() == 1
                        && !symbols[0].name.contains(':')
                        && symbols[0].name != "*"
                    {
                        let alias_name = symbols[0].name.clone();
                        // For namespace alias, the global variable representing it is the root namespace of the path
                        let root_name = import_stmt.path[0].name.clone();
                        let global_id = self.builder.declare_global(root_name, IRType::Any);
                        self.symbol_map
                            .insert(alias_name, SymbolBinding::Global(global_id, IRType::Any));
                    } else {
                        for sym in symbols {
                            if sym.name == "*" {
                                let module_path = import_stmt
                                    .path
                                    .iter()
                                    .map(|i| i.name.clone())
                                    .collect::<Vec<_>>()
                                    .join(".");
                                let registry = techscript_stdlib::StdlibRegistry::new();
                                if let Some(module) = registry.get_module(&module_path) {
                                    for func_name in module.exports.keys() {
                                        let global_id = self
                                            .builder
                                            .declare_global(func_name.clone(), IRType::Any);
                                        self.symbol_map.insert(
                                            func_name.clone(),
                                            SymbolBinding::Global(global_id, IRType::Any),
                                        );
                                    }
                                }
                            } else if sym.name.contains(':') {
                                let parts: Vec<&str> = sym.name.split(':').collect();
                                let orig_name = parts[0].to_string();
                                let alias_name = parts[1].to_string();
                                let global_id = self.builder.declare_global(orig_name, IRType::Any);
                                self.symbol_map.insert(
                                    alias_name,
                                    SymbolBinding::Global(global_id, IRType::Any),
                                );
                            } else {
                                let global_id =
                                    self.builder.declare_global(sym.name.clone(), IRType::Any);
                                self.symbol_map.insert(
                                    sym.name.clone(),
                                    SymbolBinding::Global(global_id, IRType::Any),
                                );
                            }
                        }
                    }
                } else if !import_stmt.path.is_empty() {
                    let root_name = import_stmt.path[0].name.clone();
                    let global_id = self.builder.declare_global(root_name.clone(), IRType::Any);
                    self.symbol_map.insert(
                        root_name.clone(),
                        SymbolBinding::Global(global_id, IRType::Any),
                    );
                }
            }
            Statement::EnumDecl(decl) => {
                // Declare enum type globally
                let global_id = self
                    .builder
                    .declare_global(decl.name.name.clone(), IRType::Enum(decl.name.name.clone()));
                self.symbol_map.insert(
                    decl.name.name.clone(),
                    SymbolBinding::Global(global_id, IRType::Enum(decl.name.name.clone())),
                );
            }
            Statement::DSL(block) => {
                // Lower DSL blocks to structured IR with properties, children, and args.
                // Emit a MakeDslBlock instruction and store the DSL block in the module.
                let dsl_id = self.lower_dsl_block(block);
                let _ = self
                    .builder
                    .declare_global(block.kind.clone(), IRType::DslBlock(block.kind.clone()));
            }
        }
    }

    fn lower_var_decl(&mut self, decl: &VarDecl) {
        let ty = self.map_type_spec(&decl.type_ann);
        let init_val = self.lower_expression(&decl.initializer);
        self.bind_pattern(&decl.pattern, init_val, ty, decl.span);
    }

    fn lower_const_decl(&mut self, decl: &techscript_ast::ConstDecl) {
        let ty = self.map_type_spec(&decl.type_ann);
        let init_val = self.lower_expression(&decl.initializer);
        self.bind_pattern(&decl.pattern, init_val, ty, decl.span);
    }

    fn bind_pattern(&mut self, pattern: &Pattern, val: Value, ty: IRType, span: Span) {
        match pattern {
            Pattern::Single(ident) => {
                let local_id = self.builder.allocate_local(ty.clone());
                self.builder.emit_effect(
                    Op::Store {
                        target: Value::Local(local_id),
                        value: val,
                    },
                    span,
                );
                self.symbol_map
                    .insert(ident.name.clone(), SymbolBinding::Local(local_id, ty));
            }
            Pattern::Tuple(idents) | Pattern::List(idents) => {
                for (idx, ident) in idents.iter().enumerate() {
                    let el_val = self.builder.emit_instruction(
                        Op::IndexLoad {
                            base: val.clone(),
                            index: Value::Const(LiteralVal::Int(idx as i64)),
                        },
                        IRType::Any,
                        span,
                    );
                    let local_id = self.builder.allocate_local(IRType::Any);
                    self.builder.emit_effect(
                        Op::Store {
                            target: Value::Local(local_id),
                            value: Value::Temp(el_val),
                        },
                        span,
                    );
                    self.symbol_map.insert(
                        ident.name.clone(),
                        SymbolBinding::Local(local_id, IRType::Any),
                    );
                }
            }
            Pattern::Struct(idents) => {
                for ident in idents {
                    let field_val = self.builder.emit_instruction(
                        Op::FieldLoad {
                            base: val.clone(),
                            field: ident.name.clone(),
                        },
                        IRType::Any,
                        span,
                    );
                    let local_id = self.builder.allocate_local(IRType::Any);
                    self.builder.emit_effect(
                        Op::Store {
                            target: Value::Local(local_id),
                            value: Value::Temp(field_val),
                        },
                        span,
                    );
                    self.symbol_map.insert(
                        ident.name.clone(),
                        SymbolBinding::Local(local_id, IRType::Any),
                    );
                }
            }
        }
    }

    fn lower_block(&mut self, block: &Block) {
        for stmt in &block.statements {
            self.lower_statement(stmt);
        }
    }

    fn lower_func_decl(&mut self, decl: &FuncDecl) {
        let ret_ty = self.map_type_spec(&decl.return_type);
        self.builder.start_function(decl.name.name.clone(), ret_ty);

        // Lower parameters
        for param in &decl.params {
            let ty = self.map_type_spec(&param.type_ann);
            let local_id = self
                .builder
                .add_parameter(param.name.name.clone(), ty.clone());
            self.symbol_map
                .insert(param.name.name.clone(), SymbolBinding::Local(local_id, ty));
        }

        let entry = self.builder.new_block("func_entry".to_string());
        self.builder.enter_block(entry, "func_entry".to_string());

        self.lower_block(&decl.body);

        self.builder
            .emit_terminator(TerminatorKind::Return(None), decl.span);
        self.builder.seal_function();
    }

    fn lower_struct_decl(&mut self, decl: &StructDecl) {
        let _fields = decl
            .fields
            .iter()
            .map(|f| (f.name.name.clone(), IRType::Any))
            .collect::<Vec<_>>();
        let global_id = self.builder.declare_global(
            decl.name.name.clone(),
            IRType::Struct(decl.name.name.clone()),
        );
        self.symbol_map.insert(
            decl.name.name.clone(),
            SymbolBinding::Global(global_id, IRType::Struct(decl.name.name.clone())),
        );
    }

    fn lower_model_decl(&mut self, decl: &ModelDecl) {
        let global_id = self.builder.declare_global(
            decl.name.name.clone(),
            IRType::Model(decl.name.name.clone()),
        );
        self.symbol_map.insert(
            decl.name.name.clone(),
            SymbolBinding::Global(global_id, IRType::Model(decl.name.name.clone())),
        );
    }

    /// Lower a DSL block recursively, returning its DSL block ID.
    fn lower_dsl_block(&mut self, block: &DSLBlock) -> DslBlockId {
        let id = self.builder.next_dsl_block_id();

        // Lower args: try to extract as LiteralVal constants
        let mut lowered_args: Vec<LiteralVal> = Vec::new();
        for expr in &block.args {
            if let Expression::Literal(lit) = expr {
                lowered_args.push(lit.value.clone());
            } else {
                lowered_args.push(LiteralVal::Str(self.expr_to_debug_string(expr)));
            }
        }

        // Lower properties: name -> (name, optional literal value)
        let mut lowered_props: Vec<(String, Option<LiteralVal>)> = Vec::new();
        for prop in &block.properties {
            let val = prop.value.as_ref().and_then(|expr| {
                if let Expression::Literal(lit) = expr {
                    Some(lit.value.clone())
                } else {
                    Some(LiteralVal::Str(self.expr_to_debug_string(expr)))
                }
            });
            lowered_props.push((prop.name.clone(), val));
        }

        // Lower children recursively, building a mapping of child_id -> child_kind
        let mut lowered_children: Vec<(DslBlockId, String)> = Vec::new();
        for child in &block.children {
            match child {
                DSLChild::Block(sub_block) => {
                    let child_id = self.lower_dsl_block(sub_block);
                    lowered_children.push((child_id, sub_block.kind.clone()));
                }
                DSLChild::Code(code_block) => {
                    // Lower code blocks inline
                    for stmt in &code_block.statements {
                        self.lower_statement(stmt);
                    }
                }
                DSLChild::Property(prop) => {
                    let val = prop.value.as_ref().and_then(|expr| {
                        if let Expression::Literal(lit) = expr {
                            Some(lit.value.clone())
                        } else {
                            Some(LiteralVal::Str(self.expr_to_debug_string(expr)))
                        }
                    });
                    lowered_props.push((prop.name.clone(), val));
                }
            }
        }

        // Emit the DSL block instruction
        let span_start = block.span.start;
        let span_end = block.span.end;

        let dsl_ir = DslBlockIR {
            id,
            kind: block.kind.clone(),
            args: lowered_args,
            properties: lowered_props,
            children: lowered_children,
            span: (span_start as u32, span_end as u32),
        };
        self.builder.declare_dsl_block(dsl_ir);

        id
    }
    /// Convert an expression to a debug string for IR fallback.
    fn expr_to_debug_string(&self, expr: &Expression) -> String {
        match expr {
            Expression::Literal(lit) => literal_val_to_string(&lit.value),
            Expression::Identifier(ident) => ident.name.clone(),
            _ => "<expr>".to_string(),
        }
    }

    fn lower_std_call(
        &mut self,
        module: &str,
        method: &str,
        args: Vec<Value>,
        span: Span,
    ) -> Value {
        let std_ty = IRType::Any;
        let std_global_id = self
            .builder
            .declare_global("std".to_string(), std_ty.clone());
        let std_val = Value::Temp(self.builder.emit_instruction(
            Op::Load(Value::Global(std_global_id)),
            std_ty,
            span,
        ));
        let mod_val = Value::Temp(self.builder.emit_instruction(
            Op::FieldLoad {
                base: std_val,
                field: module.to_string(),
            },
            IRType::Any,
            span,
        ));
        let method_val = Value::Temp(self.builder.emit_instruction(
            Op::FieldLoad {
                base: mod_val,
                field: method.to_string(),
            },
            IRType::Any,
            span,
        ));
        let temp = self.builder.emit_instruction(
            Op::Call {
                callee: method_val,
                args,
            },
            IRType::Any,
            span,
        );
        Value::Temp(temp)
    }

    fn lower_expression(&mut self, expr: &Expression) -> Value {
        match expr {
            Expression::Literal(lit) => match &lit.value {
                LiteralVal::None => Value::Null,
                other => Value::Const(other.clone()),
            },
            Expression::Identifier(ident) => {
                if let Some(binding) = self.symbol_map.get(&ident.name) {
                    match binding {
                        SymbolBinding::Local(local_id, ty) => {
                            let temp = self.builder.emit_instruction(
                                Op::Load(Value::Local(*local_id)),
                                ty.clone(),
                                ident.span,
                            );
                            Value::Temp(temp)
                        }
                        SymbolBinding::Global(global_id, ty) => {
                            let temp = self.builder.emit_instruction(
                                Op::Load(Value::Global(*global_id)),
                                ty.clone(),
                                ident.span,
                            );
                            Value::Temp(temp)
                        }
                    }
                } else {
                    let ty = IRType::Any;
                    let global_id = self.builder.declare_global(ident.name.clone(), ty.clone());
                    self.symbol_map.insert(
                        ident.name.clone(),
                        SymbolBinding::Global(global_id, ty.clone()),
                    );
                    let temp = self.builder.emit_instruction(
                        Op::Load(Value::Global(global_id)),
                        ty,
                        ident.span,
                    );
                    Value::Temp(temp)
                }
            }
            Expression::Binary(bin) => self.lower_binary(bin),
            Expression::Unary(un) => self.lower_unary(un),
            Expression::Call(call) => {
                if let Expression::Identifier(ref ident) = *call.callee {
                    if ident.name == "env" {
                        let arg_val = self.lower_expression(&call.args[0]);
                        return self.lower_std_call("env", "get", vec![arg_val], call.span);
                    } else if ident.name == "file" {
                        let arg_val = self.lower_expression(&call.args[0]);
                        return self.lower_std_call("file", "read", vec![arg_val], call.span);
                    } else if ident.name == "json" {
                        let file_arg = self.lower_expression(&call.args[0]);
                        let read_val =
                            self.lower_std_call("file", "read", vec![file_arg], call.span);
                        return self.lower_std_call("json", "parse", vec![read_val], call.span);
                    }
                }
                let callee_val = self.lower_expression(&call.callee);
                let mut args = Vec::new();
                for arg in &call.args {
                    args.push(self.lower_expression(arg));
                }
                let temp = self.builder.emit_instruction(
                    Op::Call {
                        callee: callee_val,
                        args,
                    },
                    IRType::Any,
                    call.span,
                );
                Value::Temp(temp)
            }
            Expression::Assignment(assign) => self.lower_assignment(assign),
            Expression::Group(inner) => self.lower_expression(inner),
            Expression::List(list) => {
                let mut elements = Vec::new();
                for item in &list.items {
                    elements.push(self.lower_expression(item));
                }
                let temp =
                    self.builder
                        .emit_instruction(Op::MakeList(elements), IRType::List, list.span);
                Value::Temp(temp)
            }
            Expression::Map(map) => {
                let mut entries = Vec::new();
                for (k, v) in &map.entries {
                    entries.push((self.lower_expression(k), self.lower_expression(v)));
                }
                let temp =
                    self.builder
                        .emit_instruction(Op::MakeMap(entries), IRType::Map, map.span);
                Value::Temp(temp)
            }
            Expression::Member(mem) => {
                let base = self.lower_expression(&mem.object);
                let temp = self.builder.emit_instruction(
                    Op::FieldLoad {
                        base,
                        field: mem.member.name.clone(),
                    },
                    IRType::Any,
                    mem.span,
                );
                Value::Temp(temp)
            }
            Expression::Index(idx) => {
                let base = self.lower_expression(&idx.object);
                let index = self.lower_expression(&idx.index);
                let temp = self.builder.emit_instruction(
                    Op::IndexLoad { base, index },
                    IRType::Any,
                    idx.span,
                );
                Value::Temp(temp)
            }
            Expression::New(new_expr) => {
                let mut args = Vec::new();
                for arg in &new_expr.args {
                    args.push(self.lower_expression(arg));
                }
                let callee = if let Some(SymbolBinding::Global(global_id, _)) =
                    self.symbol_map.get(&new_expr.class_name.name)
                {
                    Value::Global(*global_id)
                } else {
                    Value::Global(crate::types::GlobalId(1))
                };
                let temp = self.builder.emit_instruction(
                    Op::Call { callee, args },
                    IRType::Model(new_expr.class_name.name.clone()),
                    new_expr.span,
                );
                Value::Temp(temp)
            }
            Expression::Range(range) => {
                let start = self.lower_expression(&range.start);
                let end = self.lower_expression(&range.end);
                let temp = self.builder.emit_instruction(
                    Op::Call {
                        callee: Value::Global(crate::types::GlobalId(998)), // native "range" global slot
                        args: vec![start, end],
                    },
                    IRType::List,
                    range.span,
                );
                Value::Temp(temp)
            }
            Expression::Ask(ask) => {
                let prompt = self.lower_expression(&ask.prompt);
                let temp = self.builder.emit_instruction(
                    Op::Call {
                        callee: Value::Global(crate::types::GlobalId(997)), // native "ask" global slot
                        args: vec![prompt],
                    },
                    IRType::String,
                    ask.span,
                );
                Value::Temp(temp)
            }
            Expression::Lambda(lambda) => {
                // Lower lambda as compiler generated function declaration
                let func_name = format!("lambda_{}", lambda.id.0);
                self.builder.start_function(func_name, IRType::Any);
                let entry = self.builder.new_block("lambda_entry".to_string());
                self.builder.enter_block(entry, "lambda_entry".to_string());
                self.lower_block(&lambda.body);
                self.builder
                    .emit_terminator(TerminatorKind::Return(None), lambda.span);
                self.builder.seal_function();
                Value::Null
            }
            Expression::FString(fstr) => {
                let mut parts = Vec::new();
                for part in &fstr.parts {
                    match part {
                        techscript_ast::FStringPart::Literal(s) => {
                            parts.push(Value::Const(LiteralVal::Str(s.clone())));
                        }
                        techscript_ast::FStringPart::Expr(expr) => {
                            parts.push(self.lower_expression(expr));
                        }
                    }
                }
                let temp = self.builder.emit_instruction(
                    Op::Call {
                        callee: Value::Global(crate::types::GlobalId(996)), // native "fstring_concat" global slot
                        args: parts,
                    },
                    IRType::String,
                    fstr.span,
                );
                Value::Temp(temp)
            }
        }
    }

    fn lower_unary(&mut self, un: &UnaryExpr) -> Value {
        let val = self.lower_expression(&un.right);
        let temp = self.builder.emit_instruction(
            Op::UnaryOp {
                op: lookup_operator(&un.op),
                right: val,
            },
            IRType::Any,
            un.span,
        );
        Value::Temp(temp)
    }

    fn lower_binary(&mut self, bin: &BinaryExpr) -> Value {
        // Optional Chaining Short-Circuit Lowering
        if bin.op == "?." {
            let left_val = self.lower_expression(&bin.left);
            let not_null_block = self.builder.new_block("opt_not_null".to_string());
            let null_block = self.builder.new_block("opt_null".to_string());
            let merge_block = self.builder.new_block("opt_merge".to_string());

            self.builder.emit_terminator(
                TerminatorKind::ConditionalJump {
                    cond: left_val.clone(),
                    then_block: not_null_block,
                    else_block: null_block,
                },
                bin.span,
            );

            // Not Null
            self.builder
                .enter_block(not_null_block, "opt_not_null".to_string());
            let rhs_val = self.lower_expression(&bin.right);
            let res_var = self.builder.allocate_local(IRType::Any);
            self.builder.emit_effect(
                Op::Store {
                    target: Value::Local(res_var),
                    value: rhs_val,
                },
                bin.span,
            );
            self.builder
                .emit_terminator(TerminatorKind::Jump(merge_block), bin.span);

            // Null
            self.builder.enter_block(null_block, "opt_null".to_string());
            self.builder.emit_effect(
                Op::Store {
                    target: Value::Local(res_var),
                    value: Value::Null,
                },
                bin.span,
            );
            self.builder
                .emit_terminator(TerminatorKind::Jump(merge_block), bin.span);

            // Merge
            self.builder
                .enter_block(merge_block, "opt_merge".to_string());
            let load_res = self.builder.emit_instruction(
                Op::Load(Value::Local(res_var)),
                IRType::Any,
                bin.span,
            );
            return Value::Temp(load_res);
        }

        // Null Coalescing Short-Circuit Lowering
        if bin.op == "??" {
            let left_val = self.lower_expression(&bin.left);
            let null_block = self.builder.new_block("coal_null".to_string());
            let not_null_block = self.builder.new_block("coal_not_null".to_string());
            let merge_block = self.builder.new_block("coal_merge".to_string());

            self.builder.emit_terminator(
                TerminatorKind::ConditionalJump {
                    cond: left_val.clone(),
                    then_block: not_null_block,
                    else_block: null_block,
                },
                bin.span,
            );

            // Not Null
            self.builder
                .enter_block(not_null_block, "coal_not_null".to_string());
            let res_var = self.builder.allocate_local(IRType::Any);
            self.builder.emit_effect(
                Op::Store {
                    target: Value::Local(res_var),
                    value: left_val,
                },
                bin.span,
            );
            self.builder
                .emit_terminator(TerminatorKind::Jump(merge_block), bin.span);

            // Null
            self.builder
                .enter_block(null_block, "coal_null".to_string());
            let rhs_val = self.lower_expression(&bin.right);
            self.builder.emit_effect(
                Op::Store {
                    target: Value::Local(res_var),
                    value: rhs_val,
                },
                bin.span,
            );
            self.builder
                .emit_terminator(TerminatorKind::Jump(merge_block), bin.span);

            // Merge
            self.builder
                .enter_block(merge_block, "coal_merge".to_string());
            let load_res = self.builder.emit_instruction(
                Op::Load(Value::Local(res_var)),
                IRType::Any,
                bin.span,
            );
            return Value::Temp(load_res);
        }

        // Standard Binary Operations
        let left = self.lower_expression(&bin.left);
        let right = self.lower_expression(&bin.right);

        if bin.op == "=="
            || bin.op == "!="
            || bin.op == "==="
            || bin.op == "<"
            || bin.op == "<="
            || bin.op == ">"
            || bin.op == ">="
        {
            let temp = self.builder.emit_instruction(
                Op::Compare {
                    op: lookup_operator(&bin.op),
                    left,
                    right,
                },
                IRType::Bool,
                bin.span,
            );
            Value::Temp(temp)
        } else {
            let temp = self.builder.emit_instruction(
                Op::BinaryOp {
                    op: lookup_operator(&bin.op),
                    left,
                    right,
                },
                IRType::Any,
                bin.span,
            );
            Value::Temp(temp)
        }
    }

    fn lower_assignment(&mut self, assign: &AssignmentExpr) -> Value {
        let value = self.lower_expression(&assign.value);

        match &*assign.target {
            Expression::Identifier(ident) => {
                let binding = if let Some(binding) = self.symbol_map.get(&ident.name) {
                    binding.clone()
                } else {
                    if self.builder.current_function.is_some() {
                        let local_id = self.builder.allocate_local(IRType::Any);
                        let b = SymbolBinding::Local(local_id, IRType::Any);
                        self.symbol_map.insert(ident.name.clone(), b.clone());
                        b
                    } else {
                        let global_id =
                            self.builder.declare_global(ident.name.clone(), IRType::Any);
                        let b = SymbolBinding::Global(global_id, IRType::Any);
                        self.symbol_map.insert(ident.name.clone(), b.clone());
                        b
                    }
                };

                match binding {
                    SymbolBinding::Local(local_id, _) => {
                        self.builder.emit_effect(
                            Op::Store {
                                target: Value::Local(local_id),
                                value: value.clone(),
                            },
                            assign.span,
                        );
                    }
                    SymbolBinding::Global(global_id, _) => {
                        self.builder.emit_effect(
                            Op::Store {
                                target: Value::Global(global_id),
                                value: value.clone(),
                            },
                            assign.span,
                        );
                    }
                }
            }
            Expression::Member(mem) => {
                let base = self.lower_expression(&mem.object);
                self.builder.emit_effect(
                    Op::FieldStore {
                        base,
                        field: mem.member.name.clone(),
                        value: value.clone(),
                    },
                    assign.span,
                );
            }
            Expression::Index(idx) => {
                let base = self.lower_expression(&idx.object);
                let index = self.lower_expression(&idx.index);
                self.builder.emit_effect(
                    Op::IndexStore {
                        base,
                        index,
                        value: value.clone(),
                    },
                    assign.span,
                );
            }
            _ => {}
        }

        value
    }
}

fn lookup_operator(op: &str) -> techscript_syntax::TokenKind {
    match op {
        "+" => techscript_syntax::TokenKind::Plus,
        "-" => techscript_syntax::TokenKind::Minus,
        "*" => techscript_syntax::TokenKind::Star,
        "/" => techscript_syntax::TokenKind::Slash,
        "%" => techscript_syntax::TokenKind::Percent,
        "<" => techscript_syntax::TokenKind::Less,
        "<=" => techscript_syntax::TokenKind::LessEqual,
        ">" => techscript_syntax::TokenKind::Greater,
        ">=" => techscript_syntax::TokenKind::GreaterEqual,
        "==" => techscript_syntax::TokenKind::EqualEqual,
        "!=" => techscript_syntax::TokenKind::BangEqual,
        "===" => techscript_syntax::TokenKind::TripleEqual,
        "&&" | "and" => techscript_syntax::TokenKind::And,
        "||" | "or" => techscript_syntax::TokenKind::Or,
        "!" | "not" => techscript_syntax::TokenKind::Not,
        _ => techscript_syntax::lookup_keyword(op).unwrap_or(techscript_syntax::TokenKind::Plus),
    }
}

fn literal_val_to_string(lv: &LiteralVal) -> String {
    match lv {
        LiteralVal::Str(s) => s.clone(),
        LiteralVal::Int(i) => i.to_string(),
        LiteralVal::Float(f) => f.to_string(),
        LiteralVal::Bool(b) => b.to_string(),
        LiteralVal::None => "none".to_string(),
    }
}
