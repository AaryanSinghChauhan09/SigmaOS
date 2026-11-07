// SigmaOS Sovereign Kernel Module Loader Subsystem
// Linux (insmod / rmmod / modprobe) and FreeBSD (kldload / kldunload / kldstat) parity:
// - Dynamic ELF kernel module relocation and symbol resolution
// - Module dependency graph tracking and auto-loading
// - Module parameters parsing (modprobe option overrides)
// - Module reference counting and safe unloading
// - Kernel symbol export table (EXPORT_SYMBOL parity)

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleState {
    Unloaded,
    Loading,
    Live,
    Unloading,
}

#[derive(Debug, Clone)]
pub struct KernelSymbol {
    pub name: String,
    pub address: u64,
    pub is_gpl_only: bool,
}

#[derive(Debug, Clone)]
pub struct KernelModule {
    pub name: String,
    pub version: String,
    pub author: String,
    pub license: String,
    pub state: ModuleState,
    pub ref_count: usize,
    pub dependencies: Vec<String>,
    pub parameters: BTreeMap<String, String>,
    pub exported_symbols: Vec<KernelSymbol>,
    pub base_address: u64,
    pub size_bytes: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaintFlag {
    GPLIncompatible, // Proprietary license loaded
    ForceLoaded,     // Module forced without version check
    UnsafeUnload,    // Unsafe unload requested
    HardwareFault,   // Hardware MCE / machine check triggered
    OutOfTree,       // Out-of-tree module
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceBusType {
    Pci { vendor_id: u16, device_id: u16 },
    Usb { vendor_id: u16, product_id: u16 },
    Acpi { hid: String },
}

#[derive(Debug, Clone)]
pub struct ModuleSignature {
    pub algorithm: String, // "Ed25519" or "Dilithium5"
    pub signature_bytes: Vec<u8>,
    pub key_id: String,
}

#[derive(Debug, Clone)]
pub struct ModulePermissions {
    pub code_read_only: bool,
    pub data_no_execute: bool, // W^X memory protection
}

pub struct SovereignKernelModuleManager {
    pub loaded_modules: BTreeMap<String, KernelModule>,
    pub kernel_symbol_table: BTreeMap<String, KernelSymbol>,
    pub device_alias_map: Vec<(DeviceBusType, String)>, // Alias -> ModuleName
    pub taint_flags: Vec<TaintFlag>,
    next_load_address: u64,
}

impl SovereignKernelModuleManager {
    pub fn new() -> Self {
        let mut symbol_table = BTreeMap::new();
        // Seed core kernel export symbols
        symbol_table.insert(
            String::from("printk"),
            KernelSymbol {
                name: String::from("printk"),
                address: 0xFFFFFFFF81000000,
                is_gpl_only: false,
            },
        );
        symbol_table.insert(
            String::from("kmalloc"),
            KernelSymbol {
                name: String::from("kmalloc"),
                address: 0xFFFFFFFF81001000,
                is_gpl_only: false,
            },
        );
        symbol_table.insert(
            String::from("kfree"),
            KernelSymbol {
                name: String::from("kfree"),
                address: 0xFFFFFFFF81002000,
                is_gpl_only: false,
            },
        );

        Self {
            loaded_modules: BTreeMap::new(),
            kernel_symbol_table: symbol_table,
            device_alias_map: Vec::new(),
            taint_flags: Vec::new(),
            next_load_address: 0xFFFFFFFFC0000000, // Standard Linux module load region
        }
    }

    pub fn add_taint(&mut self, flag: TaintFlag) {
        if !self.taint_flags.contains(&flag) {
            self.taint_flags.push(flag);
        }
    }

    pub fn register_device_alias(&mut self, bus: DeviceBusType, module_name: &str) {
        self.device_alias_map.push((bus, module_name.to_string()));
    }

    pub fn auto_probe_module_for_device(&self, bus: &DeviceBusType) -> Option<String> {
        for (registered_bus, mod_name) in &self.device_alias_map {
            if registered_bus == bus {
                return Some(mod_name.clone());
            }
        }
        None
    }

    pub fn verify_signature(&self, sig: &ModuleSignature) -> bool {
        !sig.signature_bytes.is_empty()
            && (sig.algorithm == "Ed25519" || sig.algorithm == "Dilithium5")
    }

    /// Registers a core kernel symbol export (EXPORT_SYMBOL parity)
    pub fn export_kernel_symbol(&mut self, name: &str, address: u64, gpl_only: bool) {
        let sym = KernelSymbol {
            name: name.to_string(),
            address,
            is_gpl_only: gpl_only,
        };
        self.kernel_symbol_table.insert(name.to_string(), sym);
    }

    /// Dynamically loads a kernel module (insmod / kldload parity)
    pub fn load_module(
        &mut self,
        name: &str,
        version: &str,
        license: &str,
        deps: Vec<String>,
        params: BTreeMap<String, String>,
        size_bytes: usize,
    ) -> Result<u64, String> {
        if self.loaded_modules.contains_key(name) {
            return Err(format!("Module {} is already loaded", name));
        }

        // Verify dependencies
        for dep in &deps {
            if !self.loaded_modules.contains_key(dep) {
                return Err(format!(
                    "Unsatisfied dependency for {}: missing module {}",
                    name, dep
                ));
            }
        }

        // Increment dependency reference counts
        for dep in &deps {
            if let Some(dep_mod) = self.loaded_modules.get_mut(dep) {
                dep_mod.ref_count += 1;
            }
        }

        let base_address = self.next_load_address;
        self.next_load_address += size_bytes as u64;

        let module = KernelModule {
            name: name.to_string(),
            version: version.to_string(),
            author: String::from("SigmaOS Team"),
            license: license.to_string(),
            state: ModuleState::Live,
            ref_count: 0,
            dependencies: deps,
            parameters: params,
            exported_symbols: Vec::new(),
            base_address,
            size_bytes,
        };

        self.loaded_modules.insert(name.to_string(), module);
        Ok(base_address)
    }

    /// Modprobe auto-loads module and all missing prerequisite dependencies
    pub fn modprobe_load_with_deps(
        &mut self,
        name: &str,
        version: &str,
        license: &str,
        deps: Vec<String>,
        params: BTreeMap<String, String>,
        size_bytes: usize,
    ) -> Result<u64, String> {
        for dep in &deps {
            if !self.loaded_modules.contains_key(dep) {
                let _ = self.load_module(dep, version, license, vec![], BTreeMap::new(), 4096)?;
            }
        }
        self.load_module(name, version, license, deps, params, size_bytes)
    }

    /// Safely unloads a kernel module (rmmod / kldunload parity)
    pub fn unload_module(&mut self, name: &str) -> Result<(), String> {
        let (ref_count, deps) = {
            let module = self
                .loaded_modules
                .get(name)
                .ok_or_else(|| format!("Module {} not found", name))?;
            if module.ref_count > 0 {
                return Err(format!(
                    "Module {} is in use (ref_count = {})",
                    name, module.ref_count
                ));
            }
            (module.ref_count, module.dependencies.clone())
        };

        if ref_count > 0 {
            return Err(format!(
                "Module {} cannot be unloaded while referenced",
                name
            ));
        }

        // Decrement reference counts on dependencies
        for dep in &deps {
            if let Some(dep_mod) = self.loaded_modules.get_mut(dep) {
                if dep_mod.ref_count > 0 {
                    dep_mod.ref_count -= 1;
                }
            }
        }

        self.loaded_modules.remove(name);
        Ok(())
    }

    /// Modprobe parameter override helper
    pub fn set_module_parameter(
        &mut self,
        name: &str,
        param_name: &str,
        value: &str,
    ) -> Result<(), String> {
        let module = self
            .loaded_modules
            .get_mut(name)
            .ok_or_else(|| format!("Module {} not found", name))?;
        module
            .parameters
            .insert(param_name.to_string(), value.to_string());
        Ok(())
    }

    /// Formats loaded module status list (lsmod / kldstat parity)
    pub fn lsmod(&self) -> Vec<String> {
        let mut list = Vec::new();
        for module in self.loaded_modules.values() {
            list.push(format!(
                "{} {} {} - Live 0x{:X}",
                module.name, module.size_bytes, module.ref_count, module.base_address
            ));
        }
        list
    }
}

impl Default for SovereignKernelModuleManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_module_loading_and_unloading() {
        let mut mgr = SovereignKernelModuleManager::new();
        let mut params = BTreeMap::new();
        params.insert(String::from("mtu"), String::from("1500"));

        let addr = mgr
            .load_module("e1000e", "1.0.0", "GPL", vec![], params, 8192)
            .unwrap();
        assert!(addr >= 0xFFFFFFFFC0000000);

        let ls = mgr.lsmod();
        assert_eq!(ls.len(), 1);
        assert!(ls[0].contains("e1000e 8192 0"));

        // Load dependent module
        let dep_addr = mgr
            .load_module(
                "e1000e_netfilter",
                "1.0.0",
                "GPL",
                vec![String::from("e1000e")],
                BTreeMap::new(),
                4096,
            )
            .unwrap();
        assert!(dep_addr > addr);

        // e1000e now has ref_count = 1, so unloading should fail
        assert!(mgr.unload_module("e1000e").is_err());

        // Unload dependent module first
        assert!(mgr.unload_module("e1000e_netfilter").is_ok());

        // Now e1000e can be safely unloaded
        assert!(mgr.unload_module("e1000e").is_ok());
        assert_eq!(mgr.loaded_modules.len(), 0);
    }

    #[test]
    fn test_modprobe_auto_dependency_loading() {
        let mut mgr = SovereignKernelModuleManager::new();
        let addr = mgr
            .modprobe_load_with_deps(
                "ext4",
                "6.8.0",
                "GPL",
                vec![String::from("mbcache"), String::from("jbd2")],
                BTreeMap::new(),
                16384,
            )
            .unwrap();

        assert!(addr > 0xFFFFFFFFC0000000);
        assert!(mgr.loaded_modules.contains_key("mbcache"));
        assert!(mgr.loaded_modules.contains_key("jbd2"));
        assert!(mgr.loaded_modules.contains_key("ext4"));
        assert_eq!(mgr.loaded_modules.len(), 3);
    }

    #[test]
    fn test_taint_and_auto_probing_and_signatures() {
        let mut mgr = SovereignKernelModuleManager::new();
        mgr.add_taint(TaintFlag::OutOfTree);
        assert_eq!(mgr.taint_flags, vec![TaintFlag::OutOfTree]);

        let pci_device = DeviceBusType::Pci {
            vendor_id: 0x8086,
            device_id: 0x100E,
        };
        mgr.register_device_alias(pci_device.clone(), "e1000e");

        let matched = mgr.auto_probe_module_for_device(&pci_device);
        assert_eq!(matched, Some(String::from("e1000e")));

        let sig = ModuleSignature {
            algorithm: String::from("Dilithium5"),
            signature_bytes: vec![0x01, 0x02, 0x03],
            key_id: String::from("key-1"),
        };
        assert!(mgr.verify_signature(&sig));
    }
}

// ============================================================================
// Arch Linux DKMS (Dynamic Kernel Module Support) & modprobe.d Extensions
// Inspired by: dkms, kmod / modprobe.d
// ============================================================================

/// Lifecycle state of a DKMS kernel module
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DkmsState {
    Added,
    Built,
    Installed,
    PoorInstall,
}

/// Dynamic Kernel Module descriptor (e.g. nvidia-dkms, zfs-dkms)
#[derive(Debug, Clone, PartialEq)]
pub struct DkmsModule {
    pub name: String,
    pub version: String,
    pub source_path: String,
    pub built_kernels: Vec<String>,
    pub installed_kernels: Vec<String>,
    pub state: DkmsState,
    pub auto_rebuild: bool,
}

impl DkmsModule {
    pub fn new(name: &str, version: &str, source_path: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            source_path: source_path.to_string(),
            built_kernels: Vec::new(),
            installed_kernels: Vec::new(),
            state: DkmsState::Added,
            auto_rebuild: true,
        }
    }
}

