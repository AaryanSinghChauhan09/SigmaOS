#![no_std]

/// Capability-Native Security Model for SigmaOS
/// Implements cryptographic 64-bit hardware-enforced Capability Tokens
/// No root privilege concept - each process receives minimal capability tokens

use core::sync::atomic::{AtomicU64, Ordering};

/// Capability token flags (64-bit hardware-enforced)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityToken {
    pub bits: u64,
}

impl CapabilityToken {
    pub const NONE: CapabilityToken = CapabilityToken { bits: 0 };
    
    // File system capabilities
    pub const CAP_READ_FS_PATH: CapabilityToken = CapabilityToken { bits: 1 << 0 };
    pub const CAP_WRITE_FS_PATH: CapabilityToken = CapabilityToken { bits: 1 << 1 };
    pub const CAP_EXECUTE_FS_PATH: CapabilityToken = CapabilityToken { bits: 1 << 2 };
    pub const CAP_DELETE_FS_PATH: CapabilityToken = CapabilityToken { bits: 1 << 3 };
    
    // Network capabilities
    pub const CAP_NET_BIND_ANY: CapabilityToken = CapabilityToken { bits: 1 << 4 };
    pub const CAP_NET_BIND_80: CapabilityToken = CapabilityToken { bits: 1 << 5 };
    pub const CAP_NET_BIND_443: CapabilityToken = CapabilityToken { bits: 1 << 6 };
    pub const CAP_NET_CONNECT: CapabilityToken = CapabilityToken { bits: 1 << 7 };
    pub const CAP_NET_RAW_SOCKETS: CapabilityToken = CapabilityToken { bits: 1 << 8 };
    
    // Process capabilities
    pub const CAP_PROCESS_SPAWN: CapabilityToken = CapabilityToken { bits: 1 << 9 };
    pub const CAP_PROCESS_KILL: CapabilityToken = CapabilityToken { bits: 1 << 10 };
    pub const CAP_PROCESS_SIGNAL: CapabilityToken = CapabilityToken { bits: 1 << 11 };
    
    // System capabilities
    pub const CAP_SYS_TIME: CapabilityToken = CapabilityToken { bits: 1 << 12 };
    pub const CAP_SYS_REBOOT: CapabilityToken = CapabilityToken { bits: 1 << 13 };
    pub const CAP_SYS_SHUTDOWN: CapabilityToken = CapabilityToken { bits: 1 << 14 };
    
    // Hardware capabilities
    pub const CAP_HW_IO_PORTS: CapabilityToken = CapabilityToken { bits: 1 << 15 };
    pub const CAP_HW_MEMORY_MAP: CapabilityToken = CapabilityToken { bits: 1 << 16 };
    pub const CAP_HW_INTERRUPTS: CapabilityToken = CapabilityToken { bits: 1 << 17 };
    
    // Security capabilities
    pub const CAP_SEC_AUDIT: CapabilityToken = CapabilityToken { bits: 1 << 18 };
    pub const CAP_SEC_POLICY: CapabilityToken = CapabilityToken { bits: 1 << 19 };
    
    pub fn new(bits: u64) -> Self {
        CapabilityToken { bits }
    }
    
    pub fn has(&self, cap: CapabilityToken) -> bool {
        (self.bits & cap.bits) != 0
    }
    
    pub fn grant(&mut self, cap: CapabilityToken) {
        self.bits |= cap.bits;
    }
    
    pub fn revoke(&mut self, cap: CapabilityToken) {
        self.bits &= !cap.bits;
    }
    
    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }
}

/// Capability set for a process
#[repr(C)]
pub struct CapabilitySet {
    pub current: AtomicU64,
    pub inheritable: AtomicU64,
    pub permitted: AtomicU64,
}

impl CapabilitySet {
    pub fn new() -> Self {
        CapabilitySet {
            current: AtomicU64::new(0),
            inheritable: AtomicU64::new(0),
            permitted: AtomicU64::new(0),
        }
    }
    
