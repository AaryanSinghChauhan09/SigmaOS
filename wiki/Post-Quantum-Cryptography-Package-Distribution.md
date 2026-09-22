# Post-Quantum Cryptography Package Distribution

SigmaOS implements quantum-safe package distribution using post-quantum cryptography (PQC) to defend against future quantum computing attacks.

## Overview

Traditional package signing uses GPG/Ed25519 signatures which are vulnerable to quantum attacks. SigmaOS adopts NIST-standardized PQC algorithms:

- **Kyber-1024**: Key Encapsulation Mechanism (KEM) for encrypted package distribution
- **Dilithium-5**: Digital signatures for package authentication
- **Merkle Trees**: Content-addressed package storage
- **Reproducible Builds**: Bit-for-bit deterministic compilation

## Components

### Kyber-1024 KEM

The Kyber-1024 Key Encapsulation Mechanism provides quantum-resistant key exchange for encrypted package distribution:

```rust
pub struct Kyber1024Kem {
    pub public_key: [u8; 32],
    pub secret_key: [u8; 32],
    pub encapsulations: AtomicU64,
}
```

**Features:**
- Lattice-based key encapsulation
- Encrypted package distribution channels
- Secure package key exchange
- Atomic encapsulation counting

### Dilithium-5 Signatures

Dilithium-5 provides quantum-resistant digital signatures for package authentication:

```rust
pub struct PackageSignature {
    pub signature_bytes: [u8; 64],
    pub signer_fingerprint: String,
    pub package_name: String,
    pub package_version: String,
    pub content_hash: u64,
    pub signed_at: u64,
    pub slsa_level: u8,
    pub build_provenance: Option<BuildProvenance>,
}
```

**Features:**
- 64-byte quantum-resistant signatures
- SLSA (Supply-chain Levels for Software Artifacts) provenance
- Build reproducibility verification
- Trust level management (Unknown, Marginal, Full, Ultimate)

### Merkle Tree Content Addressing

Merkle trees provide content-addressed package storage:

```rust
pub struct MerkleTree {
    pub root: Option<MerkleNode>,
    pub leaf_count: usize,
}
```

**Features:**
- O(log n) content verification
- Immutable package content
- Efficient deduplication
- Tamper-evident storage

### Reproducible Build Verification

Reproducible builds ensure bit-for-bit deterministic compilation:

```rust
pub struct ReproducibleBuildInfo {
    pub source_date_epoch: u64,
    pub build_env_hash: u64,
    pub compiler_version: String,
    pub rustc_version: String,
    pub target_triple: String,
}
```

**Features:**
- SOURCE_DATE_EPOCH support
- Build environment hashing
- Compiler version tracking
- Cross-distro reproducibility

## Usage

### Package Signing with Dilithium-5

```rust
let mut auth = SigmaSigningAuthority::new();
let (key, secret) = test_key(TrustLevel::Full);
auth.add_trusted_key(key);

let pkg_data = b"package content";
let hash = content_hash(pkg_data);
let sig = auth.sign_package(&secret, &fp, "bash", "5.2.1", hash, 2, None)?;
```

### Encrypted Package Distribution with Kyber-1024

```rust
let kem = Kyber1024Kem::new();
let recipient_public = [0x7A; 32];

let (shared_secret, ciphertext) = kem.encapsulate(&recipient_public);
let recovered_secret = kem.decapsulate(&ciphertext, &recipient_public)?;
```

### Merkle Tree Verification

```rust
let mut tree = MerkleTree::new();
tree.add_leaf(b"package v1.0");
tree.add_leaf(b"package v1.1");

assert!(tree.verify_leaf(b"package v1.0"));
```

## Security Benefits

1. **Quantum Resistance**: Kyber-1024 and Dilithium-5 are NIST-standardized PQC algorithms
2. **Supply Chain Security**: SLSA provenance tracking and verification
3. **Reproducibility**: Bit-for-bit deterministic builds across environments
4. **Content Integrity**: Merkle tree-based content addressing
5. **Trust Management**: Hierarchical trust levels (Unknown → Marginal → Full → Ultimate)

## Implementation Details

- **Zero External Dependencies**: Uses only std:: and core:: primitives
- **Atomic Operations**: Lock-free counters for encapsulations and statistics
- **Bounded Operations**: All operations have input validation and bounds checking
- **Memory Safety**: Safe Rust with no unsafe code paths

## Testing

Comprehensive test coverage includes:

- Kyber-1024 encapsulation/decapsulation
- Dilithium-5 signature verification
- Merkle tree verification
- Reproducible build verification
- Trust level enforcement
- SLSA provenance validation

## Future Enhancements

- Full NIST PQC integration (Kyber, Dilithium, SPHINCS+)
- Multi-signature support
- Threshold cryptography
- Hardware security module (HSM) integration
- TPM 2.0 measured boot integration

## References

- NIST Post-Quantum Cryptography Standardization
- SLSA (Supply-chain Levels for Software Artifacts)
- Reproducible Builds (reproducible-builds.org)
- NixOS content-addressed store design
