// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaRecovery - System Recovery Daemon
//!
//! SigmaRecovery provides system recovery for SigmaOS, inspired by GRUB/recovery mode from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Recovery mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryMode {
    /// Normal boot
    Normal,
    /// Recovery mode
    Recovery,
    /// Safe mode
    Safe,
    /// Emergency mode
    Emergency,
}

impl fmt::Display for RecoveryMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecoveryMode::Normal => write!(f, "normal"),
            RecoveryMode::Recovery => write!(f, "recovery"),
            RecoveryMode::Safe => write!(f, "safe"),
            RecoveryMode::Emergency => write!(f, "emergency"),
        }
    }
}

/// Recovery action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryAction {
    /// Boot to shell
    BootShell,
    /// Boot to single user
    BootSingleUser,
    /// Boot to last known good
    BootLastKnownGood,
    /// Boot to recovery shell
    BootRecoveryShell,
    /// Boot to diagnostic mode
    BootDiagnostic,
}

impl fmt::Display for RecoveryAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecoveryAction::BootShell => write!(f, "boot_shell"),
            RecoveryAction::BootSingleUser => write!(f, "boot_single_user"),
            RecoveryAction::BootLastKnownGood => write!(f, "boot_last_known_good"),
            RecoveryAction::BootRecoveryShell => write!(f, "boot_recovery_shell"),
            RecoveryAction::BootDiagnostic => write!(f, "boot_diagnostic"),
        }
    }
}

/// Recovery snapshot
#[derive(Debug, Clone)]
pub struct RecoverySnapshot {
    /// Snapshot ID
    pub snapshot_id: String,
    /// Snapshot name
    pub name: String,
    /// Description
    pub description: String,
    /// Timestamp
    pub timestamp: u64,
    /// Size in MB
    pub size_mb: u64,
    /// Is bootable
    pub bootable: bool,
}

impl RecoverySnapshot {
    /// Create a new recovery snapshot
    pub fn new(snapshot_id: String, name: String, description: String) -> Self {
        Self {
            snapshot_id,
            name,
            description,
            timestamp: 0,
            size_mb: 0,
            bootable: true,
        }
    }

    /// Set timestamp
    pub fn with_timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = timestamp;
        self
    }

    /// Set size
    pub fn with_size(mut self, size_mb: u64) -> Self {
        self.size_mb = size_mb;
        self
    }

    /// Set bootable
    pub fn with_bootable(mut self, bootable: bool) -> Self {
        self.bootable = bootable;
        self
    }
}

/// SigmaRecovery - System Recovery Daemon
pub struct SigmaRecovery {
    /// Recovery snapshots (snapshot_id -> snapshot)
    snapshots: BTreeMap<String, RecoverySnapshot>,
    /// Current recovery mode
    current_mode: RecoveryMode,
    /// Running state
    running: bool,
    /// Snapshot counter
    snapshot_counter: u32,
}

impl SigmaRecovery {
    /// Create a new SigmaRecovery instance
    pub fn new() -> Self {
        Self {
            snapshots: BTreeMap::new(),
            current_mode: RecoveryMode::Normal,
            running: false,
            snapshot_counter: 0,
        }
    }

    /// Start the recovery daemon
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the recovery daemon
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the daemon is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Set recovery mode
    pub fn set_mode(&mut self, mode: RecoveryMode) {
        self.current_mode = mode;
    }

    /// Get current recovery mode
    pub fn get_mode(&self) -> RecoveryMode {
        self.current_mode
    }

    /// Add a recovery snapshot
    pub fn add_snapshot(&mut self, snapshot: RecoverySnapshot) -> Result<(), String> {
        if self.snapshots.contains_key(&snapshot.snapshot_id) {
            return Err(format!(
                "Snapshot '{}' already exists",
                snapshot.snapshot_id
            ));
        }

        self.snapshots
            .insert(snapshot.snapshot_id.clone(), snapshot);
        self.snapshot_counter += 1;
        Ok(())
    }

    /// Remove a recovery snapshot
    pub fn remove_snapshot(&mut self, snapshot_id: &str) -> Result<(), String> {
        if self.snapshots.remove(snapshot_id).is_none() {
            return Err(format!("Snapshot '{}' not found", snapshot_id));
        }
        Ok(())
    }

    /// Get a recovery snapshot
    pub fn get_snapshot(&self, snapshot_id: &str) -> Option<&RecoverySnapshot> {
        self.snapshots.get(snapshot_id)
    }

    /// Boot to recovery mode
    pub fn boot_recovery(&mut self, action: RecoveryAction) -> Result<(), String> {
        if !self.running {
            return Err("SigmaRecovery not running".to_string());
        }

        self.current_mode = RecoveryMode::Recovery;
        Ok(())
    }

    /// Boot to normal mode
    pub fn boot_normal(&mut self) -> Result<(), String> {
        if !self.running {
            return Err("SigmaRecovery not running".to_string());
        }

        self.current_mode = RecoveryMode::Normal;
        Ok(())
    }

    /// List all snapshots
    pub fn list_snapshots(&self) -> Vec<&RecoverySnapshot> {
        self.snapshots.values().collect()
    }

