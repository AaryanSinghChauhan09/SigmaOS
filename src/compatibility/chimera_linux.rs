// SigmaOS Distro Compatibility Layer
use crate::klib::Vec;
/// Chimera Linux Compatibility and Subsystem Layer for SigmaOS
/// Replicates Chimera's signature modern features:
/// Dinit Service Manager, BSD-userland/chimerautils, and apk-tools database compatibility.

use core::sync::atomic::{AtomicUsize, Ordering};
use crate::klib::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DinitServiceState {
    Stopped,
    Starting,
    Started,
    Stopping,
    Failed,
}

#[derive(Debug, Clone)]
pub struct DinitService {
    pub name: [u8; 32],
    pub state: DinitServiceState,
    pub dependencies: Vec<[u8; 32]>,
}

impl DinitService {
    pub fn new(name: &[u8]) -> Self {
        let mut name_arr = [0u8; 32];
        name_arr[..name.len().min(31)].copy_from_slice(&name[..name.len().min(31)]);
        DinitService {
            name: name_arr,
            state: DinitServiceState::Stopped,
            dependencies: Vec::new(),
        }
    }

    pub fn add_dependency(&mut self, dep: &[u8]) {
        let mut dep_arr = [0u8; 32];
        dep_arr[..dep.len().min(31)].copy_from_slice(&dep[..dep.len().min(31)]);
        self.dependencies.push(dep_arr);
    }
}

/// dinit-chimera service manager simulation
pub struct DinitServiceManager {
    pub services: Vec<DinitService>,
    pub running_count: AtomicUsize,
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
        }
    }

    pub fn register_service(&mut self, svc: DinitService) {
        self.services.push(svc);
    }

    pub fn start_service(&mut self, name: &[u8]) -> Result<(), &'static str> {
        let mut name_arr = [0u8; 32];
        name_arr[..name.len().min(31)].copy_from_slice(&name[..name.len().min(31)]);

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

        // Recursively start dependencies first (Dinit logic)
        let deps = self.services[idx].dependencies.clone();
        for dep in &deps {
            let mut dep_len = 32;
            for i in 0..32 {
                if dep[i] == 0 {
                    dep_len = i;
                    break;
                }
            }
            let dep_name = &dep[..dep_len];
            self.start_service(dep_name)?;
        }

        self.services[idx].state = DinitServiceState::Started;
        self.running_count.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
}

/// BSD chimerautils / userland core utilities compatibility layer
pub struct BsdUserlandCompat;

impl BsdUserlandCompat {
    pub fn translate_bsd_df_output(&self, total: usize, used: usize) -> (usize, usize) {
        // BSD df reports blocks, we translate to standardized byte structures
        let block_size = 512;
        (total * block_size, used * block_size)
    }
}

/// apk-tools (Alpine/Chimera) package registry compatibility layer
#[derive(Debug, Clone)]
pub struct ApkPackageMetadata {
    pub name: [u8; 32],
    pub version: [u8; 16],
    pub checksum_sha256: [u8; 32],
    pub install_size: usize,
}

impl ApkPackageMetadata {
    pub fn new(name: &[u8], version: &[u8], checksum: &[u8]) -> Self {
        let mut name_arr = [0u8; 32];
        let mut ver_arr = [0u8; 16];
        let mut csum_arr = [0u8; 32];

        name_arr[..name.len().min(31)].copy_from_slice(&name[..name.len().min(31)]);
        ver_arr[..version.len().min(15)].copy_from_slice(&version[..version.len().min(15)]);
        csum_arr[..checksum.len().min(31)].copy_from_slice(&checksum[..checksum.len().min(31)]);

        ApkPackageMetadata {
            name: name_arr,
            version: ver_arr,
            checksum_sha256: csum_arr,
            install_size: 1024 * 1024,
        }
    }
}

