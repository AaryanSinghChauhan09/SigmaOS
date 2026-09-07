# AI Agent Development Instructions for Confidentiality, Enclaves & Secret Vault Management (`src/secure/`, `src/tpm/`, `src/auth/`)

This document outlines guidelines for hardware enclave confidentiality (AMD SEV-SNP, Intel SGX/TDX, ARM Realm Extension), TPM 2.0 PCR sealing, Zero-Knowledge secret storage, secure zeroization, and user credential confidentiality in SigmaOS.

## Subsystem Architecture & Directives

1. **Hardware Confidential Enclaves (`src/secure/enclave.rs`)**
   - Enclave memory boundaries MUST be encrypted using hardware AES-XTS memory encryption engines.
   - Enclave attestation reports must verify post-quantum Dilithium / ML-DSA signature chains before injecting sensitive secrets into enclave memory spaces.

2. **TPM 2.0 PCR Sealing & Secret Binding (`src/tpm/tpm2_implementation.rs`)**
   - Bind encryption keys and master disk credentials to TPM 2.0 Platform Configuration Registers (`PCR0`-`PCR7` boot measurement logs).
   - Reject secret unsealing if PCR measurements indicate unauthorized boot loader or kernel modification.

3. **Secure Zeroization & Memory Sanitization (`src/kernel/secure_free.rs` & `secrets.rs`)**
   - Key material, passphrases, and decrypted session buffers MUST be zeroized explicitly (`core::ptr::write_bytes` / `volatile` zeroing or `zeroize` parity) immediately upon deallocation or drop. Never allow sensitive key fragments to linger in heap or stack space.

4. **Confidentiality & Access Governance (`src/auth/att_security.rs` & `systemd_homed.rs`)**
   - Home directory disk encryption (LUKS / fscrypt parity) must isolate user data confidentiality across multi-tenant sessions.
   - Enforce strict memory locking (`mlock` / `fincore`) on password hashes and cryptographic keys to prevent secret paging to swap space.

5. **Verification**
   - Validate changes with `cargo check --lib`.
