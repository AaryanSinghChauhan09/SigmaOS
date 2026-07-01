# SigmaOS Security Architecture

SigmaOS enforces security through a combination of traditional sandboxing and modern Zero-Trust principles, operating entirely in a zero-dependency environment.

## 1. Secure Boot & Attestation
- **Post-Quantum Cryptography (PQC)**: Uses CRYSTALS-Dilithium for verifying kernel and driver signatures during boot.
- **Root of Trust**: Hardcoded or TPM-sealed keys establish the initial trust anchor.
- **Rollback Protection**: Prevents downgrading to vulnerable older versions using monotonic counters.
- **TPM Measurements**: PCR[0] is extended with the SHA-256 hash of every loaded component.

## 2. Zero-Trust Enforcer (sigma_zero_trust)
- Every inter-process communication (IPC) channel requires explicit mutual authentication.
- Uses CRYSTALS-Kyber Key Encapsulation Mechanism (KEM) to establish shared session keys between processes.
- **Runtime Attestation**: Processes are verified periodically against their expected code hashes.

## 3. Capability-Based Sandboxing (sigma_sandbox)
Instead of coarse ACLs, SigmaOS uses a fine-grained capability bitmask (`CAP_NET`, `CAP_FS`, `CAP_IPC`, `CAP_HW`, `CAP_ADMIN`).
- **Namespaces**: Process ID (PID), Network, Mount, and IPC namespaces are fully isolated per sandbox profile.
- **cgroups-lite**: Enforces strict CPU time and memory allocation limits.
- **Syscall Filtering**: A bitmask allows explicitly granting only necessary system calls to a process (similar to seccomp).

## 4. Cryptographic Primitives
- **Symmetric**: AES-256 (GCM mode for authenticated encryption).
- **Hashing**: SHA-256.
- **Asymmetric/PQC**: CRYSTALS-Kyber (KEM), CRYSTALS-Dilithium (Signatures).
