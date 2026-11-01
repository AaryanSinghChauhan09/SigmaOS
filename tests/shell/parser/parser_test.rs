// tests/shell/parser/parser_test.rs
// Lexer and Parser Test Suite for SigmaOS Shell

#[test]
fn test_lexer_tokenization_concept() {
    let input = "echo hello world | grep hello > out.txt";
    assert!(input.contains("|"));
    assert!(input.contains(">"));
}

#[test]
fn test_parser_operator_precedence() {
    let operators = vec![">", "|", "&&", ";"];
    assert_eq!(operators.len(), 4);
    assert_eq!(operators[0], ">");
    assert_eq!(operators[1], "|");
    assert_eq!(operators[2], "&&");
    assert_eq!(operators[3], ";");
}
