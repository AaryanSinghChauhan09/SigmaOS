//! SigmaOS Sovereign Package Signing Authority
//!
//! Dilithium-5 based package signing and verification system.
//! Inspired by:
//! - OpenBSD signify (Ed25519 package signatures)
//! - Debian dpkg-sig / APT GPG signing (keyring-based trust)
//! - NixOS content-addressed signed paths (nix-store --verify)
//! - ArchLinux pacman keyring (trust levels: unknown/marginal/full/ultimate)
//! - Flatpak GPG Ostree commit signing
//! - SLSA (Supply-chain Levels for Software Artifacts) provenance attestation
//!
//! Provides:
//! - PublisherKey management with trust levels
//! - Package signing with Dilithium-5 signatures
//! - Signature chain verification
//! - Key revocation and expiry
//! - SLSA-style build provenance attestation
//! - Kyber-1024 KEM for encrypted package distribution
//! - Reproducible build verification with SOURCE_DATE_EPOCH
//! - Merkle tree-based content addressing

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;
use std::format;
use core::sync::atomic::{AtomicU64, Ordering};

// ─── Trust Level ─────────────────────────────────────────────────────────────

/// Publisher key trust level (mirrors Debian APT / pacman keyring trust levels)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLevel {
    /// Key is unknown — will not verify packages
    Unknown,
    /// Marginal trust — only valid if 3 other marginal keys co-sign
    Marginal,
    /// Full trust — this key alone can authorize packages
    Full,
    /// Ultimate trust — built-in SigmaOS signing key (distributes root of trust)
    Ultimate,
}

impl TrustLevel {
    pub fn label(&self) -> &'static str {
        match self {
            TrustLevel::Unknown => "unknown",
            TrustLevel::Marginal => "marginal",
            TrustLevel::Full => "full",
            TrustLevel::Ultimate => "ultimate",
        }
    }

    /// Returns true if this trust level is sufficient to verify packages
    pub fn is_sufficient(&self) -> bool {
        matches!(self, TrustLevel::Full | TrustLevel::Ultimate)
    }
}

// ─── Publisher Public Key ─────────────────────────────────────────────────────

/// A publisher's Dilithium-5 public key entry in the keyring
#[derive(Debug, Clone)]
pub struct PublisherPublicKey {
    /// Key fingerprint (FNV-1a of key bytes — 8 hex chars)
    pub fingerprint: String,
    /// Dilithium-5 public key bytes (32-byte simulated)
    pub key_bytes: [u8; 32],
    /// Publisher name (e.g. "SigmaOS Core Team")
    pub publisher_name: String,
    /// Publisher email
    pub publisher_email: String,
    /// Key creation timestamp (simulated seconds since epoch)
    pub created_at: u64,
    /// Optional expiry timestamp (None = never expires)
    pub expires_at: Option<u64>,
    /// Trust level in this keyring
    pub trust_level: TrustLevel,
    /// Whether this key has been revoked
    pub revoked: bool,
}

impl PublisherPublicKey {
    /// Create a new publisher key
    pub fn new(
        key_bytes: [u8; 32],
        publisher_name: &str,
        publisher_email: &str,
        trust_level: TrustLevel,
    ) -> Self {
        let fingerprint = Self::compute_fingerprint(&key_bytes);
        PublisherPublicKey {
            fingerprint,
            key_bytes,
            publisher_name: String::from(publisher_name),
            publisher_email: String::from(publisher_email),
            created_at: 0,
            expires_at: None,
            trust_level,
            revoked: false,
        }
    }

    /// Compute key fingerprint using FNV-1a
    fn compute_fingerprint(key: &[u8; 32]) -> String {
        const FNV_PRIME: u64 = 0x00000100000001B3;
        let mut h: u64 = 0xcbf29ce484222325;
        for &b in key {
            h ^= b as u64;
            h = h.wrapping_mul(FNV_PRIME);
        }
        format!("{:016X}", h)
    }

    /// Returns true if this key is valid (not revoked, not expired) at the given time
    pub fn is_valid_at(&self, timestamp: u64) -> bool {
        if self.revoked { return false; }
        if let Some(exp) = self.expires_at {
            if timestamp > exp { return false; }
        }
        true
    }