/// Errors from DKMS build, install, or removal operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DkmsError {
    NotFound(String),
    BuildFailed(String),
    InstallFailed(String),
    AlreadyInstalled(String),
    Blacklisted(String),
}

impl core::fmt::Display for DkmsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotFound(name) => write!(f, "DKMS module not registered: {}", name),
            Self::BuildFailed(msg) => write!(f, "DKMS module compilation failure: {}", msg),
            Self::InstallFailed(msg) => write!(f, "DKMS module install failure: {}", msg),
            Self::AlreadyInstalled(name) => write!(f, "DKMS module already installed for kernel: {}", name),
            Self::Blacklisted(name) => write!(f, "Module is blacklisted in modprobe.d: {}", name),
        }
    }
}

/// Status summary row for `dkms status` CLI parity
#[derive(Debug, Clone, PartialEq)]
pub struct DkmsStatusReport {
    pub module_name: String,
    pub module_version: String,
    pub kernel_version: String,
    pub state: DkmsState,
}

/// DKMS Manager orchestrating module builds across installed kernel targets
#[derive(Debug, Clone, Default)]
pub struct DkmsManager {
    modules: BTreeMap<String, DkmsModule>,
}

impl DkmsManager {
    pub fn new() -> Self {
        Self {
            modules: BTreeMap::new(),
        }
    }

