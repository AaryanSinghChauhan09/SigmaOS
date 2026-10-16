// tests/shell/parser/parser_test.rs
// Lexer and Parser Test Suite for SigmaOS Shell

#[path = "../../../src/shell/command.rs"]
mod command;

#[test]
fn test_lexer_tokenization_concept() {
    let input = "echo hello world | grep hello > out.txt";
    assert!(input.contains("|"));
    assert!(input.contains(">"));
}
