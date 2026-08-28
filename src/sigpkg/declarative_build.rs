//! Pure Declarative Build System (Nix & Bazel Inspired) with Curated Ratings/Reviews Subsystem
//! Implements deterministic build derivations, hermetic dependency graphs, and package reputation validation.
extern crate alloc;

use crate::klib::collections::HashMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// ==========================================
// 1. Nix-Style Store Derivations
// ==========================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NixDerivation {
    pub name: String,
    pub builder: String,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
    pub input_sources_hashes: Vec<String>, // cryptographic hashes of source assets
}

impl NixDerivation {
    pub fn new(name: &str, builder: &str) -> Self {
        Self {
            name: name.to_string(),
            builder: builder.to_string(),
            args: Vec::new(),
            env: HashMap::new(),
            input_sources_hashes: Vec::new(),
        }
    }

    /// Computes the unique deterministic content-addressed store path for the output package.
    pub fn compute_store_path(&self) -> String {
        let mut hash_input = format!("{}-{}-{}", self.name, self.builder, self.args.join(","));
        for (k, v) in &self.env {
            hash_input.push_str(&format!(";{}={}", k, v));
        }
        for hash in &self.input_sources_hashes {
            hash_input.push_str(&format!(";src={}", hash));
        }

        // Simulating SHA-256 cryptographic hash of derivation content
        let mut hash_val = 5381u64;
        for byte in hash_input.bytes() {
            hash_val = hash_val.wrapping_mul(33).wrapping_add(byte as u64);
        }
        format!("/nix/store/{:016x}-{}", hash_val, self.name)
    }
}

// ==========================================
// 2. Bazel-Style Target Rules
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BazelRuleType {
    CcLibrary,
    CcBinary,
    RustLibrary,
    RustBinary,
}

#[derive(Debug, Clone)]
pub struct BazelTarget {
    pub label: String,
    pub rule_type: BazelRuleType,
    pub srcs: Vec<String>,
    pub deps: Vec<String>,
    pub outputs: Vec<String>,
}

pub struct BazelBuildEngine {
    pub action_cache: HashMap<String, String>, // Action ID -> Output hash (for hermeticity)
}

impl BazelBuildEngine {
    pub fn new() -> Self {
        Self {
            action_cache: HashMap::new(),
        }
    }

    /// Calculates cache key representing exact input sources and dependency versions
    pub fn calculate_target_cache_key(&self, target: &BazelTarget) -> String {
        let mut key = format!("{}-{:?}", target.label, target.rule_type);
        key.push_str(&target.srcs.join(","));
        key.push_str(&target.deps.join(","));
        key
    }

    /// Compiles or returns cached output paths for a Bazel target (hermetic incremental builds)
    pub fn build_target(&mut self, target: &BazelTarget) -> (String, bool) {
        let cache_key = self.calculate_target_cache_key(target);
        if let Some(cached_output) = self.action_cache.get(&cache_key) {
            (cached_output.clone(), true) // Cache Hit!
        } else {
            // Simulated action execution compiling sources to outputs
            let generated_hash = format!("bazel-out/{:016x}", 42);
            self.action_cache.insert(cache_key, generated_hash.clone());
            (generated_hash, false) // Cache Miss
        }
    }
}

impl Default for BazelBuildEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 3. Decentralized Ratings & Reviews Subsystem
// ==========================================

#[derive(Debug, Clone)]
pub struct PackageReview {
    pub reviewer: String,
    pub score: u32, // 1 to 5 stars
    pub comment: String,
    pub timestamp_ms: u64,
}

pub struct PackageRatingsRegistry {
    pub store_reviews: HashMap<String, Vec<PackageReview>>, // StorePath -> Reviews list
}

impl PackageRatingsRegistry {
    pub fn new() -> Self {
        Self {
            store_reviews: HashMap::new(),
        }
    }

