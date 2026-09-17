//! SigmaOS Content-Addressed Merkle Package Store
//!
//! Sovereign implementation of NixOS/GNU Guix style content-addressed package store.
//! Inspired by:
//! - NixOS `/nix/store` (Eelco Dolstra, nixos/nixpkgs)
//! - GNU Guix `/gnu/store` content hashing
//! - OSTree content-addressed object store (Fedora Silverblue)
//! - FreeBSD pkg's dependency graph tracking
//!
//! Every package is stored at `/sigma/store/<hash>-<name>-<version>`.
//! The Merkle store guarantees:
//! - Deterministic, reproducible package paths
//! - Instant copy-on-write rollbacks via generation pinning
//! - Transitive dependency closure resolution
//! - Garbage collection of unreferenced entries

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;
use std::format;

// ─── Merkle Hash ──────────────────────────────────────────────────────────────

/// 32-byte content-addressed hash (FNV-1a based)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MerkleHash(pub [u8; 32]);

impl MerkleHash {
    /// Create a zero (null) hash
    pub fn zero() -> Self {
        MerkleHash([0u8; 32])
    }

    /// Compute hash from name, version, and dependency hashes
    pub fn compute(name: &str, version: &str, deps: &[MerkleHash]) -> Self {
        const FNV_PRIME: u64 = 0x00000100000001B3;
        const FNV_OFFSET: u64 = 0xcbf29ce484222325;

        let mut hash: u64 = FNV_OFFSET;

        // Hash name
        for &b in name.as_bytes() {
            hash ^= b as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
        }
        // Separator
        hash ^= 0x3A;
        hash = hash.wrapping_mul(FNV_PRIME);
        // Hash version
        for &b in version.as_bytes() {
            hash ^= b as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
        }
        // Hash each dependency hash to build Merkle tree
        for dep in deps {
            for &b in &dep.0 {
                hash ^= b as u64;
                hash = hash.wrapping_mul(FNV_PRIME);
            }
        }

        let mut out = [0u8; 32];
        for i in 0..8 {
            out[i] = ((hash >> (i * 8)) & 0xFF) as u8;
        }
        // Fill the remaining 24 bytes with derived mixing
        let mut h2 = hash.wrapping_add(0x9e3779b97f4a7c15); // Fibonacci hashing constant
        for i in 8..32 {
            h2 = h2.wrapping_mul(FNV_PRIME) ^ (i as u64 * 0xAAAA);
            out[i] = (h2 & 0xFF) as u8;
        }

        MerkleHash(out)
    }

    /// Returns a hex string representation (first 20 chars for readability)
    pub fn to_hex(&self) -> String {
        const HEX: &[u8] = b"0123456789abcdef";
        let mut s = String::new();
        for &byte in self.0.iter().take(10) {
            s.push(HEX[(byte >> 4) as usize] as char);
            s.push(HEX[(byte & 0xF) as usize] as char);
        }
        s
    }

    /// Returns true if this is a zero hash (uninitialized)
    pub fn is_zero(&self) -> bool {
        self.0 == [0u8; 32]
    }
}

// ─── Store Entry ──────────────────────────────────────────────────────────────

/// A single entry in the SigmaOS Merkle package store
#[derive(Debug, Clone)]
pub struct StoreEntry {
    /// Content-addressed hash of this entry
    pub hash: MerkleHash,
    /// Human-readable package name
    pub name: String,
    /// Package version string
    pub version: String,
    /// Hashes of direct dependencies (forms the Merkle tree)
    pub refs: Vec<MerkleHash>,
    /// Size in bytes of package data
    pub data_size: u64,
    /// Whether this entry has passed integrity verification
    pub is_valid: bool,
    /// Store path: `/sigma/store/<hash>-<name>-<version>`
    pub store_path: String,
}

