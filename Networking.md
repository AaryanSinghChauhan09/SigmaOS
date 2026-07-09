# SigmaOS Networking

## Overview

SigmaOS provides comprehensive networking capabilities including VPN support, advanced routing protocols, QoS controls, and network monitoring. This document describes the networking architecture and features.

## VPN Support

### Overview

SigmaOS includes native VPN client/server support for WireGuard and OpenVPN protocols, reducing dependency on external VPN tools.

**Implementation:** `network/vpn/sigma_vpn.rs`

### Supported Protocols

- **WireGuard:** Modern, high-performance VPN protocol
- **OpenVPN:** Industry-standard VPN with extensive configuration options
- **IKEv2:** Planned for future implementation
- **L2TP:** Planned for future implementation

### WireGuard Implementation

#### WireGuard Interface
```rust
pub struct WireGuardInterface {
    pub private_key: [u8; 32],
    pub public_key: [u8; 32],
    pub listen_port: u16,
    pub fwmark: u32,
    pub peers: *mut WireGuardPeer,
    pub peer_count: u32,
}
```

#### WireGuard Peer
```rust
pub struct WireGuardPeer {
    pub public_key: [u8; 32],
    pub endpoint_ip: [u8; 16],
    pub endpoint_port: u16,
    pub allowed_ips: [u8; 256],
    pub persistent_keepalive: u32,
    pub preshared_key: [u8; 32],
}
```

#### Features
- Automatic key pair generation using SHA-256
- Tunnel interface creation with IP assignment
- DNS server configuration (default: 1.1.1.1, 1.0.0.1)
- Peer management (add/remove peers)
- Persistent keepalive support
- Pre-shared key support for additional security

#### C-ABI Exports
```c
int vpn_wireguard_add_peer(const uint8_t* public_key, const uint8_t* endpoint_ip, 
                          uint16_t endpoint_port, const uint8_t* allowed_ips);
int vpn_wireguard_remove_peer(const uint8_t* public_key);
int vpn_wireguard_set_private_key(const uint8_t* private_key);
```

### OpenVPN Implementation

#### OpenVPN Configuration
```rust
pub struct OpenVPNConfig {
    pub ca_cert: [u8; 4096],
    pub client_cert: [u8; 4096],
    pub client_key: [u8; 4096],
    pub cipher: EncryptionAlgorithm,
    pub proto: u8, // UDP or TCP
    pub dev: [u8; 32], // tun or tap
    pub comp_lzo: bool,
    pub auth: [u8; 64],
}
```

#### Features
- CA certificate management
- Client certificate and key management
- Support for AES-256-GCM and ChaCha20-Poly1305 ciphers
- TUN/TAP device support
- LZO compression support
- Custom authentication methods

#### C-ABI Exports
```c
int vpn_openvpn_set_ca_cert(const uint8_t* ca_cert, uint32_t cert_len);
int vpn_openvpn_set_client_cert(const uint8_t* client_cert, uint32_t cert_len);
int vpn_openvpn_set_client_key(const uint8_t* client_key, uint32_t key_len);
int vpn_openvpn_set_cipher(EncryptionAlgorithm cipher);
```

### Kill Switch

The kill switch blocks all non-VPN traffic when enabled, providing enhanced privacy and security.

#### Features
- Firewall rule integration
- Automatic blocking of non-VPN traffic
- Traffic restoration on disable
- Per-interface configuration

#### C-ABI Export
```c
int vpn_set_kill_switch(bool enabled);
```

### Tunnel Interface

#### Tunnel Configuration
```rust
pub struct TunnelInterface {
    pub if_name: [u8; 16],
    pub local_ip: [u8; 16],
    pub remote_ip: [u8; 16],
    pub mtu: u32,
    pub dns_servers: [u8; 256],
    pub active: bool,
}
```

#### Features
- Automatic IP assignment
- MTU configuration (default: 1420 for WireGuard)
- DNS server configuration
- Tunnel status tracking

## Advanced Routing

### Overview

SigmaOS implements advanced routing protocols including BGP and OSPF for enterprise-grade network infrastructure.

**Implementation:** `network/routing/sigma_routing.rs`

### BGP Implementation

