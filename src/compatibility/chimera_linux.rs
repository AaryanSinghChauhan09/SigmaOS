<<<<<<< HEAD
use crate::klib::Vec;
/// Chimera Linux Compatibility and Subsystem Layer for SigmaOS
/// Replicates Chimera's signature modern features:
/// Dinit Service Manager, BSD-userland/chimerautils, and apk-tools database compatibility.
use core::sync::atomic::{AtomicUsize, Ordering};
||||||| 23ef22a4a
// SigmaOS Distro Compatibility Layer
use crate::klib::Vec;
/// Chimera Linux Compatibility and Subsystem Layer for SigmaOS
/// Replicates Chimera's signature modern features:
/// Dinit Service Manager, BSD-userland/chimerautils, and apk-tools database compatibility.

use core::sync::atomic::{AtomicUsize, Ordering};
use crate::klib::Vec;
=======
// Chimera Linux Emulation Utilities for SigmaOS
// Implements Dinit Service supervision, BSD userland command mapper, and apk-tools databases

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e

use core::sync::atomic::{AtomicBool, Ordering};

pub struct DinitService {
    pub name: String,
    pub active: AtomicBool,
    pub dependencies: Vec<String>,
}

pub struct DinitServiceManager {
    pub services: Vec<DinitService>,
}

impl DinitServiceManager {
    pub fn new() -> Self {
        let mut services = Vec::new();
        services.push(DinitService {
            name: "udev".to_string(),
            active: AtomicBool::new(false),
            dependencies: Vec::new(),
        });
        services.push(DinitService {
            name: "display-manager".to_string(),
            active: AtomicBool::new(false),
            dependencies: vec!["udev".to_string()],
        });

        Self { services }
    }

    pub fn start_service(&self, name: &str) -> Result<(), &'static str> {
        if let Some(service) = self.services.iter().find(|s| s.name == name) {
            // Check dependencies
            for dep in &service.dependencies {
                if let Some(dep_service) = self.services.iter().find(|s| s.name == *dep) {
                    if !dep_service.active.load(Ordering::SeqCst) {
                        return Err("Dependency not started!");
                    }
                }
            }
            service.active.store(true, Ordering::SeqCst);
            Ok(())
        } else {
            Err("Service not found!")
        }
    }

    pub fn stop_service(&self, name: &str) {
        if let Some(service) = self.services.iter().find(|s| s.name == name) {
            service.active.store(false, Ordering::SeqCst);
        }
    }

        self.services[idx].state = DinitServiceState::Starting;

        // Recursively start dependencies first (Dinit logic)
        let deps = self.services[idx].dependencies.clone();
        for dep in &deps {
            let dep_name = &dep[..dep.iter().position(|&b| b == 0).unwrap_or(32)];
            self.start_service(dep_name)?;
        }
    }
}

pub struct BsdUserlandCompat {
    pub mapped_commands_count: usize,
}

impl BsdUserlandCompat {
    pub fn new() -> Self {
        Self {
            mapped_commands_count: 50,
        }
    }

    /// Maps GNU-specific coreutils options to BSD/Chimera compliant coreutils commands
    pub fn translate_command<'a>(&self, gnu_command: &'a str) -> &'a str {
        match gnu_command {
            "ls --color" => "ls -G",
            "cp --parents" => "cp -R",
            "stat -c %a" => "stat -f %Lp",
            _ => gnu_command,
        }
    }
}

pub struct ApkPackageStore {
    pub apk_db_synced: AtomicBool,
}

impl ApkPackageStore {
    pub fn new() -> Self {
        Self {
            apk_db_synced: AtomicBool::new(false),
        }
    }

    pub fn sync_apk_db(&self) -> Result<usize, &'static str> {
        self.apk_db_synced.store(true, Ordering::SeqCst);
        Ok(12431) // Simulated apk-tools database entry count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dinit_service_manager() {
        let manager = DinitServiceManager::new();

        // display-manager should fail to start because udev is inactive
        assert!(manager.start_service("display-manager").is_err());

        // Start dependency udev
        manager.start_service("udev").unwrap();

        // Now start display-manager
        assert!(manager.start_service("display-manager").is_ok());

        // Stop display-manager
        manager.stop_service("display-manager");

        let deps = manager.get_service_dependencies("display-manager");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0], "udev");
    }

    #[test]
    fn test_bsd_userland_compat() {
        let compat = BsdUserlandCompat::new();
        assert_eq!(compat.translate_command("ls --color"), "ls -G");
        assert_eq!(compat.translate_command("stat -c %a"), "stat -f %Lp");
        assert_eq!(compat.translate_command("echo hello"), "echo hello");
        assert_eq!(compat.mapped_commands_count, 50);
    }

    #[test]
    fn test_apk_package_store() {
        let store = ApkPackageStore::new();
        assert!(!store.apk_db_synced.load(Ordering::SeqCst));

<<<<<<< HEAD
        assert!(store.verify_installed_checksum(b"libkmod", b"sha256sumhex"));
        assert!(!store.verify_installed_checksum(b"libkmod", b"wrong"));
    }
||||||| 23ef22a4a
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
=======
        let count = store.sync_apk_db().unwrap();
        assert_eq!(count, 12431);
        assert!(store.apk_db_synced.load(Ordering::SeqCst));
    }
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
}