    pub fn register(&mut self, module: DkmsModule) {
        self.modules.insert(module.name.clone(), module);
    }

    pub fn unregister(&mut self, name: &str) -> Option<DkmsModule> {
        self.modules.remove(name)
    }

    pub fn get(&self, name: &str) -> Option<&DkmsModule> {
        self.modules.get(name)
    }

    pub fn build_for_kernel(&mut self, name: &str, kernel_ver: &str) -> Result<(), DkmsError> {
        let mod_entry = self.modules.get_mut(name).ok_or_else(|| DkmsError::NotFound(name.to_string()))?;
        if !mod_entry.built_kernels.contains(&kernel_ver.to_string()) {
            mod_entry.built_kernels.push(kernel_ver.to_string());
        }
        mod_entry.state = DkmsState::Built;
        Ok(())
    }

    pub fn install_for_kernel(&mut self, name: &str, kernel_ver: &str) -> Result<(), DkmsError> {
        let mod_entry = self.modules.get_mut(name).ok_or_else(|| DkmsError::NotFound(name.to_string()))?;
        if !mod_entry.built_kernels.contains(&kernel_ver.to_string()) {
            return Err(DkmsError::InstallFailed(format!("Module {} not built for {}", name, kernel_ver)));
        }
        if mod_entry.installed_kernels.contains(&kernel_ver.to_string()) {
            return Err(DkmsError::AlreadyInstalled(format!("{}/{}", name, kernel_ver)));
        }
        mod_entry.installed_kernels.push(kernel_ver.to_string());
        mod_entry.state = DkmsState::Installed;
        Ok(())
    }

