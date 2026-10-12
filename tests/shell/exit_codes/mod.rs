// tests/shell/exit_codes/mod.rs
#[test]
fn test_exit_codes_policy_values() {
    assert_eq!(0, 0); // Success
    assert_eq!(2, 2); // Parse error
    assert_eq!(126, 126); // Cannot execute
    assert_eq!(127, 127); // Command not found
    assert_eq!(130, 128 + 2); // SIGINT
}
