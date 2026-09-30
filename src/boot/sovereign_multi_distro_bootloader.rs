//! Sovereign Multi-Distro Bootloader Parity Engine (`sovereign_multi_distro_bootloader.rs`)
//!
//! Universal Bootloader Interoperability Engine inspired by Linux and BSD bootloaders:
//! - FreeBSD `loader.conf` (kernel modules, zfs_enable, boot delay, kernel parameters)
//! - OpenBSD `boot.conf` (boot device, kernel path, memory limits, serial console)
//! - GRUB2 `grub.cfg` (menuentries, insmod, linux/initrd, multiboot2 parameters)
//! - systemd-boot Boot Loader Specification (BLS) `.conf` entries (`title`, `linux`, `initrd`, `options`)
//! - Limine `limine.conf` (`PROTOCOL`, `KERNEL_PATH`, `CMDLINE`, `MODULE_PATH`)
//! - rEFInd `refind.conf` (`menuentry`, `icon`, `loader`, `initrd`, `options`)
//!
//! Provides parsing, multi-format cross-translation, and configuration generation
//! ensuring seamless dual-booting and booting across any Linux and BSD distro installation.

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// Supported Bootloader Configuration Standard
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootloaderKind {
    FreeBsdLoader,
    OpenBsdBoot,
    Grub2,
    SystemdBootBls,
    Limine,
    Refind,
    SigmaNative,
}

/// Normalized Universal Boot Entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniversalBootEntry {
    pub id: String,
    pub title: String,
    pub kernel_path: String,
    pub initrd_paths: Vec<String>,
    pub cmdline_options: String,
    pub root_device: String,
    pub is_default: bool,
    pub source_kind: BootloaderKind,
}

/// FreeBSD `loader.conf` Parser & Engine
#[derive(Debug, Clone, Default)]
pub struct FreeBsdLoaderConfig {
    pub kernel: String,
    pub boot_delay: u32,
    pub zfs_load: bool,
    pub loaded_modules: Vec<String>,
    pub custom_vars: BTreeMap<String, String>,
}

impl FreeBsdLoaderConfig {
    pub fn parse(content: &str) -> Self {
        let mut cfg = Self {
            kernel: "kernel".to_string(),
            boot_delay: 3,
            zfs_load: false,
            loaded_modules: Vec::new(),
            custom_vars: BTreeMap::new(),
        };

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, val)) = line.split_once('=') {
                let key = key.trim();
                let val = val.trim().trim_matches('"').trim_matches('\'');

                match key {
                    "kernel" => cfg.kernel = val.to_string(),
                    "autoboot_delay" => {
                        if let Ok(d) = val.parse::<u32>() {
                            cfg.boot_delay = d;
                        }
                    }
                    "zfs_load" => cfg.zfs_load = val == "YES" || val == "1" || val == "true",
                    k if k.ends_with("_load") => {
                        if val == "YES" || val == "1" || val == "true" {
                            let mod_name = k.trim_end_matches("_load").to_string();
                            cfg.loaded_modules.push(mod_name);
                        }
                    }
                    _ => {
                        cfg.custom_vars.insert(key.to_string(), val.to_string());
                    }
                }
            }
        }
        cfg
    }

    pub fn to_universal_entry(&self) -> UniversalBootEntry {
        let mut cmdline = format!("zfs_load={}", if self.zfs_load { "YES" } else { "NO" });
        for (k, v) in &self.custom_vars {
            cmdline.push_str(&format!(" {}={}", k, v));
        }

        UniversalBootEntry {
            id: "freebsd-kernel".to_string(),
            title: "FreeBSD / SigmaOS BSD Subsystem Kernel".to_string(),
            kernel_path: format!("/boot/{}", self.kernel),
            initrd_paths: Vec::new(),
            cmdline_options: cmdline,
            root_device: "zfs:zroot/ROOT/default".to_string(),
            is_default: true,
            source_kind: BootloaderKind::FreeBsdLoader,
        }
    }
}

/// OpenBSD `boot.conf` Parser & Engine
#[derive(Debug, Clone, Default)]
pub struct OpenBsdBootConfig {
    pub default_kernel: String,
    pub boot_device: String,
    pub timeout: u32,
    pub serial_console: Option<String>,
    pub commands: Vec<String>,
}

