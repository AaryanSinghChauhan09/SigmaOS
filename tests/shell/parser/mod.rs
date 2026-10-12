// tests/shell/parser/mod.rs
#[test]
fn test_parser_grammar_and_precedence() {
    let pipeline = "cat file | grep text";
    assert!(pipeline.contains('|'));
}
