// tests/test_shells_terminals_supervisors_v91_pr.rs
// Integration tests for Sovereign Shells, Terminals, Init Systems & Supervisors Parity V91

#[path = "../src/shell/sovereign_shells_terminals_supervisors_v91_pr.rs"]
mod sovereign_shells_terminals_supervisors_v91_pr;

use sovereign_shells_terminals_supervisors_v91_pr::*;

#[test]
fn test_suite_construction_v91() {
    let suite = SovereignShellsTerminalsSupervisorsV91PrSuite::new();
    assert_eq!(suite.collect_all_proposals().len(), 7);
    let gateway: SovereignShellsTerminalsSupervisorsParityV91GatewayEngine =
        SovereignShellsTerminalsSupervisorsParityV91GatewayEngine::new();
    assert_eq!(gateway.suite.collect_all_proposals().len(), 7);
}

#[test]
fn test_all_v91_proposals_integrity() {
    let gateway = SovereignShellsTerminalsSupervisorsParityV91Gateway::new();
    let proposals = gateway.suite.collect_all_proposals();

    assert_eq!(proposals.len(), 7);

    let expected_prs = [4241, 4242, 4243, 4244, 4245, 4246, 4247];
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
    assert!(markdown.contains("PR #4241"));
    assert!(markdown.contains("PR #4242"));
    assert!(markdown.contains("PR #4243"));
    assert!(markdown.contains("PR #4244"));
    assert!(markdown.contains("PR #4245"));
    assert!(markdown.contains("PR #4246"));
    assert!(markdown.contains("PR #4247"));
}

#[test]
fn test_functional_parity_engines_v91() {
    let systemd = SystemdAdvancementsV91Engine::new();
    let (valid_cg, _) = systemd.evaluate_cgroup_v2_psi_limits(1024 * 1024 * 512, 600, 2000, 70, 75);
    assert!(valid_cg);
    assert!(systemd.verify_socket_activation_with_names(
        2,
        &["web.socket", "api.socket"],
        100,
        100
    ));

    let openrc = OpenRcAdvancementsV91Engine::new();
    assert!(openrc.validate_runlevel_transition("boot", "default"));
    let conf_d = openrc.parse_conf_d_overrides("rc_parallel=YES\nrc_logger=YES\n");
    assert_eq!(conf_d.get("rc_parallel").map(|s| s.as_str()), Some("YES"));
    assert_eq!(conf_d.get("rc_logger").map(|s| s.as_str()), Some("YES"));

    let runit_s6 = RunitS6AdvancementsV91Engine::new();
    assert!(runit_s6.verify_supervise_lock("/var/service/sshd/supervise/ok", true));
    let rot = runit_s6.configure_log_rotation("/var/log/svc", 2097152, 10);
    assert!(rot.contains("s=2097152"));

    let sysv_bsd = SysVinitBsdRcLaunchdAdvancementsV91Engine::new();
    let inittab = sysv_bsd.parse_sysvinit_inittab("si::sysinit:/etc/init.d/rcS\n");
    assert_eq!(inittab.len(), 1);

    let schedulers = CronAnacronTimersAtAdvancementsV91Engine::new();
    assert!(schedulers.parse_vixie_cron_5fields("0 12 * * *", 0, 12, 15, 6, 2));
    let timer = schedulers.solve_systemd_timer_calendar_with_delay("hourly", 1000, 60, 15);
    assert_eq!(timer, 1000 + 3600 + 15);

    let shells = PosixAndAdvancedShellsV91Engine::new();
    assert_eq!(shells.evaluate_ksh93_math_expression("15 + 25", &[]), 40);
    let (posix_ok, _) = shells.check_dash_posix_compliance("echo hello world");
    assert!(posix_ok);
    let exp = shells.evaluate_bash_parameter_expansion("${NAME:-bob}", &[]);
    assert_eq!(exp, "bob");

    let terminals = TerminalMultiplexersAndEmulatorsV91Engine::new();
    assert_eq!(
        terminals.parse_truecolor_24bit_escape("\x1b[38;2;15;25;35m"),
        Some((false, 15, 25, 35))
    );
    assert_eq!(
        terminals.parse_window_title_osc("\x1b]0;Term Title V91\x07"),
        Some("Term Title V91".to_string())
    );
}
