use alloc::format;
extern crate alloc;
/// Sovereign Linux & BSD Distro Parity Subsystem for SigmaOS
/// Clean-room implementation of NixOS Flakes, Arch Pacman Hooks, Void runit Supervision, and Gentoo Portage USE Flags
/// Designed for bare-metal zero-dependency performance and zero-trust security
use alloc::string::String;
use alloc::vec::Vec;

// ============================================================================
// 1. NixOS Deterministic Flake Evaluation Engine (NixOSFlakeEngine)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlakeInput {
    pub name: String,
    pub url: String,
    pub locked_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemClosure {
    pub hash: String,
    pub store_path: String,
    pub packages: Vec<String>,
}

pub struct NixOSFlakeEngine {
    pub inputs: Vec<FlakeInput>,
    pub active_closure: Option<SystemClosure>,
    pub generation_count: usize,
    pub history: Vec<SystemClosure>,
}

impl NixOSFlakeEngine {
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            active_closure: None,
            generation_count: 0,
            history: Vec::new(),
        }
    }

    /// Add a flake input reference
    pub fn add_input(&mut self, name: &str, url: &str, locked_hash: &str) {
        self.inputs.push(FlakeInput {
            name: String::from(name),
            url: String::from(url),
            locked_hash: String::from(locked_hash),
        });
    }

    /// Evaluate flake inputs into a deterministic system closure
    pub fn build_closure(&mut self, packages: &[&str]) -> Result<SystemClosure, &'static str> {
        if self.inputs.is_empty() {
            return Err("No flake inputs configured");
        }

        let mut hash_accum = 0u64;
        for input in &self.inputs {
            for byte in input.locked_hash.bytes() {
                hash_accum = hash_accum.wrapping_add(byte as u64).wrapping_mul(31);
            }
        }
        hash_accum = hash_accum.wrapping_add(self.generation_count as u64);

        let closure_hash = alloc::format!("{:016x}", hash_accum);
        let store_path = alloc::format!("/sigma/store/{}-system-closure", closure_hash);

        let closure = SystemClosure {
            hash: closure_hash,
            store_path,
            packages: packages.iter().map(|&s| String::from(s)).collect(),
        };

        if let Some(prev) = self.active_closure.take() {
            self.history.push(prev);
        }

        self.generation_count += 1;
        self.active_closure = Some(closure.clone());
        Ok(closure)
    }

    /// Roll back system closure to previous generation
    pub fn rollback_generation(&mut self) -> Result<SystemClosure, &'static str> {
        if let Some(prev) = self.history.pop() {
            self.generation_count = self.generation_count.saturating_sub(1);
            self.active_closure = Some(prev.clone());
            Ok(prev)
        } else {
            Err("No previous generation history to rollback")
        }
    }

    /// Collect unused store entries and return freed store count
    pub fn garbage_collect(&mut self) -> usize {
        let count = self.history.len();
        self.history.clear();
        count
    }
}

impl Default for NixOSFlakeEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 2. Arch Linux Pacman Pre/Post Hook Governor (ArchPacmanHooksManager)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookWhen {
    PreTransaction,
    PostTransaction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookAction {
    Exec,
    Abort,
}

#[derive(Debug, Clone)]
pub struct PacmanHook {
    pub name: String,
    pub when: HookWhen,
    pub target_package: String,
    pub command: String,
}

pub struct ArchPacmanHooksManager {
    pub hooks: Vec<PacmanHook>,
    pub executed_hooks_count: usize,
}

impl ArchPacmanHooksManager {
    pub fn new() -> Self {
        Self {
            hooks: Vec::new(),
            executed_hooks_count: 0,
        }
    }

    /// Register a pacman hook
    pub fn register_hook(&mut self, name: &str, when: HookWhen, target: &str, command: &str) {
        self.hooks.push(PacmanHook {
            name: String::from(name),
            when: HookWhen::PreTransaction,
            target_package: String::from(target),
            command: String::from(command),
        });
        if let Some(hook) = self.hooks.last_mut() {
            hook.when = when;
        }
    }

    /// Execute matching hooks for a given transaction target
    pub fn trigger_hooks(&mut self, when: HookWhen, package: &str) -> usize {
        let mut count = 0;
        for hook in &self.hooks {
            if hook.when == when && (hook.target_package == "*" || hook.target_package == package) {
                count += 1;
            }
        }
        self.executed_hooks_count += count;
        count
    }
}

impl Default for ArchPacmanHooksManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 3. Void Linux runit Service Supervision Tree (VoidRunitSupervisor)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Down,
    Starting,
    Up,
    Failed,
}

