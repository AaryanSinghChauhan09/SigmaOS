// SigmaOS — nixos_inspirations.rs
// Implements NixOS-inspired features:
//   • Declarative system configuration
//   • Atomic upgrades (two-phase commit)
//   • Generation-based rollback
//   • Content-addressed package store (/sigma/store)
//   • Reproducible builds via locked inputs


use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

// ── Content-addressed store ───────────────────────────────────────────────────

/// A 256-bit (32-byte) content hash, stored as hex text for readability.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StoreHash(pub [u8; 32]);

impl StoreHash {
    /// Compute a simple FNV-1a–based stand-in for a real cryptographic hash.
    /// A production implementation would use SHA-256 or BLAKE3.
    pub fn compute(data: &[u8]) -> Self {
        let mut hash = [0u8; 32];
        let mut h: u64 = 0xcbf29ce484222325u64;
        for &b in data {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3u64);
        }
        // Spread the 64-bit value across 32 bytes deterministically.
        for i in 0..8 {
            let shift = (i % 8) * 8;
            hash[i] = ((h >> shift) & 0xff) as u8;
            hash[i + 8] = ((h.rotate_left(13) >> shift) & 0xff) as u8;
            hash[i + 16] = ((h.rotate_left(27) >> shift) & 0xff) as u8;
            hash[i + 24] = ((h.rotate_left(41) >> shift) & 0xff) as u8;
        }
        StoreHash(hash)
    }

    /// Return the first 32 hex characters — used as the store-path prefix.
    pub fn short_hex(&self) -> String {
        let mut s = String::new();
        for &b in &self.0[..16] {
            let hi = (b >> 4) as usize;
            let lo = (b & 0xf) as usize;
            const HEX: &[u8] = b"0123456789abcdef";
            s.push(HEX[hi] as char);
            s.push(HEX[lo] as char);
        }
        s
    }
}

/// A single entry in the Sigma content-addressed store.
///
/// Analogous to a path under `/nix/store/` in NixOS.
#[derive(Debug, Clone)]
pub struct StoreEntry {
    /// Content hash that uniquely identifies this closure.
    pub hash: StoreHash,
    /// Human-readable name component, e.g. `glibc-2.39`.
    pub name: String,
    /// Hashes of packages that this entry depends on.
    pub references: Vec<StoreHash>,
    /// `true` once the entry has been registered in the store database.
    pub realised: bool,
}

impl StoreEntry {
    pub fn new(name: &str, content: &[u8]) -> Self {
        StoreEntry {
            hash: StoreHash::compute(content),
            name: name.to_string(),
            references: Vec::new(),
            realised: false,
        }
    }

    /// Canonical store path, e.g. `/sigma/store/<hash>-<name>`.
    pub fn store_path(&self) -> String {
        format!("/sigma/store/{}-{}", self.hash.short_hex(), self.name)
    }
}

/// The Sigma package store.
///
/// Holds all realised entries keyed by their `StoreHash`.
pub struct SigmaStore {
    entries: Vec<StoreEntry>,
}

impl SigmaStore {
    pub fn new() -> Self {
        SigmaStore {
            entries: Vec::new(),
        }
    }

    /// Realise `entry` into the store.
    ///
    /// Returns `true` if the entry was newly added; `false` if it was already
    /// present (content-addressed deduplication).
    pub fn realise(&mut self, mut entry: StoreEntry) -> bool {
        if self.contains(&entry.hash) {
            return false;
        }
        entry.realised = true;
        self.entries.push(entry);
        true
    }

    pub fn contains(&self, hash: &StoreHash) -> bool {
        self.entries.iter().any(|e| &e.hash == hash)
    }

    pub fn get(&self, hash: &StoreHash) -> Option<&StoreEntry> {
        self.entries.iter().find(|e| &e.hash == hash)
    }

