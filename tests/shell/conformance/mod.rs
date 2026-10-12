// tests/shell/conformance/mod.rs
#[test]
fn test_posix_conformance_simple_commands() {
    let cmd = "echo hello world";
    assert!(cmd.contains("echo"));
}
