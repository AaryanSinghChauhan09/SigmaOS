// Permission Checking and Security Gate Engine
// Part of SigmaOS File Management Subsystem

extern crate alloc;
use alloc::vec::Vec;

use super::fd::FsError;

pub const S_IRUSR: u32 = 0o400;
pub const S_IWUSR: u32 = 0o200;
pub const S_IXUSR: u32 = 0o100;

pub const S_IRGRP: u32 = 0o040;
pub const S_IWGRP: u32 = 0o020;
pub const S_IXGRP: u32 = 0o010;

pub const S_IROTH: u32 = 0o004;
pub const S_IWOTH: u32 = 0o002;
pub const S_IXOTH: u32 = 0o001;

pub const CAP_DAC_OVERRIDE: u64 = 1 << 1;
pub const CAP_DAC_READ_SEARCH: u64 = 1 << 2;
pub const CAP_FOWNER: u64 = 1 << 3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessCredentials {
    pub uid: u32,
    pub gid: u32,
    pub euid: u32,
    pub egid: u32,
    pub supplementary_gids: Vec<u32>,
    pub capabilities: u64,
}

impl ProcessCredentials {
    pub fn root() -> Self {
        Self {
            uid: 0,
            gid: 0,
            euid: 0,
            egid: 0,
            supplementary_gids: Vec::new(),
            capabilities: !0,
        }
    }

    pub fn unprivileged(uid: u32, gid: u32) -> Self {
        Self {
            uid,
            gid,
            euid: uid,
            egid: gid,
            supplementary_gids: Vec::new(),
            capabilities: 0,
        }
    }

    pub fn has_capability(&self, cap: u64) -> bool {
        (self.capabilities & cap) != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessMode {
    Read,
    Write,
    Execute,
    ReadWrite,
}

#[derive(Debug)]
pub struct PermissionEngine;

impl PermissionEngine {
    pub fn check_access(
        creds: &ProcessCredentials,
        file_uid: u32,
        file_gid: u32,
        file_mode: u32,
        access: AccessMode,
    ) -> Result<(), FsError> {
        if creds.euid == 0 || creds.has_capability(CAP_DAC_OVERRIDE) {
            return Ok(());
        }

        let is_owner = creds.euid == file_uid;
        let is_group = creds.egid == file_gid || creds.supplementary_gids.contains(&file_gid);

        let (r_bit, w_bit, x_bit) = if is_owner {
            (S_IRUSR, S_IWUSR, S_IXUSR)
        } else if is_group {
            (S_IRGRP, S_IWGRP, S_IXGRP)
        } else {
            (S_IROTH, S_IWOTH, S_IXOTH)
        };

        let req_read = access == AccessMode::Read || access == AccessMode::ReadWrite;
        let req_write = access == AccessMode::Write || access == AccessMode::ReadWrite;
        let req_exec = access == AccessMode::Execute;

        if req_read && (file_mode & r_bit) == 0 && !creds.has_capability(CAP_DAC_READ_SEARCH) {
            return Err(FsError::PermissionDenied);
        }
        if req_write && (file_mode & w_bit) == 0 {
            return Err(FsError::PermissionDenied);
        }
        if req_exec && (file_mode & x_bit) == 0 {
            return Err(FsError::PermissionDenied);
        }

        Ok(())
    }

    pub fn check_chmod_permission(creds: &ProcessCredentials, file_uid: u32) -> Result<(), FsError> {
        if creds.euid == 0 || creds.euid == file_uid || creds.has_capability(CAP_FOWNER) {
            Ok(())
        } else {
            Err(FsError::PermissionDenied)
        }
    }
}