#### BGP Router
```rust
pub struct BGPRouter {
    pub local_as: u32,
    pub router_id: u32,
    pub peers: *mut BGPPeer,
    pub peer_count: u32,
    pub routes: *mut BGPRoute,
    pub route_count: u32,
    pub routing_table: RoutingTable,
}
```

#### BGP Peer
```rust
pub struct BGPPeer {
    pub peer_ip: [u8; 16],
    pub peer_as: u32,
    pub local_as: u32,
    pub state: BGPState,
    pub hold_time: u32,
    pub keepalive_interval: u32,
    pub established: u64,
    pub routes_received: u32,
    pub routes_sent: u32,
}
```

#### BGP State Machine
- **Idle:** Initial state, waiting for start event
- **Connect:** Waiting for TCP connection to be established
- **Active:** Trying to establish TCP connection
- **OpenSent:** OPEN message sent, waiting for OPEN
- **OpenConfirm:** OPEN message received, waiting for KEEPALIVE
- **Established:** Connection fully established

#### BGP Route Selection
The BGP decision process follows these steps:
1. Highest weight (Cisco-specific)
2. Highest local preference
3. Locally originated routes
4. Shortest AS path
5. Lowest origin type
6. Lowest MED
7. Prefer eBGP over iBGP
8. Lowest IGP metric to next hop
9. Oldest route
10. Lowest router ID
11. Lowest neighbor address

#### C-ABI Exports
```c
int bgp_init(uint32_t local_as, uint32_t router_id);
int bgp_add_peer(const uint8_t* peer_ip, uint32_t peer_as);
int bgp_remove_peer(const uint8_t* peer_ip);
BGPState bgp_get_peer_state(const uint8_t* peer_ip);
int bgp_advertise_route(const uint8_t* peer_ip, const BGPRoute* route);
int bgp_withdraw_route(const uint8_t* peer_ip, const uint8_t* prefix, uint8_t prefix_len);
```

### OSPF Implementation

#### OSPF Router
```rust
pub struct OSPFRouter {
    pub router_id: u32,
    pub areas: *mut OSPFArea,
    pub area_count: u32,
    pub routing_table: RoutingTable,
}
```

#### OSPF Area
```rust
pub struct OSPFArea {
    pub area_id: [u8; 4],
    pub area_type: OSPFAreaType,
    pub router_id: u32,
    pub interfaces: *mut OSPFInterface,
    pub interface_count: u32,
}
```

#### OSPF Area Types
- **Normal:** Standard OSPF area
- **Stub:** No external routes, default route only
- **NSSA:** Not-So-Stubby Area, allows external routes

#### OSPF Interface
```rust
pub struct OSPFInterface {
    pub interface_id: u32,
    pub ip_address: [u8; 16],
    pub prefix_len: u8,
    pub area_id: [u8; 4],
    pub hello_interval: u32,
    pub dead_interval: u32,
    pub priority: u8,
    pub cost: u32,
    pub state: OSPFInterfaceState,
    pub neighbors: *mut OSPFNeighbor,
    pub neighbor_count: u32,
}
```

#### OSPF Interface States
- **Down:** Interface not operational
- **Loopback:** Loopback interface
- **Waiting:** Waiting for DR election
- **Point-to-Point:** Point-to-point network
- **DR:** Designated Router
- **BDR:** Backup Designated Router
- **DROther:** Neither DR nor BDR

#### OSPF Neighbor States
- **Down:** Initial state
- **Attempt:** Trying to contact neighbor
- **Init:** Hello received from neighbor
- **TwoWay:** Bidirectional communication established
- **ExStart:** Exchanging database description
- **Exchange:** Exchanging link state information
- **Loading:** Loading link state information
- **Full:** Adjacency fully established

#### OSPF SPF Algorithm
OSPF uses Dijkstra's Shortest Path First algorithm to calculate optimal routes based on the link state database.

#### DR/BDR Election
Multi-access networks elect a Designated Router (DR) and Backup Designated Router (BDR) to reduce LSA flooding overhead.

#### C-ABI Exports
```c
int ospf_init(uint32_t router_id);
int ospf_add_area(const uint8_t* area_id, OSPFAreaType area_type);
int ospf_add_interface(const uint8_t* area_id, uint32_t interface_id, 
                      const uint8_t* ip_address, uint8_t prefix_len);
```

