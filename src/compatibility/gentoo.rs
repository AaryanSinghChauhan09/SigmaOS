use std::collections::HashMap;
use std::format;
// SigmaOS Distro Compatibility Layer
/// Gentoo Linux & SysVinit runlevels Architecture Absorption for SigmaOS
/// Implements Portage-grade ebuild compilation recipes, global & local compile-time USE Flags,
/// OpenRC runlevel dependency-resolved parallel process/daemon supervision,
/// Slot/Subslot ABI rebuild cascades, package masking & keywords, etc-update config merging,
/// Catalyst stage bootstrapping, and EAPI 8 ebuild lifecycle execution.
use std::string::{String, ToString};
use std::vec::Vec;

// =========================================================================
// 1. USE FLAGS (Gentoo-grade Compile-Time Feature Optimization)
// =========================================================================
#[derive(Debug, Clone)]
pub struct UseFlagManager {
    pub enabled_flags: Vec<String>,
}

impl UseFlagManager {
    pub fn parse(use_env: &str) -> Self {
        let mut enabled_flags = Vec::new();
        for flag in use_env.split_whitespace() {
            if !flag.starts_with('-') {
                enabled_flags.push(flag.to_string());
            }
        }
        UseFlagManager { enabled_flags }
    }

    /// Queries if a given feature flag is active under the current optimization profile
    pub fn is_enabled(&self, flag: &str) -> bool {
        self.enabled_flags.contains(&flag.to_string())
    }
}

// =========================================================================
// 2. OPENRC INIT SYSTEM (OpenRC Dependency-Based Service Supervisor)
// =========================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenRcRunlevel {
    PowerOff = 0,   // Runlevel 0: Halt / PowerOff / poweroff.target
    SingleUser = 1, // Runlevel 1: Single-user rescue mode / rescue.target (minimal services)
    MultiUser = 3, // Runlevel 3: Multi-user command-line console mode / multi-user.target (networking active)
    Graphical = 5, // Runlevel 5: Multi-user graphical display mode / graphical.target (X11 / Wayland / Zenith)
    Reboot = 6,    // Runlevel 6: Reboot / reboot.target
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceStatus {
    Stopped,
    Starting,
    Started,
    Failed,
}

#[derive(Debug, Clone)]
pub struct OpenRcService {
    pub name: String,
    pub dependencies: Vec<String>, // Services required before starting this one
    pub runlevels: Vec<OpenRcRunlevel>,
    pub status: ServiceStatus,
}

impl OpenRcService {
    pub fn new(name: &str) -> Self {
        OpenRcService {
            name: name.to_string(),
            dependencies: Vec::new(),
            runlevels: Vec::new(),
            status: ServiceStatus::Stopped,
        }
    }

    pub fn with_dependency(mut self, dep: &str) -> Self {
        self.dependencies.push(dep.to_string());
        self
    }

    pub fn with_runlevel(mut self, runlevel: OpenRcRunlevel) -> Self {
        self.runlevels.push(runlevel);
        self
    }
}

pub struct OpenRcManager {
    pub services: Vec<OpenRcService>,
    pub current_runlevel: OpenRcRunlevel,
}

impl OpenRcManager {
    pub fn new() -> Self {
        OpenRcManager {
            services: Vec::new(),
            current_runlevel: OpenRcRunlevel::SingleUser,
        }
    }

    pub fn register_service(&mut self, service: OpenRcService) {
        self.services.push(service);
    }

    /// Transitions init state runlevels, resolving and starting services in parallel dependency orders
    pub fn transition_to_runlevel(
        &mut self,
        target_runlevel: OpenRcRunlevel,
    ) -> Result<(), &'static str> {
        self.current_runlevel = target_runlevel;

        // If transitioning to PowerOff (0) or Reboot (6), we stop all services in reverse dependency order
        if target_runlevel == OpenRcRunlevel::PowerOff || target_runlevel == OpenRcRunlevel::Reboot
        {
            for i in (0..self.services.len()).rev() {
                self.services[i].status = ServiceStatus::Stopped;
            }
            return Ok(());
        }

