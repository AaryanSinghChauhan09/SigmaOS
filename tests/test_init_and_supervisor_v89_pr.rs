//! Integration test suite for Sovereign Init Systems & Supervisors Advancements V89 PR.

#[path = "../src/system/sovereign_init_and_supervisor_advancements_v89_pr.rs"]
mod sovereign_init_and_supervisor_advancements_v89_pr;

use sovereign_init_and_supervisor_advancements_v89_pr::*;
extern crate alloc;
use alloc::collections::BTreeSet;
use alloc::string::ToString;
use alloc::vec;

#[test]
fn test_init_and_supervisor_v89_pr_integration() {
    let gateway = SovereignInitAndSupervisorV89PrGatewayEngine::new();
    let dummy_sig = [0x42u8; 64];
    let manifests = gateway.orchestrate_core_userland_v89(&dummy_sig).unwrap();

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
    assert_eq!(state_active, SystemdUnitStateV89::Active);

    let limits = SystemdCgroupV2LimitsV89 {
        memory_max_bytes: Some(1024 * 1024 * 512),
        cpu_quota_percent: Some(200),
        ..Default::default()
    };
    let cgroup_params = systemd.build_cgroup_v2_control_parameters(&limits);
    assert_eq!(cgroup_params.get("memory.max"), Some(&"536870912".to_string()));

    let mut active = BTreeSet::new();
    active.insert("network.target".to_string());
    let deps = SystemdDependencySpecV89 {
        requires: vec!["network.target".to_string()],
        ..Default::default()
    };
    let (can_start, missing) = systemd.evaluate_unit_dependencies_startable("app.service", &deps, &active);
    assert!(can_start);
    assert!(missing.is_empty());
}
