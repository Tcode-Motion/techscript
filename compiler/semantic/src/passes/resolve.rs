use crate::context::SemanticContext;
use crate::pipeline::Pass;
use crate::symbol_table::Symbol;
use crate::types::Type;
use techscript_ast::{
    Block, ConstDecl, Expression, FuncDecl, MethodDecl, MethodKeyword, ModelDecl, Pattern, Program,
    Statement, VarDecl,
};
use techscript_common::Ident;
use techscript_errors::{Diagnostic, DiagnosticLevel, ErrorCode};

/// Second semantic pass resolving scopes, constants, loops, return context, and calls.
pub struct ResolveSymbols;

impl Pass for ResolveSymbols {
    fn run(&mut self, program: &Program, context: &mut SemanticContext) {
        for stmt in &program.statements {
            let _ = self.resolve_statement(stmt, context);
        }
    }
}

impl ResolveSymbols {
    fn resolve_statement(&self, stmt: &Statement, context: &mut SemanticContext) -> Result<(), ()> {
        match stmt {
            Statement::VarDecl(decl) => self.resolve_var_decl(decl, context),
            Statement::ConstDecl(decl) => self.resolve_const_decl(decl, context),
            Statement::FuncDecl(decl) => {
                if context.symbol_table.scopes.len() > 1 {
                    let type_id = context.interner.intern(Type::Function {
                        params: vec![context.interner.any(); decl.params.len()],
                        ret_ty: context.interner.any(),
                    });
                    let symbol = Symbol::new(decl.name.name.clone(), true, true, false, type_id);
                    context
                        .symbol_table
                        .register(decl.name.name.clone(), symbol);
                }
                self.resolve_func_decl(decl, context)
            }
            Statement::StructDecl(decl) => self.resolve_struct_decl(decl, context),
            Statement::EnumDecl(decl) => self.resolve_enum_decl(decl, context),
            Statement::ModelDecl(decl) => self.resolve_model_decl(decl, context),
            Statement::ExportDecl(decl) => self.resolve_statement(&decl.declaration, context),
            Statement::Block(block) => self.resolve_block(block, context),
            Statement::If(stmt) => {
                let _ = self.resolve_expression(&stmt.condition, context);
                let _ = self.resolve_block(&stmt.body, context);
                for (cond, body) in &stmt.else_ifs {
                    let _ = self.resolve_expression(cond, context);
                    let _ = self.resolve_block(body, context);
                }
                if let Some(ref else_body) = stmt.else_body {
                    let _ = self.resolve_block(else_body, context);
                }
                Ok(())
            }
            Statement::For(stmt) => {
                let _ = self.resolve_expression(&stmt.iterable, context);
                context.loop_depth += 1;
                context.symbol_table.push_scope();

                // Register loop variable
                let symbol = Symbol::new(
                    stmt.item.name.clone(),
                    false,
                    false,
                    false,
                    context.interner.any(),
                );
                context
                    .symbol_table
                    .register(stmt.item.name.clone(), symbol);

                let _ = self.resolve_block(&stmt.body, context);

                context.symbol_table.pop_scope();
                context.loop_depth -= 1;
                Ok(())
            }
            Statement::While(stmt) => {
                let _ = self.resolve_expression(&stmt.condition, context);
                context.loop_depth += 1;
                let _ = self.resolve_block(&stmt.body, context);
                context.loop_depth -= 1;
                Ok(())
            }
            Statement::Repeat(stmt) => {
                let _ = self.resolve_expression(&stmt.count, context);
                context.loop_depth += 1;
                let _ = self.resolve_block(&stmt.body, context);
                context.loop_depth -= 1;
                Ok(())
            }
            Statement::Try(stmt) => {
                let _ = self.resolve_block(&stmt.body, context);
                context.symbol_table.push_scope();

                // Register catch variable
                let symbol = Symbol::new(
                    stmt.catch_var.name.clone(),
                    false,
                    false,
                    false,
                    context.interner.any(),
                );
                context
                    .symbol_table
                    .register(stmt.catch_var.name.clone(), symbol);

                let _ = self.resolve_block(&stmt.catch_body, context);
                context.symbol_table.pop_scope();
                Ok(())
            }
            Statement::Say(stmt) => {
                let _ = self.resolve_expression(&stmt.value, context);
                Ok(())
            }
            Statement::Return(stmt) => {
                if context.function_depth == 0 {
                    let diag = Diagnostic::new(
                        DiagnosticLevel::Error,
                        ErrorCode::E0312,
                        "Return statement outside function body".to_string(),
                        stmt.span,
                    );
                    context.diagnostics.push(diag);
                    return Err(());
                }
                if let Some(ref val) = stmt.value {
                    let _ = self.resolve_expression(val, context);
                }
                Ok(())
            }
            Statement::Throw(stmt) => {
                let _ = self.resolve_expression(&stmt.value, context);
                Ok(())
            }
            Statement::Break(stmt) => {
                if context.loop_depth == 0 {
                    let diag = Diagnostic::new(
                        DiagnosticLevel::Error,
                        ErrorCode::E0312, // Reusing flow control error category
                        "Break statement outside loop context".to_string(),
                        stmt.span,
                    );
                    context.diagnostics.push(diag);
                    return Err(());
                }
                Ok(())
            }
            Statement::Continue(stmt) => {
                if context.loop_depth == 0 {
                    let diag = Diagnostic::new(
                        DiagnosticLevel::Error,
                        ErrorCode::E0312,
                        "Continue statement outside loop context".to_string(),
                        stmt.span,
                    );
                    context.diagnostics.push(diag);
                    return Err(());
                }
                Ok(())
            }
            Statement::Import(stmt) => {
                if stmt.path.is_empty() {
                    let diag = Diagnostic::new(
                        DiagnosticLevel::Error,
                        ErrorCode::E0340,
                        "Empty module import path".to_string(),
                        stmt.span,
                    );
                    context.diagnostics.push(diag);
                    return Err(());
                }

                let path_strs = stmt.path.iter().map(|ident| ident.name.clone()).collect::<Vec<_>>();
                let resolver = techscript_module_resolver::DefaultModuleResolver::new();
                use techscript_module_resolver::ModuleResolver;
                match resolver.resolve(&path_strs) {
                    Ok(_) => {
                        if let Some(symbols) = &stmt.symbols {
                            for sym in symbols {
                                let sym_name = sym.name.clone();
                                let symbol = Symbol::new(
                                    sym_name.clone(),
                                    false,
                                    true,
                                    false,
                                    context.interner.any(),
                                );
                                context.symbol_table.register(sym_name, symbol);
                            }
                        } else {
                            let root_name = stmt.path[0].name.clone();
                            let symbol = Symbol::new(
                                root_name.clone(),
                                false,
                                true,
                                false,
                                context.interner.any(),
                            );
                            context.symbol_table.register(root_name, symbol);
                        }
                    }
                    Err(e) => {
                        let diag = Diagnostic::new(
                            DiagnosticLevel::Error,
                            ErrorCode::E0340,
                            format!("Failed to resolve module: {}", e),
                            stmt.span,
                        );
                        context.diagnostics.push(diag);
                        return Err(());
                    }
                }
                Ok(())
            }
            Statement::Expression(stmt) => {
                let _ = self.resolve_expression(&stmt.expression, context);
                Ok(())
            }
        }
    }

