use crate::control_flow::{EvalResult, ExecResult};
use crate::interpreter::Interpreter;
use crate::operations::{eval_binary, eval_unary};
use crate::visitor::AstVisitor;
use indexmap::IndexMap;
use std::cell::RefCell;
use std::rc::Rc;
use techscript_ast::{Expression, FStringPart, LiteralVal};
use techscript_runtime::{
    list_get, list_set, map_get, map_set, RuntimeError, RuntimeErrorKind, RuntimeValue,
};

impl AstVisitor for Interpreter {
    fn visit_statement(&mut self, stmt: &techscript_ast::Statement) -> ExecResult {
        self.execute_statement(stmt)
    }

    fn visit_expression(&mut self, expr: &Expression) -> EvalResult {
        match expr {
            Expression::Literal(lit) => match &lit.value {
                LiteralVal::Int(i) => Ok(RuntimeValue::Int(*i)),
                LiteralVal::Float(f) => Ok(RuntimeValue::Float(*f)),
                LiteralVal::Str(s) => Ok(RuntimeValue::Str(s.clone())),
                LiteralVal::Bool(b) => Ok(RuntimeValue::Bool(*b)),
                LiteralVal::None => Ok(RuntimeValue::Null),
            },
            Expression::Identifier(ident) => {
                let name = &ident.name;
                if name == "self" {
                    return self.env.borrow().lookup("self");
                }
                self.env.borrow().lookup(name)
            }
            Expression::Unary(un) => {
                let right_val = self.visit_expression(&un.right)?;
                eval_unary(&un.op, right_val)
            }
            Expression::Binary(bin) => {
                // Handle short-circuiting logical operators
                if bin.op == "and" || bin.op == "&&" {
                    let left_val = self.visit_expression(&bin.left)?;
                    if !left_val.is_truthy() {
                        return Ok(left_val);
                    }
                    return self.visit_expression(&bin.right);
                }
                if bin.op == "or" || bin.op == "||" {
                    let left_val = self.visit_expression(&bin.left)?;
                    if left_val.is_truthy() {
                        return Ok(left_val);
                    }
                    return self.visit_expression(&bin.right);
                }
                if bin.op == "??" {
                    let left_val = self.visit_expression(&bin.left)?;
                    if left_val != RuntimeValue::Null {
                        return Ok(left_val);
                    }
                    return self.visit_expression(&bin.right);
                }
                if bin.op == "?." {
                    let left_val = self.visit_expression(&bin.left)?;
                    if left_val == RuntimeValue::Null {
                        return Ok(RuntimeValue::Null);
                    }
                    // Evaluate RHS with left_val context
                    return self.eval_optional_chain(left_val, &bin.right);
                }

                let left_val = self.visit_expression(&bin.left)?;
                let right_val = self.visit_expression(&bin.right)?;
                eval_binary(&bin.op, left_val, right_val)
            }
            Expression::Assignment(assign) => {
                let value_val = self.visit_expression(&assign.value)?;
                self.eval_assignment(&assign.target, &assign.op, value_val, assign.span)
            }
            Expression::Group(inner) => self.visit_expression(inner),
            Expression::List(list) => {
                let mut items = Vec::new();
                for expr in &list.items {
                    items.push(self.visit_expression(expr)?);
                }
                Ok(RuntimeValue::List {
                    items: Rc::new(RefCell::new(items)),
                    is_const: false,
                })
            }
            Expression::Map(map) => {
                let mut entries = IndexMap::new();
                for (k_expr, v_expr) in &map.entries {
                    let k_val = self.visit_expression(k_expr)?;
                    let k_str = k_val.try_into_string()?;
                    let v_val = self.visit_expression(v_expr)?;
                    entries.insert(k_str, v_val);
                }
                Ok(RuntimeValue::Map {
                    entries: Rc::new(RefCell::new(entries)),
                    is_const: false,
                })
            }

            Expression::FString(fstr) => {
                let mut result = String::new();
                for part in &fstr.parts {
                    match part {
                        FStringPart::Literal(s) => result.push_str(s),
                        FStringPart::Expr(expr) => {
                            let val = self.visit_expression(expr)?;
                            result.push_str(&val.to_string());
                        }
                    }
                }
                Ok(RuntimeValue::Str(result))
            }
            Expression::Call(call) => {
                let callee_val = self.visit_expression(&call.callee)?;
                let mut args = Vec::new();
                for arg_expr in &call.args {
                    args.push(self.visit_expression(arg_expr)?);
                }

                if let RuntimeValue::Function(func) = callee_val {
                    if args.len() != func.arity()
                        && func.name() != "assert"
                        && func.name() != "exit"
                    {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::ArityMismatch {
                                expected: func.arity(),
                                found: args.len(),
                            },
                            Some(call.span),
                            None,
                        ));
                    }
                    // Prevent stack overflows
                    if self.call_stack.len() >= self.ctx.config.max_recursion_depth {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::StackOverflow,
                            Some(call.span),
                            None,
                        ));
                    }
                    self.call_stack.push(crate::control_flow::CallFrame::new(
                        func.name().to_string(),
                        Some(call.span),
                    ));
                    let res = func.call(&mut self.ctx, args);
                    self.call_stack.pop();
                    res
                } else {
                    Err(RuntimeError::new(
                        RuntimeErrorKind::TypeMismatch {
                            expected: "function".to_string(),
                            found: callee_val.runtime_type().to_string(),
                        },
                        Some(call.span),
                        None,
                    ))
                }
            }
            Expression::Member(mem) => {
                let obj_val = self.visit_expression(&mem.object)?;
                self.eval_member_access(obj_val, &mem.member.name, mem.span)
            }
            Expression::Index(idx) => {
                let obj_val = self.visit_expression(&idx.object)?;
                let idx_val = self.visit_expression(&idx.index)?;
                self.eval_index_access(obj_val, idx_val, idx.span)
            }
            Expression::New(new_expr) => {
                // Instantiate model constructor
                let model_name = &new_expr.class_name.name;
                let ctor_val = self.env.borrow().lookup(model_name)?;
                let mut args = Vec::new();
                for arg in &new_expr.args {
                    args.push(self.visit_expression(arg)?);
                }

                if let RuntimeValue::Function(func) = ctor_val {
                    func.call(&mut self.ctx, args)
                } else {
                    Err(RuntimeError::new(
                        RuntimeErrorKind::TypeMismatch {
                            expected: "model constructor".to_string(),
                            found: ctor_val.runtime_type().to_string(),
                        },
                        Some(new_expr.span),
                        None,
                    ))
                }
            }
            Expression::Range(range) => {
                let start_val = self.visit_expression(&range.start)?;
                let end_val = self.visit_expression(&range.end)?;
                let start = start_val.try_into_int()?;
                let end = end_val.try_into_int()?;
                let list = (start..end).map(RuntimeValue::Int).collect::<Vec<_>>();
                Ok(RuntimeValue::List {
                    items: Rc::new(RefCell::new(list)),
                    is_const: false,
                })
            }
            Expression::Ask(ask) => {
                let prompt_val = self.visit_expression(&ask.prompt)?;
                let ask_fn = self.ctx.registry.lookup("ask").unwrap();
                ask_fn.call(&mut self.ctx, vec![prompt_val])
            }
            Expression::Lambda(lambda) => {
                // Create a UserFunction representing the lambda and return it
                let mut params = Vec::new();
                for param in &lambda.params {
                    params.push(param.name.name.clone());
                }
                let user_func = techscript_runtime::UserFunction {
                    name: "lambda".to_string(),
                    params,
                    body: techscript_runtime::FunctionBody::Ast(lambda.body.clone()),
                    closure: Rc::clone(&self.env),
                };
                Ok(RuntimeValue::Function(Rc::new(
                    self.bridge_user_function(user_func),
                )))
            }
        }
    }
}