pub struct ApkPackageStore {
    pub installed_packages: Vec<ApkPackageMetadata>,
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
        }
    }

    pub fn register_apk_installed(&mut self, pkg: ApkPackageMetadata) {
        self.installed_packages.push(pkg);
    }

    pub fn verify_installed_checksum(&self, name: &[u8], checksum: &[u8]) -> bool {
        let mut name_arr = [0u8; 32];
        name_arr[..name.len().min(31)].copy_from_slice(&name[..name.len().min(31)]);

        for pkg in &self.installed_packages {
            if pkg.name == name_arr {
                return pkg.checksum_sha256[..checksum.len()] == checksum[..checksum.len()];
            }
        }
        false
    }
}

/// BSD kqueue (kqueue/kevent) to Linux epoll (epoll_create/epoll_ctl) compatibility mapping
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BsdFilter {
    Read = -1,
    Write = -2,
    Aio = -3,
    Vnode = -4,
    Proc = -5,
    Signal = -6,
    Timer = -7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BsdEventFlag {
    Add = 0x0001,
    Delete = 0x0002,
    Enable = 0x0004,
    Disable = 0x0008,
    Oneshot = 0x0010,
    Clear = 0x0020,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxEpollEvent {
    EpollIn = 0x0001,
    EpollOut = 0x0004,
    EpollHub = 0x0008,
    EpollErr = 0x0010,
    EpollEt = 1 << 31,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxEpollCtl {
    Add = 1,
    Del = 2,
    Mod = 3,
}

pub struct BsdLinuxSyscallBridge;

impl BsdLinuxSyscallBridge {
    pub fn translate_kqueue_filter_to_epoll(&self, filter: BsdFilter) -> u32 {
        match filter {
            BsdFilter::Read => LinuxEpollEvent::EpollIn as u32,
            BsdFilter::Write => LinuxEpollEvent::EpollOut as u32,
            _ => LinuxEpollEvent::EpollIn as u32 | LinuxEpollEvent::EpollEt as u32,
        }
    }

    pub fn translate_kqueue_flags_to_epoll_ctl(&self, flags: u16) -> Option<u32> {
        if (flags & BsdEventFlag::Add as u16) != 0 {
            Some(LinuxEpollCtl::Add as u32)
        } else if (flags & BsdEventFlag::Delete as u16) != 0 {
            Some(LinuxEpollCtl::Del as u32)
        } else if (flags & BsdEventFlag::Enable as u16) != 0 {
            Some(LinuxEpollCtl::Mod as u32)
        } else {
            None
        }
    }
}

/// FreeBSD-style `jail` configuration properties to Linux namespace mapping controls
#[derive(Debug, Clone)]
pub struct BsdJailConfig {
    pub jid: i32,
    pub name: [u8; 64],
    pub hostname: [u8; 128],
    pub root_path: [u8; 256],
    pub ipv4_addr: [u8; 4],
    pub secure_level: i32,
}

impl BsdJailConfig {
    pub fn new(jid: i32, name: &[u8], hostname: &[u8], root: &[u8]) -> Self {
        let mut name_arr = [0u8; 64];
        let mut host_arr = [0u8; 128];
        let mut root_arr = [0u8; 256];

        name_arr[..name.len().min(63)].copy_from_slice(&name[..name.len().min(63)]);
        host_arr[..hostname.len().min(127)].copy_from_slice(&hostname[..hostname.len().min(127)]);
        root_arr[..root.len().min(255)].copy_from_slice(&root[..root.len().min(255)]);

        BsdJailConfig {
            jid,
            name: name_arr,
            hostname: host_arr,
            root_path: root_arr,
            ipv4_addr: [127, 0, 0, 1],
            secure_level: 1,
        }
    }
}

/// Translates FreeBSD jail properties into Linux-style namespace cloning/security bounds
pub struct BsdJailManager {
    pub active_jails: Vec<BsdJailConfig>,
}

impl Default for BsdJailManager {
    fn default() -> Self {
        Self::new()
    }
}

impl BsdJailManager {
    pub fn new() -> Self {
        BsdJailManager {
            active_jails: Vec::new(),
        }
    }

    pub fn register_jail(&mut self, jail: BsdJailConfig) {
        self.active_jails.push(jail);
    }

    pub fn compute_linux_clone_flags(&self, _jid: i32) -> u64 {
        // Linux clone flags representing equivalent isolation properties:
        // CLONE_NEWPID (0x20000000), CLONE_NEWNET (0x40000000), CLONE_NEWNS (0x00020000), CLONE_NEWUTS (0x04000000)
        let clone_newns = 0x00020000;
        let clone_newuts = 0x04000000;
        let clone_newpid = 0x20000000;
        let clone_newnet = 0x40000000;
        clone_newns | clone_newuts | clone_newpid | clone_newnet
    }
}

/// Dynamic sysctl MIB variable management module
#[derive(Debug, Clone)]
pub struct SysctlNode {
    pub name: [u8; 64],
    pub value_int: i32,
    pub is_writable: bool,
}

pub struct BsdSysctlRegistry {
    pub nodes: Vec<SysctlNode>,
}

impl Default for BsdSysctlRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl BsdSysctlRegistry {
    pub fn new() -> Self {
        let mut reg = BsdSysctlRegistry { nodes: Vec::new() };
        reg.register_defaults();
        reg
    }

    fn register_defaults(&mut self) {
        self.register_node(b"kern.securelevel", 1, true);
        self.register_node(b"compat.linux.osrelease", 5150, true); // Models Linux 5.15
        self.register_node(b"kern.maxfiles", 65536, false);
    }

    pub fn register_node(&mut self, name: &[u8], val: i32, writable: bool) {
        let mut name_arr = [0u8; 64];
        name_arr[..name.len().min(63)].copy_from_slice(&name[..name.len().min(63)]);
        self.nodes.push(SysctlNode {
            name: name_arr,
            value_int: val,
            is_writable: writable,
        });
    }

    pub fn read_sysctl(&self, name: &[u8]) -> Result<i32, &'static str> {
        let mut name_arr = [0u8; 64];
        name_arr[..name.len().min(63)].copy_from_slice(&name[..name.len().min(63)]);

        for node in &self.nodes {
            if node.name == name_arr {
                return Ok(node.value_int);
            }
        }
        Err("sysctl MIB path not found")
    }

    pub fn write_sysctl(&mut self, name: &[u8], new_val: i32) -> Result<(), &'static str> {
        let mut name_arr = [0u8; 64];
        name_arr[..name.len().min(63)].copy_from_slice(&name[..name.len().min(63)]);

        for i in 0..self.nodes.len() {
            if self.nodes[i].name == name_arr {
                if !self.nodes[i].is_writable {
                    return Err("sysctl node is read-only");
                }
                self.nodes[i].value_int = new_val;
                return Ok(());
            }
        }
        Err("sysctl MIB path not found")
    }
}

/// Auxiliary Vector formatted for glibc/musl dynamic program loader execution environment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxAuxvType {
    Null = 0,
    Phdr = 3,
    Phent = 4,
    Phnum = 5,
    Pagesz = 6,
    Base = 7,
    Flags = 8,
    Entry = 9,
    Platform = 15,
}

