// tests/test_shells_terminals_supervisors_v89_pr.rs
// Integration tests for Sovereign Shells, Terminals, Init Systems & Supervisors Parity V89

#[path = "../src/shell/sovereign_shells_terminals_supervisors_v89_pr.rs"]
mod sovereign_shells_terminals_supervisors_v89_pr;

use sovereign_shells_terminals_supervisors_v89_pr::*;

#[test]
fn test_suite_construction() {
    let suite = SovereignShellsTerminalsSupervisorsV89PrSuite::new();
    assert_eq!(suite.collect_all_proposals().len(), 7);
}

#[test]
fn test_all_v89_proposals_integrity() {
    let gateway = SovereignShellsTerminalsSupervisorsParityV89Gateway::new();
    let proposals = gateway.suite.collect_all_proposals();

    assert_eq!(proposals.len(), 7);

    let expected_prs = [4227, 4228, 4229, 4230, 4231, 4232, 4233];
    for (i, pr) in expected_prs.iter().enumerate() {
        assert_eq!(proposals[i].pr_number, *pr);
        assert!(!proposals[i].pr_title.is_empty());
        assert!(!proposals[i].target_subsystem.is_empty());
        assert!(!proposals[i].distro_inspirations.is_empty());
        assert!(!proposals[i].tools_covered.is_empty());
        assert!(!proposals[i].key_features_added.is_empty());
        assert_eq!(proposals[i].status, "Ready for Merge");
    }

    let summary = gateway.generate_master_summary();
    assert!(summary.contains("Total PR Proposals: 7"));

    let markdown = gateway.generate_pr_proposal_markdown();
    assert!(markdown.contains("PR #4227"));
    assert!(markdown.contains("PR #4228"));
    assert!(markdown.contains("PR #4229"));
    assert!(markdown.contains("PR #4230"));
    assert!(markdown.contains("PR #4231"));
    assert!(markdown.contains("PR #4232"));
    assert!(markdown.contains("PR #4233"));
}

#[test]
fn test_functional_parity_engines_v89() {
    let systemd = SystemdAdvancementsV89Engine::new();
    let (valid_cg, _) = systemd.evaluate_cgroup_v2_limits(1024 * 1024 * 100, 500, 1000);
    assert!(valid_cg);
    assert!(systemd.verify_socket_activation(2, 100, 100));

    let openrc = OpenRcAdvancementsV89Engine::new();
    assert!(openrc.validate_runlevel_transition("boot", "default"));

    let runit_s6 = RunitS6AdvancementsV89Engine::new();
    assert!(runit_s6.verify_supervise_lock("/var/service/sshd/supervise/ok", true));

    let sysv_bsd = SysVinitBsdRcLaunchdAdvancementsV89Engine::new();
    let inittab = sysv_bsd.parse_sysvinit_inittab("si::sysinit:/etc/init.d/rcS\n");
    assert_eq!(inittab.len(), 1);

    let schedulers = CronAnacronTimersAtAdvancementsV89Engine::new();
    assert!(schedulers.parse_vixie_cron_5fields("0 12 * * *", 0, 12, 15, 6, 2));

    let shells = PosixAndAdvancedShellsV89Engine::new();
    assert_eq!(shells.evaluate_ksh93_math_expression("10 + 20", &[]), 30);
    let (posix_ok, _) = shells.check_dash_posix_compliance("echo hello world");
    assert!(posix_ok);

    let terminals = TerminalMultiplexersAndEmulatorsV89Engine::new();
    assert_eq!(terminals.parse_truecolor_24bit_escape("\x1b[38;2;10;20;30m"), Some((10, 20, 30)));
}
