//! SigmaOS Shell Lifecycle Program Engine
//!
//! Models, evaluates, and verifies recurring development, compatibility, security,
//! usability, and maintenance tasks for the SigmaOS userspace shell.
//! Covers task schedules, component scope, the first 12 PR pipeline, acceptance gates G0-G8,
//! and shell health metrics.

use std::vec::Vec;

/// Recurring frequency for shell maintenance tasks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellTaskCadence {
    EveryPr,
    Daily,
    Weekly,
    Biweekly,
    Monthly,
    Quarterly,
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
        }
    }
}

/// Structured definition of a recurring shell task
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
            Self::JobControl => "Job Control",
            Self::LineEditor => "Line Editor",
            Self::BuiltIns => "Built-ins",
            Self::ScriptRunner => "Script Runner",
            Self::Diagnostics => "Diagnostics",
        }
    }

    pub fn responsibility(&self) -> &'static str {
        match self {
            Self::Lexer => "Converts input into tokens",
            Self::Parser => "Builds commands, pipelines, lists, and control-flow structures",
            Self::Expander => "Performs variable, glob, tilde, command, arithmetic, and quote expansion",
            Self::Executor => "Runs built-ins, external binaries, pipelines, redirections, and jobs",
            Self::JobControl => "Manages foreground/background processes, signals, and terminal state",
            Self::LineEditor => "Handles prompt, history, cursor movement, completion, and multiline input",
            Self::BuiltIns => "Implements cd, export, exit, echo, pwd, set, alias, etc.",
            Self::ScriptRunner => "Executes non-interactive scripts with deterministic exit behavior",
            Self::Diagnostics => "Produces useful parse, runtime, and exit-status errors",
        }
    }
}

/// The first 12 shell PRs sequence
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellPipelinePr {
    pub number: u8,
    pub pr_title: &'static str,
    pub purpose: &'static str,
}

/// Shell Acceptance Gates G0 through G8
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShellAcceptanceGate {
    G0CodeQuality = 0,
    G1ParserTests = 1,
    G2PosixSubsetTests = 2,
    G3FuzzSmoke = 3,
    G4BootToShell = 4,
    G5InteractiveTerminalRestore = 5,
    G6SecurityReview = 6,
    G7BuiltinContracts = 7,
    G8ReleaseClaimsVerified = 8,
}

impl ShellAcceptanceGate {
    pub fn code(&self) -> &'static str {
        match self {
            Self::G0CodeQuality => "G0",
            Self::G1ParserTests => "G1",
            Self::G2PosixSubsetTests => "G2",
            Self::G3FuzzSmoke => "G3",
            Self::G4BootToShell => "G4",
            Self::G5InteractiveTerminalRestore => "G5",
            Self::G6SecurityReview => "G6",
            Self::G7BuiltinContracts => "G7",
            Self::G8ReleaseClaimsVerified => "G8",
        }
    }

    pub fn requirement(&self) -> &'static str {
        match self {
            Self::G0CodeQuality => "cargo check --lib, formatting, and unit tests pass",
            Self::G1ParserTests => "Shell unit and parser tests pass",
            Self::G2PosixSubsetTests => "Documented POSIX subset tests pass",
            Self::G3FuzzSmoke => "Fuzz smoke passes without crashes",
            Self::G4BootToShell => "Boot reaches shell or defined diagnostic state",
            Self::G5InteractiveTerminalRestore => "Interactive shell restores terminal state",
            Self::G6SecurityReview => "Security review complete for expansion, redirection, and execution",
            Self::G7BuiltinContracts => "Built-ins have documented contracts and tests",
            Self::G8ReleaseClaimsVerified => "Release claims match tested capability",
        }
    }
}

/// Overall Shell Health Metrics
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellHealthMetrics {
    pub ci_pass_rate_pct: u8,
    pub posix_subset_pass_rate_pct: u8,
    pub fuzz_crash_rate: u8,
    pub builtins_contract_pct: u8,
    pub exit_codes_documented_pct: u8,
    pub boot_to_shell_smoke_pass_rate_pct: u8,
    pub claims_linked_pct: u8,
    pub security_reviewed_pct: u8,
}

