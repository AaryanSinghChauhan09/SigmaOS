// tests/shell/diagnostics/diagnostics_test.rs
// Diagnostics Test Suite for SigmaOS Shell

#[path = "../../../src/shell/diagnostics/mod.rs"]
mod diagnostics;

use diagnostics::ShellDiagnosticsEngine;

#[test]
fn test_diagnostics_formatting_output() {
    let parse_err = ShellDiagnosticsEngine::format_parse_error(1, 4, "unexpected token", Some(";"));
    assert!(parse_err.contains("line 1, col 4"));

    let not_found_err = ShellDiagnosticsEngine::format_command_not_found("grap", &["grep", "cat"]);
    assert!(not_found_err.contains("command not found"));
    assert!(not_found_err.contains("did you mean 'grep'?"));
}

#[test]
fn test_diagnostics_redirection_error_message() {
    let redir_err = ShellDiagnosticsEngine::format_redirection_error("/etc/shadow", "Permission denied");
    assert_eq!(redir_err, "sigma-sh: /etc/shadow: Permission denied");
}

#[test]
fn test_diagnostics_unclosed_block_error() {
    let unclosed_err = ShellDiagnosticsEngine::format_unclosed_block_error("while", 5);
    assert_eq!(
        unclosed_err,
        "sigma-sh: unclosed construct 'while' originating at line 5"
    );
}
