# SigmaOS AI Agents Security Management & Hardening Guide

Welcome to the **SigmaOS AI Agents Security Management & Hardening Guide**. This document details security enforcement protocols, capability sandboxing, Post-Quantum Cryptography (PQC), Mandatory Access Control (MAC), and forensic audit requirements for autonomous AI agents and security developers in SigmaOS.

---

## 1. Zero-Trust Security Architecture

SigmaOS enforces a strict **Zero-Trust Security Architecture** across all 12 Sovereign System Shards. AI agents executing system operations or modifying security policies MUST adhere to the following principles:

1. **Least Privilege Enforcement**: Every process, tab, and autonomous subagent operates with the absolute minimum privilege set required.
2. **Multi-Model Access Control**: Unified enforcement combining Discretionary Access Control (DAC), Mandatory Access Control (MAC via SELinux & AppArmor LSM hooks), and Role-Based Access Control (RBAC).
3. **Cryptographic Attestation**: All binaries, drivers, and package manifests require Post-Quantum Cryptography (PQC Dilithium-5 signatures and Kyber-1024 key encapsulation) attestation before launch.

---

## 2. Capability Sandboxing (Pledge, Unveil & Capsicum)

AI agents executing untrusted code or managing third-party extensions MUST enforce platform-native capability sandboxing (`src/security/pledge.rs`, `src/security/unveil.rs`, `src/security/capsicum.rs`):

### OpenBSD-Style `pledge(2)` and `unveil(2)` API
```rust
use sigmaos::security::pledge::{PledgeManager, PromiseToken};

let mut manager = PledgeManager::new();
// Restrict process promises to stdio, read-path, and exec
let token = PromiseToken::empty()
    .allow_stdio()
    .allow_rpath()
    .allow_exec();

manager.apply_promises(token).expect("Pledge enforcement failed");
```

```rust
use sigmaos::security::unveil::UnveilManager;

let mut unveil = UnveilManager::new();
// Restrict filesystem access to /usr/share and /tmp
unveil.unveil_path("/usr/share", "r").expect("Unveil failed");
unveil.unveil_path("/tmp", "rw").expect("Unveil failed");
unveil.lock();
```

---

## 3. Post-Quantum Cryptography & Key Management

SigmaOS integrates PQC algorithms in `src/crypto/post_quantum.rs` and `src/security/pqc_enclave.rs`:

- **Dilithium-5**: Post-quantum lattice-based digital signatures for driver signing and release artifact verification.
- **Kyber-1024**: Post-quantum key encapsulation mechanism (KEM) for secure agent-to-agent IPC channels and PQC VPN tunnels.
- **Randomness Generation**: Secure hardware entropy pooling via `x86_64` RDRAND/RDSEED, `aarch64` RNDR, and xorshift fallback RNGs without hardcoded keys or static nonces.

---

## 4. Digital Forensics & Chain-of-Custody Tracking

For security incident response and threat investigation, AI agents should utilize `SovereignForensicsEngine` (`src/security/forensics.rs`):

- **Volatile Memory Dump Acquisition**: Cryptographic SHA-256 hash calculation of RAM state.
- **File Header Carving**: Autonomous detection of ELF, PE, PDF, PNG, and ZIP headers in raw disk images.
- **Evidence Chain-of-Custody**: Immutable evidence records tracking examiner ID, timestamps, and hash integrity.

---

## 5. Security Verification & Input Sanitization

Before submitting security changes, AI agents MUST run the security input validation test suite:

```bash
cargo test --test input_validation_tests
# Or run the complete 13-stage test runner
./run_sigma_tests.sh
```

---

## 6. Checklist for AI Agents Managing Security Subsystems

- [ ] Ensured all process launches apply `pledge`/`unveil` or `seccomp` filters.
- [ ] Verified nonces and keys are randomly generated without hardcoded values.
- [ ] Confirmed MAC/LSM hooks check Inode, Ptrace, and Socket access rules.
- [ ] Added unit tests verifying permission rejection on unauthorized syscall attempts.
- [ ] Executed `./run_sigma_tests.sh` to confirm zero security test failures.
