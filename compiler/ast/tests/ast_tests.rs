use techscript_ast::{
    AssignmentExpr, EnumDecl, EnumVariant, Expression, FieldSpec, Ident, LiteralExpr, LiteralVal,
    Pattern, StructDecl, VarDecl,
};
use techscript_common::{NodeId, Span};

#[test]
fn test_ast_construction_and_equality() {
    let span = Span::new(0, 10);
    let id = NodeId(1);
    let name_ident = Ident {
        name: "my_var".to_string(),
        span,
    };

    let var_decl = VarDecl::new(
        id,
        Pattern::Single(name_ident.clone()),
        None,
        Expression::Literal(LiteralExpr::new(NodeId(2), LiteralVal::Int(42), span)),
        span,
    );

    assert_eq!(var_decl.id, id);
    assert_eq!(var_decl.span, span);
    if let Pattern::Single(ref ident) = var_decl.pattern {
        assert_eq!(ident.name, "my_var");
    } else {
        panic!("expected Pattern::Single");
    }
}

#[test]
fn test_ast_struct_decl() {
    let span = Span::new(0, 20);
    let name_ident = Ident {
        name: "User".to_string(),
        span,
    };
    let field_name = Ident {
        name: "id".to_string(),
        span,
    };
    let type_name = Ident {
        name: "Int".to_string(),
        span,
    };
    let field = FieldSpec::new(field_name, TypeSpec::new(type_name, None, span), span);

    let struct_decl = StructDecl::new(NodeId(1), name_ident, vec![field], span);
    assert_eq!(struct_decl.fields.len(), 1);
    assert_eq!(struct_decl.fields[0].name.name, "id");
}

#[test]
fn test_ast_enum_decl() {
    let span = Span::new(0, 30);
    let name_ident = Ident {
        name: "Status".to_string(),
        span,
    };
    let variant_name = Ident {
        name: "Active".to_string(),
        span,
    };
    let variant = EnumVariant::new(variant_name, None, span);

    let enum_decl = EnumDecl::new(NodeId(1), name_ident, vec![variant], span);
    assert_eq!(enum_decl.variants.len(), 1);
    assert_eq!(enum_decl.variants[0].name.name, "Active");
}

#[test]
fn test_ast_assignment_expression() {
    let span = Span::new(0, 15);
    let target = Expression::Identifier(Ident {
        name: "x".to_string(),
        span,
    });
    let value = Expression::Literal(LiteralExpr::new(NodeId(1), LiteralVal::Int(100), span));

    let assign = AssignmentExpr::new(
        NodeId(2),
        Box::new(target),
        "=".to_string(),
        Box::new(value),
        span,
    );

    assert_eq!(assign.op, "=");
}

#[test]
fn test_ast_serialization() {
    let span = Span::new(0, 5);
    let lit = Expression::Literal(LiteralExpr::new(NodeId(1), LiteralVal::Int(42), span));
    let serialized = serde_json::to_string(&lit).expect("serialize should succeed");
    let deserialized: Expression =
        serde_json::from_str(&serialized).expect("deserialize should succeed");

    assert_eq!(lit, deserialized);
}

// Internal helper just to satisfy TypeSpec compilation in test_ast_struct_decl
use techscript_ast::TypeSpec;
