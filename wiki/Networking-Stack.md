# Σ core/net — Sovereign Networking Stack

Standalone networking subsystem with **no dependency on the Linux networking
stack**. Supports classical TCP/IP, post-quantum encrypted channels, and
experimental galactic-scale mesh routing.

## Architecture

```
Application Layer
   └─ socket.rs          (POSIX-style socket API)
         └─ tcpip.rs     (TCP/IP stack)
               ├─ tcp.rs  (TCP state machine)
               ├─ icmp.rs (ICMP echo / error)
               └─ sovereign_net.rs  (SovereignNet overlay)
                     ├─ pqfs.rs       (post-quantum forward secrecy)
                     ├─ mesh_net.rs   (local mesh)
                     └─ galactic_*.rs (long-range mesh routing)
```

## Source Files

| File | Description |
|---|---|
| `socket.rs` | POSIX-compatible socket API (`create`, `bind`, `connect`, `send`, `recv`) |
| `tcp.rs` | TCP state machine (SYN→ESTABLISHED→FIN) |
| `tcpip.rs` | IPv4/IPv6 dual-stack with ARP/NDP |
| `icmp.rs` | ICMP ping / unreachable / redirect |
| `sovereign_net.rs` | Encrypted overlay — wraps TCP frames with ChaCha20-Poly1305 |
| `pqfs.rs` | Post-Quantum Forward Secrecy (Kyber-768 + X25519 hybrid) |
| `mesh_net.rs` | Local gossip-based mesh (IoT / edge) |
| `shard_sync.rs` | Distributed shard-state consensus over the network |
| `consensus.rs` | Byzantine-fault-tolerant consensus primitive |
| `galactic_mesh.rs` | High-latency interplanetary routing layer |

## API Interface

```c
// Create a sovereign socket
int sigma_net_socket(int domain, int type, int protocol);

// Connect to a remote host (blocks until handshake complete)
int sigma_net_connect(int sockfd, const struct sigma_addr *addr);

// Send data — zero-copy if page-aligned
ssize_t sigma_net_send(int sockfd, const void *buf, size_t len, int flags);

// Enable post-quantum encryption on a socket
int sigma_net_enable_pqe(int sockfd, pq_key_pair_t *keys);

// Sovereign init
void init_core_net(void);
```

## Post-Quantum Encryption

All `sovereign_net.rs` channels use a **hybrid** scheme:

| Layer | Algorithm | Standard |
|---|---|---|
| Key Exchange | X25519 + Kyber-768 | NIST ML-KEM |
| Encryption | ChaCha20-Poly1305 | RFC 8439 |
| Integrity | BLAKE3 MAC | — |

## Roadmap

- [x] TCP state machine (`tcp.rs`)

- [x] Socket API (`socket.rs`)

- [x] ICMP (`icmp.rs`)

- [x] Post-quantum encryption stub (`pqfs.rs`)

- [x] Mesh gossip protocol (`mesh_net.rs`)

- [ ] Full IPv6 SLAAC / NDP

- [ ] DHCPv4 / DHCPv6 client

- [ ] DNSSEC resolver integration

- [ ] WireGuard-inspired VPN tunnel

- [ ] Formal Kani proofs for TCP state machine

## Related Modules

- [`modules/core/kernel`](../kernel/README.md) — IPC used by net stack

- [`modules/security/isolation`](../../security/isolation/README.md) — Network namespace isolation
