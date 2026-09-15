use techscript_formatter::DocumentFormatter;

#[test]
fn test_formatter_instantiation() {
    let formatter = DocumentFormatter::new(4);
    let output = formatter.format_source("");
    assert_eq!(output, "");
}

#[test]
fn test_formatter_complex() {
    let source = "
    make var_0 = 0
    say \"Hello 0\"
    make var_1 = 1
    say \"Hello 1\"
    ";

    let formatter = DocumentFormatter::new(4);
    let formatted = formatter.format_source(source);
    assert!(formatted.contains("make var_0 = 0"));
    assert!(formatted.contains("say \"Hello 0\""));
}
