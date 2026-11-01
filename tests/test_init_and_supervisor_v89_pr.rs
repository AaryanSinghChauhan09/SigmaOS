//! Integration test suite for Sovereign Init Systems & Supervisors Advancements V89 PR.

#[path = "../src/system/sovereign_init_and_supervisor_advancements_v89_pr.rs"]
mod sovereign_init_and_supervisor_advancements_v89_pr;

use sovereign_init_and_supervisor_advancements_v89_pr::*;
extern crate alloc;

#[test]
fn test_init_and_supervisor_v89_pr_integration() {
    let master_suite = SovereignInitAndSupervisorAdvancementsV89Suite::new();
    let manifests = master_suite.collect_all_manifests();

    assert_eq!(manifests.len(), 7);
    for manifest in &manifests {
        assert!(manifest.pr_id >= 8901 && manifest.pr_id <= 8907);
        assert!(!manifest.pr_title.is_empty());
        assert_eq!(manifest.status, "Ready for Integration");
    }

    // Verify systemd V89 state machine, freezer & cgroup v2
    let systemd = SystemdAdvancementsV89Engine::new();
    let state = systemd.transition_unit_state(SystemdUnitStateV89::Inactive, "start");
    assert_eq!(state, SystemdUnitStateV89::Activating);
    let state_active = systemd.transition_unit_state(state, "started");
    let state_freezing = systemd.transition_unit_state(state_active, "freeze");
    assert_eq!(state_freezing, SystemdUnitStateV89::Freezing);

    let limits = SystemdCgroupV2LimitsV89 {
        memory_max_bytes: Some(4096),
        memory_low_bytes: Some(1024),
        cpu_quota_percent: Some(100),
        tasks_max: Some(250),
        io_weight: Some(100),
        ..Default::default()
    };
    let cgroup_params = systemd.build_cgroup_v2_control_parameters(&limits);
    assert_eq!(cgroup_params.get("memory.max"), Some(&"4096".to_string()));
    assert_eq!(cgroup_params.get("memory.low"), Some(&"1024".to_string()));
    assert_eq!(cgroup_params.get("pids.max"), Some(&"250".to_string()));

    // Verify OpenRC V89 dependency solver
    let openrc = OpenRcAdvancementsV89Engine::new();
    let services = vec![
        OpenRcServiceSpecV89 {
            name: "localmount".to_string(),
            ..Default::default()
        },
        OpenRcServiceSpecV89 {
            name: "net".to_string(),
            dependencies: vec![(OpenRcDepKindV89::Need, "localmount".to_string())],
            ..Default::default()
        },
    ];
    let order = openrc.solve_runlevel_execution_order(&services).unwrap();
    assert_eq!(order, vec!["localmount", "net"]);

    // Verify s6 readiness notification
    let s6 = RunitS6AdvancementsV89Engine::new();
    let spec = S6ServiceReadinessSpecV89 {
        service_name: "testsvc".to_string(),
        notify_fd: Some(3),
        check_script: None,
        timeout_ms: 1000,
        max_restart_attempts: 3,
        down_file_present: false,
    };
    assert!(s6.evaluate_readiness_notification(&spec, Some(3), false));

    // Verify BSD rc jail chroot builder
    let bsd = BsdRcAdvancementsV89Engine::new();
    let hooks = BsdRcScriptHooksV89 {
        name: "test".to_string(),
        command: "/usr/bin/test".to_string(),
        jail_name: Some("secjail".to_string()),
        ..Default::default()
    };
    assert_eq!(bsd.build_jail_chroot_command(&hooks), "jail -n secjail exec /usr/bin/test");

    // Verify Cron Anacron timers
    let cron = CronAnacronTimersAtV89Engine::new();
    let (catchup, delay) = cron.evaluate_anacron_job_catchup(1, 5, 2);
    assert!(catchup);
    assert_eq!(delay, 5);
}
