//! Nix Content-Addressed Store — Functional Package Management for SigmaOS
//!
//! Inspired by the Nix package manager (https://nixos.org), this module implements
//! a content-addressed, functional package store where every package is identified
//! by a cryptographic hash of its build inputs, providing:
//! - Reproducible builds (same inputs → same hash → same output)
//! - Atomic upgrades and rollbacks
//! - Multiple versions of the same package side-by-side
//! - Per-user package profiles
//! - Garbage collection of unreferenced store paths
//! - Binary cache support
//!
//! References:
//! - Nix thesis (Eelco Dolstra 2006): https://nixos.org/~eelco/pubs/phd-thesis.pdf
//! - Nix manual: https://nixos.org/manual/nix/stable/
//! - GNU Guix (extension of Nix ideas)
//! - Tvix (Nix rewrite in Rust): https://cs.tvl.fyi/depot/-/tree/tvix
//!
//! Future Development:
//! - Tvix-compatible store protocol implementation
//! - Integration with SigmaOS sigpkg package manager
//! - Content-addressed binary cache server
//! - Flakes-inspired locked dependency manifests
//! - IPFS-based distributed store backend

extern crate alloc;
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::String;
use alloc::vec::Vec;

/// Nix store hash algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NixHashAlgo {
    Sha256,
    Sha512,
    Blake3,
    Md5, // Legacy compatibility
}

/// A content-addressed hash in the Nix format
/// Format: `<algo>:<base32_encoded_hash>`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NixHash(pub String);

impl NixHash {
    /// Create a new hash from raw bytes (simulated for no_std)
    pub fn from_bytes(algo: NixHashAlgo, data: &[u8]) -> Self {
        // Simulated hash: FNV-1a over bytes for deterministic testing
        let mut h: u64 = 0xcbf29ce484222325;
        for &b in data {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        let algo_str = match algo {
            NixHashAlgo::Sha256 => "sha256",
            NixHashAlgo::Sha512 => "sha512",
            NixHashAlgo::Blake3 => "blake3",
            NixHashAlgo::Md5 => "md5",
        };
        // Nix uses base32; we simulate with hex for simplicity
        Self(alloc::format!("{}:{:016x}", algo_str, h))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Compute the store path from a hash (like Nix's /nix/store/<hash>-<name>)
    pub fn to_store_path(&self, name: &str) -> String {
        let hash_short = &self.0[self.0.find(':').map(|i| i + 1).unwrap_or(0)..];
        let hash_prefix = &hash_short[..hash_short.len().min(32)];
        alloc::format!("/sigma/store/{}-{}", hash_prefix, name)
    }
}

/// Nix Derivation — the blueprint for building a package
/// A derivation fully specifies how to produce a store path from inputs.
#[derive(Debug, Clone)]
pub struct NixDerivation {
    /// Derivation name (e.g. "bash-5.2")
    pub name: String,
    /// System (e.g. "x86_64-linux", "aarch64-linux")
    pub system: String,
    /// Builder executable (store path)
    pub builder: String,
    /// Arguments to builder
    pub args: Vec<String>,
    /// Environment variables for build
    pub env: BTreeMap<String, String>,
    /// Input derivations: { drv_path -> [output_names] }
    pub input_drvs: BTreeMap<String, Vec<String>>,
    /// Input source files (content-addressed)
    pub input_srcs: Vec<String>,
    /// Output specifications: { output_name -> NixOutput }
    pub outputs: BTreeMap<String, NixOutput>,
    /// Hash of this derivation (content-addressed)
    pub drv_hash: NixHash,
}

/// A derivation output (a store path produced by building)
#[derive(Debug, Clone)]
pub struct NixOutput {
    /// Output name (usually "out", sometimes "dev", "lib", "doc")
    pub name: String,
    /// Store path where output will be placed
    pub store_path: String,
    /// Hash algorithm used for content-addressing
    pub hash_algo: Option<NixHashAlgo>,
    /// Fixed-output hash (for source tarballs, etc.)
    pub fixed_hash: Option<NixHash>,
}

/// Nix Store Path — a realization of a derivation output
#[derive(Debug, Clone)]
pub struct NixStorePath {
    /// Full store path (e.g. /sigma/store/abc123-bash-5.2)
    pub path: String,
    /// Name component
    pub name: String,
    /// Content hash
    pub hash: NixHash,
    /// References (other store paths this depends on)
    pub references: BTreeSet<String>,
    /// Registration timestamp
    pub registration_time: u64,
    /// Nar (normalized archive) hash
    pub nar_hash: NixHash,
    /// Nar size in bytes
    pub nar_size: u64,
    /// Is this a derivation?
    pub is_derivation: bool,
    /// Ultimate referrers (paths that reference this)
    pub referrers: BTreeSet<String>,
}

impl NixStorePath {
    pub fn new(path: String, name: String, hash: NixHash, nar_size: u64) -> Self {
        let nar_hash = NixHash::from_bytes(NixHashAlgo::Sha256, path.as_bytes());
        Self {
            path,
            name,
            hash,
            references: BTreeSet::new(),
            registration_time: 0,
            nar_hash,
            nar_size,
            is_derivation: false,
            referrers: BTreeSet::new(),
        }
    }
}

/// Nix Profile — a user-visible set of package activations
#[derive(Debug, Clone)]
pub struct NixProfile {
    /// Profile name (e.g. "default", "dev", "gaming")
    pub name: String,
    /// Current generation number
    pub generation: u32,
    /// Installed packages: { package_name -> store_path }
    pub packages: BTreeMap<String, String>,
    /// Generation history
    pub generations: Vec<NixProfileGeneration>,
}

/// A profile generation snapshot (for rollback)
#[derive(Debug, Clone)]
pub struct NixProfileGeneration {
    pub number: u32,
    pub timestamp: u64,
    pub packages: BTreeMap<String, String>,
    pub description: String,
}

impl NixProfile {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            generation: 0,
            packages: BTreeMap::new(),
            generations: Vec::new(),
        }
    }

    /// Install a package (add to profile, bump generation)
    pub fn install(&mut self, pkg_name: &str, store_path: &str) {
        self.packages
            .insert(String::from(pkg_name), String::from(store_path));
        self.bump_generation(alloc::format!("Installed {}", pkg_name));
    }

    /// Uninstall a package
    pub fn uninstall(&mut self, pkg_name: &str) -> bool {
        if self.packages.remove(pkg_name).is_some() {
            self.bump_generation(alloc::format!("Removed {}", pkg_name));
            true
        } else {
            false
        }
    }

    /// Rollback to previous generation
    pub fn rollback(&mut self) -> Result<(), &'static str> {
        if self.generations.len() < 2 {
            return Err("No previous generation to roll back to");
        }
        let prev = self.generations[self.generations.len() - 2].clone();
        self.packages = prev.packages.clone();
        self.generation = prev.number;
        Ok(())
    }

