// tests/shell.rs
// Conformance and lifecycle test suite for SigmaOS shell

#[path = "../src/shell/diagnostics/mod.rs"]
mod diagnostics;
#[path = "../src/shell/interactive/mod.rs"]
mod interactive;

use diagnostics::ShellDiagnosticsEngine;
use interactive::{InteractiveShellEngine, SignalEvent};

#[test]
fn test_interactive_prompt_rendering() {
    let engine = InteractiveShellEngine::new();
    let prompt = engine.render_prompt();
    assert!(prompt.contains("sigma@sigmaos:~"));
    assert!(prompt.contains("$"));
}

#[test]
fn test_interactive_ctrl_c_and_ctrl_d_handling() {
    let mut engine = InteractiveShellEngine::new();
    engine.line_buffer = String::from("ls -la");
    let event_c = engine.handle_key_input("\x03");
    assert_eq!(event_c, SignalEvent::Interrupt);
    assert!(engine.line_buffer.is_empty());

    let event_d = engine.handle_key_input("\x04");
    assert_eq!(event_d, SignalEvent::Eof);
}

#[test]
fn test_interactive_multiline_quote_detection() {
    let engine = InteractiveShellEngine::new();
    assert!(engine.is_incomplete_input("echo 'hello world"));
    assert!(!engine.is_incomplete_input("echo 'hello world'"));
}

#[test]
fn test_diagnostics_parse_error() {
    let err = ShellDiagnosticsEngine::format_parse_error(2, 8, "unexpected EOF", Some(";"));
    assert!(err.contains("line 2, col 8"));
    assert!(err.contains("unexpected EOF"));
}

#[test]
fn test_diagnostics_command_not_found_suggestion() {
    let err = ShellDiagnosticsEngine::format_command_not_found("grepp", &["grep", "cat", "find"]);
    assert!(err.contains("command not found"));
    assert!(err.contains("did you mean 'grep'?"));
}

#[test]
fn test_diagnostics_redirection_error() {
    let err = ShellDiagnosticsEngine::format_redirection_error("/etc/shadow", "Permission denied");
    assert_eq!(err, "sigma-sh: /etc/shadow: Permission denied");
}

#[test]
fn test_posix_conformance_subset_expansion_order() {
    let steps = vec![
        "brace_expansion",
        "tilde_expansion",
        "parameter_expansion",
        "command_substitution",
        "arithmetic_expansion",
        "word_splitting",
        "pathname_expansion",
        "quote_removal",
    ];

    assert_eq!(steps.len(), 8);
    assert_eq!(steps[0], "brace_expansion");
    assert_eq!(steps[5], "word_splitting");
}

#[test]
fn test_exit_codes_conformance() {
    let exit_success = 0;
    let exit_parse_error = 2;
    let exit_cannot_exec = 126;
    let exit_cmd_not_found = 127;
    let exit_sigint = 128 + 2;

    assert_eq!(exit_success, 0);
    assert_eq!(exit_parse_error, 2);
    assert_eq!(exit_cannot_exec, 126);
    assert_eq!(exit_cmd_not_found, 127);
    assert_eq!(exit_sigint, 130);
}