pub struct AuxiliaryVectorBuilder;

impl AuxiliaryVectorBuilder {
    pub fn build_elf_auxv_array(&self, phdr: usize, entry: usize, pagesz: usize) -> Vec<(usize, usize)> {
        let mut auxv = Vec::new();
        auxv.push((LinuxAuxvType::Pagesz as usize, pagesz));
        auxv.push((LinuxAuxvType::Phdr as usize, phdr));
        auxv.push((LinuxAuxvType::Entry as usize, entry));
        auxv.push((LinuxAuxvType::Null as usize, 0));
        auxv
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dinit_service_manager() {
        let mut dinit = DinitServiceManager::new();

        let mut console = DinitService::new(b"dinit-console");
        console.add_dependency(b"keyboard");

        let keyboard = DinitService::new(b"keyboard");

        dinit.register_service(console);
        dinit.register_service(keyboard);

        dinit.start_service(b"dinit-console").unwrap();

        assert_eq!(dinit.running_count.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_bsd_userland_compat() {
        let compat = BsdUserlandCompat;
        let (total_b, used_b) = compat.translate_bsd_df_output(1000, 400);
        assert_eq!(total_b, 512000);
        assert_eq!(used_b, 204800);
    }

    #[test]
    fn test_apk_package_store() {
        let mut store = ApkPackageStore::new();
        let pkg = ApkPackageMetadata::new(b"libkmod", b"31-r0", b"sha256sumhex");
        store.register_apk_installed(pkg);

        assert!(store.verify_installed_checksum(b"libkmod", b"sha256sumhex"));
        assert!(!store.verify_installed_checksum(b"libkmod", b"wrong"));
    }

    #[test]
    fn test_kqueue_epoll_translation() {
        let bridge = BsdLinuxSyscallBridge;
        let epoll_event = bridge.translate_kqueue_filter_to_epoll(BsdFilter::Read);
        assert_eq!(epoll_event, LinuxEpollEvent::EpollIn as u32);

        let epoll_event_write = bridge.translate_kqueue_filter_to_epoll(BsdFilter::Write);
        assert_eq!(epoll_event_write, LinuxEpollEvent::EpollOut as u32);

        let epoll_ctl_add = bridge.translate_kqueue_flags_to_epoll_ctl(BsdEventFlag::Add as u16).unwrap();
        assert_eq!(epoll_ctl_add, LinuxEpollCtl::Add as u32);

        let epoll_ctl_del = bridge.translate_kqueue_flags_to_epoll_ctl(BsdEventFlag::Delete as u16).unwrap();
        assert_eq!(epoll_ctl_del, LinuxEpollCtl::Del as u32);
    }

    #[test]
    fn test_bsd_jail_sandbox_mapping() {
        let mut manager = BsdJailManager::new();
        let jail = BsdJailConfig::new(1, b"testjail", b"jailhost", b"/jails/testjail");
        manager.register_jail(jail);

        let clone_flags = manager.compute_linux_clone_flags(1);
        let clone_newns = 0x00020000;
        let clone_newpid = 0x20000000;
        assert_eq!(clone_flags & clone_newns, clone_newns);
        assert_eq!(clone_flags & clone_newpid, clone_newpid);
    }

    #[test]
    fn test_sysctl_dynamic_namespace() {
        let mut registry = BsdSysctlRegistry::new();
        let secure_lvl = registry.read_sysctl(b"kern.securelevel").unwrap();
        assert_eq!(secure_lvl, 1);

        assert!(registry.write_sysctl(b"kern.securelevel", 2).is_ok());
        assert_eq!(registry.read_sysctl(b"kern.securelevel").unwrap(), 2);

        // Read-only check
        let max_files_res = registry.write_sysctl(b"kern.maxfiles", 100000);
        assert_eq!(max_files_res, Err("sysctl node is read-only"));
    }

    #[test]
    fn test_auxiliary_vector_formatting() {
        let builder = AuxiliaryVectorBuilder;
        let auxv = builder.build_elf_auxv_array(0x8048000, 0x8049000, 4096);
        assert_eq!(auxv.len(), 4);
        assert_eq!(auxv[0], (LinuxAuxvType::Pagesz as usize, 4096));
        assert_eq!(auxv[1], (LinuxAuxvType::Phdr as usize, 0x8048000));
        assert_eq!(auxv[2], (LinuxAuxvType::Entry as usize, 0x8049000));
        assert_eq!(auxv[3], (LinuxAuxvType::Null as usize, 0));
    }
}
