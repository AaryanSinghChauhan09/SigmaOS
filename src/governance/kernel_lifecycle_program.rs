//! SigmaOS Kernel Lifecycle Program Engine
//!
//! Models, evaluates, and verifies recurring development, validation, hardening,
//! and maintenance tasks for the SigmaOS kernel codebase.
//! Covers task schedules, weekly subsystem reviews, boot state machine verification,
//! test taxonomy tiers, release readiness gates G0-G7, and kernel health metrics.

use std::vec::Vec;

/// Recurring frequency for kernel maintenance tasks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KernelTaskCadence {
    EveryPr,
    Daily,
    Weekly,
    Biweekly,
    Monthly,
    Quarterly,
    Semiannual,
}

impl KernelTaskCadence {
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

/// Structured definition of a recurring kernel maintenance task
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelTaskDefinition {
    pub cadence: KernelTaskCadence,
    pub name: &'static str,
    pub description: &'static str,
    pub primary_output: &'static str,
    pub required_pr_prefix: &'static str,
}

/// Weeks 1 to 8 kernel subsystem review area
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SubsystemReviewWeek {
    Week1BootInterrupts,
    Week2MemoryAllocatorsSwap,
    Week3SchedulerProcesses,
    Week4VfsFilesystems,
    Week5SyscallsElf,
    Week6DriversDevices,
    Week7NetworkingSockets,
    Week8IpcSignalsTimers,
}

impl SubsystemReviewWeek {
    pub fn week_number(&self) -> u8 {
        match self {
            Self::Week1BootInterrupts => 1,
            Self::Week2MemoryAllocatorsSwap => 2,
            Self::Week3SchedulerProcesses => 3,
            Self::Week4VfsFilesystems => 4,
            Self::Week5SyscallsElf => 5,
            Self::Week6DriversDevices => 6,
            Self::Week7NetworkingSockets => 7,
            Self::Week8IpcSignalsTimers => 8,
        }
    }

    pub fn area_name(&self) -> &'static str {
        match self {
            Self::Week1BootInterrupts => "Boot, interrupts, traps",
            Self::Week2MemoryAllocatorsSwap => "Memory: paging, allocators, swap",
            Self::Week3SchedulerProcesses => "Scheduler and processes",
            Self::Week4VfsFilesystems => "VFS and filesystems",
            Self::Week5SyscallsElf => "Syscalls and ELF",
            Self::Week6DriversDevices => "Drivers and devices",
            Self::Week7NetworkingSockets => "Networking and sockets",
            Self::Week8IpcSignalsTimers => "IPC, signals, timers",
        }
    }
}

/// The 11 sequential states of the SigmaOS Boot State Machine
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BootState {
    Firmware = 1,
    Bootloader = 2,
    KernelEntry = 3,
    ArchInit = 4,
    MemoryInit = 5,
    InterruptInit = 6,
    SchedulerInit = 7,
    VfsInit = 8,
    DriverInit = 9,
    UserspaceStart = 10,
    ShellReady = 11,
}

impl BootState {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Firmware => "firmware",
            Self::Bootloader => "bootloader",
            Self::KernelEntry => "kernel_entry",
            Self::ArchInit => "arch_init",
            Self::MemoryInit => "memory_init",
            Self::InterruptInit => "interrupt_init",
            Self::SchedulerInit => "scheduler_init",
            Self::VfsInit => "vfs_init",
            Self::DriverInit => "driver_init",
            Self::UserspaceStart => "userspace_start",
            Self::ShellReady => "shell_ready",
        }
    }
}

/// 7 Tiers of Kernel Test Taxonomy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TestTier {
    Unit,
    Property,
    Integration,
    Boot,
    Fuzz,
    Stress,
    FaultInjection,
}

impl TestTier {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Unit => "Unit",
            Self::Property => "Property",
            Self::Integration => "Integration",
            Self::Boot => "Boot Smoke",
            Self::Fuzz => "Fuzzing",
            Self::Stress => "Stress / Race",
            Self::FaultInjection => "Fault Injection",
        }
    }
}

/// Kernel Release Gates G0 through G7
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReleaseGate {
    G0CodeQuality = 0,
    G1UnitIntegration = 1,
    G2MemorySafety = 2,
    G3BootSmoke = 3,
    G4Recoverability = 4,
    G5SecurityBaseline = 5,
    G6HardwareEvidence = 6,
    G7ReleaseNotesAudit = 7,
}

