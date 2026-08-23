# SigmaOS Networking

## TCP/IP Stack

Pure-Rust TCP implementation with RFC-793/7323 compliance.

### TCP State Machine

```
CLOSED → LISTEN → SYN_RECEIVED → ESTABLISHED → FIN_WAIT_1 → FIN_WAIT_2 → TIME_WAIT → CLOSED
```

### TCP Features

| Feature | Status | RFC |
|---------|--------|-----|
| SACK | ✅ | RFC 2018 |
| Window scaling | ✅ | RFC 7323 |
| Timestamps | ✅ | RFC 7323 |
| CUBIC congestion | ✅ | RFC 8312 |
| BBRv3 | 🔬 | Google |
| TCP Fast Open | ✅ | RFC 7413 |
| ECN | ✅ | RFC 3168 |
| MPTCP | 📋 | RFC 8684 |

## eBPF Firewall

### XDP (eXpress Data Path)

Runs at NIC driver level before kernel network stack — up to 24 Mpps/core.

```c
SEC("xdp")
int sigma_firewall(struct xdp_md *ctx) {
    struct iphdr *ip = ...;
    struct tcphdr *tcp = ...;
    if (tcp->dest == htons(22) || tcp->dest == htons(443))
        return XDP_PASS;
    return XDP_DROP;
}
```

## WireGuard VPN

Kernel-level WireGuard integration:
- **Key exchange**: Curve25519
- **AEAD cipher**: ChaCha20-Poly1305
- **Hash**: BLAKE2s
- **Handshake**: 1-RTT

## Zero-Trust Networking

- Never trust, always verify
- mTLS for service-to-service
- Identity-based (not IP-based)
- Micro-segmentation

## DNS

Local caching resolver with DNS-over-HTTPS (DoH), DNS-over-TLS (DoT), DNSSEC validation.

## Network Interfaces

| Type | Description |
|------|-------------|
| eth0/ens* | Physical Ethernet |
| wlan0 | WiFi 802.11ax |
| lo | Loopback |
| wg0 | WireGuard VPN |
| veth* | Virtual Ethernet (containers) |
