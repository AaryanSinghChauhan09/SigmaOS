# sigma-net — Network Stack Specification

**Status:** Draft · Target: v0.2
**Owner:** net team
**Canonical source:** `net/`, `kernel/net/`

---

## Overview

sigma-net is the networking stack for SigmaOS, inspired by smoltcp's embedded-first design principles but targeting a full-featured OS stack. It provides dual-stack IPv4/IPv6, TCP/UDP/ICMP, ARP, DHCP, DNS/DoH, TLS 1.3 with post-quantum Kyber-1024 hybrid, and a stateful firewall — all in a single coherent codebase.

## Goals

- No unsafe raw pointer aliasing: strict ownership model in all packet parsing

- Zero-copy receive path for data > 1 KB (DMA buffer ring directly to socket buffer)

- TLS 1.3 mandatory for all outbound connections; classical TLS 1.2 rejected

- PQC hybrid: X25519 + Kyber-1024 key exchange (NIST FIPS 203)

- DoH resolver with fallback to UDP DNS; DNSSEC validation

---

## Architecture

```
┌────────────────────────────────────────────────────┐
│  Socket API (POSIX: socket/bind/connect/send/recv) │
├────────────────────────────────────────────────────┤
│  TLS 1.3 layer (X25519+Kyber-1024 hybrid)          │
├───────────────┬────────────────────────────────────┤
│  TCP          │  UDP                               │
├───────────────┴────────────────────────────────────┤
│  IPv4 / IPv6 dual-stack                            │
│  (routing table, fragment reassembly, TTL/hop)     │
├──────────┬─────────┬───────────────────────────────┤
│  ICMP    │  ARP    │  Firewall hooks (netfilter-  │
│  (v4+v6) │  (IPv4) │   inspired: PREROUTING,      │
│           │  NDP    │   FORWARD, POSTROUTING)       │
│           │  (IPv6) │                               │
├───────────┴─────────┴───────────────────────────────┤
│  Device Driver Interface                            │
│  VirtIO-net · e1000 · RTL8169 · loopback            │
└─────────────────────────────────────────────────────┘
```

---

## IPv4/IPv6 Dual Stack

- IPv4: 20-byte header parse/build, options ignored except for route-record

- IPv6: 40-byte fixed header; extension header chain (hop-by-hop, routing, fragment)

- Routing table: longest-prefix match, radix trie, `ip route add/del` syscall

- IPv4 fragmentation: reassembly with 60-second timeout; send with DF bit set by default

- Link-local autoconf: IPv6 SLAAC (RA → prefix + EUI-64 derived address)

---

## TCP State Machine

States: `CLOSED → LISTEN → SYN_RECEIVED → ESTABLISHED → FIN_WAIT_1 → FIN_WAIT_2 → TIME_WAIT → CLOSE_WAIT → LAST_ACK → CLOSED`

- Sliding window with SACK, timestamp option (RFC 7323)

- Cubic congestion control (default); pluggable CC interface

- Nagle algorithm (disabled by default for interactive use; enabled with `TCP_CORK`)

- Fast retransmit (3 duplicate ACKs) + fast recovery

- TIME_WAIT recycle: 4× MSL = 4× 30s = 2 minutes

---

## UDP

- Stateless; checksum required for IPv6, optional for IPv4

- Socket receive buffer: ring with configurable depth (default 256 × 2 KB segments)

- `SO_REUSEPORT`: multiple sockets per port for load-balanced UDP (DNS server use)

---

## ICMP

- ICMPv4: echo request/reply, destination unreachable (type 3), time exceeded (type 11)

- ICMPv6: echo, neighbour solicitation/advertisement, router advertisement

- Rate limiting: max 100 ICMP error packets/second per source to prevent amplification

---

## ARP / NDP

- ARP cache: hash table, 60s TTL, gratuitous ARP on address assignment

- NDP (IPv6): neighbour cache, router cache, duplicate address detection (DAD)

- Proxy ARP: optional, disabled by default

---

## DHCP Client State Machine

States: `INIT → SELECTING → REQUESTING → BOUND → RENEWING → REBINDING → INIT`

- DHCPv4 (RFC 2131): DISCOVER → OFFER → REQUEST → ACK

- DHCPv6 (RFC 8415): SOLICIT → ADVERTISE → REQUEST → REPLY

- Lease stored in `/var/run/sigma-dhcp.lease`

- Renew at T1 (50% lease time), rebind at T2 (87.5%)

---

## DNS / DoH Resolver

- UDP DNS: port 53, 512-byte message limit, EDNS0 extension for larger responses

- TCP DNS: fallback for truncated responses

- DoH (DNS over HTTPS): `POST https://resolver/dns-query` with `application/dns-message`

- DNSSEC: validate RRSIG records against trust anchor (root KSK)

- LRU cache: 4 096 entries, respects TTL, negative caching (NXDOMAIN)

- Resolver config: `/etc/sigma-resolv.conf`

---

## TLS 1.3 + Kyber-1024

- Implemented in `crypto/tls13.c` using sigma-crypto primitives

- Key exchange: X25519 (classical) + Kyber-1024 (PQC) hybrid KEM; both required

- Signature: ECDSA P-256 or Dilithium-5 for post-quantum certs

- Session tickets: HKDF-derived resumption keys, 24-hour lifetime

- OCSP stapling: certificate revocation status in TLS handshake

---

## Firewall Hooks

Three hook points (inspired by netfilter): `PREROUTING`, `FORWARD`, `POSTROUTING`

Each hook processes a chain of rules:
```
rule: {src_ip, dst_ip, src_port, dst_port, proto, iface} → {ACCEPT, DROP, REJECT, DNAT, SNAT}
```

Stateful conntrack table: tracks TCP state, UDP "connection" by 5-tuple, ICMP by id+seq. NAT rewrites packets and updates conntrack.

---

## VirtIO-net Driver Interface

- Virtqueue pairs: one TX + one RX + one ctrl queue

- Receive path: DMA descriptor ring; kernel maps guest-physical pages into ring

- Transmit path: scatter-gather list; free TX descriptor on used-ring interrupt

- Features negotiated: `VIRTIO_NET_F_CSUM`, `VIRTIO_NET_F_GUEST_CSUM`, `VIRTIO_NET_F_MRG_RXBUF`

---

## Implementation Plan

- [ ] 1. Packet buffer (pbuf) allocator with zero-copy support

- [ ] 2. Ethernet frame parse/build + loopback driver

- [ ] 3. ARP / NDP tables

- [ ] 4. IPv4 parse/build + routing table

- [ ] 5. IPv6 parse/build + SLAAC

- [ ] 6. ICMPv4 + ICMPv6

- [ ] 7. UDP socket layer

- [ ] 8. TCP state machine + sliding window + Cubic CC

- [ ] 9. DHCP client (v4 + v6)

- [ ] 10. DNS/DoH resolver + DNSSEC

- [ ] 11. TLS 1.3 + Kyber-1024 hybrid

- [ ] 12. Firewall hook framework + conntrack

- [ ] 13. VirtIO-net driver

- [ ] 14. POSIX socket API shim

- [ ] 15. Tests: TCP state machine, DHCP, DNS, TLS handshake, firewall rules

---

## Status

| Feature | State |
|---------|-------|
| IPv4/IPv6 | ⬜ Not started |
| TCP | ⬜ Not started |
| UDP | ⬜ Not started |
| DHCP client | ⬜ Not started |
| DNS/DoH | ⬜ Not started |
| TLS 1.3 + Kyber | ⬜ Not started |
| Firewall | ⬜ Not started |
| VirtIO-net | ⬜ Not started |
