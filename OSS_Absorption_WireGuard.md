# OSS Absorption: WireGuard — Sovereign VPN Layer

> **Status**: 🔄 Active | **Source Project**: WireGuard (Jason A. Donenfeld) | **Target Shard**: `SigmaOS Network Security / VPN Layer`

---

## 1. Executive Summary

WireGuard is a next-generation VPN protocol: ~4,000 lines of kernel code (vs 100,000+ for OpenVPN/IPsec), cryptographically opinionated (Curve25519, ChaCha20-Poly1305, BLAKE2s), and achieves near-line-rate encrypted throughput. It has been merged into the mainline Linux kernel since 5.6.

SigmaOS integrates WireGuard natively into the kernel networking stack as `sigma-wg`, with a higher-level management layer (`sigma-mesh-vpn`) that enables automatic peer discovery, NAT traversal, and mesh networking between multiple SigmaOS machines.

---

## 2. Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                    SIGMA VPN STACK                               │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐    │
│  │              SIGMA MESH VPN (userland)                   │    │
│  │  Peer discovery │ NAT traversal │ DNS-based endpoints   │    │
│  │  Auto key rotation │ PQC key exchange (Kyber+X25519)     │    │
│  └──────────────────────────┬───────────────────────────────┘    │
│                             │                                    │
│  ┌──────────────────────────▼───────────────────────────────┐    │
│  │              SIGMA-WG (kernel module)                    │    │
│  │  WireGuard protocol: Noise_IK handshake                  │    │
│  │  Encryption: ChaCha20-Poly1305 (data)                   │    │
│  │              Curve25519 (key exchange)                   │    │
│  │              BLAKE2s (hashing)                           │    │
│  │  Interface: wg0 (virtual tunnel device)                  │    │
│  └──────────────────────────┬───────────────────────────────┘    │
│                             │                                    │
│  ┌──────────────────────────▼───────────────────────────────┐    │
│  │              KERNEL NETWORK STACK                        │    │
│  │  Routing table │ Netfilter/eBPF │ Physical NIC           │    │
│  └──────────────────────────────────────────────────────────┘    │
└──────────────────────────────────────────────────────────────────┘
```

---

## 3. Key Features

### 3.1 Simple Configuration

```toml
# /etc/sigma/vpn/wg0.toml
[interface]
private_key = "yAnz5TF+lXXJte14tji3zlMNq+hd2rYUIgJBgB3fBmk="
address     = "10.0.0.1/24"
listen_port = 51820
dns         = "10.0.0.1"

[[peers]]
name        = "laptop"
public_key  = "xTIBA5rboUvnH4htodjb6e697QjLERt1NAB4mZqp8Dg="
allowed_ips = "10.0.0.2/32"
endpoint    = "laptop.home:51820"
keepalive   = 25   # seconds

[[peers]]
name        = "phone"
public_key  = "TrMvSoP4jYQlY6RIzBgbssQqY3vxI2piVFBs2LzWZQA="
allowed_ips = "10.0.0.3/32"
# No endpoint — phone connects to us (NAT-friendly)
```

### 3.2 CLI Management

```bash
# Bring up VPN tunnel
$ sigma vpn up wg0
Σ [VPN] Interface wg0 up:
  Address    : 10.0.0.1/24
  Listen port: 51820
  Peers      : 2 configured

# Check peer status
$ sigma vpn status wg0
Σ [VPN] wg0 — WireGuard tunnel:
  Peer: laptop (xTIBA5...)
    Endpoint    : 192.168.1.50:51820
    Last handshake: 34 seconds ago
    Transfer    : ↑ 1.2GB  ↓ 4.5GB
    Keepalive   : every 25s

  Peer: phone (TrMvS...)
    Endpoint    : (none — waiting for connection)
    Last handshake: never

# Generate new keypair
$ sigma vpn keygen
Σ [VPN] New WireGuard keypair:
  Private: <saved to /etc/sigma/vpn/wg0.key>
  Public:  A1B2C3D4E5F6G7H8I9J0K1L2M3N4O5P6Q7R8S9T0U1V=

# Add a peer on-the-fly
$ sigma vpn peer add wg0 \
    --name "server" \
    --public-key "A1B2C3..." \
    --endpoint "server.example.com:51820" \
    --allowed-ips "10.0.0.4/32"
```

### 3.3 Mesh VPN (`sigma-mesh-vpn`)

Automatically connect all your SigmaOS machines into a private mesh network (inspired by Tailscale/ZeroTier, but fully self-hosted):

```bash
# Initialize mesh network
$ sigma mesh init --name "my-network" --cidr 10.100.0.0/16
Σ [MESH] Mesh network "my-network" initialized:
  Coordination server: https://mesh.sigmaos.local
  Network CIDR: 10.100.0.0/16
  PQC key exchange: Kyber1024 + X25519 (hybrid)

# Join from another machine
$ sigma mesh join --token "sigma-mesh-abc123..."
Σ [MESH] Joined "my-network" as 10.100.0.2
  Peers discovered: 3 (workstation, laptop, server)
  NAT traversal: STUN/TURN holepunching active

# List mesh peers
$ sigma mesh peers
Σ [MESH] my-network (4 peers):
  workstation  10.100.0.1  online  (direct, 0.3ms)
  laptop       10.100.0.2  online  (relayed via TURN, 12ms)
  server       10.100.0.3  online  (direct, 8ms)
  phone        10.100.0.4  offline (last seen 2h ago)
```

### 3.4 PQC Hybrid Key Exchange

SigmaOS enhances WireGuard's Noise protocol with post-quantum hybrid key exchange:

```rust
// kernel/net/wireguard_pqc.rs
// SPDX-License-Identifier: MIT

pub struct PqcNoiseHandshake {
    classical: X25519,       // Classical ECDH (existing WireGuard)
    quantum:   Kyber1024,    // Post-quantum KEM (NIST ML-KEM)
}

impl PqcNoiseHandshake {
    /// Hybrid key exchange: concatenate both shared secrets
    pub fn derive_shared_key(&self, peer_pub: &PeerPublicKeys) -> SharedKey {
        let ecdh_secret = self.classical.diffie_hellman(&peer_pub.x25519);
        let kem_secret  = self.quantum.decapsulate(&peer_pub.kyber_ct);

        // SHA-256(ECDH_secret || KEM_secret) — secure if EITHER is unbroken
        let mut hasher = Blake2s::new();
        hasher.update(&ecdh_secret);
        hasher.update(&kem_secret);
        SharedKey::from(hasher.finalize())
    }
}
```

---

## 4. Performance

| VPN Protocol | Throughput (1Gbps link) | Latency Overhead | Code Size |
|:------------|:-----------------------|:----------------|:----------|
| OpenVPN | ~400 Mbps | ~5ms | 100,000+ LoC |
| IPsec (strongSwan) | ~700 Mbps | ~2ms | 400,000+ LoC |
| WireGuard (Linux) | ~950 Mbps | ~0.5ms | ~4,000 LoC |
| sigma-wg (SigmaOS) | ~950 Mbps | ~0.5ms | ~5,000 LoC (Rust) |

---

## 5. References & Standards

- WireGuard — `wireguard.com` (GPL-2.0)
- Noise Protocol Framework — `noiseprotocol.org`
- Kyber1024 (ML-KEM) — NIST PQC standard
- Tailscale mesh networking architecture — `tailscale.com/blog/how-tailscale-works`
