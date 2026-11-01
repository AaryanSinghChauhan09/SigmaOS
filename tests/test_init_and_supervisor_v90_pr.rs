//! Integration test suite for Sovereign Init Systems & Supervisors Advancements V90 PR.

#[path = "../src/system/sovereign_init_and_supervisor_advancements_v90_pr.rs"]
mod sovereign_init_and_supervisor_advancements_v90_pr;

use sovereign_init_and_supervisor_advancements_v90_pr::*;
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::ToString;
use alloc::vec;

#[test]
fn test_init_and_supervisor_v90_pr_integration() {
    let master_suite = SovereignInitAndSupervisorAdvancementsV90Suite::new();
    let manifests = master_suite.collect_all_manifests();

    assert_eq!(manifests.len(), 7);
    for manifest in &manifests {
        assert!(manifest.pr_id >= 9001 && manifest.pr_id <= 9007);
        assert!(!manifest.pr_title.is_empty());
        assert_eq!(manifest.status, "Ready for Integration");
    }

    // Verify systemd V90 state machine, freezer & cgroup v2
    let systemd = SystemdAdvancementsV90Engine::new();
    let state = systemd.transition_unit_state(SystemdUnitStateV90::Inactive, "start");
    assert_eq!(state, SystemdUnitStateV90::Activating);
    let state_active = systemd.transition_unit_state(state, "started");
    let state_freezing = systemd.transition_unit_state(state_active, "freeze");
    assert_eq!(state_freezing, SystemdUnitStateV90::Freezing);

    let limits = SystemdCgroupV2LimitsV90 {
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

    let notify = systemd.parse_sd_notify_message("READY=1\nSTATUS=Healthy\nMAINPID=1050");
    assert_eq!(notify.get("READY"), Some(&"1".to_string()));
    assert_eq!(notify.get("STATUS"), Some(&"Healthy".to_string()));

    let sandboxing = SystemdSandboxDirectivesV90 {
        protect_system: Some("strict".to_string()),
        protect_home: Some("read-only".to_string()),
        private_tmp: true,
        protect_kernel_tunables: true,
        protect_control_groups: true,
        restrict_namespaces: true,
        system_call_filter: vec!["@system-service".to_string()],
    };
    let flags = systemd.translate_sandboxing_directives(&sandboxing);
    assert!(flags.contains(&"protect-system:strict".to_string()));

    // Verify OpenRC V90 dependency solver & stacked runlevels
    let openrc = OpenRcAdvancementsV90Engine::new();
    let services = vec![
        OpenRcServiceSpecV90 {
            name: "localmount".to_string(),
            ..Default::default()
        },
        OpenRcServiceSpecV90 {
            name: "net".to_string(),
            dependencies: vec![(OpenRcDepKindV90::Need, "localmount".to_string())],
            ..Default::default()
        },
    ];
    let order = openrc.solve_runlevel_execution_order(&services).unwrap();
    assert_eq!(order, vec!["localmount", "net"]);

    let mut hierarchy = BTreeMap::new();
    hierarchy.insert("default".to_string(), vec!["boot".to_string()]);
    hierarchy.insert("boot".to_string(), vec!["sysinit".to_string()]);
    let stacked = openrc.solve_stacked_runlevels(&hierarchy, "default");
    assert_eq!(stacked, vec!["sysinit", "boot", "default"]);

    // Verify s6 readiness notification & logger pipe
    let s6 = RunitS6AdvancementsV90Engine::new();
    let spec = S6ServiceReadinessSpecV90 {
        service_name: "testsvc".to_string(),
        notify_fd: Some(3),
        check_script: None,
        timeout_ms: 1000,
        max_restart_attempts: 3,
        down_file_present: false,
    };
    assert!(s6.evaluate_readiness_notification(&spec, Some(3), false));
    let log_pipe = s6.build_rotating_logger_pipe("/var/log/test", 2048, 3);
    assert_eq!(log_pipe, "s6-log -b s2048 n3 /var/log/test");

    // Verify BSD rc jail chroot builder & rcorder
    let bsd = BsdRcAdvancementsV90Engine::new();
    let hooks = BsdRcScriptHooksV90 {
        name: "test".to_string(),
        command: "/usr/bin/test".to_string(),
        jail_name: Some("secjail".to_string()),
        ..Default::default()
    };
    assert_eq!(bsd.build_jail_chroot_command(&hooks), "jail -n secjail exec /usr/bin/test");

    let scripts = vec![
        ("netif", vec!["net"], vec![]),
        ("routing", vec!["routing"], vec!["net"]),
    ];
    let rc_order = bsd.rcorder_sort(&scripts).unwrap();
    assert_eq!(rc_order, vec!["netif", "routing"]);

    // Verify Cron Anacron timers & nice priorities
    let cron = CronAnacronTimersAtV90Engine::new();
    let (catchup, delay) = cron.evaluate_anacron_job_catchup(1, 5, 2);
    assert!(catchup);
    assert_eq!(delay, 5);
    assert_eq!(cron.queue_letter_to_nice_level('a'), 0);
}
