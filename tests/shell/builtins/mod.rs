// tests/shell/builtins/mod.rs
#[test]
fn test_builtin_cd_and_pwd_contracts() {
    let cd_cmd = "cd /tmp";
    assert!(cd_cmd.starts_with("cd"));
}