impl OpenBsdBootConfig {
    pub fn parse(content: &str) -> Self {
        let mut cfg = Self {
            default_kernel: "bsd".to_string(),
            boot_device: "sd0a".to_string(),
            timeout: 5,
            serial_console: None,
            commands: Vec::new(),
        };

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "boot" => {
                    if parts.len() > 1 {
                        cfg.default_kernel = parts[1].to_string();
                    }
                }
                "set" => {
                    if parts.len() >= 3 && parts[1] == "timeout" {
                        if let Ok(t) = parts[2].parse::<u32>() {
                            cfg.timeout = t;
                        }
                    } else if parts.len() >= 3 && parts[1] == "tty" {
                        cfg.serial_console = Some(parts[2].to_string());
                    }
                }
                "stty" => {
                    if parts.len() >= 2 {
                        cfg.serial_console = Some(parts[1..].join(" "));
                    }
                }
                _ => {
                    cfg.commands.push(line.to_string());
                }
            }
        }
        cfg
    }

    pub fn to_universal_entry(&self) -> UniversalBootEntry {
        UniversalBootEntry {
            id: "openbsd-kernel".to_string(),
            title: "OpenBSD Secure Kernel / SigmaOS Guard".to_string(),
            kernel_path: format!("/{}", self.default_kernel),
            initrd_paths: Vec::new(),
            cmdline_options: format!("root_dev={}", self.boot_device),
            root_device: self.boot_device.clone(),
            is_default: true,
            source_kind: BootloaderKind::OpenBsdBoot,
        }
    }
}

/// systemd-boot BLS Entry Parser & Generator
#[derive(Debug, Clone, Default)]
pub struct SystemdBootBlsEntry {
    pub title: String,
    pub version: String,
    pub linux: String,
    pub initrd: Vec<String>,
    pub options: String,
    pub architecture: String,
}

impl SystemdBootBlsEntry {
    pub fn parse(content: &str) -> Self {
        let mut entry = Self::default();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, val)) = line.split_once(char::is_whitespace) {
                let key = key.trim();
                let val = val.trim();

                match key {
                    "title" => entry.title = val.to_string(),
                    "version" => entry.version = val.to_string(),
                    "linux" => entry.linux = val.to_string(),
                    "initrd" => entry.initrd.push(val.to_string()),
                    "options" => entry.options = val.to_string(),
                    "architecture" => entry.architecture = val.to_string(),
                    _ => {}
                }
            }
        }
        entry
    }

    pub fn generate_bls_content(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("title {}\n", self.title));
        if !self.version.is_empty() {
            out.push_str(&format!("version {}\n", self.version));
        }
        out.push_str(&format!("linux {}\n", self.linux));
        for init in &self.initrd {
            out.push_str(&format!("initrd {}\n", init));
        }
        if !self.options.is_empty() {
            out.push_str(&format!("options {}\n", self.options));
        }
        out
    }

    pub fn to_universal_entry(&self) -> UniversalBootEntry {
        UniversalBootEntry {
            id: format!("bls-{}", self.title.to_lowercase().replace(' ', "-")),
            title: self.title.clone(),
            kernel_path: self.linux.clone(),
            initrd_paths: self.initrd.clone(),
            cmdline_options: self.options.clone(),
            root_device: "/dev/nvme0n1p2".to_string(),
            is_default: false,
            source_kind: BootloaderKind::SystemdBootBls,
        }
    }
}

/// Limine `limine.conf` Parser & Engine
#[derive(Debug, Clone, Default)]
pub struct LimineBootEntry {
    pub protocol: String,
    pub kernel_path: String,
    pub cmdline: String,
    pub modules: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct LimineConfig {
    pub timeout: u32,
    pub entries: BTreeMap<String, LimineBootEntry>,
}

impl LimineConfig {
    pub fn parse(content: &str) -> Self {
        let mut cfg = Self {
            timeout: 5,
            entries: BTreeMap::new(),
        };

        let mut current_entry_name: Option<String> = None;
        let mut current_entry = LimineBootEntry::default();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
                continue;
            }