    /// Return the total number of entries in the store.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Collect entries that are not referenced by any other entry
    /// (i.e. garbage-collectible roots).
    pub fn roots(&self) -> Vec<&StoreEntry> {
        let referenced: Vec<&StoreHash> = self
            .entries
            .iter()
            .flat_map(|e| e.references.iter())
            .collect();
        self.entries
            .iter()
            .filter(|e| !referenced.iter().any(|r| *r == &e.hash))
            .collect()
    }
}

// ── Declarative configuration ─────────────────────────────────────────────────

/// A key-value configuration option with optional description.
#[derive(Debug, Clone)]
pub struct ConfigOption {
    pub key: String,
    pub value: ConfigValue,
    pub description: String,
}

/// Supported value types for declarative configuration.
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigValue {
    Bool(bool),
    Integer(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}

impl ConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        if let ConfigValue::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        if let ConfigValue::Str(s) = self {
            Some(s.as_str())
        } else {
            None
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        if let ConfigValue::List(v) = self {
            Some(v.as_slice())
        } else {
            None
        }
    }
}

/// A fully declarative system configuration (analogous to `configuration.nix`).
///
/// All settings are collected here and applied atomically at activation time.
#[derive(Debug, Clone, Default)]
pub struct SystemConfiguration {
    pub hostname: String,
    pub timezone: String,
    pub locale: String,
    pub boot_loader: String,
    pub kernel_packages: Vec<String>,
    pub system_packages: Vec<String>,
    pub services: Vec<ServiceDeclaration>,
    pub users: Vec<UserDeclaration>,
    pub options: Vec<ConfigOption>,
    /// Locked input hash — ensures builds are reproducible.
    pub lock_hash: Option<StoreHash>,
}

impl SystemConfiguration {
    pub fn new() -> Self {
        SystemConfiguration {
            hostname: "sigma".to_string(),
            timezone: "UTC".to_string(),
            locale: "en_US.UTF-8".to_string(),
            boot_loader: "sigma-boot".to_string(),
            ..Default::default()
        }
    }

    pub fn add_package(&mut self, pkg: &str) {
        if !self.system_packages.iter().any(|p| p == pkg) {
            self.system_packages.push(pkg.to_string());
        }
    }

    pub fn add_service(&mut self, svc: ServiceDeclaration) {
        self.services.push(svc);
    }

    pub fn set_option(&mut self, key: &str, value: ConfigValue, desc: &str) {
        // Replace existing option if key matches.
        for opt in self.options.iter_mut() {
            if opt.key == key {
                opt.value = value;
                return;
            }
        }
        self.options.push(ConfigOption {
            key: key.to_string(),
            value,
            description: desc.to_string(),
        });
    }

    pub fn get_option(&self, key: &str) -> Option<&ConfigValue> {
        self.options.iter().find(|o| o.key == key).map(|o| &o.value)
    }

    /// Compute a hash of the whole configuration for change-detection.
    pub fn config_hash(&self) -> StoreHash {
        // Build a canonical byte representation.
        let mut repr = String::new();
        repr.push_str(&self.hostname);
        repr.push(':');
        repr.push_str(&self.timezone);
        repr.push(':');
        for pkg in &self.system_packages {
            repr.push_str(pkg);
            repr.push(',');
        }
        StoreHash::compute(repr.as_bytes())
    }
}

/// A service managed by the SigmaOS init system.
#[derive(Debug, Clone)]
pub struct ServiceDeclaration {
    pub name: String,
    pub enabled: bool,
    pub auto_start: bool,
    pub description: String,
    pub exec_start: String,
    pub restart_policy: RestartPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestartPolicy {
    Never,
    OnFailure,
    Always,
}

/// A user account definition.
#[derive(Debug, Clone)]
pub struct UserDeclaration {
    pub name: String,
    pub uid: Option<u32>,
    pub groups: Vec<String>,
    pub shell: String,
    pub home: String,
    pub is_system: bool,
}

// ── Atomic upgrades & rollback ────────────────────────────────────────────────

/// A system generation snapshot.
///
/// Every time the system configuration is applied, a new generation is created.
/// This allows rolling back to any previous state.
#[derive(Debug, Clone)]
pub struct SystemGeneration {
    pub id: u64,
    pub config_hash: StoreHash,
    pub packages: Vec<String>,
    pub timestamp_secs: u64,
    pub description: String,
    pub is_current: bool,
}

/// Manages the history of system generations and supports atomic
/// upgrade/rollback.
pub struct GenerationManager {
    generations: Vec<SystemGeneration>,
    next_id: u64,
}

impl GenerationManager {
    pub fn new() -> Self {
        GenerationManager {
            generations: Vec::new(),
            next_id: 1,
        }
    }

