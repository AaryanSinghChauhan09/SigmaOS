// Capsicum Security Sandbox (FreeBSD-inspired)
// Provides capability-based security with fine-grained rights

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Capsicum rights (FreeBSD capsicum rights)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CapsicumRights(u64);

impl CapsicumRights {
    pub const CAP_READ: Self = Self(1 << 0);
    pub const CAP_WRITE: Self = Self(1 << 1);
    pub const CAP_SEEK: Self = Self(1 << 2);
    pub const CAP_FCNTL: Self = Self(1 << 3);
    pub const CAP_FSTAT: Self = Self(1 << 4);
    pub const CAP_FSYNC: Self = Self(1 << 5);
    pub const CAP_FCHDIR: Self = Self(1 << 6);
    pub const CAP_FCHMOD: Self = Self(1 << 7);
    pub const CAP_FCHOWN: Self = Self(1 << 8);
    pub const CAP_FUTIMES: Self = Self(1 << 9);
    pub const CAP_FPATHCONF: Self = Self(1 << 10);
    pub const CAP_MMAP: Self = Self(1 << 11);
    pub const CAP_MMAP_RW: Self = Self(1 << 12);
    pub const CAP_CREATE: Self = Self(1 << 13);
    pub const CAP_EXEC: Self = Self(1 << 14);
    pub const CAP_UNLINK: Self = Self(1 << 15);
    pub const CAP_CONNECT: Self = Self(1 << 16);
    pub const CAP_BIND: Self = Self(1 << 17);
    pub const CAP_LISTEN: Self = Self(1 << 18);
    pub const CAP_ACCEPT: Self = Self(1 << 19);
    pub const CAP_GETPEERNAME: Self = Self(1 << 20);
    pub const CAP_GETSOCKNAME: Self = Self(1 << 21);
    pub const CAP_GETSOCKOPT: Self = Self(1 << 22);
    pub const CAP_SETSOCKOPT: Self = Self(1 << 23);
    pub const CAP_RECV: Self = Self(1 << 24);
    pub const CAP_SEND: Self = Self(1 << 25);
    pub const CAP_IOCTL: Self = Self(1 << 26);

    pub fn empty() -> Self {
        Self(0)
    }

    pub fn all() -> Self {
        Self(u64::MAX)
    }

    pub fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub fn intersect(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    pub fn remove(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }
}

/// Capsicum capability descriptor
#[derive(Debug, Clone)]
pub struct CapsicumCapability {
    pub fd: i32,
    pub rights: CapsicumRights,
}

impl CapsicumCapability {
    pub fn new(fd: i32, rights: CapsicumRights) -> Self {
        Self { fd, rights }
    }

    pub fn allows(&self, right: CapsicumRights) -> bool {
        self.rights.contains(right)
    }

    pub fn restrict(&mut self, new_rights: CapsicumRights) {
        self.rights = self.rights.intersect(new_rights);
    }
}

/// Capsicum mode for process sandboxing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapsicumMode {
    /// No restrictions (unsandboxed)
    Unrestricted,
    /// Restricted to capabilities only
    CapabilityMode,
}

/// Capsicum process sandbox
#[derive(Debug, Clone)]
pub struct CapsicumSandbox {
    pub mode: CapsicumMode,
    pub capabilities: HashMap<i32, CapsicumCapability>,
}

impl CapsicumSandbox {
    pub fn new() -> Self {
        Self {
            mode: CapsicumMode::Unrestricted,
            capabilities: HashMap::new(),
        }
    }

    /// Enter capability mode (restrict process)
    pub fn enter_capability_mode(&mut self) {
        self.mode = CapsicumMode::CapabilityMode;
    }

    /// Add a capability with specific rights
    pub fn add_capability(&mut self, cap: CapsicumCapability) {
        self.capabilities.insert(cap.fd, cap);
    }

    /// Get a capability by file descriptor
    pub fn get_capability(&self, fd: i32) -> Option<&CapsicumCapability> {
        self.capabilities.get(&fd)
    }

    /// Check if a file descriptor has specific rights
    pub fn check_rights(&self, fd: i32, rights: CapsicumRights) -> bool {
        match self.mode {
            CapsicumMode::Unrestricted => true,
            CapsicumMode::CapabilityMode => {
                match self.get_capability(fd) {
                    Some(cap) => cap.allows(rights),
                    None => false, // No capability = deny
                }
            }
        }
    }

