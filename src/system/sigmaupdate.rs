// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaUpdate - System Update Daemon
//!
//! SigmaUpdate provides system update management for SigmaOS, inspired by unattended-upgrades from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Update type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateType {
    /// Security update
    Security,
    /// Bug fix update
    BugFix,
    /// Feature update
    Feature,
    /// Dependency update
    Dependency,
;

impl fmt::Display for UpdateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UpdateType::Security => write!(f, "security"),
            UpdateType::BugFix => write!(f, "bugfix"),
            UpdateType::Feature => write!(f, "feature"),
            UpdateType::Dependency => write!(f, "dependency"),
        }
    }
;

/// Update status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateStatus {
    /// Pending
    Pending,
    /// Downloading
    Downloading,
    /// Installing
    Installing,
    /// Completed
    Completed,
    /// Failed
    Failed,
;

impl fmt::Display for UpdateStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UpdateStatus::Pending => write!(f, "pending"),
            UpdateStatus::Downloading => write!(f, "downloading"),
            UpdateStatus::Installing => write!(f, "installing"),
            UpdateStatus::Completed => write!(f, "completed"),
            UpdateStatus::Failed => write!(f, "failed"),
        }
    }
;

/// Update policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdatePolicy {
    /// Auto install
    AutoInstall,
    /// Download only
    DownloadOnly,
    /// Manual only
    ManualOnly,
;

impl fmt::Display for UpdatePolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UpdatePolicy::AutoInstall => write!(f, "auto_install"),
            UpdatePolicy::DownloadOnly => write!(f, "download_only"),
            UpdatePolicy::ManualOnly => write!(f, "manual_only"),
        }
    }
;

/// System update
#[derive(Debug, Clone)]
pub struct SystemUpdate {
    /// Update ID
    pub update_id: String,
    /// Package name
    pub package: String,
    /// Old version
    pub old_version: String,
    /// New version
    pub new_version: String,
    /// Update type
    pub update_type: UpdateType,
    /// Update status
    pub status: UpdateStatus,
    /// Size in bytes
    pub size: u64,
;

impl SystemUpdate {
    /// Create a new system update
    pub fn new(
        update_id: String,
        package: String,
        old_version: String,
        new_version: String,
        update_type: UpdateType,
    ) -> Self {
        Self {
            update_id,
            package,
            old_version,
            new_version,
            update_type,
            status: UpdateStatus::Pending,
            size: 0,
        }
    }

    /// Set status
    pub fn with_status(mut self, status: UpdateStatus) -> Self {
        self.status = status;
        self
    }

    /// Set size
    pub fn with_size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }
;

/// SigmaUpdate - System Update Daemon
pub struct SigmaUpdate {
    /// System updates (update_id -> update)
    updates: BTreeMap<String, SystemUpdate>,
    /// Update policy
    policy: UpdatePolicy,
    /// Running state
    running: bool,
    /// Update counter
    update_counter: u32,
;

impl SigmaUpdate {
    /// Create a new SigmaUpdate instance
    pub fn new() -> Self {
        Self {
            updates: BTreeMap::new(),
            policy: UpdatePolicy::AutoInstall,
            running: false,
            update_counter: 0,
        }
    }

    /// Start the update daemon
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the update daemon
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the daemon is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Set update policy
    pub fn set_policy(&mut self, policy: UpdatePolicy) {
        self.policy = policy;
    }

    /// Get update policy
    pub fn get_policy(&self) -> UpdatePolicy {
        self.policy
    }

    /// Add a system update
    pub fn add_update(&mut self, update: SystemUpdate) -> Result<(), String> {
        if self.updates.contains_key(&update.update_id) {
            return Err(format!("Update '{}' already exists", update.update_id));
        }

        self.updates.insert(update.update_id.clone(), update);
        self.update_counter += 1;
        Ok(())
    }

    /// Remove a system update
    pub fn remove_update(&mut self, update_id: &str) -> Result<(), String> {
        if self.updates.remove(update_id).is_none() {
            return Err(format!("Update '{}' not found", update_id));
        }
        Ok(())
    }

    /// Get a system update
    pub fn get_update(&self, update_id: &str) -> Option<&SystemUpdate> {
        self.updates.get(update_id)
    }

    /// Apply an update
    pub fn apply_update(&mut self, update_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaUpdate not running".to_string());
        }

        match self.updates.get_mut(update_id) {
            Some(update) => {
                if self.policy == UpdatePolicy::ManualOnly {
                    return Err("Manual-only policy, cannot auto-apply".to_string());
                }

                update.status = UpdateStatus::Installing;
                update.status = UpdateStatus::Completed;
                Ok(())
            }
            None => Err(format!("Update '{}' not found", update_id)),
        }
    }

    /// List all updates
    pub fn list_updates(&self) -> Vec<&SystemUpdate> {
        self.updates.values().collect()
    }

    /// List pending updates
    pub fn list_pending(&self) -> Vec<&SystemUpdate> {
        self.updates
            .values()
            .filter(|u| u.status == UpdateStatus::Pending)
            .collect()
    }

    /// List security updates
    pub fn list_security(&self) -> Vec<&SystemUpdate> {
        self.updates
            .values()
            .filter(|u| u.update_type == UpdateType::Security)
            .collect()
    }

    /// Get update count
    pub fn update_count(&self) -> usize {
        self.updates.len()
    }