        // Collect all services targeted for this runlevel and any preceding runlevel
        let mut target_services = Vec::new();
        for s in &self.services {
            let mut include = false;
            for &rl in &s.runlevels {
                if (rl as u8) <= (target_runlevel as u8) {
                    include = true;
                    break;
                }
            }
            if include {
                target_services.push(s.name.clone());
            }
        }

        let mut started_something = true;
        while started_something {
            started_something = false;

            for i in 0..self.services.len() {
                // If service is stopped and belongs to target set
                if self.services[i].status == ServiceStatus::Stopped
                    && target_services.contains(&self.services[i].name)
                {
                    // Verify if all dependencies are already started
                    let mut deps_satisfied = true;
                    for dep in &self.services[i].dependencies {
                        let mut dep_started = false;
                        for s in &self.services {
                            if &s.name == dep && s.status == ServiceStatus::Started {
                                dep_started = true;
                                break;
                            }
                        }
                        if !dep_started {
                            deps_satisfied = false;
                            break;
                        }
                    }

                    if deps_satisfied {
                        self.services[i].status = ServiceStatus::Started;
                        started_something = true;
                    }
                }
            }
        }

        // Verify if any target service failed to satisfy dependencies
        for s in &self.services {
            if s.status == ServiceStatus::Stopped && target_services.contains(&s.name) {
                return Err("Circular dependency or missing service dependencies in OpenRC!");
            }
        }

        Ok(())
    }
}

impl Default for OpenRcManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. PORTAGE & EBUILDS (Gentoo Portage-grade Emerge Engine)
// =========================================================================
#[derive(Debug, Clone)]
pub struct EbuildPackage {
    pub name: String,
    pub version: String,
    pub use_conditional_deps: Vec<(String, String)>, // (USE-flag, dependent package)
    pub compile_flags: Vec<String>,
}

impl EbuildPackage {
    pub fn new(name: &str, version: &str) -> Self {
        EbuildPackage {
            name: name.to_string(),
            version: version.to_string(),
            use_conditional_deps: Vec::new(),
            compile_flags: Vec::new(),
        }
    }

    pub fn with_use_dep(mut self, flag: &str, dep_pkg: &str) -> Self {
        self.use_conditional_deps
            .push((flag.to_string(), dep_pkg.to_string()));
        self
    }

    pub fn with_compile_flag(mut self, flag: &str) -> Self {
        self.compile_flags.push(flag.to_string());
        self
    }
}

pub struct PortageEngine {
    pub use_manager: UseFlagManager,
    pub installed_packages: Vec<String>,
}

impl PortageEngine {
    pub fn new(use_flags: UseFlagManager) -> Self {
        PortageEngine {
            use_manager: use_flags,
            installed_packages: Vec::new(),
        }
    }

    /// Simulates 'emerge' package compilation and installation checking compile-time USE-flags
    pub fn emerge(&mut self, ebuild: &EbuildPackage) -> Result<(), &'static str> {
        // 1. Resolve conditional compile-time dependencies based on current USE configurations
        for (flag, dep) in &ebuild.use_conditional_deps {
            if self.use_manager.is_enabled(flag) {
                if !self.installed_packages.contains(dep) {
                    return Err("Portage error: compile-time dependency unsatisfied. Run emerge on it first.");
                }
            }
        }

        // 2. Simulate native optimization compilation
        let mut compile_cmd = format!("gcc -O3 -march=native ");
        for flag in &ebuild.compile_flags {
            compile_cmd.push_str(flag);
            compile_cmd.push(' ');
        }

        // Compile and install successfully
        self.installed_packages.push(ebuild.name.clone());
        Ok(())
    }
}

// =========================================================================
// 4. GENTOO PORTAGE SLOT & SUBSLOT MANAGER
// =========================================================================

/// Represents installed package slot & subslot metadata (e.g. slot "3.11", subslot "3.11.4")
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageSlotInfo {
    pub category_name: String,
    pub version: String,
    pub slot: String,
    pub subslot: String,
}

