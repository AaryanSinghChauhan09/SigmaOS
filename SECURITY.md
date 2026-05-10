# Security & PQC

## Post-Quantum Cryptography (PQC) Algorithms
SigmaOS integrates cutting-edge quantum-resistant algorithms via `liboqs`:
- **CRYSTALS-Kyber**: Used for Key Encapsulation Mechanisms (KEM) to secure the Sovereign IPC Bus and re-seed the entropy pool.
- **CRYSTALS-Dilithium**: Used for digital signatures to verify kernel shard integrity (Lattice-Based Shard Verification) at boot.

## Integration Points
- **Kernel Hooks**: `SovereignPQC.cpp` exposes `pqc_sign_shard()` and `pqc_verify_shard()` which the kernel automatically invokes when loading a new module.
- **Userland Verification**: Userland applications can verify signed binaries via the `sigma-pkg` distribution layer before execution.

## Compliance
- **FIPS-140**: (Targeted) All cryptographic modules are designed to be submitted for FIPS validation.
- **Hardware Attestation**: TPM 2.0 validates the bootloader and kernel hash before executing the `init` process.