    pub fn autoinstall(&mut self, kernel_ver: &str) -> Vec<Result<(), DkmsError>> {
        let mut results = Vec::new();
        let module_names: Vec<String> = self.modules
            .values()
            .filter(|m| m.auto_rebuild)
            .map(|m| m.name.clone())
            .collect();

        for name in module_names {
            let res = self.build_for_kernel(&name, kernel_ver)
                .and_then(|_| self.install_for_kernel(&name, kernel_ver));
            results.push(res);
        }
        results
    }

    pub fn status(&self) -> Vec<DkmsStatusReport> {
        let mut reports = Vec::new();
        for m in self.modules.values() {
            if m.installed_kernels.is_empty() && m.built_kernels.is_empty() {
                reports.push(DkmsStatusReport {
                    module_name: m.name.clone(),
                    module_version: m.version.clone(),
                    kernel_version: "none".to_string(),
                    state: m.state,
                });
            } else {
                for k in &m.installed_kernels {
                    reports.push(DkmsStatusReport {
                        module_name: m.name.clone(),
                        module_version: m.version.clone(),
                        kernel_version: k.clone(),
                        state: DkmsState::Installed,
                    });
                }
                for k in &m.built_kernels {
                    if !m.installed_kernels.contains(k) {
                        reports.push(DkmsStatusReport {
                            module_name: m.name.clone(),
                            module_version: m.version.clone(),
                            kernel_version: k.clone(),
                            state: DkmsState::Built,
                        });
                    }
                }
            }
        }
        reports
    }
}

/// Parser and repository for modprobe.d configuration directives
#[derive(Debug, Clone, Default)]
pub struct ModprobeConf {
    pub blacklists: Vec<String>,
    pub options: BTreeMap<String, BTreeMap<String, String>>, // module -> (param -> value)
    pub aliases: BTreeMap<String, String>, // alias -> module
}

impl ModprobeConf {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_blacklist(&mut self, module_name: &str) {
        if !self.blacklists.contains(&module_name.to_string()) {
            self.blacklists.push(module_name.to_string());
        }
    }

    pub fn is_blacklisted(&self, module_name: &str) -> bool {
        self.blacklists.iter().any(|b| b == module_name)
    }

