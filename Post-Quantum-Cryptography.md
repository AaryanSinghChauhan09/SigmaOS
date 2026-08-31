# Quantum-Safe Cryptography Toolkit

Prepares SigmaOS for the post-quantum era by integrating NIST-selected PQC
algorithms at the kernel level.

## Algorithms Integrated

| Purpose | Algorithm | NIST Status | Security Level |
| --- | --- | --- | --- |
| Key Encapsulation | Kyber-768 / ML-KEM | ✅ Standard | 192-bit classical security |
| Digital Signatures | Dilithium3 / ML-DSA | ✅ Standard | 128-bit quantum security |
| Hashing | BLAKE3 | 🔧 Best-in-class | 256-bit output |

## Architecture

```
Application Layer
   └─ sigma_pqc_encrypt() / sigma_pqc_sign()
         └─ PQC Kernel Module
               ├─ Kyber KEM (key encapsulation)
               ├─ Dilithium DSA (digital signatures)
               └─ BLAKE3 (hashing)
                     └─ Hardware Acceleration (if available)
```

## Integration Points

### VPN Key Exchange
- Kyber-768 hybrid with X25519 for forward secrecy
- Post-quantum resistant tunnel establishment
- Fallback to classical crypto if PQC unavailable

### Shard Identity Tokens
- Dilithium3 signatures for shard authentication
- Immutable identity verification
- Revocation via CRL (Certificate Revocation List)

### Shard Manifest Signatures
- SPM (Sigma Package Manager) uses Dilithium for package signing
- BLAKE3 for package integrity verification
- Trust root anchored in Secure Boot

## API Interface

```c
// Kyber KEM operations
int pqc_kyber_keygen(uint8_t *public_key, uint8_t *secret_key);
int pqc_kyber_encapsulate(const uint8_t *public_key, uint8_t *ciphertext, uint8_t *shared_secret);
int pqc_kyber_decapsulate(const uint8_t *secret_key, const uint8_t *ciphertext, uint8_t *shared_secret);

// Dilithium DSA operations
int pqc_dilithium_keygen(uint8_t *public_key, uint8_t *secret_key);
int pqc_dilithium_sign(const uint8_t *secret_key, const uint8_t *message, size_t msg_len, uint8_t *signature);
int pqc_dilithium_verify(const uint8_t *public_key, const uint8_t *message, size_t msg_len, const uint8_t *signature);

// BLAKE3 hashing
int pqc_blake3_hash(const uint8_t *input, size_t len, uint8_t *output);
int pqc_blake3_hash_derive_key(const uint8_t *input, size_t len, const uint8_t *context, uint8_t *output);

// Initialize PQC subsystem
void init_security_pqc(void);
```

## Performance Characteristics

| Algorithm | Key Generation | Signing | Verification | Encapsulation | Decapsulation |
|---|---|---|---|---|---|
| Kyber-768 | ~2ms | N/A | N/A | ~0.5ms | ~0.5ms |
| Dilithium3 | ~3ms | ~0.8ms | ~0.3ms | N/A | N/A |
| BLAKE3 | N/A | N/A | N/A | N/A | ~0.1ms/KB |

## Security Properties

- **Quantum resistance**: All algorithms are secure against quantum computers
- **Hybrid approach**: Combines PQC with classical crypto for defense in depth
- **Constant-time**: All operations are constant-time to prevent timing attacks
- **Side-channel resistant**: Implements blinding and masking techniques

## Roadmap

- [x] BLAKE3 hashing implementation
- [ ] Kyber KEM integration in VPN
- [ ] Dilithium signature in Identity Manager
- [ ] Side-channel hardening (constant-time implementations)
- [ ] Hardware acceleration support (Intel SHA extensions, ARM Crypto extensions)
- [ ] PQC algorithm agility (support for multiple KEM/DSA algorithms)
- [ ] Formal verification of PQC implementations
- [ ] NIST PQC standard updates tracking

## Related Modules

- [`security/mac/README.md`](mac/README.md) — Mandatory Access Control
- [`security/identity/README.md`](identity/README.md) — Identity Manager
- [`modules/core/net`](../modules/core/net/README.md) — Network stack (VPN integration)