    /// Get update statistics
    pub fn update_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("total_updates".to_string(), self.update_count().to_string());
        stats.insert(
            "pending_updates".to_string(),
            self.list_pending().len().to_string(),
        );
        stats.insert(
            "security_updates".to_string(),
            self.list_security().len().to_string(),
        );
        stats.insert("policy".to_string(), self.policy.to_string());
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
;

impl Default for SigmaUpdate {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaUpdate instance (placeholder for actual global state)
static mut SIGMAUPDATE_INSTANCE: Option<SigmaUpdate> = None;

/// Initialize the global SigmaUpdate instance
pub fn init_sigmaupdate() {
    unsafe {
        SIGMAUPDATE_INSTANCE = Some(SigmaUpdate::new());
    }
;

/// Get the global SigmaUpdate instance
pub fn get_sigmaupdate() -> Option<&'static mut SigmaUpdate> {
    unsafe { SIGMAUPDATE_INSTANCE.as_mut() }
;

/// Convenience function to apply an update
pub fn apply_system_update(update_id: &str) -> Result<(), String> {
    if let Some(update) = get_sigmaupdate() {
        update.apply_update(update_id)
    } else {
        Err("SigmaUpdate not initialized".to_string())
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_type_display() {
        assert_eq!(UpdateType::Security.to_string(), "security");
        assert_eq!(UpdateType::BugFix.to_string(), "bugfix");
    }

    #[test]
    fn test_update_status_display() {
        assert_eq!(UpdateStatus::Pending.to_string(), "pending");
        assert_eq!(UpdateStatus::Completed.to_string(), "completed");
    }

    #[test]
    fn test_update_policy_display() {
        assert_eq!(UpdatePolicy::AutoInstall.to_string(), "auto_install");
        assert_eq!(UpdatePolicy::ManualOnly.to_string(), "manual_only");
    }

    #[test]
    fn test_system_update() {
        let update = SystemUpdate::new(
            "update1".to_string(),
            "vim".to_string(),
            "1.0.0".to_string(),
            "2.0.0".to_string(),
            UpdateType::Security,
        )
        .with_status(UpdateStatus::Completed)
        .with_size(1024);

        assert_eq!(update.package, "vim");
        assert_eq!(update.status, UpdateStatus::Completed);
        assert_eq!(update.size, 1024);
    }

    #[test]
    fn test_sigmaupdate() {
        let mut update = SigmaUpdate::new();
        assert!(!update.is_running());
        update.start();
        assert!(update.is_running());
        update.stop();
        assert!(!update.is_running());
    }

    #[test]
    fn test_sigmaupdate_set_policy() {
        let mut update = SigmaUpdate::new();
        update.set_policy(UpdatePolicy::DownloadOnly);
        assert_eq!(update.get_policy(), UpdatePolicy::DownloadOnly);
    }

    #[test]
    fn test_sigmaupdate_add_update() {
        let mut update = SigmaUpdate::new();

        let sys_update = SystemUpdate::new(
            "update1".to_string(),
            "vim".to_string(),
            "1.0.0".to_string(),
            "2.0.0".to_string(),
            UpdateType::Security,
        );

        assert!(update.add_update(sys_update).is_ok());
        assert_eq!(update.update_count(), 1);
    }

    #[test]
    fn test_sigmaupdate_apply_update() {
        let mut update = SigmaUpdate::new();
        update.start();

        let sys_update = SystemUpdate::new(
            "update1".to_string(),
            "vim".to_string(),
            "1.0.0".to_string(),
            "2.0.0".to_string(),
            UpdateType::Security,
        );

        update.add_update(sys_update).unwrap();
        assert!(update.apply_update("update1").is_ok());

        let sys_update = update.get_update("update1").unwrap();
        assert_eq!(sys_update.status, UpdateStatus::Completed);
    }

    #[test]
    fn test_sigmaupdate_list_pending() {
        let mut update = SigmaUpdate::new();

        let sys_update = SystemUpdate::new(
            "update1".to_string(),
            "vim".to_string(),
            "1.0.0".to_string(),
            "2.0.0".to_string(),
            UpdateType::Security,
        )
        .with_status(UpdateStatus::Pending);

        update.add_update(sys_update).unwrap();

        let pending = update.list_pending();
        assert_eq!(pending.len(), 1);
    }

    #[test]
    fn test_sigmaupdate_list_security() {
        let mut update = SigmaUpdate::new();

        let sys_update = SystemUpdate::new(
            "update1".to_string(),
            "vim".to_string(),
            "1.0.0".to_string(),
            "2.0.0".to_string(),
            UpdateType::Security,
        );

        update.add_update(sys_update).unwrap();

        let security = update.list_security();
        assert_eq!(security.len(), 1);
    }

    #[test]
    fn test_sigmaupdate_stats() {
        let mut update = SigmaUpdate::new();

        let sys_update = SystemUpdate::new(
            "update1".to_string(),
            "vim".to_string(),
            "1.0.0".to_string(),
            "2.0.0".to_string(),
            UpdateType::Security,
        )
        .with_status(UpdateStatus::Pending);

        update.add_update(sys_update).unwrap();

        let stats = update.update_stats();
        assert_eq!(stats.get("total_updates"), Some(&"1".to_string()));
        assert_eq!(stats.get("pending_updates"), Some(&"1".to_string()));
    }

    #[test]
    fn test_apply_system_update_convenience() {
        init_sigmaupdate();

        let mut update = get_sigmaupdate().unwrap();
        update.start();

        let sys_update = SystemUpdate::new(
            "update1".to_string(),
            "vim".to_string(),
            "1.0.0".to_string(),
            "2.0.0".to_string(),
            UpdateType::Security,
        );

        update.add_update(sys_update).unwrap();

        let result = apply_system_update("update1");
        assert!(result.is_ok());
    }
;
