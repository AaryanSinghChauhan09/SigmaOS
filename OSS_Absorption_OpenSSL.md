# OSS Absorption: OpenSSL — Cryptographic Engine

> **Status**: 📋 Planned | **Source Project**: OpenSSL | **Target Shard**: `SigmaOS Cryptographic Library & Hardware Shard`

---

## 1. Executive Summary

OpenSSL is the industry-standard cryptography toolkit implementing SSL/TLS protocols and a wide range of cryptographic algorithms. 

While SigmaOS targets memory safety through Rust-native cryptography (like `ring`), it absorbs OpenSSL's **assembly-optimized hardware acceleration** routines for x86_64 (AVX-512) and ARM (NEON/AES), wrapping them in a secure FFI layer to guarantee top-tier performance for cryptographic operations.

---

## 2. Key Features Absorbed

### 2.1 Hardware-Optimized Assembly Core

For critical operations (such as AES-GCM and SHA-256), memory safety alone is not enough; side-channel resistance and maximum throughput are mandatory. SigmaOS imports OpenSSL's highly optimized, constant-time assembly routines.

```rust
// kernel/crypto/aes.rs
// SPDX-License-Identifier: MIT

extern "C" {
    // Call direct, audited assembly optimizations from OpenSSL
    fn aesni_encrypt_gcm(input: *const u8, output: *mut u8, len: usize, key: *const u8);
}
```

### 2.2 Quantum-Resistant Hybrid Fallback

SigmaOS implements TLS 1.3 by default, wrapping the OpenSSL-derived cryptographic hardware accelerations in a hybrid wrapper with post-quantum cryptography (Kyber/Dilithium).

---

## 3. References & Standards

- OpenSSL — `openssl.org` (Apache-2.0 License)
- Ring Cryptography — `github.com/briansmith/ring` (ISC-like)
