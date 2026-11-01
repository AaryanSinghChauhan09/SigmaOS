// Integration Test Suite for Sovereign Networking Stack & Daemons Advancements (v87 PR Suite)

extern crate alloc;

#[path = "../src/net/sovereign_networking_stack_and_daemons_v87_pr.rs"]
mod sovereign_networking_stack_and_daemons_v87_pr;

use sovereign_networking_stack_and_daemons_v87_pr::*;

#[test]
fn test_integration_networking_stack_and_daemons_v87_suite() {
    let gateway = SovereignNetworkingStackAndDaemonsV87PrGateway::new();
    let proposals = gateway.generate_all_pr_manifests();

    assert_eq!(proposals.len(), 7);
    assert_eq!(proposals[0].pr_id, 8151);
    assert_eq!(proposals[6].pr_id, 8157);

    let summary = gateway.generate_master_pr_summary();
    assert!(summary.contains("Sovereign Linux & BSD Networking Stack & Control Daemons Parity Suite (v87)"));
    assert!(summary.contains("PR #8151"));
    assert!(summary.contains("PR #8157"));
}

#[test]
fn test_integration_sub_engines_v87() {
    let mut gateway = SovereignNetworkingStackAndDaemonsV87PrGateway::new();

    // 1. Core TCP/IP Stack
    let conn = TcpConnectionDescriptorV87 {
        local_ip: "192.168.10.5".to_string(),
        remote_ip: "1.1.1.1".to_string(),
        local_port: 49152,
        remote_port: 443,
        congestion_algo: CongestionControlAlgoV87::Bbrv3,
        mptcp_subflows: 4,
        tcp_fastopen_enabled: true,
        ecn_active: true,
        rtt_us: 15000,
        cwnd_bytes: 131072,
    };
    gateway.core_tcpip.register_connection(conn);
    let pacing = gateway.core_tcpip.evaluate_bbrv3_pacing(0);
    assert!(pacing > 0);

    // 2. Network Configuration & Daemons
    let yaml = "network:\n  version: 2\n  ethernets:\n    eth0:\n      dhcp4: true\n";
    assert!(gateway.net_config.parse_netplan_config(yaml));
    let ifconfig_res = gateway.net_config.route_iproute2_cmd("ifconfig -a");
    assert!(ifconfig_res.contains("eth0"));

    // 3. DHCP & DNS Services
    let lease = gateway.dhcp_dns.assign_dhcp_lease("00:11:22:33:44:55", "10.0.0.123");
    assert_eq!(lease.ip_address, "10.0.0.123");
    let doh_res = gateway.dhcp_dns.resolve_doh_query("kernel.org");
    assert_eq!(doh_res, "93.184.216.34");

    // 4. Wireless & Bluetooth
    assert!(gateway.wireless_bt.connect_wpa3_sae("Corporate-Enterprise", "AA:BB:CC:DD:EE:01"));
    assert!(gateway.wireless_bt.initialize_bluez_lc3_audio());
    assert!(gateway.wireless_bt.bluetooth_active_codecs.contains(&"LC3".to_string()));

    // 5. Packet Filtering & Firewalls
    gateway.firewall.add_nftables_rule("inet", "filter", "ip saddr 10.0.0.0/8 drop", "drop");
    gateway.firewall.load_openbsd_pf_anchor("vpn_filter");
    assert_eq!(gateway.firewall.iptables_rules.len(), 1);
    assert_eq!(gateway.firewall.pf_anchors[0], "vpn_filter");

    // 6. Cryptographic Tunnels & VPNs
    assert!(gateway.crypto_vpn.establish_wireguard_pqc_tunnel("wg1", "198.51.100.1:51820"));
    assert_eq!(gateway.crypto_vpn.active_tunnels[0].tunnel_type, "WireGuard-PQC");

    // 7. Network Utilities & Diagnostics
    let http3_res = gateway.net_utils.execute_curl_http3_download("https://www.freebsd.org");
    assert!(http3_res.contains("HTTP/3"));
    let bpf_count = gateway.net_utils.capture_tcpdump_bpf("tcp and port 443");
    assert_eq!(bpf_count, 500);
}