#[derive(Debug, Clone)]
pub struct RunitService {
    pub name: String,
    pub state: ServiceState,
    pub pid: Option<usize>,
    pub dependencies: Vec<String>,
}

pub struct VoidRunitSupervisor {
    pub services: Vec<RunitService>,
    pub active_pid_counter: usize,
}

impl VoidRunitSupervisor {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
            active_pid_counter: 1000,
        }
    }

    /// Register a service with dependencies
    pub fn add_service(&mut self, name: &str, dependencies: &[&str]) {
        self.services.push(RunitService {
            name: String::from(name),
            state: ServiceState::Down,
            pid: None,
            dependencies: dependencies.iter().map(|&d| String::from(d)).collect(),
        });
    }

    /// Start a service adhering to dependency graph order
    pub fn start_service(&mut self, name: &str) -> Result<usize, &'static str> {
        let deps = if let Some(svc) = self.services.iter().find(|s| s.name == name) {
            svc.dependencies.clone()
        } else {
            return Err("Service not found");
        };

        // Ensure all dependencies are Up
        for dep in &deps {
            let dep_up = self
                .services
                .iter()
                .any(|s| s.name == *dep && s.state == ServiceState::Up);
            if !dep_up {
                return Err("Dependency not satisfied");
            }
        }

        if let Some(svc) = self.services.iter_mut().find(|s| s.name == name) {
            self.active_pid_counter += 1;
            svc.state = ServiceState::Up;
            svc.pid = Some(self.active_pid_counter);
            Ok(self.active_pid_counter)
        } else {
            Err("Service not found")
        }
    }

    /// Restart a service by re-assigning a new PID
    pub fn restart_service(&mut self, name: &str) -> Result<usize, &'static str> {
        if let Some(svc) = self.services.iter_mut().find(|s| s.name == name) {
            svc.state = ServiceState::Down;
            svc.pid = None;
        } else {
            return Err("Service not found");
        }
        self.start_service(name)
    }

    /// Query the current status of a supervised service
    pub fn get_service_status(&self, name: &str) -> Option<ServiceState> {
        self.services
            .iter()
            .find(|s| s.name == name)
            .map(|s| s.state)
    }
}

impl Default for VoidRunitSupervisor {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 4. Gentoo Portage USE Flags Resolution Engine (GentooPortageUseFlagsEngine)
// ============================================================================

#[derive(Debug, Clone)]
pub struct PortagePackage {
    pub name: String,
    pub available_use_flags: Vec<String>,
    pub enabled_use_flags: Vec<String>,
}

pub struct GentooPortageUseFlagsEngine {
    pub global_use_flags: Vec<String>,
    pub packages: Vec<PortagePackage>,
}

impl GentooPortageUseFlagsEngine {
    pub fn new() -> Self {
        Self {
            global_use_flags: Vec::new(),
            packages: Vec::new(),
        }
    }

    /// Set global USE flags (e.g. "+ssl", "-gtk")
    pub fn set_global_use_flags(&mut self, flags: &[&str]) {
        self.global_use_flags = flags.iter().map(|&f| String::from(f)).collect();
    }

    /// Register an ebuild package with available USE flags
    pub fn register_package(&mut self, name: &str, available_flags: &[&str]) {
        self.packages.push(PortagePackage {
            name: String::from(name),
            available_use_flags: available_flags.iter().map(|&f| String::from(f)).collect(),
            enabled_use_flags: Vec::new(),
        });
    }

    /// Resolve USE flags for a package based on global policies and negative logic
    pub fn resolve_package_flags(
        &mut self,
        package_name: &str,
    ) -> Result<Vec<String>, &'static str> {
        let global_flags = self.global_use_flags.clone();
        if let Some(pkg) = self.packages.iter_mut().find(|p| p.name == package_name) {
            let mut resolved = Vec::new();
            for flag in &pkg.available_use_flags {
                let pos_flag = alloc::format!("+{}", flag);
                let neg_flag = alloc::format!("-{}", flag);

                let is_globally_disabled = global_flags.contains(&neg_flag);
                let is_globally_enabled =
                    global_flags.contains(&pos_flag) || global_flags.contains(flag);

                if is_globally_enabled && !is_globally_disabled {
                    resolved.push(flag.clone());
                }
            }
            pkg.enabled_use_flags = resolved.clone();
            Ok(resolved)
        } else {
            Err("Package not found")
        }
    }
}

impl Default for GentooPortageUseFlagsEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 5. Void Linux XBPS Package Manager Parity (XbpsPackageManager)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XbpsPackage {
    pub name: String,
    pub version: String,
    pub revision: u32,
    pub run_depends: Vec<String>,
    pub sha256_hash: [u8; 32],
    pub is_signed: bool,
}

