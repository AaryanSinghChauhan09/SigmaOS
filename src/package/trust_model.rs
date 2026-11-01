// SPDX-License-Identifier: MIT
// SigmaOS Package Signing & Repository Trust Model
// (`src/package/trust_model.rs`)
//
// Provides cryptographic package-signature verification and repository
// trust management for SigmaOS.  The trust model mirrors the approach used by
// Debian (GPG-signed Release files), Arch Linux (pacman-key), and RPM
// (GPG-signed RPMs), while adding first-class support for Ed25519 and
// revocation awareness.
//
// Design notes:
//  - `PackageTrustManager` is the single entry point; construct with `new()`.
//  - The manager holds its state internally; it is NOT `Send`/`Sync` by
//    default — wrap in `Arc<Mutex<…>>` if shared across threads.
//  - In production the `verify_*` methods must be backed by a real crypto
//    library (e.g. `ring`, `ed25519-dalek`).  The current implementation
//    returns `Ok` stubs for testing and documents every hook site.

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

use thiserror::Error;

// ─────────────────────────────────────────────────────────────────────────────
// Error type
// ─────────────────────────────────────────────────────────────────────────────

/// All errors that can surface from the package trust subsystem.
#[derive(Debug, Error)]
pub enum TrustError {
    /// The cryptographic signature did not verify against the payload.
    #[error("Signature is invalid for '{path}'")]
    SignatureInvalid { path: String },

    /// No signing key matching the package's key-ID was found in the keyring.
    #[error("Signing key '{key_id}' not found in keyring")]
    KeyNotFound { key_id: String },

    /// The signing key has passed its expiry date.
    #[error("Signing key '{fingerprint}' has expired")]
    KeyExpired { fingerprint: String },

    /// The signing key has been explicitly revoked.
    #[error("Signing key '{fingerprint}' has been revoked")]
    KeyRevoked { fingerprint: String },

    /// The repository is not in the trusted-repos list.
    #[error("Repository '{url}' is not trusted")]
    RepoNotTrusted { url: String },

    /// A generic, unrecoverable verification failure.
    #[error("Verification failed: {reason}")]
    VerificationFailed { reason: String },
}

// ─────────────────────────────────────────────────────────────────────────────
// Enumerations
// ─────────────────────────────────────────────────────────────────────────────

/// Overall trust verdict for a signing key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyTrust {
    /// Key is valid, not expired, not revoked, and signed by a trusted root.
    Trusted,
    /// Key is not yet verified or belongs to an unknown signer.
    Untrusted,
    /// Key's validity period has elapsed.
    Expired,
    /// Key has been explicitly revoked by the SigmaOS security team.
    Revoked,
    /// Trust status could not be determined (network unavailable, etc.).
    Unknown,
}

/// Cryptographic algorithm used to produce the package signature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SignatureAlgorithm {
    /// Ed25519 — preferred for new SigmaOS packages.
    Ed25519,
    /// RSA-4096 — accepted for legacy and third-party repos.
    Rsa4096,
    /// RSA-2048 — accepted with a deprecation warning.
    Rsa2048,
}

/// Relative trust level assigned to a repository.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TrustLevel {
    /// First-party SigmaOS repositories — strongest guarantees.
    Official,
    /// Verified community repositories with a known maintainer.
    Community,
    /// User-added third-party repositories — treated with caution.
    ThirdParty,
    /// Repository is explicitly distrusted or blacklisted.
    Untrusted,
}

// ─────────────────────────────────────────────────────────────────────────────
// Structs
// ─────────────────────────────────────────────────────────────────────────────

/// The decoded signature attached to a package or repository metadata file.
#[derive(Debug, Clone)]
pub struct PackageSignature {
    /// Hex-encoded full fingerprint of the signing key (e.g. 40-char SHA-1 or
    /// 64-char SHA-256 depending on the algorithm).
    pub fingerprint: String,

    /// Short key identifier (typically the last 16 hex chars of the
    /// fingerprint) used for fast keyring lookups.
    pub key_id: String,

    /// Human-readable identity of the signer (name and/or e-mail address).
    pub signer: String,

    /// Unix timestamp (seconds since epoch) when the signature was created.
    pub signed_at: u64,

    /// Cryptographic algorithm used.
    pub algorithm: SignatureAlgorithm,

    /// Resolved trust verdict after keyring and revocation list checks.
    pub trust: KeyTrust,
}