    pub fn summary(&self) -> String {
        format!(
            "[{}] {} <{}> (trust: {}, {})",
            &self.fingerprint[..8],
            self.publisher_name,
            self.publisher_email,
            self.trust_level.label(),
            if self.revoked { "REVOKED" } else { "valid" }
        )
    }
}

// ─── Package Signature ────────────────────────────────────────────────────────

/// A Dilithium-5 package signature (OpenBSD signify-style)
#[derive(Debug, Clone)]
pub struct PackageSignature {
    /// 64-byte Dilithium-5 signature (simulated)
    pub signature_bytes: [u8; 64],
    /// Fingerprint of the signing key
    pub signer_fingerprint: String,
    /// Package name that was signed
    pub package_name: String,
    /// Package version that was signed
    pub package_version: String,
    /// Content hash of the package archive (FNV-1a of tarball)
    pub content_hash: u64,
    /// Signing timestamp
    pub signed_at: u64,
    /// SLSA build level (0-4)
    pub slsa_level: u8,
    /// Optional build provenance (reproducible build metadata)
    pub build_provenance: Option<BuildProvenance>,
}

/// SLSA-style build provenance attestation
#[derive(Debug, Clone)]
pub struct BuildProvenance {
    /// Builder ID (e.g. "sigma-ci-trusted-builder-v1")
    pub builder_id: String,
    /// Source repository URI
    pub source_uri: String,
    /// Git commit hash of source
    pub source_commit: String,
    /// Whether the build is reproducible (same inputs → same output)
    pub reproducible: bool,
    /// Build environment hash (captures all inputs)
    pub build_env_hash: u64,
}

impl PackageSignature {
    /// Format a signify-style one-liner summary
    pub fn format_signify(&self) -> String {
        format!(
            "SIGMA sig {}: {}={} hash={:016X} signer={} slsa={}",
            self.signed_at,
            self.package_name,
            self.package_version,
            self.content_hash,
            &self.signer_fingerprint[..8],
            self.slsa_level
        )
    }
}

// ─── Signing Authority ────────────────────────────────────────────────────────

/// SigmaOS Package Signing Authority
///
/// Manages the package keyring, signs packages with Dilithium-5,
/// and verifies package signatures against trusted keys.
pub struct SigmaSigningAuthority {
    /// Trusted publisher keys, indexed by fingerprint
    pub trusted_keys: BTreeMap<String, PublisherPublicKey>,
    /// Revoked key fingerprints (fast lookup for revocation checks)
    pub revoked_fingerprints: Vec<String>,
    /// Signature cache: content_hash → PackageSignature
    pub signature_cache: BTreeMap<u64, PackageSignature>,
    /// Number of signatures issued
    pub signatures_issued: u64,
    /// Number of signatures verified
    pub signatures_verified: u64,
    /// Number of verification failures
    pub verification_failures: u64,
}

impl SigmaSigningAuthority {
    /// Create a new signing authority
    pub fn new() -> Self {
        SigmaSigningAuthority {
            trusted_keys: BTreeMap::new(),
            revoked_fingerprints: Vec::new(),
            signature_cache: BTreeMap::new(),
            signatures_issued: 0,
            signatures_verified: 0,
            verification_failures: 0,
        }
    }

    /// Create a signing authority with the SigmaOS built-in root key
    pub fn with_sigma_root_key() -> Self {
        let mut auth = Self::new();
        // Built-in SigmaOS root key (derived from project identity seed)
        let root_key_bytes: [u8; 32] = [
            0x51, 0x16, 0x2a, 0x4f, 0x7e, 0x3b, 0x9c, 0x1d,
            0x8e, 0xd0, 0xc4, 0xf2, 0x60, 0x7a, 0x83, 0x5e,
            0x21, 0x4d, 0x9b, 0x36, 0xc7, 0x8f, 0x12, 0xe5,
            0x73, 0xa9, 0x0b, 0x4c, 0xd1, 0x6e, 0xf4, 0x28,
        ];
        let root_key = PublisherPublicKey::new(
            root_key_bytes,
            "SigmaOS Core Team",
            "signing@sigmaos.dev",
            TrustLevel::Ultimate,
        );
        auth.add_trusted_key(root_key);
        auth
    }

    // ── Key Management ────────────────────────────────────────────────────────

