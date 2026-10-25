//! Integration test suite for Sovereign Init Systems & Supervisors Advancements V75 PR.

#[path = "../src/system/sovereign_init_and_supervisor_advancements_v75_pr.rs"]
mod sovereign_init_and_supervisor_advancements_v75_pr;

use sovereign_init_and_supervisor_advancements_v75_pr::*;

#[test]
fn test_init_and_supervisor_v75_pr_integration() {
    let master_suite = SovereignInitAndSupervisorAdvancementsV75Suite::new();
    let manifests = master_suite.collect_all_manifests();

    assert_eq!(manifests.len(), 7);
    for manifest in &manifests {
        assert!(manifest.pr_id >= 4178 && manifest.pr_id <= 4184);
        assert!(!manifest.pr_title.is_empty());
        assert_eq!(manifest.status, "Ready for Integration");
    }

    // Verify systemd V75 state machine & cgroup v2
    let systemd = SystemdAdvancementsV75Engine::new();
    let state = systemd.transition_unit_state(SystemdUnitStateV75::Inactive, "start");
    assert_eq!(state, SystemdUnitStateV75::Activating);

    let limits = SystemdCgroupV2LimitsV75 {
        memory_max_bytes: Some(4096),
        memory_high_bytes: None,
        cpu_quota_percent: Some(100),
        tasks_max: Some(250),
        io_weight: Some(100),
    };
    let cgroup_params = systemd.build_cgroup_v2_control_parameters(&limits);
    assert_eq!(cgroup_params.get("memory.max"), Some(&"4096".to_string()));
    assert_eq!(cgroup_params.get("pids.max"), Some(&"250".to_string()));

    // Verify OpenRC V75 dependency solver
    let openrc = OpenRcAdvancementsV75Engine::new();
    let services = vec![
        OpenRcServiceSpecV75 {
            name: "localmount".to_string(),
            ..Default::default()
        },
        OpenRcServiceSpecV75 {
            name: "net".to_string(),
            dependencies: vec![(OpenRcDependencyTypeV75::Need, "localmount".to_string())],
            ..Default::default()
        },
    ];
    let order = openrc.resolve_rc_service_order(&services).unwrap();
    assert_eq!(order, vec!["localmount", "net"]);

    // Run master verification
    assert!(master_suite.run_master_verification());
}