    /// Restrict an existing capability's rights
    pub fn restrict_capability(&mut self, fd: i32, new_rights: CapsicumRights) -> Result<(), String> {
        match self.capabilities.get_mut(&fd) {
            Some(cap) => {
                cap.restrict(new_rights);
                Ok(())
            }
            None => Err(format!("Capability for fd {} not found", fd)),
        }
    }

    /// Remove a capability
    pub fn remove_capability(&mut self, fd: i32) -> Result<(), String> {
        match self.capabilities.remove(&fd) {
            Some(_) => Ok(()),
            None => Err(format!("Capability for fd {} not found", fd)),
        }
    }

    /// Get number of capabilities
    pub fn capability_count(&self) -> usize {
        self.capabilities.len()
    }

    /// Check if in capability mode
    pub fn is_capability_mode(&self) -> bool {
        self.mode == CapsicumMode::CapabilityMode
    }
}

impl Default for CapsicumSandbox {
    fn default() -> Self {
        Self::new()
    }
}

/// Capsicum manager for system-wide sandbox management
pub struct CapsicumManager {
    sandboxes: Arc<Mutex<HashMap<u64, CapsicumSandbox>>>,
    next_sandbox_id: Arc<Mutex<u64>>,
}

impl CapsicumManager {
    pub fn new() -> Self {
        Self {
            sandboxes: Arc::new(Mutex::new(HashMap::new())),
            next_sandbox_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new sandbox
    pub fn create_sandbox(&self) -> u64 {
        let mut next_id = self.next_sandbox_id.lock().unwrap();
        let sandbox_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let sandbox = CapsicumSandbox::new();
        let mut sandboxes = self.sandboxes.lock().unwrap();
        sandboxes.insert(sandbox_id, sandbox);

        sandbox_id
    }

    /// Get a sandbox by ID
    pub fn get_sandbox(&self, sandbox_id: u64) -> Option<CapsicumSandbox> {
        let sandboxes = self.sandboxes.lock().unwrap();
        sandboxes.get(&sandbox_id).cloned()
    }

    /// Remove a sandbox
    pub fn remove_sandbox(&self, sandbox_id: u64) -> Result<(), String> {
        let mut sandboxes = self.sandboxes.lock().unwrap();
        match sandboxes.remove(&sandbox_id) {
            Some(_) => Ok(()),
            None => Err(format!("Sandbox {} not found", sandbox_id)),
        }
    }

    /// Enter capability mode for a sandbox
    pub fn enter_capability_mode(&self, sandbox_id: u64) -> Result<(), String> {
        let mut sandboxes = self.sandboxes.lock().unwrap();
        match sandboxes.get_mut(&sandbox_id) {
            Some(sandbox) => {
                sandbox.enter_capability_mode();
                Ok(())
            }
            None => Err(format!("Sandbox {} not found", sandbox_id)),
        }
    }

    /// Add capability to a sandbox
    pub fn add_capability(&self, sandbox_id: u64, cap: CapsicumCapability) -> Result<(), String> {
        let mut sandboxes = self.sandboxes.lock().unwrap();
        match sandboxes.get_mut(&sandbox_id) {
            Some(sandbox) => {
                sandbox.add_capability(cap);
                Ok(())
            }
            None => Err(format!("Sandbox {} not found", sandbox_id)),
        }
    }

    /// Check rights for a sandbox
    pub fn check_rights(&self, sandbox_id: u64, fd: i32, rights: CapsicumRights) -> bool {
        match self.get_sandbox(sandbox_id) {
            Some(sandbox) => sandbox.check_rights(fd, rights),
            None => false,
        }
    }

    /// Get number of active sandboxes
    pub fn sandbox_count(&self) -> usize {
        let sandboxes = self.sandboxes.lock().unwrap();
        sandboxes.len()
    }
}

impl Default for CapsicumManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capsicum_rights_flags() {
        let read = CapsicumRights::CAP_READ;
        let write = CapsicumRights::CAP_WRITE;
        let both = read.union(write);

        assert!(both.contains(read));
        assert!(both.contains(write));
        assert!(!read.contains(write));
    }

    #[test]
    fn test_capsicum_rights_all() {
        let all = CapsicumRights::all();
        assert!(all.contains(CapsicumRights::CAP_READ));
        assert!(all.contains(CapsicumRights::CAP_WRITE));
        assert!(all.contains(CapsicumRights::CAP_SEEK));
    }

    #[test]
    fn test_capsicum_rights_remove() {
        let all = CapsicumRights::all();
        let without_write = all.remove(CapsicumRights::CAP_WRITE);

        assert!(without_write.contains(CapsicumRights::CAP_READ));
        assert!(!without_write.contains(CapsicumRights::CAP_WRITE));
    }

    #[test]
    fn test_capsicum_capability() {
        let cap = CapsicumCapability::new(3, CapsicumRights::CAP_READ);
        assert_eq!(cap.fd, 3);
        assert!(cap.allows(CapsicumRights::CAP_READ));
        assert!(!cap.allows(CapsicumRights::CAP_WRITE));
    }

    #[test]
    fn test_capsicum_capability_restrict() {
        let mut cap = CapsicumCapability::new(3, CapsicumRights::CAP_READ.union(CapsicumRights::CAP_WRITE));
        cap.restrict(CapsicumRights::CAP_READ);

        assert!(cap.allows(CapsicumRights::CAP_READ));
        assert!(!cap.allows(CapsicumRights::CAP_WRITE));
    }

    #[test]
    fn test_capsicum_sandbox_unrestricted() {
        let sandbox = CapsicumSandbox::new();
        assert_eq!(sandbox.mode, CapsicumMode::Unrestricted);
        assert!(sandbox.check_rights(3, CapsicumRights::CAP_READ));
    }

    #[test]
    fn test_capsicum_sandbox_capability_mode() {
        let mut sandbox = CapsicumSandbox::new();
        sandbox.enter_capability_mode();

        assert_eq!(sandbox.mode, CapsicumMode::CapabilityMode);
        assert!(!sandbox.check_rights(3, CapsicumRights::CAP_READ)); // No capability

        sandbox.add_capability(CapsicumCapability::new(3, CapsicumRights::CAP_READ));
        assert!(sandbox.check_rights(3, CapsicumRights::CAP_READ));
    }

    #[test]
    fn test_capsicum_sandbox_restrict_capability() {
        let mut sandbox = CapsicumSandbox::new();
        sandbox.enter_capability_mode();

        sandbox.add_capability(CapsicumCapability::new(3, CapsicumRights::CAP_READ.union(CapsicumRights::CAP_WRITE)));
        sandbox.restrict_capability(3, CapsicumRights::CAP_READ).unwrap();

        assert!(sandbox.check_rights(3, CapsicumRights::CAP_READ));
        assert!(!sandbox.check_rights(3, CapsicumRights::CAP_WRITE));
    }

    #[test]
    fn test_capsicum_manager() {
        let manager = CapsicumManager::new();

        let sandbox_id = manager.create_sandbox();
        assert_eq!(sandbox_id, 1);

        manager.enter_capability_mode(sandbox_id).unwrap();
        manager.add_capability(sandbox_id, CapsicumCapability::new(3, CapsicumRights::CAP_READ)).unwrap();

        assert!(manager.check_rights(sandbox_id, 3, CapsicumRights::CAP_READ));
        assert!(!manager.check_rights(sandbox_id, 3, CapsicumRights::CAP_WRITE));

        assert_eq!(manager.sandbox_count(), 1);

        manager.remove_sandbox(sandbox_id).unwrap();
        assert_eq!(manager.sandbox_count(), 0);
    }

    #[test]
    fn test_capsicum_manager_multiple_sandboxes() {
        let manager = CapsicumManager::new();

        let sandbox_id1 = manager.create_sandbox();
        let sandbox_id2 = manager.create_sandbox();

        manager.enter_capability_mode(sandbox_id1).unwrap();
        manager.add_capability(sandbox_id1, CapsicumCapability::new(3, CapsicumRights::CAP_READ)).unwrap();

        manager.enter_capability_mode(sandbox_id2).unwrap();
        manager.add_capability(sandbox_id2, CapsicumCapability::new(4, CapsicumRights::CAP_WRITE)).unwrap();

        assert!(manager.check_rights(sandbox_id1, 3, CapsicumRights::CAP_READ));
        assert!(!manager.check_rights(sandbox_id2, 3, CapsicumRights::CAP_READ));

        assert!(manager.check_rights(sandbox_id2, 4, CapsicumRights::CAP_WRITE));
        assert!(!manager.check_rights(sandbox_id1, 4, CapsicumRights::CAP_WRITE));

        assert_eq!(manager.sandbox_count(), 2);
    }

    #[test]
    fn test_capsicum_rights_intersect() {
        let all = CapsicumRights::CAP_READ.union(CapsicumRights::CAP_WRITE);
        let read_only = CapsicumRights::CAP_READ;
        let intersection = all.intersect(read_only);

        assert_eq!(intersection, read_only);
    }
}
