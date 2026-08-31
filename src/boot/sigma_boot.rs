//! Advanced Boot Manager inspired by GRUB2, systemd-boot, and rEFInd
//! Multi-boot entry management, measured boot TPM PCR registers, custom themes,
//! and fallback boot recovery environments.
extern crate alloc;


use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct BootEntry {
    pub id: String,
    pub title: String,
    pub kernel_path: String,
    pub initrd_path: Option<String>,
    pub cmdline_params: String,
    pub is_default: bool,
    pub is_recovery: bool,
}

#[derive(Debug, Clone)]
pub struct BootTheme {
    pub name: String,
    pub background_color_rgb: (u8, u8, u8),
    pub text_color_rgb: (u8, u8, u8),
    pub highlight_color_rgb: (u8, u8, u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandoffProtocol {
    LinuxEfiStub,
    Multiboot2,
    FreeBsdBtxElf,
    OpenBsdBootConf,
    LiveIsoOverlayFs,
}

#[derive(Debug, Clone)]
pub struct SovereignDistroBootStageHandoff {
    pub protocol: HandoffProtocol,
    pub root_uuid: String,
    pub is_initramfs_mounted: bool,
    pub is_overlayfs_active: bool,
    pub kernel_entry_point_addr: u64,
}

impl SovereignDistroBootStageHandoff {
    pub fn new(protocol: HandoffProtocol, root_uuid: &str) -> Self {
        Self {
            protocol,
            root_uuid: root_uuid.to_string(),
            is_initramfs_mounted: false,
            is_overlayfs_active: false,
            kernel_entry_point_addr: 0x0010_0000,
        }
    }

    pub fn mount_initramfs_vfs(&mut self) -> Result<(), &'static str> {
        if self.root_uuid.is_empty() {
            return Err("Boot Handoff: Root UUID cannot be empty");
        }
        self.is_initramfs_mounted = true;
        Ok(())
    }

    pub fn setup_live_iso_overlayfs(&mut self) -> Result<(), &'static str> {
        if !self.is_initramfs_mounted {
            return Err("Boot Handoff: Initramfs VFS must be mounted before overlayfs setup");
        }
        self.is_overlayfs_active = true;
        Ok(())
    }

    pub fn execute_stage_handoff(&self) -> bool {
        self.is_initramfs_mounted && self.kernel_entry_point_addr > 0
    }
}

pub struct BootManager {
    pub entries: Vec<BootEntry>,
    pub default_entry_id: String,
    pub timeout_seconds: u32,
    pub theme: BootTheme,
    pub is_measured_boot_active: bool,
    pub tpm_pcr_hashes: Vec<[u8; 32]>,
}

impl BootManager {
    pub fn new(timeout_seconds: u32) -> Self {
        let default_theme = BootTheme {
            name: "SigmaOS Sovereign Dark".to_string(),
            background_color_rgb: (15, 23, 42),
            text_color_rgb: (226, 232, 240),
            highlight_color_rgb: (59, 130, 246),
        };

        let mut mgr = Self {
            entries: Vec::new(),
            default_entry_id: String::new(),
            timeout_seconds,
            theme: default_theme,
            is_measured_boot_active: true,
            tpm_pcr_hashes: Vec::new(),
        };

        // Add standard fallback recovery entry
        mgr.add_entry(BootEntry {
            id: "sigmaos-recovery".to_string(),
            title: "SigmaOS Safe Recovery Environment".to_string(),
            kernel_path: "/boot/vmlinuz-recovery.efi".to_string(),
            initrd_path: Some("/boot/initramfs-recovery.img".to_string()),
            cmdline_params: "root=UUID=sigma_root ro recovery nomodeset".to_string(),
            is_default: false,
            is_recovery: true,
        });

        mgr
    }

    pub fn add_entry(&mut self, entry: BootEntry) {
        if entry.is_default || self.entries.is_empty() {
            self.default_entry_id = entry.id.clone();
        }
        self.entries.push(entry);
    }

    pub fn set_default_entry(&mut self, id: &str) -> bool {
        if self.entries.iter().any(|e| e.id == id) {
            self.default_entry_id = id.to_string();
            for e in &mut self.entries {
                e.is_default = e.id == id;
            }
            return true;
        }
        false
    }

    pub fn measure_boot_components(&mut self, stage_bytes: &[u8]) -> [u8; 32] {
        let mut pcr = [0u8; 32];
        for (i, &b) in stage_bytes.iter().enumerate() {
            pcr[i % 32] ^= b.wrapping_add(i as u8);
        }
        self.tpm_pcr_hashes.push(pcr);
        pcr
    }

    pub fn find_root_by_uuid(&self, uuid: &str) -> Option<&BootEntry> {
        self.entries.iter().find(|e| e.cmdline_params.contains(uuid))
    }

    pub fn generate_bootloader_config(&self) -> String {
        let mut cfg = String::new();
        cfg.push_str("# SigmaOS Boot Configuration\n");
        cfg.push_str("timeout ");
        cfg.push_str(&self.timeout_seconds.to_string());
        cfg.push_str("\ndefault ");
        cfg.push_str(&self.default_entry_id);
        cfg.push_str("\n\n");

        for entry in &self.entries {
            cfg.push_str("title ");
            cfg.push_str(&entry.title);
            cfg.push_str("\nkernel ");
            cfg.push_str(&entry.kernel_path);
            cfg.push_str("\noptions ");
            cfg.push_str(&entry.cmdline_params);
            cfg.push_str("\n\n");
        }
        cfg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boot_manager_measured_boot() {
        let mut boot = BootManager::new(5);
        let std_entry = BootEntry {
            id: "sigmaos-main".to_string(),
            title: "SigmaOS 2.0 Sovereign".to_string(),
            kernel_path: "/boot/vmlinuz-sigma.efi".to_string(),
            initrd_path: Some("/boot/initramfs.img".to_string()),
            cmdline_params: "root=UUID=root quiet splash".to_string(),
            is_default: true,
            is_recovery: false,
        };
        boot.add_entry(std_entry);

        assert_eq!(boot.default_entry_id, "sigmaos-main");

        let pcr = boot.measure_boot_components(b"KERNEL_STAGE_1_BINARY");
        assert_ne!(pcr, [0u8; 32]);
        assert_eq!(boot.tpm_pcr_hashes.len(), 1);

        let cfg = boot.generate_bootloader_config();
        assert!(cfg.contains("SigmaOS 2.0 Sovereign"));
    }

    #[test]
    fn test_boot_stage_handoff_and_root_discovery() {
        let mut boot = BootManager::new(3);
        boot.add_entry(BootEntry {
            id: "sigma-root-uuid".to_string(),
            title: "SigmaOS Main Root".to_string(),
            kernel_path: "/boot/vmlinuz".to_string(),
            initrd_path: Some("/boot/initramfs".to_string()),
            cmdline_params: "root=UUID=1234-5678 ro".to_string(),
            is_default: true,
            is_recovery: false,
        });

        let found = boot.find_root_by_uuid("1234-5678");
        assert!(found.is_some());
        assert_eq!(found.unwrap().title, "SigmaOS Main Root");

        let mut handoff = SovereignDistroBootStageHandoff::new(HandoffProtocol::LinuxEfiStub, "1234-5678");
        assert!(!handoff.execute_stage_handoff());

        handoff.mount_initramfs_vfs().unwrap();
        assert!(handoff.is_initramfs_mounted);

        handoff.setup_live_iso_overlayfs().unwrap();
        assert!(handoff.is_overlayfs_active);
        assert!(handoff.execute_stage_handoff());
    }
}