    fn bump_generation(&mut self, desc: String) {
        let gen = NixProfileGeneration {
            number: self.generation,
            timestamp: 0,
            packages: self.packages.clone(),
            description: desc,
        };
        self.generations.push(gen);
        self.generation += 1;
    }
}

/// GC root — a path that prevents garbage collection of a store path
#[derive(Debug, Clone)]
pub struct NixGcRoot {
    pub name: String,
    pub target: String,
}

/// Nix Store — the main content-addressed package store
#[derive(Debug)]
pub struct NixStore {
    /// Store prefix path (e.g. "/sigma/store")
    pub store_dir: String,
    /// All store paths: { path -> StorePath }
    pub paths: BTreeMap<String, NixStorePath>,
    /// All derivations: { drv_path -> Derivation }
    pub derivations: BTreeMap<String, NixDerivation>,
    /// User profiles: { username/profile_name -> Profile }
    pub profiles: BTreeMap<String, NixProfile>,
    /// GC roots (prevent garbage collection)
    pub gc_roots: Vec<NixGcRoot>,
    /// Binary cache URLs
    pub binary_caches: Vec<String>,
    /// Statistics
    pub stats: NixStoreStats,
}

/// Nix store statistics
#[derive(Debug, Default)]
pub struct NixStoreStats {
    pub total_paths: u64,
    pub total_size_bytes: u64,
    pub paths_garbage_collected: u64,
    pub bytes_freed: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub builds_completed: u64,
    pub builds_failed: u64,
}

impl NixStore {
    /// Create a new Nix-compatible store at /sigma/store
    pub fn new() -> Self {
        let mut store = Self {
            store_dir: String::from("/sigma/store"),
            paths: BTreeMap::new(),
            derivations: BTreeMap::new(),
            profiles: BTreeMap::new(),
            gc_roots: Vec::new(),
            binary_caches: vec![String::from("https://cache.sigma-os.dev/")],
            stats: NixStoreStats::default(),
        };
        // Create default user profile
        store
            .profiles
            .insert(String::from("root/default"), NixProfile::new("default"));
        store
    }

