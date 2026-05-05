# Security Policy

## Reporting a Vulnerability

**Do not open a public issue.** Please report security vulnerabilities privately to the maintainers at `security@sigmaos.lattice`.

We follow a 90-day disclosure policy. We will acknowledge your report within 48 hours and provide a status update within 7 days.

## Supported Versions

| Version | Supported |
| :--- | :--- |
| Zenith v100.x | ✅ |
| Sovereign v2.x | ✅ |
| Legacy v1.x | ❌ |

## Industrial Hardening Standards

SigmaOS adheres to the following security principles:

### 1. Shard Isolation (Lattice-Level)
No shard can access the matrix without explicit `SovereignEventBus` authorization. The 600-shard architecture ensures that a compromise in one subsystem (e.g., UI) cannot escalate to the kernel core.

### 2. Post-Quantum Cryptography (PQC)
We utilize Lattice-Based Shard Verification (LBSV) for all internal communication. This ensures that the system is resistant to future quantum computing-based decryption threats.

### 3. Hardware-Assisted Attestation
Critical kernel shards are verified via silicon-native trust chains (TEE). The `SovereignAttestation` engine performs real-time measurement of the execution environment.

### 4. Memory Safety & Path Sanitization
- All filesystem requests are strictly validated by the `PathValidator`.
- The `Heap` primitives include active allocation tracking and overflow protection.
- Subscribers must cleanly unmount to prevent event-registry memory leaks.

---

### The Work of Sovereignty is never complete.