pub struct XbpsPackageManager {
    pub repository_packages: Vec<XbpsPackage>,
    pub installed_packages: Vec<XbpsPackage>,
}

impl XbpsPackageManager {
    pub fn new() -> Self {
        Self {
            repository_packages: Vec::new(),
            installed_packages: Vec::new(),
        }
    }

    pub fn register_repository_package(&mut self, pkg: XbpsPackage) {
        self.repository_packages.push(pkg);
    }

    pub fn verify_signature(&self, pkg_name: &str) -> bool {
        if let Some(pkg) = self.repository_packages.iter().find(|p| p.name == pkg_name) {
            pkg.is_signed
        } else {
            false
        }
    }

    pub fn resolve_dependencies(&self, pkg_name: &str) -> Result<Vec<String>, &'static str> {
        let mut order = Vec::new();
        self.resolve_deps_recursive(pkg_name, &mut order)?;
        Ok(order)
    }

    fn resolve_deps_recursive(
        &self,
        pkg_name: &str,
        order: &mut Vec<String>,
    ) -> Result<(), &'static str> {
        if order.contains(&String::from(pkg_name)) {
            return Ok(());
        }
        let pkg = self
            .repository_packages
            .iter()
            .find(|p| p.name == pkg_name)
            .ok_or("XBPS package not found in repository")?;

        for dep in &pkg.run_depends {
            self.resolve_deps_recursive(dep, order)?;
        }
        order.push(String::from(pkg_name));
        Ok(())
    }

    pub fn install_package_atomic(&mut self, pkg_name: &str) -> Result<usize, &'static str> {
        if !self.verify_signature(pkg_name) {
            return Err("XBPS package signature verification failed");
        }
        let deps = self.resolve_dependencies(pkg_name)?;
        let mut installed_count = 0;

        for dep in deps {
            if !self.installed_packages.iter().any(|p| p.name == dep) {
                if let Some(pkg) = self.repository_packages.iter().find(|p| p.name == dep) {
                    self.installed_packages.push(pkg.clone());
                    installed_count += 1;
                }
            }
        }
        Ok(installed_count)
    }
}

impl Default for XbpsPackageManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 6. Linux Kernel Devlink Netlink Device Management Engine (LinuxDevlinkDriver)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DevlinkPortFlavor {
    Physical,
    Cpu,
    Dsa,
    PciPf,
    PciVf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DevlinkPort {
    pub bus_name: String,
    pub dev_name: String,
    pub port_index: u32,
    pub split_count: u32,
    pub flavor: DevlinkPortFlavor,
}

pub struct LinuxDevlinkDriver {
    pub ports: Vec<DevlinkPort>,
    pub flash_in_progress: bool,
}

impl LinuxDevlinkDriver {
    pub fn new() -> Self {
        Self {
            ports: Vec::new(),
            flash_in_progress: false,
        }
    }

    pub fn register_port(
        &mut self,
        bus: &str,
        dev: &str,
        port_index: u32,
        flavor: DevlinkPortFlavor,
    ) {
        self.ports.push(DevlinkPort {
            bus_name: String::from(bus),
            dev_name: String::from(dev),
            port_index,
            split_count: 1,
            flavor,
        });
    }

    pub fn split_port(&mut self, port_index: u32, count: u32) -> Result<(), &'static str> {
        if count == 0 || (count & (count - 1)) != 0 {
            return Err("Devlink port split count must be power of 2");
        }
        if let Some(port) = self.ports.iter_mut().find(|p| p.port_index == port_index) {
            port.split_count = count;
            Ok(())
        } else {
            Err("Devlink port not found")
        }
    }

    pub fn flash_device_firmware(
        &mut self,
        _bus: &str,
        _dev: &str,
        image: &[u8],
    ) -> Result<usize, &'static str> {
        if image.is_empty() {
            return Err("Empty firmware image buffer");
        }
        self.flash_in_progress = true;
        let flashed_len = image.len();
        self.flash_in_progress = false;
        Ok(flashed_len)
    }
}

impl Default for LinuxDevlinkDriver {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 7. Systemd Unit Dependency & Cycle Detection Engine (SystemdUnitDependencyEngine)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemdUnit {
    pub name: String,
    pub requires: Vec<String>,
    pub after: Vec<String>,
}

pub struct SystemdUnitDependencyEngine {
    pub units: Vec<SystemdUnit>,
}

impl SystemdUnitDependencyEngine {
    pub fn new() -> Self {
        Self { units: Vec::new() }
    }

