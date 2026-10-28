// SPDX-License-Identifier: GPL-2.0-or-later
// LibertyBSD libre-only firmware policy for SigmaOS.
// Audits firmware blobs to enforce free/libre-only policy,
// rejects non-free blobs, and supports per-blob whitelisting.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// A firmware blob descriptor with a 256-bit (SHA-256) hash.
#[derive(Debug, Clone)]
pub struct FirmwareBlob {
    pub name: String,
    pub hash: [u8; 32],
    pub is_libre: bool,
}

impl FirmwareBlob {
    pub fn new(name: &str, hash: [u8; 32], is_libre: bool) -> Self {
        Self {
            name: name.to_string(),
            hash,
            is_libre,
        }
    }
}

/// Audits all registered firmware blobs for libre compliance.
pub struct FirmwareAuditEngine {
    pub blobs: Vec<FirmwareBlob>,
}

impl FirmwareAuditEngine {
    pub fn new() -> Self {
        Self { blobs: Vec::new() }
    }

    pub fn register_blob(&mut self, blob: FirmwareBlob) {
        self.blobs.push(blob);
    }

    /// Returns a list of non-libre blob names.
    pub fn audit_all(&self) -> Vec<&str> {
        self.blobs
            .iter()
            .filter(|b| !b.is_libre)
            .map(|b| b.name.as_str())
            .collect()
    }

    /// Remove all non-libre blobs from the registry.
    pub fn reject_nonfree(&mut self) {
        self.blobs.retain(|b| b.is_libre);
    }

    /// Return only libre blobs.
    pub fn get_libre_blobs(&self) -> Vec<&FirmwareBlob> {
        self.blobs.iter().filter(|b| b.is_libre).collect()
    }
}

impl Default for FirmwareAuditEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Libre-only boot policy with optional strict mode.
pub struct LibrePolicy {
    pub strict_mode: bool,
    pub whitelist: Vec<[u8; 32]>,
}

impl LibrePolicy {
    pub fn new() -> Self {
        Self {
            strict_mode: false,
            whitelist: Vec::new(),
        }
    }

    pub fn enforce_strict_mode(&mut self) {
        self.strict_mode = true;
    }

    /// Add a blob hash to the allow-list (even in strict mode).
    pub fn whitelist_blob(&mut self, hash: [u8; 32]) {
        self.whitelist.push(hash);
    }

    /// Returns true if the blob is allowed under the current policy.
    pub fn is_allowed(&self, blob: &FirmwareBlob) -> bool {
        if blob.is_libre {
            return true;
        }
        if self.strict_mode {
            // In strict mode, non-libre blobs are only allowed if whitelisted.
            return self.whitelist.iter().any(|h| h == &blob.hash);
        }
        // Non-strict: non-libre blobs pass.
        true
    }
}

impl Default for LibrePolicy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_hash(seed: u8) -> [u8; 32] {
        [seed; 32]
    }

    #[test]
    fn test_nonfree_blob_rejected_in_strict_mode() {
        let mut policy = LibrePolicy::new();
        policy.enforce_strict_mode();

        let nonfree =
            FirmwareBlob::new("iwlwifi-9260-th-b0-jf-b0-46.ucode", make_hash(0xAB), false);
        assert!(!policy.is_allowed(&nonfree));
    }

    #[test]
    fn test_libre_blob_always_allowed() {
        let mut policy = LibrePolicy::new();
        policy.enforce_strict_mode();
        let libre = FirmwareBlob::new("open-ath9k.fw", make_hash(0x01), true);
        assert!(policy.is_allowed(&libre));
    }

    #[test]
    fn test_audit_engine_reject_nonfree() {
        let mut engine = FirmwareAuditEngine::new();
        engine.register_blob(FirmwareBlob::new("libre.fw", make_hash(0x00), true));
        engine.register_blob(FirmwareBlob::new("nonfree.fw", make_hash(0xFF), false));
        assert_eq!(engine.audit_all().len(), 1);
        engine.reject_nonfree();
        assert_eq!(engine.blobs.len(), 1);
        assert!(engine.blobs[0].is_libre);
    }

    #[test]
    fn test_whitelisted_hash_allowed_in_strict() {
        let hash = make_hash(0x55);
        let mut policy = LibrePolicy::new();
        policy.enforce_strict_mode();
        policy.whitelist_blob(hash);
        let blob = FirmwareBlob::new("special.fw", hash, false);
        assert!(policy.is_allowed(&blob));
    }
}