            if line.starts_with("TIMEOUT:") {
                if let Some((_, val)) = line.split_once(':') {
                    if let Ok(t) = val.trim().parse::<u32>() {
                        cfg.timeout = t;
                    }
                }
                continue;
            }

            if line.starts_with(':') {
                if let Some(name) = current_entry_name.take() {
                    cfg.entries.insert(name, current_entry);
                    current_entry = LimineBootEntry::default();
                }
                current_entry_name = Some(line.trim_start_matches(':').trim().to_string());
                continue;
            }

            if let Some((key, val)) = line.split_once(':') {
                let key = key.trim();
                let val = val.trim();

                match key {
                    "PROTOCOL" => current_entry.protocol = val.to_string(),
                    "KERNEL_PATH" => current_entry.kernel_path = val.to_string(),
                    "CMDLINE" => current_entry.cmdline = val.to_string(),
                    "MODULE_PATH" => current_entry.modules.push(val.to_string()),
                    _ => {}
                }
            }
        }

        if let Some(name) = current_entry_name {
            cfg.entries.insert(name, current_entry);
        }

        cfg
    }
}

/// rEFInd `refind.conf` Parser & Engine
#[derive(Debug, Clone, Default)]
pub struct RefindMenuEntry {
    pub name: String,
    pub icon: String,
    pub loader: String,
    pub initrd: String,
    pub options: String,
}

impl RefindMenuEntry {
    pub fn parse(content: &str) -> Self {
        let mut entry = Self::default();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.starts_with("menuentry") {
                let name = line.trim_start_matches("menuentry").trim().trim_matches('{').trim().trim_matches('"');
                entry.name = name.to_string();
                continue;
            }

            if let Some((key, val)) = line.split_once(char::is_whitespace) {
                let key = key.trim();
                let val = val.trim().trim_matches('"');

                match key {
                    "icon" => entry.icon = val.to_string(),
                    "loader" => entry.loader = val.to_string(),
                    "initrd" => entry.initrd = val.to_string(),
                    "options" => entry.options = val.to_string(),
                    _ => {}
                }
            }
        }
        entry
    }
}

/// Sovereign Master Multi-Distro Bootloader Orchestrator
#[derive(Debug, Clone, Default)]
pub struct SovereignMultiDistroBootloaderEngine {
    pub entries: Vec<UniversalBootEntry>,
}

impl SovereignMultiDistroBootloaderEngine {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn register_entry(&mut self, entry: UniversalBootEntry) {
        self.entries.push(entry);
    }

    pub fn import_freebsd_loader(&mut self, loader_conf: &str) {
        let cfg = FreeBsdLoaderConfig::parse(loader_conf);
        self.entries.push(cfg.to_universal_entry());
    }

    pub fn import_openbsd_boot(&mut self, boot_conf: &str) {
        let cfg = OpenBsdBootConfig::parse(boot_conf);
        self.entries.push(cfg.to_universal_entry());
    }

    pub fn import_bls_entry(&mut self, bls_conf: &str) {
        let entry = SystemdBootBlsEntry::parse(bls_conf);
        self.entries.push(entry.to_universal_entry());
    }

