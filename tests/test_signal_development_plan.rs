// Integration test for SigmaOS Signal Development and Maintenance Plan
// (`tests/test_signal_development_plan.rs`)

#[path = "../src/signal/sovereign_signal_development_and_pr_suite.rs"]
mod sovereign_signal_development_and_pr_suite;

use sovereign_signal_development_and_pr_suite::*;

#[test]
fn test_signal_development_plan_19_prs() {
    let suite = SovereignSignalDevelopmentAndPrSuite::new();
    assert_eq!(suite.pr_status_map.len(), 19);

    for pr_num in 1..=19 {
        let (title, status) = suite
            .pr_status_map
            .get(&(pr_num as u32))
            .unwrap_or_else(|| panic!("PR {} missing from suite", pr_num));
        assert!(!title.is_empty());
        assert_eq!(*status, SignalMaturityStatus::Prototype);
    }
}

#[test]
fn test_signal_number_abi_constants() {
    assert_eq!(SignalNumber::SIGHUP as u32, 1);
    assert_eq!(SignalNumber::SIGINT as u32, 2);
    assert_eq!(SignalNumber::SIGQUIT as u32, 3);
    assert_eq!(SignalNumber::SIGILL as u32, 4);
    assert_eq!(SignalNumber::SIGTRAP as u32, 5);
    assert_eq!(SignalNumber::SIGABRT as u32, 6);
    assert_eq!(SignalNumber::SIGBUS as u32, 7);
    assert_eq!(SignalNumber::SIGFPE as u32, 8);
    assert_eq!(SignalNumber::SIGKILL as u32, 9);
    assert_eq!(SignalNumber::SIGUSR1 as u32, 10);
    assert_eq!(SignalNumber::SIGSEGV as u32, 11);
    assert_eq!(SignalNumber::SIGUSR2 as u32, 12);
    assert_eq!(SignalNumber::SIGPIPE as u32, 13);
    assert_eq!(SignalNumber::SIGALRM as u32, 14);
    assert_eq!(SignalNumber::SIGTERM as u32, 15);
    assert_eq!(SignalNumber::SIGCHLD as u32, 17);
    assert_eq!(SignalNumber::SIGCONT as u32, 18);
    assert_eq!(SignalNumber::SIGSTOP as u32, 19);
    assert_eq!(SignalNumber::SIGSYS as u32, 31);
    assert_eq!(SignalNumber::SIGRTMIN as u32, 32);
    assert_eq!(SignalNumber::SIGRTMAX as u32, 64);
}

#[test]
fn test_uncatchable_signals_contract() {
    assert!(SignalNumber::SIGKILL.is_uncatchable());
    assert!(SignalNumber::SIGSTOP.is_uncatchable());
    assert!(!SignalNumber::SIGTERM.is_uncatchable());
    assert!(!SignalNumber::SIGINT.is_uncatchable());

    let mut set = SigSet::full();
    set.sanitize_unblockable();
    assert!(!set.is_member(SignalNumber::SIGKILL));
    assert!(!set.is_member(SignalNumber::SIGSTOP));
}

#[test]
fn test_sigaction_uncatchable_rejection() {
    let mut suite = SovereignSignalDevelopmentAndPrSuite::new();
    let mut old_act = None;
    let new_act = SigAction::default();

    assert!(suite
        .execute_pr5_sigaction_safe_user_copy(1, SignalNumber::SIGKILL, Some(new_act), &mut old_act)
        .is_err());
    assert!(suite
        .execute_pr5_sigaction_safe_user_copy(1, SignalNumber::SIGSTOP, Some(new_act), &mut old_act)
        .is_err());
}

#[test]
fn test_complete_19_pr_pipeline_execution() {
    let mut suite = SovereignSignalDevelopmentAndPrSuite::new();
    let report_res = suite.run_all_pr_stages();
    assert!(report_res.is_ok(), "19-PR pipeline execution failed");

    let report = report_res.unwrap();
    assert!(report.contains("SigmaOS Signal Parity Maturity Matrix:"));
    assert!(report.contains("PR 01: docs(signal): define signal model and supported profile"));
    assert!(report.contains("PR 19: docs(signal): publish signal reference and maturity matrix"));
}
