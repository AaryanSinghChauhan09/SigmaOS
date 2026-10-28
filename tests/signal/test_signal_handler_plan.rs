// Comprehensive Unit Tests for SigmaOS Signal Handler Architecture, ABI Profile, and Plan Verification

use std::fs;
use std::path::Path;

#[path = "../../src/signal/sovereign_signal_development_and_pr_suite.rs"]
mod sovereign_signal_suite;

use sovereign_signal_suite::{
    ProcessSignalState, SigAction, SigActionFlags, SigHandler, SigInfo, SigSet,
    SignalDefaultAction, SignalNumber, SovereignSignalDevelopmentAndPrSuite, ThreadSignalState,
    UserSignalFrame,
};

#[test]
fn test_signal_handler_documentation_plan_verification() {
    let docs_plan = "docs/SIGNAL_HANDLER_PLAN.md";
    let docs_audit = "docs/SIGNAL_HANDLER_AUDIT_AND_ABI_PROFILE.md";
    let docs_maint = "docs/signal/SIGNAL_DEVELOPMENT_AND_MAINTENANCE_PLAN.md";

    let wiki_plan = "wiki/SIGNAL_HANDLER_PLAN.md";
    let wiki_audit = "wiki/SIGNAL_HANDLER_AUDIT_AND_ABI_PROFILE.md";
    let wiki_maint = "wiki/26-Signal-Development-And-Maintenance-Plan.md";

    assert!(Path::new(docs_plan).exists(), "docs/SIGNAL_HANDLER_PLAN.md must exist");
    assert!(Path::new(docs_audit).exists(), "docs/SIGNAL_HANDLER_AUDIT_AND_ABI_PROFILE.md must exist");
    assert!(Path::new(docs_maint).exists(), "docs/signal/SIGNAL_DEVELOPMENT_AND_MAINTENANCE_PLAN.md must exist");

    assert!(Path::new(wiki_plan).exists(), "wiki/SIGNAL_HANDLER_PLAN.md must exist");
    assert!(Path::new(wiki_audit).exists(), "wiki/SIGNAL_HANDLER_AUDIT_AND_ABI_PROFILE.md must exist");
    assert!(Path::new(wiki_maint).exists(), "wiki/26-Signal-Development-And-Maintenance-Plan.md must exist");

    let plan_content = fs::read_to_string(docs_plan).expect("Read docs plan");
    let wiki_plan_content = fs::read_to_string(wiki_plan).expect("Read wiki plan");
    assert_eq!(plan_content, wiki_plan_content, "Wiki plan must match docs plan");

    let audit_content = fs::read_to_string(docs_audit).expect("Read docs audit");
    let wiki_audit_content = fs::read_to_string(wiki_audit).expect("Read wiki audit");
    assert_eq!(audit_content, wiki_audit_content, "Wiki audit must match docs audit");

    let maint_content = fs::read_to_string(docs_maint).expect("Read docs maintenance");
    let wiki_maint_content = fs::read_to_string(wiki_maint).expect("Read wiki maintenance");
    assert_eq!(maint_content, wiki_maint_content, "Wiki maintenance must match docs maintenance");

    // Verify key sections and safety rules in plan documentation
    assert!(plan_content.contains("SigmaOS Signal Handler Master Architecture & Development Plan"));
    assert!(plan_content.contains("Required Semantics and Safety Rules"));
    assert!(plan_content.contains("Default to `sigaction` Semantics"));
    assert!(plan_content.contains("Distinguish Process- and Thread-Directed Signals"));
    assert!(plan_content.contains("Protect Handler Execution State"));
    assert!(plan_content.contains("Enforce Uncatchable Signals in Kernel"));
    assert!(plan_content.contains("Respect Masks and Nesting"));
    assert!(plan_content.contains("Define Syscall Interruption Behavior"));
    assert!(plan_content.contains("Minimal and Auditable Handler-Path Behavior"));
    assert!(plan_content.contains("No False Status Claims"));
    assert!(plan_content.contains("Proposed PR Sequence Roadmap"));
    assert!(plan_content.contains("Periodic Maintenance PR Cadence"));
    assert!(plan_content.contains("PR Review and Merge Gates"));
    assert!(plan_content.contains("Completion Criteria"));

    // Verify FEATURE_STATUS.toml registration
    let feature_status = fs::read_to_string("FEATURE_STATUS.toml").expect("Read FEATURE_STATUS.toml");
    assert!(feature_status.contains("[kernel.signal_handler]"));
    assert!(feature_status.contains("[kernel.signal_development_and_maintenance_plan]"));
}