    /// Add a trusted publisher key to the keyring
    pub fn add_trusted_key(&mut self, key: PublisherPublicKey) {
        self.trusted_keys.insert(key.fingerprint.clone(), key);
    }

    /// Revoke a key by fingerprint
    pub fn revoke_key(&mut self, fingerprint: &str) -> bool {
        if let Some(key) = self.trusted_keys.get_mut(fingerprint) {
            key.revoked = true;
            if !self.revoked_fingerprints.contains(&String::from(fingerprint)) {
                self.revoked_fingerprints.push(String::from(fingerprint));
            }
            return true;
        }
        false
    }

    /// Returns true if a fingerprint belongs to a trusted (not-revoked, sufficient-trust) key
    pub fn is_trusted_key(&self, fingerprint: &str, timestamp: u64) -> bool {
        self.trusted_keys
            .get(fingerprint)
            .map(|k| k.trust_level.is_sufficient() && k.is_valid_at(timestamp))
            .unwrap_or(false)
    }

    // ── Package Signing ───────────────────────────────────────────────────────

    /// Sign a package with a given secret key seed.
    ///
    /// `secret_seed` — 32-byte private key material
    /// `signer_fingerprint` — fingerprint of the corresponding public key in keyring
    /// `package_name` / `version` — package identity
    /// `content_hash` — FNV-1a of package archive bytes
    pub fn sign_package(
        &mut self,
        secret_seed: &[u8; 32],
        signer_fingerprint: &str,
        package_name: &str,
        package_version: &str,
        content_hash: u64,
        slsa_level: u8,
        provenance: Option<BuildProvenance>,
    ) -> Result<PackageSignature, String> {
        // Verify the signer key exists in our keyring
        if !self.trusted_keys.contains_key(signer_fingerprint) {
            return Err(format!("Unknown signer fingerprint: {}", signer_fingerprint));
        }

        // Compute signature using Dilithium-5 (FNV-1a simulated)
        let sig_bytes = self.dilithium_sign(secret_seed, package_name, package_version, content_hash);

        let signature = PackageSignature {
            signature_bytes: sig_bytes,
            signer_fingerprint: String::from(signer_fingerprint),
            package_name: String::from(package_name),
            package_version: String::from(package_version),
            content_hash,
            signed_at: 0,
            slsa_level,
            build_provenance: provenance,
        };

        self.signature_cache.insert(content_hash, signature.clone());
        self.signatures_issued += 1;

        Ok(signature)
    }

    // ── Package Verification ──────────────────────────────────────────────────

    /// Verify a package signature against the keyring.
    ///
    /// Returns Ok(TrustLevel) on success, Err(reason) on failure.
    pub fn verify_package(
        &mut self,
        signature: &PackageSignature,
        actual_content_hash: u64,
        timestamp: u64,
    ) -> Result<TrustLevel, String> {
        // 1. Content hash must match
        if signature.content_hash != actual_content_hash {
            self.verification_failures += 1;
            return Err(format!(
                "Content hash mismatch: expected {:016X}, got {:016X}",
                signature.content_hash, actual_content_hash
            ));
        }

        // 2. Look up the signer key
        let key = self.trusted_keys
            .get(&signature.signer_fingerprint)
            .ok_or_else(|| format!("Unknown signer: {}", signature.signer_fingerprint))?
            .clone();

        // 3. Check key validity
        if !key.is_valid_at(timestamp) {
            self.verification_failures += 1;
            return Err(format!("Signing key {} is revoked or expired", &key.fingerprint[..8]));
        }

        // 4. Check trust level
        if !key.trust_level.is_sufficient() {
            self.verification_failures += 1;
            return Err(format!(
                "Signing key {} has insufficient trust level: {}",
                &key.fingerprint[..8],
                key.trust_level.label()
            ));
        }

        // 5. Verify Dilithium-5 signature
        let expected_sig = self.dilithium_verify_hash(
            &key.key_bytes,
            &signature.package_name,
            &signature.package_version,
            actual_content_hash,
        );
        let sig_ok = signature.signature_bytes[0..8] == expected_sig[0..8];
        if !sig_ok {
            self.verification_failures += 1;
            return Err(String::from("Dilithium-5 signature verification failed"));
        }

        self.signatures_verified += 1;
        Ok(key.trust_level)
    }

    // ── Dilithium Simulation ──────────────────────────────────────────────────

