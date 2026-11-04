//! Integration test suite for Sovereign Init Systems & Supervisors Advancements V89 PR.

#[path = "../src/system/sovereign_init_and_supervisor_advancements_v89_pr.rs"]
mod sovereign_init_and_supervisor_advancements_v89_pr;

use sovereign_init_and_supervisor_advancements_v89_pr::*;
extern crate alloc;
use alloc::collections::BTreeMap;

#[test]
fn test_init_and_supervisor_v89_pr_integration() {
    let gateway = SovereignInitAndSupervisorV89PrGatewayEngine::new();
    let manifests = gateway.orchestrate_core_userland_v89().unwrap();

    assert_eq!(manifests.len(), 7);
    for manifest in &manifests {
        assert!(manifest.pr_id >= 8900 && manifest.pr_id <= 8907);
        assert!(!manifest.pr_title.is_empty());
        assert_eq!(manifest.status, "Ready for Integration");
    }

    // Verify systemd V89 state machine & cgroup v2
    let systemd = SystemdAdvancementsV89Engine::new();
    let state = systemd.transition_unit_state(SystemdUnitStateV89::Inactive, "start");
    assert_eq!(state, SystemdUnitStateV89::Activating);
    let state_active = systemd.transition_unit_state(state, "started");
    let state_failed = systemd.transition_unit_state(state_active, "crash");
    assert_eq!(state_failed, SystemdUnitStateV89::Failed);

    let limits = SystemdCgroupV2LimitsV89 {
        memory_max_bytes: Some(4096),
        memory_high_bytes: Some(1024),
        cpu_quota_percent: Some(100),
        tasks_max: Some(250),
        io_weight: Some(100),
    };
    let cgroup_params = systemd.build_cgroup_v2_control_parameters(&limits);
    assert_eq!(cgroup_params.get("memory.max"), Some(&"4096".to_string()));
    assert_eq!(cgroup_params.get("memory.high"), Some(&"1024".to_string()));
    assert_eq!(cgroup_params.get("pids.max"), Some(&"250".to_string()));

    // Verify OpenRC V89 dependency solver
    let openrc = OpenRcAdvancementsV89Engine::new();
    let services = vec![
        OpenRcServiceSpecV89 {
            name: "localmount".to_string(),
            runlevel: "boot".to_string(),
            dependencies: vec![],
            conf_overrides: BTreeMap::new(),
        },
        OpenRcServiceSpecV89 {
            name: "net".to_string(),
            runlevel: "default".to_string(),
            dependencies: vec![(OpenRcDepKindV89::Need, "localmount".to_string())],
            conf_overrides: BTreeMap::new(),
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
    };
    assert!(s6.evaluate_readiness_notification(&spec, Some(3), false));

    // Verify BSD rc jail chroot builder
    let bsd = BsdRcAdvancementsV89Engine::new();
    let hooks = BsdRcScriptHooksV89 {
        name: "test".to_string(),
        rcvar: "test_enable".to_string(),
        command: "/usr/bin/test".to_string(),
        command_args: None,
        start_precmd: None,
        start_postcmd: None,
        jail_name: Some("secjail".to_string()),
        pledge_promises: None,
        unveil_paths: vec![],
    };
    assert_eq!(
        bsd.build_jail_chroot_command(&hooks),
        "jail -n secjail exec /usr/bin/test"
    );

    // Verify Cron timers & load average
    let cron = CronAnacronTimersAtV89Engine::new();
    let timer = SystemdTimerSpecV89 {
        on_calendar: None,
        on_boot_sec: Some(100),
        on_unit_active_sec: Some(300),
        persistent: true,
        accuracy_sec: 1,
    };
    assert!(cron.evaluate_timer_readiness(&timer, 1000, 0, 500));
}
