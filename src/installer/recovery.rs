/// SigmaOS Recovery Manager (Phase 3)
/// Inspired by Linux Mint's Timeshift and GRUB recovery entries.

use std::string::String;
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RecoveryMode { RescueShell, RollbackLast, SystemRepair, BootRepair }

#[derive(Debug, Clone)]
pub struct Snapshot { pub id: String, pub timestamp: u64, pub description: String, pub verified: bool }

pub struct RecoveryManager {
    pub snapshots: Vec<Snapshot>,
    pub boot_success_confirmed: bool,
    pub current_boot_snapshot_id: Option<String>,
    pub max_snapshots: usize,
}

impl RecoveryManager {
    pub fn new(max: usize) -> Self {
        Self { snapshots: Vec::new(), boot_success_confirmed: false, current_boot_snapshot_id: None, max_snapshots: max }
    }

    pub fn create_snapshot(&mut self, id: &str, ts: u64, desc: &str) {
        if self.snapshots.len() >= self.max_snapshots {
            self.snapshots.remove(0); // FIFO rotation
        }
        self.snapshots.push(Snapshot { id: id.into(), timestamp: ts, description: desc.into(), verified: true });
    }

    pub fn confirm_boot_success(&mut self) { self.boot_success_confirmed = true; }

    pub fn should_auto_rollback(&self) -> bool {
        !self.boot_success_confirmed && self.current_boot_snapshot_id.is_some()
    }

    pub fn rollback_to_last(&mut self) -> Result<&Snapshot, &'static str> {
        self.snapshots.last().ok_or("No snapshots available for rollback")
    }

    pub fn enter_recovery(&self, mode: RecoveryMode) -> String {
        match mode {
            RecoveryMode::RescueShell => "Launching rescue shell...".into(),
            RecoveryMode::RollbackLast => "Rolling back to last snapshot...".into(),
            RecoveryMode::SystemRepair => "Running filesystem repair...".into(),
            RecoveryMode::BootRepair => "Reinstalling bootloader...".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_lifecycle() {
        let mut rm = RecoveryManager::new(3);
        rm.create_snapshot("snap1", 1000, "Before update");
        rm.create_snapshot("snap2", 2000, "After update");
        assert_eq!(rm.snapshots.len(), 2);
        assert!(rm.rollback_to_last().unwrap().id == "snap2");
    }

    #[test]
    fn test_auto_rollback_detection() {
        let mut rm = RecoveryManager::new(5);
        rm.current_boot_snapshot_id = Some("snap1".into());
        assert!(rm.should_auto_rollback());
        rm.confirm_boot_success();
        assert!(!rm.should_auto_rollback());
    }

    #[test]
    fn test_snapshot_rotation() {
        let mut rm = RecoveryManager::new(2);
        rm.create_snapshot("s1", 1, "first");
        rm.create_snapshot("s2", 2, "second");
        rm.create_snapshot("s3", 3, "third");
        assert_eq!(rm.snapshots.len(), 2);
        assert_eq!(rm.snapshots[0].id, "s2");
    }
}
