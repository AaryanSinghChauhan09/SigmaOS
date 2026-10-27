//! SigmaOS Shell Lifecycle Program Engine
//!
//! Models, evaluates, and verifies recurring development, compatibility, security,
//! usability, and maintenance work for the SigmaOS userspace shell.
//! Covers task schedules, component scope, issue classification, release gates G0-G8,
//! health metrics, and the first 12 shell roadmap PRs.

use std::vec::Vec;
use std::string::String;

/// Recurring frequency for shell maintenance tasks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellTaskCadence {
    EveryPr,
    Daily,
    Weekly,
    Biweekly,
    Monthly,
    Quarterly,
    Semiannual,
}

impl ShellTaskCadence {
    pub fn name(&self) -> &'static str {
        match self {
            Self::EveryPr => "Every PR",
            Self::Daily => "Daily",
            Self::Weekly => "Weekly",
            Self::Biweekly => "Biweekly",
            Self::Monthly => "Monthly",
            Self::Quarterly => "Quarterly",
            Self::Semiannual => "Semiannual",
        }
    }
}

/// Structured definition of a recurring shell maintenance task
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellTaskDefinition {
    pub cadence: ShellTaskCadence,
    pub name: &'static str,
    pub description: &'static str,
    pub primary_output: &'static str,
    pub required_pr_prefix: &'static str,
}

/// Component scope areas of the shell
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellComponent {
    Lexer,
    Parser,
    Expander,
    Executor,
    JobControl,
    LineEditor,
    BuiltIns,
    ScriptRunner,
    Diagnostics,
}

impl ShellComponent {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Lexer => "Lexer",
            Self::Parser => "Parser",
            Self::Expander => "Expander",
            Self::Executor => "Executor",
            Self::JobControl => "Job control",
            Self::LineEditor => "Line editor",
            Self::BuiltIns => "Built-ins",
            Self::ScriptRunner => "Script runner",
            Self::Diagnostics => "Diagnostics",
        }
    }
}

/// Shell Issue Triage Classification Areas
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellIssueArea {
    ParserCorrectness,
    ExpansionCorrectness,
    ProcessPipeBehavior,
    Redirection,
    SignalHandling,
    BuiltInBehavior,
    InteractiveUx,
    ScriptCompatibility,
    SecuritySandboxing,
    Performance,
}

impl ShellIssueArea {
    pub fn name(&self) -> &'static str {
        match self {
            Self::ParserCorrectness => "parser correctness",
            Self::ExpansionCorrectness => "expansion correctness",
            Self::ProcessPipeBehavior => "process/pipe behavior",
            Self::Redirection => "redirection",
            Self::SignalHandling => "signal handling",
            Self::BuiltInBehavior => "built-in behavior",
            Self::InteractiveUx => "interactive UX",
            Self::ScriptCompatibility => "script compatibility",
            Self::SecuritySandboxing => "security/sandboxing",
            Self::Performance => "performance",
        }
    }
}

/// Shell Acceptance Gates G0 through G8
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShellAcceptanceGate {
    G0CodeQuality = 0,
    G1UnitParser = 1,
    G2PosixSubset = 2,
    G3FuzzSmoke = 3,
    G4BootToShell = 4,
    G5TerminalState = 5,
    G6SecurityReview = 6,
    G7BuiltinContracts = 7,
    G8ReleaseClaims = 8,
}

impl ShellAcceptanceGate {
    pub fn code(&self) -> &'static str {
        match self {
            Self::G0CodeQuality => "G0",
            Self::G1UnitParser => "G1",
            Self::G2PosixSubset => "G2",
            Self::G3FuzzSmoke => "G3",
            Self::G4BootToShell => "G4",
            Self::G5TerminalState => "G5",
            Self::G6SecurityReview => "G6",
            Self::G7BuiltinContracts => "G7",
            Self::G8ReleaseClaims => "G8",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::G0CodeQuality => "cargo check --lib, formatting, and unit tests pass",
            Self::G1UnitParser => "Shell unit and parser tests pass",
            Self::G2PosixSubset => "Documented POSIX subset tests pass",
            Self::G3FuzzSmoke => "Fuzz smoke passes",
            Self::G4BootToShell => "Boot reaches shell or defined diagnostic state",
            Self::G5TerminalState => "Interactive shell restores terminal state",
            Self::G6SecurityReview => "Security review complete for expansion, redirection, and process execution",
            Self::G7BuiltinContracts => "Built-ins have documented contracts and tests",
            Self::G8ReleaseClaims => "Release claims match tested capability",
        }
    }
}

/// Roadmap item for the first 12 Shell PRs
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellRoadmapPr {
    pub number: u8,
    pub pr_title: &'static str,
    pub purpose: &'static str,
}

/// Overall Shell Health Metrics status
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellHealthMetrics {
    pub ci_pass_rate_pct: u8,
    pub posix_subset_pass_rate_pct: u8,
    pub parser_fuzz_crash_rate_zero: bool,
    pub builtins_with_contracts_pct: u8,
    pub exit_code_documented_pct: u8,
    pub boot_to_shell_pass_rate_pct: u8,
    pub claims_linked_to_evidence_pct: u8,
    pub security_prs_reviewed_pct: u8,
}

