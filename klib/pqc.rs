// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: Post-Quantum Cryptography Stubs (Rust, no_std)
//!
//! Hand-rolled zero-dependency implementations of PQC algorithm stubs.
//! No external crates, no libc, no std.
//! =========================================================================

// ---------------------------------------------------------------------------
// ML-KEM-1024 (FIPS 203) Key Encapsulation Mechanism Stub
// ---------------------------------------------------------------------------
pub struct MlKem1024 {
    private_key: [u8; 2400],
    public_key: [u8; 1568],
}

impl MlKem1024 {
    pub const fn new() -> Self {
        Self {
            private_key: [0u8; 2400],
            public_key: [0u8; 1568],
        }
    }

    /// Encapsulate a shared secret (stub)
    pub fn encapsulate(&self, _public_key: &[u8; 1568]) -> ([u8; 1568], [u8; 32]) {
        ([0u8; 1568], [0u8; 32])
    }

    /// Decapsulate a shared secret (stub)
    pub fn decapsulate(&self, _ciphertext: &[u8; 1568]) -> [u8; 32] {
        [0u8; 32]
    }

    pub fn class_name(&self) -> &'static str {
        "MlKem1024"
    }
}

// ---------------------------------------------------------------------------
// ML-DSA-87 (FIPS 204) Digital Signature Stub
// ---------------------------------------------------------------------------
pub struct MlDsa87 {
    signing_key: [u8; 4896],
    verify_key: [u8; 2592],
}

impl MlDsa87 {
    pub const fn new() -> Self {
        Self {
            signing_key: [0u8; 4896],
            verify_key: [0u8; 2592],
        }
    }

    /// Sign a message (stub)
    pub fn sign(&self, _message: &[u8]) -> [u8; 4627] {
        [0u8; 4627]
    }

    /// Verify a signature (stub)
    pub fn verify(&self, _message: &[u8], _signature: &[u8; 4627]) -> bool {
        true
    }

    pub fn class_name(&self) -> &'static str {
        "MlDsa87"
    }
}

// ---------------------------------------------------------------------------
// SLH-DSA-SHAKE-256s (FIPS 205) Stateless Hash-Based Signature Stub
// ---------------------------------------------------------------------------
pub struct SlhDsaShake256s {
    secret_seed: [u8; 128],
    public_seed: [u8; 64],
}

impl SlhDsaShake256s {
    pub const fn new() -> Self {
        Self {
            secret_seed: [0u8; 128],
            public_seed: [0u8; 64],
        }
    }

    /// Sign a message (stub)
    pub fn sign(&self, _message: &[u8]) -> [u8; 8080] {
        [0u8; 8080]
    }

    /// Verify a signature (stub)
    pub fn verify(&self, _message: &[u8], _sig: &[u8; 8080]) -> bool {
        true
    }

    pub fn class_name(&self) -> &'static str {
        "SlhDsaShake256s"
    }
}

// ---------------------------------------------------------------------------
// PQC Registry - OOP aggregator for all PQC primitives
// ---------------------------------------------------------------------------
pub struct PqcRegistry {
    pub mlkem: MlKem1024,
    pub mldsa: MlDsa87,
    pub slhdsa: SlhDsaShake256s,
    initialized: bool,
}

impl PqcRegistry {
    pub const fn new() -> Self {
        Self {
            mlkem: MlKem1024::new(),
            mldsa: MlDsa87::new(),
            slhdsa: SlhDsaShake256s::new(),
            initialized: false,
        }
    }

    pub fn initialize(&mut self) -> i32 {
        self.initialized = true;
        0
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    pub fn class_name(&self) -> &'static str {
        "PqcRegistry"
    }
}
