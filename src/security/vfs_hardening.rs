// SigmaOS VFS Hardening Subsystem
// Enforces fs.protected_symlinks, fs.protected_hardlinks, and sticky bit directory access controls.

#![allow(dead_code)]
extern crate alloc;

/// Policy configuration for VFS link protection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VfsHardeningConfig {
    pub protected_symlinks: bool, // Prevent following symlinks in sticky world-writable dirs if follower != owner
    pub protected_hardlinks: bool, // Prevent hardlinking to files caller doesn't own or can't write to
    pub protected_fifos: bool, // Prevent opening FIFOs in sticky world-writable dirs not owned by caller
    pub protected_regular: bool, // Prevent opening regular files in sticky world-writable dirs not owned by caller
}

impl Default for VfsHardeningConfig {
    fn default() -> Self {
        Self {
            protected_symlinks: true,
            protected_hardlinks: true,
            protected_fifos: true,
            protected_regular: true,
        }
    }
}

/// VFS Security Enforcer
#[derive(Debug, Clone, Default)]
pub struct VfsHardeningEnforcer {
    pub config: VfsHardeningConfig,
    pub violations_count: u64,
}

impl VfsHardeningEnforcer {
    pub fn new(config: VfsHardeningConfig) -> Self {
        Self {
            config,
            violations_count: 0,
        }
    }

    /// Check if following a symlink is permitted under fs.protected_symlinks
    pub fn verify_symlink_follow(
        &mut self,
        follower_uid: u32,
        symlink_uid: u32,
        parent_dir_uid: u32,
        parent_dir_is_sticky: bool,
    ) -> Result<(), &'static str> {
        if !self.config.protected_symlinks {
            return Ok(());
        }

        // If parent directory is sticky world-writable (like /tmp):
        // Follower must be either the owner of the symlink, owner of the directory, or root (uid 0)
        if parent_dir_is_sticky {
            if follower_uid != 0 && follower_uid != symlink_uid && follower_uid != parent_dir_uid {
                self.violations_count += 1;
                return Err("EACCES: Symlink traversal rejected by fs.protected_symlinks policy");
            }
        }

        Ok(())
    }

    /// Check if creating a hardlink is permitted under fs.protected_hardlinks
    pub fn verify_hardlink_creation(
        &mut self,
        creator_uid: u32,
        target_file_uid: u32,
        target_file_is_regular: bool,
        target_file_has_suid_sgid: bool,
        caller_has_write_perm: bool,
    ) -> Result<(), &'static str> {
        if !self.config.protected_hardlinks {
            return Ok(());
        }

        if creator_uid == 0 {
            return Ok(()); // Root can hardlink
        }

        // Cannot hardlink to setuid/setgid files
        if target_file_has_suid_sgid {
            self.violations_count += 1;
            return Err(
                "EPERM: Hardlink to setuid/setgid binary rejected by fs.protected_hardlinks",
            );
        }

        // Must own the target or have read+write permissions to regular file
        if creator_uid != target_file_uid && (!target_file_is_regular || !caller_has_write_perm) {
            self.violations_count += 1;
            return Err(
                "EPERM: Hardlink creation to unowned file without write permissions rejected",
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vfs_hardening_symlink_and_hardlink_rules() {
        let mut enforcer = VfsHardeningEnforcer::default();

        // Symlink in /tmp (sticky) owned by victim (uid 1000). Attacker (uid 2000) tries to follow -> FAIL
        assert!(enforcer.verify_symlink_follow(2000, 1000, 0, true).is_err());
        assert_eq!(enforcer.violations_count, 1);

        // Owner (uid 1000) following own symlink -> OK
        assert!(enforcer.verify_symlink_follow(1000, 1000, 0, true).is_ok());

        // Hardlink to setuid file -> FAIL
        assert!(enforcer
            .verify_hardlink_creation(1000, 0, true, true, true)
            .is_err());
        assert_eq!(enforcer.violations_count, 2);
    }
}