    fn resolve_var_decl(&self, decl: &VarDecl, context: &mut SemanticContext) -> Result<(), ()> {
        let _ = self.resolve_expression(&decl.initializer, context);
        self.register_pattern(&decl.pattern, false, context);
        Ok(())
    }

    fn resolve_const_decl(
        &self,
        decl: &ConstDecl,
        context: &mut SemanticContext,
    ) -> Result<(), ()> {
        let _ = self.resolve_expression(&decl.initializer, context);
        self.register_pattern(&decl.pattern, true, context);
        Ok(())
    }

    fn resolve_func_decl(&self, decl: &FuncDecl, context: &mut SemanticContext) -> Result<(), ()> {
        context.function_depth += 1;
        context.symbol_table.push_scope();

        // Count minimum and maximum required arguments for call arity checking
        let mut _min_args = 0;
        let mut max_args = 0;
        for param in &decl.params {
            max_args += 1;
            if param.default.is_none() {
                _min_args += 1;
            }

            // Register parameter
            let symbol = Symbol::new(
                param.name.name.clone(),
                false,
                false,
                false,
                context.interner.any(),
            );
            context
                .symbol_table
                .register(param.name.name.clone(), symbol);
        }

        // Update hoisted function metadata with arity type information
        let param_types = vec![context.interner.any(); max_args];
        let type_id = context.interner.intern(Type::Function {
            params: param_types,
            ret_ty: context.interner.any(),
        });
        if let Some(symbol) = context.symbol_table.lookup_mut(&decl.name.name) {
            symbol.type_id = type_id;
        }

        // Save arity (min, max) in context using node_types or a side-table.
        // We will store max_args in node_types or we can just encode arity in a separate mapping.
        // Let's store function type_id in node_types.
        context.node_types.insert(decl.id, type_id);

        let _ = self.resolve_block(&decl.body, context);

        context.symbol_table.pop_scope();
        context.function_depth -= 1;
        Ok(())
    }

