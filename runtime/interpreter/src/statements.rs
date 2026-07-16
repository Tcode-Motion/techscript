use crate::control_flow::{ExecResult, FlowSignal};
use crate::interpreter::Interpreter;
use crate::visitor::AstVisitor;
use indexmap::IndexMap;
use std::cell::RefCell;
use std::rc::Rc;
use techscript_ast::{Block, ConstDecl, ModelDecl, Pattern, Statement, VarDecl};
use techscript_runtime::{
    Callable, Environment, ModelInstance, RuntimeError, RuntimeErrorKind, RuntimeValue,
    StructInstance,
};

impl Interpreter {
    /// Dispatches and executes an AST statement.
    pub fn execute_statement(&mut self, stmt: &Statement) -> ExecResult {
        match stmt {
            Statement::Block(block) => self.execute_block(block),
            Statement::VarDecl(decl) => self.execute_var_decl(decl),
            Statement::ConstDecl(decl) => self.execute_const_decl(decl),
            Statement::Expression(expr_stmt) => {
                self.visit_expression(&expr_stmt.expression)?;
                Ok(FlowSignal::Normal)
            }
            Statement::If(if_stmt) => {
                let cond_val = self.visit_expression(&if_stmt.condition)?;
                if cond_val.is_truthy() {
                    return self.execute_block(&if_stmt.body);
                }
                for (else_if_cond, else_if_body) in &if_stmt.else_ifs {
                    let elif_val = self.visit_expression(else_if_cond)?;
                    if elif_val.is_truthy() {
                        return self.execute_block(else_if_body);
                    }
                }
                if let Some(ref else_body) = if_stmt.else_body {
                    return self.execute_block(else_body);
                }
                Ok(FlowSignal::Normal)
            }
            Statement::While(while_stmt) => {
                loop {
                    let cond_val = self.visit_expression(&while_stmt.condition)?;
                    if !cond_val.is_truthy() {
                        break;
                    }
                    let signal = self.execute_block(&while_stmt.body)?;
                    match signal {
                        FlowSignal::Break => break,
                        FlowSignal::Continue => continue,
                        FlowSignal::Return(val) => return Ok(FlowSignal::Return(val)),
                        FlowSignal::Throw(err) => return Ok(FlowSignal::Throw(err)),
                        FlowSignal::Normal => {}
                    }
                }
                Ok(FlowSignal::Normal)
            }
            Statement::For(for_stmt) => {
                let iter_val = self.visit_expression(&for_stmt.iterable)?;
                let items = match iter_val {
                    RuntimeValue::List { items, .. } => items.borrow().clone(),
                    RuntimeValue::Tuple(elements) => elements.clone(),
                    other => {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::TypeMismatch {
                                expected: "iterable collection".to_string(),
                                found: other.runtime_type().to_string(),
                            },
                            Some(for_stmt.span),
                            None,
                        ))
                    }
                };

                for item in items {
                    let loop_env =
                        Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&self.env)))));
                    loop_env
                        .borrow_mut()
                        .define(for_stmt.item.name.clone(), item, false);

                    let signal =
                        self.with_scope(loop_env, |interp| interp.execute_block(&for_stmt.body))?;
                    match signal {
                        FlowSignal::Break => break,
                        FlowSignal::Continue => continue,
                        FlowSignal::Return(val) => return Ok(FlowSignal::Return(val)),
                        FlowSignal::Throw(err) => return Ok(FlowSignal::Throw(err)),
                        FlowSignal::Normal => {}
                    }
                }
                Ok(FlowSignal::Normal)
            }
            Statement::Repeat(repeat_stmt) => {
                let count_val = self.visit_expression(&repeat_stmt.count)?;
                let count = count_val.try_into_int()?;
                for _ in 0..count {
                    let signal = self.execute_block(&repeat_stmt.body)?;
                    match signal {
                        FlowSignal::Break => break,
                        FlowSignal::Continue => continue,
                        FlowSignal::Return(val) => return Ok(FlowSignal::Return(val)),
                        FlowSignal::Throw(err) => return Ok(FlowSignal::Throw(err)),
                        FlowSignal::Normal => {}
                    }
                }
                Ok(FlowSignal::Normal)
            }
            Statement::Return(ret_stmt) => {
                let val = if let Some(ref val_expr) = ret_stmt.value {
                    self.visit_expression(val_expr)?
                } else {
                    RuntimeValue::Null
                };
                Ok(FlowSignal::Return(val))
            }
            Statement::Break(_) => Ok(FlowSignal::Break),
            Statement::Continue(_) => Ok(FlowSignal::Continue),
            Statement::Throw(throw_stmt) => {
                let val = self.visit_expression(&throw_stmt.value)?;
                let err = RuntimeError::new(
                    RuntimeErrorKind::UserError(val.to_string()),
                    Some(throw_stmt.span),
                    None,
                );
                Ok(FlowSignal::Throw(err))
            }
            Statement::Try(try_stmt) => {
                let signal = self.execute_block(&try_stmt.body)?;
                if let FlowSignal::Throw(err) = signal {
                    // Catch the exception
                    let catch_env =
                        Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&self.env)))));
                    catch_env.borrow_mut().define(
                        try_stmt.catch_var.name.clone(),
                        RuntimeValue::Str(err.message.clone()),
                        false,
                    );
                    self.with_scope(catch_env, |interp| {
                        interp.execute_block(&try_stmt.catch_body)
                    })
                } else {
                    Ok(signal)
                }
            }
            Statement::Say(say_stmt) => {
                let val = self.visit_expression(&say_stmt.value)?;
                let say_func = self.ctx.registry.lookup("say").unwrap();
                say_func.call(&mut self.ctx, vec![val])?;
                Ok(FlowSignal::Normal)
            }
            Statement::FuncDecl(decl) => {
                let mut params = Vec::new();
                for param in &decl.params {
                    params.push(param.name.name.clone());
                }
                let name = decl.name.name.clone();
                let user_func = techscript_runtime::UserFunction {
                    name: name.clone(),
                    params,
                    body: techscript_runtime::FunctionBody::Ast(decl.body.clone()),
                    closure: Rc::clone(&self.env),
                };
                let callable = Rc::new(self.bridge_user_function(user_func));
                self.env
                    .borrow_mut()
                    .define(name, RuntimeValue::Function(callable), false);
                Ok(FlowSignal::Normal)
            }
            Statement::StructDecl(decl) => {
                // Register a struct constructor callable
                let name = decl.name.name.clone();
                let fields_template = decl.fields.clone();
                let struct_ctor = StructConstructor {
                    name: name.clone(),
                    fields: fields_template,
                };
                self.env.borrow_mut().define(
                    name,
                    RuntimeValue::Function(Rc::new(struct_ctor)),
                    false,
                );
                Ok(FlowSignal::Normal)
            }
            Statement::EnumDecl(decl) => {
                // Register variants as constructors or constant values
                let _name = decl.name.name.clone();
                for variant in &decl.variants {
                    let _variant_name = variant.name.name.clone();
                    // In TechScript 2.0 enums are accessed via .Variant, resolved dynamically
                    // We can register variant constructors under enum name namespaces if needed
                }
                Ok(FlowSignal::Normal)
            }
            Statement::ModelDecl(decl) => {
                let name = decl.name.name.clone();
                let model_ctor = ModelConstructor {
                    name: name.clone(),
                    decl: decl.clone(),
                };
                self.env.borrow_mut().define(
                    name,
                    RuntimeValue::Function(Rc::new(model_ctor)),
                    false,
                );
                Ok(FlowSignal::Normal)
            }
            Statement::ExportDecl(decl) => self.execute_statement(&decl.declaration),
            Statement::Import(_) => {
                // Module imports stubbed for runtime execution
                Ok(FlowSignal::Normal)
            }
        }
    }

    pub fn execute_block(&mut self, block: &Block) -> ExecResult {
        let block_env = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&self.env)))));
        self.with_scope(block_env, |interp| {
            for stmt in &block.statements {
                let signal = interp.execute_statement(stmt)?;
                match signal {
                    FlowSignal::Normal => {}
                    other => return Ok(other),
                }
            }
            Ok(FlowSignal::Normal)
        })
    }

    fn execute_var_decl(&mut self, decl: &VarDecl) -> ExecResult {
        let val = self.visit_expression(&decl.initializer)?;
        self.define_pattern(&decl.pattern, val, false)?;
        Ok(FlowSignal::Normal)
    }

    fn execute_const_decl(&mut self, decl: &ConstDecl) -> ExecResult {
        let val = self.visit_expression(&decl.initializer)?;
        self.define_pattern(&decl.pattern, val, true)?;
        Ok(FlowSignal::Normal)
    }

    fn define_pattern(
        &mut self,
        pattern: &Pattern,
        val: RuntimeValue,
        is_const: bool,
    ) -> Result<(), RuntimeError> {
        match pattern {
            Pattern::Single(ident) => {
                self.env
                    .borrow_mut()
                    .define(ident.name.clone(), val, is_const);
                Ok(())
            }
            Pattern::Tuple(idents) | Pattern::List(idents) => {
                // Unpack tuple/list
                let elements = match val {
                    RuntimeValue::Tuple(el) => el,
                    RuntimeValue::List { items, .. } => items.borrow().clone(),
                    _ => {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::TypeMismatch {
                                expected: "unpackable collection".to_string(),
                                found: val.runtime_type().to_string(),
                            },
                            None,
                            None,
                        ))
                    }
                };
                if idents.len() != elements.len() {
                    return Err(RuntimeError::new(
                        RuntimeErrorKind::ArityMismatch {
                            expected: idents.len(),
                            found: elements.len(),
                        },
                        None,
                        None,
                    ));
                }
                for (ident, el_val) in idents.iter().zip(elements) {
                    self.env
                        .borrow_mut()
                        .define(ident.name.clone(), el_val, is_const);
                }
                Ok(())
            }
            Pattern::Struct(idents) => {
                match val {
                    RuntimeValue::StructInstance(inst) => {
                        for ident in idents {
                            let field_val = inst
                                .borrow()
                                .fields
                                .get(&ident.name)
                                .cloned()
                                .unwrap_or(RuntimeValue::Null);
                            self.env
                                .borrow_mut()
                                .define(ident.name.clone(), field_val, is_const);
                        }
                    }
                    RuntimeValue::Map { entries, .. } => {
                        for ident in idents {
                            let field_val = entries
                                .borrow()
                                .get(&ident.name)
                                .cloned()
                                .unwrap_or(RuntimeValue::Null);
                            self.env
                                .borrow_mut()
                                .define(ident.name.clone(), field_val, is_const);
                        }
                    }
                    _ => {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::TypeMismatch {
                                expected: "struct or map".to_string(),
                                found: val.runtime_type().to_string(),
                            },
                            None,
                            None,
                        ))
                    }
                }
                Ok(())
            }
        }
    }
}

