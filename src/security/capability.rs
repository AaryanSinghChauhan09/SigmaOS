// SigmaOS Capability-Based Security System
// Implements 64-bit hardware-enforced capability model

use core::sync::atomic::{AtomicU64, Ordering};

/// Capability token representing access rights
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityToken {
    /// 64-bit capability bitmask
    bits: u64,
}

impl Default for CapabilityToken {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilityToken {
    /// Create a new capability token with no permissions
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    /// Create capability token from raw bits
    pub fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    /// Allow network access
    pub fn allow_network(mut self, protocol: &str, port: u16) -> Self {
        match protocol {
            "tcp" => self.bits |= 1 << 0,
            "udp" => self.bits |= 1 << 1,
            _ => {}
        }
        // Mask and clear target bit ranges (bits 16-31) to prevent bitmask overlap privilege escalation
        self.bits &= !(0xFFFF_u64 << 16);
        self.bits |= (port as u64) << 16;
        self
    }

    /// Allow file read access
    pub fn allow_read(mut self, path: &str) -> Self {
        if path.starts_with("/var/www") {
            self.bits |= 1 << 2;
        }
        self
    }

    /// Allow file write access
    pub fn allow_write(mut self, path: &str) -> Self {
        if path.starts_with("/tmp") || path.starts_with("/home") {
            self.bits |= 1 << 3;
        }
        self
    }

    /// Allow process execution
    pub fn allow_exec(mut self) -> Self {
        self.bits |= 1 << 4;
        self
    }

    /// Allow IPC communication
    pub fn allow_ipc(mut self) -> Self {
        self.bits |= 1 << 5;
        self
    }

    /// Check if capability has specific permission
    pub fn has_permission(&self, permission: Permission) -> bool {
        (self.bits & (1u64 << (permission as u64))) != 0
    }

    /// Revoke all permissions
    pub fn revoke_all(&mut self) {
        self.bits = 0;
    }

    /// Get raw capability bits
    pub fn bits(&self) -> u64 {
        self.bits
    }

    pub fn allow_capability(&mut self, bitmask: u64) {
        self.bits |= bitmask;
    }

    pub fn contains(&self, bitmask: u64) -> bool {
        (self.bits & bitmask) == bitmask
    }
}

/// Permission types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    NetworkTcp = 0,
    NetworkUdp = 1,
    FileRead = 2,
    FileWrite = 3,
    ProcessExec = 4,
    Ipc = 5,
    AudioPlayback = 6,
    DisplayAccess = 7,
}

/// Linux POSIX capability definitions (`capabilities(7)`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum LinuxCapability {
    Chown = 0,
    DacOverride = 1,
    DacReadSearch = 2,
    FOwner = 3,
    FSetId = 4,
    Kill = 5,
    SetGid = 6,
    SetUid = 7,
    SetPcap = 8,
    LinuxImmutable = 9,
    NetBindService = 10,
    NetBroadcast = 11,
    NetAdmin = 12,
    NetRaw = 13,
    IpcLock = 14,
    IpcOwner = 15,
    SysModule = 16,
    SysRawIo = 17,
    SysChroot = 18,
    SysPtrace = 19,
    SysPacct = 20,
    SysAdmin = 21,
    SysBoot = 22,
    SysNice = 23,
    SysResource = 24,
    SysTime = 25,
    SysTtyConfig = 26,
    Mknod = 27,
    Lease = 28,
    AuditWrite = 29,
    AuditControl = 30,
    SetFCap = 31,
    MacOverride = 32,
    MacAdmin = 33,
    Syslog = 34,
    WakeAlarm = 35,
    BlockSuspend = 36,
    AuditRead = 37,
    PerfMon = 38,
    Bpf = 39,
    CheckpointRestore = 40,
}

/// Linux process capability set managing effective, permitted, inheritable, bounding, and ambient sets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinuxCapabilitySet {
    pub effective: u64,
    pub permitted: u64,
    pub inheritable: u64,
    pub bounding: u64,
    pub ambient: u64,
}

impl LinuxCapabilitySet {
    /// Create full capabilities set (e.g. root UID 0 initial state)
    pub fn full() -> Self {
        Self {
            effective: u64::MAX,
            permitted: u64::MAX,
            inheritable: 0,
            bounding: u64::MAX,
            ambient: 0,
        }
    }

    /// Create empty unprivileged capability set
    pub fn empty() -> Self {
        Self {
            effective: 0,
            permitted: 0,
            inheritable: 0,
            bounding: u64::MAX, // Bounding set defaults to all allowed unless dropped
            ambient: 0,
        }
    }

    /// Check if effective set has specific Linux capability
    pub fn has_cap(&self, cap: LinuxCapability) -> bool {
        let bit = 1u64 << (cap as u64);
        (self.effective & bit) != 0
    }