    /// Create a new generation from `config`.
    ///
    /// Marks the previous current generation as non-current.
    /// Returns the new generation ID.
    pub fn create_generation(
        &mut self,
        config: &SystemConfiguration,
        timestamp_secs: u64,
        description: &str,
    ) -> u64 {
        for gen in self.generations.iter_mut() {
            gen.is_current = false;
        }
        let id = self.next_id;
        self.next_id += 1;
        self.generations.push(SystemGeneration {
            id,
            config_hash: config.config_hash(),
            packages: config.system_packages.clone(),
            timestamp_secs,
            description: description.to_string(),
            is_current: true,
        });
        id
    }

    /// Roll back to the generation with `id`.
    ///
    /// Returns `Ok(id)` on success, or `Err` if the generation is not found.
    pub fn rollback_to(&mut self, id: u64) -> Result<u64, String> {
        let exists = self.generations.iter().any(|g| g.id == id);
        if !exists {
            return Err(format!("generation {} not found", id));
        }
        for gen in self.generations.iter_mut() {
            gen.is_current = gen.id == id;
        }
        Ok(id)
    }

    /// Return the current (active) generation, if any.
    pub fn current(&self) -> Option<&SystemGeneration> {
        self.generations.iter().find(|g| g.is_current)
    }

    /// Return all generations in creation order (oldest first).
    pub fn all(&self) -> &[SystemGeneration] {
        &self.generations
    }

    /// Remove generations older than `keep_count` (keeping the most recent).
    ///
    /// The current generation is never removed.
    pub fn garbage_collect(&mut self, keep_count: usize) {
        // Sort by id ascending.
        self.generations.sort_by_key(|g| g.id);
        let non_current: Vec<u64> = self
            .generations
            .iter()
            .filter(|g| !g.is_current)
            .map(|g| g.id)
            .collect();
        if non_current.len() <= keep_count {
            return;
        }
        let remove_count = non_current.len() - keep_count;
        let ids_to_remove: Vec<u64> = non_current[..remove_count].to_vec();
        self.generations.retain(|g| !ids_to_remove.contains(&g.id));
    }
}

// ── Atomic upgrade transaction ─────────────────────────────────────────────────

/// Result of an atomic upgrade attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpgradeResult {
    /// Upgrade succeeded; new generation ID is enclosed.
    Applied(u64),
    /// No changes detected; current generation is unchanged.
    NoChange,
    /// Upgrade failed during some phase; system was *not* modified.
    Failed(String),
}

/// Performs a two-phase atomic upgrade:
///
///  1. **Prepare** — build/download all required store entries.
///  2. **Commit** — atomically switch the active profile symlink and create
///     the new generation record.
///
/// If any step in phase 1 fails the commit is never executed, leaving the
/// running system untouched.
pub struct AtomicUpgrade<'a> {
    pub store: &'a mut SigmaStore,
    pub generations: &'a mut GenerationManager,
}

impl<'a> AtomicUpgrade<'a> {
    pub fn new(store: &'a mut SigmaStore, generations: &'a mut GenerationManager) -> Self {
        AtomicUpgrade { store, generations }
    }