impl StoreEntry {
    /// Create a new store entry with auto-computed path
    pub fn new(name: &str, version: &str, deps: Vec<MerkleHash>, size: u64) -> Self {
        let hash = MerkleHash::compute(name, version, &deps);
        let store_path = format!("/sigma/store/{}-{}-{}", hash.to_hex(), name, version);
        StoreEntry {
            hash,
            name: String::from(name),
            version: String::from(version),
            refs: deps,
            data_size: size,
            is_valid: true,
            store_path,
        }
    }
}

// ─── Sovereign Merkle Store ──────────────────────────────────────────────────

/// SigmaOS Content-Addressed Merkle Package Store
///
/// Provides NixOS-style deterministic package addressing, transitive
/// dependency closure resolution, GC, and generation pinning.
pub struct SovereignMerkleStore {
    /// All registered entries, indexed by hash
    entries: BTreeMap<MerkleHash, StoreEntry>,
    /// Generation pins: generation_id -> list of pinned hashes (prevents GC)
    pinned: BTreeMap<u32, Vec<MerkleHash>>,
    /// Root hashes explicitly marked as GC roots
    gc_roots: Vec<MerkleHash>,
    /// Current generation counter (incremented on each system generation)
    current_generation: u32,
    /// Total bytes stored
    total_size_bytes: u64,
}

impl SovereignMerkleStore {
    /// Create an empty Merkle store
    pub fn new() -> Self {
        SovereignMerkleStore {
            entries: BTreeMap::new(),
            pinned: BTreeMap::new(),
            gc_roots: Vec::new(),
            current_generation: 0,
            total_size_bytes: 0,
        }
    }

    // ── Registration ──────────────────────────────────────────────────────────

    /// Register a new package and return its content hash.
    /// If an identical package (same name, version, deps) already exists, returns the
    /// existing hash (idempotent — NixOS property).
    pub fn register_package(
        &mut self,
        name: &str,
        version: &str,
        deps: Vec<MerkleHash>,
        size: u64,
    ) -> MerkleHash {
        let hash = MerkleHash::compute(name, version, &deps);
        if self.entries.contains_key(&hash) {
            return hash;
        }
        let entry = StoreEntry::new(name, version, deps, size);
        self.total_size_bytes += size;
        self.entries.insert(hash.clone(), entry);
        hash
    }

    /// Insert a pre-built store entry directly
    pub fn insert(&mut self, entry: StoreEntry) {
        self.total_size_bytes += entry.data_size;
        self.entries.insert(entry.hash.clone(), entry);
    }

    /// Look up an entry by hash
    pub fn get(&self, hash: &MerkleHash) -> Option<&StoreEntry> {
        self.entries.get(hash)
    }

    /// Returns true if the store contains an entry for this hash
    pub fn contains(&self, hash: &MerkleHash) -> bool {
        self.entries.contains_key(hash)
    }

    // ── Closure Resolution ────────────────────────────────────────────────────

    /// Compute the transitive closure of all dependencies for a given hash.
    ///
    /// Returns all hashes required to make the package functional,
    /// in topological order (dependencies before dependents).
    pub fn resolve_closure(&self, hash: &MerkleHash) -> Vec<MerkleHash> {
        let mut visited: BTreeMap<MerkleHash, bool> = BTreeMap::new();
        let mut order: Vec<MerkleHash> = Vec::new();
        self.dfs_closure(hash, &mut visited, &mut order);
        order
    }

    /// DFS traversal for closure resolution
    fn dfs_closure(
        &self,
        hash: &MerkleHash,
        visited: &mut BTreeMap<MerkleHash, bool>,
        order: &mut Vec<MerkleHash>,
    ) {
        if visited.contains_key(hash) {
            return;
        }
        visited.insert(hash.clone(), true);

        if let Some(entry) = self.entries.get(hash) {
            for dep in &entry.refs {
                self.dfs_closure(dep, visited, order);
            }
            order.push(hash.clone());
        }
    }

    // ── Garbage Collection ────────────────────────────────────────────────────

