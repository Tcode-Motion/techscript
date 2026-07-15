use techscript_common::{Span, NodeId, Ident};

#[test]
fn test_common_types() {
    let span = Span::new(0, 10);
    assert_eq!(span.start, 0);
    assert_eq!(span.end, 10);

    let id = NodeId(42);
    assert_eq!(id.0, 42);

    let ident = Ident::new("x".to_string(), span);
    assert_eq!(ident.name, "x");
    assert_eq!(ident.span, span);
}
