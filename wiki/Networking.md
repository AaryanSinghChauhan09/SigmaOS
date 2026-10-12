# Networking Architecture & Stack

The **SigmaOS Network Subsystem** provides a high-throughput, memory-safe, zero-copy network architecture written natively in **Rust** and **Zig**. It combines Linux advanced routing performance with OpenBSD's battle-tested Packet Filter (PF) security and Illumos Crossbow virtualization primitives.

---

## 1. Network Stack Subsystems

### 1.1 Core L2-L4 Protocol Suite
- **Location**: `src/net/`, `src/network/`
- **Ethernet (L2)**: ARP cache with anti-poisoning verification, MAC address learning, IEEE 802.1Q VLAN tagging (`src/network/vlan.rs`), and VXLAN network overlay encapsulation (`src/network/vxlan.rs`).
- **IPv4 / IPv6 (L3)**: Classless Inter-Domain Routing (CIDR), ICMPv6 Neighbor Discovery Protocol (NDP), Router Solicitation, Path MTU Discovery, and dynamic multi-path routing (`src/network/ipv6_routing.rs`).
- **TCP / UDP (L4)**: RFC 793 / 9293 compliant TCP state machine with BBR and CUBIC congestion control, Selective Acknowledgments (SACK), and SYN cookies for SYN flood mitigation.

### 1.2 Multipath TCP (MPTCP)
- **Location**: `src/network/mptcp.rs`
- **Specification**: RFC 8684 Multipath TCP implementation. Dynamically aggregates cellular, Wi-Fi, and Ethernet links into a single logical TCP connection with subflow management and cross-path failover.

### 1.3 Kernel TLS (kTLS) Offload
- **Location**: `src/network/ktls.rs`
- **Specification**: Moves symmetric AES-GCM and ChaCha20-Poly1305 encryption/decryption into kernel socket send/receive routines. Enables zero-copy `splice` and `sendfile` from NVMe storage directly to encrypted network interfaces.

### 1.4 Stream Control Transmission Protocol (SCTP)
- **Location**: `src/network/sctp.rs`
- **Specification**: RFC 4960 multi-streaming and multi-homing transport protocol. Provides message-oriented delivery, unordered transmission, and head-of-line blocking avoidance for telecommunications and real-time infrastructure.

### 1.5 OpenBSD Packet Filter (PF) Engine
- **Location**: `src/network/openbsd_pf.rs`
- **Specification**: Stateful packet filter supporting rule tables, Network Address Translation (NAT), port forwarding (`rdr-to`), normalization (`scrub`), and state tracking with anti-spoofing (`antispoof`).

### 1.6 Illumos Crossbow Virtualization
- **Location**: `src/network/illumos_crossbow.rs`
- **Specification**: Enterprise network virtualization architecture. Allows provisioning arbitrary Virtual Network Interfaces (VNICs), dedicated software rings, hardware CPU affinity bindings, and bandwidth policing (token bucket rate limiters) over physical network ports.

### 1.7 WireGuard Post-Quantum Mesh VPN
- **Location**: `src/network/wireguard.rs`
- **Specification**: High-speed ChaCha20-Poly1305 / Noise protocol VPN mesh extended with Post-Quantum ML-KEM pre-shared keys.

---

## 2. Operating System Networking Parity Matrix

| Capability | Linux 6.x | FreeBSD 14 | OpenBSD 7.x | SigmaOS Implementation |
|---|---|---|---|---|
| **Firewall** | nftables / iptables | IPFW / PF | PF (Packet Filter) | Native `openbsd_pf.rs` |
| **Network Virtualization** | macvlan / veth | epair / VNET | vet(4) | Illumos `illumos_crossbow.rs` VNICs |
| **Multipath TCP** | In-kernel MPTCP | Limited | None | Built-in `mptcp.rs` |
| **Overlay Encapsulation** | VXLAN | VXLAN | None | Built-in `vxlan.rs` |
| **SCTP Transport** | lksctp | In-kernel SCTP | In-kernel | Native `sctp.rs` |
| **Kernel TLS** | kTLS | KTLS | None | Built-in `ktls.rs` |

---

## ## AI Agent Maintenance Instructions for Future Releases

1. **Language Invariant**: The network stack and firewall must be implemented in **100% Rust** or **Zig**. No external C network libraries (e.g., libpcap, lwIP) are allowed.
2. **Zero-Copy Hot Paths**: Packet ingress and egress paths MUST use ring-buffer slice passing. Do not allocate heap memory (`Vec<u8>` or `Box`) per individual network packet.
3. **Buffer Overflow Prevention**: All header parsers (IPv4, IPv6, TCP, UDP, SCTP, VXLAN) must perform explicit length and bounds checks before indexing into slice buffers.
4. **Denial of Service Immunity**: Sockets must enforce bounded receive and send queues with backpressure and SYN cookie fallbacks under heavy loads.
5. **Compilation Verification**: All network components must compile without errors:
   ```bash
   cargo check --lib
   ```
