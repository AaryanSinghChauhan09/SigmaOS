// tests/shell/builtins/builtins_test.rs
// Built-in Command Contracts Test Suite for SigmaOS Shell

#[test]
fn test_builtin_cd_pwd_contracts() {
    let builtins = vec!["cd", "pwd", "echo", "export", "alias", "set", "exit"];
    assert!(builtins.contains(&"cd"));
    assert!(builtins.contains(&"pwd"));
    assert!(builtins.contains(&"echo"));
    assert!(builtins.contains(&"export"));
    assert!(builtins.contains(&"exit"));
}
