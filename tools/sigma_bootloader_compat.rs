// SPDX-License-Identifier: MIT
//! SigmaOS Dual Boot & OS Prober Manager Tool
//! Safe, zero-dependency, `#![no_std]` compliant utility for detecting operating systems
//! and managing multi-boot loader configurations.

#![no_std]

/// Detected Operating System architecture/type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsType {
    Windows,
    Linux,
    MacOS,
    FreeBSD,
    SigmaOS,
    Unknown,
}

/// Information about a discovered bootable operating system partition.
#[derive(Debug, Clone, Copy)]
pub struct DiscoveredOs {
    pub os_type: OsType,
    pub partition_id: u8,
    pub partition_type_guid: &'static str,
    pub efi_loader_path: &'static str,
    pub label: &'static str,
}

/// GRUB / systemd-boot compatible Boot Menu Entry configuration.
#[derive(Debug, Clone, Copy)]
pub struct BootMenuEntry {
    pub entry_id: u8,
    pub title: &'static str,
    pub efi_path: &'static str,
    pub kernel_args: &'static str,
    pub is_default: bool,
}

/// OS Prober for probing partition tables and EFI System Partitions (ESP).
pub struct OsProber {
    pub discovered: [Option<DiscoveredOs>; 8],
    pub count: usize,
}

impl OsProber {
    pub fn new() -> Self {
        Self {
            discovered: [None; 8],
            count: 0,
        }
    }

    /// Probes a simulated disk partition table signature and registers any found OS.
    pub fn probe_partition(&mut self, partition_id: u8, partition_type_guid: &'static str, efi_loader_path: &'static str, label: &'static str) -> bool {
        if self.count >= self.discovered.len() {
            return false;
        }

        let os_type = match efi_loader_path {
            p if p.contains("bootmgfw.efi") || p.contains("Microsoft") => OsType::Windows,
            p if p.contains("vmlinuz") || p.contains("grub") || p.contains("systemd") => OsType::Linux,
            p if p.contains("boot.efi") || p.contains("Apple") => OsType::MacOS,
            p if p.contains("loader.efi") || p.contains("FreeBSD") => OsType::FreeBSD,
            p if p.contains("sigma_kernel") || p.contains("SigmaOS") => OsType::SigmaOS,
            _ => OsType::Unknown,
        };

        self.discovered[self.count] = Some(DiscoveredOs {
            os_type,
            partition_id,
            partition_type_guid,
            efi_loader_path,
            label,
        });
        self.count += 1;
        true
    }
}

impl Default for OsProber {
    fn default() -> Self {
        Self::new()
    }
}

/// Dual Boot Manager for organizing boot options, setting defaults, and generating systemd-boot/GRUB configs.
pub struct DualBootManager {
    pub entries: [Option<BootMenuEntry>; 8],
    pub timeout_seconds: u8,
    pub default_entry_id: u8,
}

impl DualBootManager {
    pub fn new(timeout_seconds: u8) -> Self {
        Self {
            entries: [None; 8],
            timeout_seconds,
            default_entry_id: 0,
        }
    }

    /// Populates boot menu entries directly from probed OS targets.
    pub fn populate_from_prober(&mut self, prober: &OsProber) -> usize {
        let mut added = 0;
        for item in prober.discovered.iter().flatten() {
            if added >= self.entries.len() {
                break;
            }

            let kernel_args = match item.os_type {
                OsType::SigmaOS => "root=UUID=sigma_root quiet splash init=/sbin/init",
                OsType::Linux => "root=UUID=linux_root ro quiet splash",
                OsType::Windows => "bootmgr",
                OsType::MacOS => "arch=x86_64",
                OsType::FreeBSD => "boot_multicons=YES",
                OsType::Unknown => "quiet",
            };

            let entry_id = item.partition_id;
            let is_default = item.os_type == OsType::SigmaOS;

            if is_default {
                self.default_entry_id = entry_id;
            }

            self.entries[added] = Some(BootMenuEntry {
                entry_id,
                title: item.label,
                efi_path: item.efi_loader_path,
                kernel_args,
                is_default,
            });
            added += 1;
        }
        added
    }

    /// Sets the active default boot entry ID.
    pub fn set_default_entry(&mut self, entry_id: u8) -> bool {
        let mut found = false;
        for entry in self.entries.iter_mut().flatten() {
            if entry.entry_id == entry_id {
                entry.is_default = true;
                self.default_entry_id = entry_id;
                found = true;
            } else {
                entry.is_default = false;
            }
        }
        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_os_prober_and_dual_boot() {
        let mut prober = OsProber::new();
        assert!(prober.probe_partition(1, "EBD0A0A2-B9E5-4433-87C0-68B6B72699C7", "/EFI/Microsoft/Boot/bootmgfw.efi", "Windows 11 Pro"));
        assert!(prober.probe_partition(2, "0FC63DAF-8483-4772-8E79-3D69D8477DE4", "/EFI/SigmaOS/sigma_kernel.efi", "SigmaOS 2.0 Sovereign"));

        assert_eq!(prober.count, 2);
        assert_eq!(prober.discovered[0].unwrap().os_type, OsType::Windows);
        assert_eq!(prober.discovered[1].unwrap().os_type, OsType::SigmaOS);

        let mut manager = DualBootManager::new(5);
        let count = manager.populate_from_prober(&prober);
        assert_eq!(count, 2);

        assert_eq!(manager.default_entry_id, 2); // Default auto-set to SigmaOS
        assert!(manager.set_default_entry(1)); // Switch default to Windows
        assert_eq!(manager.default_entry_id, 1);
    }
}
