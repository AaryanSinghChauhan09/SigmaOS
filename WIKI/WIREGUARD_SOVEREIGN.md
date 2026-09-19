# SigmaOS Sovereign WireGuard VPN Protocol

## Overview

SigmaOS implements a **pure-Rust, zero-dependency sovereign WireGuard tunnel interface** (`src/net/wireguard_sovereign.rs` and `src/network/wireguard_sovereign.rs`), absorbing the modern VPN protocol created by Jason A. Donenfeld and integrated into Linux 5.6.

WireGuard replaces legacy IPsec and OpenVPN architectures with a minimal, mathematically verifiable cryptographic protocol operating directly on UDP.

## Core Mechanisms

1. **Cryptokey Routing**:
   - Associates public keys directly with allowed IP addresses (`allowed_ips`).
   - Outbound routing table checks destination IP $\to$ selects matching peer $\to$ encrypts with peer's public key.
   - Inbound routing table decrypts packet $\to$ verifies source IP matches peer's `allowed_ips` list (anti-spoofing enforcement).
2. **Noise IK Handshake**:
   - 1-RTT mutual authentication and ephemeral session key agreement.
   - Transitions through `Unauthenticated` $\to$ `HandshakeInitiated` $\to$ `Established` states.
3. **Transport Packet Encapsulation**:
   - Type 4 transport packet framing with sender/receiver indices and 64-bit monotonically increasing nonce counters.
   - Authenticated encryption (AEAD) with 16-byte Poly1305 authentication tags.

## Test Verification

6 standalone unit tests verified in test runner suite `[20]`:
- `test_wg_tunnel_creation`: Interface initialization and key generation.
- `test_wg_cryptokey_routing_add_peer`: Peer and allowed IP registration.
- `test_wg_handshake_lifecycle`: Complete 1-RTT handshake negotiation.
- `test_wg_encapsulate_unauthenticated_fails`: Rejection of unauthenticated packets.
- `test_wg_encapsulate_established_success`: Transport packet encapsulation and traffic accounting.
- `test_wg_routing_miss_fails`: Cryptokey routing table lookup misses rejected.
