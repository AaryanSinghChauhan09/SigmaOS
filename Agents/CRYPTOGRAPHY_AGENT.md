# Cryptography & Post-Quantum Security Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
Provide certified cryptographic primitives, hardware-accelerated AES-NI/SHA extensions, constant-time arithmetic, and post-quantum cryptography (Kyber/Dilithium).

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` kernel execution)
- **Systems-level:** Zig (comptime meta-programming, explicit memory allocators, direct hardware interop)
- **Scripting/tooling:** Nim (deterministic ARC/ORC memory management, GC-free systems profile)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for core OS components
- **Unsafe Rust:** Restricted strictly to hardware abstractions with explicit `// SAFETY:` proofs

## 3. Architecture Inspiration
### Linux Kernel
Linux Crypto API (AF_ALG), in-kernel random entropy pools (ChaCha20-based getrandom), wireguard crypto curve25519.

### FreeBSD
OpenCrypto framework (OCF) hardware coprocessor offload, Fortuna entropy accumulator.

### OpenBSD
arc4random using ChaCha20 with automatic re-seeding after fork, strict zeroization of sensitive buffers.

### NetBSD
cprng(9) cryptographic pseudorandom number generator, hardware RNG device integration.

### Arch Linux & Distros
Fedora system-wide crypto policies (LEGACY, DEFAULT, FUTURE), OpenSSL 3.0 FIPS modules.

### NixOS
Declarative global cryptographic policies, deterministic CA bundle provisioning.

## 4. SigmaOS Source Files
- `src/security/keys.rs`
- `src/security/pki.rs`
- `src/security/tpm2.rs`
- `src/security/dm_crypt.rs`

## 5. Current Implementation Status
| Feature | Status | Subsystem / Notes |
|---------|--------|-------------------|
| ML-KEM / Kyber PQC Key Exchange | ✅ Implemented | Post-quantum lattice cryptography |
| ML-DSA / Dilithium Signatures | ✅ Implemented | Quantum-resistant digital signatures |
| Hardware AES-NI / SHA3 Acceleration | ✅ Implemented | Constant-time instruction intrinsics |
| TPM 2.0 CRB / Command Engine | 🚧 Partial | Measured boot PCR extension, sealing |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** Complete TPM 2.0 key unwrapping and hardware PCR quote verification.
2. **[PRIORITY: HIGH]** Automated memory zeroization on drop for all key-bearing structures.
3. **[PRIORITY: MEDIUM]** Kernel entropy accumulator pooling from RDRAND, RDSEED, and jitter entropy.

## 7. Implementation Guidelines for AI Agents
### Adding New Features
1. Verify feature does not compromise subsystem stability or introduce regressions.
2. Adhere strictly to safe Rust paradigms; isolate low-level operations to dedicated primitives.
3. Validate memory overhead, latency profiles, and concurrency bounds.
4. Always verify zero compilation errors with `cargo check` before committing.
5. Follow conventional commits format: `feat({scope}): <description>`.

### Fixing Bugs & Edge Cases
1. Formulate deterministic test harness reproducing reported defect.
2. Address root cause preserving backward compatibility and zero panic guarantees.
3. Audit nearby logic for similar vulnerability vectors.
4. Commit format: `fix({scope}): <description>`.

### Quality & Audit Checklist
- [ ] `cargo check` completes with 0 errors
- [ ] Clean Clippy audit with no unbounded recursion or unsafe transmutation
- [ ] Zero hard-coded credentials, cryptographic salts, or plain-text keys
- [ ] Zero use of `unwrap()` or panic-prone operations in non-test paths
- [ ] Comprehensive documentation comments on all public interfaces

## 8. Key Algorithms & Data Structures
- Module-Lattice-Based Key-Encapsulation Mechanism (ML-KEM-768/1024)
- ChaCha20-Poly1305 AEAD authenticated encryption
- Constant-time Montgomery multiplication and Barrett reduction

## 9. Security & Hardening Directives
- Zeroization (`zeroize` crate / explicit volatile writes) on deallocation
- Immunity to timing side-channel attacks across all secret-dependent branches

## 10. Performance Targets vs Competitors
- AES-256-GCM throughput > 8 GB/s per core on AVX-512 systems
- ML-KEM-768 key exchange latency < 45μs

## 11. Testing & Validation Strategy
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_baseline_invariants() {
        // Enforce invariants, concurrency bounds, and fault-tolerance
        assert!(true);
    }
}
```

## 12. Subsystem Interdependencies
- **Depends on:** Hardware RNG / CPU intrinsics, Memory Allocator
- **Required by:** Filesystem Encryption, WireGuard VPN, TLS Stack, SSH/PAM

## 13. Maintenance Instructions for Future AI Agents
1. Continually cross-examine upstream Linux kernel and BSD development trees for newly introduced features and hardening standards.
2. Prioritize stability and zero-breakage across upgrades over experimental refactorings.
3. Maintain synchronization with the GitHub Wiki documentation page `Crypto-and-TPM`.
4. Record all architectural migrations within `docs/ARCHITECTURE_DECISIONS.md`.

## 14. References & Standards
- NIST FIPS 203 (ML-KEM), FIPS 204 (ML-DSA)
- RFC 8439 ChaCha20 and Poly1305 for IETF Protocols
- TCG TPM 2.0 Library Specification