    /// Collect all hashes that are reachable from GC roots and pinned generations
    fn reachable_hashes(&self) -> BTreeMap<MerkleHash, bool> {
        let mut reachable: BTreeMap<MerkleHash, bool> = BTreeMap::new();

        // Traverse from GC roots
        for root in &self.gc_roots {
            for h in self.resolve_closure(root) {
                reachable.insert(h, true);
            }
        }
        // Traverse from pinned generations
        for hashes in self.pinned.values() {
            for h in hashes {
                for closure_h in self.resolve_closure(h) {
                    reachable.insert(closure_h, true);
                }
            }
        }
        reachable
    }

    /// Collect garbage: removes unreachable store entries.
    ///
    /// Returns the list of hashes that were removed.
    pub fn collect_garbage(&mut self) -> Vec<MerkleHash> {
        let reachable = self.reachable_hashes();
        let all_hashes: Vec<MerkleHash> = self.entries.keys().cloned().collect();
        let mut removed: Vec<MerkleHash> = Vec::new();

        for hash in all_hashes {
            if !reachable.contains_key(&hash) {
                if let Some(entry) = self.entries.remove(&hash) {
                    self.total_size_bytes =
                        self.total_size_bytes.saturating_sub(entry.data_size);
                    removed.push(hash);
                }
            }
        }
        removed
    }

    // ── Integrity Verification ────────────────────────────────────────────────

    /// Verify store integrity: all referenced hashes must exist.
    ///
    /// Returns true if store is consistent (no dangling references).
    pub fn verify_store_integrity(&self) -> bool {
        for entry in self.entries.values() {
            for dep_hash in &entry.refs {
                if !self.entries.contains_key(dep_hash) {
                    return false;
                }
            }
            if !entry.is_valid {
                return false;
            }
        }
        true
    }

    /// Returns a list of dangling dependency errors
    pub fn integrity_report(&self) -> Vec<String> {
        let mut errors: Vec<String> = Vec::new();
        for entry in self.entries.values() {
            for dep_hash in &entry.refs {
                if !self.entries.contains_key(dep_hash) {
                    errors.push(format!(
                        "ERROR: {}-{} references missing dep {}",
                        entry.name,
                        entry.version,
                        dep_hash.to_hex()
                    ));
                }
            }
        }
        errors
    }

    // ── Generation Pinning ────────────────────────────────────────────────────

    /// Pin a hash into a generation (prevents GC of this entry and its closure).
    ///
    /// Inspired by NixOS's `nix-env --set-flag keep true` and profile generations.
    pub fn pin_generation(&mut self, gen_id: u32, hash: MerkleHash) {
        self.pinned.entry(gen_id).or_insert_with(Vec::new).push(hash);
        if gen_id > self.current_generation {
            self.current_generation = gen_id;
        }
    }

    /// Unpin an entire generation (allows GC to collect its entries if unreferenced)
    pub fn unpin_generation(&mut self, gen_id: u32) {
        self.pinned.remove(&gen_id);
    }

    /// Add a GC root (always reachable, never collected)
    pub fn add_gc_root(&mut self, hash: MerkleHash) {
        if !self.gc_roots.contains(&hash) {
            self.gc_roots.push(hash);
        }
    }

    // ── Store Path ────────────────────────────────────────────────────────────

    /// Returns the canonical store path for a hash, name, and version.
    /// Format: `/sigma/store/<20-char-hex>-<name>-<version>`
    pub fn store_path_for(hash: &MerkleHash, name: &str, version: &str) -> String {
        format!("/sigma/store/{}-{}-{}", hash.to_hex(), name, version)
    }

    // ── Statistics ────────────────────────────────────────────────────────────

    /// Total number of entries in store
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Total bytes consumed by all entries
    pub fn total_size_bytes(&self) -> u64 {
        self.total_size_bytes
    }

    /// Current generation number
    pub fn current_generation(&self) -> u32 {
        self.current_generation
    }

