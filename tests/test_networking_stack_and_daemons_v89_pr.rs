// Integration Test Suite for Sovereign Networking Stack & Daemons Advancements (v89 PR Suite)

extern crate alloc;

use alloc::string::ToString;

#[path = "../src/net/sovereign_networking_stack_and_daemons_v89_pr.rs"]
mod sovereign_networking_stack_and_daemons_v89_pr;

use sovereign_networking_stack_and_daemons_v89_pr::*;

#[test]
fn test_integration_networking_stack_and_daemons_v89_suite() {
    let gateway = SovereignNetworkingStackAndDaemonsV89PrGateway::new();
    let proposals = gateway.generate_all_pr_manifests();

    assert_eq!(proposals.len(), 7);
    assert_eq!(proposals[0].pr_id, 8171);
    assert_eq!(proposals[6].pr_id, 8177);

    let summary = gateway.generate_master_pr_summary();
    assert!(summary
        .contains("Sovereign Linux & BSD Networking Stack & Control Daemons Parity Suite (v89)"));
    assert!(summary.contains("PR #8171"));
    assert!(summary.contains("PR #8177"));
}

#[test]
fn test_integration_sub_engines_v89() {
    let mut gateway = SovereignNetworkingStackAndDaemonsV89PrGateway::new();

    // 1. Core TCP/IP Stack
    let conn = TcpConnectionDescriptorV89 {
        local_ip: "192.168.10.5".to_string(),
        remote_ip: "1.1.1.1".to_string(),
        local_port: 49152,
        remote_port: 443,
        congestion_algo: CongestionControlAlgoV89::Bbrv3,
        mptcp_subflows: 4,
        tcp_fastopen_enabled: true,
        ecn_active: true,
        big_tcp_tso_enabled: true,
        rtt_us: 15000,
        cwnd_bytes: 131072,
    };
    gateway.core_tcpip.register_connection(conn);
    let pacing = gateway.core_tcpip.evaluate_bbrv3_pacing(0);
    assert!(pacing > 0);
    assert!(gateway.core_tcpip.dual_stack_ipv4_ipv6);
    assert!(gateway.core_tcpip.big_tcp_gro_enabled);

    // 2. Network Configuration & Daemons
    let yaml = "network:\n  version: 2\n  ethernets:\n    eth0:\n      dhcp4: true\n  bonds:\n    bond0:\n      interfaces: [eth0]\n";
    assert!(gateway.net_config.parse_netplan_config(yaml));
    assert!(gateway.net_config.bonding_active);
    let ifconfig_res = gateway.net_config.route_iproute2_cmd("ifconfig -a");
    assert!(ifconfig_res.contains("eth0"));
    let ip_route_res = gateway.net_config.route_iproute2_cmd("ip route show");
    assert!(ip_route_res.contains("default via"));

    // 3. DHCP & DNS Services
    let lease = gateway
        .dhcp_dns
        .assign_dhcp_lease("00:11:22:33:44:55", "10.0.0.123");
    assert_eq!(lease.ip_address, "10.0.0.123");
    let doh_res = gateway.dhcp_dns.resolve_doh_query("kernel.org");
    assert_eq!(doh_res, "93.184.216.34");
    let doq_res = gateway.dhcp_dns.resolve_doq_query("dns.quad9.net");
    assert_eq!(doq_res, "93.184.216.34");
    assert_eq!(gateway.dhcp_dns.unbound_cache_hits, 2);

    // 4. Wireless & Bluetooth
    assert!(gateway
        .wireless_bt
        .connect_wpa3_sae("Corporate-Enterprise", "AA:BB:CC:DD:EE:01"));
    assert!(gateway.wireless_bt.initialize_bluez_lc3_audio());
    assert!(gateway
        .wireless_bt
        .bluetooth_active_codecs
        .contains(&"LC3".to_string()));
    assert!(gateway.wireless_bt.auracast_broadcast_active);
    assert!(gateway.wireless_bt.wifi7_multilink_operation);

    // 5. Packet Filtering & Firewalls
    gateway
        .firewall
        .add_nftables_rule("inet", "filter", "ip saddr 10.0.0.0/8 drop", "drop");
    gateway.firewall.load_openbsd_pf_anchor("vpn_filter");
    assert_eq!(gateway.firewall.iptables_rules.len(), 1);
    assert_eq!(gateway.firewall.pf_anchors[0], "vpn_filter");
    assert_eq!(gateway.firewall.nftables_flowtables, 4);

    // 6. Cryptographic Tunnels & VPNs
    assert!(gateway
        .crypto_vpn
        .establish_wireguard_pqc_tunnel("wg1", "198.51.100.1:51820"));
    assert_eq!(
        gateway.crypto_vpn.active_tunnels[0].tunnel_type,
        "WireGuard-PQC"
    );
    assert_eq!(gateway.crypto_vpn.tls_record_version, "TLS 1.3");
    assert!(gateway.crypto_vpn.ikev2_mobike_supported);

    // 7. Network Utilities & Diagnostics
    let http3_res = gateway
        .net_utils
        .execute_curl_http3_download("https://www.freebsd.org");
    assert!(http3_res.contains("HTTP/3"));
    let bpf_count = gateway.net_utils.capture_tcpdump_bpf("tcp and port 443");
    assert_eq!(bpf_count, 800);
    gateway.net_utils.register_avahi_service("_ipp._tcp.local");
    assert!(gateway
        .net_utils
        .registered_mdns_services
        .contains(&"_ipp._tcp.local".to_string()));
}