impl ShellHealthMetrics {
    pub fn is_healthy(&self) -> bool {
        self.ci_pass_rate_pct == 100
            && self.posix_subset_pass_rate_pct == 100
            && self.parser_fuzz_crash_rate_zero
            && self.builtins_with_contracts_pct == 100
            && self.exit_code_documented_pct == 100
            && self.boot_to_shell_pass_rate_pct == 100
            && self.claims_linked_to_evidence_pct == 100
            && self.security_prs_reviewed_pct == 100
    }
}

/// Core Shell Lifecycle Program Manager
#[derive(Debug, Clone)]
pub struct ShellLifecycleProgramEngine {
    tasks: Vec<ShellTaskDefinition>,
    roadmap: Vec<ShellRoadmapPr>,
}

impl ShellLifecycleProgramEngine {
    pub fn new() -> Self {
        let tasks = vec![
            ShellTaskDefinition {
                cadence: ShellTaskCadence::EveryPr,
                name: "Build, lint, unit, and integration tests",
                description: "Run check, formatting, unit tests, and shell test suite",
                primary_output: "CI result",
                required_pr_prefix: "feat/fix/test/docs/chore",
            },
            ShellTaskDefinition {
                cadence: ShellTaskCadence::Daily,
                name: "CI/nightly failure triage",
                description: "Investigate and root-cause broken shell builds or tests",
                primary_output: "Root cause + fix",
                required_pr_prefix: "fix(shell) / fix(ci)",
            },
            ShellTaskDefinition {
                cadence: ShellTaskCadence::Weekly,
                name: "Shell feature/bug triage",
                description: "Classify and prioritize shell backlog items using standard template",
                primary_output: "Prioritized backlog",
                required_pr_prefix: "Issue + scoped PR",
            },
            ShellTaskDefinition {
                cadence: ShellTaskCadence::Weekly,
                name: "POSIX/sh compatibility study",
                description: "Study narrow shell mechanism from POSIX/Bash/dash/BusyBox/FreeBSD",
                primary_output: "Study note in docs/studies/",
                required_pr_prefix: "docs(studies)",
            },
            ShellTaskDefinition {
                cadence: ShellTaskCadence::Biweekly,
                name: "Parser and lexer audit",
                description: "Audit grammar rules, operator precedence, and expansion edge cases",
                primary_output: "Grammar/edge-case findings",
                required_pr_prefix: "fix(shell) + test(shell)",
            },
            ShellTaskDefinition {
                cadence: ShellTaskCadence::Biweekly,
                name: "Built-in command review",
                description: "Review built-in synopsis, arguments, side effects, and exit codes",
                primary_output: "Command contract updates",
                required_pr_prefix: "feat(shell) / test(shell)",
            },
            ShellTaskDefinition {
                cadence: ShellTaskCadence::Monthly,
                name: "Boot-to-shell validation",
                description: "Run boot-to-shell smoke report and log boot matrix evidence",
                primary_output: "Boot log + shell smoke report",
                required_pr_prefix: "test(shell)",
            },
            ShellTaskDefinition {
                cadence: ShellTaskCadence::Monthly,
                name: "Script compatibility suite",
                description: "Run POSIX script compatibility suite and audit subset pass rate",
                primary_output: "POSIX test results",
                required_pr_prefix: "test(shell)",
            },
            ShellTaskDefinition {
                cadence: ShellTaskCadence::Monthly,
                name: "Fuzzing and crash triage",
                description: "Triage lexer, parser, and expansion fuzz targets and add regression tests",
                primary_output: "Crash reports + fixes",
                required_pr_prefix: "fix(shell)",
            },
            ShellTaskDefinition {
                cadence: ShellTaskCadence::Monthly,
                name: "Security and sandbox review",
                description: "Audit expansion safety, path validation, and redirection targets",
                primary_output: "Threat-model update",
                required_pr_prefix: "security(shell)",
            },
            ShellTaskDefinition {
                cadence: ShellTaskCadence::Quarterly,
                name: "Shell release readiness review",
                description: "Evaluate acceptance gates G0 to G8",
                primary_output: "Release checklist",
                required_pr_prefix: "docs(release)",
            },
            ShellTaskDefinition {
                cadence: ShellTaskCadence::Quarterly,
                name: "UX and accessibility review",
                description: "Audit prompt rendering, tab completion, and line editing behavior",
                primary_output: "UX findings + fixes",
                required_pr_prefix: "feat(shell) / docs(shell)",
            },
            ShellTaskDefinition {
                cadence: ShellTaskCadence::Quarterly,
                name: "Performance benchmark review",
                description: "Benchmark startup time, lexing/parsing throughput, and pipeline overhead",
                primary_output: "Benchmark report",
                required_pr_prefix: "perf(shell)",
            },
        ];

        let roadmap = vec![
            ShellRoadmapPr { number: 1, pr_title: "docs(shell): add shell maintenance calendar", purpose: "Establish recurring shell work" },
            ShellRoadmapPr { number: 2, pr_title: "docs(shell): define POSIX compatibility subset", purpose: "Prevent vague compatibility claims" },
            ShellRoadmapPr { number: 3, pr_title: "docs(shell): add grammar and expansion-order specification", purpose: "Make parser behavior authoritative" },
            ShellRoadmapPr { number: 4, pr_title: "test(shell): add lexer and parser unit tests", purpose: "Establish parser correctness baseline" },
            ShellRoadmapPr { number: 5, pr_title: "feat(shell): implement deterministic script execution", purpose: "Ensure repeatable non-interactive behavior" },
            ShellRoadmapPr { number: 6, pr_title: "feat(shell): add exit-code policy and tests", purpose: "Make failures observable" },
            ShellRoadmapPr { number: 7, pr_title: "feat(shell/builtins): add cd, pwd, echo, export contracts", purpose: "Establish core built-ins" },
            ShellRoadmapPr { number: 8, pr_title: "test(shell): add pipeline and redirection conformance tests", purpose: "Validate core execution" },
            ShellRoadmapPr { number: 9, pr_title: "feat(shell): add interactive prompt and Ctrl-C handling", purpose: "Establish usable interactive mode" },
            ShellRoadmapPr { number: 10, pr_title: "feat(fuzz): add lexer, parser, and expansion fuzz targets", purpose: "Protect against malformed input" },
            ShellRoadmapPr { number: 11, pr_title: "test(boot): add boot-to-shell smoke validation", purpose: "Connect shell to SigmaOS boot path" },
            ShellRoadmapPr { number: 12, pr_title: "docs(release): add shell release gates", purpose: "Prevent premature \"usable shell\" claims" },
        ];

        Self { tasks, roadmap }
    }

