# SigmaOS Post-Quantum Security

SigmaOS is the first OS designed with post-quantum cryptography baked in at every layer — not bolted on after the fact.

---

## Why Post-Quantum Now?

**Harvest-now-decrypt-later**: adversaries are archiving encrypted traffic today to decrypt retroactively once quantum computers scale. Data encrypted now with classical crypto (RSA, ECDSA, X25519) will be vulnerable within 10–15 years.

NIST finalised the first PQC standards in 2024:

- **FIPS 203** — ML-KEM (Kyber-1024) for key encapsulation

- **FIPS 204** — ML-DSA (Dilithium-5) for digital signatures

SigmaOS implements both, everywhere classical crypto would appear.

---

## Algorithms

### Kyber-1024 (ML-KEM) — Key Encapsulation

Used for: TLS key exchange, package decryption, CryptFS key wrapping, sigma-bus channel setup.

```
Security level: AES-256 equivalent (Category 5)
Public key:     1568 bytes
Ciphertext:     1568 bytes
Shared secret:  32 bytes
```

**Hybrid mode** (TLS): X25519 + Kyber-1024 in parallel. The session key is the KDF of both secrets — if either algorithm is broken, the session is still protected by the other.

### Dilithium-5 (ML-DSA) — Digital Signatures

Used for: package signing, sigma-boot.efi signing, audit trail chaining, commit signing.

```
Security level: AES-256 equivalent (Category 5)
Public key:     2592 bytes
Private key:    4864 bytes
Signature:      4595 bytes
```

### Supporting Primitives

| Primitive | Use |
|-----------|-----|
| AES-256-GCM | Symmetric encryption (CryptFS, sigma-vault) |
| BLAKE3 | Package content hashes |
| BLAKE2b-256 | Audit trail integrity chains |
| Argon2id | Key derivation (CryptFS passphrase → key) |
| SHA-3-256 | Fallback hash for PQC pre-images |

---

## Where PQC is Used

### TLS 1.3 (`net/tls/`)

```
ClientHello:  X25519 keyshare + Kyber-1024 keyshare
ServerHello:  Kyber-1024 ciphertext + X25519 response
Session key:  HKDF(kyber_secret || x25519_secret)
```

### Package Signing (`userland/pkg/`)

```bash

# Sign a package

sigma-pkg sign myapp.spkg --key developer.dilithium5.key

# Verify on install

sigma-pkg install myapp.spkg   # auto-verifies Dilithium-5 signature

```

### Audit Trail (`kernel/security/sigma_immutable_audit_trail.cpp`)

Every audit record is BLAKE2b-chained to the previous one:
```
record[n].hash = BLAKE2b(record[n].data || record[n-1].hash)
```
Tampering with any record invalidates the entire chain.

### sigma-boot.efi (Phase G)

The bootloader will be signed with Dilithium-5 and verified against a TPM2-sealed trust anchor before the kernel loads.

---

## SIMD Acceleration

### AVX-512 (x86_64) — `crypto/sigma_kyber_avx512.cpp`

- Kyber NTT (Number Theoretic Transform) vectorised with AVX-512

- Target: ≥ 5.8 M Kyber-1024 ops/sec on modern Intel Xeon

### ARM NEON — `crypto/sigma_kyber_neon.cpp`

- Kyber NTT on ARM Cortex-A72+ (Raspberry Pi 4/5)

- Target: ≥ 1.2 M Kyber-1024 ops/sec

---

## Source Files

| File | Purpose |
|------|---------|
| `crypto/SovereignKyber.cpp` | Kyber-1024 KEM |
| `crypto/SovereignDilithium5.cpp` | Dilithium-5 signatures |
| `crypto/sigma_kyber_avx512.cpp` | AVX-512 accelerated Kyber |
| `crypto/sigma_kyber_neon.cpp` | ARM NEON accelerated Kyber |
| `include/sigma_pqc.h` | PQC type definitions |
| `include/sigma_pqc_keygen.h` | Key generation API |
| `include/sigma_pqc_sign.h` | Signing API |
| `include/sigma_pqc_verify.h` | Verification API |
| `security/SovereignEntropy.cpp` | CSPRNG for key generation |

---

## Known Issues

| ID | Issue | Severity |
|----|-------|---------|
| #1009 | CryptFS `derive_key()` returns 32 zero bytes | Critical — Phase G fix |
| — | Side-channel resistance not yet audited | Medium — Phase H audit |

---

## Benchmarks (Target — Phase G)

| Operation | Algorithm | Target |
|-----------|-----------|--------|
| Key generation | Kyber-1024 | < 0.1 ms |
| Encapsulation | Kyber-1024 | < 0.2 ms |
| Decapsulation | Kyber-1024 | < 0.2 ms |
| Sign | Dilithium-5 | < 1 ms |
| Verify | Dilithium-5 | < 0.5 ms |
| TLS handshake | Hybrid | < 5 ms |

---

*See also: [Security-Model](Security-Model) · [PQC_HARDENING](PQC_HARDENING) · [Verified-Boot](Verified-Boot)*