impl Interpreter {
    fn eval_member_access(
        &mut self,
        obj_val: RuntimeValue,
        member: &str,
        span: techscript_common::Span,
    ) -> EvalResult {
        match obj_val {
            RuntimeValue::StructInstance(inst) => {
                if let Some(val) = inst.borrow().fields.get(member) {
                    Ok(val.clone())
                } else {
                    Ok(RuntimeValue::Null)
                }
            }
            RuntimeValue::ModelInstance(inst) => {
                if let Some(val) = inst.borrow().fields.get(member) {
                    Ok(val.clone())
                } else {
                    Err(RuntimeError::new(
                        RuntimeErrorKind::MemberNotFound(member.to_string()),
                        Some(span),
                        None,
                    ))
                }
            }
            RuntimeValue::Map { entries, .. } => {
                if let Some(val) = entries.borrow().get(member) {
                    Ok(val.clone())
                } else {
                    Ok(RuntimeValue::Null)
                }
            }
            other => Err(RuntimeError::new(
                RuntimeErrorKind::TypeMismatch {
                    expected: "struct, model, or map".to_string(),
                    found: other.runtime_type().to_string(),
                },
                Some(span),
                None,
            )),
        }
    }

    fn eval_index_access(
        &mut self,
        obj_val: RuntimeValue,
        idx_val: RuntimeValue,
        span: techscript_common::Span,
    ) -> EvalResult {
        match obj_val {
            RuntimeValue::List { .. } => {
                let idx = idx_val.try_into_int()?;
                list_get(&obj_val, idx).map_err(|mut e| {
                    e.span = Some(span);
                    e
                })
            }
            RuntimeValue::Map { .. } => {
                let key = idx_val.try_into_string()?;
                map_get(&obj_val, &key).map_err(|mut e| {
                    e.span = Some(span);
                    e
                })
            }
            RuntimeValue::Tuple(elements) => {
                let idx = idx_val.try_into_int()?;
                let len = elements.len() as i64;
                let final_idx = if idx < 0 { len + idx } else { idx };
                if final_idx < 0 || final_idx >= len {
                    return Err(RuntimeError::new(
                        RuntimeErrorKind::IndexOutOfBounds,
                        Some(span),
                        None,
                    ));
                }
                Ok(elements[final_idx as usize].clone())
            }
            other => Err(RuntimeError::new(
                RuntimeErrorKind::TypeMismatch {
                    expected: "list, map, or tuple".to_string(),
                    found: other.runtime_type().to_string(),
                },
                Some(span),
                None,
            )),
        }
    }

