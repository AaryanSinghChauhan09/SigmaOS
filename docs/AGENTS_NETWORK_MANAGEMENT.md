# SigmaOS Network Stack, eBPF/XDP & PQC VPN Management Guide for AI Agents

This guide provides technical specifications, packet filtering architectures, eBPF/XDP zero-copy processing pipelines, Post-Quantum Cryptography (PQC) VPN engines, and network policy enforcement rules for AI agents managing networking in SigmaOS.

---

## 1. Zero-Dependency `#![no_std]` Network Architecture

SigmaOS implements a high-performance network stack without external crate dependencies (`src/net/`, `src/open_source_obsoletion.rs`, `src/open_source_os_gap_closure.rs`):

* **High-Performance eBPF/XDP Processing (`process_xdp_zero_copy_packet` in `src/open_source_os_gap_closure.rs`):**
  Direct ring-buffer kernel-bypass driver hooks for $O(1)$ packet filtering and microsecond-level forwarding (obsoletes DPDK / kernel iptables overhead).
* **PQC VPN & Firewall Engine (`SovereignPqcVpnFirewall` in `src/open_source_obsoletion.rs`):**
  Post-Quantum Cryptography (Kyber / Dilithium) key exchange, WireGuard-inspired peer tunnel handshake, and stateful packet filtering (obsoletes WireGuard / iptables).
* **Cilium BPF Network Engine (`SovereignCiliumBpfNetworkEngine` in `src/open_source_obsoletion.rs`):**
  Container Network Interface (CNI) eBPF load balancing, identity-based L3/L4 policy enforcement, and egress NAT (obsoletes Cilium / Calico / Flannel CNI).
* **Packet Inspection & Sniffing (`SovereignPacketInspector` in `src/open_source_obsoletion.rs`):**
  Zero-copy promiscuous ring buffer capture, PCAP file stream writing, and deep protocol dissection for IP, TCP, UDP, ICMP, DNS, and HTTP (obsoletes Wireshark / tcpdump).
* **OpenBSD PF Firewall Parity (`src/unimplemented_features.rs`):**
  Stateful packet inspection (`keep state`), bandwidth queueing (`ALTQ`), and NAT rule evaluation inspired by OpenBSD `pf`.
* **Bluetooth Stack & Subsystem (`src/bluetooth/`):**
  Linux BlueZ `bluetoothctl` and FreeBSD/NetBSD `hciconfig` inspired zero-dependency Bluetooth HCI driver layer, L2CAP channel multiplexing, RFCOMM serial TTY bonding, and GATT/HID profiles.

---

## 2. eBPF/XDP Zero-Copy Packet Pipeline Guidelines

When modifying packet filter drivers or eBPF network hooks:

1. **Zero-Copy Ring Allocation:**
   Network interfaces MUST allocate DMA ring buffers using `dma_ring_buffer_allocator` to prevent kernel-to-userland buffer copy penalties.
2. **XDP Filter Returns:**
   eBPF XDP hook functions MUST explicitly return canonical action codes:
   * `XDP_PASS`: Pass packet up to kernel network stack.
   * `XDP_DROP`: Drop packet immediately at network interface card (NIC) level.
   * `XDP_TX`: Bounce packet back out the same interface.
   * `XDP_REDIRECT`: Forward packet directly to another interface or socket.

---

## 3. PQC VPN Security & Tunnel Peer Rules

* **Tunnel Handshakes:**
  Every PQC VPN peer MUST establish session keys via hybrid post-quantum key encapsulation before data packet transmission.
* **Network Input Validation:**
  Network frame parsers MUST validate IP header lengths and payload boundaries to prevent buffer overread vulnerabilities (tested via `tests::test_ipv4_validation` and `tests::test_ipv6_validation`).

---

## 4. Checklist for AI Agents Managing Network Subsystems

1. **Verify #![no_std] Compatibility:** Ensure network protocol parsers avoid `std::net` and utilize native `crate::klib` or `alloc` types.
2. **Test Network Security & Input Parsers:**
   Run input validation and security inspection tests:
   ```bash
   cargo test --lib -- net::tests
   ./run_sigma_tests.sh
   ```
