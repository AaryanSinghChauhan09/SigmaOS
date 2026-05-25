# Cryptographic Identity Manager

Zero-trust process identity: every shard receives a cryptographically signed
identity token at spawn time. No token = no resource access.

## Token Structure
```json
{
  "shard_id": "uuid-v4",
  "capabilities": ["NET_BIND", "FS_READ:/media"],
  "issued_at": 1748200000,
  "signature": "<ED25519 sig over above fields>"
}
```

## Lifecycle
1. Kernel spawns shard → generates ephemeral key pair
2. Trust Root signs the token
3. Every IPC / syscall presents token → verified in O(1) via cached pubkey

## Roadmap
- [ ] ED25519 token issuance
- [ ] Token revocation list (CRL equivalent)
- [ ] PQC upgrade path (Dilithium)