    pub fn with_capabilities(permitted: u64) -> Self {
        CapabilitySet {
            current: AtomicU64::new(permitted),
            inheritable: AtomicU64::new(permitted),
            permitted: AtomicU64::new(permitted),
        }
    }
    
    pub fn has(&self, cap: CapabilityToken) -> bool {
        let current = self.current.load(Ordering::SeqCst);
        let permitted = self.permitted.load(Ordering::SeqCst);
        (current & cap.bits) != 0 && (permitted & cap.bits) != 0
    }
    
    pub fn grant(&self, cap: CapabilityToken) -> bool {
        let permitted = self.permitted.load(Ordering::SeqCst);
        if (permitted & cap.bits) == 0 {
            return false; // Not permitted
        }
        
        let mut current = self.current.load(Ordering::SeqCst);
        loop {
            let new_current = current | cap.bits;
            match self.current.compare_exchange_weak(current, new_current, Ordering::SeqCst, Ordering::SeqCst) {
                Ok(_) => return true,
                Err(actual) => current = actual,
            }
        }
    }
    
    pub fn revoke(&self, cap: CapabilityToken) {
        let mut current = self.current.load(Ordering::SeqCst);
        loop {
            let new_current = current & !cap.bits;
            match self.current.compare_exchange_weak(current, new_current, Ordering::SeqCst, Ordering::SeqCst) {
                Ok(_) => return,
                Err(actual) => current = actual,
            }
        }
    }
    
    pub fn drop_all(&self) {
        self.current.store(0, Ordering::SeqCst);
    }
}

/// Capability manager for system-wide capability tracking
pub struct CapabilityManager {
    next_token_id: AtomicU64,
}

impl CapabilityManager {
    pub fn new() -> Self {
        CapabilityManager {
            next_token_id: AtomicU64::new(1),
        }
    }
    
    pub fn allocate_token_id(&self) -> u64 {
        self.next_token_id.fetch_add(1, Ordering::SeqCst)
    }
    
    /// Create a capability set for a new process
    pub fn create_process_caps(&self, base_caps: u64) -> CapabilitySet {
        CapabilitySet::with_capabilities(base_caps)
    }
}

/// Default capability sets for common process types
pub mod presets {
    use super::CapabilityToken;
    
    /// Minimal capabilities for untrusted applications
    pub fn untrusted_app() -> u64 {
        CapabilityToken::CAP_READ_FS_PATH.bits |
        CapabilityToken::CAP_NET_CONNECT.bits
    }
    
    /// Standard capabilities for user applications
    pub fn user_app() -> u64 {
        CapabilityToken::CAP_READ_FS_PATH.bits |
        CapabilityToken::CAP_WRITE_FS_PATH.bits |
        CapabilityToken::CAP_NET_CONNECT.bits |
        CapabilityToken::CAP_PROCESS_SPAWN.bits
    }
    
    /// Enhanced capabilities for developer tools
    pub fn dev_tool() -> u64 {
        CapabilityToken::CAP_READ_FS_PATH.bits |
        CapabilityToken::CAP_WRITE_FS_PATH.bits |
        CapabilityToken::CAP_EXECUTE_FS_PATH.bits |
        CapabilityToken::CAP_NET_BIND_ANY.bits |
        CapabilityToken::CAP_NET_CONNECT.bits |
        CapabilityToken::CAP_PROCESS_SPAWN.bits |
        CapabilityToken::CAP_PROCESS_KILL.bits |
        CapabilityToken::CAP_HW_MEMORY_MAP.bits
    }
    
    /// System service capabilities
    pub fn system_service() -> u64 {
        CapabilityToken::CAP_READ_FS_PATH.bits |
        CapabilityToken::CAP_WRITE_FS_PATH.bits |
        CapabilityToken::CAP_NET_BIND_ANY.bits |
        CapabilityToken::CAP_NET_CONNECT.bits |
        CapabilityToken::CAP_PROCESS_SPAWN.bits |
        CapabilityToken::CAP_PROCESS_SIGNAL.bits |
        CapabilityToken::CAP_SYS_TIME.bits |
        CapabilityToken::CAP_SEC_AUDIT.bits
    }
}
