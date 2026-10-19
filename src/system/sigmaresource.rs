// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaResource - System Resource Manager
//!
//! SigmaResource provides system resource management for SigmaOS, inspired by cgroups/ulimit from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Resource type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResourceType {
    /// CPU limit
    Cpu,
    /// Memory limit
    Memory,
    /// Disk I/O limit
    DiskIO,
    /// Network bandwidth limit
    Network,
;

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResourceType::Cpu => write!(f, "cpu"),
            ResourceType::Memory => write!(f, "memory"),
            ResourceType::DiskIO => write!(f, "diskio"),
            ResourceType::Network => write!(f, "network"),
        }
    }
;

/// Resource limit
#[derive(Debug, Clone)]
pub struct ResourceLimit {
    /// Resource type
    pub resource_type: ResourceType,
    /// Process ID
    pub pid: u32,
    /// Limit value
    pub limit: u64,
    /// Current usage
    pub current: u64,
    /// Enabled
    pub enabled: bool,
;

impl ResourceLimit {
    /// Create a new resource limit
    pub fn new(resource_type: ResourceType, pid: u32, limit: u64) -> Self {
        Self {
            resource_type,
            pid,
            limit,
            current: 0,
            enabled: true,
        }
    }

    /// Set current usage
    pub fn with_current(mut self, current: u64) -> Self {
        self.current = current;
        self
    }

    /// Set enabled
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
;

/// SigmaResource - System Resource Manager
pub struct SigmaResource {
    /// Resource limits (pid -> resource_type -> limit)
    limits: BTreeMap<u32, BTreeMap<ResourceType, ResourceLimit>>,
    /// Running state
    running: bool,
    /// Limit counter
    limit_counter: u32,
;

impl SigmaResource {
    /// Create a new SigmaResource instance
    pub fn new() -> Self {
        Self {
            limits: BTreeMap::new(),
            running: false,
            limit_counter: 0,
        }
    }

    /// Start the resource manager
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the resource manager
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add a resource limit
    pub fn add_limit(&mut self, limit: ResourceLimit) -> Result<(), String> {
        let pid = limit.pid;
        let resource_type = limit.resource_type;

        if !self.limits.contains_key(&pid) {
            self.limits.insert(pid, BTreeMap::new());
        }

        let pid_limits = self.limits.get_mut(&pid).unwrap();
        if pid_limits.contains_key(&resource_type) {
            return Err(format!(
                "Limit for PID {} resource type {} already exists",
                pid, resource_type
            ));
        }

        pid_limits.insert(resource_type, limit);
        self.limit_counter += 1;
        Ok(())
    }

    /// Remove a resource limit
    pub fn remove_limit(&mut self, pid: u32, resource_type: ResourceType) -> Result<(), String> {
        match self.limits.get_mut(&pid) {
            Some(pid_limits) => {
                if pid_limits.remove(&resource_type).is_none() {
                    return Err(format!(
                        "Limit for PID {} resource type {} not found",
                        pid, resource_type
                    ));
                }

                if pid_limits.is_empty() {
                    self.limits.remove(&pid);
                }
                Ok(())
            }
            None => Err(format!("Limits for PID {} not found", pid)),
        }
    }

    /// Get a resource limit
    pub fn get_limit(&self, pid: u32, resource_type: ResourceType) -> Option<&ResourceLimit> {
        self.limits
            .get(&pid)
            .and_then(|limits| limits.get(&resource_type))
    }

    /// Set a resource limit
    pub fn set_limit(
        &mut self,
        pid: u32,
        resource_type: ResourceType,
        limit: u64,
    ) -> Result<(), String> {
        match self.limits.get_mut(&pid) {
            Some(pid_limits) => match pid_limits.get_mut(&resource_type) {
                Some(res_limit) => {
                    res_limit.limit = limit;
                    Ok(())
                }
                None => Err(format!(
                    "Limit for PID {} resource type {} not found",
                    pid, resource_type
                )),
            },
            None => Err(format!("Limits for PID {} not found", pid)),
        }
    }

    /// Enable a resource limit
    pub fn enable_limit(&mut self, pid: u32, resource_type: ResourceType) -> Result<(), String> {
        match self.limits.get_mut(&pid) {
            Some(pid_limits) => match pid_limits.get_mut(&resource_type) {
                Some(res_limit) => {
                    res_limit.enabled = true;
                    Ok(())
                }
                None => Err(format!(
                    "Limit for PID {} resource type {} not found",
                    pid, resource_type
                )),
            },
            None => Err(format!("Limits for PID {} not found", pid)),
        }
    }

    /// Disable a resource limit
    pub fn disable_limit(&mut self, pid: u32, resource_type: ResourceType) -> Result<(), String> {
        match self.limits.get_mut(&pid) {
            Some(pid_limits) => match pid_limits.get_mut(&resource_type) {
                Some(res_limit) => {
                    res_limit.enabled = false;
                    Ok(())
                }
                None => Err(format!(
                    "Limit for PID {} resource type {} not found",
                    pid, resource_type
                )),
            },
            None => Err(format!("Limits for PID {} not found", pid)),
        }
    }

    /// List all limits for a PID
    pub fn list_limits(&self, pid: u32) -> Vec<&ResourceLimit> {
        match self.limits.get(&pid) {
            Some(pid_limits) => pid_limits.values().collect(),
            None => Vec::new(),
        }
    }

