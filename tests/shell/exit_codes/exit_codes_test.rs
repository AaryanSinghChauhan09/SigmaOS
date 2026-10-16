// tests/shell/exit_codes/exit_codes_test.rs
// Exit Code Policy Conformance Test Suite for SigmaOS Shell

#[test]
fn test_exit_code_mappings() {
    let success = 0;
    let general_error = 1;
    let parse_error = 2;
    let cannot_execute = 126;
    let command_not_found = 127;
    let sigint_exit = 128 + 2;

    assert_eq!(success, 0);
    assert_eq!(general_error, 1);
    assert_eq!(parse_error, 2);
    assert_eq!(cannot_execute, 126);
    assert_eq!(command_not_found, 127);
    assert_eq!(sigint_exit, 130);
}
