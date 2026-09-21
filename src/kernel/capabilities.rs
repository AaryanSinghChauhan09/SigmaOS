// Linux-inspired capability-based security
// POSIX capabilities for SigmaOS

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU32, Ordering};

/// POSIX capability identifiers (Linux capabilities.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    // File capabilities
    Chown,
    DACOverride,
    DACReadSearch,
    Fowner,
    Fsetid,
    Kill,
    Setgid,
    Setuid,
    Setpcap,
    LinuxImmutable,
    NetBindService,
    NetBroadcast,
    NetAdmin,
    NetRaw,
    SysBoot,
    SysChroot,
    SysModule,
    SysTime,
    SysResource,
    Mknod,
    SysAdmin,
}

impl Capability {
    /// Get capability name
    pub fn name(&self) -> &'static str {
        match self {
            Capability::Chown => "CAP_CHOWN",
            Capability::DACOverride => "CAP_DAC_OVERRIDE",
            Capability::DACReadSearch => "CAP_DAC_READ_SEARCH",
            Capability::Fowner => "CAP_FOWNER",
            Capability::Fsetid => "CAP_FSETID",
            Capability::Kill => "CAP_KILL",
            Capability::Setgid => "CAP_SETGID",
            Capability::Setuid => "CAP_SETUID",
            Capability::Setpcap => "CAP_SETPCAP",
            Capability::LinuxImmutable => "CAP_LINUX_IMMUTABLE",
            Capability::NetBindService => "CAP_NET_BIND_SERVICE",
            Capability::NetBroadcast => "CAP_NET_BROADCAST",
            Capability::NetAdmin => "CAP_NET_ADMIN",
            Capability::NetRaw => "CAP_NET_RAW",
            Capability::SysBoot => "CAP_SYS_BOOT",
            Capability::SysChroot => "CAP_SYS_CHROOT",
            Capability::SysModule => "CAP_SYS_MODULE",
            Capability::SysTime => "CAP_SYS_TIME",
            Capability::SysResource => "CAP_SYS_RESOURCE",
            Capability::Mknod => "CAP_MKNOD",
            Capability::SysAdmin => "CAP_SYS_ADMIN",
        }
    }
}

/// Capability set for a process
#[derive(Debug, Clone)]
pub struct CapabilitySet {
    permitted: u64,
    effective: u64,
    inheritable: u64,
}

impl CapabilitySet {
    pub fn new() -> Self {
        CapabilitySet {
            permitted: 0,
            effective: 0,
            inheritable: 0,
        }
    }

    /// Add capability to permitted set
    pub fn add_permitted(&mut self, cap: Capability) {
        let bit = 1u64 << (cap as u64);
        self.permitted |= bit;
    }

    /// Add capability to effective set
    pub fn add_effective(&mut self, cap: Capability) {
        let bit = 1u64 << (cap as u64);
        self.effective |= bit;
    }

    /// Add capability to inheritable set
    pub fn add_inheritable(&mut self, cap: Capability) {
        let bit = 1u64 << (cap as u64);
        self.inheritable |= bit;
    }

    /// Check if capability is in effective set
    pub fn has_effective(&self, cap: Capability) -> bool {
        let bit = 1u64 << (cap as u64);
        (self.effective & bit) != 0
    }

    /// Check if capability is in permitted set
    pub fn has_permitted(&self, cap: Capability) -> bool {
        let bit = 1u64 << (cap as u64);
        (self.permitted & bit) != 0
    }

    /// Check if capability is in inheritable set
    pub fn has_inheritable(&self, cap: Capability) -> bool {
        let bit = 1u64 << (cap as u64);
        (self.inheritable & bit) != 0
    }

    /// Remove capability from effective set
    pub fn remove_effective(&mut self, cap: Capability) {
        let bit = 1u64 << (cap as u64);
        self.effective &= !bit;
    }

    /// Get permitted set
    pub fn permitted(&self) -> u64 {
        self.permitted
    }

    /// Get effective set
    pub fn effective(&self) -> u64 {
        self.effective
    }

    /// Get inheritable set
    pub fn inheritable(&self) -> u64 {
        self.inheritable
    }
}

impl Default for CapabilitySet {
    fn default() -> Self {
        Self::new()
    }
}

/// Capability manager for the system
pub struct CapabilityManager {
    process_capabilities: BTreeMap<u32, Arc<Mutex<CapabilitySet>>>,
    next_pid: AtomicU32,
}