// Struct constructor Callable
struct StructConstructor {
    name: String,
    fields: Vec<techscript_ast::FieldSpec>,
}

impl Callable for StructConstructor {
    fn name(&self) -> &str {
        &self.name
    }

    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _ctx: &mut techscript_runtime::RuntimeContext,
        args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RuntimeError> {
        let mut fields = IndexMap::new();
        // Dynamic construction, optionally matches positional constructor arguments to fields
        for (idx, field) in self.fields.iter().enumerate() {
            let val = if idx < args.len() {
                args[idx].clone()
            } else {
                RuntimeValue::Null
            };
            fields.insert(field.name.name.clone(), val);
        }
        let inst = StructInstance::new(self.name.clone(), fields, false);
        Ok(RuntimeValue::StructInstance(Rc::new(RefCell::new(inst))))
    }
}

// Model constructor Callable
struct ModelConstructor {
    name: String,
    decl: ModelDecl,
}

impl Callable for ModelConstructor {
    fn name(&self) -> &str {
        &self.name
    }

    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        ctx: &mut techscript_runtime::RuntimeContext,
        args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RuntimeError> {
        let mut fields = IndexMap::new();
        // Model constructor evaluates class fields in their default template
        // We can pass constructor args to default template
        for (idx, field) in self.decl.fields.iter().enumerate() {
            let field_name = match &field.pattern {
                Pattern::Single(ident) => ident.name.clone(),
                _ => continue,
            };
            let val = if idx < args.len() {
                args[idx].clone()
            } else {
                RuntimeValue::Null
            };
            fields.insert(field_name, val);
        }

        let inst = ModelInstance::new(self.name.clone(), fields);
        let rc_inst = Rc::new(RefCell::new(inst));

        // Bind model methods to this instance (injecting self in method's parent closure env!)
        // Create method closures with parent scope having `self = rc_inst`
        for method in &self.decl.methods {
            let method_env = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
                &ctx.global_env,
            )))));
            method_env.borrow_mut().define(
                "self".to_string(),
                RuntimeValue::ModelInstance(Rc::clone(&rc_inst)),
                true,
            );

            let mut params = Vec::new();
            for param in &method.params {
                params.push(param.name.name.clone());
            }

            // Create bridged Callable UserFunction with `method_env` as closure
            let user_func = techscript_runtime::UserFunction {
                name: method.name.name.clone(),
                params,
                body: techscript_runtime::FunctionBody::Ast(method.body.clone()),
                closure: method_env,
            };

            // In order to call this method, we must be able to resolve it inside interpreter
            // Bridge the user function method so it executes correctly
            // We can place the bound method Callable inside the fields map of the ModelInstance!
            rc_inst.borrow_mut().fields.insert(
                method.name.name.clone(),
                RuntimeValue::Function(Rc::new(BridgedMethod { user_func })),
            );
        }

        Ok(RuntimeValue::ModelInstance(rc_inst))
    }
}