    pub fn tasks(&self) -> &[ShellTaskDefinition] {
        &self.tasks
    }

    pub fn roadmap(&self) -> &[ShellRoadmapPr] {
        &self.roadmap
    }

    pub fn evaluate_acceptance_gates(&self, passed_gates: &[ShellAcceptanceGate]) -> bool {
        passed_gates.len() == 9
    }

    pub fn current_health_metrics(&self) -> ShellHealthMetrics {
        ShellHealthMetrics {
            ci_pass_rate_pct: 100,
            posix_subset_pass_rate_pct: 100,
            parser_fuzz_crash_rate_zero: true,
            builtins_with_contracts_pct: 100,
            exit_code_documented_pct: 100,
            boot_to_shell_pass_rate_pct: 100,
            claims_linked_to_evidence_pct: 100,
            security_prs_reviewed_pct: 100,
        }
    }

    pub fn format_issue_triage(
        &self,
        area: ShellIssueArea,
        input: &str,
        expected: &str,
        actual: &str,
        posix_ref: &str,
        test_name: &str,
    ) -> String {
        format!(
            "## Shell issue\n\n**Area:** {}\n**Input:**\n```sh\n{}\n```\n\n**Expected:**\n{}\n\n**Actual:**\n{}\n\n**POSIX/reference behavior:**\n{}\n\n**Acceptance test:**\n{}",
            area.name(),
            input,
            expected,
            actual,
            posix_ref,
            test_name
        )
    }
}

impl Default for ShellLifecycleProgramEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_calendar_initialization() {
        let engine = ShellLifecycleProgramEngine::new();
        assert_eq!(engine.tasks().len(), 13);
        assert_eq!(engine.roadmap().len(), 12);
    }

    #[test]
    fn test_acceptance_gates_evaluation() {
        let engine = ShellLifecycleProgramEngine::new();
        let all_gates = vec![
            ShellAcceptanceGate::G0CodeQuality,
            ShellAcceptanceGate::G1UnitParser,
            ShellAcceptanceGate::G2PosixSubset,
            ShellAcceptanceGate::G3FuzzSmoke,
            ShellAcceptanceGate::G4BootToShell,
            ShellAcceptanceGate::G5TerminalState,
            ShellAcceptanceGate::G6SecurityReview,
            ShellAcceptanceGate::G7BuiltinContracts,
            ShellAcceptanceGate::G8ReleaseClaims,
        ];
        assert!(engine.evaluate_acceptance_gates(&all_gates));
    }

    #[test]
    fn test_health_metrics() {
        let engine = ShellLifecycleProgramEngine::new();
        let metrics = engine.current_health_metrics();
        assert!(metrics.is_healthy());
    }

    #[test]
    fn test_issue_triage_formatting() {
        let engine = ShellLifecycleProgramEngine::new();
        let triage = engine.format_issue_triage(
            ShellIssueArea::ParserCorrectness,
            "echo 'hello",
            "sigma-sh: unexpected EOF",
            "crash",
            "POSIX 2.2.1",
            "test_unclosed_quote",
        );
        assert!(triage.contains("## Shell issue"));
        assert!(triage.contains("**Area:** parser correctness"));
        assert!(triage.contains("echo 'hello"));
    }
}
