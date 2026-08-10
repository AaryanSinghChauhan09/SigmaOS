// SigmaOS Parrot Security Parity Implementation
// Implements AnonSurf routing, AppSandbox policy engine, and forensic write-blocker

use crate::klib::SigmaString;
use core::cell::{Cell, RefCell};

/// Routing modes for network traffic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutingMode {
    DirectCleartext,
    TorAnonymized,
    I2pAnonymized,
}

/// AnonSurf routing shunt for anonymous network routing
pub struct AnonSurfShunt {
    pub current_mode: Cell<RoutingMode>,
    pub dns_leak_protection: Cell<bool>,
    pub anonymized_packets_routed: Cell<u64>,
}

impl AnonSurfShunt {
    pub fn new() -> Self {
        AnonSurfShunt {
            current_mode: Cell::new(RoutingMode::DirectCleartext),
            dns_leak_protection: Cell::new(true),
            anonymized_packets_routed: Cell::new(0),
        }
    }

    /// Enable anonymized routing
    pub fn enable_anonsurf(&self) {
        self.current_mode.set(RoutingMode::TorAnonymized);
        self.dns_leak_protection.set(true);
    }

    /// Disable anonymized routing
    pub fn disable_anonsurf(&self) {
        self.current_mode.set(RoutingMode::DirectCleartext);
    }

    /// Route packet through anonymized network
    pub fn shunt_packet(&self, _packet_id: u32, _size_bytes: usize) -> bool {
        if self.current_mode.get() != RoutingMode::DirectCleartext {
            let count = self.anonymized_packets_routed.get();
            self.anonymized_packets_routed.set(count + 1);
            true
        } else {
            false
        }
    }

    /// Get current routing mode
    pub fn current_mode(&self) -> RoutingMode {
        self.current_mode.get()
    }
}

/// Sandbox policy for application security
#[derive(Debug, Clone)]
pub struct SandboxPolicy {
    pub allow_network: bool,
    pub allow_raw_sockets: bool,
    pub allow_filesystem_write: bool,
    pub permitted_subpath: SigmaString,
}

/// AppSandbox engine for process security
pub struct AppSandboxEngine {
    pub current_policy: RefCell<SandboxPolicy>,
}

impl AppSandboxEngine {
    pub fn new() -> Self {
        AppSandboxEngine {
            current_policy: RefCell::new(SandboxPolicy {
                allow_network: false,
                allow_raw_sockets: false,
                allow_filesystem_write: false,
                permitted_subpath: SigmaString::from_str("/sandbox/tmp"),
            }),
        }
    }

    /// Validate filesystem write access
    pub fn validate_filesystem_write(&self, path: &str) -> bool {
        let policy = self.current_policy.borrow();
        if !policy.allow_filesystem_write {
            path.starts_with(policy.permitted_subpath.as_str())
        } else {
            true
        }
    }

    /// Validate network socket creation
    pub fn validate_network_socket(&self, is_raw: bool) -> bool {
        let policy = self.current_policy.borrow();
        if is_raw && !policy.allow_raw_sockets {
            false
        } else if !is_raw && !policy.allow_network {
            false
        } else {
            true
        }
    }

    /// Set sandbox policy
    pub fn set_policy(&self, policy: SandboxPolicy) {
        *self.current_policy.borrow_mut() = policy;
    }

    /// Get current policy
    pub fn current_policy(&self) -> SandboxPolicy {
        self.current_policy.borrow().clone()
    }
}

/// Forensic storage filter for write protection
pub struct ForensicStorageFilter {
    pub is_write_blocked: Cell<bool>,
}

impl ForensicStorageFilter {
    pub fn new() -> Self {
        ForensicStorageFilter {
            is_write_blocked: Cell::new(true),
        }
    }

    /// Set write blocker state
    pub fn set_write_blocker(&self, enabled: bool) {
        self.is_write_blocked.set(enabled);
    }

    /// Intercept device write operations
    pub fn intercept_device_write(&self, _sector_id: u64, _buffer: &[u8]) -> bool {
        !self.is_write_blocked.get()
    }

    /// Secure memory wipe
    pub fn secure_memory_wipe(&self, target_buffer: &mut [u8]) {
        for byte in target_buffer.iter_mut() {
            // SAFETY: We're writing to valid memory within the buffer bounds
            // Using volatile write ensures the compiler doesn't optimize away the zeroing
            unsafe { core::ptr::write_volatile(byte, 0x00); }
        }
    }

    /// Check if write blocking is enabled
    pub fn is_write_blocked(&self) -> bool {
        self.is_write_blocked.get()
    }
}

impl Default for AnonSurfShunt {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for AppSandboxEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ForensicStorageFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SandboxPolicy {
    fn default() -> Self {
        SandboxPolicy {
            allow_network: false,
            allow_raw_sockets: false,
            allow_filesystem_write: false,
            permitted_subpath: SigmaString::from_str("/sandbox/tmp"),
        }
    }
}
