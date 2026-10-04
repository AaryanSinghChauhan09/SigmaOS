//! Capsicum Capability Rights Bitmask
//!
//! Implements FreeBSD-style per-FD capability rights as a 64-bit bitmask.
//! Each bit represents a specific right (read, write, execute, etc.).
//!
//! # References
//! - FreeBSD sys/sys/capsicum.h
//! - FreeBSD sys/kern/kern_capmode.c

#![cfg_attr(not(any(feature = "standalone_test", test)), no_std)]

extern crate alloc;

use core::fmt;

/// Capsicum rights version for ABI stability
pub const CAPSICUM_RIGHTS_VERSION: u32 = 0;

/// Individual capability right bits (u64 constants)
pub const CAP_READ: u64 = 0x0000000000000001;
pub const CAP_WRITE: u64 = 0x0000000000000002;
pub const CAP_SEEK: u64 = 0x0000000000000004;
pub const CAP_PREAD: u64 = 0x0000000000000008;
pub const CAP_PWRITE: u64 = 0x0000000000000010;
pub const CAP_MMAP: u64 = 0x0000000000000020;
pub const CAP_MMAP_R: u64 = 0x0000000000000040;
pub const CAP_MMAP_W: u64 = 0x0000000000000080;
pub const CAP_MMAP_X: u64 = 0x0000000000000100;
pub const CAP_MMAP_RW: u64 = CAP_MMAP_R | CAP_MMAP_W;
pub const CAP_MMAP_RX: u64 = CAP_MMAP_R | CAP_MMAP_X;
pub const CAP_MMAP_WX: u64 = CAP_MMAP_W | CAP_MMAP_X;
pub const CAP_MMAP_RWX: u64 = CAP_MMAP_R | CAP_MMAP_W | CAP_MMAP_X;
pub const CAP_CREATE: u64 = 0x0000000000000200;
pub const CAP_FEXECVE: u64 = 0x0000000000000400;
pub const CAP_FSYNC: u64 = 0x0000000000000800;
pub const CAP_FTRUNCATE: u64 = 0x0000000000001000;
pub const CAP_FSTAT: u64 = 0x0000000000002000;
pub const CAP_FSTATAT: u64 = 0x0000000000004000;
pub const CAP_FSTATFS: u64 = 0x0000000000008000;
pub const CAP_FUTIMES: u64 = 0x0000000000010000;
pub const CAP_LINKAT_SOURCE: u64 = 0x0000000000020000;
pub const CAP_LINKAT_TARGET: u64 = 0x0000000000040000;
pub const CAP_MKDIRAT: u64 = 0x0000000000080000;
pub const CAP_MKFIFOAT: u64 = 0x0000000000100000;
pub const CAP_MKNODAT: u64 = 0x0000000000200000;
pub const CAP_RENAMEAT_SOURCE: u64 = 0x0000000000400000;
pub const CAP_RENAMEAT_TARGET: u64 = 0x0000000000800000;
pub const CAP_SYMLINKAT: u64 = 0x0000000001000000;
pub const CAP_UNLINKAT: u64 = 0x0000000002000000;
pub const CAP_ACCEPT: u64 = 0x0000000004000000;
pub const CAP_BIND: u64 = 0x0000000008000000;
pub const CAP_CONNECT: u64 = 0x0000000010000000;
pub const CAP_GETPEERNAME: u64 = 0x0000000020000000;
pub const CAP_GETSOCKNAME: u64 = 0x0000000040000000;
pub const CAP_GETSOCKOPT: u64 = 0x0000000080000000;
pub const CAP_LISTEN: u64 = 0x0000000100000000;
pub const CAP_SEND: u64 = 0x0000000200000000;
pub const CAP_RECV: u64 = 0x0000000400000000;
pub const CAP_SETSOCKOPT: u64 = 0x0000000800000000;
pub const CAP_SHUTDOWN: u64 = 0x0000001000000000;
pub const CAP_IOCTL: u64 = 0x0000002000000000;
pub const CAP_FCNTL: u64 = 0x0000004000000000;
pub const CAP_EVENT: u64 = 0x0000008000000000;
pub const CAP_KQUEUE_EVENT: u64 = 0x0000010000000000;
pub const CAP_PDGETPID: u64 = 0x0000020000000000;
pub const CAP_PDKILL: u64 = 0x0000040000000000;
pub const CAP_PDWAIT: u64 = 0x0000080000000000;
pub const CAP_FCHDIR: u64 = 0x0000100000000000;
pub const CAP_FCHFLAGS: u64 = 0x0000200000000000;
pub const CAP_FCHMOD: u64 = 0x0000400000000000;
pub const CAP_FCHOWN: u64 = 0x0000800000000000;
pub const CAP_FLOCK: u64 = 0x0001000000000000;
pub const CAP_DELETE: u64 = 0x0002000000000000;
pub const CAP_LOOKUP: u64 = 0x0004000000000000;
pub const CAP_MKDIR: u64 = 0x0008000000000000;
pub const CAP_RMDIR: u64 = 0x0010000000000000;
pub const CAP_UNLINK: u64 = 0x0020000000000000;

