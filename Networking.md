# Networking Stack

SigmaOS ships a custom TCP/IP stack for the bare-metal kernel layer. This is a ground-up implementation — no lwIP, no Linux kernel networking code — designed for silicon-direct execution with zero external dependencies.

---

## Stack Overview

```
Application (syscall: net_socket, net_connect, net_send, net_recv)
         │
         ▼
   Socket Layer (FD table, connection state)
         │
         ▼
   TCP State Machine ──── UDP Datagram Handler
         │
         ▼
   IP Layer (routing table, fragmentation)
         │
         ▼
   NIC Driver (loopback lo, virtio-net, e1000)
         │
         ▼
   Physical / Virtual Network
```

---

## Socket API

All networking is accessed through POSIX-inspired socket calls mapped to SigmaOS syscalls.

### `net_socket`

```c
sigma_i32 net_socket(sigma_i32 domain, sigma_i32 type, sigma_i32 protocol);
```

Allocates a new socket slot in the kernel socket table.

- `domain`: `AF_INET` (2) — IPv4 only.
- `type`: `SOCK_STREAM` (1) for TCP, `SOCK_DGRAM` (2) for UDP.
- Returns: file descriptor (≥0) on success, `-EMFILE` if the socket table is full.

---

### `net_connect`

```c
sigma_i32 net_connect(sigma_i32 fd, sigma_u32 remote_ip, sigma_u16 remote_port);
```

Initiates the TCP 3-way handshake to a remote host:

```
Client                 Server
  │── SYN ──────────────▶│
  │◀── SYN-ACK ──────────│
  │── ACK ──────────────▶│
  │  (connection open)   │
```

- `remote_ip`: IPv4 address in network byte order.
- `remote_port`: Port in network byte order.
- Returns: `0` on success, `-ECONNREFUSED` if the server sent RST, `-ETIMEDOUT` if no response within the retransmission limit.

---

### `net_send` / `net_recv`

```c
sigma_i32 net_send(sigma_i32 fd, const void* data, sigma_size_t size);
sigma_i32 net_recv(sigma_i32 fd, void* buf, sigma_size_t len);
```

Send and receive data on an open TCP connection. `net_recv` blocks until data is available or the connection is closed.

---

## TCP State Machine

The TCP implementation tracks connection state per socket:

```
CLOSED → LISTEN → SYN_RECEIVED → ESTABLISHED → FIN_WAIT_1 → FIN_WAIT_2 → TIME_WAIT → CLOSED
         SYN_SENT → ESTABLISHED → CLOSE_WAIT → LAST_ACK → CLOSED
```

Key features:
- **Retransmission timer**: Unacknowledged segments are retransmitted with exponential backoff (initial RTO 1s, max 64s).
- **Window scaling**: Receive window advertised to the peer, respecting available socket buffer space.
- **RST handling**: Unexpected RST segments close the socket and wake any blocked `net_recv` call.

---

## DNS Resolver

A lightweight local DNS resolver maps domain names to IPv4 addresses:

```c
sigma_u32 net_resolve(const char* hostname);
// Returns: IPv4 address in network byte order, or 0 on failure
```

For the bare-metal environment, the resolver uses a static host table loaded from `/etc/hosts`. Dynamic DNS (UDP queries to a configured nameserver) is on the roadmap.

---

## Loopback Interface

The loopback device (`lo`, `127.0.0.1/8`) is a virtual NIC implemented entirely in software. Packets sent to `127.x.x.x` are looped back directly to the receive path without touching any hardware.

This is the primary network interface used in QEMU development builds. The Go daemons and the kernel communicate over loopback sockets.

---

## Connection Tracking (Conntrack)

The `sigma_shield.cpp` firewall maintains a connection tracking table to correlate packets with established flows. Key design constraints:

- The `g_conntrack_entries` counter is **incremented when a new entry is created and decremented when it is removed**. The counter always equals the actual number of live entries.
- A configurable `CONNTRACK_MAX` limit (default: 65536) caps the table size. When the limit is reached, new connection tracking entries are rejected and a warning is logged.

---

## Firewall (sigma_shield)

The firewall evaluates rules against **actual packet header fields** — not mocked or hardcoded values. Each incoming packet is parsed to extract:

- Source IP, destination IP
- Protocol (TCP=6, UDP=17, ICMP=1)
- Source port, destination port
- Connection tracking state (NEW, ESTABLISHED, RELATED, INVALID)

Rules are evaluated in order. The first matching rule determines the verdict (`NF_ACCEPT`, `NF_DROP`). Each rule has a hit counter updated with the real packet's 5-tuple, enabling accurate traffic statistics.

---

## Web Shell Networking (daemon layer)

When a web app uses `navigator.sigmaos.process.spawn()` with `net:host` capability, the spawned process runs in a network namespace with a veth pair connecting it to the host network via NAT. Without `net:host`, the process gets only loopback — it cannot make outbound connections.

---

*See also: [Kernel](Kernel) · [Security Model](Security-Model) · [Architecture Overview](Architecture-Overview)*