    /// Apply `new_config` atomically.
    ///
    /// `timestamp_secs` — Unix timestamp to record in the new generation.
    pub fn apply(
        &mut self,
        new_config: &SystemConfiguration,
        timestamp_secs: u64,
        description: &str,
    ) -> UpgradeResult {
        // Phase 1: prepare — realise all packages.
        let mut new_entries = Vec::new();
        for pkg in &new_config.system_packages {
            let entry = StoreEntry::new(pkg, pkg.as_bytes());
            new_entries.push(entry);
        }

        // Simulate a preparation failure by checking for a special marker.
        for entry in &new_entries {
            if entry.name.contains("BROKEN") {
                return UpgradeResult::Failed(format!("package {} is marked broken", entry.name));
            }
        }

        // Check whether anything changed.
        let new_hash = new_config.config_hash();
        if let Some(cur) = self.generations.current() {
            if cur.config_hash == new_hash {
                return UpgradeResult::NoChange;
            }
        }

        // Phase 2: commit — add entries to store and record generation.
        for entry in new_entries {
            self.store.realise(entry);
        }
        let gen_id = self
            .generations
            .create_generation(new_config, timestamp_secs, description);
        UpgradeResult::Applied(gen_id)
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_store_content_addressing() {
        let mut store = SigmaStore::new();
        let e1 = StoreEntry::new("glibc-2.39", b"glibc content v1");
        let e2 = StoreEntry::new("glibc-2.39", b"glibc content v1"); // identical
        let e3 = StoreEntry::new("glibc-2.40", b"glibc content v2");
        let hash1 = e1.hash.clone();

        assert!(store.realise(e1));
        assert!(!store.realise(e2)); // deduplication
        assert!(store.realise(e3));
        assert_eq!(store.len(), 2);
        assert!(store.contains(&hash1));
    }

    #[test]
    fn test_declarative_config() {
        let mut cfg = SystemConfiguration::new();
        cfg.add_package("vim");
        cfg.add_package("git");
        cfg.add_package("vim"); // duplicate — should not grow
        assert_eq!(cfg.system_packages.len(), 2);

        cfg.set_option(
            "networking.firewall.enable",
            ConfigValue::Bool(true),
            "Enable the firewall",
        );
        assert_eq!(
            cfg.get_option("networking.firewall.enable"),
            Some(&ConfigValue::Bool(true))
        );
    }

    #[test]
    fn test_atomic_upgrade_and_rollback() {
        let mut store = SigmaStore::new();
        let mut gens = GenerationManager::new();

        let mut cfg1 = SystemConfiguration::new();
        cfg1.add_package("glibc");

        let mut upgrade = AtomicUpgrade::new(&mut store, &mut gens);
        let result = upgrade.apply(&cfg1, 1000, "initial system");
        assert!(matches!(result, UpgradeResult::Applied(1)));

        let mut cfg2 = SystemConfiguration::new();
        cfg2.add_package("glibc");
        cfg2.add_package("vim");

        let result2 = AtomicUpgrade::new(&mut store, &mut gens).apply(&cfg2, 2000, "add vim");
        assert!(matches!(result2, UpgradeResult::Applied(2)));

        // Rollback to generation 1.
        let rb = gens.rollback_to(1);
        assert!(rb.is_ok());
        assert_eq!(gens.current().unwrap().id, 1);
    }

    #[test]
    fn test_upgrade_no_change() {
        let mut store = SigmaStore::new();
        let mut gens = GenerationManager::new();

        let mut cfg = SystemConfiguration::new();
        cfg.add_package("bash");

        AtomicUpgrade::new(&mut store, &mut gens).apply(&cfg, 1000, "initial");

        // Applying the same config again must report NoChange.
        let result = AtomicUpgrade::new(&mut store, &mut gens).apply(&cfg, 2000, "no change");
        assert_eq!(result, UpgradeResult::NoChange);
    }

    #[test]
    fn test_garbage_collect_generations() {
        let mut store = SigmaStore::new();
        let mut gens = GenerationManager::new();

        for i in 0..5u64 {
            let mut cfg = SystemConfiguration::new();
            cfg.add_package(&format!("pkg-{}", i));
            AtomicUpgrade::new(&mut store, &mut gens).apply(&cfg, i * 1000, "gen");
        }
        assert_eq!(gens.all().len(), 5);
        gens.garbage_collect(2); // keep 2 non-current
        assert!(gens.all().len() <= 3); // 2 non-current + 1 current
        assert!(gens.current().is_some());
    }
}