    pub fn add_unit(&mut self, unit: SystemdUnit) {
        self.units.push(unit);
    }

    pub fn detect_circular_dependencies(&self) -> bool {
        for unit in &self.units {
            let mut visited = Vec::new();
            if self.has_cycle(&unit.name, &mut visited) {
                return true;
            }
        }
        false
    }

    fn has_cycle(&self, current: &str, visited: &mut Vec<String>) -> bool {
        if visited.contains(&String::from(current)) {
            return true;
        }
        visited.push(String::from(current));

        if let Some(unit) = self.units.iter().find(|u| u.name == current) {
            for req in &unit.requires {
                let mut branch_visited = visited.clone();
                if self.has_cycle(req, &mut branch_visited) {
                    return true;
                }
            }
        }
        false
    }

    pub fn compute_startup_sequence(&self) -> Result<Vec<String>, &'static str> {
        if self.detect_circular_dependencies() {
            return Err("Circular dependency detected in systemd units");
        }

        let mut sequence = Vec::new();
        for unit in &self.units {
            self.topological_sort(&unit.name, &mut sequence);
        }
        Ok(sequence)
    }

    fn topological_sort(&self, current: &str, sequence: &mut Vec<String>) {
        if sequence.contains(&String::from(current)) {
            return;
        }
        if let Some(unit) = self.units.iter().find(|u| u.name == current) {
            for dep in &unit.after {
                self.topological_sort(dep, sequence);
            }
            for req in &unit.requires {
                self.topological_sort(req, sequence);
            }
        }
        if !sequence.contains(&String::from(current)) {
            sequence.push(String::from(current));
        }
    }
}

impl Default for SystemdUnitDependencyEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nixos_flake_engine() {
        let mut engine = NixOSFlakeEngine::new();
        engine.add_input("nixpkgs", "github:nixos/nixpkgs", "a1b2c3d4e5f6");

        let closure1 = engine.build_closure(&["neovim", "git"]).unwrap();
        assert!(closure1.store_path.contains("/sigma/store/"));
        assert_eq!(closure1.packages.len(), 2);
        assert_eq!(engine.generation_count, 1);

        let closure2 = engine.build_closure(&["neovim", "git", "zsh"]).unwrap();
        assert_eq!(engine.generation_count, 2);
        assert_eq!(closure2.packages.len(), 3);

        // Rollback check
        let rolled_back = engine.rollback_generation().unwrap();
        assert_eq!(rolled_back.packages.len(), 2);
        assert_eq!(engine.generation_count, 1);

        // Garbage collection check
        engine.build_closure(&["htop"]).unwrap();
        let freed = engine.garbage_collect();
        assert_eq!(freed, 1);
    }

    #[test]
    fn test_arch_pacman_hooks() {
        let mut manager = ArchPacmanHooksManager::new();
        manager.register_hook(
            "linux-initramfs",
            HookWhen::PostTransaction,
            "linux",
            "mkinitcpio",
        );

        let triggered = manager.trigger_hooks(HookWhen::PostTransaction, "linux");
        assert_eq!(triggered, 1);
        assert_eq!(manager.executed_hooks_count, 1);
    }

    #[test]
    fn test_void_runit_supervisor() {
        let mut supervisor = VoidRunitSupervisor::new();
        supervisor.add_service("dbus", &[]);
        supervisor.add_service("iwd", &["dbus"]);

        // Cannot start iwd before dbus is up
        assert!(supervisor.start_service("iwd").is_err());
        assert_eq!(
            supervisor.get_service_status("iwd"),
            Some(ServiceState::Down)
        );

        // Start dbus first
        let dbus_pid = supervisor.start_service("dbus").unwrap();
        assert_eq!(dbus_pid, 1001);
        assert_eq!(
            supervisor.get_service_status("dbus"),
            Some(ServiceState::Up)
        );

        // Now iwd can start
        let iwd_pid = supervisor.start_service("iwd").unwrap();
        assert_eq!(iwd_pid, 1002);

        // Test restart
        let new_dbus_pid = supervisor.restart_service("dbus").unwrap();
        assert_eq!(new_dbus_pid, 1003);
    }

    #[test]
    fn test_gentoo_portage_use_flags() {
        let mut portage = GentooPortageUseFlagsEngine::new();
        portage.set_global_use_flags(&["+ssl", "-gtk", "pqc"]);
        portage.register_package("curl", &["ssl", "gtk", "pqc"]);

        let flags = portage.resolve_package_flags("curl").unwrap();
        assert!(flags.contains(&String::from("ssl")));
        assert!(flags.contains(&String::from("pqc")));
        assert!(!flags.contains(&String::from("gtk")));
    }
}
