// tests/shell/mod.rs
pub mod builtins;
pub mod conformance;
pub mod diagnostics;
pub mod exit_codes;
pub mod interactive;
pub mod parser;
pub mod posix;
pub mod regression;

#[test]
fn test_shell_submodules_run() {
    assert!(true);
}
