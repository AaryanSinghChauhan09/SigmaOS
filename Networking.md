# SigmaOS Networking Stack

SigmaOS ships a sovereign, zero-trust network stack built from scratch — no Linux `net/` code, no legacy socket API baggage.

---

## Stack Overview

```
Applications (user space)
    │  sys_socket / sys_connect / sys_send / sys_recv
    ▼
Socket layer (sigma_socket.cpp)
    │
    ├── TLS 1.3 + Kyber-1024 hybrid         (all connections)
    ├── QUIC (planned)
    └── Raw TCP/UDP
         │
         ▼
Transport layer (sigma_tcp.c, sigma_udp.c)
    │  TCP RFC 793 state machine, UDP
    ▼
Network layer (sigma_ip.c)
    │  IPv4 + IPv6, ICMP, routing table
    ▼
Link layer (sigma_ethernet.c)
    │  Ethernet II frames, ARP
    ▼
NIC Driver (SDF layer)
    │  e1000, iwlwifi, rtl8xxxu, VirtIO-net
    ▼
Hardware
```

---

## Implemented Features

### Transport
- TCP state machine (SYN → ESTABLISHED → FIN)
- UDP datagram sockets
- ICMP echo (ping)
- IPv4 + IPv6 dual-stack
- ARP request/reply

### Security & Privacy
- **TLS 1.3**: `net/tls/` — X25519/Kyber-1024 hybrid key exchange
- **WPA3/SAE**: dragonfly key exchange (P-256) for Wi-Fi
- **WireGuard VPN**: `net/vpn/sigma_wireguard.cpp`
- **DNS-over-HTTPS**: all DNS queries encrypted by default
- **DNSSEC**: validation via `net/dns/sigma_dnssec.cpp`
- **Stateful firewall**: `net/firewall/sigma_firewall.cpp`
- **NAT + conntrack**: `net/sigma_nat.cpp`
- **Zero-trust**: every connection requires SPIFFE workload identity

### Connectivity
- **DHCP client**: RFC 2131/2132 full state machine
- **CRDT offline sync**: `net/sigma_offline_sync.cpp` — merge without server
- **Mesh networking**: `net/mesh/` — ZeroNet for `release/distributed`

---

## DNS Architecture

```
sigma_gethostbyname("example.com")
    │
    ▼
sigma_dns_resolve()
    ├── Check LRU cache (TTL-aware)
    ├── HIT → return cached answer
    └── MISS →
          ├── Try DNS-over-HTTPS (primary)
          ├── Fall back to UDP DNS (if DoH unavailable)
          └── Validate DNSSEC signature
                └── Cache result → return
```

---

## Network Daemons

| Daemon | Socket | Purpose |
|--------|--------|---------|
| `sigmad-netd` | `/run/sigma/netd.sock` | Interface management, routing |
| `sigmad-dnsd` | `/run/sigma/dnsd.sock` | Local DNS resolver |
| `sigmad-dhcpd` | `/run/sigma/dhcpd.sock` | DHCP client daemon |
| `sigmad-vpnd` | `/run/sigma/vpnd.sock` | WireGuard VPN management |
| `sigmad-firewalld` | `/run/sigma/fwd.sock` | Firewall rule management |

---

## Open Issues (Phase G)

| ID | Issue | Status |
|----|-------|--------|
| #851-WLAN | Wi-Fi 6 driver (iwlwifi) | ⬜ Phase G |
| #851-BT | Bluetooth 5.3 HCI | ⬜ Phase G |
| #1012 | Full TCP/UDP RFC 793 state machine | ⬜ Phase G |
| — | QUIC transport | ⬜ Phase H |

---

## Source Files

| File | Purpose |
|------|---------|
| `net/sigma_net.cpp` | Stack initialisation |
| `net/tcp.c` | TCP state machine |
| `net/udp.c` | UDP datagrams |
| `net/sigma_ip.c` | IPv4 routing |
| `net/sigma_ethernet.c` | Ethernet II framing |
| `net/dns.c` | DNS resolver |
| `net/dhcp.c` | DHCP client |
| `net/tls/` | TLS 1.3 + PQC |
| `net/vpn/` | WireGuard |
| `net/firewall/` | Stateful firewall |
| `net/mesh/` | ZeroNet mesh |
| `net/sigma_offline_sync.cpp` | CRDT sync |
| `drivers/net/sigma_iwlwifi.cpp` | Wi-Fi 6 driver (Phase G) |

---

*See also: [Architecture-Overview](Architecture-Overview) · [Security-Model](Security-Model) · [System-Daemons](System-Daemons)*
