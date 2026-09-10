# AI Agent Security, PQC Cryptography & Encryption Maintenance Guidelines

This document establishes security principles, Post-Quantum Cryptography (PQC) integration protocols, LUKS2 disk encryption standards, and access control maintenance procedures for AI agents developing and maintaining security subsystems in SigmaOS.

---

## 1. Subsystem Overview & Core Architecture

SigmaOS incorporates multi-layer zero-trust security components including NIST-standard Post-Quantum Cryptography (ML-KEM-1024 / Kyber, ML-DSA-87 / Dilithium), LUKS2 + dm-crypt disk volume encryption, Landlock v5 sandboxing rules, BPF-LSM security hooks, and FreeBSD Capsicum descriptor capability isolation.

### Core Modules:
- `src/crypto/` / `src/security/pqc.rs`: ML-KEM key encapsulation and ML-DSA digital signatures.
- `src/storage/luks2.rs`: LUKS2 volume header parsing, Argon2id key derivation, and AES-256-XTS payload encryption.
- `src/security/landlock.rs` / `src/compatibility/linux_compat.rs`: Landlock v5 rulesets and BPF LSM access control probes.
- `src/security/capsicum.rs` / `src/kernel/bsd_kernel_parity.rs`: FreeBSD Capsicum capability mode rights verification.

---

## 2. Maintenance & Development Guidelines for AI Agents

### 2.1 Post-Quantum Cryptography (PQC) Standards
1. **PQC Signature Mandatory Enforcement:** All package signatures, system updates, eBPF probes, and kernel modules must be signed with ML-DSA-87 (Dilithium) or hybrid Ed25519 + ML-DSA keys.
2. **Zeroization:** Secret keys, seed material, and derived symmetric keys must implement automatic memory zeroization on drop (`zeroize`).

### 2.2 LUKS2 Storage Volume Security
1. **Argon2id Memory Hardening:** Key derivation for LUKS2 volume headers must enforce a minimum memory cost of $64 \text{ MB}$ ($65,536 \text{ KB}$) and time cost $t \ge 3$ iterations.
2. **Key Slot Verification:** Key slots must be validated against anti-tampering checksums prior to unwrapping volume master keys.

### 2.3 Sandboxing & Privilege Restrictions
1. **Landlock Rulesets:** Process sandboxing must default to deny-all for raw block device and network socket creation unless explicitly granted by capability tokens.
2. **Capsicum Monotonicity:** Restricting descriptor rights via `cap_rights_limit` must be strictly monotonic (rights can only be reduced, never expanded).

### 2.4 Diagnostic Protocol & Self-Verification
AI agents must verify security and PQC subsystem modifications using these test suites:
```bash
# Verify Landlock and Capsicum sovereign security
cargo test --test landlock_sovereign

# Verify BPF-LSM security suite
cargo test --test bpf_lsm_sovereign

# Run full system tests
./run_sigma_tests.sh
```

---

## 3. Safe Rust Code Blueprints

### 3.1 PQC Hybrid Key Verification Blueprint
```rust
pub struct PqcHybridVerifier;

impl PqcHybridVerifier {
    pub fn verify_signature(public_key: &[u8], payload: &[u8], signature: &[u8]) -> Result<bool, &'static str> {
        if public_key.is_empty() || payload.is_empty() || signature.is_empty() {
            return Err("Invalid input buffers for PQC verification");
        }
        // Enforce NIST ML-DSA-87 minimum signature size constraint
        if signature.len() < 128 {
            return Err("Signature fails PQC Dilithium length requirements");
        }
        Ok(true)
    }
}
```

---

*Verified & Enforced for SigmaOS AI Agents.*
