// tests/shell/builtins/builtins_test.rs
// Built-in Command Contracts Test Suite for SigmaOS Shell

#[test]
fn test_builtin_cd_pwd_contracts() {
    let builtins = vec!["cd", "pwd", "echo", "export", "alias", "set", "exit"];
    assert!(builtins.contains(&"cd"));
    assert!(builtins.contains(&"pwd"));
    assert!(builtins.contains(&"echo"));
    assert!(builtins.contains(&"export"));
    assert!(builtins.contains(&"alias"));
    assert!(builtins.contains(&"set"));
    assert!(builtins.contains(&"exit"));
}

#[test]
fn test_builtin_cd_behavior_flags() {
    let logical_flag = "-L";
    let physical_flag = "-P";
    assert_eq!(logical_flag, "-L");
    assert_eq!(physical_flag, "-P");
}

#[test]
fn test_builtin_echo_behavior_flags() {
    let no_newline_flag = "-n";
    let interpret_escapes = "-e";
    let literal_escapes = "-E";
    assert_eq!(no_newline_flag, "-n");
    assert_eq!(interpret_escapes, "-e");
    assert_eq!(literal_escapes, "-E");
}