### Routing Table Management

#### Route Entry
```rust
pub struct RouteEntry {
    pub destination: [u8; 16],
    pub prefix_len: u8,
    pub gateway: [u8; 16],
    pub interface: [u8; 16],
    pub metric: u32,
    pub protocol: RoutingProtocol,
    pub flags: u32,
}
```

#### C-ABI Exports
```c
int routing_add_static_route(const uint8_t* destination, uint8_t prefix_len,
                              const uint8_t* gateway, const uint8_t* interface,
                              uint32_t metric);
int routing_remove_route(const uint8_t* destination, uint8_t prefix_len);
int routing_lookup_route(const uint8_t* destination, RouteEntry* route);
int routing_get_table(RouteEntry* routes, uint32_t max_routes, uint32_t* route_count);
int routing_flush_table(void);
```

## QoS Controls

### Overview

SigmaOS provides comprehensive Quality of Service (QoS) controls for traffic shaping, prioritization, and rate limiting.

**Implementation:** `network/qos/sigma_qos.rs`

### Scheduling Algorithms

#### Supported Schedulers
- **FIFO:** First-In-First-Out queuing
- **Priority:** Priority-based queuing
- **WFQ:** Weighted Fair Queuing
- **CBQ:** Class-Based Queuing
- **HTB:** Hierarchical Token Bucket

### Traffic Classes

#### Class Types
- **BestEffort:** Default traffic class
- **Bulk:** Low-priority bulk transfers
- **Interactive:** Interactive user traffic
- **Realtime:** Real-time traffic (VoIP, video)
- **Control:** Control plane traffic

### Token Bucket Algorithm

#### Token Bucket Structure
```rust
pub struct TokenBucket {
    pub rate: u32, // tokens per second
    pub burst: u32, // maximum bucket size
    pub tokens: u32, // current tokens
    pub last_update: u64, // timestamp of last update
}
```

#### Features
- Configurable rate limit
- Burst allowance
- Automatic token refill
- Precise rate limiting

#### C-ABI Exports
```c
int qos_token_bucket_init(TokenBucket* bucket, uint32_t rate, uint32_t burst);
bool qos_token_bucket_consume(TokenBucket* bucket, uint32_t tokens);
```

### QoS Rules

#### Rule Structure
```rust
pub struct QoSRule {
    pub rule_id: u32,
    pub priority: u32,
    pub source_ip: [u8; 16],
    pub source_mask: u8,
    pub dest_ip: [u8; 16],
    pub dest_mask: u8,
    pub source_port: u16,
    pub dest_port: u16,
    pub protocol: u8,
    pub traffic_class: TrafficClass,
    pub action: QoSAction,
    pub rate_limit: u32,
    pub burst: u32,
    pub latency: u32,
    pub jitter: u32,
}
```

#### QoS Actions
- **Accept:** Allow traffic
- **Reject:** Drop traffic
- **RateLimit:** Apply rate limiting
- **Prioritize:** Mark for priority scheduling
- **Delay:** Add artificial delay

#### C-ABI Exports
```c
int qos_add_rule(const QoSRule* rule);
int qos_remove_rule(uint32_t rule_id);
int qos_list_rules(QoSRule* rules, uint32_t max_rules, uint32_t* rule_count);
```

### Traffic Queues

#### Queue Structure
```rust
pub struct TrafficQueue {
    pub queue_id: u32,
    pub scheduler: QoSScheduler,
    pub priority: u32,
    pub weight: u32,
    pub rate_limit: u32,
    pub bucket: TokenBucket,
    pub packets: u64,
    pub bytes: u64,
    pub dropped: u64,
}
```

#### C-ABI Exports
```c
uint32_t qos_create_queue(QoSScheduler scheduler, uint32_t priority, uint32_t weight, uint32_t rate_limit);
int qos_delete_queue(uint32_t queue_id);
int qos_set_rate_limit(uint32_t queue_id, uint32_t rate_limit, uint32_t burst);
int qos_get_queue_stats(uint32_t queue_id, uint64_t* packets, uint64_t* bytes, uint64_t* dropped);
```