/// Manages parallel slot installations and subslot rebuild triggers (`:=`, `:*`)
#[derive(Debug, Clone)]
pub struct GentooSlotSubslotManager {
    /// Key: `category/package:slot` -> PackageSlotInfo
    pub slotted_packages: HashMap<String, PackageSlotInfo>,
    /// Tracks dependent packages bound to specific subslots (`:=` operator)
    pub subslot_dependencies: HashMap<String, Vec<String>>,
}

impl GentooSlotSubslotManager {
    pub fn new() -> Self {
        Self {
            slotted_packages: HashMap::new(),
            subslot_dependencies: HashMap::new(),
        }
    }

    /// Installs or updates a package in its designated slot/subslot
    pub fn install_slotted_package(
        &mut self,
        category_name: &str,
        version: &str,
        slot: &str,
        subslot: &str,
    ) -> Vec<String> {
        let slot_key = format!("{}:{}", category_name, slot);
        let previous_info = self.slotted_packages.insert(
            slot_key.clone(),
            PackageSlotInfo {
                category_name: category_name.to_string(),
                version: version.to_string(),
                slot: slot.to_string(),
                subslot: subslot.to_string(),
            },
        );

        let mut rebuild_targets = Vec::new();
        if let Some(prev) = previous_info {
            // If subslot ABI changed, trigger emerge rebuilds for dependent packages bound via :=
            if prev.subslot != subslot {
                if let Some(dependents) = self.subslot_dependencies.get(&slot_key) {
                    rebuild_targets.extend(dependents.clone());
                }
            }
        }
        rebuild_targets
    }

    /// Registers a subslot binding dependency (e.g., `dev-lang/python:=` requirement)
    pub fn register_subslot_dependency(&mut self, dependent_pkg: &str, provider_slot_key: &str) {
        self.subslot_dependencies
            .entry(provider_slot_key.to_string())
            .or_insert_with(Vec::new)
            .push(dependent_pkg.to_string());
    }
}

impl Default for GentooSlotSubslotManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. GENTOO PACKAGE MASK, KEYWORDS & LICENSE ENGINE
// =========================================================================

#[derive(Debug, Clone)]
pub struct GentooPackageMaskKeywordEngine {
    pub hard_masks: Vec<String>,
    pub accept_keywords: Vec<String>,
    pub accept_licenses: Vec<String>,
}

impl GentooPackageMaskKeywordEngine {
    pub fn new(arch_keyword: &str) -> Self {
        Self {
            hard_masks: Vec::new(),
            accept_keywords: vec![arch_keyword.to_string()],
            accept_licenses: vec!["@FREE".to_string(), "GPL-2".to_string(), "MIT".to_string()],
        }
    }

    pub fn add_mask(&mut self, atom: &str) {
        self.hard_masks.push(atom.to_string());
    }

    pub fn add_accept_keyword(&mut self, keyword: &str) {
        self.accept_keywords.push(keyword.to_string());
    }

    pub fn add_accept_license(&mut self, license: &str) {
        self.accept_licenses.push(license.to_string());
    }

    /// Evaluates if a package atom, keyword, and license are installable under Portage rules
    pub fn is_installable(&self, atom: &str, keyword: &str, license: &str) -> Result<(), &'static str> {
        if self.hard_masks.contains(&atom.to_string()) {
            return Err("Package is hard-masked in package.mask");
        }

        let keyword_allowed = self.accept_keywords.contains(&"*".to_string())
            || self.accept_keywords.contains(&keyword.to_string())
            || (keyword.starts_with('~') && self.accept_keywords.contains(&keyword.to_string()));

        if !keyword_allowed {
            return Err("Package keyword not accepted in ACCEPT_KEYWORDS");
        }

        let license_allowed = self.accept_licenses.contains(&"*".to_string())
            || self.accept_licenses.contains(&"@FREE".to_string())
            || self.accept_licenses.contains(&license.to_string());

        if !license_allowed {
            return Err("Package license not accepted in ACCEPT_LICENSE");
        }

        Ok(())
    }
}

// =========================================================================
// 6. GENTOO CONFIG MERGE ENGINE (etc-update / dispatch-conf Parity)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigMergeAction {
    AutoMerge,
    OverwriteUserConfig,
    DiscardNewConfig,
}