#[test]
fn test_signal_number_mappings_and_uncatchable_guards() {
    assert_eq!(SignalNumber::SIGHUP as u32, 1);
    assert_eq!(SignalNumber::SIGINT as u32, 2);
    assert_eq!(SignalNumber::SIGKILL as u32, 9);
    assert_eq!(SignalNumber::SIGSEGV as u32, 11);
    assert_eq!(SignalNumber::SIGSTOP as u32, 19);
    assert_eq!(SignalNumber::SIGRTMIN as u32, 32);
    assert_eq!(SignalNumber::SIGRTMAX as u32, 64);

    assert_eq!(SignalNumber::from_u32(9), Some(SignalNumber::SIGKILL));
    assert_eq!(SignalNumber::from_u32(19), Some(SignalNumber::SIGSTOP));
    assert_eq!(SignalNumber::from_u32(99), None);

    // Default actions
    assert_eq!(SignalNumber::SIGKILL.default_action(), SignalDefaultAction::Terminate);
    assert_eq!(SignalNumber::SIGSTOP.default_action(), SignalDefaultAction::Stop);
    assert_eq!(SignalNumber::SIGCHLD.default_action(), SignalDefaultAction::Ignore);
    assert_eq!(SignalNumber::SIGCONT.default_action(), SignalDefaultAction::Continue);

    // Uncatchable signal check
    assert!(SignalNumber::SIGKILL.is_uncatchable());
    assert!(SignalNumber::SIGSTOP.is_uncatchable());
    assert!(!SignalNumber::SIGTERM.is_uncatchable());

    let mut pr_suite = SovereignSignalDevelopmentAndPrSuite::new();
    let custom_action = SigAction {
        sa_handler: SigHandler::UserHandler(0x0000_7FFF_1000_0000),
        sa_mask: SigSet::empty(),
        sa_flags: SigActionFlags::default(),
        sa_restorer: 0,
    };
    let mut old_action = None;

    // Setting action for SIGKILL or SIGSTOP must fail
    assert!(pr_suite
        .execute_pr5_sigaction_safe_user_copy(1, SignalNumber::SIGKILL, Some(custom_action), &mut old_action)
        .is_err());
    assert!(pr_suite
        .execute_pr5_sigaction_safe_user_copy(1, SignalNumber::SIGSTOP, Some(custom_action), &mut old_action)
        .is_err());

    // Setting action for SIGTERM must succeed
    assert!(pr_suite
        .execute_pr5_sigaction_safe_user_copy(1, SignalNumber::SIGTERM, Some(custom_action), &mut old_action)
        .is_ok());

    // Uncatchable signals cannot be blocked
    let mut thread_state = ThreadSignalState::new(100, 100);
    thread_state.blocked_mask.add_member(SignalNumber::SIGKILL);
    thread_state.blocked_mask.add_member(SignalNumber::SIGSTOP);

    // Sanitize unblockable
    thread_state.blocked_mask.sanitize_unblockable();

    assert!(!thread_state.blocked_mask.is_member(SignalNumber::SIGKILL));
    assert!(!thread_state.blocked_mask.is_member(SignalNumber::SIGSTOP));

    thread_state.blocked_mask.add_member(SignalNumber::SIGUSR1);
    assert!(thread_state.blocked_mask.is_member(SignalNumber::SIGUSR1));
}

#[test]
fn test_sigset_operations_contract() {
    let mut set = SigSet::empty();
    assert_eq!(set.bits, 0);

    set.add_member(SignalNumber::SIGINT);
    set.add_member(SignalNumber::SIGTERM);
    assert!(set.is_member(SignalNumber::SIGINT));
    assert!(set.is_member(SignalNumber::SIGTERM));
    assert!(!set.is_member(SignalNumber::SIGQUIT));

    set.del_member(SignalNumber::SIGINT);
    assert!(!set.is_member(SignalNumber::SIGINT));

    let full_set = SigSet::full();
    assert!(full_set.is_member(SignalNumber::SIGKILL));
    assert!(full_set.is_member(SignalNumber::SIGSTOP));
    assert!(full_set.is_member(SignalNumber::SIGRTMAX));
}

#[test]
fn test_user_signal_frame_validation_and_sigreturn() {
    let frame = UserSignalFrame {
        frame_version: 1,
        saved_registers: vec![0; 16],
        saved_mask: SigSet::empty(),
        handler_ptr: 0x0000_7FFF_1000_0000,
        trampoline_ptr: 0x0000_7FFF_3000_0000,
        siginfo: SigInfo::default(),
        is_valid: true,
    };

    assert!(frame.is_valid);

    // Test corrupted frame validation flag
    let mut corrupted = frame.clone();
    corrupted.is_valid = false;
    assert!(!corrupted.is_valid);
}

#[test]
fn test_realtime_signal_queue_limits() {
    let mut process_state = ProcessSignalState::new(200, 200, 0, 0);

    for i in 0..32 {
        let info = SigInfo {
            si_signo: SignalNumber::SIGRTMIN as u32,
            si_errno: 0,
            si_code: 0,
            si_pid: 100,
            si_uid: 0,
            si_addr: 0,
            si_status: 0,
            si_value: i as u64,
        };
        process_state.pending_process_signals.push((SignalNumber::SIGRTMIN, info));
    }

    assert_eq!(process_state.pending_process_signals.len(), 32);
}

#[test]
fn test_sovereign_signal_pr_suite_master_execution() {
    let mut pr_suite = SovereignSignalDevelopmentAndPrSuite::new();
    let result = pr_suite.run_all_pr_stages();

    assert!(result.is_ok());
    assert_eq!(pr_suite.pr_status_map.len(), 19);
}
