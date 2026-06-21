# Post-Quantum Cryptography — ML-KEM-1024 + ML-DSA-87

SigmaOS is future-proofed against cryptographically-relevant quantum computers (CRQCs) via its **SovereignPQC** shard, which implements both NIST-standardized post-quantum schemes at their highest security parameter sets.

---

## Standards Compliance

| Standard | Scheme | Parameter Set | Security Level |
|---|---|---|---|
| NIST FIPS 203 | **ML-KEM (Kyber)** | ML-KEM-1024 | Level 5 (AES-256 equivalent) |
| NIST FIPS 204 | **ML-DSA (Dilithium)** | ML-DSA-87 | Level 5 (AES-256 equivalent) |

---

## ML-KEM-1024 — Key Encapsulation Mechanism

**File:** [`crypto/SovereignKyber.cpp`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/crypto/SovereignKyber.cpp)

Kyber is a lattice-based Key Encapsulation Mechanism used to establish shared secrets over untrusted channels — a quantum-secure replacement for RSA/ECDH key exchange.

### Key Sizes (ML-KEM-1024)

| Type | Size |
|---|---|
| Public Key | **1,568 bytes** |
| Secret Key | **3,168 bytes** |
| Ciphertext | **1,568 bytes** |
| Shared Secret | **32 bytes** |

### C-API

```c
// Key generation
int kyber_keygen(kyber_public_key_t* pk, kyber_secret_key_t* sk);

// Encapsulate (sender): generates ciphertext + shared secret
int kyber_encapsulate(const kyber_public_key_t* pk,
                      kyber_ciphertext_t* ct,
                      kyber_shared_secret_t* ss);

// Decapsulate (receiver): recovers shared secret from ciphertext
int kyber_decapsulate(const kyber_secret_key_t* sk,
                      const kyber_ciphertext_t* ct,
                      kyber_shared_secret_t* ss);
```

### Internal Design

1. **KeyGen**: Generates A-matrix via SHAKE-128 XOF expansion; secret vector s, error vector e; computes public key `t = A·s + e`
2. **Encapsulate**: Samples random message coin `m`, encodes via `ct = (u, v) = Compress(A·r + e1, B·r + e2 + ⌈q/2⌉·m)`, derives shared secret `ss = KDF(G(m, H(pk)))`
3. **Decapsulate**: Recovers `m' = Decompress(v - s·u)`, re-encapsulates to verify implicit rejection, returns `ss = KDF(G(m', H(pk)))` or `H(z || ct)` on failure (implicit rejection)

---

## ML-DSA-87 — Digital Signatures

**File:** [`crypto/SovereignDilithium5.cpp`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/crypto/SovereignDilithium5.cpp)

Dilithium is used for all kernel attestation, package signing, audit trail signatures, and secure boot chain verification.

### Key & Signature Sizes (ML-DSA-87)

| Type | Size |
|---|---|
| Public Key | **2,592 bytes** |
| Secret Key | **4,864 bytes** |
| Signature | **4,595 bytes** |

### C-API

```c
// Initialize Dilithium engine
void pqc_init(void);

// Key generation
int pqc_generate_keypair(pqc_public_key_t* pk, pqc_secret_key_t* sk);

// Sign message (deterministic HEDGED variant)
int pqc_sign(const pqc_secret_key_t* sk,
             const sigma_u8* msg, sigma_usize len,
             pqc_signature_t* sig);

// Verify signature
int pqc_verify(const pqc_public_key_t* pk,
               const sigma_u8* msg, sigma_usize len,
               const pqc_signature_t* sig);
```

### Internal Design

1. **KeyGen**: Expands seed `ξ` → `(ρ, ρ', K, z)`; generates A-matrix in NTT domain; samples secret vectors `(s1, s2) = ExpandS(ρ')`; computes `t = A·s1 + s2`
2. **Sign**: Commitment `c̃ = H(μ, w1)` where `w1 = HighBits(A·y)`; response `z = y + c·s1`; hint `h = MakeHint(-c·t0, w - c·s2 + c·t0)`; output `σ = (c̃, z, h)`
3. **Verify**: Parses `(c̃, z, h)` from `σ`; recomputes `w' = A·z - c·t1·2^d`; checks `c̃ = H(μ, UseHint(h, w'))` and norm bound `||z||∞ < γ1 − β`
4. **Dilithium prime**: All polynomial arithmetic mod `q = 8,380,417`

---

## Immutable Audit Trail Integration

**File:** [`kernel/security/sigma_immutable_audit_trail.cpp`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/kernel/security/sigma_immutable_audit_trail.cpp)

Every kernel audit event is:
1. Hashed with SHA-256-sim to produce a **payload hash**
2. Chained: `chain_hash = H(payload_hash || prev_hash)` (Merkle-style)
3. **Dilithium-5 signed** by the ledger's private key
4. Appended to the 4,096-record in-memory ledger

The entire chain can be verified at any time with `sigma_audit_verify_chain()`.

```c
// Log events
sigma_audit_log_syscall(uid, resource_id, "open /etc/passwd");
sigma_audit_log_file(uid, inode_id, "/etc/shadow");
sigma_audit_log_security(uid, res_id, "AUTH_FAIL");
sigma_audit_log_crypto(uid, key_id, "DILITHIUM_KEYGEN");

// Verify entire chain integrity (replays all hashes + signatures)
sigma_status result = sigma_audit_verify_chain(); // K_OK or K_ERR_INVAL
```

---

## Secure Boot Chain

The PQC shard integrates directly with the SigmaOS boot sequence:

```
Stage 1 (BIOS/UEFI)
  └─ Loads Stage 2 bootloader
        └─ pqc_verify(pk_boot, stage2_hash, stage2_sig) ✓
              └─ Loads kernel image
                    └─ pqc_verify(pk_kernel, kernel_hash, kernel_sig) ✓
                          └─ sigma_kernel_main() — PQC-verified boot
```

---

## OmniPackage Integration

All `.omni` packages must be signed with a Dilithium-5 key:

```sh
# Sign a package (developer)
sigma-forge sign --algo dilithium5 package.omni --key dev.sk

# Install (verifies signature automatically)
sigma-pkg install package.omni
# → pqc_verify(registry_pk, package_hash, package_sig) ✓
```

---

## Compliance Engine

Audit reports generated by `sigma_compliance_cli` are signed using Dilithium-5, making them cryptographically tamper-evident. The audit ledger can be exported and its chain verified offline.

---

## Roadmap

- [ ] **Hardware acceleration**: Integrate with AVX-512 NTT butterfly operations for 10× faster polynomial multiplication
- [ ] **liboqs backend**: Replace simulated lattice ops with production pqcrystals-kyber/dilithium reference implementation
- [ ] **FIPS certification path**: Documented entropy injection via hardware RDRAND/RDSEED
- [ ] **Hybrid mode**: Kyber + X25519 combined KEM for transition period

---

> **See also:** [Adaptive Zero-Trust Engine](Zero-Trust-Engine) · [Security Architecture](Security-Architecture) · [Immutable Audit Trail](Immutable-Audit-Trail)