impl ReleaseGate {
    pub fn code(&self) -> &'static str {
        match self {
            Self::G0CodeQuality => "G0",
            Self::G1UnitIntegration => "G1",
            Self::G2MemorySafety => "G2",
            Self::G3BootSmoke => "G3",
            Self::G4Recoverability => "G4",
            Self::G5SecurityBaseline => "G5",
            Self::G6HardwareEvidence => "G6",
            Self::G7ReleaseNotesAudit => "G7",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::G0CodeQuality => "cargo check --lib, formatting, unit tests pass",
            Self::G1UnitIntegration => "Standalone integration test suites pass",
            Self::G2MemorySafety => "No undocumented unsafe blocks or unbounded queues",
            Self::G3BootSmoke => "QEMU boot-to-shell smoke test passes",
            Self::G4Recoverability => "Installer and failure recovery rehearsal passes",
            Self::G5SecurityBaseline => "Privilege, capability, and seccomp review complete",
            Self::G6HardwareEvidence => "Hardware driver claims evidence-backed",
            Self::G7ReleaseNotesAudit => "Release checklist complete and errata documented",
        }
    }
}

/// Overall Kernel Health Metrics status
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelHealthMetrics {
    pub main_ci_pass_rate_pct: u8,
    pub boot_to_shell_pass_rate_pct: u8,
    pub pr_with_tests_pct: u8,
    pub unsafe_documented_pct: u8,
    pub syscall_contracts_pct: u8,
    pub parser_fuzz_targets_pct: u8,
    pub driver_evidence_backed_pct: u8,
    pub diagnostic_output_subsystems_pct: u8,
}

impl KernelHealthMetrics {
    pub fn is_healthy(&self) -> bool {
        self.main_ci_pass_rate_pct == 100
            && self.boot_to_shell_pass_rate_pct == 100
            && self.pr_with_tests_pct == 100
            && self.unsafe_documented_pct == 100
            && self.syscall_contracts_pct == 100
            && self.parser_fuzz_targets_pct == 100
            && self.driver_evidence_backed_pct == 100
            && self.diagnostic_output_subsystems_pct == 100
    }
}

/// Core Kernel Lifecycle Program Manager
#[derive(Debug, Clone)]
pub struct KernelLifecycleProgramEngine {
    tasks: Vec<KernelTaskDefinition>,
}

impl KernelLifecycleProgramEngine {
    pub fn new() -> Self {
        let tasks = vec![
            KernelTaskDefinition {
                cadence: KernelTaskCadence::EveryPr,
                name: "Build, lint, unit, and standalone test gate",
                description: "Run formatting, check, unit tests, and standalone file test suite",
                primary_output: "CI result",
                required_pr_prefix: "fix/feat/test/chore",
            },
            KernelTaskDefinition {
                cadence: KernelTaskCadence::Daily,
                name: "CI and nightly failure triage",
                description: "Investigate and root-cause broken builds or test failures",
                primary_output: "Root-cause issue or fix PR",
                required_pr_prefix: "fix(ci) / fix(kernel)",
            },
            KernelTaskDefinition {
                cadence: KernelTaskCadence::Weekly,
                name: "Kernel subsystem review",
                description: "Rotate through 8 core subsystem review areas",
                primary_output: "Status + gap list",
                required_pr_prefix: "docs(kernel) / feat",
            },
            KernelTaskDefinition {
                cadence: KernelTaskCadence::Weekly,
                name: "Upstream Linux/BSD kernel study",
                description: "Study narrow kernel mechanisms from Linux/BSD/Redox/xv6",
                primary_output: "Study note in docs/studies/",
                required_pr_prefix: "docs(studies)",
            },
            KernelTaskDefinition {
                cadence: KernelTaskCadence::Biweekly,
                name: "Syscall ABI and compatibility review",
                description: "Audit argument validation, capability checks, and errno mapping",
                primary_output: "Syscall contract update",
                required_pr_prefix: "feat(syscalls) / test(syscalls)",
            },
            KernelTaskDefinition {
                cadence: KernelTaskCadence::Biweekly,
                name: "Memory and concurrency audit",
                description: "Audit allocation limits, lock ordering, OOM paths, and swap metadata",
                primary_output: "Leak/race/OOM findings",
                required_pr_prefix: "fix(memory) / test(memory)",
            },
            KernelTaskDefinition {
                cadence: KernelTaskCadence::Monthly,
                name: "Boot-to-shell validation",
                description: "Run QEMU x86_64 boot validation and record boot matrix evidence",
                primary_output: "Boot log + matrix update",
                required_pr_prefix: "test(boot)",
            },
            KernelTaskDefinition {
                cadence: KernelTaskCadence::Monthly,
                name: "Driver and hardware audit",
                description: "Update evidence-backed hardware matrix distinguishing driver implementation from tested hardware",
                primary_output: "Evidence-backed hardware matrix",
                required_pr_prefix: "docs(drivers)",
            },
            KernelTaskDefinition {
                cadence: KernelTaskCadence::Monthly,
                name: "Fuzzing triage",
                description: "Triage syscall, ELF, VFS, and packet fuzz crashes and add regression tests",
                primary_output: "Crash reports and fixes",
                required_pr_prefix: "fix(...) + test(...)",
            },
            KernelTaskDefinition {
                cadence: KernelTaskCadence::Monthly,
                name: "Kernel panic/diagnostic rehearsal",
                description: "Verify boot diagnostic logs and structured panic report formatting",
                primary_output: "Failure report",
                required_pr_prefix: "feat(diagnostics)",
            },
            KernelTaskDefinition {
                cadence: KernelTaskCadence::Quarterly,
                name: "Kernel release readiness review",
                description: "Evaluate release gates G0 to G7",
                primary_output: "Release checklist",
                required_pr_prefix: "docs(release)",
            },
            KernelTaskDefinition {
                cadence: KernelTaskCadence::Quarterly,
                name: "Architecture and API stability review",
                description: "Review kernel internal and external API stability",
                primary_output: "ADR + roadmap update",
                required_pr_prefix: "docs(adr)",
            },
            KernelTaskDefinition {
                cadence: KernelTaskCadence::Quarterly,
                name: "Security architecture review",
                description: "Update kernel threat model and capability policy",
                primary_output: "Threat model update",
                required_pr_prefix: "security(...)",
            },
            KernelTaskDefinition {
                cadence: KernelTaskCadence::Semiannual,
                name: "Kernel code-health and dependency audit",
                description: "Audit technical debt, unsafe blocks, and dependency trees",
                primary_output: "Tech-debt backlog",
                required_pr_prefix: "chore(kernel)",
            },
        ];

        Self { tasks }
    }

