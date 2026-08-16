// SigmaOS Debian Clean-Room Parity Subsystem
// Independent, zero-dependency implementations of Debian-specific core tooling

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::string::ToString;
use alloc::format;

/// SysVinit Runlevel Target represents runlevels (0 to 6) in SysVinit model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SysVinitRunlevel {
    Level0, // Halt
    Level1, // Single-user / rescue
    Level2, // Multi-user (default Debian, without network services)
    Level3, // Multi-user with networking
    Level4, // Unused / custom
    Level5, // Multi-user with graphical display manager
    Level6, // Reboot
}

/// SysVinitManager emulates Debian's legacy SysVinit runlevel targets management.
pub struct SysVinitManager {
    pub current_runlevel: SysVinitRunlevel,
    pub startup_scripts: BTreeMap<SysVinitRunlevel, Vec<String>>,
    pub shutdown_scripts: BTreeMap<SysVinitRunlevel, Vec<String>>,
    pub execution_log: Vec<String>,
}

impl SysVinitManager {
    pub fn new() -> Self {
        let mut startup = BTreeMap::new();
        startup.insert(SysVinitRunlevel::Level2, vec!["rc.local".to_string(), "cron".to_string()]);
        startup.insert(SysVinitRunlevel::Level3, vec!["rc.local".to_string(), "networking".to_string(), "ssh".to_string()]);
        startup.insert(SysVinitRunlevel::Level5, vec!["rc.local".to_string(), "networking".to_string(), "gdm3".to_string()]);

        let mut shutdown = BTreeMap::new();
        shutdown.insert(SysVinitRunlevel::Level0, vec!["sendsigs".to_string(), "urandom".to_string(), "halt".to_string()]);
        shutdown.insert(SysVinitRunlevel::Level6, vec!["sendsigs".to_string(), "urandom".to_string(), "reboot".to_string()]);

        Self {
            current_runlevel: SysVinitRunlevel::Level2,
            startup_scripts: startup,
            shutdown_scripts: shutdown,
            execution_log: Vec::new(),
        }
    }

    pub fn switch_runlevel(&mut self, target: SysVinitRunlevel) {
        let old = self.current_runlevel;

        // Log shutdown sequence of old runlevel
        if let Some(scripts) = self.shutdown_scripts.get(&target) {
            for script in scripts {
                self.execution_log.push(format!("stop:{:?}:{}", old, script));
            }
        }

        // Log startup sequence of new runlevel
        if let Some(scripts) = self.startup_scripts.get(&target) {
            for script in scripts {
                self.execution_log.push(format!("start:{:?}:{}", target, script));
            }
        }

        self.current_runlevel = target;
    }
}

impl Default for SysVinitManager {
    fn default() -> Self {
        Self::new()
    }
}

/// AptPackageMetadata tracks Debian APT package fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AptPackageMetadata {
    pub package_name: String,
    pub version: String,
    pub depends: Vec<String>,
    pub sha256: String,
}

/// AptRepositorySynchronizer emulates APT GPG-verified repository indexing.
pub struct AptRepositorySynchronizer {
    pub registered_packages: BTreeMap<String, AptPackageMetadata>,
    pub local_package_cache: BTreeMap<String, AptPackageMetadata>,
    pub gpg_keys_trusted: Vec<String>,
    pub sync_completed: bool,
}

impl AptRepositorySynchronizer {
    pub fn new() -> Self {
        Self {
            registered_packages: BTreeMap::new(),
            local_package_cache: BTreeMap::new(),
            gpg_keys_trusted: Vec::new(),
            sync_completed: false,
        }
    }

    pub fn add_trusted_gpg_key(&mut self, key_fingerprint: &str) {
        self.gpg_keys_trusted.push(key_fingerprint.to_string());
    }

    pub fn register_repo_package(&mut self, metadata: AptPackageMetadata) {
        self.registered_packages.insert(metadata.package_name.clone(), metadata);
    }

    pub fn sync_indices(&mut self, signing_key_fingerprint: &str) -> Result<usize, &'static str> {
        if !self.gpg_keys_trusted.contains(&signing_key_fingerprint.to_string()) {
            return Err("APT sync failed: Repository signature key not trusted!");
        }

        // Simulates copying verified metadata into local cache
        self.local_package_cache = self.registered_packages.clone();
        self.sync_completed = true;
        Ok(self.local_package_cache.len())
    }
}

impl Default for AptRepositorySynchronizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents an alternative provider pathway.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlternativeProvider {
    pub path: String,
    pub priority: u32,
}

/// DebianAlternativesSystem emulates update-alternatives prioritizing links.
pub struct DebianAlternativesSystem {
    pub links: BTreeMap<String, String>, // link_name -> current_selected_path
    pub providers: BTreeMap<String, Vec<AlternativeProvider>>, // link_name -> available_providers
}

impl DebianAlternativesSystem {
    pub fn new() -> Self {
        Self {
            links: BTreeMap::new(),
            providers: BTreeMap::new(),
        }
    }

    pub fn register_alternative(&mut self, link_name: &str, path: &str, priority: u32) {
        let provider = AlternativeProvider {
            path: path.to_string(),
            priority,
        };

        if let Some(list) = self.providers.get_mut(&link_name.to_string()) {
            list.push(provider);
            // Sort in descending order of priority
            list.sort_by(|a, b| b.priority.cmp(&a.priority));
        } else {
            self.providers.insert(link_name.to_string(), vec![provider]);
        }

        self.auto_select(link_name);
    }

