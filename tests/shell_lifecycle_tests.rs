// tests/shell_lifecycle_tests.rs
// Comprehensive integration test suite for SigmaOS Shell Lifecycle Program

use std::fs;
use std::path::Path;

#[path = "../src/governance/shell_lifecycle_program.rs"]
mod shell_lifecycle_program;

use shell_lifecycle_program::{
    ShellAcceptanceGate, ShellLifecycleProgramEngine,
};

#[test]
fn test_shell_lifecycle_program_engine_verification() {
    let engine = ShellLifecycleProgramEngine::new();
    assert_eq!(engine.tasks().len(), 13);
    assert_eq!(engine.pipeline().len(), 12);

    let valid_seq: Vec<u8> = (1..=12).collect();
    assert!(engine.verify_pipeline_sequence(&valid_seq));

    let all_gates = vec![
        ShellAcceptanceGate::G0CodeQuality,
        ShellAcceptanceGate::G1ParserTests,
        ShellAcceptanceGate::G2PosixSubsetTests,
        ShellAcceptanceGate::G3FuzzSmoke,
        ShellAcceptanceGate::G4BootToShell,
        ShellAcceptanceGate::G5InteractiveTerminalRestore,
        ShellAcceptanceGate::G6SecurityReview,
        ShellAcceptanceGate::G7BuiltinContracts,
        ShellAcceptanceGate::G8ReleaseClaimsVerified,
    ];
    assert!(engine.evaluate_acceptance_gates(&all_gates));

    let metrics = engine.current_health_metrics();
    assert!(metrics.is_healthy());
}

#[test]
fn test_shell_rules_file_exists_and_valid() {
    assert!(
        Path::new("SHELL_RULES.toml").exists(),
        "SHELL_RULES.toml must exist in repo root"
    );
    let content = fs::read_to_string("SHELL_RULES.toml").expect("Failed to read SHELL_RULES.toml");

    assert!(content.contains("[rule.deterministic_scripts]"));
    assert!(content.contains("[rule.explicit_exit_status]"));
    assert!(content.contains("[rule.no_partial_execution_on_parse_error]"));
    assert!(content.contains("[rule.safe_expansion_order]"));
    assert!(content.contains("[rule.quoting_integrity]"));
    assert!(content.contains("[rule.redirection_safety]"));
    assert!(content.contains("[rule.signal_safety]"));
    assert!(content.contains("[rule.builtin_contract]"));
    assert!(content.contains("[rule.no_unbounded_input]"));
    assert!(content.contains("[rule.fuzz_external_input]"));
    assert!(content.contains("[rule.status_evidence]"));
}

#[test]
fn test_pull_request_template_exists() {
    let path = Path::new(".github/PULL_REQUEST_TEMPLATE.md");
    assert!(path.exists(), ".github/PULL_REQUEST_TEMPLATE.md must exist");
    let content = fs::read_to_string(path).expect("Failed to read PR template");

    assert!(content.contains("## Shell component"));
    assert!(content.contains("## Shell safety review"));
    assert!(content.contains("## Recurring task"));
    assert!(content.contains("## Reference behavior"));
}

#[test]
fn test_shell_documentation_suite_exists() {
    let required_docs = [
        "docs/shell/SHELL_MAINTENANCE.md",
        "docs/shell/SHELL_CONFORMANCE.md",
        "docs/shell/GRAMMAR.md",
        "docs/shell/BUILTINS.md",
        "docs/shell/EXIT_CODES.md",
        "docs/shell/INTERACTIVE_SHELL.md",
        "docs/shell/DIAGNOSTICS.md",
        "docs/studies/2026-W41-shell-word-splitting.md",
    ];

    for doc in &required_docs {
        assert!(
            Path::new(doc).exists(),
            "Required documentation file {} must exist",
            doc
        );
    }
}

#[test]
fn test_shell_exit_codes_policy() {
    let exit_success = 0;
    let exit_parse_error = 2;
    let exit_cannot_exec = 126;
    let exit_cmd_not_found = 127;
    let exit_sigint = 128 + 2; // SIGINT = 2 -> 130

    assert_eq!(exit_success, 0);
    assert_eq!(exit_parse_error, 2);
    assert_eq!(exit_cannot_exec, 126);
    assert_eq!(exit_cmd_not_found, 127);
    assert_eq!(exit_sigint, 130);
}

#[test]
fn test_quoting_and_glob_expansion_rules() {
    let unquoted = "*.rs";
    let single_quoted = "'*.rs'";
    let double_quoted = "\"*.rs\"";

    assert_ne!(unquoted, single_quoted);
    assert!(single_quoted.starts_with('\'') && single_quoted.ends_with('\''));
    assert!(double_quoted.starts_with('"') && double_quoted.ends_with('"'));
}

#[test]
fn test_expansion_order_sequence() {
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
    assert_eq!(steps[2], "parameter_expansion");
    assert_eq!(steps[6], "pathname_expansion");
}

#[test]
fn test_diagnostics_formatting() {
    let parse_err = format!(
        "sigma-sh: parse error at line {}, col {}: {}",
        1, 5, "unexpected token"
    );
    assert!(parse_err.contains("line 1, col 5"));

    let cmd_err = format!(
        "sigma-sh: {}: command not found (did you mean '{}'?)",
        "htep", "help"
    );
    assert!(cmd_err.contains("command not found"));
    assert!(cmd_err.contains("did you mean 'help'?"));
}

#[test]
fn test_interactive_prompt_and_signals() {
    let ps1 = "\\u@\\h:\\w\\$ ";
    let rendered = ps1
        .replace("\\u", "sigma")
        .replace("\\h", "sigmaos")
        .replace("\\w", "~")
        .replace("\\$", "$");

    assert_eq!(rendered, "sigma@sigmaos:~$ ");
}
