use techscript_formatter::DocumentFormatter;

#[test]
fn test_formatter_instantiation() {
    let formatter = DocumentFormatter::new(4);
    let output = formatter.format_source("");
    assert_eq!(output, "");
}
