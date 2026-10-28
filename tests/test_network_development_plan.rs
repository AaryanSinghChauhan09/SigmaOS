// Integration Test for SigmaOS Network Development and Maintenance Plan
// (`tests/test_network_development_plan.rs`)

#[path = "../src/net/sovereign_network_development_and_pr_suite.rs"]
mod sovereign_network_development_and_pr_suite;

use sovereign_network_development_and_pr_suite::*;

#[test]
fn test_network_development_plan_tracker_13_prs() {
    let tracker = NetworkPrTracker::new();
    assert_eq!(tracker.total_prs(), 13);

    for pr_num in 1..=13 {
        let pr = tracker
            .get_pr(pr_num)
            .unwrap_or_else(|| panic!("PR {} missing from network tracker", pr_num));
        assert_eq!(pr.pr_number, pr_num);
        assert!(!pr.title.is_empty());
        assert!(!pr.main_work.is_empty());
        assert!(!pr.acceptance_criteria.is_empty());
    }

    assert_eq!(
        tracker.get_pr(1).unwrap().status,
        NetworkSupportStatus::Supported
    );
    assert_eq!(
        tracker.get_pr(3).unwrap().status,
        NetworkSupportStatus::Prototype
    );
    assert_eq!(
        tracker.get_pr(13).unwrap().status,
        NetworkSupportStatus::Supported
    );
}

#[test]
fn test_virtio_nic_queue_lifecycle() {
    let mut driver = VirtIoNetDriverReceiveTransmitPath::new([0x52, 0x54, 0x00, 0x12, 0x34, 0x56]);
    assert!(driver.initialize_queues(64, 64));

    let frame = vec![0xFF; 60];
    assert!(driver.transmit_frame(&frame).is_ok());
    assert!(driver.receive_frame(&[0x02; 60]).is_ok());
}

#[test]
fn test_interface_and_link_state_model() {
    let mut model = NetworkInterfaceAndLinkStateModel::new(1, "eth0", [0x52, 0x54, 0x00, 0x12, 0x34, 0x56]);
    assert_eq!(model.mac_address, [0x52, 0x54, 0x00, 0x12, 0x34, 0x56]);
    assert_eq!(model.link_state, LinkState::Down);

    model.set_link_state(LinkState::Up);
    assert_eq!(model.link_state, LinkState::Up);

    model.counters.record_tx(100);
    model.counters.record_rx(200);
    assert_eq!(model.counters.tx_bytes, 100);
    assert_eq!(model.counters.rx_bytes, 200);
}

#[test]
fn test_ipv4_routing_and_icmp_host_engine() {
    let mut router = Ipv4RoutingAndIcmpHostEngine::new();
    router.add_route(RouteEntry {
        destination: [10, 0, 0, 0],
        netmask: [255, 0, 0, 0],
        gateway: None,
        interface_name: "eth0".to_string(),
        metric: 1,
    });

    assert!(router.lookup_route([10, 1, 2, 3]).is_some());
    assert!(router.lookup_route([192, 168, 1, 1]).is_none());

    assert_eq!(
        router.process_icmp_header(8, 0, 64),
        Ok(IcmpType::EchoReply)
    );
    assert_eq!(
        router.process_icmp_header(8, 0, 1),
        Ok(IcmpType::TimeExceeded)
    );
}

#[test]
fn test_ipv6_host_networking_foundation_engine() {
    let mut engine = Ipv6HostNetworkingFoundationEngine::new();
    let link_local = engine.generate_link_local_from_mac([0x52, 0x54, 0x00, 0x12, 0x34, 0x56]);
    assert_eq!(link_local[0..2], [0xfe, 0x80]);

    let ra_payload = [0u8; 16];
    assert!(engine.process_router_advertisement(&ra_payload));
}

#[test]
fn test_tcp_transport_rfc9293() {
    let mut tcp = TcpTransportAndSocketLifecycleEngine::new(8080);
    assert_eq!(tcp.state, TcpState::Closed);

    assert!(tcp.listen().is_ok());
    assert_eq!(tcp.state, TcpState::Listen);

    assert_eq!(
        tcp.process_segment(true, false, false, false, 500, 0),
        Ok(TcpState::SynRcvd)
    );
    assert_eq!(
        tcp.process_segment(false, true, false, false, 501, 1001),
        Ok(TcpState::Established)
    );
}

#[test]
fn test_dhcp_dns_hardened_client() {
    let mut engine = HardenedDhcpAndDnsClientEngine::new();
    assert!(engine.add_dns_record("sigmaos.org", [192, 168, 1, 100], 300).is_ok());
    assert_eq!(
        engine.resolve("sigmaos.org"),
        Some([192, 168, 1, 100])
    );
    assert_eq!(engine.resolve("unknown.org"), None);
}

#[test]
fn test_persistent_config_rollback() {
    let initial = DeclarativeNetworkConfig {
        version: 1,
        interface_name: "eth0".to_string(),
        use_dhcp: true,
        static_ip: None,
        gateway: None,
        dns_servers: vec![],
    };
    let mut mgr = PersistentNetworkConfigAndRollbackEngine::new(initial);

    let invalid = DeclarativeNetworkConfig {
        version: 0,
        interface_name: "".to_string(),
        use_dhcp: false,
        static_ip: None,
        gateway: None,
        dns_servers: vec![],
    };
    assert!(mgr.apply_config_transactional(invalid).is_err());

    let valid_static = DeclarativeNetworkConfig {
        version: 1,
        interface_name: "eth0".to_string(),
        use_dhcp: false,
        static_ip: Some([10, 0, 0, 2]),
        gateway: Some([10, 0, 0, 1]),
        dns_servers: vec![[8, 8, 8, 8]],
    };
    assert!(mgr.apply_config_transactional(valid_static).is_ok());
    assert_eq!(mgr.active_config.static_ip, Some([10, 0, 0, 2]));

    mgr.rollback_configuration();
    assert!(mgr.active_config.use_dhcp);
}

#[test]
fn test_network_control_cli_diagnostics() {
    let model = NetworkInterfaceAndLinkStateModel::new(1, "eth0", [0x52, 0x54, 0x00, 0x12, 0x34, 0x56]);
    let status_json = NetworkControlCliAndDiagnosticsEngine::generate_interface_status_json(&model);
    assert!(status_json.contains("eth0"));
}

#[test]
fn test_firewall_policy_framework() {
    let mut fw = FirewallPolicyFrameworkEngine::new();
    fw.add_rule(FirewallRule {
        rule_id: 1,
        port: 443,
        is_inbound: true,
        action: FirewallAction::Allow,
    });

    assert_eq!(fw.evaluate_packet(443, true), FirewallAction::Allow);
    assert_eq!(fw.evaluate_packet(23, true), FirewallAction::Deny); // Default deny
}

#[test]
fn test_master_network_development_suite() {
    let mut master = SovereignNetworkDevelopmentAndPrMasterSuite::new();
    assert!(master.run_complete_verification_suite());
}
