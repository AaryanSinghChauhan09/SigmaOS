# SigmaOS Security Policy

## Supported Versions

| Version | Status               |
| ------- | -------------------- |

| v15.x   | Actively supported   |
| v14.x   | Security fixes only  |
| < v14   | End of Life          |

## Reporting a Vulnerability

If you discover a security vulnerability in the SigmaOS Sovereign Lattice, **do not open a public issue**. Instead:

1. Email the **SigmaOS Security Council** via the contact listed on the repository profile.

2. Include: affected shard name, reproduction steps, and potential impact.

3. You will receive a response within **72 hours**.

We operate a responsible disclosure policy with a **90-day coordinated fix window**.

---

## Security Architecture

SigmaOS employs multiple layers of sovereign defence:

### Ring-0 Access Control

- **`SovereignSEL`** Â— Mandatory Access Control (MAC) enforced at Ring-0 via capability tables. No process can escalate privileges without a PQC-signed policy token.

- **`SovereignEnclave`** Â— Hardware-level isolation (Intel TDX / AMD SEV) for all cryptographic state. Key material never exists in accessible RAM.

### Post-Quantum Cryptography

- **`SovereignPQC`** Â— CRYSTALS-Kyber-1024 (KEM) + CRYSTALS-Dilithium-5 (DSA) per NIST FIPS 203/204.

- All shard packages are Dilithium-5 signed before distribution via S-PKG.

- Session keys are ephemeral and wiped via `sigma_secure_memset` after use.

### Process & Memory Isolation

- **`SovereignSandbox`** Â— Zero-trust container isolation for all Ring-3 processes. Each process gets its own CR3 page directory.

- **`SovereignMemoryManager`** Â— Double-free detection + amnesic wipe on every `deallocate()` call.

- **NX / SMEP / SMAP** Â— Hardware-enforced no-execute + supervisor-mode protection.

### Continuous Security Monitoring

- **`SovereignWatchdog`** Â— Kernel heartbeat monitor; triggers atomic rollback on deadlock.

- **`SovereignAudit`** Â— Immutable audit log for all Ring-0 operations.

- **CodeQL** Â— Automated vulnerability scanning on every pull request.

- **Dependabot** Â— Weekly automated supply-chain patch management.

---

## Security vs. Linux & Windows

| Property                  | SigmaOS         | Linux          | Windows         |
|---------------------------|-----------------|----------------|-----------------|
| PQC native (kernel)       | YES (Kyber-1024)| NO             | NO              |
| Amnesic memory wipe       | YES (always)    | Optional only  | NO              |
| Per-process CR3 isolation | YES             | YES (standard) | YES (standard)  |
| Double-free detection     | YES (kernel)    | Partial (KASAN)| Partial         |
| Atomic rollback           | YES             | NO             | NO              |
| 0-dependency boot         | YES             | NO (glibc)     | NO (ntdll)      |

---

*Stay Sovereign. Stay Quantum-Safe.*
