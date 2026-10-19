// SPDX-License-Identifier: MIT
// Integration Test for Sovereign Open Source OS Unimplemented Ideas V63 PR Suite
// (`tests/test_open_source_os_unimplemented_ideas_v63_pr.rs`)

use sigmaos::distro::open_source_os_unimplemented_ideas_v63_pr::{
    DragonFly68Hammer3MultiMasterSyncEngine, FreeBsd151CapsicumV3CapabilityEngine,
    HaikuBfsRelationalQuerySqlEngineV63, IllumosCrossbowVswitchMeshEngineV63,
    Linux619SchedExtPredictiveGovernorEngine, NetBsd111RumpHypercallV3BridgeEngine,
    NixGuixMerkleClosureCasEngineV63, OpenBsd80PledgeUnveilV3AuditEngine,
    Plan99P2000LSyntheticVfsEngineV63, SovereignOpenSourceOsUnimplementedIdeasV63MasterSuite,
};

#[test]
fn test_v63_schedext_predictive_governor() {
    let mut sched = Linux619SchedExtPredictiveGovernorEngine::new();
    assert_eq!(sched.latency_target_us, 120);
    let msg = sched.optimize_latency("gaming");
    assert!(msg.contains("40us"));
    assert_eq!(sched.latency_target_us, 40);
}

#[test]
fn test_v63_freebsd_capsicum_v3() {
    let mut cap = FreeBsd151CapsicumV3CapabilityEngine::new();
    assert_eq!(cap.protected_descriptors_count, 128);
    let msg = cap.restrict_fd_rights(8, 0x0000_FFFF);
    assert!(msg.contains("FD 8"));
    assert_eq!(cap.protected_descriptors_count, 129);
}

#[test]
fn test_v63_openbsd_pledge_unveil() {
    let mut pledge = OpenBsd80PledgeUnveilV3AuditEngine::new();
    assert!(pledge.unveil_directory("/srv", "rw"));
    assert!(pledge.audit_syscall("stdio"));
    assert!(!pledge.audit_syscall("execve_unauthorized"));
    assert_eq!(pledge.violation_logs_count, 1);
}

#[test]
fn test_v63_dragonfly_hammer3_sync() {
    let mut hammer = DragonFly68Hammer3MultiMasterSyncEngine::new();
    assert_eq!(hammer.sync_active_nodes(), 16);
    assert_eq!(hammer.sync_latency_ms, 1);
}

#[test]
fn test_v63_netbsd_rump_v3() {
    let mut rump = NetBsd111RumpHypercallV3BridgeEngine::new();
    let msg = rump.route_hypercall_v3(512);
    assert!(msg.contains("vector 512"));
}

#[test]
fn test_v63_haiku_bfs_sql() {
    let mut haiku = HaikuBfsRelationalQuerySqlEngineV63::new();
    let res = haiku.execute_sql_query("SELECT name FROM files");
    assert_eq!(res.len(), 2);
    assert_eq!(haiku.live_queries, 25);
}

#[test]
fn test_v63_plan9_synthetic_vfs() {
    let mut p9 = Plan99P2000LSyntheticVfsEngineV63::new();
    let msg = p9.mount_synthetic_vfs("/mnt/vfs_v63");
    assert!(msg.contains("/mnt/vfs_v63"));
    assert!(p9.mounted_namespaces.contains(&"/mnt/vfs_v63".to_string()));
}

#[test]
fn test_v63_nix_guix_merkle_cas() {
    let mut nix = NixGuixMerkleClosureCasEngineV63::new();
    let msg = nix.build_merkle_closure("sigma_kernel");
    assert!(msg.contains("sigma_kernel"));
}

#[test]
fn test_v63_illumos_crossbow_vswitch() {
    let mut illumos = IllumosCrossbowVswitchMeshEngineV63::new();
    let msg = illumos.provision_vswitch("vswitch_core");
    assert!(msg.contains("vswitch_core"));
    assert_eq!(illumos.vnics_count, 33);
}

#[test]
fn test_v63_open_source_os_master_suite() {
    let mut master = SovereignOpenSourceOsUnimplementedIdeasV63MasterSuite::new();
    assert_eq!(master.get_merged_pr_count(), 10);
    assert!(master.execute_and_verify_all());
}
