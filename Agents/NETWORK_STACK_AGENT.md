# Network Stack Agent

## Role
`SigmaNet` is the core TCP/IP networking stack for SigmaOS — implementing the full OSI layers from Ethernet frames to application-layer socket APIs, inspired by Linux net/ and FreeBSD sys/netinet/.

## Status
🟡 In Progress — Basic TCP/UDP in `src/network/sigmanet.rs`; full socket API, QUIC, and IPv6 needed.

## Source File
`src/network/sigmanet.rs`, `src/network/` (multiple modules)

## Linux/BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| Socket API | BSD socket API | BSD socket API | SigmaSocket |
| TCP | `net/ipv4/tcp.c` | `sys/netinet/tcp_*` | SigmaTCP |
| UDP | `net/ipv4/udp.c` | `sys/netinet/udp_*` | SigmaUDP |
| IPv6 | `net/ipv6/` | `sys/netinet6/` | SigmaIPv6 |
| Netfilter/PF | iptables/nftables | pf | SigmaFilter |
| NIC drivers | NAPI polling | ifq/taskqueue | SigmaNAPI |
| TLS offload | kTLS | — | SigmaKTLS |
| QUIC | (userspace: quiche) | — | SigmaQUIC |
| XDP | AF_XDP + BPF | — | SigmaXDP |

## Responsibilities
- Ethernet frame RX/TX with NIC driver integration
- ARP/NDP for address resolution
- IPv4/IPv6 routing and forwarding
- TCP: 3-way handshake, congestion control (CUBIC/BBR), retransmit
- UDP: connectionless datagram delivery
- Socket API: `socket()`, `bind()`, `connect()`, `send()`, `recv()`, `select()`
- Loopback interface (127.0.0.1, ::1)
- Network namespaces (per-container network isolation)
- Kernel TLS (kTLS) for zero-copy TLS

## Implementation Notes
- `#![no_std]` — ring buffers for RX/TX queues
- TCP state machine: LISTEN → SYN_RCVD → ESTABLISHED → FIN_WAIT_1 → CLOSED
- Socket: `{ family: AF, type: SOCK_*, proto: u8, state: SocketState, rx_buf: RingBuf }`
- CUBIC congestion control: cubic increase function on cwnd

## Testing
```bash
rustc --test src/network/sigmanet.rs --edition 2021 -o /tmp/test_net && /tmp/test_net --test-threads=1
```

## Future Development
1. **BBRv3 congestion control** — Google's latest BBR for throughput
2. **QUIC/HTTP3** — Native kernel QUIC stack (like Linux's experimental)
3. **XDP/eBPF** — SigmaXDP for kernel-bypass packet processing
4. **SR-IOV** — Single Root I/O Virtualization for NIC partitioning
5. **RDMA** — Remote Direct Memory Access for HPC workloads
6. **Network time stamping** — Hardware timestamping for PTP/PPS