    /// List bootable snapshots
    pub fn list_bootable(&self) -> Vec<&RecoverySnapshot> {
        self.snapshots.values().filter(|s| s.bootable).collect()
    }

    /// Get snapshot count
    pub fn snapshot_count(&self) -> usize {
        self.snapshots.len()
    }

    /// Get recovery statistics
    pub fn recovery_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert(
            "total_snapshots".to_string(),
            self.snapshot_count().to_string(),
        );
        stats.insert(
            "bootable_snapshots".to_string(),
            self.list_bootable().len().to_string(),
        );
        stats.insert("current_mode".to_string(), self.current_mode.to_string());
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaRecovery {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaRecovery instance (placeholder for actual global state)
static mut SIGMARECOVERY_INSTANCE: Option<SigmaRecovery> = None;

/// Initialize the global SigmaRecovery instance
pub fn init_sigmarecovery() {
    unsafe {
        SIGMARECOVERY_INSTANCE = Some(SigmaRecovery::new());
    }
}

/// Get the global SigmaRecovery instance
pub fn get_sigmarecovery() -> Option<&'static mut SigmaRecovery> {
    unsafe { SIGMARECOVERY_INSTANCE.as_mut() }
}

/// Convenience function to boot to recovery
pub fn boot_to_recovery(action: RecoveryAction) -> Result<(), String> {
    if let Some(recovery) = get_sigmarecovery() {
        recovery.boot_recovery(action)
    } else {
        Err("SigmaRecovery not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recovery_mode_display() {
        assert_eq!(RecoveryMode::Normal.to_string(), "normal");
        assert_eq!(RecoveryMode::Recovery.to_string(), "recovery");
    }

    #[test]
    fn test_recovery_action_display() {
        assert_eq!(RecoveryAction::BootShell.to_string(), "boot_shell");
        assert_eq!(
            RecoveryAction::BootSingleUser.to_string(),
            "boot_single_user"
        );
    }

    #[test]
    fn test_recovery_snapshot() {
        let snapshot = RecoverySnapshot::new(
            "snap1".to_string(),
            "Before Update".to_string(),
            "System snapshot before update".to_string(),
        )
        .with_timestamp(1234567890)
        .with_size(1024)
        .with_bootable(true);

        assert_eq!(snapshot.snapshot_id, "snap1");
        assert_eq!(snapshot.bootable, true);
        assert_eq!(snapshot.timestamp, 1234567890);
    }

    #[test]
    fn test_sigmarecovery() {
        let mut recovery = SigmaRecovery::new();
        assert!(!recovery.is_running());
        recovery.start();
        assert!(recovery.is_running());
        recovery.stop();
        assert!(!recovery.is_running());
    }

    #[test]
    fn test_sigmarecovery_set_mode() {
        let mut recovery = SigmaRecovery::new();
        recovery.set_mode(RecoveryMode::Recovery);
        assert_eq!(recovery.get_mode(), RecoveryMode::Recovery);
    }

    #[test]
    fn test_sigmarecovery_add_snapshot() {
        let mut recovery = SigmaRecovery::new();

        let snapshot = RecoverySnapshot::new(
            "snap1".to_string(),
            "Before Update".to_string(),
            "System snapshot before update".to_string(),
        );

        assert!(recovery.add_snapshot(snapshot).is_ok());
        assert_eq!(recovery.snapshot_count(), 1);
    }

    #[test]
    fn test_sigmarecovery_boot_recovery() {
        let mut recovery = SigmaRecovery::new();
        recovery.start();

        assert!(recovery.boot_recovery(RecoveryAction::BootShell).is_ok());
        assert_eq!(recovery.get_mode(), RecoveryMode::Recovery);
    }

    #[test]
    fn test_sigmarecovery_boot_normal() {
        let mut recovery = SigmaRecovery::new();
        recovery.start();

        assert!(recovery.boot_normal().is_ok());
        assert_eq!(recovery.get_mode(), RecoveryMode::Normal);
    }

    #[test]
    fn test_sigmarecovery_list_bootable() {
        let mut recovery = SigmaRecovery::new();

        let snapshot = RecoverySnapshot::new(
            "snap1".to_string(),
            "Before Update".to_string(),
            "System snapshot before update".to_string(),
        )
        .with_bootable(true);

        recovery.add_snapshot(snapshot).unwrap();

        let bootable = recovery.list_bootable();
        assert_eq!(bootable.len(), 1);
    }

    #[test]
    fn test_sigmarecovery_stats() {
        let mut recovery = SigmaRecovery::new();

        let snapshot = RecoverySnapshot::new(
            "snap1".to_string(),
            "Before Update".to_string(),
            "System snapshot before update".to_string(),
        )
        .with_bootable(true);

        recovery.add_snapshot(snapshot).unwrap();

        let stats = recovery.recovery_stats();
        assert_eq!(stats.get("total_snapshots"), Some(&"1".to_string()));
        assert_eq!(stats.get("bootable_snapshots"), Some(&"1".to_string()));
    }

    #[test]
    fn test_boot_to_recovery_convenience() {
        init_sigmarecovery();

        let mut recovery = get_sigmarecovery().unwrap();
        recovery.start();

        let result = boot_to_recovery(RecoveryAction::BootShell);
        assert!(result.is_ok());
    }
}
