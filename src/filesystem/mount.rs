use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MountError {
    AlreadyMounted,
    NotMounted,
    DoubleUnmount,
    ResourceBusy,
    InvalidSource,
    InvalidTarget,
    InitializationFailed,
    RollbackSuccessful,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MountState {
    Unmounted,
    Mounting,
    Mounted,
    Unmounting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CustomMountFlags {
    pub read_only: bool,
    pub no_exec: bool,
    pub no_suid: bool,
    pub sync: bool,
    pub bind: bool,
}

impl Default for CustomMountFlags {
    fn default() -> Self {
        Self {
            read_only: false,
            no_exec: false,
            no_suid: false,
            sync: true,
            bind: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MountEntry {
    pub mount_point: String,
    pub device_source: String,
    pub fs_type: String,
    pub flags: CustomMountFlags,
    pub state: MountState,
    pub open_ref_count: usize,
}

pub struct MountManager {
    pub mounts: BTreeMap<String, MountEntry>,
}

impl MountManager {
    pub fn new() -> Self {
        Self {
            mounts: BTreeMap::new(),
        }
    }

    pub fn mount(
        &mut self,
        source: &str,
        target: &str,
        fs_type: &str,
        flags: CustomMountFlags,
    ) -> Result<(), MountError> {
        let target_str = target.to_string();
        if let Some(entry) = self.mounts.get(&target_str) {
            if entry.state == MountState::Mounted {
                return Err(MountError::AlreadyMounted);
            }
        }

        let entry = MountEntry {
            mount_point: target_str.clone(),
            device_source: source.to_string(),
            fs_type: fs_type.to_string(),
            flags,
            state: MountState::Mounted,
            open_ref_count: 0,
        };

        self.mounts.insert(target_str, entry);
        Ok(())
    }

    pub fn mount_with_rollback<F>(
        &mut self,
        source: &str,
        target: &str,
        fs_type: &str,
        flags: CustomMountFlags,
        init_fn: F,
    ) -> Result<(), MountError>
    where
        F: FnOnce() -> Result<(), ()>,
    {
        let target_str = target.to_string();
        if self.mounts.contains_key(&target_str) {
            return Err(MountError::AlreadyMounted);
        }

        let mut entry = MountEntry {
            mount_point: target_str.clone(),
            device_source: source.to_string(),
            fs_type: fs_type.to_string(),
            flags,
            state: MountState::Mounting,
            open_ref_count: 0,
        };

        if init_fn().is_err() {
            entry.state = MountState::Unmounted;
            return Err(MountError::RollbackSuccessful);
        }

        entry.state = MountState::Mounted;
        self.mounts.insert(target_str, entry);
        Ok(())
    }

    pub fn unmount(&mut self, target: &str) -> Result<(), MountError> {
        let target_str = target.to_string();
        let entry = match self.mounts.get_mut(&target_str) {
            Some(e) => e,
            None => return Err(MountError::NotMounted),
        };

        if entry.state == MountState::Unmounted {
            return Err(MountError::DoubleUnmount);
        }

        if entry.open_ref_count > 0 {
            return Err(MountError::ResourceBusy);
        }

        entry.state = MountState::Unmounted;
        self.mounts.remove(&target_str);
        Ok(())
    }

    pub fn acquire_ref(&mut self, target: &str) -> Result<(), MountError> {
        let entry = self
            .mounts
            .get_mut(target)
            .ok_or(MountError::NotMounted)?;
        if entry.state != MountState::Mounted {
            return Err(MountError::NotMounted);
        }
        entry.open_ref_count += 1;
        Ok(())
    }

    pub fn release_ref(&mut self, target: &str) -> Result<(), MountError> {
        let entry = self
            .mounts
            .get_mut(target)
            .ok_or(MountError::NotMounted)?;
        if entry.open_ref_count > 0 {
            entry.open_ref_count -= 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mount_lifecycle_and_double_unmount_prevention() {
        let mut mgr = MountManager::new();
        let flags = CustomMountFlags::default();

        assert!(mgr.mount("/dev/nvme0n1p1", "/mnt/data", "ext4", flags).is_ok());

        // Acquire active file reference
        assert!(mgr.acquire_ref("/mnt/data").is_ok());

        // Attempting to unmount active mount fails with ResourceBusy
        assert_eq!(mgr.unmount("/mnt/data"), Err(MountError::ResourceBusy));

        // Release reference and unmount
        assert!(mgr.release_ref("/mnt/data").is_ok());
        assert!(mgr.unmount("/mnt/data").is_ok());

        // Double unmount test
        assert_eq!(mgr.unmount("/mnt/data"), Err(MountError::NotMounted));
    }

    #[test]
    fn test_mount_rollback_on_failure() {
        let mut mgr = MountManager::new();
        let flags = CustomMountFlags::default();

        let res = mgr.mount_with_rollback("/dev/sdb1", "/mnt/fail", "zfs", flags, || {
            Err(()) // Initialization failure triggers rollback
        });

        assert_eq!(res, Err(MountError::RollbackSuccessful));
        assert!(!mgr.mounts.contains_key("/mnt/fail"));
    }
}
