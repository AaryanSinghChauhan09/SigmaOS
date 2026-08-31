# Sovereign Cryptography Guide in SigmaOS

## Overview

SigmaOS utilizes a sovereign, zero-external-dependency cryptographic suite written in memory-safe Rust with constant-time algorithmic implementations to resist side-channel attacks.

***

## Cryptographic Primitives

    ┌─────────────────────────────────────────────────────────────┐
    │                    SigmaOS Crypto Suite                     │
    ├──────────────────────────────┬──────────────────────────────┤
    │ Symmetric Cryptography       │ Asymmetric / Signatures      │
    │ - ChaCha20-Poly1305 (AEAD)   │ - Ed25519 (Signatures)       │
    │ - AES-256-GCM                │ - X25519 (Key Exchange)      │
    ├──────────────────────────────┼──────────────────────────────┤
    │ Hashing & KDF                │ Post-Quantum Cryptography    │
    │ - BLAKE3 (High-speed hash)   │ - ML-KEM / Kyber (KEM)       │
    │ - Argon2id (Memory-hard KDF) │ - ML-DSA / Dilithium (PQC)   │
    └──────────────────────────────┴──────────────────────────────┘

***

## Key Modules

*   [`src/crypto/libsodium.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/crypto/libsodium.rs): Pure-Rust libsodium-compatible API (BoxCipher, SecretBox, Sign, Scalarmult).
*   [`kernel/crypto/chacha20.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/kernel/crypto/chacha20.rs): In-kernel ChaCha20 engine for fast disk & network packet encryption.
*   [`crypto/sigma_key_derive.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/crypto/sigma_key_derive.rs): Argon2id password-based key derivation engine.
*   [`src/crypto/pqc_dilithium.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/crypto/pqc_dilithium.rs): Post-quantum lattice signature engine.

***

## Security Invariants

1.  **Constant-Time Verification**: All MAC, tag, and signature comparisons use `subtle::ConstantTimeEq` semantics to prevent timing attacks.
2.  **Zero-On-Drop**: Sensitive key buffers implement `Zeroize` on deallocation.
3.  **No Hard-Coded Keys**: Production cryptographic keys are strictly derived at runtime or stored in TPM-backed secure hardware enclaves.
