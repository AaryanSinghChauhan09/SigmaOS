// SPDX-License-Identifier: MIT
// SigmaOS - Resource Manager
// Resource management inspired by Arch (cgroups), Debian (systemd resource limits), and CachyOS

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Resource type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    CPU,
    Memory,
    DiskIO,
    Network,
}

/// Resource limit
#[derive(Debug, Clone)]
pub struct ResourceLimit {
    pub resource_type: ResourceType,
    pub limit_value: u64,
    pub unit: String,
}

/// Resource quota
#[derive(Debug, Clone)]
pub struct ResourceQuota {
    pub process_id: u32,
    pub limits: Vec<ResourceLimit>,
}

/// Resource manager
pub struct SigmaResourceManager {
    quotas: Vec<ResourceQuota>,
}

impl SigmaResourceManager {
    pub fn new() -> Self {
        Self {
            quotas: Vec::new(),
        }
    }

    pub fn set_quota(&mut self, process_id: u32, limits: Vec<ResourceLimit>) {
        let quota = ResourceQuota {
            process_id,
            limits,
        };
        if let Some(idx) = self.quotas.iter().position(|q| q.process_id == process_id) {
            self.quotas[idx] = quota;
        } else {
            self.quotas.push(quota);
        }
    }

    pub fn get_quotas(&self) -> &[ResourceQuota] {
        &self.quotas
    }

    pub fn get_quota(&self, process_id: u32) -> Option<&ResourceQuota> {
        self.quotas.iter().find(|q| q.process_id == process_id)
    }

    pub fn remove_quota(&mut self, process_id: u32) -> Result<(), String> {
        if let Some(idx) = self.quotas.iter().position(|q| q.process_id == process_id) {
            self.quotas.remove(idx);
            Ok(())
        } else {
            Err(format!("Quota for process {} not found", process_id))
        }
    }

    pub fn get_limit(&self, process_id: u32, resource_type: ResourceType) -> Option<&ResourceLimit> {
        self.get_quota(process_id)
            .and_then(|q| q.limits.iter().find(|l| l.resource_type == resource_type))
    }

    pub fn set_limit(&mut self, process_id: u32, limit: ResourceLimit) {
        if let Some(quota) = self.quotas.iter_mut().find(|q| q.process_id == process_id) {
            if let Some(idx) = quota.limits.iter().position(|l| l.resource_type == limit.resource_type) {
                quota.limits[idx] = limit;
            } else {
                quota.limits.push(limit);
            }
        } else {
            let quota = ResourceQuota {
                process_id,
                limits: vec![limit],
            };
            self.quotas.push(quota);
        }
    }
}

impl Default for SigmaResourceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_manager_creation() {
        let manager = SigmaResourceManager::new();
        assert_eq!(manager.get_quotas().len(), 0);
    }

    #[test]
    fn test_set_quota() {
        let mut manager = SigmaResourceManager::new();
        let limits = vec![
            ResourceLimit {
                resource_type: ResourceType::CPU,
                limit_value: 50,
                unit: "percent".to_string(),
            },
        ];
        manager.set_quota(1, limits);
        assert_eq!(manager.get_quotas().len(), 1);
    }

    #[test]
    fn test_get_quota() {
        let mut manager = SigmaResourceManager::new();
        let limits = vec![
            ResourceLimit {
                resource_type: ResourceType::CPU,
                limit_value: 50,
                unit: "percent".to_string(),
            },
        ];
        manager.set_quota(1, limits);
        assert!(manager.get_quota(1).is_some());
        assert!(manager.get_quota(2).is_none());
    }

    #[test]
    fn test_remove_quota() {
        let mut manager = SigmaResourceManager::new();
        let limits = vec![
            ResourceLimit {
                resource_type: ResourceType::CPU,
                limit_value: 50,
                unit: "percent".to_string(),
            },
        ];
        manager.set_quota(1, limits);
        manager.remove_quota(1).unwrap();
        assert_eq!(manager.get_quotas().len(), 0);
    }

    #[test]
    fn test_get_limit() {
        let mut manager = SigmaResourceManager::new();
        let limits = vec![
            ResourceLimit {
                resource_type: ResourceType::CPU,
                limit_value: 50,
                unit: "percent".to_string(),
            },
        ];
        manager.set_quota(1, limits);
        assert!(manager.get_limit(1, ResourceType::CPU).is_some());
        assert!(manager.get_limit(1, ResourceType::Memory).is_none());
    }

    #[test]
    fn test_set_limit() {
        let mut manager = SigmaResourceManager::new();
        let limit = ResourceLimit {
            resource_type: ResourceType::CPU,
            limit_value: 50,
            unit: "percent".to_string(),
        };
        manager.set_limit(1, limit);
        assert_eq!(manager.get_quotas().len(), 1);
    }
}