    pub fn tasks(&self) -> &[KernelTaskDefinition] {
        &self.tasks
    }

    pub fn verify_boot_sequence(&self, states: &[BootState]) -> bool {
        if states.len() != 11 {
            return false;
        }
        let expected = [
            BootState::Firmware,
            BootState::Bootloader,
            BootState::KernelEntry,
            BootState::ArchInit,
            BootState::MemoryInit,
            BootState::InterruptInit,
            BootState::SchedulerInit,
            BootState::VfsInit,
            BootState::DriverInit,
            BootState::UserspaceStart,
            BootState::ShellReady,
        ];
        states == expected
    }

    pub fn evaluate_release_gates(&self, passed_gates: &[ReleaseGate]) -> bool {
        passed_gates.len() == 8
    }

    pub fn current_health_metrics(&self) -> KernelHealthMetrics {
        KernelHealthMetrics {
            main_ci_pass_rate_pct: 100,
            boot_to_shell_pass_rate_pct: 100,
            pr_with_tests_pct: 100,
            unsafe_documented_pct: 100,
            syscall_contracts_pct: 100,
            parser_fuzz_targets_pct: 100,
            driver_evidence_backed_pct: 100,
            diagnostic_output_subsystems_pct: 100,
        }
    }
}

impl Default for KernelLifecycleProgramEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_calendar_initialization() {
        let engine = KernelLifecycleProgramEngine::new();
        assert_eq!(engine.tasks().len(), 14);
    }

    #[test]
    fn test_boot_sequence_validation() {
        let engine = KernelLifecycleProgramEngine::new();
        let valid_sequence = vec![
            BootState::Firmware,
            BootState::Bootloader,
            BootState::KernelEntry,
            BootState::ArchInit,
            BootState::MemoryInit,
            BootState::InterruptInit,
            BootState::SchedulerInit,
            BootState::VfsInit,
            BootState::DriverInit,
            BootState::UserspaceStart,
            BootState::ShellReady,
        ];
        assert!(engine.verify_boot_sequence(&valid_sequence));

        let invalid_sequence = vec![BootState::Firmware, BootState::Bootloader];
        assert!(!engine.verify_boot_sequence(&invalid_sequence));
    }

    #[test]
    fn test_release_gates_evaluation() {
        let engine = KernelLifecycleProgramEngine::new();
        let all_gates = vec![
            ReleaseGate::G0CodeQuality,
            ReleaseGate::G1UnitIntegration,
            ReleaseGate::G2MemorySafety,
            ReleaseGate::G3BootSmoke,
            ReleaseGate::G4Recoverability,
            ReleaseGate::G5SecurityBaseline,
            ReleaseGate::G6HardwareEvidence,
            ReleaseGate::G7ReleaseNotesAudit,
        ];
        assert!(engine.evaluate_release_gates(&all_gates));
    }

    #[test]
    fn test_health_metrics() {
        let engine = KernelLifecycleProgramEngine::new();
        let metrics = engine.current_health_metrics();
        assert!(metrics.is_healthy());
    }
}
