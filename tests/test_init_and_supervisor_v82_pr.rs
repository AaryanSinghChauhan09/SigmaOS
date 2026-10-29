//! Integration test suite for Sovereign Init Systems & Supervisors Advancements V82 PR.

#[path = "../src/system/sovereign_init_and_supervisor_advancements_v82_pr.rs"]
mod sovereign_init_and_supervisor_advancements_v82_pr;

use sovereign_init_and_supervisor_advancements_v82_pr::*;

#[test]
fn test_init_and_supervisor_v82_pr_integration() {
    let master_suite = SovereignInitAndSupervisorAdvancementsV82Suite::new();
    let manifests = master_suite.collect_all_manifests();

    assert_eq!(manifests.len(), 7);
    for manifest in &manifests {
        assert!(manifest.pr_id >= 4892 && manifest.pr_id <= 4898);
        assert!(!manifest.pr_title.is_empty());
        assert_eq!(manifest.status, "Ready for Integration");
    }

    // Verify systemd V82 state machine & cgroup v2
    let systemd = SystemdAdvancementsV82Engine::new();
    let state = systemd.transition_unit_state(SystemdUnitStateV82::Inactive, "start");
    assert_eq!(state, SystemdUnitStateV82::Activating);

    let limits = SystemdCgroupV2LimitsV82 {
        memory_max_bytes: Some(4096),
        memory_high_bytes: None,
        cpu_quota_percent: Some(100),
        tasks_max: Some(250),
        io_weight: Some(100),
    };
    let cgroup_params = systemd.build_cgroup_v2_control_parameters(&limits);
    assert_eq!(cgroup_params.get("memory.max"), Some(&"4096".to_string()));
    assert_eq!(cgroup_params.get("pids.max"), Some(&"250".to_string()));

    // Verify OpenRC V82 dependency solver
    let openrc = OpenRcAdvancementsV82Engine::new();
    let services = vec![
        OpenRcServiceSpecV82 {
            name: "localmount".to_string(),
            ..Default::default()
        },
        OpenRcServiceSpecV82 {
            name: "net".to_string(),
            dependencies: vec![(OpenRcDependencyTypeV82::Need, "localmount".to_string())],
            ..Default::default()
        },
    ];
    let order = openrc.resolve_rc_service_order(&services).unwrap();
    assert_eq!(order, vec!["localmount", "net"]);

    // Verify gateway orchestration with PQC signature
    let gateway = SovereignInitAndSupervisorV82GatewayEngine::new();
    let sig = [0u8; 64];
    let orchestrated_manifests = gateway.orchestrate_core_userland_v82(&sig).unwrap();
    assert_eq!(orchestrated_manifests.len(), 7);

    // Run master verification
    assert!(master_suite.run_master_verification());
}