/// Trust record for a repository.
#[derive(Debug, Clone)]
pub struct RepoTrust {
    /// Base URL of the repository (e.g. `https://packages.sigmaos.io/main`).
    pub url: String,

    /// Hex-encoded fingerprint of the repository signing key.
    pub fingerprint: String,

    /// Assigned trust level.
    pub trust_level: TrustLevel,

    /// Unix timestamp when the trust record was last verified against an
    /// online certificate or key server.
    pub last_verified: u64,
}

// ─────────────────────────────────────────────────────────────────────────────
// PackageTrustManager
// ─────────────────────────────────────────────────────────────────────────────

/// Manages the package keyring, repository trust list, and revocation records.
///
/// # Example
/// ```rust,no_run
/// use crate::package::trust_model::PackageTrustManager;
///
/// let mut mgr = PackageTrustManager::new();
/// assert!(!mgr.official_repos().is_empty());
/// ```
pub struct PackageTrustManager {
    /// Trusted repositories indexed by URL.
    trusted_repos: BTreeMap<String, RepoTrust>,

    /// Set of explicitly revoked key fingerprints.
    revoked_keys: BTreeMap<String, String>, // fingerprint → reason
}

impl PackageTrustManager {
    /// Creates a new manager pre-populated with the official SigmaOS
    /// repositories.
    pub fn new() -> Self {
        let mut mgr = Self {
            trusted_repos: BTreeMap::new(),
            revoked_keys: BTreeMap::new(),
        };

        // Pre-populate with official repos at construction time so that the
        // manager is immediately usable without any additional setup.
        for repo in Self::official_repos_static() {
            mgr.trusted_repos.insert(repo.url.clone(), repo);
        }
        mgr
    }

    // ── Core verification methods ────────────────────────────────────────────

    /// Verifies the detached signature on a local package file.
    ///
    /// The implementation **must** be replaced with a call to a real crypto
    /// library before the package manager goes into production.
    ///
    /// # Errors
    /// Returns `TrustError::SignatureInvalid` if the signature bytes do not
    /// match, `TrustError::KeyRevoked` if the key has been revoked, or
    /// `TrustError::KeyExpired` if the key is past its validity period.
    pub fn verify_package_signature(
        &self,
        package_path: &str,
    ) -> Result<PackageSignature, TrustError> {
        if package_path.is_empty() {
            return Err(TrustError::SignatureInvalid {
                path: package_path.to_string(),
            });
        }

        // TODO: replace with real signature verification (e.g. ring, ed25519-dalek).
        // 1. Read the .sig file alongside package_path.
        // 2. Decode the signing key fingerprint from the signature.
        // 3. Reject if fingerprint is in self.revoked_keys.
        // 4. Look up the key in the local keyring.
        // 5. Verify the signature cryptographically.

        // Stub: derive a deterministic fake fingerprint from the path.
        let stub_fingerprint = format!("STUB_{:040x}", package_path.len());
        let stub_key_id = stub_fingerprint[stub_fingerprint.len() - 16..].to_string();

        // Fast-fail if the key is revoked.
        if self.revoked_keys.contains_key(&stub_fingerprint) {
            return Err(TrustError::KeyRevoked {
                fingerprint: stub_fingerprint,
            });
        }

        Ok(PackageSignature {
            fingerprint: stub_fingerprint,
            key_id: stub_key_id,
            signer: "SigmaOS Build System <build@sigmaos.io>".to_string(),
            signed_at: 0, // TODO: parse from signature header
            algorithm: SignatureAlgorithm::Ed25519,
            trust: KeyTrust::Trusted,
        })
    }

    /// Verifies repository metadata against its detached signature.
    ///
    /// Returns `Ok(true)` when the signature is valid and the repo is trusted,
    /// `Ok(false)` when the signature is valid but the repo is unknown/untrusted,
    /// and `Err(TrustError)` on a hard failure.
    pub fn verify_repo_metadata(
        &self,
        repo_url: &str,
        metadata: &[u8],
        sig: &[u8],
    ) -> Result<bool, TrustError> {
        if metadata.is_empty() || sig.is_empty() {
            return Err(TrustError::VerificationFailed {
                reason: "metadata or signature bytes are empty".to_string(),
            });
        }

        // TODO: cryptographically verify `sig` over `metadata` using the repo
        // key fetched from trusted_repos[repo_url].fingerprint.

        let is_trusted = self.trusted_repos.contains_key(repo_url);
        Ok(is_trusted)
    }

