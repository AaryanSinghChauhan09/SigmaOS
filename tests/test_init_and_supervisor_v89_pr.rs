//! Integration test suite for Sovereign Init Systems & Supervisors Advancements V89 PR.

#[path = "../src/system/sovereign_init_and_supervisor_advancements_v89_pr.rs"]
mod sovereign_init_and_supervisor_advancements_v89_pr;

use sovereign_init_and_supervisor_advancements_v89_pr::*;
extern crate alloc;
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::ToString;
use alloc::vec;

#[test]
fn test_init_and_supervisor_v89_pr_integration() {
    let gateway = SovereignInitAndSupervisorV89PrGatewayEngine::new();
    let manifests = gateway.orchestrate_core_userland_v89().unwrap();

    assert_eq!(manifests.len(), 7);
    for manifest in &manifests {
        assert!(manifest.pr_id >= 8900 && manifest.pr_id <= 8906);
        assert!(!manifest.pr_title.is_empty());
        assert_eq!(manifest.status, "Ready for Integration");
    }

    // Verify systemd V89 state machine & cgroup v2
    let systemd = SystemdAdvancementsV89Engine::new();
    let state = systemd.transition_unit_state(SystemdUnitStateV89::Inactive, "start");
    assert_eq!(state, SystemdUnitStateV89::Activating);
    let state_active = systemd.transition_unit_state(state, "started");
    assert_eq!(state_active, SystemdUnitStateV89::Active);

    let limits = SystemdCgroupV2LimitsV89 {
        memory_max_bytes: Some(4096),
        memory_high_bytes: Some(1024),
        cpu_quota_percent: Some(100),
        tasks_max: Some(250),
        io_weight: Some(100),
    };
    let cgroup_params = systemd.build_cgroup_v2_control_parameters(&limits);
    assert_eq!(cgroup_params.get("memory.max"), Some(&"4096".to_string()));
    assert_eq!(cgroup_params.get("pids.max"), Some(&"250".to_string()));

    // Verify OpenRC V89 topological solver
    let openrc = OpenRcAdvancementsV89Engine::new();
    let svc1 = OpenRcServiceSpecV89 {
        name: "net".to_string(),
        runlevel: "default".to_string(),
        dependencies: vec![],
        conf_overrides: BTreeMap::new(),
    };
    let svc2 = OpenRcServiceSpecV89 {
        name: "sshd".to_string(),
        runlevel: "default".to_string(),
        dependencies: vec![(OpenRcDepKindV89::Need, "net".to_string())],
        conf_overrides: BTreeMap::new(),
    };
    let order = openrc.solve_runlevel_execution_order(&[svc2, svc1]).unwrap();
    assert_eq!(order, vec!["net".to_string(), "sshd".to_string()]);

    // Verify Runit & s6 V89
    let s6 = RunitS6AdvancementsV89Engine::new();
    let readiness = S6ServiceReadinessSpecV89 {
        service_name: "app".to_string(),
        notify_fd: Some(3),
        check_script: None,
        timeout_ms: 5000,
        max_restart_attempts: 5,
    };
    assert!(s6.evaluate_readiness_notification(&readiness, Some(3), false));
    assert!(!s6.evaluate_readiness_notification(&readiness, Some(4), false));

    // Verify SysVinit V89
    let sysv = SysVinitAdvancementsV89Engine::new();
    let line = "si::sysinit:/etc/init.d/rcS";
    let spec = sysv.parse_inittab_line(line).unwrap();
    assert_eq!(spec.id, "si");

    // Verify BSD rc.d V89
    let bsd = BsdRcAdvancementsV89Engine::new();
    let script = "#!/bin/sh\nname=\"test\"\nrcvar=\"test_enable\"\n";
    let parsed = bsd.parse_rc_d_script_hooks(script);
    assert_eq!(parsed.name, "test");
    assert_eq!(parsed.rcvar, "test_enable");

    // Verify Launchd & Android init V89
    let launchd = LaunchdAndroidInitAdvancementsV89Engine::new();
    let trig = launchd.parse_android_trigger_condition("on early-init");
    assert_eq!(trig, Some(("event".to_string(), "early-init".to_string())));

    // Verify Cron & Timers V89
    let cron = CronAnacronTimersAtV89Engine::new();
    let cron_parts = cron.parse_crontab_expression("0 12 * * *").unwrap();
    assert_eq!(cron_parts.len(), 5);
}
