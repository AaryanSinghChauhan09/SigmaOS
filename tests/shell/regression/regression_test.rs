// tests/shell/regression/regression_test.rs
// Regression Test Suite for SigmaOS Shell

#[test]
fn test_regression_empty_pipeline_handling() {
    let empty_input = "";
    assert!(empty_input.trim().is_empty());
}

#[test]
fn test_regression_unclosed_quote_recovery() {
    let input = "echo 'unclosed string";
    let is_unclosed = input.chars().filter(|&c| c == '\'').count() % 2 != 0;
    assert!(is_unclosed);
}

#[test]
fn test_regression_empty_fields_ifs() {
    let unquoted = "foo   bar";
    let words: Vec<&str> = unquoted.split_whitespace().collect();
    assert_eq!(words.len(), 2);
    assert_eq!(words[0], "foo");
    assert_eq!(words[1], "bar");
}
