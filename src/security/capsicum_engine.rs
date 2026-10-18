//! FreeBSD Capsicum Object-Capability Sandboxing Framework for SigmaOS
//!
//! Inspired by FreeBSD's Capsicum (`sys/kern/kern_capsicum.c`):
//! - Fine-grained file descriptor rights bitmask (`cap_rights_t`)
//! - Strict capability mode (`cap_enter`) disabling global namespaces
//! - Irreversible rights restriction (`cap_rights_limit`)
//! - Sandboxed process boundaries with capability delegation

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;

pub const CAP_READ: u64 = 1 << 0;
pub const CAP_WRITE: u64 = 1 << 1;
pub const CAP_SEEK: u64 = 1 << 2;
pub const CAP_MMAP: u64 = 1 << 3;
pub const CAP_FSTAT: u64 = 1 << 4;
pub const CAP_FCNTL: u64 = 1 << 5;
pub const CAP_EVENT: u64 = 1 << 6;
pub const CAP_FSYNC: u64 = 1 << 7;
pub const CAP_SOCK_CLIENT: u64 = 1 << 8;
pub const CAP_SOCK_SERVER: u64 = 1 << 9;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapRights {
    pub mask: u64,
}

impl CapRights {
    pub const fn empty() -> Self {
        Self { mask: 0 }
    }

    pub const fn all() -> Self {
        Self {
            mask: CAP_READ
                | CAP_WRITE
                | CAP_SEEK
                | CAP_MMAP
                | CAP_FSTAT
                | CAP_FCNTL
                | CAP_EVENT
                | CAP_FSYNC
                | CAP_SOCK_CLIENT
                | CAP_SOCK_SERVER,
        }
    }

    pub fn from_mask(mask: u64) -> Self {
        Self { mask }
    }

    pub fn contains(&self, required: u64) -> bool {
        (self.mask & required) == required
    }

    /// Irreversibly restrict rights: new rights must be a subset of old rights
    pub fn limit(&mut self, subset_mask: u64) -> Result<(), &'static str> {
        if (subset_mask & !self.mask) != 0 {
            return Err("Cannot expand capabilities during cap_rights_limit");
        }
        self.mask &= subset_mask;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CapDescriptor {
    pub fd: u32,
    pub target_resource_id: u64,
    pub rights: CapRights,
}

pub struct CapsicumEngine {
    pub capability_mode: bool,
    pub descriptors: BTreeMap<u32, CapDescriptor>,
    pub next_fd: u32,
}

impl CapsicumEngine {
    pub fn new() -> Self {
        Self {
            capability_mode: false,
            descriptors: BTreeMap::new(),
            next_fd: 3, // 0, 1, 2 reserved for stdio
        }
    }

    /// Enter capability mode (`cap_enter(2)`)
    /// Once entered, this state is irreversible for the process and its children
    pub fn cap_enter(&mut self) {
        self.capability_mode = true;
    }

    pub fn in_capability_mode(&self) -> bool {
        self.capability_mode
    }

    /// Register a new resource descriptor with initial rights
    pub fn wrap_resource(&mut self, resource_id: u64, initial_rights: CapRights) -> u32 {
        let fd = self.next_fd;
        self.next_fd += 1;
        let desc = CapDescriptor {
            fd,
            target_resource_id: resource_id,
            rights: initial_rights,
        };
        self.descriptors.insert(fd, desc);
        fd
    }

    /// Restrict the rights on an existing descriptor (`cap_rights_limit(2)`)
    pub fn cap_rights_limit(&mut self, fd: u32, new_rights_mask: u64) -> Result<(), &'static str> {
        let desc = self
            .descriptors
            .get_mut(&fd)
            .ok_or("Invalid file descriptor")?;
        desc.rights.limit(new_rights_mask)
    }

    /// Verify if a descriptor has the required right to perform an operation
    pub fn check_right(&self, fd: u32, required_op: u64) -> Result<u64, &'static str> {
        let desc = self
            .descriptors
            .get(&fd)
            .ok_or("Bad file descriptor (EBADF)")?;
        if desc.rights.contains(required_op) {
            Ok(desc.target_resource_id)
        } else {
            Err("Capsicum capability violation (ENOTCAPABLE)")
        }
    }

    /// Prohibit global path lookups when running in capability mode
    pub fn check_path_access(&self, _path: &str) -> Result<(), &'static str> {
        if self.capability_mode {
            Err("Direct path access disallowed in Capsicum capability mode (ECAPMODE)")
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capsicum_rights_limiting() {
        let mut engine = CapsicumEngine::new();
        let fd = engine.wrap_resource(1001, CapRights::all());

        // Allowed to read and write initially
        assert!(engine.check_right(fd, CAP_READ).is_ok());
        assert!(engine.check_right(fd, CAP_WRITE).is_ok());

        // Limit to read-only
        engine.cap_rights_limit(fd, CAP_READ | CAP_FSTAT).unwrap();

        assert!(engine.check_right(fd, CAP_READ).is_ok());
        // Write should now fail with ENOTCAPABLE
        assert_eq!(
            engine.check_right(fd, CAP_WRITE).unwrap_err(),
            "Capsicum capability violation (ENOTCAPABLE)"
        );

        // Cannot re-expand rights
        assert!(engine.cap_rights_limit(fd, CAP_READ | CAP_WRITE).is_err());
    }

    #[test]
    fn test_capsicum_capability_mode_enforcement() {
        let mut engine = CapsicumEngine::new();
        assert!(engine.check_path_access("/etc/passwd").is_ok());

        engine.cap_enter();
        assert!(engine.in_capability_mode());

        // Direct path lookup must be rejected
        assert_eq!(
            engine.check_path_access("/etc/passwd").unwrap_err(),
            "Direct path access disallowed in Capsicum capability mode (ECAPMODE)"
        );
    }
}
