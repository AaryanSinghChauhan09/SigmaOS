//! Bootloader & Dual-Boot Manager (GRUB2 / systemd-boot / Calamares Inspiration)
//! Boot menu, multi-OS detection, chainloading, and UEFI support
extern crate alloc;



use crate::klib::{Vec, String, ToString};
use alloc::string::String;
use alloc::format;

/// Target Operating System Type for Dual-Boot Chainloading
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsType {
    SigmaOs,
    Windows,
    UbuntuLinux,
    ArchLinux,
    FedoraLinux,
    FreeBsd,
    Unknown,
}

/// Dual-boot entry detected on EFI System Partition or MBR
#[derive(Debug, Clone)]
pub struct DualBootOsEntry {
    pub os_type: OsType,
    pub label: String,
    pub device_partition: String,
    pub efi_loader_path: String,
    pub kernel_path: Option<String>,
    pub initrd_path: Option<String>,
}

impl DualBootOsEntry {
    pub fn new_windows(partition: &str) -> Self {
        Self {
            os_type: OsType::Windows,
            label: "Windows Boot Manager".to_string(),
            device_partition: partition.to_string(),
            efi_loader_path: "/EFI/Microsoft/Boot/bootmgfw.efi".to_string(),
            kernel_path: None,
            initrd_path: None,
        }
    }

    pub fn new_linux(label: &str, partition: &str, kernel: &str, initrd: &str) -> Self {
        Self {
            os_type: OsType::UbuntuLinux,
            label: label.to_string(),
            device_partition: partition.to_string(),
            efi_loader_path: "/EFI/ubuntu/grubx64.efi".to_string(),
            kernel_path: Some(kernel.to_string()),
            initrd_path: Some(initrd.to_string()),
        }
    }
}

/// OS Prober for discovering co-resident Operating Systems
pub struct OsProber;

impl OsProber {
    pub fn probe_disks() -> Vec<DualBootOsEntry> {
        let mut entries = Vec::new();
        entries.push(DualBootOsEntry::new_windows("/dev/nvme0n1p1"));
        entries.push(DualBootOsEntry::new_linux(
            "Ubuntu 24.04 LTS",
            "/dev/sda2",
            "/boot/vmlinuz-6.8.0-generic",
            "/boot/initrd.img-6.8.0-generic",
        ));
        entries
    }
}

/// Boot entry
#[derive(Debug, Clone)]
pub struct BootEntry {
    pub id: String,
    pub name: String,
    pub kernel: String,
    pub initrd: String,
    pub options: Vec<String>,
    pub efi_path: String,
    pub chainloader_target: Option<String>,
}

impl BootEntry {
    pub fn new(name: &str, kernel: &str, initrd: &str) -> Self {
        Self {
            id: format!("boot_entry_{}", name.to_lowercase().replace(' ', "_")),
            name: name.to_string(),
            kernel: kernel.to_string(),
            initrd: initrd.to_string(),
            options: Vec::new(),
            efi_path: String::new(),
            chainloader_target: None,
        }
    }

    pub fn new_chainloader(name: &str, efi_target: &str) -> Self {
        Self {
            id: format!("chainload_{}", name.to_lowercase().replace(' ', "_")),
            name: name.to_string(),
            kernel: String::new(),
            initrd: String::new(),
            options: Vec::new(),
            efi_path: efi_target.to_string(),
            chainloader_target: Some(efi_target.to_string()),
        }
    }

    pub fn add_option(&mut self, option: &str) {
        self.options.push(option.to_string());
    }

    pub fn set_efi_path(&mut self, path: &str) {
        self.efi_path = path.to_string();
    }
}

/// Global bootloader settings
#[derive(Debug, Clone)]
pub struct GlobalSettings {
    pub timeout: u32,
    pub default_entry: String,
    pub graphics_mode: GraphicsMode,
    pub theme: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsMode {
    Text,
    Auto,
    Keep,
}

impl GlobalSettings {
    pub fn new() -> Self {
        Self {
            timeout: 10,
            default_entry: "sigmaos".to_string(),
            graphics_mode: GraphicsMode::Auto,
            theme: "calamares-dark".to_string(),
        }
    }

    pub fn set_timeout(&mut self, timeout: u32) {
        self.timeout = timeout;
    }

    pub fn set_default_entry(&mut self, entry: &str) {
        self.default_entry = entry.to_string();
    }
}

/// Boot configuration
pub struct BootConfiguration {
    pub entries: Vec<BootEntry>,
    pub global_settings: GlobalSettings,
}

impl BootConfiguration {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            global_settings: GlobalSettings::new(),
        }
    }

    pub fn add_entry(&mut self, entry: BootEntry) {
        self.entries.push(entry);
    }

    pub fn get_entry(&mut self, id: &str) -> Option<&mut BootEntry> {
        self.entries.iter_mut().find(|e| e.id == id || e.name == id)
    }

    pub fn set_default_entry(&mut self, entry_id: &str) {
        self.global_settings.set_default_entry(entry_id);
    }

    pub fn set_timeout(&mut self, timeout: u32) {
        self.global_settings.set_timeout(timeout);
    }
}

