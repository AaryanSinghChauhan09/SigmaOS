// SigmaOS Sovereign Subsystems Inspection Unit Test Suite
// Verifies working mechanisms across Sovereign Subsystems:
// - Open Source Obsoletion Subsystem (Vcs, Init, PqcVpn, Observability, KnowledgeGraph, ApiTest, Partition)
// - Sovereign Data Workspace (SovereignML, SovereignCapture, SovereignQuery, SovereignGuard, SovereignCatalog)
// - POSIX Capabilities & CapabilityToken
// - OpenBSD Pledge Promises & Unveil Path Protections

#[path = "../src/open_source_obsoletion.rs"]
mod open_source_obsoletion;

#[path = "../src/ml/sovereign_data_workspace.rs"]
mod sovereign_data_workspace;

#[path = "../src/security/capability_enforcer.rs"]
mod capability_enforcer;

use capability_enforcer::*;
use open_source_obsoletion::*;
use sovereign_data_workspace::*;

#[test]
fn test_open_source_obsoletion_subsystem_inspection() {
    let mut vcs = SovereignVcsEngine::new();
    vcs.stage_file(
        "kernel/src/lib.rs",
        b"// SPDX-License-Identifier: MIT\npub fn init() {}",
    );
    assert_eq!(vcs.staging_area.len(), 1);

    let mut init = SovereignInitSupervisor::new();
    let service = ServiceUnit {
        name: "sigma-networkd".to_string(),
        exec_start: "/bin/net".to_string(),
        dependencies: Vec::new(),
        auto_restart_on_failure: true,
        current_state: SupervisorServiceState::Stopped,
        restart_count: 0,
    };
    assert!(init.register_service(service).is_ok());
    assert!(init.start_service("sigma-networkd").is_ok());

    let mut vpn = SovereignPqcVpnFirewall::new();
    vpn.add_firewall_rule(FirewallRule {
        rule_id: 101,
        source_cidr: "0.0.0.0/0".to_string(),
        port_range: (8080, 8080),
        action: FirewallAction::Allow,
    });
    let action = vpn.inspect_incoming_packet("192.168.1.10", 8080);
    assert_eq!(action, FirewallAction::Allow);

    let mut obs = SovereignObservabilitySuite::new();
    obs.record_metric("cpu_usage_pct", 12.5, 1000);
    obs.record_metric("cpu_usage_pct", 15.0, 1001);
    assert_eq!(obs.detect_anomalies().len(), 0);

    let mut kg = SovereignKnowledgeGraph::new();
    kg.add_note("Microkernel", "Zero-dependency Rust kernel core");
    assert_eq!(kg.query_backlinks("Microkernel").len(), 0);

    let mut api = SovereignApiTestSuite::new();
    api.add_request(ApiRequestSpec {
        method: "GET".to_string(),
        endpoint_url: "/api/v1/status".to_string(),
        headers: Vec::new(),
        body_json: String::new(),
    });
    let (passed, failed) = api.execute_suite();
    assert_eq!(passed, 1);
    assert_eq!(failed, 0);

    let mut part = SovereignPartitionEngine::new(100000);
    let ok = part.create_partition(SovereignFsType::Ext4, 1000, "Primary-NVMe");
    assert!(ok.is_ok());
    assert!(part.verify_alignment());
}

#[test]
fn test_sovereign_data_workspace_inspection() {
    let t1 = SovereignTensor::new(vec![1, 3], vec![1.0, 2.0, 3.0]);
    let t2 = SovereignTensor::new(vec![1, 3], vec![4.0, 5.0, 6.0]);
    let added = t1.add(&t2).unwrap();
    assert_eq!(added.data, vec![5.0, 7.0, 9.0]);

    let mut capture = SovereignCapture::new();
    capture.push_keystroke('a');
    capture.push_keystroke('d');
    capture.push_keystroke('m');
    capture.push_keystroke('i');
    capture.push_keystroke('n');
    let rendered = capture.render_masked_buffer();
    assert!(!rendered.is_empty());

    let mut query = SovereignQuery::new();
    query.add_column("cpu_usage".to_string(), vec![10.0, 25.0, 50.0, 80.0]);
    let gt = query.filter_greater_than("cpu_usage", 20.0).unwrap();
    assert_eq!(gt, vec![25.0, 50.0, 80.0]);

    let mut guard = SovereignGuard::new();
    let is_compliant = guard.inspect_payload(b"Public telemetry payload without secrets", 1000);
    assert!(is_compliant.is_ok());

    let mut catalog = SovereignCatalog::new();
    catalog.register_dataset(
        "telemetry_v1".to_string(),
        "Memory".to_string(),
        "0xABC123".to_string(),
    );
    assert!(catalog.lookup_residency("telemetry_v1").is_some());
}

#[test]
fn test_posix_capabilities_and_pledge_inspection() {
    let token = CapabilityToken::new(1001)
        .grant_posix_capability(21) // CAP_SYS_ADMIN bit 21
        .allow_fs_read();
    assert!(token.has_posix_capability(21));

    let mut enforcer = SecurityEnforcer::new();
    assert!(enforcer.assign_token(token).is_ok());
    assert!(enforcer.validate_filesystem_access(1001, false));
    assert!(!enforcer.validate_filesystem_access(1001, true));

    let token_ref = enforcer.find_token_mut(1001).unwrap();
    token_ref.pledge(&["stdio", "rpath"]);
    assert!(token_ref.validate_pledge_operation("stdio"));
    assert!(token_ref.validate_pledge_operation("rpath"));
    assert!(!token_ref.validate_pledge_operation("exec"));
}