    fn eval_assignment(
        &mut self,
        target: &Expression,
        op: &str,
        value_val: RuntimeValue,
        span: techscript_common::Span,
    ) -> EvalResult {
        match target {
            Expression::Identifier(ident) => {
                let name = &ident.name;
                let final_val = if op == "=" {
                    value_val
                } else {
                    let current = self.env.borrow().lookup(name)?;
                    let basic_op = &op[0..op.len() - 1]; // e.g. "+=" -> "+"
                    eval_binary(basic_op, current, value_val)?
                };
                self.env
                    .borrow_mut()
                    .assign(name, final_val.clone())
                    .map_err(|mut e| {
                        e.span = Some(span);
                        e
                    })?;
                Ok(final_val)
            }
            Expression::Index(idx) => {
                let obj_val = self.visit_expression(&idx.object)?;
                let idx_val = self.visit_expression(&idx.index)?;
                let final_val = if op == "=" {
                    value_val
                } else {
                    let current = self.eval_index_access(obj_val.clone(), idx_val.clone(), span)?;
                    let basic_op = &op[0..op.len() - 1];
                    eval_binary(basic_op, current, value_val)?
                };

                match obj_val {
                    RuntimeValue::List { .. } => {
                        let i = idx_val.try_into_int()?;
                        list_set(&obj_val, i, final_val.clone()).map_err(|mut e| {
                            e.span = Some(span);
                            e
                        })?;
                    }
                    RuntimeValue::Map { .. } => {
                        let k = idx_val.try_into_string()?;
                        map_set(&obj_val, k, final_val.clone()).map_err(|mut e| {
                            e.span = Some(span);
                            e
                        })?;
                    }
                    _ => {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::TypeMismatch {
                                expected: "mutable list or map".to_string(),
                                found: obj_val.runtime_type().to_string(),
                            },
                            Some(span),
                            None,
                        ))
                    }
                }
                Ok(final_val)
            }
            Expression::Member(mem) => {
                let obj_val = self.visit_expression(&mem.object)?;
                let final_val = if op == "=" {
                    value_val
                } else {
                    let current =
                        self.eval_member_access(obj_val.clone(), &mem.member.name, span)?;
                    let basic_op = &op[0..op.len() - 1];
                    eval_binary(basic_op, current, value_val)?
                };

                match obj_val {
                    RuntimeValue::StructInstance(ref inst) => {
                        if inst.borrow().is_const {
                            return Err(RuntimeError::new(
                                RuntimeErrorKind::InvalidOperation(
                                    "Cannot mutate const struct".to_string(),
                                ),
                                Some(span),
                                None,
                            ));
                        }
                        inst.borrow_mut()
                            .fields
                            .insert(mem.member.name.clone(), final_val.clone());
                    }
                    RuntimeValue::ModelInstance(ref inst) => {
                        inst.borrow_mut()
                            .fields
                            .insert(mem.member.name.clone(), final_val.clone());
                    }
                    RuntimeValue::Map {
                        ref entries,
                        is_const,
                    } => {
                        if is_const {
                            return Err(RuntimeError::new(
                                RuntimeErrorKind::InvalidOperation(
                                    "Cannot mutate const map".to_string(),
                                ),
                                Some(span),
                                None,
                            ));
                        }
                        entries
                            .borrow_mut()
                            .insert(mem.member.name.clone(), final_val.clone());
                    }
                    _ => {
                        return Err(RuntimeError::new(
                            RuntimeErrorKind::TypeMismatch {
                                expected: "struct, model, or map".to_string(),
                                found: obj_val.runtime_type().to_string(),
                            },
                            Some(span),
                            None,
                        ))
                    }
                }
                Ok(final_val)
            }
            _ => Err(RuntimeError::new(
                RuntimeErrorKind::InvalidOperation("Invalid assignment target".to_string()),
                Some(span),
                None,
            )),
        }
    }

    fn eval_optional_chain(&mut self, left_val: RuntimeValue, right: &Expression) -> EvalResult {
        match right {
            Expression::Identifier(ident) => {
                // e.g. a ?. b
                self.eval_member_access(left_val, &ident.name, ident.span)
            }
            Expression::Member(mem) => {
                // nested access: e.g. a ?. b.c
                // Evaluate left side of MemberExpr first under LeftVal
                let intermediate = self.eval_optional_chain(left_val, &mem.object)?;
                self.eval_member_access(intermediate, &mem.member.name, mem.span)
            }
            Expression::Index(idx) => {
                // e.g. a ?. [idx]
                let idx_val = self.visit_expression(&idx.index)?;
                self.eval_index_access(left_val, idx_val, idx.span)
            }
            Expression::Call(call) => {
                // Method call: e.g. a ?. greet() or a ?. b.greet()
                // Let's resolve callee
                let method_val = self.eval_optional_chain(left_val, &call.callee)?;
                let mut args = Vec::new();
                for arg in &call.args {
                    args.push(self.visit_expression(arg)?);
                }
                if let RuntimeValue::Function(func) = method_val {
                    func.call(&mut self.ctx, args)
                } else {
                    Err(RuntimeError::new(
                        RuntimeErrorKind::TypeMismatch {
                            expected: "method".to_string(),
                            found: method_val.runtime_type().to_string(),
                        },
                        Some(call.span),
                        None,
                    ))
                }
            }
            _ => self.visit_expression(right),
        }
    }
}
