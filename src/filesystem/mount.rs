// SPDX-License-Identifier: MIT
//! Mount Manager Implementation for SigmaOS Filesystems.

use std::collections::BTreeMap;
use std::string::String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MountError {
    Success,
    AlreadyMounted,
    NotMounted,
    PointNotFound,
    DoubleUnmount,
    Busy,
}

#[derive(Debug, Clone)]
pub struct MountPoint {
    pub mount_id: u64,
    pub source: String,
    pub target: String,
    pub fstype: String,
    pub active_references: usize,
}

pub struct MountManager {
    pub mount_table: BTreeMap<String, MountPoint>,
    pub next_mount_id: u64,
}

impl MountManager {
    pub fn new() -> Self {
        Self {
            mount_table: BTreeMap::new(),
            next_mount_id: 1,
        }
    }

    pub fn mount(&mut self, source: &str, target: &str, fstype: &str) -> Result<u64, MountError> {
        let target_str = String::from(target);
        if self.mount_table.contains_key(&target_str) {
            return Err(MountError::AlreadyMounted);
        }

        let mount_id = self.next_mount_id;
        self.next_mount_id += 1;

        self.mount_table.insert(
            target_str.clone(),
            MountPoint {
                mount_id,
                source: String::from(source),
                target: target_str,
                fstype: String::from(fstype),
                active_references: 1,
            },
        );

        Ok(mount_id)
    }

    pub fn unmount(&mut self, target: &str) -> Result<(), MountError> {
        let target_str = String::from(target);
        if let Some(mp) = self.mount_table.get_mut(&target_str) {
            if mp.active_references > 1 {
                mp.active_references -= 1;
                return Err(MountError::Busy);
            }
            self.mount_table.remove(&target_str);
            Ok(())
        } else {
            Err(MountError::DoubleUnmount)
        }
    }
}