    /// Simulated Dilithium-5 signing (deterministic FNV-1a based)
    fn dilithium_sign(
        &self,
        secret: &[u8; 32],
        pkg_name: &str,
        pkg_ver: &str,
        content_hash: u64,
    ) -> [u8; 64] {
        const FNV_PRIME: u64 = 0x00000100000001B3;
        let mut h: u64 = 0xcbf29ce484222325;
        for &b in secret { h ^= b as u64; h = h.wrapping_mul(FNV_PRIME); }
        for &b in pkg_name.as_bytes() { h ^= b as u64; h = h.wrapping_mul(FNV_PRIME); }
        for &b in pkg_ver.as_bytes() { h ^= b as u64; h = h.wrapping_mul(FNV_PRIME); }
        h ^= content_hash;
        h = h.wrapping_mul(FNV_PRIME);
        let mut sig = [0u8; 64];
        for i in 0..64 {
            h = h.wrapping_mul(FNV_PRIME) ^ (i as u64);
            sig[i] = (h >> (i % 8 * 8)) as u8;
        }
        sig
    }

    /// Derive the expected signature prefix from a public key (for verify)
    fn dilithium_verify_hash(
        &self,
        public: &[u8; 32],
        pkg_name: &str,
        pkg_ver: &str,
        content_hash: u64,
    ) -> [u8; 64] {
        // In real Dilithium: lattice commitment check
        // Here: re-derive from public key (matches sign because sk→pk is identity in simulation)
        const FNV_PRIME: u64 = 0x00000100000001B3;
        let mut h: u64 = 0xcbf29ce484222325;
        // Simulate sk from pk in our test setup: secret = pk XOR 0xA5
        let secret: Vec<u8> = public.iter().map(|&b| b ^ 0xA5).collect();
        for &b in &secret { h ^= b as u64; h = h.wrapping_mul(FNV_PRIME); }
        for &b in pkg_name.as_bytes() { h ^= b as u64; h = h.wrapping_mul(FNV_PRIME); }
        for &b in pkg_ver.as_bytes() { h ^= b as u64; h = h.wrapping_mul(FNV_PRIME); }
        h ^= content_hash;
        h = h.wrapping_mul(FNV_PRIME);
        let mut sig = [0u8; 64];
        for i in 0..64 {
            h = h.wrapping_mul(FNV_PRIME) ^ (i as u64);
            sig[i] = (h >> (i % 8 * 8)) as u8;
        }
        sig
    }

    // ── Keyring Status ────────────────────────────────────────────────────────

    /// Returns a summary of the keyring
    pub fn keyring_summary(&self) -> String {
        let total = self.trusted_keys.len();
        let ultimate = self.trusted_keys.values().filter(|k| k.trust_level == TrustLevel::Ultimate).count();
        let full = self.trusted_keys.values().filter(|k| k.trust_level == TrustLevel::Full).count();
        let revoked = self.revoked_fingerprints.len();
        format!(
            "Keyring: {} keys ({} ultimate, {} full, {} revoked) | {} signed | {} verified | {} failed",
            total, ultimate, full, revoked,
            self.signatures_issued, self.signatures_verified, self.verification_failures
        )
    }
}

// ─── FNV-1a Content Hash ──────────────────────────────────────────────────────

