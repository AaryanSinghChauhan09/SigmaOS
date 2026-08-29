//! Old Linux Release Compatibility & Personality Layer for SigmaOS
//!
//! Based on early Linux kernel releases (0.01, 0.11, 0.12, 0.95, 0.96, 0.97, 0.98, 0.99, 1.0)
//! from the Princeton University history archives.
#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;

use crate::klib::BTreeMap;

/// Historical early Linux release metadata
#[derive(Debug, Clone)]
pub struct OldLinuxRelease {
    pub version: &'static str,
    pub date: &'static str,
    pub primary_advance: &'static str,
    pub min_ram_mb: u32,
}

/// Dynamic personality translator mapping early releases to modern interfaces
pub struct OldLinuxCompatManager {
    pub active_version: &'static str,
    pub releases: BTreeMap<&'static str, OldLinuxRelease>,
    pub port_routing: BTreeMap<u16, String>,
}

impl OldLinuxCompatManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut releases = BTreeMap::new();

        releases.insert(
            "0.01",
            OldLinuxRelease {
                version: "0.01",
                date: "Sept 17, 1991",
                primary_advance: "First raw shell, task switching, harddisk & console driver",
                min_ram_mb: 2,
            },
        );
        releases.insert(
            "0.11",
            OldLinuxRelease {
                version: "0.11",
                date: "Dec 8, 1991",
                primary_advance: "Self-hosting capability, floppy disk driver & virtual consoles",
                min_ram_mb: 2,
            },
        );
        releases.insert(
            "0.12",
            OldLinuxRelease {
                version: "0.12",
                date: "Jan 15, 1992",
                primary_advance: "Math co-processor emulation & virtual memory paging support",
                min_ram_mb: 4,
            },
        );
        releases.insert(
            "0.95",
            OldLinuxRelease {
                version: "0.95",
                date: "March 8, 1992",
                primary_advance: "First Virtual Filesystem (VFS) & basic networking foundation",
                min_ram_mb: 4,
            },
        );
        releases.insert(
            "0.96",
            OldLinuxRelease {
                version: "0.96",
                date: "May 22, 1992",
                primary_advance: "X Window System support & TCP/IP loopback",
                min_ram_mb: 4,
            },
        );
        releases.insert(
            "0.97",
            OldLinuxRelease {
                version: "0.97",
                date: "July 24, 1992",
                primary_advance: "Introduction of the Ext filesystem & early sound card drivers",
                min_ram_mb: 4,
            },
        );
        releases.insert(
            "0.98",
            OldLinuxRelease {
                version: "0.98",
                date: "Oct 29, 1992",
                primary_advance: "Sound Blaster 16 & Ethernet card support (NE2000)",
                min_ram_mb: 4,
            },
        );
        releases.insert(
            "0.99",
            OldLinuxRelease {
                version: "0.99",
                date: "Dec 13, 1992",
                primary_advance: "High stability, pre-1.0 series with TCP/IP standard stack",
                min_ram_mb: 4,
            },
        );
        releases.insert("1.0", OldLinuxRelease {
            version: "1.0",
            date: "March 14, 1994",
            primary_advance: "Official stable release, fully functional network stack & standard POSIX compliance",
            min_ram_mb: 8,
        });

        let mut port_routing = BTreeMap::new();
        // Setup default port maps for ancient peripherals
        port_routing.insert(0x3F0, "Floppy Disk Controller".to_string());
        port_routing.insert(0x220, "Sound Blaster 16".to_string());
        port_routing.insert(0x378, "Parallel Printer Port".to_string());
        port_routing.insert(0x3D4, "CGA Video Controller".to_string());
        port_routing.insert(0x3F8, "Serial Port COM1".to_string());

        OldLinuxCompatManager {
            active_version: "0.11",
            releases,
            port_routing,
        }
    }

    pub fn set_active_personality(&mut self, version: &'static str) -> Result<(), &'static str> {
        if self.releases.contains_key(version) {
            self.active_version = version;
            Ok(())
        } else {
            Err("Unsupported old Linux release version")
        }
    }

    /// Emulates execution of an old syscall with customized routing based on the active personality
    pub fn emulate_syscall(&self, num: u32, args: &[u64]) -> Result<String, &'static str> {
        let release = self
            .releases
            .get(self.active_version)
            .ok_or("No active release loaded")?;

        match num {
            1 => {
                // sys_exit
                Ok(format!(
                    "Linux v{} (released {}): Handled exit with code {}",
                    release.version, release.date, args[0]
                ))
            }
            2 => {
                // sys_fork
                if release.min_ram_mb <= 2 {
                    Ok(format!(
                        "Linux v{} (released {}): Spawned lightweight single-segment thread",
                        release.version, release.date
                    ))
                } else {
                    Ok(format!(
                        "Linux v{} (released {}): Spawned full virtual-paged child process",
                        release.version, release.date
                    ))
                }
            }
            4 => {
                // sys_write
                Ok(format!(
                    "Linux v{} (released {}): Emulated write to descriptor {} of len {}",
                    release.version, release.date, args[0], args[2]
                ))
            }
            _ => Err("Syscall not supported in this legacy personality"),
        }
    }

    /// Emulates hardware port access for obsolete devices based on the historical context
    pub fn handle_port_io(&self, port: u16, data: u8) -> Result<String, &'static str> {
        if let Some(device) = self.port_routing.get(&port) {
            Ok(format!(
                "Personality v{} routed port 0x{:X} to: '{}' with data 0x{:X}",
                self.active_version, port, device, data
            ))
        } else {
            Err("Port not registered in ancient peripheral routing list")
        }
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_old_linux_release_metadata() {
        let manager = OldLinuxCompatManager::new();
        let rel_0_01 = manager.releases.get("0.01").unwrap();
        assert_eq!(rel_0_01.version, "0.01");
        assert_eq!(rel_0_01.min_ram_mb, 2);
        assert!(rel_0_01.primary_advance.contains("First raw shell"));
    }

    #[test]
    fn test_personality_sys_exit_fork() {
        let mut manager = OldLinuxCompatManager::new();

        // Emulate under 0.01 personality
        assert!(manager.set_active_personality("0.01").is_ok());
        let exit_msg = manager.emulate_syscall(1, &[42]).unwrap();
        assert!(exit_msg.contains("Handled exit with code 42"));

        let fork_0_01 = manager.emulate_syscall(2, &[]).unwrap();
        assert!(fork_0_01.contains("Spawned lightweight single-segment thread"));

        // Emulate under 0.12 personality (Paging introduced)
        assert!(manager.set_active_personality("0.12").is_ok());
        let fork_0_12 = manager.emulate_syscall(2, &[]).unwrap();
        assert!(fork_0_12.contains("Spawned full virtual-paged child process"));
    }

    #[test]
    fn test_ancient_port_io_routing() {
        let manager = OldLinuxCompatManager::new();
        // Route Floppy Controller IO
        let floppy_msg = manager.handle_port_io(0x3F0, 0x01).unwrap();
        assert!(floppy_msg.contains("Floppy Disk Controller"));
        assert!(floppy_msg.contains("routed port 0x3F0"));
    }
}