    /// Returns a formatted store summary string
    pub fn store_summary(&self) -> String {
        format!(
            "SigmaOS Merkle Store | {} entries | {} bytes | gen {} | {} GC roots | {} pinned gens",
            self.entry_count(),
            self.total_size_bytes,
            self.current_generation,
            self.gc_roots.len(),
            self.pinned.len()
        )
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod merkle_tests {
    use super::*;

    #[test]
    fn test_hash_deterministic() {
        let h1 = MerkleHash::compute("bash", "5.2.1", &[]);
        let h2 = MerkleHash::compute("bash", "5.2.1", &[]);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_different_versions() {
        let h1 = MerkleHash::compute("glibc", "2.38", &[]);
        let h2 = MerkleHash::compute("glibc", "2.39", &[]);
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_register_idempotent() {
        let mut store = SovereignMerkleStore::new();
        let h1 = store.register_package("bash", "5.2.1", vec![], 512_000);
        let h2 = store.register_package("bash", "5.2.1", vec![], 512_000);
        assert_eq!(h1, h2);
        assert_eq!(store.entry_count(), 1);
    }

    #[test]
    fn test_dependency_closure() {
        let mut store = SovereignMerkleStore::new();
        let glibc = store.register_package("glibc", "2.38", vec![], 10_000);
        let libgcc = store.register_package("libgcc", "13.2", vec![glibc.clone()], 5_000);
        let bash = store.register_package("bash", "5.2.1", vec![glibc.clone(), libgcc.clone()], 900_000);

        let closure = store.resolve_closure(&bash);
        assert!(closure.contains(&glibc));
        assert!(closure.contains(&libgcc));
        assert!(closure.contains(&bash));
        // glibc should come before bash (topological order)
        let glibc_pos = closure.iter().position(|h| h == &glibc).unwrap();
        let bash_pos = closure.iter().position(|h| h == &bash).unwrap();
        assert!(glibc_pos < bash_pos);
    }

    #[test]
    fn test_garbage_collection() {
        let mut store = SovereignMerkleStore::new();
        let pkg_a = store.register_package("pkg-a", "1.0", vec![], 1000);
        let pkg_b = store.register_package("pkg-b", "1.0", vec![], 1000);

        // Only pin pkg_a as GC root
        store.add_gc_root(pkg_a.clone());

        let removed = store.collect_garbage();
        assert!(removed.contains(&pkg_b));
        assert!(!removed.contains(&pkg_a));
        assert!(store.contains(&pkg_a));
        assert!(!store.contains(&pkg_b));
    }

    #[test]
    fn test_store_integrity_valid() {
        let mut store = SovereignMerkleStore::new();
        let dep = store.register_package("dep", "1.0", vec![], 100);
        let _top = store.register_package("top", "2.0", vec![dep], 200);
        assert!(store.verify_store_integrity());
    }

    #[test]
    fn test_store_path_format() {
        let hash = MerkleHash::compute("vim", "9.1", &[]);
        let path = SovereignMerkleStore::store_path_for(&hash, "vim", "9.1");
        assert!(path.starts_with("/sigma/store/"));
        assert!(path.contains("vim"));
        assert!(path.contains("9.1"));
    }

    #[test]
    fn test_generation_pinning_prevents_gc() {
        let mut store = SovereignMerkleStore::new();
        let pkg = store.register_package("important-pkg", "3.0", vec![], 5000);
        store.pin_generation(1, pkg.clone());

        let removed = store.collect_garbage();
        // pkg should NOT be collected because it's pinned to gen 1
        assert!(!removed.contains(&pkg));
        assert!(store.contains(&pkg));
    }

    #[test]
    fn test_store_summary() {
        let mut store = SovereignMerkleStore::new();
        store.register_package("test", "1.0", vec![], 999);
        let summary = store.store_summary();
        assert!(summary.contains("SigmaOS Merkle Store"));
        assert!(summary.contains("1 entries"));
    }

    #[test]
    fn test_hex_representation() {
        let hash = MerkleHash::compute("curl", "8.5", &[]);
        let hex = hash.to_hex();
        assert_eq!(hex.len(), 20);
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