### Traffic Classification

#### C-ABI Exports
```c
TrafficClass qos_classify_packet(const uint8_t* packet, uint32_t packet_len, uint32_t* rule_id);
int qos_apply_action(const uint8_t* packet, uint32_t packet_len, QoSAction action);
```

### Interface Management

#### C-ABI Exports
```c
int qos_enable_interface(const uint8_t* interface_name, uint32_t ingress_queue, uint32_t egress_queue);
int qos_disable_interface(const uint8_t* interface_name);
```

### QoS Statistics

#### Statistics Structure
```rust
pub struct QoSStats {
    pub total_packets: u64,
    pub total_bytes: u64,
    pub dropped_packets: u64,
    pub dropped_bytes: u64,
    pub shaped_packets: u64,
    pub shaped_bytes: u64,
    pub latency_avg: u32,
    pub latency_max: u32,
}
```

#### C-ABI Exports
```c
int qos_get_stats(QoSStats* stats);
int qos_reset_stats(void);
```

## Network Monitoring

### Overview

SigmaOS provides a comprehensive network monitoring dashboard for real-time network statistics, interface monitoring, and traffic analysis.

**Implementation:** `network/monitor/sigma_netmon.rs`

### Interface Monitoring

#### Interface Statistics
```rust
pub struct InterfaceStats {
    pub interface_name: [u8; 16],
    pub rx_packets: u64,
    pub rx_bytes: u64,
    pub rx_errors: u64,
    pub rx_dropped: u64,
    pub tx_packets: u64,
    pub tx_bytes: u64,
    pub tx_errors: u64,
    pub tx_dropped: u64,
    pub rx_rate: u32,
    pub tx_rate: u32,
    pub link_speed: u32,
    pub duplex: bool,
    pub up: bool,
}
```

#### C-ABI Exports
```c
int netmon_add_interface(const uint8_t* interface_name);
int netmon_remove_interface(const uint8_t* interface_name);
int netmon_get_interface_stats(const uint8_t* interface_name, InterfaceStats* stats);
int netmon_list_interfaces(InterfaceStats* interfaces, uint32_t max_interfaces, uint32_t* interface_count);
```

### Connection Monitoring

#### Connection Entry
```rust
pub struct ConnectionEntry {
    pub local_ip: [u8; 16],
    pub local_port: u16,
    pub remote_ip: [u8; 16],
    pub remote_port: u16,
    pub protocol: u8,
    pub state: u8,
    pub pid: u32,
    pub process_name: [u8; 64],
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub established: u64,
}
```

#### C-ABI Exports
```c
int netmon_update_connections(void);
int netmon_get_connections(ConnectionEntry* connections, uint32_t max_connections, uint32_t* connection_count);
int netmon_get_connections_by_pid(uint32_t pid, ConnectionEntry* connections, uint32_t max_connections, uint32_t* connection_count);
int netmon_get_connections_by_protocol(uint8_t protocol, ConnectionEntry* connections, uint32_t max_connections, uint32_t* connection_count);
```

### Traffic Statistics

#### Traffic Stats
```rust
pub struct TrafficStats {
    pub total_rx: u64,
    pub total_tx: u64,
    pub total_rx_rate: u32,
    pub total_tx_rate: u32,
    pub tcp_connections: u32,
    pub udp_connections: u32,
    pub active_connections: u32,
}
```

#### C-ABI Exports
```c
int netmon_update_traffic(void);
int netmon_get_traffic_stats(TrafficStats* traffic);
int netmon_get_protocol_stats(ProtocolStats* protocols, uint32_t max_protocols, uint32_t* protocol_count);
```

### Alert Management

#### Alert Threshold
```rust
pub struct AlertThreshold {
    pub metric: AlertMetric,
    pub threshold: u32,
    pub enabled: bool,
}
```

#### Alert Metrics
- **RxRate:** Receive rate threshold
- **TxRate:** Transmit rate threshold
- **ConnectionCount:** Connection count threshold
- **ErrorRate:** Error rate threshold
- **Latency:** Latency threshold

