# SigmaOS AI Agent Broad Network Access Management Guidelines

## 1. Overview
SigmaOS incorporates a complete networking stack managed by AI agents (such as `NetworkAccessGovernor`, `DhcpDnsResolver`, `TcpCongestionController`, and `FirewallSecmarkManager`). These guidelines define hardware network drivers (E1000 Ethernet, Broadcom/Intel Wifi), in-kernel DHCP/DNS resolvers, TCP congestion algorithms (CUBIC, BBR), stateful PF/IPTables firewalls, SELinux SECMARK packet labeling, and socket-activated network services.

## 2. Core Broad Network Access Management Principles

### 2.1 Hardware Network Drivers & Multi-Arch NICs
- **Ethernet & Wireless Drivers**: AI agents supervise Intel E1000 Gigabit Ethernet (`E1000Driver` in `src/drivers/intel_e1000.rs`) and 802.11 Wi-Fi drivers (`ModernWifiDriver` in `src/drivers/modern_wifi.rs`).
- **Ring Buffer DMA Transfers**: Transmit (TX) and Receive (RX) descriptors use 64-byte aligned ring buffers with zero-copy packet allocation.

### 2.2 In-Kernel DHCP Client & DNS TLS Resolver
- **DHCP Client**: Automatically obtains IPv4/IPv6 leases, netmasks, gateways, and DNS server configurations (`src/network/dhcp_dns.rs`).
- **DNS-over-TLS (DoT)**: System DNS queries execute over TLS-encrypted sockets to prevent DNS spoofing and eavesdropping.

### 2.3 TCP Congestion Control (CUBIC & BBR) & Window Scaling
- **CUBIC & BBR Algorithms**: High-bandwidth network sockets utilize Linux CUBIC (`CubicCongestionControl`) or Google BBR (`BbrPhase`) congestion window control in `src/network/tcp_udp.rs`.
- **TCP Options**: Enables TCP Selective Acknowledgment (SACK), Window Scaling, and Timestamps for high-speed multi-gigabit connections.

### 2.4 Firewall & SECMARK Context Flow Labeling
- **PF & IPTables Chains**: Agents evaluate stateful packet filter rules across `Filter`, `Nat`, `Mangle`, and `Raw` tables (`src/security/firewall.rs`).
- **SELINUX SECMARK Labeling**: Enforces packet-level SELinux security context labeling (`SecmarkPacketLabel`) for zero-trust network isolation.

---
*Maintained by the SigmaOS Networking & Security Steering Committee.*