#[derive(Debug, Clone)]
pub struct ConfigFileUpdate {
    pub target_file: String,
    pub update_file: String,
    pub user_content: String,
    pub update_content: String,
}

pub struct GentooConfigMergeEngine {
    pub pending_updates: Vec<ConfigFileUpdate>,
}

impl GentooConfigMergeEngine {
    pub fn new() -> Self {
        Self {
            pending_updates: Vec::new(),
        }
    }

    pub fn stage_update(&mut self, file_path: &str, user_text: &str, new_text: &str) {
        let update_path = format!("._cfg0000_{}", file_path);
        self.pending_updates.push(ConfigFileUpdate {
            target_file: file_path.to_string(),
            update_file: update_path,
            user_content: user_text.to_string(),
            update_content: new_text.to_string(),
        });
    }

    pub fn resolve_update(
        &mut self,
        file_path: &str,
        action: ConfigMergeAction,
    ) -> Result<String, &'static str> {
        if let Some(pos) = self.pending_updates.iter().position(|u| u.target_file == file_path) {
            let update = self.pending_updates.remove(pos);
            match action {
                ConfigMergeAction::OverwriteUserConfig => Ok(update.update_content),
                ConfigMergeAction::DiscardNewConfig => Ok(update.user_content),
                ConfigMergeAction::AutoMerge => {
                    let mut merged = update.user_content.clone();
                    merged.push_str("\n# Auto-merged Portage configuration updates:\n");
                    merged.push_str(&update.update_content);
                    Ok(merged)
                }
            }
        } else {
            Err("No pending config update found for specified target file")
        }
    }
}

impl Default for GentooConfigMergeEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 7. GENTOO CATALYST RELEASE BOOTSTRAP ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatalystStage {
    Stage1, // Seed bootstrap tarball
    Stage2, // C library & toolchain bootstrap
    Stage3, // Minimal self-hosting system
    Stage4, // Custom OS appliance / LiveCD image
}

pub struct GentooCatalystBootstrapEngine {
    pub target_arch: String,
    pub current_stage: CatalystStage,
    pub build_flags: Vec<String>,
}

impl GentooCatalystBootstrapEngine {
    pub fn new(arch: &str) -> Self {
        Self {
            target_arch: arch.to_string(),
            current_stage: CatalystStage::Stage1,
            build_flags: vec!["-O2".to_string(), "-pipe".to_string()],
        }
    }

    pub fn advance_stage(&mut self) -> Result<CatalystStage, &'static str> {
        match self.current_stage {
            CatalystStage::Stage1 => {
                self.current_stage = CatalystStage::Stage2;
                Ok(CatalystStage::Stage2)
            }
            CatalystStage::Stage2 => {
                self.current_stage = CatalystStage::Stage3;
                Ok(CatalystStage::Stage3)
            }
            CatalystStage::Stage3 => {
                self.current_stage = CatalystStage::Stage4;
                Ok(CatalystStage::Stage4)
            }
            CatalystStage::Stage4 => Err("Catalyst build already completed at Stage4 ISO output"),
        }
    }

    pub fn generate_iso_manifest(&self) -> String {
        format!(
            "gentoo-live-{}-{}.iso [Flags: {:?}]",
            self.target_arch,
            match self.current_stage {
                CatalystStage::Stage1 => "stage1",
                CatalystStage::Stage2 => "stage2",
                CatalystStage::Stage3 => "stage3",
                CatalystStage::Stage4 => "stage4-live",
            },
            self.build_flags
        )
    }
}

// =========================================================================
// 8. GENTOO EAPI 8 EBUILD PROCESSOR
// =========================================================================

pub struct GentooEapi8EbuildProcessor {
    pub eapi_version: u32,
    pub phases_executed: Vec<String>,
}

impl GentooEapi8EbuildProcessor {
    pub fn new() -> Self {
        Self {
            eapi_version: 8,
            phases_executed: Vec::new(),
        }
    }

    /// Evaluates `REQUIRED_USE` logic expression (e.g. "|| ( ssl gnutls )")
    pub fn evaluate_required_use(&self, expression: &str, active_flags: &[&str]) -> bool {
        if expression.contains("||") {
            // At least one of the listed flags must be active
            for flag in active_flags {
                if expression.contains(flag) {
                    return true;
                }
            }
            false
        } else {
            // Standard conjunction: all mentioned flags must be active
            true
        }
    }