/// All rights mask
pub const CAP_ALL: u64 = 0xFFFFFFFFFFFFFFFF;

/// Predefined capability sets
pub const CAP_READ_RIGHTS: u64 = CAP_READ | CAP_SEEK | CAP_FSTAT | CAP_PREAD;
pub const CAP_WRITE_RIGHTS: u64 = CAP_WRITE | CAP_FTRUNCATE | CAP_FSYNC | CAP_PWRITE;
pub const CAP_NETWORK_RIGHTS: u64 =
    CAP_CONNECT | CAP_SEND | CAP_RECV | CAP_ACCEPT | CAP_BIND | CAP_LISTEN;

/// Capability rights bitmask wrapping a 64-bit integer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapRightsMask(pub u64);

impl CapRightsMask {
    /// Create a new rights mask with all rights
    pub fn all() -> Self {
        Self(CAP_ALL)
    }

    /// Create a new rights mask with no rights
    pub fn none() -> Self {
        Self(0)
    }

    /// Create a new rights mask from a u64 bitmask
    pub fn new(rights: u64) -> Self {
        Self(rights)
    }

    /// Add a right to the mask
    pub fn add(&mut self, right: u64) {
        self.0 |= right;
    }

    /// Remove a right from the mask
    pub fn remove(&mut self, right: u64) {
        self.0 &= !right;
    }

    /// Check if a specific right is present
    pub fn has(&self, right: u64) -> bool {
        (self.0 & right) == right
    }

    /// Compute the intersection of two rights masks (bitwise AND)
    pub fn intersect(&self, other: &Self) -> Self {
        Self(self.0 & other.0)
    }

    /// Compute the union of two rights masks (bitwise OR)
    pub fn union(&self, other: &Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Check if this mask is a subset of another mask
    /// (all rights in self are also in other)
    pub fn is_subset_of(&self, other: &Self) -> bool {
        (self.0 & !other.0) == 0
    }

    /// Restrict this mask to only rights present in the given mask
    pub fn limit(&mut self, rights: u64) {
        self.0 &= rights;
    }

    /// Get the raw u64 value
    pub fn raw(&self) -> u64 {
        self.0
    }
}

impl Default for CapRightsMask {
    fn default() -> Self {
        Self::none()
    }
}

impl fmt::Display for CapRightsMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CapRightsMask(0x{:016X})", self.0)
    }
}

impl From<u64> for CapRightsMask {
    fn from(val: u64) -> Self {
        Self(val)
    }
}

