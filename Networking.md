# SigmaOS Networking Stack

## Architecture

SigmaOS uses a modern, eBPF-first networking architecture:

    Application Layer
        │
        ▼
    Socket API (AF_INET, AF_INET6, AF_UNIX, io_uring)
        │
        ▼
    NetworkBolt Daemon (sigma-net)
        │
        ├── WireGuard VPN
        ├── DNS Resolver (DoH/DoT)
        ├── Network Manager
        └── Zero-Trust Agent
        │
        ▼
    Kernel Network Stack
        │
        ├── eBPF TC (Traffic Control)
        ├── eBPF XDP (Express Data Path)
        └── netfilter (legacy compat)
        │
        ▼
    Network Hardware Drivers
        │
        └── (WiFi/Ethernet/Bluetooth)

## NetworkBolt (`sigma-net`)

The primary network management daemon:

```bash
# Configure network interface
sigma-net connect eth0 dhcp
sigma-net connect wlan0 ssid "MyWiFi" --password

# Static IP
sigma-net set eth0 ip 192.168.1.100/24 gateway 192.168.1.1

# VPN
sigma-net vpn connect wg0 /etc/wireguard/wg0.conf
sigma-net vpn status

# DNS
sigma-net dns set 1.1.1.1 8.8.8.8 --doh
sigma-net dns status

# Firewall
sigma-net fw allow in port 22 proto tcp
sigma-net fw deny out dst 10.0.0.0/8
sigma-net fw status
```

## eBPF Firewall

SigmaOS replaces iptables with a pure eBPF XDP/TC firewall:

| Feature | iptables | SigmaOS eBPF |
|---------|----------|-------------|
| Packet rate | ~11 Mpps | ~14.8 Mpps |
| Rule evaluation | Linear | Hash table O(1) |
| Dynamic updates | Requires flush | Hot-reload |
| DDoS mitigation | Limited | XDP drop at NIC |
| Observability | Poor | Full eBPF metrics |

## WireGuard Integration

WireGuard is built into the kernel as a native module:

```bash
# Generate keypair
sigma-net wg genkey > private.key
sigma-net wg pubkey < private.key > public.key

# Configure peer
sigma-net wg add-peer wg0 \
  --pubkey <peer-pubkey> \
  --endpoint peer.example.com:51820 \
  --allowed-ips 10.0.0.0/8
```

## Zero-Trust Network Access

SigmaOS implements Zero-Trust principles:

*   Every connection authenticated and authorized
*   Mutual TLS for all internal services
*   Policy-based access control via Sentinel
*   Continuous verification (not just at login)

## DNS Resolution

Multi-layer DNS with privacy protection:

1.  Local cache (dnsmasq)
2.  DNS-over-HTTPS (Cloudflare/Quad9/custom)
3.  DNS-over-TLS fallback
4.  DNSSEC validation
5.  Split-horizon DNS for VPN