    pub fn generate_unified_grub2_cfg(&self) -> String {
        let mut out = String::from("# Generated by Sovereign Multi-Distro Bootloader\nset timeout=5\nset default=0\n\n");
        for entry in &self.entries {
            out.push_str(&format!("menuentry '{}' {{\n", entry.title));
            out.push_str("    insmod gpt\n");
            out.push_str("    insmod ext2\n");
            out.push_str(&format!("    linux {} {}\n", entry.kernel_path, entry.cmdline_options));
            for initrd in &entry.initrd_paths {
                out.push_str(&format!("    initrd {}\n", initrd));
            }
            out.push_str("    boot\n");
            out.push_str("}\n\n");
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freebsd_loader_parsing() {
        let conf = r#"
            # FreeBSD loader.conf
            kernel="kernel"
            autoboot_delay="5"
            zfs_load="YES"
            geom_mirror_load="YES"
            kern.geom.label.disk_ident.enable="0"
        "#;
        let cfg = FreeBsdLoaderConfig::parse(conf);
        assert_eq!(cfg.boot_delay, 5);
        assert!(cfg.zfs_load);
        assert_eq!(cfg.loaded_modules, vec!["geom_mirror".to_string()]);
        assert_eq!(cfg.custom_vars.get("kern.geom.label.disk_ident.enable").unwrap(), "0");

        let entry = cfg.to_universal_entry();
        assert_eq!(entry.source_kind, BootloaderKind::FreeBsdLoader);
        assert!(entry.cmdline_options.contains("zfs_load=YES"));
    }

    #[test]
    fn test_openbsd_boot_parsing() {
        let conf = r#"
            stty com0 9600
            set tty com0
            set timeout 10
            boot sd0a:/bsd.sp
        "#;
        let cfg = OpenBsdBootConfig::parse(conf);
        assert_eq!(cfg.timeout, 10);
        assert_eq!(cfg.default_kernel, "sd0a:/bsd.sp");
        assert_eq!(cfg.serial_console, Some("com0".to_string()));

        let entry = cfg.to_universal_entry();
        assert_eq!(entry.source_kind, BootloaderKind::OpenBsdBoot);
        assert_eq!(entry.root_device, "sd0a");
    }

    #[test]
    fn test_systemd_boot_bls_parsing() {
        let conf = r#"
            title Arch Linux
            version 6.6.1-arch1-1
            linux /vmlinuz-linux
            initrd /intel-ucode.img
            initrd /initramfs-linux.img
            options root=UUID=1234-5678 rw quiet
        "#;
        let entry = SystemdBootBlsEntry::parse(conf);
        assert_eq!(entry.title, "Arch Linux");
        assert_eq!(entry.linux, "/vmlinuz-linux");
        assert_eq!(entry.initrd.len(), 2);
        assert!(entry.options.contains("UUID=1234-5678"));

        let bls_text = entry.generate_bls_content();
        assert!(bls_text.contains("title Arch Linux"));
        assert!(bls_text.contains("initrd /intel-ucode.img"));
    }

    #[test]
    fn test_limine_parsing() {
        let conf = r#"
            TIMEOUT: 3
            :SigmaOS Microkernel
                PROTOCOL: limine
                KERNEL_PATH: boot:///boot/sigma_kernel
                CMDLINE: console=ttyS0 quiet
                MODULE_PATH: boot:///boot/initramfs
        "#;
        let cfg = LimineConfig::parse(conf);
        assert_eq!(cfg.timeout, 3);
        let entry = cfg.entries.get("SigmaOS Microkernel").unwrap();
        assert_eq!(entry.protocol, "limine");
        assert_eq!(entry.kernel_path, "boot:///boot/sigma_kernel");
        assert_eq!(entry.modules.len(), 1);
    }

    #[test]
    fn test_refind_parsing() {
        let conf = r#"
            menuentry "SigmaOS Zenith" {
                icon /EFI/refind/icons/os_linux.png
                loader /vmlinuz-sigma
                initrd /initramfs-sigma.img
                options "root=/dev/nvme0n1p2 quiet"
            }
        "#;
        let entry = RefindMenuEntry::parse(conf);
        assert_eq!(entry.name, "SigmaOS Zenith");
        assert_eq!(entry.loader, "/vmlinuz-sigma");
        assert_eq!(entry.initrd, "/initramfs-sigma.img");
        assert!(entry.options.contains("root=/dev/nvme0n1p2"));
    }

    #[test]
    fn test_sovereign_multi_distro_orchestrator() {
        let mut engine = SovereignMultiDistroBootloaderEngine::new();
        engine.import_freebsd_loader("zfs_load=YES");
        engine.import_openbsd_boot("boot /bsd.mp");
        engine.import_bls_entry("title Fedora\nlinux /vmlinuz\noptions root=/dev/sda1");

        assert_eq!(engine.entries.len(), 3);
        let grub = engine.generate_unified_grub2_cfg();
        assert!(grub.contains("menuentry 'FreeBSD / SigmaOS BSD Subsystem Kernel'"));
        assert!(grub.contains("menuentry 'OpenBSD Secure Kernel / SigmaOS Guard'"));
        assert!(grub.contains("menuentry 'Fedora'"));
    }
}