    /// Raise effective capability (must be in permitted set)
    pub fn raise_effective(&mut self, cap: LinuxCapability) -> Result<(), &'static str> {
        let bit = 1u64 << (cap as u64);
        if (self.permitted & bit) == 0 {
            return Err("Capability not in permitted set");
        }
        self.effective |= bit;
        Ok(())
    }

    /// Lower/drop effective capability
    pub fn drop_effective(&mut self, cap: LinuxCapability) {
        let bit = 1u64 << (cap as u64);
        self.effective &= !bit;
    }

    /// Drop capability from bounding set (irreversible per `prctl(PR_CAP_BOUNDING_DROP)`)
    pub fn drop_bounding(&mut self, cap: LinuxCapability) {
        let bit = 1u64 << (cap as u64);
        self.bounding &= !bit;
        // Also trim permitted, effective, ambient if exceeding bounding set
        self.permitted &= self.bounding;
        self.effective &= self.bounding;
        self.ambient &= self.bounding;
    }

    /// Raise capability in ambient set (must be in both permitted and inheritable sets, and bounding set)
    pub fn raise_ambient(&mut self, cap: LinuxCapability) -> Result<(), &'static str> {
        let bit = 1u64 << (cap as u64);
        if (self.permitted & bit) == 0
            || (self.inheritable & bit) == 0
            || (self.bounding & bit) == 0
        {
            return Err("Ambient capability must be in permitted, inheritable, and bounding sets");
        }
        self.ambient |= bit;
        Ok(())
    }

    /// Clear ambient capability set
    pub fn clear_ambient(&mut self) {
        self.ambient = 0;
    }

    /// Perform execve capability transition (Linux capability transformation rules)
    pub fn execve_transform(&mut self, is_suid_execution: bool) {
        if is_suid_execution {
            self.ambient = 0;
            self.permitted = self.bounding;
            self.effective = self.permitted;
        } else {
            self.permitted = (self.inheritable & self.inheritable) | self.ambient;
            self.permitted &= self.bounding;
            self.effective = self.permitted;
        }
    }
}

/// Capability gate for syscall validation
pub struct CapabilityGate {
    /// Current capability token
    current: AtomicU64,
}

impl CapabilityGate {
    /// Create new capability gate
    pub fn new() -> Self {
        Self {
            current: AtomicU64::new(0),
        }
    }

    /// Set current capability
    pub fn set_capability(&self, token: CapabilityToken) {
        self.current.store(token.bits(), Ordering::SeqCst);
    }

    /// Validate syscall against current capability
    pub fn validate_syscall(&self, permission: Permission) -> bool {
        let current = self.current.load(Ordering::SeqCst);
        (current & (1 << permission as u64)) != 0
    }

    /// Get current capability
    pub fn current_capability(&self) -> CapabilityToken {
        CapabilityToken {
            bits: self.current.load(Ordering::SeqCst),
        }
    }
}

impl Default for CapabilityGate {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_creation() {
        let token = CapabilityToken::new();
        assert_eq!(token.bits(), 0);
    }

    #[test]
    fn test_network_permission() {
        let token = CapabilityToken::new().allow_network("tcp", 80);
        assert!(token.has_permission(Permission::NetworkTcp));
    }

    #[test]
    fn test_file_read_permission() {
        let token = CapabilityToken::new().allow_read("/var/www");
        assert!(token.has_permission(Permission::FileRead));
    }

    #[test]
    fn test_capability_revocation() {
        let mut token = CapabilityToken::new().allow_network("tcp", 80);
        token.revoke_all();
        assert_eq!(token.bits(), 0);
    }

    #[test]
    fn test_capability_gate_validation() {
        let gate = CapabilityGate::new();
        let token = CapabilityToken::new().allow_network("tcp", 80);
        gate.set_capability(token);
        assert!(gate.validate_syscall(Permission::NetworkTcp));
    }

    #[test]
    fn test_bitmask_overlap_prevention() {
        // Registering port 80 and then 443 should not result in a corrupted port 507,
        // but rather only store the latest port 443 cleanly.
        let token = CapabilityToken::new()
            .allow_network("tcp", 80)
            .allow_network("tcp", 443);
        // Extracts port stored in bits 16-31
        let port = (token.bits() >> 16) & 0xFFFF;
        assert_eq!(port, 443);
    }

    #[test]
    fn test_linux_capabilities_parity() {
        let mut caps = LinuxCapabilitySet::full();
        assert!(caps.has_cap(LinuxCapability::SysAdmin));
        assert!(caps.has_cap(LinuxCapability::NetAdmin));

        // Drop CAP_SYS_ADMIN_BIT from bounding set
        caps.drop_bounding(LinuxCapability::SysAdmin);
        assert!(!caps.has_cap(LinuxCapability::SysAdmin));

        // Trying to raise CAP_SYS_ADMIN_BIT effective when not permitted fails
        assert!(caps.raise_effective(LinuxCapability::SysAdmin).is_err());

        // Ambient capability requirements test
        let mut user_caps = LinuxCapabilitySet::empty();
        assert!(user_caps
            .raise_ambient(LinuxCapability::NetBindService)
            .is_err());

        user_caps.permitted |= 1 << (LinuxCapability::NetBindService as u64);
        user_caps.inheritable |= 1 << (LinuxCapability::NetBindService as u64);
        assert!(user_caps
            .raise_ambient(LinuxCapability::NetBindService)
            .is_ok());

        user_caps.clear_ambient();
        assert_eq!(user_caps.ambient, 0);
    }
}
