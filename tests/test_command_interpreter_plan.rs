// Integration Test for SigmaOS Command Interpreter Plan
// (`tests/test_command_interpreter_plan.rs`)

#[path = "../src/shell/sovereign_command_interpreter.rs"]
mod sovereign_command_interpreter;

use sovereign_command_interpreter::*;

#[test]
fn test_command_interpreter_plan_17_prs() {
    let suite = SovereignCommandInterpreterSuite::new();
    assert_eq!(suite.pr_maturity_matrix.len(), 17);

    for pr_id in 1..=17 {
        let (title, level) = suite
            .pr_maturity_matrix
            .get(&(pr_id as u32))
            .unwrap_or_else(|| panic!("PR {} missing from command interpreter suite", pr_id));
        assert!(!title.is_empty(), "PR {} title should not be empty", pr_id);
        assert_eq!(*level, PrMaturityLevel::Prototype);
    }
}

#[test]
fn test_shell_product_rules() {
    let rules = ShellProductRule::all_rules();
    assert_eq!(rules.len(), 8);

    for rule in &rules {
        let desc = rule.description();
        assert!(!desc.is_empty());
    }

    assert_eq!(
        ShellProductRule::ParseBeforeExecuting.description(),
        "Parse before executing: syntax errors identified prior to running any stage"
    );
    assert_eq!(
        ShellProductRule::PreserveArgumentsExactly.description(),
        "Preserve arguments exactly: structured arguments without re-splitting"
    );
    assert_eq!(
        ShellProductRule::UseAstNotAdHocSplitting.description(),
        "Use an AST, not ad hoc string splitting: distinct lexer, parser, expander, executor"
    );
    assert_eq!(
        ShellProductRule::KeepScriptAndInteractiveDistinct.description(),
        "Keep script and interactive modes distinct: no interactive hooks in scripts"
    );
    assert_eq!(
        ShellProductRule::DefineBehaviorForUnsupportedSyntax.description(),
        "Define behavior for unsupported syntax: explicit error and non-zero exit status"
    );
    assert_eq!(
        ShellProductRule::TreatInputAndEnvironmentAsUntrusted.description(),
        "Treat input and environment as untrusted: enforce bounded resources"
    );
    assert_eq!(
        ShellProductRule::NoCompatibilityClaimsWithoutEvidence.description(),
        "No compatibility claims based on examples alone: require linked tests"
    );
    assert_eq!(
        ShellProductRule::KeepCommandsAndPrivilegesSeparate.description(),
        "Keep commands and privileged operations separate: typed authorization interfaces"
    );
}

#[test]
fn test_shell_maintenance_cadences() {
    let cadences = ShellMaintenanceCadence::all_cadences();
    assert_eq!(cadences.len(), 9);

    for cadence in &cadences {
        let task = cadence.task_name();
        assert!(!task.is_empty());
    }

    assert_eq!(
        ShellMaintenanceCadence::EveryPr.task_name(),
        "ci(shell): run shell-specific checks for changed code"
    );
    assert_eq!(
        ShellMaintenanceCadence::WeeklyFuzzReview.task_name(),
        "test(shell): review fuzz crashes and parser regressions"
    );
    assert_eq!(
        ShellMaintenanceCadence::WeeklyDocReconciliation.task_name(),
        "docs(shell): reconcile feature status with implementation"
    );
    assert_eq!(
        ShellMaintenanceCadence::MonthlySecurityBoundaries.task_name(),
        "security(shell): review command execution and expansion boundaries"
    );
    assert_eq!(
        ShellMaintenanceCadence::MonthlyScriptCompatibility.task_name(),
        "test(shell): run script compatibility corpus"
    );
    assert_eq!(
        ShellMaintenanceCadence::MonthlyCleanupAudit.task_name(),
        "maint(shell): review terminal, signal, and child-process cleanup"
    );
    assert_eq!(
        ShellMaintenanceCadence::QuarterlyPosixAlignment.task_name(),
        "docs(shell): review POSIX and reference-shell alignment"
    );
    assert_eq!(
        ShellMaintenanceCadence::ReleaseCandidateCertification.task_name(),
        "release(shell): certify shell support claims"
    );
    assert_eq!(
        ShellMaintenanceCadence::PostIncidentFix.task_name(),
        "fix(shell): prevent recurrence of <bug>"
    );
}

#[test]
fn test_shell_rollout_tiers() {
    let foundation = ShellRolloutTier::Foundation;
    let runner = ShellRolloutTier::UsableCommandRunner;
    let interactive = ShellRolloutTier::InteractiveShell;
    let hardening = ShellRolloutTier::CompatibilityAndHardening;

    assert_eq!(foundation, ShellRolloutTier::Foundation);
    assert_eq!(runner, ShellRolloutTier::UsableCommandRunner);
    assert_eq!(interactive, ShellRolloutTier::InteractiveShell);
    assert_eq!(hardening, ShellRolloutTier::CompatibilityAndHardening);
}

#[test]
fn test_command_interpreter_parser_ast_execution() {
    let mut suite = SovereignCommandInterpreterSuite::new();

    // Verify AST generation
    let ast_simple = suite.parser_engine.parse("echo hello sovereign").unwrap();
    assert!(matches!(ast_simple, ShellAstNode::SimpleCommand { .. }));

    let status = suite.executor.execute_ast(&ast_simple).unwrap();
    assert_eq!(status, 0);
    let stdout = String::from_utf8_lossy(&suite.executor.captured_stdout);
    assert!(stdout.contains("hello sovereign"));

    // Verify pipeline AST
    let ast_pipeline = suite.parser_engine.parse("echo foo | cat").unwrap();
    assert!(matches!(ast_pipeline, ShellAstNode::Pipeline { .. }));

    // Verify AND / OR lists
    let ast_and = suite.parser_engine.parse("true && echo success").unwrap();
    assert!(matches!(ast_and, ShellAstNode::And { .. }));

    let ast_or = suite.parser_engine.parse("false || echo fallback").unwrap();
    assert!(matches!(ast_or, ShellAstNode::Or { .. }));
}

#[test]
fn test_command_interpreter_security_limits() {
    let gov = ShellResourceLimitsGovernor::new();
    let valid_input = "echo safe_input";
    assert!(gov.validate_input_size(valid_input).is_ok());

    let massive_input = "a".repeat(70000);
    assert!(gov.validate_input_size(&massive_input).is_err());
}

#[test]
fn test_full_17_pr_verification_pipeline() {
    let mut suite = SovereignCommandInterpreterSuite::new();
    let report_res = suite.run_all_17_pr_stages();
    assert!(report_res.is_ok(), "17-PR verification failed");

    let report = report_res.unwrap();
    assert!(report.contains("SigmaOS Command Interpreter PR Maturity Matrix:"));
    assert!(report.contains("PR 01: docs(shell): define command interpreter goals"));
    assert!(report.contains("PR 17: docs(shell): publish command reference"));

    for pr_id in 1..=17 {
        let (_, level) = suite.pr_maturity_matrix.get(&(pr_id as u32)).unwrap();
        assert_eq!(*level, PrMaturityLevel::Supported);
    }
}
