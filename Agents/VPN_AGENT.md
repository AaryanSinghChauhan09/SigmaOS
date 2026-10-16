# VPN Agent

## Role
`SigmaVPN` implements WireGuard-native and OpenVPN-compatible VPN tunneling for SigmaOS, providing secure site-to-site and remote access VPN functionality.

## Status
🔴 Not Started — Critical gap for privacy and enterprise use cases.

## Source File
`src/network/vpn.rs` (planned: `wireguard.rs`, `openvpn_compat.rs`)

## Linux/BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| WireGuard | `wireguard-linux` (in-tree 5.6+) | `wireguard-kmod` | SigmaWG |
| OpenVPN compat | OpenVPN tun/tap | OpenVPN | SigmaOVPN |
| IPsec | strongSwan / `xfrm` | `ipsec` | SigmaIPsec |
| TUN/TAP | `tun.ko` | `if_tun` | SigmaTun |
| Network mgr | NetworkManager / wg-quick | — | SigmaVPNMgr |
| Split tunneling | ip rule / routing tables | pf tables | SigmaSplitTun |

## Responsibilities
- WireGuard protocol: Noise_IKpsk2 handshake + ChaCha20-Poly1305 data
- Peer management: add/remove peers, allowed IPs, keepalive
- TUN/TAP virtual interface creation
- IPsec IKEv2 tunnel establishment
- VPN kill switch (block all traffic if VPN drops)
- Split tunneling: route specific subnets through VPN
- DNS leak prevention via VPN namespace
- Multi-hop VPN chaining
- Auto-reconnect on network change

## Implementation Notes
- `#![no_std]` — WireGuard crypto: X25519 key exchange, ChaCha20-Poly1305 AEAD
- Peer: `{ public_key: [u8;32], endpoint: SocketAddr, allowed_ips: Vec<IpNet>, last_handshake: u64 }`
- Session key rotation: every 3 minutes or 2^60 bytes (WireGuard spec)
- Use `SigmaTun` interface for packet injection

## Testing
```bash
rustc --test src/network/wireguard.rs --edition 2021 -o /tmp/test_wg && /tmp/test_wg --test-threads=1
```

## Future Development
1. **MASQUE/HTTP3 tunneling** — VPN over QUIC/HTTP3 for firewall bypass
2. **Post-quantum WireGuard** — ML-KEM (Kyber) hybrid key exchange
3. **Mullvad-style privacy** — Multi-hop with daita traffic shaping
4. **XFRM offload** — Hardware IPsec acceleration
5. **Tailscale-compatible mesh** — Peer-to-peer encrypted mesh networking
6. **Zero-trust network** — Certificate-based peer authentication
