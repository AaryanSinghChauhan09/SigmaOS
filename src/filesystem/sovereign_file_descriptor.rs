#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SigmaOS Sovereign File Descriptor Engine
// Linux & BSD inspired file descriptor table management with FreeBSD Capsicum capabilities
// - Standard file descriptors (0 stdin, 1 stdout, 2 stderr)
// - dup(), dup2(), dup3() with O_CLOEXEC support
// - fcntl() descriptor and status flag management (F_GETFD, F_SETFD, F_GETFL, F_SETFL)
// - FreeBSD Capsicum fine-grained per-descriptor capability delegation (CAP_READ, CAP_WRITE, etc.)
// - Close-on-exec (O_CLOEXEC) descriptor sweeping upon process execve

use std::collections::BTreeMap;
use std::vec::Vec;

pub mod flags {
    pub const O_RDONLY: u32   = 0x0000;
    pub const O_WRONLY: u32   = 0x0001;
    pub const O_RDWR: u32     = 0x0002;
    pub const O_APPEND: u32   = 0x0400;
    pub const O_NONBLOCK: u32 = 0x0800;
    pub const O_SYNC: u32     = 0x1000;
    pub const O_CLOEXEC: u32  = 0x80000;
}

pub mod cap_rights {
    pub const CAP_READ: u64    = 1 << 0;
    pub const CAP_WRITE: u64   = 1 << 1;
    pub const CAP_SEEK: u64    = 1 << 2;
    pub const CAP_FCNTL: u64   = 1 << 3;
    pub const CAP_MMAP: u64    = 1 << 4;
    pub const CAP_ACCEPT: u64  = 1 << 5;
    pub const CAP_BIND: u64    = 1 << 6;
    pub const CAP_CONNECT: u64 = 1 << 7;
    pub const CAP_ALL: u64     = 0xFFFFFFFFFFFFFFFF;
}

pub mod fcntl_cmd {
    pub const F_DUPFD: i32         = 0;
    pub const F_GETFD: i32         = 1;
    pub const F_SETFD: i32         = 2;
    pub const F_GETFL: i32         = 3;
    pub const F_SETFL: i32         = 4;
    pub const F_DUPFD_CLOEXEC: i32 = 1030;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SovereignFileDescriptor {
    pub inode_id: u64,
    pub offset: u64,
    pub flags: u32,
    pub cap_rights: u64,
}

impl SovereignFileDescriptor {
    pub fn new(inode_id: u64, flags: u32) -> Self {
        Self {
            inode_id,
            offset: 0,
            flags,
            cap_rights: cap_rights::CAP_ALL,
        }
    }

    pub fn has_capability(&self, right: u64) -> bool {
        (self.cap_rights & right) == right
    }

    pub fn is_cloexec(&self) -> bool {
        (self.flags & flags::O_CLOEXEC) != 0
    }
}

#[derive(Debug, Clone)]
pub struct SovereignFdTable {
    pub max_fds: usize,
    pub descriptors: BTreeMap<i32, SovereignFileDescriptor>,
}

impl SovereignFdTable {
    pub fn new() -> Self {
        let mut table = BTreeMap::new();
        // Initialize standard descriptors stdin (0), stdout (1), stderr (2)
        table.insert(0, SovereignFileDescriptor::new(1, flags::O_RDONLY));
        table.insert(1, SovereignFileDescriptor::new(2, flags::O_WRONLY));
        table.insert(2, SovereignFileDescriptor::new(3, flags::O_WRONLY));

        Self {
            max_fds: 1024,
            descriptors: table,
        }
    }

    pub fn allocate_lowest_fd(&self) -> Result<i32, &'static str> {
        for fd in 0..(self.max_fds as i32) {
            if !self.descriptors.contains_key(&fd) {
                return Ok(fd);
            }
        }
        Err("Too many open file descriptors")
    }

