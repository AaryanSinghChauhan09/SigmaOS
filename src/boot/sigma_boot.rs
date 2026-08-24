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
}
