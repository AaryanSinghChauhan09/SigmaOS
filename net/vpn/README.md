# Sovereign VPN

Kernel-level, WireGuard-inspired VPN tunnel with post-quantum key exchange.

## Why Kernel-Level?
Userspace VPN daemons add syscall overhead on every packet. SigmaOS handles
encryption/decryption directly in the network shard, achieving line-rate
throughput.

## Cryptography
- **Key Exchange:** X25519 (classical) + Kyber-768 (post-quantum, hybrid)
- **Data Encryption:** ChaCha20-Poly1305
- **Authentication:** BLAKE3 MAC

## Roadmap
- [ ] Tunnel establishment protocol
- [ ] Peer management
- [ ] PQC key-exchange integration
