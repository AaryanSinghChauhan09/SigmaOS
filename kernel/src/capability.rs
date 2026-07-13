// Capability System for SigmaOS
// Implements fine-grained capability tokens

use std::collections::HashSet;
use std::fmt;

/// Capability types in SigmaOS
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Capability {
    // File capabilities
    FileRead,
    FileWrite,
    FileExecute,
    FileCreate,
    FileDelete,
    
    // Network capabilities
    NetworkBind,
    NetworkConnect,
    NetworkListen,
    NetworkAccept,
    
    // Process capabilities
    ProcessCreate,
    ProcessKill,
    ProcessDebug,
    ProcessSignal,
    
    // Memory capabilities
    MemoryAllocate,
    MemoryProtect,
    MemoryShare,
    
    // Device capabilities
    DeviceRead,
    DeviceWrite,
    DeviceControl,
    
    // System capabilities
    SystemAdmin,
    SystemReboot,
    SystemShutdown,
    
    // Security capabilities
    SecurityAudit,
    SecurityPolicy,
    
    // IPC capabilities
    IPCSend,
    IPCReceive,
    
    // Unknown capability
    Unknown,
}

impl fmt::Display for Capability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Capability::FileRead => write!(f, "file:read"),
            Capability::FileWrite => write!(f, "file:write"),
            Capability::FileExecute => write!(f, "file:execute"),
            Capability::FileCreate => write!(f, "file:create"),
            Capability::FileDelete => write!(f, "file:delete"),
            Capability::NetworkBind => write!(f, "network:bind"),
            Capability::NetworkConnect => write!(f, "network:connect"),
            Capability::NetworkListen => write!(f, "network:listen"),
            Capability::NetworkAccept => write!(f, "network:accept"),
            Capability::ProcessCreate => write!(f, "process:create"),
            Capability::ProcessKill => write!(f, "process:kill"),
            Capability::ProcessDebug => write!(f, "process:debug"),
            Capability::ProcessSignal => write!(f, "process:signal"),
            Capability::MemoryAllocate => write!(f, "memory:allocate"),
            Capability::MemoryProtect => write!(f, "memory:protect"),
            Capability::MemoryShare => write!(f, "memory:share"),
            Capability::DeviceRead => write!(f, "device:read"),
            Capability::DeviceWrite => write!(f, "device:write"),
            Capability::DeviceControl => write!(f, "device:control"),
            Capability::SystemAdmin => write!(f, "system:admin"),
            Capability::SystemReboot => write!(f, "system:reboot"),
            Capability::SystemShutdown => write!(f, "system:shutdown"),
            Capability::SecurityAudit => write!(f, "security:audit"),
            Capability::SecurityPolicy => write!(f, "security:policy"),
            Capability::IPCSend => write!(f, "ipc:send"),
            Capability::IPCReceive => write!(f, "ipc:receive"),
            Capability::Unknown => write!(f, "unknown"),
        }
    }
}

/// A set of capabilities
#[derive(Debug, Clone)]
pub struct CapabilitySet {
    capabilities: HashSet<Capability>,
}

impl CapabilitySet {
    /// Create a new empty capability set
    pub fn new() -> Self {
        Self {
            capabilities: HashSet::new(),
        }
    }

    /// Insert a capability into the set
    pub fn insert(&mut self, capability: Capability) {
        self.capabilities.insert(capability);
    }

    /// Remove a capability from the set
    pub fn remove(&mut self, capability: &Capability) {
        self.capabilities.remove(capability);
    }

    /// Check if the set contains a capability
    pub fn has(&self, capability: &Capability) -> bool {
        self.capabilities.contains(capability)
    }

    /// Get all capabilities
    pub fn all(&self) -> &HashSet<Capability> {
        &self.capabilities
    }

    /// Merge another capability set into this one
    pub fn merge(&mut self, other: CapabilitySet) {
        for cap in other.capabilities {
            self.capabilities.insert(cap);
        }
    }
}

impl Default for CapabilitySet {
    fn default() -> Self {
        Self::new()
    }
}

/// A capability token for delegation
#[derive(Debug, Clone)]
pub struct CapabilityToken {
    capability: Capability,
    issuer_pid: u64,
    target_pid: u64,
    timestamp: u64,
    expiry: Option<u64>,
}

impl CapabilityToken {
    /// Create a new capability token
    pub fn new(capability: Capability, issuer_pid: u64, target_pid: u64) -> Self {
        Self {
            capability,
            issuer_pid,
            target_pid,
            timestamp: Self::get_timestamp(),
            expiry: None,
        }
    }

    /// Create a token with expiry
    pub fn with_expiry(capability: Capability, issuer_pid: u64, target_pid: u64, expiry_ms: u64) -> Self {
        Self {
            capability,
            issuer_pid,
            target_pid,
            timestamp: Self::get_timestamp(),
            expiry: Some(Self::get_timestamp() + expiry_ms),
        }
    }

    /// Check if token is valid for a process
    pub fn is_valid_for(&self, pid: u64) -> bool {
        if self.target_pid != pid {
            return false;
        }

        if let Some(expiry) = self.expiry {
            if Self::get_timestamp() > expiry {
                return false;
            }
        }

        true
    }

    /// Get the capability
    pub fn capability(&self) -> &Capability {
        &self.capability
    }

    /// Get current timestamp
    fn get_timestamp() -> u64 {
        // In real implementation, this would get actual timestamp
        0
    }
}