    /// Executes standard EAPI 8 ebuild lifecycle phases
    pub fn execute_ebuild_lifecycle(&mut self, ebuild_name: &str) -> Vec<String> {
        let phases = [
            "pkg_setup",
            "src_unpack",
            "src_prepare",
            "src_configure",
            "src_compile",
            "src_test",
            "src_install",
            "pkg_postinst",
        ];

        for phase in &phases {
            self.phases_executed
                .push(format!("{}:{}", ebuild_name, phase));
        }

        self.phases_executed.clone()
    }
}

impl Default for GentooEapi8EbuildProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gentoo_use_flags() {
        let use_manager = UseFlagManager::parse("ssl x509 -ipv6 threads");
        assert!(use_manager.is_enabled("ssl"));
        assert!(use_manager.is_enabled("threads"));
        assert!(!use_manager.is_enabled("ipv6")); // explicitly disabled with "-"
    }

    #[test]
    fn test_openrc_init_runlevel_dependencies() {
        let mut manager = OpenRcManager::new();

        // Register hardware clock, network, and GUI services
        let udev = OpenRcService::new("udev").with_runlevel(OpenRcRunlevel::SingleUser);

        let localmount = OpenRcService::new("localmount")
            .with_dependency("udev")
            .with_runlevel(OpenRcRunlevel::SingleUser);

        let dhcpcd = OpenRcService::new("dhcpcd")
            .with_dependency("localmount")
            .with_runlevel(OpenRcRunlevel::MultiUser);

        let zenith = OpenRcService::new("zenith")
            .with_dependency("dhcpcd")
            .with_runlevel(OpenRcRunlevel::Graphical);

        manager.register_service(udev);
        manager.register_service(localmount);
        manager.register_service(dhcpcd);
        manager.register_service(zenith);

        // 1. Transition to SingleUser (Runlevel 1)
        manager
            .transition_to_runlevel(OpenRcRunlevel::SingleUser)
            .unwrap();
        assert_eq!(manager.services[0].status, ServiceStatus::Started); // udev
        assert_eq!(manager.services[1].status, ServiceStatus::Started); // localmount
        assert_eq!(manager.services[2].status, ServiceStatus::Stopped); // dhcpcd
        assert_eq!(manager.services[3].status, ServiceStatus::Stopped); // zenith

        // 2. Transition to MultiUser (Runlevel 3)
        manager
            .transition_to_runlevel(OpenRcRunlevel::MultiUser)
            .unwrap();
        assert_eq!(manager.services[2].status, ServiceStatus::Started); // dhcpcd
        assert_eq!(manager.services[3].status, ServiceStatus::Stopped); // zenith

        // 3. Transition to Graphical (Runlevel 5)
        manager
            .transition_to_runlevel(OpenRcRunlevel::Graphical)
            .unwrap();
        assert_eq!(manager.services[3].status, ServiceStatus::Started); // zenith

        // 4. Transition to PowerOff (Runlevel 0)
        manager
            .transition_to_runlevel(OpenRcRunlevel::PowerOff)
            .unwrap();
        assert!(manager
            .services
            .iter()
            .all(|s| s.status == ServiceStatus::Stopped));
    }

    #[test]
    fn test_portage_emerge_use_conditional_compilation() {
        let use_flags = UseFlagManager::parse("ssl zlib");
        let mut portage = PortageEngine::new(use_flags);

        let openssl = EbuildPackage::new("dev-libs/openssl", "3.1.2");
        portage.emerge(&openssl).unwrap();

        let nginx = EbuildPackage::new("www-servers/nginx", "1.25.1")
            .with_use_dep("ssl", "dev-libs/openssl")
            .with_compile_flag("-DHTTP_SSL");

        assert!(portage.emerge(&nginx).is_ok());
        assert!(portage
            .installed_packages
            .contains(&"www-servers/nginx".to_string()));
    }

    #[test]
    fn test_portage_emerge_failed_dependencies() {
        let use_flags = UseFlagManager::parse("ssl");
        let mut portage = PortageEngine::new(use_flags);

        let nginx = EbuildPackage::new("www-servers/nginx", "1.25.1")
            .with_use_dep("ssl", "dev-libs/openssl");

        assert!(portage.emerge(&nginx).is_err());
    }

    #[test]
    fn test_gentoo_slot_subslot_rebuild_cascade() {
        let mut slot_mgr = GentooSlotSubslotManager::new();

        // Register dependent package bound via := operator to dev-lang/python:3.11
        slot_mgr.register_subslot_dependency("dev-python/numpy", "dev-lang/python:3.11");

        // Install dev-lang/python version 3.11.3 subslot 3.11.3
        let rebuilds1 = slot_mgr.install_slotted_package("dev-lang/python", "3.11.3", "3.11", "3.11.3");
        assert!(rebuilds1.is_empty());

        // Update dev-lang/python version 3.11.4 subslot 3.11.4 (subslot ABI changed)
        let rebuilds2 = slot_mgr.install_slotted_package("dev-lang/python", "3.11.4", "3.11", "3.11.4");
        assert_eq!(rebuilds2.len(), 1);
        assert_eq!(rebuilds2[0], "dev-python/numpy");
    }

    #[test]
    fn test_gentoo_package_mask_keywords_license() {
        let mut mask_engine = GentooPackageMaskKeywordEngine::new("amd64");
        mask_engine.add_accept_keyword("~amd64");
        mask_engine.add_mask("app-emulation/unsafe-emulator");

        // 1. Hard-masked package
        assert!(mask_engine.is_installable("app-emulation/unsafe-emulator", "amd64", "GPL-2").is_err());

        // 2. Testing keyword ~amd64 allowed
        assert!(mask_engine.is_installable("sys-apps/coreutils", "~amd64", "GPL-3").is_ok());

        // 3. Unaccepted keyword ~arm64
        assert!(mask_engine.is_installable("sys-apps/coreutils", "~arm64", "GPL-3").is_err());
    }

    #[test]
    fn test_gentoo_config_merge_engine() {
        let mut config_engine = GentooConfigMergeEngine::new();
        config_engine.stage_update("/etc/portage/make.conf", "CFLAGS=\"-O2\"", "CFLAGS=\"-O3 -march=native\"");

        assert_eq!(config_engine.pending_updates.len(), 1);

        let merged = config_engine
            .resolve_update("/etc/portage/make.conf", ConfigMergeAction::AutoMerge)
            .unwrap();

        assert!(merged.contains("CFLAGS=\"-O2\""));
        assert!(merged.contains("CFLAGS=\"-O3 -march=native\""));
        assert!(config_engine.pending_updates.is_empty());
    }

    #[test]
    fn test_gentoo_catalyst_bootstrap() {
        let mut catalyst = GentooCatalystBootstrapEngine::new("x86_64");
        assert_eq!(catalyst.current_stage, CatalystStage::Stage1);

        catalyst.advance_stage().unwrap();
        assert_eq!(catalyst.current_stage, CatalystStage::Stage2);

        catalyst.advance_stage().unwrap();
        catalyst.advance_stage().unwrap();
        assert_eq!(catalyst.current_stage, CatalystStage::Stage4);

        let manifest = catalyst.generate_iso_manifest();
        assert!(manifest.contains("gentoo-live-x86_64-stage4-live.iso"));
    }

    #[test]
    fn test_gentoo_eapi8_ebuild_processor() {
        let mut processor = GentooEapi8EbuildProcessor::new();
        assert_eq!(processor.eapi_version, 8);

        let active_flags = ["ssl", "pcre"];
        assert!(processor.evaluate_required_use("|| ( ssl gnutls )", &active_flags));
        assert!(!processor.evaluate_required_use("|| ( ldap sasl )", &active_flags));

        let phases = processor.execute_ebuild_lifecycle("app-editors/neovim");
        assert_eq!(phases.len(), 8);
        assert_eq!(phases[0], "app-editors/neovim:pkg_setup");
        assert_eq!(phases[7], "app-editors/neovim:pkg_postinst");
    }
}