    /// Adds a peer review score for a deterministic store output path
    pub fn submit_review(
        &mut self,
        store_path: &str,
        review: PackageReview,
    ) -> Result<(), &'static str> {
        if review.score < 1 || review.score > 5 {
            return Err("Score must be between 1 and 5 stars");
        }
        let entry = self
            .store_reviews
            .entry(store_path.to_string())
            .or_default();
        entry.push(review);
        Ok(())
    }

    /// Returns aggregated review statistics (average score, count)
    pub fn get_aggregate_rating(&self, store_path: &str) -> Option<(f32, usize)> {
        let reviews = self.store_reviews.get(store_path)?;
        if reviews.is_empty() {
            return None;
        }
        let total_score: u32 = reviews.iter().map(|r| r.score).sum();
        let average = total_score as f32 / reviews.len() as f32;
        Some((average, reviews.len()))
    }
}

impl Default for PackageRatingsRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 4. Tests Module
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nix_derivation_deterministic_store_paths() {
        let mut deriv1 = NixDerivation::new("nginx", "bash");
        deriv1.env.insert("VERSION".to_string(), "1.25".to_string());
        deriv1.args.push("-c".to_string());
        deriv1.args.push("make build".to_string());
        deriv1
            .input_sources_hashes
            .push("sha256-abc123xyz".to_string());

        let mut deriv2 = NixDerivation::new("nginx", "bash");
        deriv2.env.insert("VERSION".to_string(), "1.25".to_string());
        deriv2.args.push("-c".to_string());
        deriv2.args.push("make build".to_string());
        deriv2
            .input_sources_hashes
            .push("sha256-abc123xyz".to_string());

        // Same derivation inputs -> identical output store paths (Pure reproducibility)
        assert_eq!(deriv1.compute_store_path(), deriv2.compute_store_path());

        // Modified inputs -> different store path
        deriv2.env.insert("VERSION".to_string(), "1.26".to_string());
        assert_ne!(deriv1.compute_store_path(), deriv2.compute_store_path());
    }

    #[test]
    fn test_bazel_build_hermetic_cache() {
        let mut engine = BazelBuildEngine::new();
        let mut srcs = Vec::new();
        srcs.push("main.rs".to_string());

        let target = BazelTarget {
            label: "//src/main:app".to_string(),
            rule_type: BazelRuleType::RustBinary,
            srcs,
            deps: Vec::new(),
            outputs: Vec::new(),
        };

        // First build -> Cache Miss
        let (out1, is_cached1) = engine.build_target(&target);
        assert!(!is_cached1);

        // Second build -> Cache Hit!
        let (out2, is_cached2) = engine.build_target(&target);
        assert!(is_cached2);
        assert_eq!(out1, out2);
    }

    #[test]
    fn test_package_reviews_and_ratings() {
        let mut registry = PackageRatingsRegistry::new();
        let store_path = "/nix/store/1abc789-git-2.40";

        assert_eq!(registry.get_aggregate_rating(store_path), None);

        let rev1 = PackageReview {
            reviewer: "curator_alpha".to_string(),
            score: 5,
            comment: "Extremely stable, passed security audit".to_string(),
            timestamp_ms: 1000,
        };
        let rev2 = PackageReview {
            reviewer: "developer_beta".to_string(),
            score: 4,
            comment: "Solid compile times".to_string(),
            timestamp_ms: 2000,
        };

        assert!(registry.submit_review(store_path, rev1).is_ok());
        assert!(registry.submit_review(store_path, rev2).is_ok());

        // Invalid score check
        let bad_rev = PackageReview {
            reviewer: "malicious_user".to_string(),
            score: 6,
            comment: "Exploit attempt".to_string(),
            timestamp_ms: 3000,
        };
        assert!(registry.submit_review(store_path, bad_rev).is_err());

        // Dynamic aggregate validation ( (5 + 4) / 2 = 4.5 average )
        let (avg, count) = registry.get_aggregate_rating(store_path).unwrap();
        assert_eq!(count, 2);
        assert_eq!(avg, 4.5);
    }
}