#### Network Alert
```rust
pub struct NetworkAlert {
    pub alert_id: u32,
    pub metric: AlertMetric,
    pub value: u32,
    pub threshold: u32,
    pub timestamp: u64,
    pub message: [u8; 256],
}
```

#### C-ABI Exports
```c
int netmon_add_threshold(AlertMetric metric, uint32_t threshold);
int netmon_remove_threshold(AlertMetric metric);
int netmon_check_thresholds(void);
int netmon_get_alerts(NetworkAlert* alerts, uint32_t max_alerts, uint32_t* alert_count);
int netmon_clear_alerts(void);
```

### Dashboard Data Export

#### C-ABI Exports
```c
int netmon_export_json(uint8_t* buffer, uint32_t buffer_size, uint32_t* bytes_written);
int netmon_export_csv(uint8_t* buffer, uint32_t buffer_size, uint32_t* bytes_written);
```

### Monitor Control

#### C-ABI Exports
```c
int netmon_init(void);
int netmon_start(void);
int netmon_stop(void);
bool netmon_initialized(void);
bool netmon_is_monitoring(void);
```

## Configuration

### VPN Configuration

Example WireGuard configuration:
```ini
[Interface]
PrivateKey = <private-key>
Address = 10.0.0.2/24
DNS = 1.1.1.1,1.0.0.1

[Peer]
PublicKey = <peer-public-key>
Endpoint = 192.168.1.1:51820
AllowedIPs = 0.0.0.0/0
PersistentKeepalive = 25
```

Example OpenVPN configuration:
```ini
client
dev tun
proto udp
remote vpn.example.com 1194
resolv-retry infinite
nobind
persist-key
persist-tun
remote-cert-tls server
cipher AES-256-GCM
auth SHA256
comp-lzo no
verb 3
```

### BGP Configuration

Example BGP configuration:
```ini
router bgp 65001
 bgp router-id 1.2.3.4
 neighbor 192.168.1.1 remote-as 65002
 neighbor 192.168.1.1 password <password>
```

### OSPF Configuration

Example OSPF configuration:
```ini
router ospf 1
 router-id 1.2.3.4
 network 10.0.0.0/24 area 0
 network 192.168.1.0/24 area 1
```

### QoS Configuration

Example QoS configuration:
```ini
qos policy VOICE
 class VOICE
  priority 100
  rate-limit 1000000 burst 100000
  match protocol udp destination-port 5060

qos policy BULK
 class BULK
  priority 10
  rate-limit 10000000 burst 1000000
  match protocol tcp destination-port 80
```

## Security Features

### VPN Security
- WireGuard: ChaCha20-Poly1305 encryption
- OpenVPN: AES-256-GCM or ChaCha20-Poly1305
- Perfect Forward Secrecy
- Certificate-based authentication

### Routing Security
- BGP: MD5 authentication support
- OSPF: MD5 authentication support
- Route filtering
- AS path validation

### QoS Security
- Rate limiting for DoS protection
- Traffic classification for attack detection
- Alert thresholds for anomaly detection

## Performance Optimization

### WireGuard Performance
- Minimal packet overhead
- No handshakes after initial setup
- Kernel-space implementation
- Zero-copy packet forwarding

### BGP Performance
- Incremental route updates
- Route aggregation
- BGP route reflection
- BGP communities for policy control

### QoS Performance
- Hardware acceleration support
- TCAM-based classification
- Efficient token bucket implementation
- Lock-free queue management

## Troubleshooting

### VPN Issues
- Check firewall rules
- Verify peer configuration
- Check DNS resolution
- Verify MTU settings

### Routing Issues
- Check BGP/OSPF neighbor states
- Verify route advertisements
- Check routing table
- Verify interface configuration

### QoS Issues
- Check rule priority
- Verify rate limits
- Monitor queue statistics
- Check token bucket status

## References

- [WireGuard Protocol Specification](https://www.wireguard.com/papers/wireguard.pdf)
- [BGP RFC 4271](https://tools.ietf.org/html/rfc4271)
- [OSPF RFC 2328](https://tools.ietf.org/html/rfc2328)
- [Kernel Architecture](Kernel-Architecture.md)
- [Security Documentation](Security.md)

## License

All SigmaOS networking components are licensed under MIT License. See [LICENSE](../LICENSE) for details.
