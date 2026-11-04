// tests/test_shells_terminals_supervisors_v90_pr.rs
// Integration tests for Sovereign Shells, Terminals, Init Systems & Supervisors Parity V90

#[path = "../src/shell/sovereign_shells_terminals_supervisors_v90_pr.rs"]
mod sovereign_shells_terminals_supervisors_v90_pr;

use sovereign_shells_terminals_supervisors_v90_pr::*;

#[test]
fn test_suite_construction_v90() {
    let suite = SovereignShellsTerminalsSupervisorsV90PrSuite::new();
    assert_eq!(suite.collect_all_proposals().len(), 7);
    let gateway: SovereignShellsTerminalsSupervisorsParityV90GatewayEngine =
        SovereignShellsTerminalsSupervisorsParityV90GatewayEngine::new();
    assert_eq!(gateway.suite.collect_all_proposals().len(), 7);
}

#[test]
fn test_all_v90_proposals_integrity() {
    let gateway = SovereignShellsTerminalsSupervisorsParityV90Gateway::new();
    let proposals = gateway.suite.collect_all_proposals();

    assert_eq!(proposals.len(), 7);

    let expected_prs = [4234, 4235, 4236, 4237, 4238, 4239, 4240];
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
    assert!(markdown.contains("PR #4234"));
    assert!(markdown.contains("PR #4235"));
    assert!(markdown.contains("PR #4236"));
    assert!(markdown.contains("PR #4237"));
    assert!(markdown.contains("PR #4238"));
    assert!(markdown.contains("PR #4239"));
    assert!(markdown.contains("PR #4240"));
}

#[test]
fn test_functional_parity_engines_v90() {
    let systemd = SystemdAdvancementsV90Engine::new();
    let (valid_cg, _) = systemd.evaluate_cgroup_v2_psi_limits(1024 * 1024 * 256, 500, 1000, 75, 80);
    assert!(valid_cg);
    assert!(systemd.verify_socket_activation_with_names(
        2,
        &["web.socket", "api.socket"],
        100,
        100
    ));

    let openrc = OpenRcAdvancementsV90Engine::new();
    assert!(openrc.validate_runlevel_transition("boot", "default"));
    let conf_d = openrc.parse_conf_d_overrides("rc_parallel=YES\n");
    assert_eq!(conf_d.get("rc_parallel").map(|s| s.as_str()), Some("YES"));

    let runit_s6 = RunitS6AdvancementsV90Engine::new();
    assert!(runit_s6.verify_supervise_lock("/var/service/sshd/supervise/ok", true));
    let rot = runit_s6.configure_log_rotation("/var/log/svc", 2097152, 10);
    assert!(rot.contains("s=2097152"));

    let sysv_bsd = SysVinitBsdRcLaunchdAdvancementsV90Engine::new();
    let inittab = sysv_bsd.parse_sysvinit_inittab("si::sysinit:/etc/init.d/rcS\n");
    assert_eq!(inittab.len(), 1);

    let schedulers = CronAnacronTimersAtAdvancementsV90Engine::new();
    assert!(schedulers.parse_vixie_cron_5fields("0 12 * * *", 0, 12, 15, 6, 2));
    let timer = schedulers.solve_systemd_timer_calendar_with_delay("hourly", 1000, 60, 15);
    assert_eq!(timer, 1000 + 3600 + 15);

    let shells = PosixAndAdvancedShellsV90Engine::new();
    assert_eq!(shells.evaluate_ksh93_math_expression("10 + 20", &[]), 30);
    let (posix_ok, _) = shells.check_dash_posix_compliance("echo hello world");
    assert!(posix_ok);
    let exp = shells.evaluate_bash_parameter_expansion("${NAME:-alice}", &[]);
    assert_eq!(exp, "alice");

    let terminals = TerminalMultiplexersAndEmulatorsV90Engine::new();
    assert_eq!(
        terminals.parse_truecolor_24bit_escape("\x1b[38;2;10;20;30m"),
        Some((false, 10, 20, 30))
    );
    assert_eq!(
        terminals.parse_window_title_osc("\x1b]0;Term Title\x07"),
        Some("Term Title".to_string())
    );
}