    /// Adds a new repository to the trusted list.
    ///
    /// Overwrites any existing record for the same URL.
    ///
    /// # Errors
    /// Returns `TrustError::RepoNotTrusted` if the repo's `trust_level` is
    /// `TrustLevel::Untrusted`.
    pub fn add_trusted_repo(&mut self, repo: RepoTrust) -> Result<(), TrustError> {
        if repo.trust_level == TrustLevel::Untrusted {
            return Err(TrustError::RepoNotTrusted { url: repo.url });
        }
        self.trusted_repos.insert(repo.url.clone(), repo);
        Ok(())
    }

    /// Marks a key fingerprint as revoked.
    ///
    /// Any package whose signature resolves to this fingerprint will be
    /// rejected from this point forward.
    pub fn revoke_key(&mut self, fingerprint: &str) -> Result<(), TrustError> {
        if fingerprint.is_empty() {
            return Err(TrustError::KeyNotFound {
                key_id: fingerprint.to_string(),
            });
        }
        self.revoked_keys.insert(
            fingerprint.to_string(),
            "Revoked by SigmaOS security team".to_string(),
        );
        Ok(())
    }

    /// Returns `true` if the package at `package_path` carries a valid,
    /// non-revoked signature from a trusted key.
    ///
    /// This is a convenience wrapper that swallows verification errors and
    /// treats them as "not trusted".
    pub fn is_package_trusted(&self, package_path: &str) -> bool {
        match self.verify_package_signature(package_path) {
            Ok(sig) => sig.trust == KeyTrust::Trusted,
            Err(_) => false,
        }
    }

    /// Returns the current list of trusted repositories.
    pub fn list_trusted_repos(&self) -> Vec<&RepoTrust> {
        self.trusted_repos.values().collect()
    }

    // ── Official repository seed data ────────────────────────────────────────

    /// Returns the built-in list of official SigmaOS repositories.
    ///
    /// This is called once at construction and can also be used to restore
    /// the official repo list after local edits.
    pub fn official_repos() -> Vec<RepoTrust> {
        Self::official_repos_static()
    }

