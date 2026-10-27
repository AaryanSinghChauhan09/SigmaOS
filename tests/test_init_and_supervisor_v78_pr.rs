//! Integration test suite for Sovereign Init Systems & Supervisors Advancements V78 PR.

#[path = "../src/system/sovereign_init_and_supervisor_advancements_v78_pr.rs"]
mod sovereign_init_and_supervisor_advancements_v78_pr;

use sovereign_init_and_supervisor_advancements_v78_pr::*;

#[test]
fn test_init_and_supervisor_v78_pr_integration() {
    let master_suite = SovereignInitAndSupervisorAdvancementsV78Suite::new();
    let manifests = master_suite.collect_all_manifests();

    assert_eq!(manifests.len(), 7);
    for manifest in &manifests {
        assert!(manifest.pr_id >= 4192 && manifest.pr_id <= 4198);
        assert!(!manifest.pr_title.is_empty());
        assert_eq!(manifest.status, "Ready for Integration");
    }

    // Verify systemd V78 state machine & cgroup v2
    let systemd = SystemdAdvancementsV78Engine::new();
    let state = systemd.transition_unit_state(SystemdUnitStateV78::Inactive, "start");
    assert_eq!(state, SystemdUnitStateV78::Activating);

    let limits = SystemdCgroupV2LimitsV78 {
        memory_max_bytes: Some(4096),
        memory_high_bytes: None,
        cpu_quota_percent: Some(100),
        tasks_max: Some(250),
        io_weight: Some(100),
    };
    let cgroup_params = systemd.build_cgroup_v2_control_parameters(&limits);
    assert_eq!(cgroup_params.get("memory.max"), Some(&"4096".to_string()));
    assert_eq!(cgroup_params.get("pids.max"), Some(&"250".to_string()));

    // Verify OpenRC V78 dependency solver
    let openrc = OpenRcAdvancementsV78Engine::new();
    let services = vec![
        OpenRcServiceSpecV78 {
            name: "localmount".to_string(),
            ..Default::default()
        },
        OpenRcServiceSpecV78 {
            name: "net".to_string(),
            dependencies: vec![(OpenRcDependencyTypeV78::Need, "localmount".to_string())],
            ..Default::default()
        },
    ];
    let order = openrc.resolve_rc_service_order(&services).unwrap();
    assert_eq!(order, vec!["localmount", "net"]);

    // Run master verification
    assert!(master_suite.run_master_verification());
}
