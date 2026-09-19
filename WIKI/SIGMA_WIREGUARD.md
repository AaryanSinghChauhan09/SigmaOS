# SigmaOS WireGuard — Sovereign VPN Kernel Module

## Overview

SigmaOS implements the WireGuard VPN protocol natively in the kernel. No external cryptographic libraries — Curve25519, ChaCha20-Poly1305, and BLAKE2s are implemented as sovereign algorithms.

**Location:** `src/net/sigma_wireguard.rs`

---

## Protocol

WireGuard uses Noise_IKpsk2 handshake framework:
- **Curve25519** — Elliptic-curve Diffie-Hellman key exchange
- **ChaCha20-Poly1305** — Authenticated encryption
- **BLAKE2s** — Hashing and MAC
- **1.5 RTT** handshake with perfect forward secrecy

---

## API Reference

```rust
// Create a WireGuard interface
let private_key = PrivateKey([/* 32 bytes */]);
let mut dev = WgDevice::new("wg0", private_key, 51820);
dev.interface_ip = Some([10, 0, 0, 1]);

// Add a peer
let mut peer = WgPeer::new(remote_public_key);
peer.add_allowed_ip(AllowedIp::ipv4(10, 0, 0, 0, 24));
peer.set_endpoint(WgEndpoint::new(203, 0, 113, 1, 51820));
peer.persistent_keepalive = 25;
dev.add_peer(peer);

// Perform handshake
dev.initiate_handshake(&remote_key, now_ns).unwrap();
dev.complete_handshake(&remote_key, now_ns).unwrap();

// Encrypt and send a packet
let encrypted = dev.encapsulate([10, 0, 0, 2], &payload).unwrap();
```

---

## wg-quick Config Parsing

```rust
let config = r#"
[Interface]
ListenPort = 51820
Address = 10.0.0.1/24

[Peer]
PublicKey = <base64-encoded-key>
Endpoint = 203.0.113.1:51820
AllowedIPs = 0.0.0.0/0
PersistentKeepalive = 25
"#;
let cfg = WgConfig::parse(config);
```

---

## Security Properties

| Property | Description |
|----------|-------------|
| PFS | Ephemeral keys per session |
| Anti-replay | 64-packet sliding window |
| PSK | Optional post-quantum hardening |
| Session timeout | 180 seconds |
| Handshake timeout | 5 seconds |

---

## Comparison

| Feature | OpenVPN | IPSec | WireGuard (Linux) | SigmaOS WireGuard |
|---------|---------|-------|------------------|-------------------|
| Protocol | TLS | IKEv2 | Noise_IKpsk2 | Noise_IKpsk2 |
| Code size | ~400k | ~400k | ~4k | ~600 lines |
| no_std | No | No | No | **Yes** |
| Config parsing | Yes | Yes | wg-quick | **Yes** |
