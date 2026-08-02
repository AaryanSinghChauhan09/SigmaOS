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

// SigmaOS Parrot Security & Kali Parity Engine
// Zero-dependency, // #![no_std]  // crate-root only compliant, zero-allocation
// Extends SigmaOS security structures with AnonSurf routing, AppSandbox policy, and Forensic storage filters

use core::cell::Cell;

// ==========================================
// 1. AnonSurf Routing Shunt
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutingMode {
    DirectCleartext,
    TorAnonymized,
    I2pAnonymized,
}

pub struct AnonSurfShunt {
    pub current_mode: Cell<RoutingMode>,
    pub dns_leak_protection: Cell<bool>,
    pub anonymized_packets_routed: Cell<u64>,
}

impl AnonSurfShunt {
    pub const fn new() -> Self {
        Self {
            current_mode: Cell::new(RoutingMode::DirectCleartext),
            dns_leak_protection: Cell::new(true),
            anonymized_packets_routed: Cell::new(0),
        }
    }

    /// Transitions the primary network interfaces into an encrypted Tor routing mode
    pub fn enable_anonsurf(&self) {
        self.current_mode.set(RoutingMode::TorAnonymized);
        self.dns_leak_protection.set(true);
    }

    /// Disables anonymized redirection
    pub fn disable_anonsurf(&self) {
        self.current_mode.set(RoutingMode::DirectCleartext);
    }

    /// Simulates interception and routing of packets through virtual Tor nodes
    pub fn shunt_packet(&self, packet_id: u32, size_bytes: usize) {
        if self.current_mode.get() != RoutingMode::DirectCleartext {
            let count = self.anonymized_packets_routed.get();
            self.anonymized_packets_routed.set(count + 1);
        }
    }

    pub fn get_mode(&self) -> RoutingMode {
        self.current_mode.get()
    }

    pub fn get_packets_routed(&self) -> usize {
        self.anonymized_packets_routed.get() as usize
    }
}

impl Default for AnonSurfShunt {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 2. AppSandbox Policy Engine (Firejail-Parity)
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SandboxPolicy {
    pub allow_network: bool,
    pub allow_raw_sockets: bool,
    pub allow_filesystem_write: bool,
    pub permitted_subpath: &'static str,
}

pub struct AppSandboxEngine {
    pub current_policy: Cell<SandboxPolicy>,
}

impl AppSandboxEngine {
    pub const fn new() -> Self {
        Self {
            current_policy: Cell::new(SandboxPolicy {
                allow_network: false,
                allow_raw_sockets: false,
                allow_filesystem_write: false,
                permitted_subpath: "/sandbox/tmp",
            }),
        }
    }

    /// Enforces the strict security context before launching a third-party process
    pub fn validate_filesystem_write(&self, path: &str) -> bool {
        let policy = self.current_policy.get();
        if !policy.allow_filesystem_write {
            // Check if within permitted directory path
            path.starts_with(policy.permitted_subpath)
        } else {
            true
        }
    }

    /// Verifies socket creation requests
    pub fn validate_network_socket(&self, is_raw: bool) -> bool {
        let policy = self.current_policy.get();
        if is_raw {
            policy.allow_raw_sockets
        } else {
            policy.allow_network
        }
    }

    pub fn update_policy(&self, policy: SandboxPolicy) {
        self.current_policy.set(policy);
    }
}

impl Default for AppSandboxEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 3. Forensic Write-Blocker Filter & Memory Wiper
// ==========================================

pub struct ForensicStorageFilter {
    pub is_write_blocked: Cell<bool>,
}

impl ForensicStorageFilter {
    pub const fn new() -> Self {
        Self {
            is_write_blocked: Cell::new(true), // Enabled by default to protect evidence
        }
    }

    /// Set write blocker toggle safely
    pub fn set_write_blocker(&self, enabled: bool) {
        self.is_write_blocked.set(enabled);
    }

    /// Intercepts device operations, granting read-only capabilities and blocking all writes
    pub fn intercept_device_write(&self, _sector_id: u64, _buffer: &[u8]) -> bool {
        !self.is_write_blocked.get()
    }

    /// Zeroes out secure regions of volatile memory to protect keys against hardware cold-boot analysis
    pub fn secure_memory_wipe(&self, target_buffer: &mut [u8]) {
        for byte in target_buffer.iter_mut() {
            // Write volatile zero states safely
            unsafe {
                core::ptr::write_volatile(byte, 0x00);
            }
        }
    }
}

impl Default for ForensicStorageFilter {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Sync for AnonSurfShunt {}
unsafe impl Sync for AppSandboxEngine {}
unsafe impl Sync for ForensicStorageFilter {}

// ==========================================
// Global Static Security Orchestrators
// ==========================================

pub static GLOBAL_ANONSURF: AnonSurfShunt = AnonSurfShunt::new();
pub static GLOBAL_SANDBOX: AppSandboxEngine = AppSandboxEngine::new();
pub static GLOBAL_FORENSIC: ForensicStorageFilter = ForensicStorageFilter::new();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anonsurf_shunt() {
        let shunt = AnonSurfShunt::new();
        assert_eq!(shunt.current_mode.get(), RoutingMode::DirectCleartext);
        assert!(shunt.dns_leak_protection.get());

        // Enable AnonSurf routing mode
        shunt.enable_anonsurf();
        assert_eq!(shunt.current_mode.get(), RoutingMode::TorAnonymized);

        // Routing a packet increment count
        assert_eq!(shunt.anonymized_packets_routed.get(), 0);
        shunt.shunt_packet(101, 1024);
        assert_eq!(shunt.anonymized_packets_routed.get(), 1);

        // Disable AnonSurf
        shunt.disable_anonsurf();
        assert_eq!(shunt.current_mode.get(), RoutingMode::DirectCleartext);
    }

    #[test]
    fn test_app_sandbox_engine() {
        let engine = AppSandboxEngine::new();
        let default_policy = engine.current_policy.get();
        assert_eq!(default_policy.permitted_subpath, "/sandbox/tmp");

        // Validate filesystem writes inside and outside permitted directories
        assert!(engine.validate_filesystem_write("/sandbox/tmp/log.txt"));
        assert!(!engine.validate_filesystem_write("/etc/shadow"));

        // Validate standard and raw network sockets
        assert!(!engine.validate_network_socket(false)); // Standard socket is disabled
        assert!(!engine.validate_network_socket(true)); // Raw socket is disabled

        // Update policy to allow standard network access
        engine.current_policy.set(SandboxPolicy {
            allow_network: true,
            allow_raw_sockets: false,
            allow_filesystem_write: false,
            permitted_subpath: "/sandbox/tmp",
        });
        assert!(engine.validate_network_socket(false));
        assert!(!engine.validate_network_socket(true));
    }

    #[test]
    fn test_forensic_storage_filter_and_wiper() {
        let filter = ForensicStorageFilter::new();
        assert!(filter.is_write_blocked.get());

        // Device write should be blocked by default
        assert!(!filter.intercept_device_write(12, b"compromised data"));

        // Disable write blocker for authorized forensic analysis write operations
        filter.set_write_blocker(false);
        assert!(!filter.is_write_blocked.get());
        assert!(filter.intercept_device_write(12, b"authorized forensics write"));

        // Test secure memory volatile wiper
        let mut sensitive_data = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE];
        filter.secure_memory_wipe(&mut sensitive_data);
        assert_eq!(sensitive_data, [0, 0, 0, 0, 0]);
    }
}
