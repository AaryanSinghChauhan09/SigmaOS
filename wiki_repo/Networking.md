# SigmaOS Networking Stack

Complete IPv4/IPv6 + TCP/UDP stack implemented in `kernel/net/`.

---

## Architecture

```
User process
    │  socket() → connect() → send() → recv()
    ▼
Socket API (kernel/net/socket.rs)
    │  maps POSIX fds to TCP/UDP sockets
    ▼
TCP (kernel/net/tcp.rs)         UDP (kernel/net/udp.rs)
    │                               │
    └──────────┬────────────────────┘
               ▼
         IPv4/IPv6 Layer (kernel/net/ip.rs)
               │
         sigma_ip_send() → sigma-bus IPC_CH_NET_TX
               │
         NIC Driver (e1000, virtio-net)
```

---

## TCP Implementation

Full RFC 793 state machine. See [TCP-Stack](TCP-Stack) for full details.

```c
int fd = tcp_socket_create();

// Client connect
tcp_connect(fd, 0xC0A80001, 80);   // 192.168.0.1:80
// state: CLOSED → SYN_SENT → ESTABLISHED

// Send data
tcp_send(fd, data, len);

// Receive data (returns bytes read, 0 = EOF)
n = tcp_recv(fd, buf, sizeof(buf));

// Close (graceful FIN handshake)
tcp_close(fd);
```

---

## UDP Implementation

```c
int fd = udp_socket();
udp_bind(fd, 5353);                  // bind port 5353 (mDNS)
udp_sendto(fd, data, len, dst_ip, dst_port);
n = udp_recvfrom(fd, buf, sizeof(buf));
```

---

## POSIX Socket API

```c
// Create socket
int fd = sigma_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
int fd = sigma_socket(AF_INET, SOCK_DGRAM,  IPPROTO_UDP);

// Connect (TCP)
struct SockAddrIn addr = { AF_INET, htons(80), htonl(0xC0A80001) };
sigma_connect(fd, &addr, sizeof(addr));

// Send / receive
sigma_send(fd, buf, len, 0);
sigma_recv(fd, buf, len, 0);

// Close
sigma_socket_close(fd);
```

---

## IP Layer

```c
// Send raw IP packet (used by TCP/UDP internally)
sigma_ip_send(dst_ip, IPPROTO_TCP, payload, len);

// Receive IP packet (called from NIC IRQ handler)
sigma_ip_rx(data, len);
```

Features:
- IPv4 header construction with auto-checksum
- IPv6 header support
- ICMP echo request/reply (ping)
- ARP cache (64 entries, LRU eviction)
- Routing table (32 routes, longest-prefix match)

---

## DNS + DHCP (Already Implemented)

```bash
# DNS query (from sigma-sh)
sigma-net resolve example.com     # → 93.184.216.34

# DHCP (automatic at boot)
sigmad-netd --dhcp eth0            # RFC 2131 state machine
```

Sources: `net/sigma_dns.rs`, `net/sigma_dhcp.rs`

---

## TLS 1.3

```bash
# Secure connection using Kyber-1024 hybrid key exchange
sigma-net connect --tls https://example.com
```

Cipher suites:
- `TLS_AES_256_GCM_SHA384`
- `TLS_KYBER1024_AES256GCM_SHA384` (PQC hybrid)

Source: `net/sigma_tls.rs`

---

## Network Configuration

```bash
# Manual IP configuration
sigma-net set-ip eth0 192.168.0.2/24
sigma-net set-gw eth0 192.168.0.1

# DHCP
sigma-net dhcp eth0

# Show interfaces
sigma-net list
```

---

## Hardware Drivers

| Driver | Hardware | Status |
|--------|----------|--------|
| e1000 | Intel 82540/82545/etc. (QEMU default) | ✅ |
| virtio-net | QEMU/KVM virtual NIC | 🔄 |
| rtl8139 | Realtek RTL8139 (QEMU alt) | 🔄 |
| iwlwifi | Intel Wi-Fi 6 | 🔄 partial |

---

*Sources: `kernel/net/ip.rs`, `kernel/net/tcp.rs`, `kernel/net/udp.rs`, `kernel/net/socket.rs`*