    pub fn open_descriptor(&mut self, inode_id: u64, flags: u32) -> Result<i32, &'static str> {
        let fd = self.allocate_lowest_fd()?;
        let descriptor = SovereignFileDescriptor::new(inode_id, flags);
        self.descriptors.insert(fd, descriptor);
        Ok(fd)
    }

    pub fn close_descriptor(&mut self, fd: i32) -> bool {
        self.descriptors.remove(&fd).is_some()
    }

    pub fn get_descriptor(&self, fd: i32) -> Option<&SovereignFileDescriptor> {
        self.descriptors.get(&fd)
    }

    pub fn get_descriptor_mut(&mut self, fd: i32) -> Option<&mut SovereignFileDescriptor> {
        self.descriptors.get_mut(&fd)
    }

    pub fn dup(&mut self, old_fd: i32) -> Result<i32, &'static str> {
        let old_desc = self.descriptors.get(&old_fd).ok_or("Bad file descriptor")?.clone();
        let new_fd = self.allocate_lowest_fd()?;
        // dup clears O_CLOEXEC on duplicated descriptor
        let mut new_desc = old_desc;
        new_desc.flags &= !flags::O_CLOEXEC;
        self.descriptors.insert(new_fd, new_desc);
        Ok(new_fd)
    }

    pub fn dup2(&mut self, old_fd: i32, new_fd: i32) -> Result<i32, &'static str> {
        if old_fd == new_fd {
            if self.descriptors.contains_key(&old_fd) {
                return Ok(new_fd);
            } else {
                return Err("Bad file descriptor");
            }
        }

        let old_desc = self.descriptors.get(&old_fd).ok_or("Bad file descriptor")?.clone();
        self.close_descriptor(new_fd);

        let mut new_desc = old_desc;
        new_desc.flags &= !flags::O_CLOEXEC;
        self.descriptors.insert(new_fd, new_desc);
        Ok(new_fd)
    }

    pub fn dup3(&mut self, old_fd: i32, new_fd: i32, flags_arg: u32) -> Result<i32, &'static str> {
        if old_fd == new_fd {
            return Err("dup3 oldfd and newfd cannot be equal");
        }

        let old_desc = self.descriptors.get(&old_fd).ok_or("Bad file descriptor")?.clone();
        self.close_descriptor(new_fd);

        let mut new_desc = old_desc;
        if (flags_arg & flags::O_CLOEXEC) != 0 {
            new_desc.flags |= flags::O_CLOEXEC;
        } else {
            new_desc.flags &= !flags::O_CLOEXEC;
        }
        self.descriptors.insert(new_fd, new_desc);
        Ok(new_fd)
    }

    pub fn fcntl(&mut self, fd: i32, cmd: i32, arg: u32) -> Result<i32, &'static str> {
        let desc = self.descriptors.get_mut(&fd).ok_or("Bad file descriptor")?;
        match cmd {
            fcntl_cmd::F_GETFD => {
                if desc.is_cloexec() {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
            fcntl_cmd::F_SETFD => {
                if (arg & 1) != 0 {
                    desc.flags |= flags::O_CLOEXEC;
                } else {
                    desc.flags &= !flags::O_CLOEXEC;
                }
                Ok(0)
            }
            fcntl_cmd::F_GETFL => Ok(desc.flags as i32),
            fcntl_cmd::F_SETFL => {
                // F_SETFL only permits modifying O_APPEND, O_NONBLOCK, O_SYNC
                let allowed_mask = flags::O_APPEND | flags::O_NONBLOCK | flags::O_SYNC;
                desc.flags = (desc.flags & !allowed_mask) | (arg & allowed_mask);
                Ok(0)
            }
            _ => Err("Invalid fcntl command"),
        }
    }

    /// FreeBSD Capsicum capability rights limitation
    pub fn cap_rights_limit(&mut self, fd: i32, rights: u64) -> Result<(), &'static str> {
        let desc = self.descriptors.get_mut(&fd).ok_or("Bad file descriptor")?;
        if (desc.cap_rights & rights) != rights {
            return Err("Cannot expand capability rights during cap_rights_limit");
        }
        desc.cap_rights &= rights;
        Ok(())
    }

    /// Execve close-on-exec descriptor sweep
    pub fn exec_cloexec_sweep(&mut self) -> usize {
        let to_remove: Vec<i32> = self.descriptors
            .iter()
            .filter(|(_, desc)| desc.is_cloexec())
            .map(|(&fd, _)| fd)
            .collect();

        let count = to_remove.len();
        for fd in to_remove {
            self.descriptors.remove(&fd);
        }
        count
    }
}

impl Default for SovereignFdTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_fd_table_dup_dup2_dup3() {
        let mut table = SovereignFdTable::new();
        let fd = table.open_descriptor(100, flags::O_RDWR | flags::O_CLOEXEC).unwrap();
        assert_eq!(fd, 3);

        let dup_fd = table.dup(fd).unwrap();
        assert_eq!(dup_fd, 4);
        assert!(!table.get_descriptor(dup_fd).unwrap().is_cloexec());

        let dup2_fd = table.dup2(fd, 10).unwrap();
        assert_eq!(dup2_fd, 10);
        assert!(!table.get_descriptor(dup2_fd).unwrap().is_cloexec());

        let dup3_fd = table.dup3(fd, 11, flags::O_CLOEXEC).unwrap();
        assert_eq!(dup3_fd, 11);
        assert!(table.get_descriptor(dup3_fd).unwrap().is_cloexec());
    }

    #[test]
    fn test_fcntl_flags_get_set() {
        let mut table = SovereignFdTable::new();
        let fd = table.open_descriptor(200, flags::O_RDWR).unwrap();

        assert_eq!(table.fcntl(fd, fcntl_cmd::F_GETFD, 0).unwrap(), 0);
        table.fcntl(fd, fcntl_cmd::F_SETFD, 1).unwrap();
        assert_eq!(table.fcntl(fd, fcntl_cmd::F_GETFD, 0).unwrap(), 1);

        table.fcntl(fd, fcntl_cmd::F_SETFL, flags::O_NONBLOCK).unwrap();
        assert_ne!(table.get_descriptor(fd).unwrap().flags & flags::O_NONBLOCK, 0);
    }

    #[test]
    fn test_cloexec_sweep() {
        let mut table = SovereignFdTable::new();
        table.open_descriptor(300, flags::O_RDONLY).unwrap(); // FD 3
        table.open_descriptor(301, flags::O_RDWR | flags::O_CLOEXEC).unwrap(); // FD 4

        let removed = table.exec_cloexec_sweep();
        assert_eq!(removed, 1);
        assert!(table.get_descriptor(3).is_some());
        assert!(table.get_descriptor(4).is_none());
    }

    #[test]
    fn test_capsicum_rights_limiting() {
        let mut table = SovereignFdTable::new();
        let fd = table.open_descriptor(400, flags::O_RDWR).unwrap();

        assert!(table.get_descriptor(fd).unwrap().has_capability(cap_rights::CAP_READ));
        assert!(table.get_descriptor(fd).unwrap().has_capability(cap_rights::CAP_WRITE));

        table.cap_rights_limit(fd, cap_rights::CAP_READ | cap_rights::CAP_SEEK).unwrap();
        assert!(table.get_descriptor(fd).unwrap().has_capability(cap_rights::CAP_READ));
        assert!(!table.get_descriptor(fd).unwrap().has_capability(cap_rights::CAP_WRITE));

        // Expanding rights fails
        assert!(table.cap_rights_limit(fd, cap_rights::CAP_ALL).is_err());
    }
}
