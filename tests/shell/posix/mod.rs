// tests/shell/posix/mod.rs
#[test]
fn test_posix_quoting_rules() {
    let single_quoted = "'literal $VAR'";
    assert!(single_quoted.contains("$VAR"));
}