    /// Register a new store path
    pub fn register_path(&mut self, path: NixStorePath) {
        self.stats.total_size_bytes += path.nar_size;
        self.stats.total_paths += 1;
        self.paths.insert(path.path.clone(), path);
    }

    /// Check if a path is in the store
    pub fn is_valid_path(&self, path: &str) -> bool {
        self.paths.contains_key(path)
    }

    /// Add a derivation to the store
    pub fn add_derivation(&mut self, drv: NixDerivation) -> String {
        let drv_path = alloc::format!(
            "{}/{}.drv",
            self.store_dir,
            drv.drv_hash.to_store_path(&drv.name)
        );
        self.derivations.insert(drv_path.clone(), drv);
        drv_path
    }

    /// Realize (build) a derivation — produces output store paths
    /// In a real implementation this would actually build the package.
    pub fn realise(&mut self, drv_path: &str) -> Result<Vec<String>, &'static str> {
        if let Some(drv) = self.derivations.get(drv_path).cloned() {
            let mut output_paths = Vec::new();
            for (out_name, output) in &drv.outputs {
                let path = output.store_path.clone();
                if !self.paths.contains_key(&path) {
                    let hash = NixHash::from_bytes(NixHashAlgo::Sha256, path.as_bytes());
                    let store_path = NixStorePath::new(
                        path.clone(),
                        alloc::format!("{}-{}", drv.name, out_name),
                        hash,
                        1024 * 1024, // 1MB simulated
                    );
                    self.register_path(store_path);
                }
                output_paths.push(path);
            }
            self.stats.builds_completed += 1;
            Ok(output_paths)
        } else {
            self.stats.builds_failed += 1;
            Err("Derivation not found in store")
        }
    }

    /// Install a package into a user profile
    pub fn install_package(
        &mut self,
        profile_key: &str,
        pkg_name: &str,
        store_path: &str,
    ) -> Result<(), &'static str> {
        if !self.paths.contains_key(store_path) {
            return Err("Store path not valid — build or fetch package first");
        }
        let profile = self
            .profiles
            .entry(String::from(profile_key))
            .or_insert_with(|| NixProfile::new(profile_key));
        profile.install(pkg_name, store_path);
        Ok(())
    }

    /// Uninstall a package from a user profile
    pub fn uninstall_package(&mut self, profile_key: &str, pkg_name: &str) -> bool {
        if let Some(profile) = self.profiles.get_mut(profile_key) {
            profile.uninstall(pkg_name)
        } else {
            false
        }
    }

    /// Get all paths referenced from GC roots (live set)
    pub fn compute_live_paths(&self) -> BTreeSet<String> {
        let mut live = BTreeSet::new();
        // Add all profile packages as live
        for profile in self.profiles.values() {
            for path in profile.packages.values() {
                self.add_closure(path, &mut live);
            }
        }
        // Add GC roots
        for root in &self.gc_roots {
            self.add_closure(&root.target, &mut live);
        }
        live
    }

    /// Recursively add closure (transitive dependencies)
    fn add_closure(&self, path: &str, live: &mut BTreeSet<String>) {
        if live.contains(path) {
            return;
        }
        live.insert(String::from(path));
        if let Some(store_path) = self.paths.get(path) {
            for dep in &store_path.references {
                self.add_closure(dep, live);
            }
        }
    }

    /// Garbage collect unreferenced store paths
    pub fn gc(&mut self) -> (u64, u64) {
        let live = self.compute_live_paths();
        let dead_paths: Vec<String> = self
            .paths
            .keys()
            .filter(|p| !live.contains(*p))
            .cloned()
            .collect();

        let mut freed_count = 0u64;
        let mut freed_bytes = 0u64;

        for path in dead_paths {
            if let Some(sp) = self.paths.remove(&path) {
                freed_bytes += sp.nar_size;
                freed_count += 1;
            }
        }

        self.stats.paths_garbage_collected += freed_count;
        self.stats.bytes_freed += freed_bytes;
        self.stats.total_paths -= freed_count;
        self.stats.total_size_bytes -= freed_bytes;

        (freed_count, freed_bytes)
    }

    /// Add a binary cache
    pub fn add_binary_cache(&mut self, url: &str) {
        self.binary_caches.push(String::from(url));
    }

    /// Add a GC root
    pub fn add_gc_root(&mut self, name: &str, target: &str) {
        self.gc_roots.push(NixGcRoot {
            name: String::from(name),
            target: String::from(target),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nix_hash_deterministic() {
        let h1 = NixHash::from_bytes(NixHashAlgo::Sha256, b"hello world");
        let h2 = NixHash::from_bytes(NixHashAlgo::Sha256, b"hello world");
        assert_eq!(h1, h2); // Same input = same hash (reproducible)
    }

    #[test]
    fn test_nix_hash_different_inputs() {
        let h1 = NixHash::from_bytes(NixHashAlgo::Sha256, b"bash-5.2");
        let h2 = NixHash::from_bytes(NixHashAlgo::Sha256, b"bash-5.3");
        assert_ne!(h1, h2); // Different inputs = different hash
    }

    #[test]
    fn test_store_path_format() {
        let hash = NixHash::from_bytes(NixHashAlgo::Sha256, b"bash-5.2");
        let path = hash.to_store_path("bash-5.2");
        assert!(path.starts_with("/sigma/store/"));
        assert!(path.contains("bash-5.2"));
    }

    #[test]
    fn test_store_register_path() {
        let mut store = NixStore::new();
        let hash = NixHash::from_bytes(NixHashAlgo::Sha256, b"test-pkg-1.0");
        let path_str = hash.to_store_path("test-pkg-1.0");
        let sp = NixStorePath::new(path_str.clone(), String::from("test-pkg-1.0"), hash, 512);
        store.register_path(sp);
        assert!(store.is_valid_path(&path_str));
        assert_eq!(store.stats.total_paths, 1);
    }

    #[test]
    fn test_profile_install_uninstall() {
        let mut profile = NixProfile::new("default");
        profile.install("bash", "/sigma/store/abc-bash-5.2");
        assert_eq!(profile.packages.len(), 1);
        assert_eq!(profile.generation, 1);

        let removed = profile.uninstall("bash");
        assert!(removed);
        assert_eq!(profile.packages.len(), 0);
        assert_eq!(profile.generation, 2);
    }

    #[test]
    fn test_profile_rollback() {
        let mut profile = NixProfile::new("default");
        profile.install("bash", "/sigma/store/abc-bash-5.2");
        profile.install("vim", "/sigma/store/def-vim-9.0");

        // 2 packages installed
        assert_eq!(profile.packages.len(), 2);

        // Rollback to before vim
        profile.rollback().unwrap();
        assert_eq!(profile.packages.len(), 1);
        assert!(profile.packages.contains_key("bash"));
    }

    #[test]
    fn test_garbage_collection() {
        let mut store = NixStore::new();
        let hash1 = NixHash::from_bytes(NixHashAlgo::Sha256, b"pkg1");
        let hash2 = NixHash::from_bytes(NixHashAlgo::Sha256, b"pkg2");
        let path1 = hash1.to_store_path("pkg1");
        let path2 = hash2.to_store_path("pkg2");

        store.register_path(NixStorePath::new(
            path1.clone(),
            String::from("pkg1"),
            hash1,
            100,
        ));
        store.register_path(NixStorePath::new(
            path2.clone(),
            String::from("pkg2"),
            hash2,
            200,
        ));

        // Add pkg1 to a profile (live), pkg2 is unreferenced (dead)
        store
            .profiles
            .entry(String::from("root/default"))
            .or_insert_with(|| NixProfile::new("default"))
            .install("pkg1", &path1);

        let (freed_count, freed_bytes) = store.gc();
        assert_eq!(freed_count, 1); // pkg2 was GC'd
        assert_eq!(freed_bytes, 200);
        assert!(store.is_valid_path(&path1)); // pkg1 still alive
        assert!(!store.is_valid_path(&path2)); // pkg2 GC'd
    }

    #[test]
    fn test_closure_computation() {
        let mut store = NixStore::new();
        let hash_a = NixHash::from_bytes(NixHashAlgo::Sha256, b"dep-a");
        let hash_b = NixHash::from_bytes(NixHashAlgo::Sha256, b"pkg-b");
        let path_a = hash_a.to_store_path("dep-a");
        let path_b = hash_b.to_store_path("pkg-b");

        let sp_a = NixStorePath::new(path_a.clone(), String::from("dep-a"), hash_a, 50);
        let mut sp_b = NixStorePath::new(path_b.clone(), String::from("pkg-b"), hash_b, 150);
        sp_b.references.insert(path_a.clone()); // pkg-b depends on dep-a

        store.register_path(sp_a);
        store.register_path(sp_b);

        // Install pkg-b (should keep dep-a alive through closure)
        store
            .profiles
            .entry(String::from("root/default"))
            .or_insert_with(|| NixProfile::new("default"))
            .install("pkg-b", &path_b);

        let live = store.compute_live_paths();
        assert!(live.contains(&path_a)); // dep-a is in closure
        assert!(live.contains(&path_b)); // pkg-b is direct
    }
}