impl CapabilityManager {
    pub fn new() -> Self {
        CapabilityManager {
            process_capabilities: BTreeMap::new(),
            next_pid: AtomicU32::new(1),
        }
    }

    /// Create a new process with default capabilities
    pub fn create_process(&mut self) -> u32 {
        let pid = self.next_pid.fetch_add(1, Ordering::SeqCst);

        let caps = Arc::new(Mutex::new(CapabilitySet::new()));
        self.process_capabilities.insert(pid, caps);

        pid
    }

    /// Get process capabilities
    pub fn get_capabilities(&self, pid: u32) -> Option<Arc<Mutex<CapabilitySet>>> {
        self.process_capabilities.get(&pid).cloned()
    }

    /// Grant capability to process
    pub fn grant(&self, pid: u32, cap: Capability) -> Result<(), String> {
        let caps = self.process_capabilities.get(&pid)
            .ok_or_else(|| format!("Process not found: {}", pid))?;
        
        let mut caps_guard = caps.lock().unwrap();
        caps_guard.add_permitted(cap);
        caps_guard.add_effective(cap);
        Ok(())
    }

    /// Revoke capability from process
    pub fn revoke(&self, pid: u32, cap: Capability) -> Result<(), String> {
        let caps = self.process_capabilities.get(&pid)
            .ok_or_else(|| format!("Process not found: {}", pid))?;
        
        let mut caps_guard = caps.lock().unwrap();
        caps_guard.remove_effective(cap);
        Ok(())
    }

    /// Remove process
    pub fn remove_process(&mut self, pid: u32) -> Result<(), String> {
        self.process_capabilities.remove(&pid)
            .ok_or_else(|| format!("Process not found: {}", pid))?;
        Ok(())
    }

    /// Get process count
    pub fn process_count(&self) -> usize {
        self.process_capabilities.len()
    }
}

impl Default for CapabilityManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_name() {
        assert_eq!(Capability::Chown.name(), "CAP_CHOWN");
        assert_eq!(Capability::SysAdmin.name(), "CAP_SYS_ADMIN");
    }

    #[test]
    fn test_capability_set_creation() {
        let capset = CapabilitySet::new();
        assert_eq!(capset.permitted(), 0);
        assert_eq!(capset.effective(), 0);
        assert_eq!(capset.inheritable(), 0);
    }

    #[test]
    fn test_capability_set_add() {
        let mut capset = CapabilitySet::new();
        capset.add_permitted(Capability::Chown);
        capset.add_effective(Capability::Chown);
        
        assert!(capset.has_permitted(Capability::Chown));
        assert!(capset.has_effective(Capability::Chown));
    }

    #[test]
    fn test_capability_set_remove() {
        let mut capset = CapabilitySet::new();
        capset.add_permitted(Capability::Chown);
        capset.add_effective(Capability::Chown);
        capset.remove_effective(Capability::Chown);
        
        assert!(capset.has_permitted(Capability::Chown));
        assert!(!capset.has_effective(Capability::Chown));
    }

    #[test]
    fn test_capability_manager_creation() {
        let manager = CapabilityManager::new();
        assert_eq!(manager.process_count(), 0);
    }

    #[test]
    fn test_capability_manager_create_process() {
        let mut manager = CapabilityManager::new();
        let pid = manager.create_process();
        
        assert_eq!(pid, 1);
        assert_eq!(manager.process_count(), 1);
    }

    #[test]
    fn test_capability_manager_grant() {
        let mut manager = CapabilityManager::new();
        let pid = manager.create_process();
        
        manager.grant(pid, Capability::Chown).unwrap();
        
        let caps = manager.get_capabilities(pid).unwrap();
        let caps_guard = caps.lock().unwrap();
        assert!(caps_guard.has_effective(Capability::Chown));
    }

    #[test]
    fn test_capability_manager_revoke() {
        let mut manager = CapabilityManager::new();
        let pid = manager.create_process();
        
        manager.grant(pid, Capability::Chown).unwrap();
        manager.revoke(pid, Capability::Chown).unwrap();
        
        let caps = manager.get_capabilities(pid).unwrap();
        let caps_guard = caps.lock().unwrap();
        assert!(!caps_guard.has_effective(Capability::Chown));
    }

    #[test]
    fn test_capability_manager_remove() {
        let mut manager = CapabilityManager::new();
        let pid = manager.create_process();
        
        manager.remove_process(pid).unwrap();
        assert_eq!(manager.process_count(), 0);
    }
}
