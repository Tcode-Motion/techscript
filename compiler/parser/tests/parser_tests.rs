use techscript_ast::{Expression, LiteralVal, Pattern, Statement};
use techscript_errors::DiagnosticReporter;
use techscript_lexer::lex;
use techscript_parser::parse;

fn parse_source(source: &str) -> techscript_ast::Program {
    let mut reporter = DiagnosticReporter::new();
    let tokens = lex(source, &mut reporter).expect("lexing should succeed");
    parse(&tokens, &mut reporter).expect("parsing should succeed")
}

#[test]
fn test_parser_empty() {
    let program = parse_source("");
    assert_eq!(program.statements.len(), 0);
}

#[test]
fn test_parser_variable_and_constant_declarations() {
    let program = parse_source("make x = 42\nconst y = 3.14\nlet (a, b) = 100");
    assert_eq!(program.statements.len(), 3);

    // 1. make x = 42
    if let Statement::VarDecl(ref decl) = program.statements[0] {
        if let Pattern::Single(ref ident) = decl.pattern {
            assert_eq!(ident.name, "x");
        } else {
            panic!("Expected Pattern::Single");
        }
        if let Expression::Literal(ref lit) = decl.initializer {
            assert_eq!(lit.value, LiteralVal::Int(42));
        } else {
            panic!("Expected LiteralExpr");
        }
    } else {
        panic!("Expected Statement::VarDecl");
    }

    // 2. const y = 3.14
    if let Statement::ConstDecl(ref decl) = program.statements[1] {
        if let Pattern::Single(ref ident) = decl.pattern {
            assert_eq!(ident.name, "y");
        } else {
            panic!("Expected Pattern::Single");
        }
    } else {
        panic!("Expected Statement::ConstDecl");
    }

    // 3. let (a, b) = 100
    if let Statement::VarDecl(ref decl) = program.statements[2] {
        if let Pattern::Tuple(ref idents) = decl.pattern {
            assert_eq!(idents.len(), 2);
            assert_eq!(idents[0].name, "a");
            assert_eq!(idents[1].name, "b");
        } else {
            panic!("Expected Pattern::Tuple");
        }
    } else {
        panic!("Expected Statement::VarDecl");
    }
}

#[test]
fn test_parser_function_declaration() {
    let program =
        parse_source("async build calculate<T>(a: T, b = 10) -> Int {\n  return a + b\n}");
    assert_eq!(program.statements.len(), 1);

    if let Statement::FuncDecl(ref decl) = program.statements[0] {
        assert!(decl.async_kw);
        assert_eq!(decl.name.name, "calculate");
        assert_eq!(decl.generic_params.as_ref().unwrap().len(), 1);
        assert_eq!(decl.generic_params.as_ref().unwrap()[0].name, "T");
        assert_eq!(decl.params.len(), 2);
        assert_eq!(decl.params[0].name.name, "a");
        assert!(decl.params[0].type_ann.is_some());
        assert_eq!(decl.params[1].name.name, "b");
        assert!(decl.params[1].default.is_some());
        assert_eq!(decl.return_type.as_ref().unwrap().name.name, "Int");
        assert_eq!(decl.body.statements.len(), 1);
    } else {
        panic!("Expected Statement::FuncDecl");
    }
}

#[test]
fn test_parser_struct_and_enum() {
    let program = parse_source(
        "struct Point {\n  x: Int\n  y: Int\n}\n\nenum Option {\n  Some(T)\n  None\n}",
    );
    assert_eq!(program.statements.len(), 2);

    // struct Point
    if let Statement::StructDecl(ref decl) = program.statements[0] {
        assert_eq!(decl.name.name, "Point");
        assert_eq!(decl.fields.len(), 2);
        assert_eq!(decl.fields[0].name.name, "x");
        assert_eq!(decl.fields[1].name.name, "y");
    } else {
        panic!("Expected Statement::StructDecl");
    }

    // enum Option
    if let Statement::EnumDecl(ref decl) = program.statements[1] {
        assert_eq!(decl.name.name, "Option");
        assert_eq!(decl.variants.len(), 2);
        assert_eq!(decl.variants[0].name.name, "Some");
        assert!(decl.variants[0].payload.is_some());
        assert_eq!(decl.variants[1].name.name, "None");
    } else {
        panic!("Expected Statement::EnumDecl");
    }
}