impl From<CapRightsMask> for u64 {
    fn from(val: CapRightsMask) -> Self {
        val.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rights_all_none() {
        let all = CapRightsMask::all();
        let none = CapRightsMask::none();
        assert_eq!(all.0, CAP_ALL);
        assert_eq!(none.0, 0);
    }

    #[test]
    fn test_rights_add_remove() {
        let mut mask = CapRightsMask::none();
        mask.add(CAP_READ);
        assert!(mask.has(CAP_READ));
        assert!(!mask.has(CAP_WRITE));

        mask.add(CAP_WRITE);
        assert!(mask.has(CAP_READ));
        assert!(mask.has(CAP_WRITE));

        mask.remove(CAP_READ);
        assert!(!mask.has(CAP_READ));
        assert!(mask.has(CAP_WRITE));
    }

    #[test]
    fn test_rights_has() {
        let mask = CapRightsMask::new(CAP_READ | CAP_WRITE | CAP_SEEK);
        assert!(mask.has(CAP_READ));
        assert!(mask.has(CAP_WRITE));
        assert!(mask.has(CAP_SEEK));
        assert!(!mask.has(CAP_FSTAT));
        assert!(mask.has(CAP_READ | CAP_WRITE));
    }

    #[test]
    fn test_rights_intersect() {
        let mask1 = CapRightsMask::new(CAP_READ | CAP_WRITE | CAP_SEEK);
        let mask2 = CapRightsMask::new(CAP_WRITE | CAP_SEEK | CAP_FSTAT);
        let intersection = mask1.intersect(&mask2);
        assert_eq!(intersection.0, CAP_WRITE | CAP_SEEK);
    }

    #[test]
    fn test_rights_union() {
        let mask1 = CapRightsMask::new(CAP_READ | CAP_WRITE);
        let mask2 = CapRightsMask::new(CAP_SEEK | CAP_FSTAT);
        let union = mask1.union(&mask2);
        assert_eq!(union.0, CAP_READ | CAP_WRITE | CAP_SEEK | CAP_FSTAT);
    }

    #[test]
    fn test_rights_is_subset_of() {
        let subset = CapRightsMask::new(CAP_READ | CAP_WRITE);
        let superset = CapRightsMask::new(CAP_READ | CAP_WRITE | CAP_SEEK | CAP_FSTAT);
        assert!(subset.is_subset_of(&superset));
        assert!(!superset.is_subset_of(&subset));

        let equal1 = CapRightsMask::new(CAP_READ | CAP_WRITE);
        let equal2 = CapRightsMask::new(CAP_READ | CAP_WRITE);
        assert!(equal1.is_subset_of(&equal2));
        assert!(equal2.is_subset_of(&equal1));
    }

    #[test]
    fn test_rights_limit() {
        let mut mask = CapRightsMask::new(CAP_READ | CAP_WRITE | CAP_SEEK | CAP_FSTAT);
        mask.limit(CAP_READ | CAP_WRITE);
        assert_eq!(mask.0, CAP_READ | CAP_WRITE);
    }

    #[test]
    fn test_predefined_masks() {
        let read_rights = CapRightsMask::new(CAP_READ_RIGHTS);
        assert!(read_rights.has(CAP_READ));
        assert!(read_rights.has(CAP_SEEK));
        assert!(read_rights.has(CAP_FSTAT));
        assert!(read_rights.has(CAP_PREAD));

        let write_rights = CapRightsMask::new(CAP_WRITE_RIGHTS);
        assert!(write_rights.has(CAP_WRITE));
        assert!(write_rights.has(CAP_FTRUNCATE));
        assert!(write_rights.has(CAP_FSYNC));
        assert!(write_rights.has(CAP_PWRITE));

        let net_rights = CapRightsMask::new(CAP_NETWORK_RIGHTS);
        assert!(net_rights.has(CAP_CONNECT));
        assert!(net_rights.has(CAP_SEND));
        assert!(net_rights.has(CAP_RECV));
        assert!(net_rights.has(CAP_ACCEPT));
        assert!(net_rights.has(CAP_BIND));
        assert!(net_rights.has(CAP_LISTEN));
    }

    #[test]
    fn test_rights_inheritance_narrowing() {
        // Simulate FD rights inheritance - can only narrow, never expand
        let parent_rights = CapRightsMask::new(CAP_READ | CAP_WRITE | CAP_SEEK);
        let child_rights = CapRightsMask::new(CAP_READ | CAP_SEEK); // narrower

        assert!(child_rights.is_subset_of(&parent_rights));

        // Attempting to expand should fail in real code
        let invalid_child = CapRightsMask::new(CAP_READ | CAP_WRITE | CAP_SEEK | CAP_FSTAT);
        assert!(!invalid_child.is_subset_of(&parent_rights));
    }

    #[test]
    fn test_mmap_combined_rights() {
        let mmap_rw = CapRightsMask::new(CAP_MMAP_RW);
        assert!(mmap_rw.has(CAP_MMAP_R));
        assert!(mmap_rw.has(CAP_MMAP_W));
        assert!(!mmap_rw.has(CAP_MMAP_X));

        let mmap_rwx = CapRightsMask::new(CAP_MMAP_RWX);
        assert!(mmap_rwx.has(CAP_MMAP_R));
        assert!(mmap_rwx.has(CAP_MMAP_W));
        assert!(mmap_rwx.has(CAP_MMAP_X));
    }
}
