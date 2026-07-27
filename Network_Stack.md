# 🌐 TCP/UDP Network Stack

## Overview

SigmaOS includes a **from-scratch, libc-free TCP/IP and UDP networking stack** implemented in pure Rust with no external networking frameworks. The implementation lives in `src/network/tcp_udp.rs` and provides the full network layer needed for sovereign connectivity.

---

## TCP State Machine

The TCP implementation models all 10 RFC 793 states using an atomic state variable:

```rust
pub enum TCPState {
    Closed = 0,
    Listen = 1,
    SynSent = 2,
    SynReceived = 3,
    Established = 4,
    FinWait1 = 5,
    FinWait2 = 6,
    CloseWait = 7,
    Closing = 8,
    TimeWait = 9,
}
```

### Connection Lifecycle

```
Client                        Server
  |                              |
  |------- SYN ---------->       |  → SynSent
  |       <-------- SYN-ACK ---- |  → Established (simulated 3-way handshake)
  |------- ACK ---------->       |
  |                              |
  |===== DATA TRANSFER =====     |
  |                              |
  |------- FIN ---------->       |  → FinWait1/FinWait2
  |       <-------- ACK -----    |
  |       <-------- FIN -----    |
  |------- ACK ---------->       |  → TimeWait → Closed
```

---

## UDP Zero-Copy Datagrams

UDP is implemented via the `UDPSocket` trait with minimal-copy semantics:

```rust
pub trait UDPSocket {
    fn sendto(&mut self, data: &[u8], remote_port: Port) -> Result<usize, NetworkError>;
    fn recvfrom(&mut self, buffer: &mut [u8]) -> Result<(usize, Port), NetworkError>;
}
```

The `ZeroCopyNetwork` struct stores only a DMA buffer pointer, avoiding any intermediate copies between NIC buffers and user space.

---

## Congestion Control

Two congestion control algorithms are provided, both implementing the `CongestionControl` trait:

### TCP Reno

Classic AIMD (Additive Increase, Multiplicative Decrease):
- **Slow Start**: `cwnd += acked` until `cwnd >= ssthresh`
- **Congestion Avoidance**: `cwnd += 1` per RTT
- **Loss Event**: `ssthresh = cwnd / 2`, `cwnd = 1`

### BBR (Bottleneck Bandwidth and RTT)

Google's model-based algorithm:
- Estimates `bw_estimate` and `rtt_min` continuously
- Sets `cwnd = bw_estimate * rtt_min` (BDP-based)
- Handles loss by halving cwnd without entering slow-start

---

## Firewall

The `SimpleFirewall` uses a heap-allocated port permission bitmap:

```rust
pub struct SimpleFirewall {
    pub allowed_ports: Vec<AtomicUsize>,  // 65,536 entries
}
```

All ports are blocked by default. Ports must be explicitly opened:

```rust
firewall.allow_port(80);    // HTTP
firewall.allow_port(443);   // HTTPS
firewall.allow_port(22);    // SSH
```

---

## Network Stack

The `SimpleNetworkStack` aggregates sockets, firewall, and congestion control into a unified API:

```rust
let mut stack = SimpleNetworkStack::new();

// TCP server
let server_id = stack.create_socket(Protocol::TCP, 8080)?;
// ... accept, send, recv

// UDP client
let udp_id = stack.create_socket(Protocol::UDP, 1234)?;
// ... sendto, recvfrom
```

---

## Zero-Trust Integration

All network connections require a valid `CapabilityToken` specifying allowed protocols and ports. Without a capability, the Zero-Trust engine in `src/network/zero_trust.rs` will deny the connection at the policy layer before it reaches the socket layer.

---

## Tests

```
test network::tcp_udp::tests::test_tcp_connection ... ok
test network::tcp_udp::tests::test_udp_socket ... ok
test network::tcp_udp::tests::test_network_stack ... ok
test network::protocols::tests::test_dns_resolution ... ok
test network::protocols::tests::test_mdns_discovery ... ok
test network::protocols::tests::test_quic_h3 ... ok
```


---
## Merged from Network-Stack.md
# S-NET: Sovereign Networking

SigmaOS implements a high-performance network stack designed for industrial reliability and security.

## Features

- **PQC Encryption**: All traffic is encrypted via CRYSTALS-Kyber by default.

- **Lattice Routing**: Shard-aware packet routing to minimize latency in distributed workflows.

- **Hardware Agnostic**: Supports generic WiFi and Ethernet chipsets via the Sovereign Driver Framework.