    pub fn auto_select(&mut self, link_name: &str) {
        if let Some(list) = self.providers.get(&link_name.to_string()) {
            if let Some(best) = list.first() {
                self.links.insert(link_name.to_string(), best.path.clone());
            }
        }
    }

    pub fn set_manual_alternative(&mut self, link_name: &str, path: &str) -> Result<(), &'static str> {
        if let Some(list) = self.providers.get(&link_name.to_string()) {
            if list.iter().any(|p| p.path == path) {
                self.links.insert(link_name.to_string(), path.to_string());
                Ok(())
            } else {
                Err("Path is not a valid registered provider for this alternative link")
            }
        } else {
            Err("Alternative link not registered")
        }
    }

    pub fn resolve_link(&self, link_name: &str) -> Option<String> {
        self.links.get(link_name).cloned()
    }
}

impl Default for DebianAlternativesSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// DebootstrapEngine emulates bootstrapping a base Debian root filesystem.
pub struct DebootstrapEngine {
    pub bootstrap_suite: String,
    pub unpacked_packages: Vec<String>,
    pub essential_installed: bool,
}

impl DebootstrapEngine {
    pub fn new(suite: &str) -> Self {
        Self {
            bootstrap_suite: suite.to_string(),
            unpacked_packages: Vec::new(),
            essential_installed: false,
        }
    }

    pub fn download_and_unpack_essentials(&mut self) -> Result<usize, &'static str> {
        if self.bootstrap_suite.is_empty() {
            return Err("Target suite not configured");
        }

        // Simulate debian base system standard essential list
        self.unpacked_packages.push("base-files".to_string());
        self.unpacked_packages.push("base-passwd".to_string());
        self.unpacked_packages.push("bash".to_string());
        self.unpacked_packages.push("coreutils".to_string());
        self.unpacked_packages.push("dpkg".to_string());
        self.unpacked_packages.push("libc6".to_string());

        self.essential_installed = true;
        Ok(self.unpacked_packages.len())
    }

    pub fn verify_target_rootfs(&self) -> bool {
        self.essential_installed && self.unpacked_packages.contains(&"dpkg".to_string())
    }
}

// =========================================================================
// UNIT TESTS MODULE
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sysvinit_runlevel_switching() {
        let mut sysv = SysVinitManager::new();
        assert_eq!(sysv.current_runlevel, SysVinitRunlevel::Level2);

        // Switch to reboot runlevel (Level6)
        sysv.switch_runlevel(SysVinitRunlevel::Level6);
        assert_eq!(sysv.current_runlevel, SysVinitRunlevel::Level6);
        assert_eq!(sysv.execution_log.len(), 3); // RC shutdown scripts
        assert_eq!(sysv.execution_log[0], "stop:Level2:sendsigs");
        assert_eq!(sysv.execution_log[2], "stop:Level2:reboot");
    }

    #[test]
    fn test_apt_repository_sync() {
        let mut sync = AptRepositorySynchronizer::new();
        sync.register_repo_package(AptPackageMetadata {
            package_name: "nginx".to_string(),
            version: "1.24.0-1~deb12u1".to_string(),
            depends: vec!["libc6".to_string(), "libssl3".to_string()],
            sha256: "sha256_mock_nginx_deb_bytes".to_string(),
        });

        // Sync without GPG key validation (fails)
        assert!(sync.sync_indices("0xDEB12345").is_err());

        // Trust GPG key and sync (succeeds)
        sync.add_trusted_gpg_key("0xDEB12345");
        let count = sync.sync_indices("0xDEB12345").unwrap();
        assert_eq!(count, 1);
        assert!(sync.sync_completed);
        assert_eq!(
            sync.local_package_cache.get("nginx").unwrap().version,
            "1.24.0-1~deb12u1"
        );
    }

    #[test]
    fn test_debian_alternatives_system() {
        let mut alt = DebianAlternativesSystem::new();

        // Register default nano editor with low priority
        alt.register_alternative("editor", "/usr/bin/nano", 10);
        assert_eq!(alt.resolve_link("editor").unwrap(), "/usr/bin/nano");

        // Register vim editor with higher priority (auto-swaps)
        alt.register_alternative("editor", "/usr/bin/vim", 50);
        assert_eq!(alt.resolve_link("editor").unwrap(), "/usr/bin/vim");

        // Set manual alternative
        assert!(alt.set_manual_alternative("editor", "/usr/bin/nano").is_ok());
        assert_eq!(alt.resolve_link("editor").unwrap(), "/usr/bin/nano");

        // Set invalid manual path
        assert!(alt.set_manual_alternative("editor", "/usr/bin/emacs").is_err());
    }

    #[test]
    fn test_debootstrap_engine() {
        let mut debootstrap = DebootstrapEngine::new("bookworm");
        assert!(!debootstrap.verify_target_rootfs());

        let count = debootstrap.download_and_unpack_essentials().unwrap();
        assert_eq!(count, 6);
        assert!(debootstrap.verify_target_rootfs());
    }
}
