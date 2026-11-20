// File Range and Advisory Locking Engine
// Part of SigmaOS File Management Subsystem

extern crate alloc;
use alloc::vec::Vec;

use super::fd::FsError;

pub const LOCK_SH: u32 = 1;
pub const LOCK_EX: u32 = 2;
pub const LOCK_NB: u32 = 4;
pub const LOCK_UN: u32 = 8;

pub const F_RDLCK: u16 = 0;
pub const F_WRLCK: u16 = 1;
pub const F_UNLCK: u16 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockType {
    Shared,
    Exclusive,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileRangeLock {
    pub owner_pid: u32,
    pub fd_id: i32,
    pub lock_type: LockType,
    pub start: u64,
    pub len: u64,
}

impl FileRangeLock {
    pub fn new(owner_pid: u32, fd_id: i32, lock_type: LockType, start: u64, len: u64) -> Self {
        Self {
            owner_pid,
            fd_id,
            lock_type,
            start,
            len,
        }
    }

    pub fn overlaps(&self, start: u64, len: u64) -> bool {
        let self_end = if self.len == 0 { u64::MAX } else { self.start.saturating_add(self.len) };
        let other_end = if len == 0 { u64::MAX } else { start.saturating_add(len) };
        self.start < other_end && start < self_end
    }

    pub fn conflicts_with(&self, other_pid: u32, lock_type: LockType, start: u64, len: u64) -> bool {
        if self.owner_pid == other_pid {
            return false;
        }
        if !self.overlaps(start, len) {
            return false;
        }
        match (self.lock_type, lock_type) {
            (LockType::Shared, LockType::Shared) => false,
            _ => true,
        }
    }
}

#[derive(Debug)]
pub struct FileLockManager {
    locks: Vec<FileRangeLock>,
}

impl FileLockManager {
    pub fn new() -> Self {
        Self { locks: Vec::new() }
    }

    pub fn apply_flock(
        &mut self,
        pid: u32,
        fd_id: i32,
        operation: u32,
    ) -> Result<(), FsError> {
        let is_unlock = (operation & LOCK_UN) != 0;

        if is_unlock {
            self.release_locks_for_fd(pid, fd_id);
            return Ok(());
        }

        let lock_type = if (operation & LOCK_EX) != 0 {
            LockType::Exclusive
        } else if (operation & LOCK_SH) != 0 {
            LockType::Shared
        } else {
            return Err(FsError::InvalidOperation);
        };

        let confl = self.locks.iter().any(|l| l.conflicts_with(pid, lock_type, 0, 0));
        if confl {
            return Err(FsError::ResourceBusy);
        }

        self.release_locks_for_fd(pid, fd_id);
        self.locks.push(FileRangeLock::new(pid, fd_id, lock_type, 0, 0));
        Ok(())
    }

    pub fn apply_range_lock(
        &mut self,
        pid: u32,
        fd_id: i32,
        lock_type_flag: u16,
        start: u64,
        len: u64,
        _non_blocking: bool,
    ) -> Result<(), FsError> {
        if lock_type_flag == F_UNLCK {
            self.unlock_range(pid, start, len);
            return Ok(());
        }

        let lock_type = match lock_type_flag {
            F_RDLCK => LockType::Shared,
            F_WRLCK => LockType::Exclusive,
            _ => return Err(FsError::InvalidOperation),
        };

        let confl = self
            .locks
            .iter()
            .any(|l| l.conflicts_with(pid, lock_type, start, len));
        if confl {
            return Err(FsError::ResourceBusy);
        }

        self.locks.push(FileRangeLock::new(pid, fd_id, lock_type, start, len));
        Ok(())
    }

    pub fn unlock_range(&mut self, pid: u32, start: u64, len: u64) {
        self.locks.retain(|l| !(l.owner_pid == pid && l.overlaps(start, len)));
    }

    pub fn release_locks_for_fd(&mut self, pid: u32, fd_id: i32) {
        self.locks.retain(|l| !(l.owner_pid == pid && l.fd_id == fd_id));
    }

    pub fn release_locks_for_process(&mut self, pid: u32) {
        self.locks.retain(|l| l.owner_pid != pid);
    }

    pub fn active_lock_count(&self) -> usize {
        self.locks.len()
    }
}
