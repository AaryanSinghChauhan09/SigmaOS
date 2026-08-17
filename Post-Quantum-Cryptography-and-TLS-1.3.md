# Post-Quantum Cryptography & TLS 1.3 0-RTT in SigmaOS

## Overview

SigmaOS implements a native, pure-Rust **TLS 1.3** network transport stack with built-in **Post-Quantum Cryptography (PQC)** hybrid key exchange mechanisms (X25519 + ML-KEM / Kyber-768).

---

## Key Modules

- [`src/crypto/pqc_dilithium.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/crypto/pqc_dilithium.rs): Post-quantum digital signatures (ML-DSA).
- [`src/docs/PQC-TLS13-0RTT-Network-Spec.md`](file:///home/aaryansinghchauhan/SigmaOS/src/docs/PQC-TLS13-0RTT-Network-Spec.md): Protocol state machine and cryptographic handshake specification.
- [`src/net/tls.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/net/tls.rs): Native zero-copy TLS record layer.

---

## Handshake Architecture

```
Client                                               Server
  │                                                     │
  │─── ClientHello + KeyShare (X25519 + ML-KEM-768) ───>│
  │    + EarlyData (0-RTT Session Ticket)               │
  │                                                     │
  │<── ServerHello + KeyShare ──────────────────────────│
  │    {EncryptedExtensions}                            │
  │    {Certificate (ML-DSA)}                           │
  │    {CertVerify}                                     │
  │    {Finished}                                       │
  │                                                     │
  │─── {Finished} ─────────────────────────────────────>│
  │                                                     │
  │<═══════════ [Quantum-Safe Encrypted Channel] ══════>│
```