    fn resolve_struct_decl(
        &self,
        decl: &techscript_ast::StructDecl,
        context: &mut SemanticContext,
    ) -> Result<(), ()> {
        let mut fields = std::collections::HashSet::new();
        for field in &decl.fields {
            if !fields.insert(&field.name.name) {
                let diag = Diagnostic::new(
                    DiagnosticLevel::Error,
                    ErrorCode::E0301,
                    format!(
                        "Duplicate field '{}' in struct '{}'",
                        field.name.name, decl.name.name
                    ),
                    field.name.span,
                );
                context.diagnostics.push(diag);
            }
        }
        Ok(())
    }

    fn resolve_enum_decl(
        &self,
        decl: &techscript_ast::EnumDecl,
        context: &mut SemanticContext,
    ) -> Result<(), ()> {
        let mut variants = std::collections::HashSet::new();
        for variant in &decl.variants {
            if !variants.insert(&variant.name.name) {
                let diag = Diagnostic::new(
                    DiagnosticLevel::Error,
                    ErrorCode::E0301,
                    format!(
                        "Duplicate variant '{}' in enum '{}'",
                        variant.name.name, decl.name.name
                    ),
                    variant.name.span,
                );
                context.diagnostics.push(diag);
            }
        }
        Ok(())
    }

    fn resolve_model_decl(
        &self,
        decl: &ModelDecl,
        context: &mut SemanticContext,
    ) -> Result<(), ()> {
        context.current_model = Some(decl.name.name.clone());

        // Resolve class fields
        for field in &decl.fields {
            let _ = self.resolve_expression(&field.initializer, context);
        }

        // Resolve methods
        for method in &decl.methods {
            if method.keyword == MethodKeyword::Fun {
                let diag = Diagnostic::new(
                    DiagnosticLevel::Warning,
                    techscript_errors::ErrorCode::W0015,
                    format!(
                        "Use of deprecated 'fun' keyword in method '{}'",
                        method.name.name
                    ),
                    method.span,
                );
                context.diagnostics.push(diag);
            }
            let _ = self.resolve_method_decl(method, context);
        }

        context.current_model = None;
        Ok(())
    }

    fn resolve_method_decl(
        &self,
        method: &MethodDecl,
        context: &mut SemanticContext,
    ) -> Result<(), ()> {
        context.function_depth += 1;
        context.symbol_table.push_scope();

        for param in &method.params {
            let symbol = Symbol::new(
                param.name.name.clone(),
                false,
                false,
                false,
                context.interner.any(),
            );
            context
                .symbol_table
                .register(param.name.name.clone(), symbol);
        }

        let _ = self.resolve_block(&method.body, context);

        context.symbol_table.pop_scope();
        context.function_depth -= 1;
        Ok(())
    }

    fn resolve_block(&self, block: &Block, context: &mut SemanticContext) -> Result<(), ()> {
        context.symbol_table.push_scope();
        for stmt in &block.statements {
            let _ = self.resolve_statement(stmt, context);
        }
        context.symbol_table.pop_scope();
        Ok(())
    }

    fn register_pattern(
        &self,
        pattern: &Pattern,
        is_constant: bool,
        context: &mut SemanticContext,
    ) {
        match pattern {
            Pattern::Single(ident) => self.register_ident(ident, is_constant, context),
            Pattern::Tuple(list) | Pattern::List(list) | Pattern::Struct(list) => {
                for ident in list {
                    self.register_ident(ident, is_constant, context);
                }
            }
        }
    }

    fn register_ident(&self, ident: &Ident, is_constant: bool, context: &mut SemanticContext) {
        let name = ident.name.clone();

        // 1. Check duplicate declaration in the current scope
        if let Some(scope) = context.symbol_table.scopes.last() {
            if scope.symbols.contains_key(&name) {
                let diag = Diagnostic::new(
                    DiagnosticLevel::Error,
                    ErrorCode::E0301,
                    format!("Duplicate variable declaration '{}'", name),
                    ident.span,
                );
                context.diagnostics.push(diag);
                return;
            }
        }

        // 2. Check shadowing in parent scopes
        if context.symbol_table.check_shadowing(&name) {
            let diag = Diagnostic::new(
                DiagnosticLevel::Warning,
                techscript_errors::ErrorCode::W0010,
                format!(
                    "Variable '{}' shadows an existing outer scope variable",
                    name
                ),
                ident.span,
            );
            context.diagnostics.push(diag);
        }

        // 3. Register symbol
        let symbol = Symbol::new(
            name.clone(),
            is_constant,
            false,
            false,
            context.interner.any(),
        );
        context.symbol_table.register(name, symbol);
    }

