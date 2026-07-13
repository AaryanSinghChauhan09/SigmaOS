# Future Suggestions & Architectural Roadmap

This document outlines structural suggestions, architectural improvements, and advanced research directions for future versions of the SigmaOS Sovereign Lattice.

---

## 🛡️ 1. Formal Verification (SPARK & Coq Contracts)

To achieve absolute mathematical proof of security and memory safety, we propose expanding formal verification coverage beyond the base IPC layer.

### Target Objectives
- **Verification of EEVDF Scheduler**: Prove that the scheduler runs without deadlocks and guarantees task deadlines mathematically.
- **Verification of PMM Page Allocation**: Formally verify the buddy allocator memory map state machine against double-free errors.
- **Toolchain**: Write Coq specs in `modules/tools/verification/coq/` and run `gnatprove` on Ada kernel modules.

---

## 🔑 2. Full Post-Quantum Cryptographic Migration

Ensure all storage, networking, and system identity structures are safe from quantum computing decryption threats.

### Proposed Cryptographic Suite

| Protocol Layer | Classical Algorithm | Post-Quantum Upgrade | Standard |
| :--- | :--- | :--- | :--- |
| **System Identity** | Ed25519 | **Dilithium5** | FIPS 204 |
| **Key Exchange** | ECDH | **Kyber-1024** | FIPS 203 |
| **Symmetric Cipher** | AES-256 | **AES-256-GCM (No change)** | — |
| **Asset Hashing** | SHA-256 / SHA-512 | **BLAKE3 / SPHINCS+** | FIPS 205 |

---

## ⚡ 3. Neuromorphic Hardware Abstraction Layer

Introduce hardware-accelerated AI scheduling queues designed for neuromorphic processors (e.g. Intel Loihi) to handle local on-device neural networking.

```
       [Sovereign Neuromorphic Scheduler]
                       │
       ┌───────────────┴───────────────┐
       ▼                               ▼
[Spiking Neural Nets]       [Event-Driven Execution]
(Local AI Core execution)   (Sensor-triggered tasks)
```
