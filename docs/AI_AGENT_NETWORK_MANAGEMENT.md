# AI Agent Guidelines for SigmaOS Broad Network Access Management

This document provides guidelines, architectural specifications, and verification protocols for AI agents developing, inspecting, or extending the **SigmaOS Network Subsystem & Broad Network Access Management**.

---

## 1. System Architecture & Network Subsystem Layout

SigmaOS implements a modular, high-performance networking architecture across two primary Rust subsystems:

1. **`src/net/` (Core Network Transport & Application Stack)**
   - `src/net/tcp_ip_implementation.rs`: Core TCP/IP stack implementation (`TcpIpStack`), `TcpSocket`, `UdpSocket`, IPv4/MAC addressing, ARP table, DNS resolver, and DHCP client.
   - `src/net/stack.rs`: Network device interface (`NetDevice`), Netfilter firewall rule evaluation (`Netfilter`, `NetfilterRule`, `NFAction`), queueing disciplines (`QdiscManager`, `PfifoFast`), and congestion control algorithms (`BbrCongestionControl`, `RenoCongestionControl`).
   - `src/net/socket.rs`: Low-level BSD socket abstractions and connection state management.
   - `src/net/dns.rs` & `mesh.rs`: Domain Name System resolution and peer-to-peer mesh networking overlay.
   - `src/net/torrent.rs`: Native BitTorrent client (`TorrentClient`), DHT routing table (`DhtRoutingTable`), and uTP delay controller.

2. **`src/network/` (Network Management, Firewalls, Wireless & Remote Sharing)**
   - `src/network/zenithnet.rs`: Native ZenithNet TCP/IP networking engine (`ZenithNet`), network interface management (`NetworkInterface`), Ethernet frames, and IPv4/TCP/UDP packet headers.
   - `src/network/socket.rs`: BSD Socket table (`SocketTable`), socket types (`SocketType`), address families (`AddressFamily`), options, and connection state machine.
   - `src/network/security.rs`: System firewall (`Firewall`, `FirewallRule`, `FirewallAction`), network protocol classifier (`NetworkProtocol`), and TLS configuration/cipher suites (`TlsConfig`, `TlsCipherSuite`).
   - `src/network/wireless_manager.rs`: Wireless & Bluetooth hardware manager (`WirelessManager`), Wi-Fi security profiles (`WifiProfile`, `WifiSecurity`), and Bluetooth device pairing.
   - `src/network/sovereign_remote_sharing.rs`: Sovereign remote sharing suite including SSH (`SovereignSshEngine`), Samba/SMB (`SovereignSambaEngine`), NFS (`SovereignNfsEngine`), Rsync (`SovereignRsyncEngine`), and SCP (`SovereignScpEngine`).
   - `src/network/discovery.rs`: Network service discovery (`SovereignNetworkDiscoveryEngine`), mDNS/DNS-SD, SSDP discovery, and LLMNR/NBNS resolution.
   - `src/network/routing.rs`: Packet forwarding engine (`RoutingEngine`), routing table (`RoutingTable`), and forwarding decisions (`ForwardingDecision`).
   - `src/network/ring_buffer_stack.rs`: High-performance ring buffer packet processor (`PacketRingBuffer`) and hardware checksum calculation (`compute_checksum`).

---

## 2. Network Security, Firewalls & Access Controls

AI agents modifying network policies, security rules, or firewall enforcement must follow these core security patterns:

### Firewall Rule Enforcement
Firewall rules in `src/network/security.rs` and `src/net/stack.rs` filter ingress and egress network packets based on:
- **Network Protocol:** `NetworkProtocol` (`Tcp`, `Udp`, `Icmp`, `Arp`).
- **Firewall Action:** `FirewallAction` / `NFAction` (`Accept`, `Drop`, `Reject`, `Log`).
- **Rule Specification:** Source/Destination IP subnets, port ranges, and stateful connection tracking.

```rust
use sigma::network::security::{Firewall, FirewallRule, FirewallAction, NetworkProtocol};

let mut firewall = Firewall::new();
let rule = FirewallRule {
    id: 1,
    protocol: NetworkProtocol::Tcp,
    source_ip: None,
    destination_ip: None,
    destination_port: Some(22),
    action: FirewallAction::Accept,
};
firewall.add_rule(rule);
```

### Network Isolation & Containers
Container and process network isolation is managed via network namespaces:
- Isolated loopback interface per namespace.
- Virtual Ethernet (`veth`) pair creation and inter-namespace bridging.
- Socket table isolation per process namespace via `SocketTable`.

---

## 3. Remote Sharing & Discovery Services

SigmaOS provides native implementations of standard network enterprise protocols:

| Protocol | Engine / Structure | Source Location | Capability |
| :--- | :--- | :--- | :--- |
| **SSH / SCP** | `SovereignSshEngine`, `SovereignScpEngine` | `src/network/sovereign_remote_sharing.rs` | Certificate auth, multiplexing, SFTP file transfer |
| **Samba / SMB** | `SovereignSambaEngine`, `SmbSession` | `src/network/sovereign_remote_sharing.rs` | SMB2/SMB3 file share serving and access control |
| **NFS** | `SovereignNfsEngine`, `NfsExportRule` | `src/network/sovereign_remote_sharing.rs` | NFSv4 compound operations and lock management |
| **Rsync** | `SovereignRsyncEngine` | `src/network/sovereign_remote_sharing.rs` | Rolling checksum delta synchronization |
| **Discovery** | `SovereignNetworkDiscoveryEngine` | `src/network/discovery.rs` | mDNS, SSDP, LLMNR, and ICMPv6 NDP discovery |

---

## 4. Testing & Verification Protocol for AI Agents

When making changes to network source files or network access control policies, AI agents must run the following validation commands in order:

### 1. Standalone Module Test Execution
Automated standalone compilation verifies that modified Rust source files compile and pass their unit test suites independently:

```bash
./scripts/changed_files_rustc_tests.sh
```

Alternatively, test specific network modules directly using `rustc`:

```bash
rustc --test --edition=2021 src/net/tcp_ip_implementation.rs -o build/test_tcpip && ./build/test_tcpip
rustc --test --edition=2021 src/network/security.rs -o build/test_net_sec && ./build/test_net_sec
rustc --test --edition=2021 src/network/sovereign_remote_sharing.rs -o build/test_sharing && ./build/test_sharing
```

### 2. Full System Integration & Inspection Suite
Run the master test script to validate all C++ test runners, inspection test binaries, Python test suites, and core network stack subsystems:

```bash
./run_sigma_tests.sh
```

---

## 5. Coding Standards & Import Conventions

- **`#![no_std]` Compatibility:** When editing core kernel network drivers or low-level frame parsers, maintain `#![no_std]` compatibility by importing types from `alloc::` (`alloc::string::String`, `alloc::vec::Vec`).
- **Thread Safety & Async Ring Buffers:** Network packet queues must use lock-free or spinlock-protected ring buffers (`PacketRingBuffer`) to guarantee non-blocking packet ingestion.
- **Verification Rule:** Always use `read_file` or `list_files` after modifying codebase files to confirm that all edits were correctly applied.
