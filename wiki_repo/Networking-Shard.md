# 🌐 SigmaOS Networking Stack

> **Raw Sovereignty from Ethernet Frame to Application Layer.**

The entire SigmaOS networking stack is built from the ground up with zero reliance on BSD sockets, lwIP, or any POSIX networking APIs.

---

## Architecture

```
┌─────────────────────────────┐
│  sigma_net_dns.cpp (DNS)    │
│  sigma_firewall.cpp (FW)    │
├─────────────────────────────┤
│  sigma_net_socket.cpp       │  ← Sovereign Socket API
├─────────────────────────────┤
│  sigma_tcp.cpp (TCP FSM)    │
│  sigma_ipv6.cpp (IPv6+NDP)  │
├─────────────────────────────┤
│  sigma_e1000.cpp (NIC)      │
│  sigma_rtl8139.cpp (NIC)    │
└─────────────────────────────┘
```

---

## Sovereign Socket API (`sigma_net_socket.cpp`)

Replaces POSIX `<sys/socket.h>` entirely.

| Function | Purpose |
|----------|---------|
| `sigma_net_socket_create(proto)` | Create TCP/UDP/RAW socket |
| `sigma_net_socket_bind(sock, ip, port)` | Bind to local address |
| `sigma_net_socket_connect(sock, ip, port)` | Initiate connection |
| `sigma_net_socket_send(sock, data, len)` | Send data |
| `sigma_net_socket_recv(sock, buf, max)` | Receive data |
| `sigma_net_socket_close(sock)` | Close socket |

- Up to 1024 concurrent sockets
- Ring buffer TX/RX for zero-copy IPC

---

## TCP Stack (`sigma_tcp.cpp`)

**Absorbs**: RFC 793, Linux `tcp.c` state machine, uIP embedded stack.

Implements the **full TCP finite state machine**:
- Three-way handshake (SYN → SYN-ACK → ACK)
- 11 states: CLOSED → LISTEN → SYN_SENT → SYN_RCVD → ESTABLISHED → FIN_WAIT1/2 → CLOSE_WAIT → CLOSING → LAST_ACK → TIME_WAIT
- RFC 1071 one's complement checksum
- Retransmission timer (3s RTO)

---

## IPv6 (`sigma_ipv6.cpp`)

**Absorbs**: RFC 8200, Linux `net/ipv6/`.

- Fixed 40-byte header parsing
- Next-header routing (TCP=6, UDP=17, ICMPv6=58)
- NDP Neighbor Cache (32 entries)
- Address comparison for 128-bit addresses

---

## Firewall (`sigma_firewall.cpp`)

**Absorbs**: `iptables` / `nftables` chain architecture.

- 128 firewall rules
- Match by: source IP/mask, dest IP/mask, dest port, protocol
- Actions: `ACCEPT`, `DROP`
- Default policy: ACCEPT

---

## DNS Resolver (`sigma_net_dns.cpp`)

- Manually constructs raw UDP DNS query packets
- Formats domain names into DNS wire format
- Sends to port 53 via sovereign socket API
- Parses A record answers
