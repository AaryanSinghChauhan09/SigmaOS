# Post-Quantum Cryptography (PQC)

SigmaOS implements FIPS 203/204/205 compliant post-quantum cryptographic primitives.

## Algorithms Implemented

1. **ML-KEM-1024** (Key Encapsulation Mechanism)

2. **ML-DSA-87** (Digital Signature Algorithm)

3. **SLH-DSA-SHAKE-256s** (Stateless Hash-Based Signatures)

## Implementation Details

- Written in **Rust, no_std** (zero external dependencies, no pre-defined libraries/functions)

- Stored in `/klib/pqc.rs`

- OOP-style registry (`PqcRegistry`) for easy access

## Usage

```rust
use sigmaos::klib::pqc::*;

let mut registry = PqcRegistry::new();
registry.initialize();
// Use kyber/dilithium/sphincsplus via registry
```