#[test]
fn test_parser_models() {
    let program = parse_source("model User extends Entity {\n  make name = \"User\"\n  build greet() {\n    say name\n  }\n}");
    assert_eq!(program.statements.len(), 1);

    if let Statement::ModelDecl(ref decl) = program.statements[0] {
        assert_eq!(decl.name.name, "User");
        assert_eq!(decl.parent.as_ref().unwrap().name, "Entity");
        assert_eq!(decl.fields.len(), 1);
        assert_eq!(decl.methods.len(), 1);
        assert_eq!(decl.methods[0].name.name, "greet");
    } else {
        panic!("Expected Statement::ModelDecl");
    }
}

#[test]
fn test_parser_control_flow() {
    let program =
        parse_source("if condition {\n  say 1\n} elif other {\n  say 2\n} else {\n  say 3\n}");
    assert_eq!(program.statements.len(), 1);

    if let Statement::If(ref stmt) = program.statements[0] {
        assert_eq!(stmt.else_ifs.len(), 1);
        assert!(stmt.else_body.is_some());
    } else {
        panic!("Expected Statement::If");
    }
}

#[test]
fn test_parser_loops_and_try() {
    let program = parse_source("for x in list {\n  break\n}\nwhile condition {\n  continue\n}\nrepeat 5 {\n  say x\n}\ntry {\n  throw error\n} catch err {\n  say err\n}");
    assert_eq!(program.statements.len(), 4);

    assert!(matches!(program.statements[0], Statement::For(_)));
    assert!(matches!(program.statements[1], Statement::While(_)));
    assert!(matches!(program.statements[2], Statement::Repeat(_)));
    assert!(matches!(program.statements[3], Statement::Try(_)));
}

#[test]
fn test_parser_pratt_precedence_and_associativity() {
    // 1. Precedence: * has higher precedence than +
    // 2. Associativity: + is left-associative, ** is right-associative
    let program = parse_source("1 + 2 * 3 ** 4 ** 5");
    assert_eq!(program.statements.len(), 1);

    if let Statement::Expression(ref stmt) = program.statements[0] {
        if let Expression::Binary(ref bin) = stmt.expression {
            assert_eq!(bin.op, "+"); // + has lowest precedence, parsed at root
            if let Expression::Binary(ref right_bin) = *bin.right {
                assert_eq!(right_bin.op, "*"); // * is parent of **
                if let Expression::Binary(ref exp_bin) = *right_bin.right {
                    assert_eq!(exp_bin.op, "**"); // first ** (right associative)
                    if let Expression::Binary(ref inner_exp) = *exp_bin.right {
                        assert_eq!(inner_exp.op, "**");
                    } else {
                        panic!("Expected nested exponentiation BinaryExpr");
                    }
                }
            }
        } else {
            panic!("Expected BinaryExpr at root");
        }
    }
}

#[test]
fn test_parser_member_index_and_calls() {
    let program = parse_source("obj.prop[idx](arg)");
    assert_eq!(program.statements.len(), 1);

    if let Statement::Expression(ref stmt) = program.statements[0] {
        assert!(matches!(stmt.expression, Expression::Call(_)));
    } else {
        panic!("Expected ExpressionStatement");
    }
}

#[test]
fn test_parser_fstrings() {
    let program = parse_source("say f\"Hello {name}!\"");
    assert_eq!(program.statements.len(), 1);

    if let Statement::Say(ref stmt) = program.statements[0] {
        assert!(matches!(stmt.value, Expression::FString(_)));
    } else {
        panic!("Expected Statement::Say");
    }
}

#[test]
fn test_parser_error_recovery() {
    let mut reporter = DiagnosticReporter::new();
    let source = "make x = ;\nsay 42"; // x = ; is malformed variable declaration
    let tokens = lex(source, &mut reporter).expect("lex should succeed");
    let _program = parse(&tokens, &mut reporter).expect_err("should have errors");

    // The parser should recover after semicolon and parse `say 42` successfully, but return Err because reporter has errors.
    assert!(reporter.has_errors());
    assert_eq!(reporter.get_diagnostics().len(), 1);
}