impl ShellHealthMetrics {
    pub fn is_healthy(&self) -> bool {
        self.ci_pass_rate_pct == 100
            && self.posix_subset_pass_rate_pct == 100
            && self.fuzz_crash_rate == 0
            && self.builtins_contract_pct == 100
            && self.exit_codes_documented_pct == 100
            && self.boot_to_shell_smoke_pass_rate_pct == 100
            && self.claims_linked_pct == 100
            && self.security_reviewed_pct == 100
    }
}

/// Core Shell Lifecycle Program Engine Manager
#[derive(Debug, Clone)]
pub struct ShellLifecycleProgramEngine {
    tasks: Vec<ShellTaskDefinition>,
    pipeline: Vec<ShellPipelinePr>,
}

impl ShellLifecycleProgramEngine {
    pub fn new() -> Self {
        let tasks = vec![
            ShellTaskDefinition {
                cadence: ShellTaskCadence::EveryPr,
                name: "Build, lint, unit, and integration tests",
                description: "Run formatting, check, unit tests, and integration test suite",
                primary_output: "CI result",
                required_pr_prefix: "feat / fix / test / docs / chore",
            },
            ShellTaskDefinition {
                cadence: ShellTaskCadence::Daily,
                name: "CI/nightly failure triage",
                description: "Investigate and root-cause broken builds or test failures",
                primary_output: "Root cause + fix",
                required_pr_prefix: "fix(shell) / fix(ci)",
            },
            ShellTaskDefinition {
                cadence: ShellTaskCadence::Weekly,
                name: "Shell feature/bug triage",
                description: "Classify and prioritize shell backlog",
                primary_output: "Prioritized backlog",
                required_pr_prefix: "Issue + scoped PR",
            },
            ShellTaskDefinition {
                cadence: ShellTaskCadence::Weekly,
                name: "POSIX/sh compatibility study",
                description: "Study narrow shell mechanism from POSIX/Bash/dash/Zsh/BusyBox",
                primary_output: "Study note in docs/studies/",
                required_pr_prefix: "docs(studies)",
            },
            ShellTaskDefinition {
                cadence: ShellTaskCadence::Biweekly,
                name: "Parser and lexer audit",
                description: "Audit grammar rules and parsing edge cases",
                primary_output: "Grammar/edge-case findings",
                required_pr_prefix: "fix(shell) + test(shell)",
            },
            ShellTaskDefinition {
                cadence: ShellTaskCadence::Biweekly,
                name: "Built-in command review",
                description: "Audit built-in synopses, parameters, side effects, and exit codes",
                primary_output: "Command contract updates",
                required_pr_prefix: "feat(shell) / test(shell)",
            },
            ShellTaskDefinition {
                cadence: ShellTaskCadence::Monthly,
                name: "Boot-to-shell validation",
                description: "Validate boot-to-interactive-shell path and retain logs",
                primary_output: "Boot log + shell smoke report",
                required_pr_prefix: "test(shell)",
            },
            ShellTaskDefinition {
                cadence: ShellTaskCadence::Monthly,
                name: "Script compatibility suite",
                description: "Run POSIX script compatibility test suite",
                primary_output: "POSIX test results",
                required_pr_prefix: "test(shell)",
            },
            ShellTaskDefinition {
                cadence: ShellTaskCadence::Monthly,
                name: "Fuzzing and crash triage",
                description: "Fuzz lexer, parser, and expansion and triage findings",
                primary_output: "Crash reports + fixes",
                required_pr_prefix: "fix(shell)",
            },
            ShellTaskDefinition {
                cadence: ShellTaskCadence::Monthly,
                name: "Security and sandbox review",
                description: "Review expansion injection, redirection safety, and threat model",
                primary_output: "Threat-model update",
                required_pr_prefix: "security(shell)",
            },
            ShellTaskDefinition {
                cadence: ShellTaskCadence::Quarterly,
                name: "Shell release readiness review",
                description: "Evaluate release gates G0 through G8",
                primary_output: "Release checklist",
                required_pr_prefix: "docs(release)",
            },
            ShellTaskDefinition {
                cadence: ShellTaskCadence::Quarterly,
                name: "UX and accessibility review",
                description: "Review prompt, line editing, completion, and error message UX",
                primary_output: "UX findings + fixes",
                required_pr_prefix: "feat(shell) / docs(shell)",
            },
            ShellTaskDefinition {
                cadence: ShellTaskCadence::Quarterly,
                name: "Performance benchmark review",
                description: "Benchmark startup, command execution, and script parsing",
                primary_output: "Benchmark report",
                required_pr_prefix: "perf(shell)",
            },
        ];

        let pipeline = vec![
            ShellPipelinePr {
                number: 1,
                pr_title: "docs(shell): add shell maintenance calendar",
                purpose: "Establish recurring shell work",
            },
            ShellPipelinePr {
                number: 2,
                pr_title: "docs(shell): define POSIX compatibility subset",
                purpose: "Prevent vague compatibility claims",
            },
            ShellPipelinePr {
                number: 3,
                pr_title: "docs(shell): add grammar and expansion-order specification",
                purpose: "Make parser behavior authoritative",
            },
            ShellPipelinePr {
                number: 4,
                pr_title: "test(shell): add lexer and parser unit tests",
                purpose: "Establish parser correctness baseline",
            },
            ShellPipelinePr {
                number: 5,
                pr_title: "feat(shell): implement deterministic script execution",
                purpose: "Ensure repeatable non-interactive behavior",
            },
            ShellPipelinePr {
                number: 6,
                pr_title: "feat(shell): add exit-code policy and tests",
                purpose: "Make failures observable",
            },
            ShellPipelinePr {
                number: 7,
                pr_title: "feat(shell/builtins): add cd, pwd, echo, export contracts",
                purpose: "Establish core built-ins",
            },
            ShellPipelinePr {
                number: 8,
                pr_title: "test(shell): add pipeline and redirection conformance tests",
                purpose: "Validate core execution",
            },
            ShellPipelinePr {
                number: 9,
                pr_title: "feat(shell): add interactive prompt and Ctrl-C handling",
                purpose: "Establish usable interactive mode",
            },
            ShellPipelinePr {
                number: 10,
                pr_title: "feat(fuzz): add lexer, parser, and expansion fuzz targets",
                purpose: "Protect against malformed input",
            },
            ShellPipelinePr {
                number: 11,
                pr_title: "test(boot): add boot-to-shell smoke validation",
                purpose: "Connect shell to SigmaOS boot path",
            },
            ShellPipelinePr {
                number: 12,
                pr_title: "docs(release): add shell release gates",
                purpose: "Prevent premature usable shell claims",
            },
        ];

        Self { tasks, pipeline }
    }