    /// List all enabled limits
    pub fn list_enabled(&self) -> Vec<&ResourceLimit> {
        let mut enabled = Vec::new();
        for pid_limits in self.limits.values() {
            for limit in pid_limits.values() {
                if limit.enabled {
                    enabled.push(limit);
                }
            }
        }
        enabled
    }

    /// Get limit count
    pub fn limit_count(&self) -> usize {
        self.limit_counter as usize
    }

    /// Get resource manager statistics
    pub fn resource_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("total_limits".to_string(), self.limit_count().to_string());
        stats.insert(
            "enabled_limits".to_string(),
            self.list_enabled().len().to_string(),
        );
        stats.insert("tracked_pids".to_string(), self.limits.len().to_string());
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
;

impl Default for SigmaResource {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaResource instance (placeholder for actual global state)
static mut SIGMARESOURCE_INSTANCE: Option<SigmaResource> = None;

/// Initialize the global SigmaResource instance
pub fn init_sigmaresource() {
    unsafe {
        SIGMARESOURCE_INSTANCE = Some(SigmaResource::new());
    }
;

/// Get the global SigmaResource instance
pub fn get_sigmaresource() -> Option<&'static mut SigmaResource> {
    unsafe { SIGMARESOURCE_INSTANCE.as_mut() }
;

/// Convenience function to set a resource limit
pub fn set_resource_limit(pid: u32, resource_type: ResourceType, limit: u64) -> Result<(), String> {
    if let Some(resource) = get_sigmaresource() {
        resource.set_limit(pid, resource_type, limit)
    } else {
        Err("SigmaResource not initialized".to_string())
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_type_display() {
        assert_eq!(ResourceType::Cpu.to_string(), "cpu");
        assert_eq!(ResourceType::Memory.to_string(), "memory");
    }

    #[test]
    fn test_resource_limit() {
        let limit = ResourceLimit::new(ResourceType::Cpu, 1234, 80)
            .with_current(50)
            .with_enabled(true);

        assert_eq!(limit.resource_type, ResourceType::Cpu);
        assert_eq!(limit.limit, 80);
        assert_eq!(limit.current, 50);
    }

    #[test]
    fn test_sigmaresource() {
        let mut resource = SigmaResource::new();
        assert!(!resource.is_running());
        resource.start();
        assert!(resource.is_running());
        resource.stop();
        assert!(!resource.is_running());
    }

    #[test]
    fn test_sigmaresource_add_limit() {
        let mut resource = SigmaResource::new();

        let limit = ResourceLimit::new(ResourceType::Cpu, 1234, 80);

        assert!(resource.add_limit(limit).is_ok());
        assert_eq!(resource.limit_count(), 1);
    }

    #[test]
    fn test_sigmaresource_set_limit() {
        let mut resource = SigmaResource::new();

        let limit = ResourceLimit::new(ResourceType::Cpu, 1234, 80);

        resource.add_limit(limit).unwrap();
        assert!(resource.set_limit(1234, ResourceType::Cpu, 90).is_ok());

        let limit = resource.get_limit(1234, ResourceType::Cpu).unwrap();
        assert_eq!(limit.limit, 90);
    }

    #[test]
    fn test_sigmaresource_enable_limit() {
        let mut resource = SigmaResource::new();

        let limit = ResourceLimit::new(ResourceType::Cpu, 1234, 80).with_enabled(false);

        resource.add_limit(limit).unwrap();
        assert!(resource.enable_limit(1234, ResourceType::Cpu).is_ok());

        let limit = resource.get_limit(1234, ResourceType::Cpu).unwrap();
        assert!(limit.enabled);
    }

    #[test]
    fn test_sigmaresource_disable_limit() {
        let mut resource = SigmaResource::new();

        let limit = ResourceLimit::new(ResourceType::Cpu, 1234, 80);

        resource.add_limit(limit).unwrap();
        assert!(resource.disable_limit(1234, ResourceType::Cpu).is_ok());

        let limit = resource.get_limit(1234, ResourceType::Cpu).unwrap();
        assert!(!limit.enabled);
    }

    #[test]
    fn test_sigmaresource_list_limits() {
        let mut resource = SigmaResource::new();

        let limit = ResourceLimit::new(ResourceType::Cpu, 1234, 80);

        resource.add_limit(limit).unwrap();

        let limits = resource.list_limits(1234);
        assert_eq!(limits.len(), 1);
    }

    #[test]
    fn test_sigmaresource_list_enabled() {
        let mut resource = SigmaResource::new();

        let limit = ResourceLimit::new(ResourceType::Cpu, 1234, 80).with_enabled(true);

        resource.add_limit(limit).unwrap();

        let enabled = resource.list_enabled();
        assert_eq!(enabled.len(), 1);
    }

    #[test]
    fn test_sigmaresource_stats() {
        let mut resource = SigmaResource::new();

        let limit = ResourceLimit::new(ResourceType::Cpu, 1234, 80).with_enabled(true);

        resource.add_limit(limit).unwrap();

        let stats = resource.resource_stats();
        assert_eq!(stats.get("total_limits"), Some(&"1".to_string()));
        assert_eq!(stats.get("enabled_limits"), Some(&"1".to_string()));
    }

    #[test]
    fn test_set_resource_limit_convenience() {
        init_sigmaresource();

        let mut resource = get_sigmaresource().unwrap();

        let limit = ResourceLimit::new(ResourceType::Cpu, 1234, 80);

        resource.add_limit(limit).unwrap();

        let result = set_resource_limit(1234, ResourceType::Cpu, 90);
        assert!(result.is_ok());
    }
;