    fn official_repos_static() -> Vec<RepoTrust> {
        alloc::vec![
            // Main repository — stable, vetted packages.
            RepoTrust {
                url: "https://packages.sigmaos.io/main".to_string(),
                fingerprint: "A1B2C3D4E5F6A7B8C9D0E1F2A3B4C5D6E7F8A9B0".to_string(),
                trust_level: TrustLevel::Official,
                last_verified: 0,
            },
            // Security repository — CVE patches, released outside of the normal
            // release cycle.
            RepoTrust {
                url: "https://packages.sigmaos.io/security".to_string(),
                fingerprint: "B2C3D4E5F6A7B8C9D0E1F2A3B4C5D6E7F8A9B0A1".to_string(),
                trust_level: TrustLevel::Official,
                last_verified: 0,
            },
            // Updates repository — point-release updates to stable packages.
            RepoTrust {
                url: "https://packages.sigmaos.io/updates".to_string(),
                fingerprint: "C3D4E5F6A7B8C9D0E1F2A3B4C5D6E7F8A9B0A1B2".to_string(),
                trust_level: TrustLevel::Official,
                last_verified: 0,
            },
            // Contrib repository — community-maintained, reviewed by SigmaOS team.
            RepoTrust {
                url: "https://packages.sigmaos.io/contrib".to_string(),
                fingerprint: "D4E5F6A7B8C9D0E1F2A3B4C5D6E7F8A9B0A1B2C3".to_string(),
                trust_level: TrustLevel::Community,
                last_verified: 0,
            },
        ]
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Helpers ──────────────────────────────────────────────────────────────

    fn make_manager() -> PackageTrustManager {
        PackageTrustManager::new()
    }

    // ── official_repos ───────────────────────────────────────────────────────

    #[test]
    fn official_repos_non_empty() {
        let repos = PackageTrustManager::official_repos();
        assert!(
            !repos.is_empty(),
            "official_repos() must return at least one entry"
        );
    }

    #[test]
    fn official_repos_all_official_or_community() {
        for repo in PackageTrustManager::official_repos() {
            assert!(
                matches!(repo.trust_level, TrustLevel::Official | TrustLevel::Community),
                "repo '{}' must be Official or Community",
                repo.url
            );
        }
    }

    #[test]
    fn manager_pre_populated_with_official_repos() {
        let mgr = make_manager();
        let trusted = mgr.list_trusted_repos();
        assert_eq!(
            trusted.len(),
            PackageTrustManager::official_repos().len(),
            "manager must start with exactly the official repos"
        );
    }

    // ── add_trusted_repo ─────────────────────────────────────────────────────

    #[test]
    fn add_trusted_repo_succeeds_for_community() {
        let mut mgr = make_manager();
        let repo = RepoTrust {
            url: "https://community.example.com/packages".to_string(),
            fingerprint: "DEADBEEFDEADBEEFDEADBEEFDEADBEEF00000001".to_string(),
            trust_level: TrustLevel::Community,
            last_verified: 1_700_000_000,
        };
        mgr.add_trusted_repo(repo).expect("community repo should be accepted");
    }

    #[test]
    fn add_trusted_repo_rejects_untrusted() {
        let mut mgr = make_manager();
        let repo = RepoTrust {
            url: "https://sketchy.example.com/packages".to_string(),
            fingerprint: "CAFEBABECAFEBABECAFEBABECAFEBABE00000002".to_string(),
            trust_level: TrustLevel::Untrusted,
            last_verified: 0,
        };
        let result = mgr.add_trusted_repo(repo);
        assert!(result.is_err(), "Untrusted repo must be rejected");
    }

    // ── revoke_key ───────────────────────────────────────────────────────────

    #[test]
    fn revoke_key_changes_package_trust() {
        let mut mgr = make_manager();

        // Verify a package — should succeed (stub returns Trusted).
        let path = "sigma-browser_1.0.0_amd64.deb";
        let sig_before = mgr
            .verify_package_signature(path)
            .expect("initial verification must succeed");
        assert_eq!(sig_before.trust, KeyTrust::Trusted);

        // Revoke the key that the stub derives from this path.
        let stub_fingerprint = format!("STUB_{:040x}", path.len());
        mgr.revoke_key(&stub_fingerprint)
            .expect("revoke_key must succeed for valid fingerprint");

        // Now the same package must be rejected.
        let result = mgr.verify_package_signature(path);
        assert!(result.is_err(), "revoked key must cause verification failure");
        match result.unwrap_err() {
            TrustError::KeyRevoked { fingerprint } => {
                assert_eq!(fingerprint, stub_fingerprint);
            }
            other => panic!("unexpected error: {:?}", other),
        }
    }

    #[test]
    fn revoke_key_empty_fingerprint_returns_error() {
        let mut mgr = make_manager();
        let result = mgr.revoke_key("");
        assert!(result.is_err(), "empty fingerprint must return error");
    }

    // ── is_package_trusted ───────────────────────────────────────────────────

    #[test]
    fn is_package_trusted_true_for_valid_path() {
        let mgr = make_manager();
        assert!(mgr.is_package_trusted("some-package_1.0_amd64.deb"));
    }

    #[test]
    fn is_package_trusted_false_for_empty_path() {
        let mgr = make_manager();
        assert!(!mgr.is_package_trusted(""));
    }

    // ── verify_repo_metadata ─────────────────────────────────────────────────

    #[test]
    fn verify_repo_metadata_true_for_official_url() {
        let mgr = make_manager();
        let result = mgr.verify_repo_metadata(
            "https://packages.sigmaos.io/main",
            b"Release: 1.0\nDate: 2026-01-01",
            b"fakesig",
        );
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn verify_repo_metadata_false_for_unknown_url() {
        let mgr = make_manager();
        let result = mgr.verify_repo_metadata(
            "https://unknown.example.com/repo",
            b"Release: 1.0",
            b"fakesig",
        );
        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn verify_repo_metadata_error_on_empty_bytes() {
        let mgr = make_manager();
        let result = mgr.verify_repo_metadata(
            "https://packages.sigmaos.io/main",
            b"",
            b"",
        );
        assert!(result.is_err());
    }

    // ── trust level ordering ──────────────────────────────────────────────────

    #[test]
    fn trust_level_ordering_official_stronger_than_community() {
        assert!(TrustLevel::Official < TrustLevel::Community);
        assert!(TrustLevel::Community < TrustLevel::ThirdParty);
        assert!(TrustLevel::ThirdParty < TrustLevel::Untrusted);
    }
}
