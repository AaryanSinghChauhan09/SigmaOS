/// SigmaOS Kernel Module Loader Engine
/// Manages insmod/modprobe, symbol resolution, alias DB, and module signing.
///
/// Standard library (std, libc) are fully authorized per SigmaOS policy.
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Module signing status
#[derive(Debug, Clone, PartialEq)]
pub enum ModuleSignature {
    Unsigned,
    SignedValid,
    SignedInvalid,
}

/// Module state
#[derive(Debug, Clone, PartialEq)]
pub enum ModuleState {
    Loading,
    Live,
    Unloading,
    Failed,
}

impl fmt::Display for ModuleState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Loading => write!(f, "Loading"),
            Self::Live => write!(f, "Live"),
            Self::Unloading => write!(f, "Unloading"),
            Self::Failed => write!(f, "Failed"),
        }
    }
}

/// Kernel module descriptor
#[derive(Debug, Clone)]
pub struct KernelModule {
    pub name: String,
    pub filename: String,
    pub size_bytes: u64,
    pub use_count: u32,
    pub state: ModuleState,
    pub depends: Vec<String>,
    pub aliases: Vec<String>,
    pub signature: ModuleSignature,
    pub params: HashMap<String, String>,
}

impl KernelModule {
    pub fn new(name: &str, filename: &str) -> Self {
        Self {
            name: name.to_string(),
            filename: filename.to_string(),
            size_bytes: 0,
            use_count: 0,
            state: ModuleState::Loading,
            depends: Vec::new(),
            aliases: Vec::new(),
            signature: ModuleSignature::Unsigned,
            params: HashMap::new(),
        }
    }
}

/// Module blacklist and autoload config
pub struct KmodConfig {
    pub blacklist: HashSet<String>,
    pub autoload: Vec<String>,
    pub enforce_signing: bool,
    pub module_base_path: String,
}

impl KmodConfig {
    pub fn new() -> Self {
        Self {
            blacklist: HashSet::new(),
            autoload: Vec::new(),
            enforce_signing: false,
            module_base_path: "/lib/modules".to_string(),
        }
    }

    pub fn blacklist_module(&mut self, name: &str) {
        self.blacklist.insert(name.to_string());
    }

    pub fn is_blacklisted(&self, name: &str) -> bool {
        self.blacklist.contains(name)
    }
}

/// Alias database (maps device IDs / modalias to module names)
pub struct AliasDatabase {
    entries: HashMap<String, Vec<String>>, // alias pattern -> module names
}

impl AliasDatabase {
    pub fn new() -> Self {
        let mut db = AliasDatabase {
            entries: HashMap::new(),
        };
        // Pre-populate with common aliases (simplified)
        db.add_alias("pci:v00008086d*", "e1000e");
        db.add_alias("pci:v00001022d*", "amd_iommu");
        db.add_alias("sound-service-0-0", "snd_hda_intel");
        db.add_alias("usb:v045Ep028F*", "xpad");
        db
    }

    pub fn add_alias(&mut self, pattern: &str, module: &str) {
        self.entries
            .entry(pattern.to_string())
            .or_default()
            .push(module.to_string());
    }

    pub fn lookup(&self, modalias: &str) -> Vec<String> {
        // Simple exact-match lookup (real impl does glob matching)
        self.entries.get(modalias).cloned().unwrap_or_default()
    }

    pub fn total_aliases(&self) -> usize {
        self.entries.values().map(|v| v.len()).sum()
    }
}

/// Kernel module loader
pub struct SigmaKmodLoader {
    pub loaded: HashMap<String, KernelModule>,
    pub config: KmodConfig,
    pub aliases: AliasDatabase,
}

impl SigmaKmodLoader {
    pub fn new() -> Self {
        Self {
            loaded: HashMap::new(),
            config: KmodConfig::new(),
            aliases: AliasDatabase::new(),
        }
    }

    /// Load a module (simulated insmod)
    pub fn insmod(
        &mut self,
        name: &str,
        size_bytes: u64,
        signature: ModuleSignature,
    ) -> Result<(), String> {
        if self.config.is_blacklisted(name) {
            return Err(format!("module '{}' is blacklisted", name));
        }
        if self.config.enforce_signing && signature != ModuleSignature::SignedValid {
            return Err(format!("module '{}' signature verification failed", name));
        }
        if self.loaded.contains_key(name) {
            return Err(format!("module '{}' already loaded", name));
        }
        let mut module = KernelModule::new(
            name,
            &format!("{}/{}.ko", self.config.module_base_path, name),
        );
        module.size_bytes = size_bytes;
        module.state = ModuleState::Live;
        module.signature = signature;
        self.loaded.insert(name.to_string(), module);
        Ok(())
    }

    /// Remove a module (simulated rmmod)
    pub fn rmmod(&mut self, name: &str) -> Result<(), String> {
        let module = self
            .loaded
            .get(name)
            .ok_or(format!("module '{}' not loaded", name))?;
        if module.use_count > 0 {
            return Err(format!(
                "module '{}' is in use (refcount={})",
                name, module.use_count
            ));
        }
        self.loaded.remove(name);
        Ok(())
    }

    /// Load module by modalias (modprobe equivalent)
    pub fn modprobe_alias(&mut self, modalias: &str) -> Vec<String> {
        let modules = self.aliases.lookup(modalias);
        let mut loaded = Vec::new();
        for m in &modules {
            if self.insmod(m, 65536, ModuleSignature::Unsigned).is_ok() {
                loaded.push(m.clone());
            }
        }
        loaded
    }

    pub fn list_loaded(&self) -> Vec<&KernelModule> {
        self.loaded.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insmod_basic() {
        let mut loader = SigmaKmodLoader::new();
        let result = loader.insmod("btrfs", 524288, ModuleSignature::Unsigned);
        assert!(result.is_ok());
        assert!(loader.loaded.contains_key("btrfs"));
    }

    #[test]
    fn test_blacklist_prevents_load() {
        let mut loader = SigmaKmodLoader::new();
        loader.config.blacklist_module("nouveau");
        let result = loader.insmod("nouveau", 1048576, ModuleSignature::Unsigned);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("blacklisted"));
    }

    #[test]
    fn test_double_insmod_fails() {
        let mut loader = SigmaKmodLoader::new();
        loader
            .insmod("kvm", 262144, ModuleSignature::Unsigned)
            .unwrap();
        let result = loader.insmod("kvm", 262144, ModuleSignature::Unsigned);
        assert!(result.is_err());
    }

    #[test]
    fn test_rmmod_success() {
        let mut loader = SigmaKmodLoader::new();
        loader
            .insmod("loop", 32768, ModuleSignature::Unsigned)
            .unwrap();
        let result = loader.rmmod("loop");
        assert!(result.is_ok());
        assert!(!loader.loaded.contains_key("loop"));
    }

    #[test]
    fn test_alias_database_lookup() {
        let db = AliasDatabase::new();
        let modules = db.lookup("pci:v00008086d*");
        assert!(modules.contains(&"e1000e".to_string()));
    }
}
