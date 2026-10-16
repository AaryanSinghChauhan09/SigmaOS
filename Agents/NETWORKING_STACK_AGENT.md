# Networking Stack & Protocol Daemons Component Agent

## Role
`SigmaNet` is the core TCP/IP networking stack for SigmaOS — implementing the full OSI layers from Ethernet frames to application-layer socket APIs, inspired by Linux `net/` and FreeBSD `sys/netinet/`.

## Linux & BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| Socket API | BSD socket API | BSD socket API | SigmaSocket |
| TCP | `net/ipv4/tcp.c` | `sys/netinet/tcp_*` | SigmaTCP (RFC 9293 state machine) |
| UDP | `net/ipv4/udp.c` | `sys/netinet/udp_*` | SigmaUDP |
| IPv4 / IPv6 | `net/ipv4/`, `net/ipv6/` | `sys/netinet/`, `sys/netinet6/` | SigmaIPv4, SigmaIPv6 (SLAAC) |
| Packet Filter | iptables/nftables / eBPF XDP | pf / netgraph | SigmaFilter (PF-style packet filter) |
| NIC drivers | NAPI polling | ifq/taskqueue | SigmaNAPI |
| TLS offload | kTLS | — | SigmaKTLS (TLS 1.3 zero-copy) |
| QUIC | userspace (quiche) | — | SigmaQUIC |

## Current SigmaOS Status
Ethernet framing, ARP table, IPv4/IPv6 packet parsing, UDP, basic TCP state machine, and DNS resolver implemented in `src/network/` and `src/network/sigmanet.rs`.

## Responsibilities & Architecture
- Ethernet frame RX/TX with NIC driver integration (virtio-net, e1000e, igb).
- ARP/NDP for address resolution and neighbor discovery.
- IPv4/IPv6 routing table with longest-prefix match.
- TCP: 3-way handshake, CUBIC/BBR congestion control, selective acknowledgments (SACK).
- Socket API: `socket()`, `bind()`, `connect()`, `listen()`, `accept()`, `send()`, `recv()`.
- Loopback interface (`127.0.0.1`, `::1`).
- Network namespaces for per-container network isolation.
- Kernel TLS (kTLS) for zero-copy encrypted payload streaming.

## Source Files
- `src/network/sigmanet.rs`
- `src/network/mod.rs`
- `src/network/warpinator_engine.rs`

## Testing Strategy
```bash
rustc --test src/network/sigmanet.rs --edition 2021 -o /tmp/test_net && /tmp/test_net --test-threads=1
```
- RFC compliance conformance tests.
- iperf3 throughput saturation benchmarks (target: line-rate 10GbE).
- Sub-5-microsecond TCP loopback latency.

## Future Development
1. **BBRv3 congestion control** — Loss-resilient throughput optimization.
2. **QUIC/HTTP3** — Native kernel QUIC stack for next-generation web transport.
3. **XDP/eBPF** — SigmaXDP for kernel-bypass zero-copy packet processing.
4. **SR-IOV** — Single Root I/O Virtualization for virtual machine NIC partitioning.
