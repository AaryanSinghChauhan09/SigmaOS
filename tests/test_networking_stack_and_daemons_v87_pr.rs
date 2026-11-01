// Integration Test Suite for Sovereign Networking Stack & Daemons Advancements (v87 PR Suite)

extern crate alloc;

#[path = "../src/net/sovereign_networking_stack_and_daemons_v87_pr.rs"]
mod sovereign_networking_stack_and_daemons_v87_pr;

use sovereign_networking_stack_and_daemons_v87_pr::*;

#[test]
fn test_integration_networking_stack_and_daemons_v87_suite() {
    let gateway = SovereignNetworkingStackAndDaemonsV87PrGateway::new();
    let manifests = gateway.generate_all_pr_manifests();

    assert_eq!(manifests.len(), 7);
    assert_eq!(manifests[0].pr_id, 8151);
    assert_eq!(manifests[1].pr_id, 8152);
    assert_eq!(manifests[2].pr_id, 8153);
    assert_eq!(manifests[3].pr_id, 8154);
    assert_eq!(manifests[4].pr_id, 8155);
    assert_eq!(manifests[5].pr_id, 8156);
    assert_eq!(manifests[6].pr_id, 8157);

    let summary = gateway.generate_master_pr_summary();
    assert!(summary.contains("Sovereign Networking Stack & Daemons Advancements (v87 PR Suite)"));
    assert!(summary.contains("PR #8151"));
    assert!(summary.contains("PR #8157"));
}

#[test]
fn test_integration_sub_engines_v87() {
    let mut gateway = SovereignNetworkingStackAndDaemonsV87PrGateway::new();

    // 1. Core TCP/IP Stack
    let conn = TcpConnectionDescriptorV87 {
        local_ip: "192.168.1.10".to_string(),
        remote_ip: "1.0.0.1".to_string(),
        local_port: 50000,
        remote_port: 443,
        congestion_algo: CongestionControlAlgoV87::Bbrv3,
        mptcp_subflows: 4,
        tcp_fastopen_enabled: true,
        ecn_active: true,
        rtt_us: 8000,
        cwnd_bytes: 131072,
    };
    gateway.core_tcpip_engine.register_connection(conn);
    let pacing = gateway.core_tcpip_engine.evaluate_bbrv3_pacing(0);
    assert!(pacing > 0);

    // 2. Network Configuration & Daemons
    let netplan_cfg = "network:\n  version: 2\n  renderer: NetworkManager\n  ethernets:\n    enp3s0:\n      dhcp4: true\n";
    assert!(gateway.config_daemons_engine.parse_netplan_config(netplan_cfg));
    let iface_info = gateway.config_daemons_engine.route_iproute2_cmd("ip addr show");
    assert!(iface_info.contains("eth0"));

    // 3. DHCP & DNS Services
    let lease = gateway.dhcp_dns_engine.assign_dhcp_lease("00:1A:2B:3C:4D:5E", "10.0.0.15");
    assert_eq!(lease.ip_address, "10.0.0.15");
    let resolved_ip = gateway.dhcp_dns_engine.resolve_doh_query("example.com");
    assert_eq!(resolved_ip, "93.184.216.34");

    // 4. Wireless & Bluetooth
    assert!(gateway.wireless_bluetooth_engine.connect_wpa3_sae("SecureOffice-WPA3", "AA:BB:CC:DD:EE:01"));
    assert!(gateway.wireless_bluetooth_engine.initialize_bluez_lc3_audio());
    assert!(gateway.wireless_bluetooth_engine.bluetooth_active_codecs.contains(&"LC3".to_string()));

    // 5. Packet Filtering & Firewall
    gateway.firewall_engine.add_nftables_rule("inet", "filter", "tcp dport 443", "accept");
    gateway.firewall_engine.load_openbsd_pf_anchor("vpn_filter_anchor");
    assert_eq!(gateway.firewall_engine.pf_anchors[0], "vpn_filter_anchor");

    // 6. Crypto Tunnels & VPN
    assert!(gateway.crypto_vpn_engine.establish_wireguard_pqc_tunnel("wg1", "198.51.100.1:51820"));
    assert_eq!(gateway.crypto_vpn_engine.active_tunnels[0].tunnel_type, "WireGuard-PQC");

    // 7. Network Utilities & Diagnostics
    let http_response = gateway.utilities_engine.execute_curl_http3_download("https://sigmaos.org");
    assert!(http_response.contains("HTTP/3 200 OK"));
    let captured = gateway.utilities_engine.capture_tcpdump_bpf("tcp and port 443");
    assert_eq!(captured, 500);
}