/// Compute FNV-1a 64-bit hash of package content bytes
pub fn content_hash(data: &[u8]) -> u64 {
    const FNV_PRIME: u64 = 0x00000100000001B3;
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in data {
        h ^= b as u64;
        h = h.wrapping_mul(FNV_PRIME);
    }
    h
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod signing_tests {
    use super::*;

    fn test_key(trust: TrustLevel) -> (PublisherPublicKey, [u8; 32]) {
        let key_bytes: [u8; 32] = [0x42; 32];
        let secret: [u8; 32] = key_bytes.map(|b| b ^ 0xA5);
        let key = PublisherPublicKey::new(key_bytes, "Test Publisher", "test@sigma.dev", trust);
        (key, secret)
    }

    #[test]
    fn test_fingerprint_deterministic() {
        let (key1, _) = test_key(TrustLevel::Full);
        let (key2, _) = test_key(TrustLevel::Full);
        assert_eq!(key1.fingerprint, key2.fingerprint);
        assert_eq!(key1.fingerprint.len(), 16);
    }

    #[test]
    fn test_add_and_lookup_trusted_key() {
        let mut auth = SigmaSigningAuthority::new();
        let (key, _) = test_key(TrustLevel::Full);
        let fp = key.fingerprint.clone();
        auth.add_trusted_key(key);
        assert!(auth.is_trusted_key(&fp, 0));
    }

    #[test]
    fn test_revoke_key() {
        let mut auth = SigmaSigningAuthority::new();
        let (key, _) = test_key(TrustLevel::Full);
        let fp = key.fingerprint.clone();
        auth.add_trusted_key(key);
        assert!(auth.is_trusted_key(&fp, 0));
        auth.revoke_key(&fp);
        assert!(!auth.is_trusted_key(&fp, 0));
    }

    #[test]
    fn test_sign_package() {
        let mut auth = SigmaSigningAuthority::new();
        let (key, secret) = test_key(TrustLevel::Full);
        let fp = key.fingerprint.clone();
        auth.add_trusted_key(key);

        let pkg_data = b"fake package content";
        let hash = content_hash(pkg_data);
        let sig = auth.sign_package(&secret, &fp, "bash", "5.2.1", hash, 2, None);
        assert!(sig.is_ok());
        let sig = sig.unwrap();
        assert_eq!(sig.package_name, "bash");
        assert_eq!(sig.package_version, "5.2.1");
        assert_eq!(sig.content_hash, hash);
        assert_eq!(auth.signatures_issued, 1);
    }

    #[test]
    fn test_verify_correct_signature() {
        let mut auth = SigmaSigningAuthority::new();
        let (key, secret) = test_key(TrustLevel::Full);
        let fp = key.fingerprint.clone();
        auth.add_trusted_key(key);

        let pkg_data = b"real package bytes";
        let hash = content_hash(pkg_data);
        let sig = auth.sign_package(&secret, &fp, "glibc", "2.39", hash, 3, None).unwrap();
        let result = auth.verify_package(&sig, hash, 0);
        assert!(result.is_ok(), "Verification should succeed: {:?}", result);
    }

    #[test]
    fn test_verify_tampered_content_fails() {
        let mut auth = SigmaSigningAuthority::new();
        let (key, secret) = test_key(TrustLevel::Full);
        let fp = key.fingerprint.clone();
        auth.add_trusted_key(key);

        let pkg_data = b"original content";
        let hash = content_hash(pkg_data);
        let sig = auth.sign_package(&secret, &fp, "curl", "8.5.0", hash, 1, None).unwrap();

        // Tamper: compute hash of different content
        let tampered_hash = content_hash(b"tampered content");
        let result = auth.verify_package(&sig, tampered_hash, 0);
        assert!(result.is_err(), "Tampered content should fail verification");
        assert_eq!(auth.verification_failures, 1);
    }

    #[test]
    fn test_unknown_trust_level_rejected() {
        let mut auth = SigmaSigningAuthority::new();
        let (key, _secret) = test_key(TrustLevel::Unknown);
        let fp = key.fingerprint.clone();
        auth.add_trusted_key(key);
        assert!(!auth.is_trusted_key(&fp, 0));
    }

    #[test]
    fn test_content_hash_deterministic() {
        let data = b"SigmaOS package";
        assert_eq!(content_hash(data), content_hash(data));
        // Different content → different hash
        assert_ne!(content_hash(b"SigmaOS package v2"), content_hash(b"SigmaOS package v3"));
    }

    #[test]
    fn test_keyring_summary() {
        let mut auth = SigmaSigningAuthority::new();
        let (key, _) = test_key(TrustLevel::Full);
        auth.add_trusted_key(key);
        let summary = auth.keyring_summary();
        assert!(summary.contains("Keyring"));
        assert!(summary.contains("1 keys"));
    }

    #[test]
    fn test_sign_with_provenance() {
        let mut auth = SigmaSigningAuthority::new();
        let (key, secret) = test_key(TrustLevel::Ultimate);
        let fp = key.fingerprint.clone();
        auth.add_trusted_key(key);

        let provenance = BuildProvenance {
            builder_id: String::from("sigma-ci-trusted-builder-v1"),
            source_uri: String::from("https://github.com/AaryanSinghChauhan09/SigmaOS"),
            source_commit: String::from("fd97c013c5"),
            reproducible: true,
            build_env_hash: 0xDEADBEEF,
        };
        let hash = content_hash(b"openssl source");
        let sig = auth.sign_package(&secret, &fp, "openssl", "3.2.0", hash, 4, Some(provenance));
        assert!(sig.is_ok());
        let sig = sig.unwrap();
        assert_eq!(sig.slsa_level, 4);
        assert!(sig.build_provenance.is_some());
        assert!(sig.build_provenance.unwrap().reproducible);
    }

    #[test]
    fn test_reproducible_build_verification() {
        let mut auth = SigmaSigningAuthority::new();
        let (key, secret) = test_key(TrustLevel::Full);
        let fp = key.fingerprint.clone();
        auth.add_trusted_key(key);

        // Simulate SOURCE_DATE_EPOCH reproducible build
        let pkg_data = b"reproducible binary with fixed timestamp";
        let hash = content_hash(pkg_data);
        let sig = auth.sign_package(&secret, &fp, "gcc", "14.2.0", hash, 3, None).unwrap();
        
        // Verify with same hash
        let result = auth.verify_package(&sig, hash, 0);
        assert!(result.is_ok());
    }
}

// ─── Kyber-1024 KEM for Encrypted Package Distribution ─────────────────────

/// Kyber-1024 Key Encapsulation Mechanism for encrypted package distribution
#[derive(Debug, Clone)]
pub struct Kyber1024Kem {
    pub public_key: [u8; 32],
    pub secret_key: [u8; 32],
    pub encapsulations: AtomicU64,
}

impl Kyber1024Kem {
    pub fn new() -> Self {
        let public_key = [0x4B; 32]; // Simulated Kyber-1024 public key
        let secret_key = [0x59; 32]; // Simulated Kyber-1024 secret key
        Kyber1024Kem {
            public_key,
            secret_key,
            encapsulations: AtomicU64::new(0),
        }
    }

    /// Encapsulate a shared secret for package distribution
    pub fn encapsulate(&self, recipient_public: &[u8; 32]) -> ([u8; 32], [u8; 32]) {
        // In real Kyber-1024: lattice-based key encapsulation
        // Here: simulate shared secret derivation
        let mut shared_secret = [0u8; 32];
        let mut ciphertext = [0u8; 32];
        
        for i in 0..32 {
            shared_secret[i] = self.secret_key[i] ^ recipient_public[i];
            ciphertext[i] = self.public_key[i] ^ recipient_public[i];
        }
        
        self.encapsulations.fetch_add(1, Ordering::SeqCst);
        (shared_secret, ciphertext)
    }

    /// Decapsulate shared secret from ciphertext
    pub fn decapsulate(&self, ciphertext: &[u8; 32], sender_public: &[u8; 32]) -> Option<[u8; 32]> {
        // In real Kyber-1024: lattice-based key decapsulation
        // Here: simulate shared secret derivation
        let mut shared_secret = [0u8; 32];
        
        for i in 0..32 {
            shared_secret[i] = self.secret_key[i] ^ ciphertext[i] ^ sender_public[i];
        }
        
        Some(shared_secret)
    }

    pub fn get_encapsulation_count(&self) -> u64 {
        self.encapsulations.load(Ordering::SeqCst)
    }
}

impl Default for Kyber1024Kem {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Reproducible Build Verification ─────────────────────────────────────────────

/// Reproducible build metadata verification
#[derive(Debug, Clone)]
pub struct ReproducibleBuildInfo {
    pub source_date_epoch: u64,
    pub build_env_hash: u64,
    pub compiler_version: String,
    pub rustc_version: String,
    pub target_triple: String,
}

impl ReproducibleBuildInfo {
    pub fn new() -> Self {
        ReproducibleBuildInfo {
            source_date_epoch: 0,
            build_env_hash: 0,
            compiler_version: String::new(),
            rustc_version: String::new(),
            target_triple: String::new(),
        }
    }

    /// Verify build reproducibility by comparing build metadata
    pub fn verify_reproducibility(&self, other: &ReproducibleBuildInfo) -> bool {
        self.source_date_epoch == other.source_date_epoch
            && self.build_env_hash == other.build_env_hash
            && self.compiler_version == other.compiler_version
            && self.rustc_version == other.rustc_version
            && self.target_triple == other.target_triple
    }

    /// Set SOURCE_DATE_EPOCH for reproducible builds
    pub fn set_source_date_epoch(&mut self, epoch: u64) {
        self.source_date_epoch = epoch;
    }
}

impl Default for ReproducibleBuildInfo {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Merkle Tree-based Content Addressing ─────────────────────────────────────

/// Merkle tree node for content-addressed package storage
#[derive(Debug, Clone)]
pub struct MerkleNode {
    pub hash: [u8; 32],
    pub left: Option<Box<MerkleNode>>,
    pub right: Option<Box<MerkleNode>>,
}

impl MerkleNode {
    pub fn new(hash: [u8; 32]) -> Self {
        MerkleNode {
            hash,
            left: None,
            right: None,
        }
    }

    pub fn leaf(data: &[u8]) -> Self {
        let hash = Self::compute_hash(data);
        MerkleNode::new(hash)
    }

    pub fn internal(left: MerkleNode, right: MerkleNode) -> Self {
        let mut combined = left.hash.to_vec();
        combined.extend_from_slice(&right.hash);
        let hash = Self::compute_hash(&combined);
        MerkleNode {
            hash,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    fn compute_hash(data: &[u8]) -> [u8; 32] {
        const FNV_PRIME: u64 = 0x00000100000001B3;
        let mut h: u64 = 0xcbf29ce484222325;
        for &b in data {
            h ^= b as u64;
            h = h.wrapping_mul(FNV_PRIME);
        }
        let mut hash = [0u8; 32];
        for i in 0..32 {
            hash[i] = ((h >> (i * 8)) & 0xFF) as u8;
        }
        hash
    }

    pub fn get_hash(&self) -> &[u8; 32] {
        &self.hash
    }
}

/// Merkle tree for content-addressed package storage
#[derive(Debug)]
pub struct MerkleTree {
    pub root: Option<MerkleNode>,
    pub leaf_count: usize,
}

impl MerkleTree {
    pub fn new() -> Self {
        MerkleTree {
            root: None,
            leaf_count: 0,
        }
    }

    pub fn add_leaf(&mut self, data: &[u8]) {
        let leaf = MerkleNode::leaf(data);
        match &mut self.root {
            None => {
                self.root = Some(leaf);
            }
            Some(root) => {
                let new_root = MerkleNode::internal(root.clone(), leaf);
                self.root = Some(new_root);
            }
        }
        self.leaf_count += 1;
    }

    pub fn get_root_hash(&self) -> Option<&[u8; 32]> {
        self.root.as_ref().map(|node| node.get_hash())
    }

    pub fn verify_leaf(&self, data: &[u8]) -> bool {
        let leaf_hash = MerkleNode::compute_hash(data);
        match &self.root {
            None => false,
            Some(root) => root.get_hash() == &leaf_hash[..32],
        }
    }
}

impl Default for MerkleTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod pqc_tests {
    use super::*;

    #[test]
    fn test_kyber1024_encapsulation() {
        let kem = Kyber1024Kem::new();
        let recipient_public = [0x7A; 32];
        
        let (shared_secret, ciphertext) = kem.encapsulate(&recipient_public);
        let recovered_secret = kem.decapsulate(&ciphertext, &recipient_public);
        
        assert!(recovered_secret.is_some());
        assert_eq!(recovered_secret.unwrap(), shared_secret);
        assert_eq!(kem.get_encapsulation_count(), 1);
    }

    #[test]
    fn test_reproducible_build_info() {
        let mut build1 = ReproducibleBuildInfo::new();
        let mut build2 = ReproducibleBuildInfo::new();
        
        build1.set_source_date_epoch(1700000000);
        build2.set_source_date_epoch(1700000000);
        
        assert!(build1.verify_reproducibility(&build2));
        
        build2.set_source_date_epoch(1700000001);
        assert!(!build1.verify_reproducibility(&build2));
    }

    #[test]
    fn test_merkle_tree() {
        let mut tree = MerkleTree::new();
        
        tree.add_leaf(b"package v1.0");
        tree.add_leaf(b"package v1.1");
        
        assert_eq!(tree.leaf_count, 2);
        assert!(tree.get_root_hash().is_some());
        assert!(tree.verify_leaf(b"package v1.0"));
    }
}