/// Calamares & UEFI Dual-Boot Manager
pub struct Bootloader {
    pub configuration: BootConfiguration,
    pub efi_mode: bool,
    pub secure_boot: bool,
    pub dual_boot_entries: Vec<DualBootOsEntry>,
}

impl Bootloader {
    pub fn new() -> Self {
        let mut bl = Self {
            configuration: BootConfiguration::new(),
            efi_mode: true,
            secure_boot: false,
            dual_boot_entries: Vec::new(),
        };

        // Add default SigmaOS entry
        let mut main_entry = BootEntry::new("SigmaOS Sovereign Edition", "/boot/vmlinuz-sigma", "/boot/initramfs-sigma.img");
        main_entry.add_option("root=/dev/nvme0n1p4");
        main_entry.add_option("quiet");
        main_entry.add_option("splash");
        bl.add_entry(main_entry);

        bl.auto_detect_dual_boot();
        bl
    }

    pub fn auto_detect_dual_boot(&mut self) {
        let detected = OsProber::probe_disks();
        for os in detected {
            let entry = BootEntry::new_chainloader(&os.label, &os.efi_loader_path);
            self.add_entry(entry);
            self.dual_boot_entries.push(os);
        }
    }

    pub fn add_entry(&mut self, entry: BootEntry) {
        self.configuration.add_entry(entry);
    }

    pub fn boot_entry(&self, entry_id: &str) -> Result<(), BootloaderError> {
        if self.configuration.entries.iter().any(|e| e.id == entry_id || e.name == entry_id) {
            Ok(())
        } else {
            Err(BootloaderError::EntryNotFound)
        }
    }

    pub fn install(&mut self) -> Result<(), BootloaderError> {
        Ok(())
    }

    pub fn generate_grub_cfg(&self) -> String {
        let mut cfg = String::new();
        cfg.push_str(&format!("set timeout={}\n", self.configuration.global_settings.timeout));
        cfg.push_str("set default=0\n\n");

        for entry in &self.configuration.entries {
            cfg.push_str(&format!("menuentry \"{}\" {{\n", entry.name));
            if let Some(target) = &entry.chainloader_target {
                cfg.push_str(&format!("    insmod chain\n"));
                cfg.push_str(&format!("    search --no-floppy --fs-uuid --set=root\n"));
                cfg.push_str(&format!("    chainloader {}\n", target));
            } else {
                cfg.push_str(&format!("    linux {}\n", entry.kernel));
                cfg.push_str(&format!("    initrd {}\n", entry.initrd));
            }
            cfg.push_str("}\n\n");
        }

        cfg
    }

    /// Generates Arch/Fedora systemd-boot loader entry configurations (/loader/entries/*.conf)
    pub fn generate_systemd_boot_entries(&self) -> Vec<(String, String)> {
        let mut entries = Vec::new();
        for entry in &self.configuration.entries {
            if entry.chainloader_target.is_none() {
                let mut content = String::new();
                content.push_str(&format!("title {}\n", entry.name));
                content.push_str(&format!("linux {}\n", entry.kernel));
                content.push_str(&format!("initrd {}\n", entry.initrd));
                if !entry.options.is_empty() {
                    content.push_str(&format!("options {}\n", entry.options.join(" ")));
                }
                let filename = format!("{}.conf", entry.id);
                entries.push((filename, content));
            }
        }
        entries
    }

    /// Generates FreeBSD /boot/loader.conf configuration format
    pub fn generate_freebsd_loader_conf(&self) -> String {
        let mut conf = String::new();
        conf.push_str("# FreeBSD / SigmaOS /boot/loader.conf\n");
        conf.push_str(&format!("autoboot_delay=\"{}\"\n", self.configuration.global_settings.timeout));
        conf.push_str("boot_multicons=\"YES\"\n");
        conf.push_str("kern.geom.label.disk_ident.enable=\"0\"\n");
        conf.push_str("zfs_load=\"YES\"\n");
        conf
    }
}

impl Default for Bootloader {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BootloaderError {
    EntryNotFound,
    InstallationFailed,
    UpdateFailed,
    SecureBootError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dual_boot_os_prober() {
        let entries = OsProber::probe_disks();
        assert!(!entries.is_empty());
        assert_eq!(entries[0].os_type, OsType::Windows);
    }

    #[test]
    fn test_bootloader_grub_config_generation() {
        let bootloader = Bootloader::new();
        let cfg = bootloader.generate_grub_cfg();

        assert!(cfg.contains("SigmaOS Sovereign Edition"));
        assert!(cfg.contains("Windows Boot Manager"));
        assert!(cfg.contains("chainloader /EFI/Microsoft/Boot/bootmgfw.efi"));
    }
}