    fn resolve_expression(
        &self,
        expr: &Expression,
        context: &mut SemanticContext,
    ) -> Result<crate::types::TypeId, ()> {
        match expr {
            Expression::Literal(lit) => {
                let ty = match lit.value {
                    techscript_ast::LiteralVal::Int(_) => context.interner.int(),
                    techscript_ast::LiteralVal::Float(_) => context.interner.float(),
                    techscript_ast::LiteralVal::Str(_) => context.interner.string(),
                    techscript_ast::LiteralVal::Bool(_) => context.interner.bool(),
                    techscript_ast::LiteralVal::None => context.interner.none(),
                };
                context.node_types.insert(lit.id, ty);
                Ok(ty)
            }
            Expression::Identifier(ident) => {
                let name = ident.name.clone();
                if name == "self" {
                    if context.current_model.is_none() {
                        let diag = Diagnostic::new(
                            DiagnosticLevel::Error,
                            ErrorCode::E0320,
                            "Cannot reference 'self' outside model context".to_string(),
                            ident.span,
                        );
                        context.diagnostics.push(diag);
                        return Err(());
                    }
                    return Ok(context.interner.any());
                }

                if let Some(symbol) = context.symbol_table.lookup(&name) {
                    return Ok(symbol.type_id);
                }

                // Unresolved variable: compute Levenshtein suggestion on-demand
                let suggestion = find_suggestion(&name, context);
                let message = if let Some(suggest) = suggestion {
                    format!("Undefined variable '{}'. Did you mean '{}'?", name, suggest)
                } else {
                    format!("Undefined variable '{}'", name)
                };

                let diag = Diagnostic::new(
                    DiagnosticLevel::Error,
                    ErrorCode::E0300,
                    message,
                    ident.span,
                );
                context.diagnostics.push(diag);
                Err(())
            }
            Expression::Binary(bin) => {
                if bin.op == "?." {
                    let _left_ty = self.resolve_expression(&bin.left, context)?;
                    match &*bin.right {
                        Expression::Identifier(_) => {}
                        Expression::Call(call) => {
                            if let Expression::Identifier(_) = *call.callee {
                                // Member call
                            } else {
                                let _ = self.resolve_expression(&call.callee, context);
                            }
                            for arg in &call.args {
                                let _ = self.resolve_expression(arg, context);
                            }
                        }
                        Expression::Member(mem) => {
                            let _ = self.resolve_expression(&mem.object, context);
                        }
                        other => {
                            let _ = self.resolve_expression(other, context);
                        }
                    }
                    let res_ty = context.interner.any();
                    context.node_types.insert(bin.id, res_ty);
                    return Ok(res_ty);
                }

                let left_ty = self.resolve_expression(&bin.left, context)?;
                let right_ty = self.resolve_expression(&bin.right, context)?;

                // Implicit numeric coercions: if either operand is Float, output is Float.
                let res_ty = if left_ty == context.interner.float()
                    || right_ty == context.interner.float()
                    || bin.op == "/"
                {
                    context.interner.float()
                } else if bin.op == "//" {
                    context.interner.int()
                } else {
                    left_ty
                };
                context.node_types.insert(bin.id, res_ty);
                Ok(res_ty)
            }
            Expression::Unary(un) => {
                let ty = self.resolve_expression(&un.right, context)?;
                context.node_types.insert(un.id, ty);
                Ok(ty)
            }
            Expression::Assignment(assign) => {
                // Check mutation of constant variable
                if let Expression::Identifier(ref ident) = *assign.target {
                    if let Some(symbol) = context.symbol_table.lookup(&ident.name) {
                        if symbol.is_constant {
                            let diag = Diagnostic::new(
                                DiagnosticLevel::Error,
                                ErrorCode::E0302,
                                format!("Cannot reassign constant variable '{}'", ident.name),
                                ident.span,
                            );
                            context.diagnostics.push(diag);
                            return Err(());
                        }
                    }
                }

                let _left_ty = self.resolve_expression(&assign.target, context)?;
                let right_ty = self.resolve_expression(&assign.value, context)?;
                context.node_types.insert(assign.id, right_ty);
                Ok(right_ty)
            }
            Expression::Call(call) => {
                let _callee_ty = self.resolve_expression(&call.callee, context)?;

                // Validate call arity check if callee is a known hoisted function
                if let Expression::Identifier(ref ident) = *call.callee {
                    if let Some(symbol) = context.symbol_table.lookup(&ident.name) {
                        if symbol.is_function {
                            let function_type = context.interner.get(symbol.type_id);
                            if let Type::Function { ref params, .. } = function_type {
                                // For TechScript 2.0 simple checking, let's compare argument counts.
                                // (min, max checking can be detailed as we expand)
                                if call.args.len() < params.len() {
                                    let diag = Diagnostic::new(
                                        DiagnosticLevel::Error,
                                        ErrorCode::E0310,
                                        format!("Too few arguments in call to '{}'. Expected {}, found {}", ident.name, params.len(), call.args.len()),
                                        call.span,
                                    );
                                    context.diagnostics.push(diag);
                                } else if call.args.len() > params.len() {
                                    let diag = Diagnostic::new(
                                        DiagnosticLevel::Error,
                                        ErrorCode::E0311,
                                        format!("Too many arguments in call to '{}'. Expected {}, found {}", ident.name, params.len(), call.args.len()),
                                        call.span,
                                    );
                                    context.diagnostics.push(diag);
                                }
                            }
                        }
                    }
                }

                for arg in &call.args {
                    let _ = self.resolve_expression(arg, context);
                }

                context.node_types.insert(call.id, context.interner.any());
                Ok(context.interner.any())
            }
            Expression::Member(mem) => {
                let _ = self.resolve_expression(&mem.object, context)?;
                context.node_types.insert(mem.id, context.interner.any());
                Ok(context.interner.any())
            }
            Expression::Index(idx) => {
                let _ = self.resolve_expression(&idx.object, context)?;
                let _ = self.resolve_expression(&idx.index, context)?;
                context.node_types.insert(idx.id, context.interner.any());
                Ok(context.interner.any())
            }
            Expression::Range(range) => {
                let _ = self.resolve_expression(&range.start, context)?;
                let _ = self.resolve_expression(&range.end, context)?;
                let list_ty = context.interner.intern(Type::List(context.interner.int()));
                context.node_types.insert(range.id, list_ty);
                Ok(list_ty)
            }
            Expression::Ask(ask) => {
                let _ = self.resolve_expression(&ask.prompt, context)?;
                let str_ty = context.interner.string();
                context.node_types.insert(ask.id, str_ty);
                Ok(str_ty)
            }
            Expression::New(new_expr) => {
                for arg in &new_expr.args {
                    let _ = self.resolve_expression(arg, context);
                }
                let model_ty = context
                    .interner
                    .intern(Type::Model(new_expr.class_name.name.clone()));
                context.node_types.insert(new_expr.id, model_ty);
                Ok(model_ty)
            }
            Expression::List(list) => {
                for item in &list.items {
                    let _ = self.resolve_expression(item, context);
                }
                let list_ty = context.interner.intern(Type::List(context.interner.any()));
                context.node_types.insert(list.id, list_ty);
                Ok(list_ty)
            }
            Expression::Map(map) => {
                for (k, v) in &map.entries {
                    let _ = self.resolve_expression(k, context);
                    let _ = self.resolve_expression(v, context);
                }
                let map_ty = context
                    .interner
                    .intern(Type::Map(context.interner.any(), context.interner.any()));
                context.node_types.insert(map.id, map_ty);
                Ok(map_ty)
            }
            Expression::FString(fstr) => {
                for part in &fstr.parts {
                    if let techscript_ast::FStringPart::Expr(ref expr) = part {
                        let _ = self.resolve_expression(expr, context);
                    }
                }
                let str_ty = context.interner.string();
                context.node_types.insert(fstr.id, str_ty);
                Ok(str_ty)
            }
            Expression::Group(group) => {
                let ty = self.resolve_expression(group, context)?;
                Ok(ty)
            }
            _ => Ok(context.interner.any()),
        }
    }
}

#[allow(clippy::needless_range_loop)]
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        dp[i][0] = i;
    }
    for j in 0..=len2 {
        dp[0][j] = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            if c1 == c2 {
                dp[i + 1][j + 1] = dp[i][j];
            } else {
                dp[i + 1][j + 1] =
                    1 + std::cmp::min(dp[i][j + 1], std::cmp::min(dp[i + 1][j], dp[i][j]));
            }
        }
    }
    dp[len1][len2]
}

fn find_suggestion(name: &str, context: &SemanticContext) -> Option<String> {
    let mut best_dist = 3;
    let mut suggestion = None;

    for scope in context.symbol_table.scopes.iter().rev() {
        for sym_name in scope.symbols.keys() {
            let dist = levenshtein_distance(name, sym_name);
            if dist < best_dist {
                best_dist = dist;
                suggestion = Some(sym_name.clone());
            }
        }
    }
    suggestion
}