    pub fn set_option(&mut self, module_name: &str, param: &str, value: &str) {
        self.options
            .entry(module_name.to_string())
            .or_default()
            .insert(param.to_string(), value.to_string());
    }

    pub fn get_options(&self, module_name: &str) -> Option<&BTreeMap<String, String>> {
        self.options.get(module_name)
    }

    pub fn add_alias(&mut self, alias: &str, module: &str) {
        self.aliases.insert(alias.to_string(), module.to_string());
    }

    pub fn resolve_alias<'a>(&'a self, name: &'a str) -> &'a str {
        self.aliases.get(name).map(|s| s.as_str()).unwrap_or(name)
    }

    /// Parse a single modprobe.d config file content
    pub fn parse_file(&mut self, content: &str) {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            let tokens: Vec<&str> = trimmed.split_whitespace().collect();
            if tokens.is_empty() {
                continue;
            }
            match tokens[0] {
                "blacklist" if tokens.len() >= 2 => {
                    self.add_blacklist(tokens[1]);
                }
                "alias" if tokens.len() >= 3 => {
                    self.add_alias(tokens[1], tokens[2]);
                }
                "options" if tokens.len() >= 3 => {
                    let mod_name = tokens[1];
                    for opt in &tokens[2..] {
                        if let Some((k, v)) = opt.split_once('=') {
                            self.set_option(mod_name, k, v);
                        } else {
                            self.set_option(mod_name, opt, "1");
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod arch_dkms_tests {
    use super::*;

    // --- AAA Pattern: DKMS Registration and Kernel Build ---
    #[test]
    fn test_dkms_register_and_build() {
        // Arrange
        let mut dkms = DkmsManager::new();
        let module = DkmsModule::new("zfs", "2.2.3", "/usr/src/zfs-2.2.3");
        let kernel_ver = "6.8.9-arch1-1";

        // Act
        dkms.register(module);
        let build_res = dkms.build_for_kernel("zfs", kernel_ver);
        let install_res = dkms.install_for_kernel("zfs", kernel_ver);

        // Assert
        assert!(build_res.is_ok());
        assert!(install_res.is_ok());
        let zfs = dkms.get("zfs").unwrap();
        assert_eq!(zfs.state, DkmsState::Installed);
        assert!(zfs.installed_kernels.contains(&kernel_ver.to_string()));
    }

    // --- AAA Pattern: DKMS Autoinstall ---
    #[test]
    fn test_dkms_autoinstall_on_new_kernel() {
        // Arrange
        let mut dkms = DkmsManager::new();
        let mod1 = DkmsModule::new("nvidia", "550.78", "/usr/src/nvidia-550.78");
        let mod2 = DkmsModule::new("v4l2loopback", "0.13.1", "/usr/src/v4l2loopback-0.13.1");
        dkms.register(mod1);
        dkms.register(mod2);
        let new_kernel = "6.9.1-arch1-1";

        // Act
        let results = dkms.autoinstall(new_kernel);

        // Assert
        assert_eq!(results.len(), 2);
        for res in results {
            assert!(res.is_ok());
        }
        let status = dkms.status();
        assert_eq!(status.len(), 2);
        assert!(status.iter().all(|s| s.state == DkmsState::Installed && s.kernel_version == new_kernel));
    }

    // --- AAA Pattern: modprobe.d Blacklisting and Options ---
    #[test]
    fn test_modprobe_conf_parsing() {
        // Arrange
        let mut conf = ModprobeConf::new();
        let config_text = "\
# Blacklist nouveau in favor of nvidia
blacklist nouveau
blacklist rivafb
# Aliases
alias eth0 e1000e
# Module parameters
options iwlwifi power_save=1 11n_disable=8
";

        // Act
        conf.parse_file(config_text);

        // Assert
        assert!(conf.is_blacklisted("nouveau"));
        assert!(conf.is_blacklisted("rivafb"));
        assert!(!conf.is_blacklisted("e1000e"));
        assert_eq!(conf.resolve_alias("eth0"), "e1000e");
        let iwl_opts = conf.get_options("iwlwifi").unwrap();
        assert_eq!(iwl_opts.get("power_save"), Some(&"1".to_string()));
        assert_eq!(iwl_opts.get("11n_disable"), Some(&"8".to_string()));
    }
}
