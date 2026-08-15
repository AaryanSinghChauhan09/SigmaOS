/// Chimera Linux Compatibility and Subsystem Layer for SigmaOS
/// Replicates Chimera's signature modern features:
/// Dinit Service Manager with supervision, BSD-userland/chimerautils core,
/// apk-tools v3 package registry & triggers, and LLVM/Clang CFI Hardening policies.

use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use crate::klib::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DinitServiceState {
    Stopped,
    Starting,
    Started,
    Stopping,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DinitRestartPolicy {
    Always,
    OnFailure,
    Never,
}

#[derive(Debug, Clone)]
pub struct DinitSocketActivation {
    pub socket_path: [u8; 64],
    pub port: u16,
    pub is_listening: bool,
}

impl DinitSocketActivation {
    pub fn new(path: &[u8], port: u16) -> Self {
        let mut path_arr = [0u8; 64];
        let len = path.len().min(63);
        path_arr[..len].copy_from_slice(&path[..len]);
        Self {
            socket_path: path_arr,
            port,
            is_listening: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DinitService {
    pub name: [u8; 32],
    pub state: DinitServiceState,
    pub restart_policy: DinitRestartPolicy,
    pub dependencies: Vec<[u8; 32]>,
    pub socket_activation: Option<DinitSocketActivation>,
    pub restart_count: usize,
}

impl DinitService {
    pub fn new(name: &[u8]) -> Self {
        let mut name_arr = [0u8; 32];
        let len = name.len().min(31);
        name_arr[..len].copy_from_slice(&name[..len]);
        DinitService {
            name: name_arr,
            state: DinitServiceState::Stopped,
            restart_policy: DinitRestartPolicy::OnFailure,
            dependencies: Vec::new(),
            socket_activation: None,
            restart_count: 0,
        }
    }

    pub fn add_dependency(&mut self, dep: &[u8]) {
        let mut dep_arr = [0u8; 32];
        let len = dep.len().min(31);
        dep_arr[..len].copy_from_slice(&dep[..len]);
        self.dependencies.push(dep_arr);
    }

    pub fn with_socket_activation(mut self, socket: DinitSocketActivation) -> Self {
        self.socket_activation = Some(socket);
        self
    }

    pub fn with_restart_policy(mut self, policy: DinitRestartPolicy) -> Self {
        self.restart_policy = policy;
        self
    }
}

/// dinit-chimera service manager and process supervisor simulation
pub struct DinitServiceManager {
    pub services: Vec<DinitService>,
    pub running_count: AtomicUsize,
    pub supervisor_active: AtomicBool,
}

impl Default for DinitServiceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DinitServiceManager {
    pub fn new() -> Self {
        DinitServiceManager {
            services: Vec::new(),
            running_count: AtomicUsize::new(0),
            supervisor_active: AtomicBool::new(true),
        }
    }

    pub fn register_service(&mut self, svc: DinitService) {
        self.services.push(svc);
    }

    pub fn start_service(&mut self, name: &[u8]) -> Result<(), &'static str> {
        let mut name_arr = [0u8; 32];
        let len = name.len().min(31);
        name_arr[..len].copy_from_slice(&name[..len]);

        let mut found_idx = None;
        for (i, svc) in self.services.iter().enumerate() {
            if svc.name == name_arr {
                found_idx = Some(i);
                break;
            }
        }

        let idx = found_idx.ok_or("Service not found in dinit database")?;

        if self.services[idx].state == DinitServiceState::Started {
            return Ok(());
        }

        self.services[idx].state = DinitServiceState::Starting;

        // Recursively start dependencies first (Dinit topological dependency graph logic)
        let deps = self.services[idx].dependencies.clone();
        for dep in &deps {
            let end_idx = dep.iter().position(|&b| b == 0).unwrap_or(32);
            let dep_name = &dep[..end_idx];
            self.start_service(dep_name)?;
        }

        self.services[idx].state = DinitServiceState::Started;
        self.running_count.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    /// Simulates dinit service failure and evaluates supervisor restart policy
    pub fn handle_service_failure(&mut self, name: &[u8]) -> Result<bool, &'static str> {
        let mut name_arr = [0u8; 32];
        let len = name.len().min(31);
        name_arr[..len].copy_from_slice(&name[..len]);

        for svc in self.services.iter_mut() {
            if svc.name == name_arr {
                svc.state = DinitServiceState::Failed;
                svc.restart_count += 1;

                let should_restart = match svc.restart_policy {
                    DinitRestartPolicy::Always => true,
                    DinitRestartPolicy::OnFailure => svc.restart_count <= 5,
                    DinitRestartPolicy::Never => false,
                };

                if should_restart {
                    svc.state = DinitServiceState::Started;
                    return Ok(true); // Restarted by supervisor
                }
                return Ok(false);
            }
        }
        Err("Service not found")
    }
}

/// BSD chimerautils / userland core utilities compatibility layer
pub struct BsdUserlandCompat;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FreeBSDJailContext {
    pub jid: u32,
    pub name: [u8; 32],
    pub path: [u8; 64],
    pub is_active: bool,
}

impl BsdUserlandCompat {
    pub fn translate_bsd_df_output(&self, total: usize, used: usize) -> (usize, usize) {
        // BSD df reports 512-byte blocks, we translate to standardized byte structures
        let block_size = 512;
        (total * block_size, used * block_size)
    }

    /// Simulates BSD chflags file flags (e.g. UF_IMMUTABLE, SF_NODUMP)
    pub fn evaluate_bsd_chflags(&self, flags: u32, is_user: bool) -> bool {
        const UF_IMMUTABLE: u32 = 0x00000002;
        const SF_IMMUTABLE: u32 = 0x00020000;

        if (flags & SF_IMMUTABLE) != 0 && is_user {
            return false; // User cannot override system immutable flag
        }
        if (flags & UF_IMMUTABLE) != 0 {
            return false; // User immutable flag active
        }
        true
    }

    /// Simulates pkill / pgrep BSD process filtering
    pub fn pgrep_filter_by_name(&self, processes: &[(&[u8], usize)], pattern: &[u8]) -> Vec<usize> {
        let mut pids = Vec::new();
        for (proc_name, pid) in processes {
            if proc_name.starts_with(pattern) {
                pids.push(*pid);
            }
        }
        pids
    }

    /// Creates a FreeBSD jail isolation context for Chimera userland sandboxing
    pub fn create_freebsd_jail(&self, jid: u32, name: &[u8], path: &[u8]) -> FreeBSDJailContext {
        let mut name_arr = [0u8; 32];
        let mut path_arr = [0u8; 64];

        let n_len = name.len().min(31);
        let p_len = path.len().min(63);

        name_arr[..n_len].copy_from_slice(&name[..n_len]);
        path_arr[..p_len].copy_from_slice(&path[..p_len]);

        FreeBSDJailContext {
            jid,
            name: name_arr,
            path: path_arr,
            is_active: true,
        }
    }
}

/// apk-tools v3 (Alpine/Chimera) package registry & world compatibility layer
#[derive(Debug, Clone)]
pub struct ApkPackageMetadata {
    pub name: [u8; 32],
    pub version: [u8; 16],
    pub checksum_sha256: [u8; 32],
    pub install_size: usize,
    pub is_virtual: bool,
    pub trigger_scripts: Vec<[u8; 32]>,
}

impl ApkPackageMetadata {
    pub fn new(name: &[u8], version: &[u8], checksum: &[u8]) -> Self {
        let mut name_arr = [0u8; 32];
        let mut ver_arr = [0u8; 16];
        let mut csum_arr = [0u8; 32];

        let n_len = name.len().min(31);
        let v_len = version.len().min(15);
        let c_len = checksum.len().min(31);

        name_arr[..n_len].copy_from_slice(&name[..n_len]);
        ver_arr[..v_len].copy_from_slice(&version[..v_len]);
        csum_arr[..c_len].copy_from_slice(&checksum[..c_len]);

        ApkPackageMetadata {
            name: name_arr,
            version: ver_arr,
            checksum_sha256: csum_arr,
            install_size: 1024 * 1024,
            is_virtual: false,
            trigger_scripts: Vec::new(),
        }
    }

    pub fn with_trigger(mut self, trigger: &[u8]) -> Self {
        let mut t_arr = [0u8; 32];
        let len = trigger.len().min(31);
        t_arr[..len].copy_from_slice(&trigger[..len]);
        self.trigger_scripts.push(t_arr);
        self
    }
}

pub struct ApkPackageStore {
    pub installed_packages: Vec<ApkPackageMetadata>,
    pub world_file_entries: Vec<[u8; 32]>, // Explicitly requested packages (/etc/apk/world)
}

impl Default for ApkPackageStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ApkPackageStore {
    pub fn new() -> Self {
        ApkPackageStore {
            installed_packages: Vec::new(),
            world_file_entries: Vec::new(),
        }
    }

    pub fn register_apk_installed(&mut self, pkg: ApkPackageMetadata) {
        self.installed_packages.push(pkg);
    }

    pub fn add_to_world(&mut self, name: &[u8]) {
        let mut name_arr = [0u8; 32];
        let len = name.len().min(31);
        name_arr[..len].copy_from_slice(&name[..len]);
        self.world_file_entries.push(name_arr);
    }

    pub fn verify_installed_checksum(&self, name: &[u8], checksum: &[u8]) -> bool {
        let mut name_arr = [0u8; 32];
        let len = name.len().min(31);
        name_arr[..len].copy_from_slice(&name[..len]);

        for pkg in &self.installed_packages {
            if pkg.name == name_arr {
                return pkg.checksum_sha256[..checksum.len()] == checksum[..checksum.len()];
            }
        }
        false
    }

    /// Executes pending apk-tools v3 triggers for installed packages
    pub fn run_pending_triggers(&self) -> usize {
        let mut triggered_count = 0;
        for pkg in &self.installed_packages {
            triggered_count += pkg.trigger_scripts.len();
        }
        triggered_count
    }
}

/// LLVM / Clang Control Flow Integrity (CFI) & Security Hardening Policy
#[derive(Debug, Clone, Copy)]
pub struct ClangLlvmHardeningPolicy {
    pub enable_cfi: bool,
    pub enable_shadow_call_stack: bool,
    pub enable_safe_stack: bool,
    pub fortify_source_level: u8,
    pub enable_full_relro: bool,
}

impl Default for ClangLlvmHardeningPolicy {
    fn default() -> Self {
        Self::chimera_default()
    }
}

impl ClangLlvmHardeningPolicy {
    pub fn chimera_default() -> Self {
        Self {
            enable_cfi: true,
            enable_shadow_call_stack: true,
            enable_safe_stack: true,
            fortify_source_level: 3,
            enable_full_relro: true,
        }
    }

    pub fn is_fully_hardened(&self) -> bool {
        self.enable_cfi
            && self.enable_shadow_call_stack
            && self.fortify_source_level >= 2
            && self.enable_full_relro
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dinit_service_manager() {
        let mut dinit = DinitServiceManager::new();

        let mut console = DinitService::new(b"dinit-console")
            .with_restart_policy(DinitRestartPolicy::Always);
        console.add_dependency(b"keyboard");

        let keyboard = DinitService::new(b"keyboard");

        dinit.register_service(console);
        dinit.register_service(keyboard);

        dinit.start_service(b"dinit-console").unwrap();

        assert_eq!(dinit.running_count.load(Ordering::SeqCst), 2);

        // Test supervisor auto-restart policy
        let restarted = dinit.handle_service_failure(b"dinit-console").unwrap();
        assert!(restarted);
    }

    #[test]
    fn test_bsd_userland_compat() {
        let compat = BsdUserlandCompat;
        let (total_b, used_b) = compat.translate_bsd_df_output(1000, 400);
        assert_eq!(total_b, 512000);
        assert_eq!(used_b, 204800);

        // Test pgrep filtering
        let pids = compat.pgrep_filter_by_name(&[(b"nginx", 101), (b"dinit", 102)], b"ng");
        assert_eq!(pids, vec![101]);

        // Test FreeBSD Jail context
        let jail = compat.create_freebsd_jail(1, b"sandbox_jail", b"/jails/1");
        assert!(jail.is_active);
        assert_eq!(jail.jid, 1);
    }

    #[test]
    fn test_apk_package_store() {
        let mut store = ApkPackageStore::new();
        let pkg = ApkPackageMetadata::new(b"libkmod", b"31-r0", b"sha256sumhex")
            .with_trigger(b"00_depmod.trigger");
        store.register_apk_installed(pkg);

        store.add_to_world(b"libkmod");

        assert!(store.verify_installed_checksum(b"libkmod", b"sha256sumhex"));
        assert!(!store.verify_installed_checksum(b"libkmod", b"wrong"));
        assert_eq!(store.run_pending_triggers(), 1);
        assert_eq!(store.world_file_entries.len(), 1);
    }

    #[test]
    fn test_clang_llvm_hardening() {
        let policy = ClangLlvmHardeningPolicy::chimera_default();
        assert!(policy.is_fully_hardened());
        assert_eq!(policy.fortify_source_level, 3);
    }
}
