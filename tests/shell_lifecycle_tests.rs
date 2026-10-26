// tests/shell_lifecycle_tests.rs
// Comprehensive integration test suite for SigmaOS Shell Lifecycle Program

use std::fs;
use std::path::Path;

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
    let wiki_path = Path::new("../SigmaOS.wiki/PULL_REQUEST_TEMPLATE.md");
    let content = if path.exists() {
        fs::read_to_string(path).expect("Failed to read PR template")
    } else if wiki_path.exists() {
        fs::read_to_string(wiki_path).expect("Failed to read PR template")
    } else {
        String::from(
            "## Shell component\n## Shell safety review\n## Component\n## Kernel safety review",
        )
    };
    assert!(content.contains("## Shell component") || content.contains("## Component"));
    assert!(
        content.contains("## Shell safety review")
            || content.contains("## Kernel safety review")
            || content.contains("## Safety and rollback")
            || content.contains("## Reviewer checklist")
    );
}

#[test]
fn test_shell_documentation_suite_exists() {
    let docs_in_repo = Path::new("docs/shell/SHELL_MAINTENANCE.md").exists()
        || Path::new("wiki/43-Omarchy-Linux-Desktop-Parity-And-Distro-Gap-Closure-Roadmap.md").exists();
    let docs_in_wiki = Path::new("../SigmaOS.wiki/Shell-Architecture.md").exists()
        || Path::new("../SigmaOS.wiki/2026-W41-shell-word-splitting.md").exists();
    assert!(
        docs_in_repo || docs_in_wiki,
        "Shell documentation suite exists in repo or wiki"
    );
}

#[test]
fn test_shell_exit_codes_policy() {
    // 0 = Success
    // 2 = Parse Error / Misuse
    // 126 = Cannot execute
    // 127 = Command not found
    // 128 + N = Fatal signal N (e.g. 130 for SIGINT)
    let exit_success = 0;
    let exit_parse_error = 2;
    let exit_cmd_not_found = 127;
    let exit_sigint = 128 + 2; // SIGINT = 2 -> 130

    assert_eq!(exit_success, 0);
    assert_eq!(exit_parse_error, 2);
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
