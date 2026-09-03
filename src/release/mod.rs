// SigmaOS Release Engineering & Release Cadence Infrastructure
// Inspired by Debian's stable/testing/unstable branches, OpenBSD's 6-month release cadence,
// GPG/Dilithium-5 signed release tags, reproducible build hash publication, and errata advisory distribution.

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseCadence {
    Stable,
    Testing,
    Unstable,
}

#[derive(Debug, Clone)]
pub struct ReleaseTag {
    pub version_tag: String,             // e.g. "v1.0.0-release"
    pub release_branch: String,          // e.g. "release/v1.0"
    pub git_commit_sha: String,          // Git commit hash
    pub gpg_dilithium_signature: String, // Dilithium-5 / GPG signed commit
    pub reproducible_hash: String,       // Published reproducible build SHA256
    pub timestamp_sec: u64,
    pub cadence: ReleaseCadence,
}

#[derive(Debug, Clone)]
pub struct ErrataAdvisory {
    pub advisory_id: String,          // e.g. "SIGMA-ERRATA-2025-001"
    pub affected_subsystem: String,   // e.g. "kernel/mm"
    pub severity: String,             // e.g. "CRITICAL", "HIGH"
    pub patch_commit_sha: String,     // Fix commit SHA
    pub description: String,
}

#[derive(Debug, Default)]
pub struct ReleaseEngineeringEngine {
    pub active_branch: String,
    pub release_tags: BTreeMap<String, ReleaseTag>,
    pub errata_advisories: Vec<ErrataAdvisory>,
}

impl ReleaseEngineeringEngine {
    pub fn new(active_branch: &str) -> Self {
        Self {
            active_branch: active_branch.to_string(),
            release_tags: BTreeMap::new(),
            errata_advisories: Vec::new(),
        }
    }

    pub fn cut_release_branch(
        &mut self,
        version: &str,
        commit_sha: &str,
        pqc_sig: &str,
        repro_hash: &str,
    ) -> Result<ReleaseTag, &'static str> {
        if version.is_empty() || commit_sha.is_empty() {
            return Err("ReleaseEngineering: Version and commit SHA cannot be empty");
        }

        if !pqc_sig.starts_with("dilithium5:") && !pqc_sig.starts_with("gpg:") {
            return Err("ReleaseEngineering: Release tag must be signed with GPG or Dilithium-5 signature");
        }

        let branch_name = format!("release/{}", version.trim_start_matches('v'));
        let tag = ReleaseTag {
            version_tag: version.to_string(),
            release_branch: branch_name,
            git_commit_sha: commit_sha.to_string(),
            gpg_dilithium_signature: pqc_sig.to_string(),
            reproducible_hash: repro_hash.to_string(),
            timestamp_sec: 1741000000,
            cadence: ReleaseCadence::Stable,
        };

        self.release_tags.insert(version.to_string(), tag.clone());
        Ok(tag)
    }

    pub fn publish_errata_advisory(
        &mut self,
        advisory_id: &str,
        subsystem: &str,
        severity: &str,
        patch_sha: &str,
        description: &str,
    ) -> Result<(), &'static str> {
        if advisory_id.is_empty() || patch_sha.is_empty() {
            return Err("ReleaseEngineering: Advisory ID and patch SHA cannot be empty");
        }

        let errata = ErrataAdvisory {
            advisory_id: advisory_id.to_string(),
            affected_subsystem: subsystem.to_string(),
            severity: severity.to_string(),
            patch_commit_sha: patch_sha.to_string(),
            description: description.to_string(),
        };

        self.errata_advisories.push(errata);
        Ok(())
    }

    pub fn verify_reproducible_build_hash(&self, version: &str, computed_hash: &str) -> bool {
        if let Some(tag) = self.release_tags.get(version) {
            tag.reproducible_hash == computed_hash
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_release_engineering_engine() {
        let mut engine = ReleaseEngineeringEngine::new("main");

        let version = "v1.0.0";
        let commit = "9a8b7c6d5e4f3a2b1c0d";
        let sig = "dilithium5:sig_release_tag_v1.0_signed";
        let hash = "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let tag = engine.cut_release_branch(version, commit, sig, hash).unwrap();
        assert_eq!(tag.release_branch, "release/1.0.0");
        assert_eq!(tag.cadence, ReleaseCadence::Stable);

        assert!(engine.verify_reproducible_build_hash(version, hash));
        assert!(!engine.verify_reproducible_build_hash(version, "sha256:tampered_hash"));

        // Errata advisory
        assert!(engine
            .publish_errata_advisory(
                "SIGMA-ERRATA-2025-001",
                "kernel/mm",
                "HIGH",
                "1a2b3c4d5e",
                "Fix buddy allocator page order bounds check panic"
            )
            .is_ok());
        assert_eq!(engine.errata_advisories.len(), 1);
    }
}
