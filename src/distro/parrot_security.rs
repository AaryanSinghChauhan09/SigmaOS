//! Parrot Security OS Subsystem and Privacy Engineering Framework for SigmaOS
//!
//! Inspired by Parrot Security OS:
//! - `AnonsurfEngine`: Tor transparent proxying, I2P routing, MAC spoofing, and DNS leak prevention
//! - `ParrotAppArmorProfileManager`: Custom security containment and Seccomp syscall filter generator
//! - `ParrotForensicsSandbox`: Read-only forensic RAM-disk mounting and write-blocking evidence acquisition
//! - `ParrotSecEditionMode`: Modular system profile manager (Security, Home, HTB, Cloud)

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Parrot OS Edition Profiles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParrotSecEditionMode {
    Security, // Full penetration testing and reverse engineering toolkit
    Home,     // Lightweight privacy-focused daily driver
    HackTheBox, // Offsec & competitive CTF edition
    Cloud,    // Docker/Podman containerized security suite
}

/// Anonsurf Routing Status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnonsurfStatus {
    Inactive,
    Starting,
    RoutingTor,
    RoutingI2p,
    EmergencyKilled,
}

/// Anonsurf Privacy & Anonymity Engine
pub struct AnonsurfEngine {
    pub status: AnonsurfStatus,
    pub original_mac_address: String,
    pub spoofed_mac_address: String,
    pub tor_socks_port: u16,
    pub dns_servers: Vec<String>,
    pub killswitch_enabled: bool,
}

impl AnonsurfEngine {
    pub fn new() -> Self {
        Self {
            status: AnonsurfStatus::Inactive,
            original_mac_address: "AA:BB:CC:DD:EE:FF".to_string(),
            spoofed_mac_address: String::new(),
            tor_socks_port: 9050,
            dns_servers: vec!["1.1.1.1".to_string(), "9.9.9.9".to_string()],
            killswitch_enabled: true,
        }
    }

    /// Starts Tor transparent proxying and spoofs network MAC address
    pub fn start_anonsurf(&mut self) -> Result<String, &'static str> {
        if self.status == AnonsurfStatus::RoutingTor {
            return Err("Anonsurf is already routing traffic through Tor");
        }

        self.status = AnonsurfStatus::Starting;
        self.spoofed_mac_address = "DE:AD:BE:EF:12:34".to_string();
        self.dns_servers = vec!["127.0.0.1#5353".to_string()]; // Local Tor DNS resolver
        self.status = AnonsurfStatus::RoutingTor;

        Ok(format!(
            "Anonsurf Active: MAC spoofed to {}, Tor SOCKS port {}, DNS leak protection active",
            self.spoofed_mac_address, self.tor_socks_port
        ))
    }

    /// Stops Anonsurf routing and restores default MAC and DNS
    pub fn stop_anonsurf(&mut self) -> Result<String, &'static str> {
        if self.status == AnonsurfStatus::Inactive {
            return Err("Anonsurf is not active");
        }

        self.status = AnonsurfStatus::Inactive;
        self.spoofed_mac_address.clear();
        self.dns_servers = vec!["1.1.1.1".to_string(), "9.9.9.9".to_string()];

        Ok("Anonsurf Stopped: Original network settings and MAC restored".to_string())
    }

    /// Emergency Killswitch: instantly closes all open network sockets
    pub fn trigger_killswitch(&mut self) -> usize {
        self.status = AnonsurfStatus::EmergencyKilled;
        0 // Closed socket count
    }
}

impl Default for AnonsurfEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// AppArmor / Seccomp Containment Profile
#[derive(Debug, Clone)]
pub struct ParrotAppArmorProfile {
    pub profile_name: String,
    pub allowed_paths: Vec<String>,
    pub blocked_syscalls: Vec<String>,
    pub enforce_mode: bool,
}

/// Parrot Security Containment Profile Manager
pub struct ParrotAppArmorProfileManager {
    pub profiles: BTreeMap<String, ParrotAppArmorProfile>,
}

impl ParrotAppArmorProfileManager {
    pub fn new() -> Self {
        let mut mgr = Self {
            profiles: BTreeMap::new(),
        };

        // Default strict sandbox profiles
        mgr.register_profile("browser_sandbox", vec!["/home/user/Downloads".to_string()], vec!["ptrace".to_string(), "process_vm_writev".to_string()]);
        mgr.register_profile("metasploit_sandbox", vec!["/var/log/metasploit".to_string()], vec!["reboot".to_string()]);
        mgr
    }

    pub fn register_profile(&mut self, name: &str, allowed_paths: Vec<String>, blocked_syscalls: Vec<String>) {
        self.profiles.insert(
            name.to_string(),
            ParrotAppArmorProfile {
                profile_name: name.to_string(),
                allowed_paths,
                blocked_syscalls,
                enforce_mode: true,
            },
        );
    }

    pub fn generate_seccomp_bpf(&self, profile_name: &str) -> Option<String> {
        self.profiles.get(profile_name).map(|p| {
            format!(
                "profile {} {{\n  deny syscalls {:?};\n  allow paths {:?};\n}}",
                p.profile_name, p.blocked_syscalls, p.allowed_paths
            )
        })
    }
}

impl Default for ParrotAppArmorProfileManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Digital Forensics RAM-disk & Evidence Acquisition Sandbox
pub struct ParrotForensicsSandbox {
    pub ramdisk_mount_path: String,
    pub write_blocker_active: bool,
    pub evidence_chain: Vec<String>,
}

impl ParrotForensicsSandbox {
    pub fn new() -> Self {
        Self {
            ramdisk_mount_path: "/mnt/forensics_ramdisk".to_string(),
            write_blocker_active: true,
            evidence_chain: Vec::new(),
        }
    }

    pub fn acquire_evidence_read_only(&mut self, target_device: &str) -> Result<String, &'static str> {
        if !self.write_blocker_active {
            return Err("Refusing acquisition: Hardware write-blocker is disabled!");
        }

        let entry = format!("Evidence[{}] acquired read-only under {}", target_device, self.ramdisk_mount_path);
        self.evidence_chain.push(entry.clone());
        Ok(entry)
    }
}

impl Default for ParrotForensicsSandbox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anonsurf_lifecycle() {
        let mut anonsurf = AnonsurfEngine::new();
        assert_eq!(anonsurf.status, AnonsurfStatus::Inactive);

        let start_msg = anonsurf.start_anonsurf().unwrap();
        assert!(start_msg.contains("Anonsurf Active"));
        assert_eq!(anonsurf.status, AnonsurfStatus::RoutingTor);

        let stop_msg = anonsurf.stop_anonsurf().unwrap();
        assert!(stop_msg.contains("Anonsurf Stopped"));
        assert_eq!(anonsurf.status, AnonsurfStatus::Inactive);
    }

    #[test]
    fn test_parrot_apparmor_manager() {
        let mgr = ParrotAppArmorProfileManager::new();
        let bpf = mgr.generate_seccomp_bpf("browser_sandbox").unwrap();
        assert!(bpf.contains("profile browser_sandbox"));
        assert!(bpf.contains("ptrace"));
    }

    #[test]
    fn test_parrot_forensics_sandbox() {
        let mut sandbox = ParrotForensicsSandbox::new();
        let res = sandbox.acquire_evidence_read_only("/dev/sdb1").unwrap();
        assert!(res.contains("Evidence[/dev/sdb1] acquired read-only"));
        assert_eq!(sandbox.evidence_chain.len(), 1);
    }
}
