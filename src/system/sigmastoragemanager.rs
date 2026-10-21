use alloc::string::String;

#[derive(Debug, Clone)]
pub struct StorageDevice {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilesystemType {
    Ext4,
    Zfs,
    Btrfs,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MountState {
    Mounted,
    Unmounted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageType {
    Ssd,
    Hdd,
    Nvme,
}

pub struct SigmaStorageManager;