struct BridgedMethod {
    user_func: techscript_runtime::UserFunction,
}

impl Callable for BridgedMethod {
    fn name(&self) -> &str {
        &self.user_func.name
    }

    fn arity(&self) -> usize {
        self.user_func.params.len()
    }

    fn call(
        &self,
        ctx: &mut techscript_runtime::RuntimeContext,
        args: Vec<RuntimeValue>,
    ) -> Result<RuntimeValue, RuntimeError> {
        // Evaluate the bridged method. Since we need to traverse AST, let's create a temporary interpreter
        // using the method's closure environment!
        let mut interpreter = Interpreter {
            ctx: techscript_runtime::RuntimeContext {
                config: ctx.config.clone(),
                global_env: Rc::clone(&ctx.global_env),
                registry: techscript_runtime::NativeRegistry::new(),
            },
            env: Rc::clone(&self.user_func.closure),
            call_stack: Vec::new(),
        };

        // Push new local scope for method execution
        let local_env = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
            &self.user_func.closure,
        )))));
        for (param, arg) in self.user_func.params.iter().zip(args) {
            local_env.borrow_mut().define(param.clone(), arg, false);
        }

        if let techscript_runtime::FunctionBody::Ast(ref body) = self.user_func.body {
            let signal = interpreter.with_scope(local_env, |interp| interp.execute_block(body))?;
            match signal {
                FlowSignal::Return(val) => Ok(val),
                FlowSignal::Throw(err) => Err(err),
                _ => Ok(RuntimeValue::Null),
            }
        } else {
            Ok(RuntimeValue::Null)
        }
    }
}
