// SPDX-License-Identifier: MIT
//! Core Filesystem Manager Implementation for SigmaOS.

use std::collections::BTreeMap;
use std::string::String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsCoreError {
    Success,
    CapacityExceeded,
    NodeNotFound,
    AlreadyExists,
    InconsistentState,
}

#[derive(Debug, Clone)]
pub struct FsNode {
    pub node_id: u64,
    pub name: String,
    pub is_dir: bool,
    pub size_bytes: u64,
}

pub struct FilesystemCoreManager {
    pub volume_name: String,
    pub max_capacity_bytes: u64,
    pub used_capacity_bytes: u64,
    pub nodes: BTreeMap<u64, FsNode>,
    pub next_node_id: u64,
}

impl FilesystemCoreManager {
    pub fn new(volume_name: &str, max_capacity_bytes: u64) -> Self {
        Self {
            volume_name: String::from(volume_name),
            max_capacity_bytes,
            used_capacity_bytes: 0,
            nodes: BTreeMap::new(),
            next_node_id: 1,
        }
    }

    pub fn create_file(&mut self, name: &str, size_bytes: u64) -> Result<u64, FsCoreError> {
        if self.used_capacity_bytes + size_bytes > self.max_capacity_bytes {
            return Err(FsCoreError::CapacityExceeded);
        }

        let id = self.next_node_id;
        self.next_node_id += 1;

        self.nodes.insert(
            id,
            FsNode {
                node_id: id,
                name: String::from(name),
                is_dir: false,
                size_bytes,
            },
        );

        self.used_capacity_bytes += size_bytes;
        Ok(id)
    }

    pub fn delete_file(&mut self, node_id: u64) -> Result<(), FsCoreError> {
        if let Some(node) = self.nodes.remove(&node_id) {
            self.used_capacity_bytes = self.used_capacity_bytes.saturating_sub(node.size_bytes);
            Ok(())
        } else {
            Err(FsCoreError::NodeNotFound)
        }
    }
}