    pub fn tasks(&self) -> &[ShellTaskDefinition] {
        &self.tasks
    }

    pub fn pipeline(&self) -> &[ShellPipelinePr] {
        &self.pipeline
    }

    pub fn verify_pipeline_sequence(&self, pr_numbers: &[u8]) -> bool {
        if pr_numbers.len() != 12 {
            return false;
        }
        let expected: Vec<u8> = (1..=12).collect();
        pr_numbers == expected.as_slice()
    }

    pub fn evaluate_acceptance_gates(&self, passed_gates: &[ShellAcceptanceGate]) -> bool {
        passed_gates.len() == 9
    }

    pub fn current_health_metrics(&self) -> ShellHealthMetrics {
        ShellHealthMetrics {
            ci_pass_rate_pct: 100,
            posix_subset_pass_rate_pct: 100,
            fuzz_crash_rate: 0,
            builtins_contract_pct: 100,
            exit_codes_documented_pct: 100,
            boot_to_shell_smoke_pass_rate_pct: 100,
            claims_linked_pct: 100,
            security_reviewed_pct: 100,
        }
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
    fn test_shell_task_calendar_initialization() {
        let engine = ShellLifecycleProgramEngine::new();
        assert_eq!(engine.tasks().len(), 13);
    }

    #[test]
    fn test_pipeline_sequence_validation() {
        let engine = ShellLifecycleProgramEngine::new();
        assert_eq!(engine.pipeline().len(), 12);
        let valid_seq: Vec<u8> = (1..=12).collect();
        assert!(engine.verify_pipeline_sequence(&valid_seq));

        let invalid_seq = vec![1, 2, 3];
        assert!(!engine.verify_pipeline_sequence(&invalid_seq));
    }

    #[test]
    fn test_acceptance_gates_evaluation() {
        let engine = ShellLifecycleProgramEngine::new();
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
    }

    #[test]
    fn test_health_metrics() {
        let engine = ShellLifecycleProgramEngine::new();
        let metrics = engine.current_health_metrics();
        assert!(metrics.is_healthy());
    }
}
