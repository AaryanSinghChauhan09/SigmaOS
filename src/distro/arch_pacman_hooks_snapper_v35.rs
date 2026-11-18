// SPDX-License-Identifier: MIT OR Apache-2.0
//! SigmaOS ALPM Transaction Hooks & Snapper Btrfs Snapshot Engine
//!
//! Inspired by Arch Linux's libalpm post-transaction hooks (snap-pac / pacman-hook)
//! and openSUSE / Omarchy automated Btrfs pre/post snapshot mechanisms.
//! Provides ALPM hook target declaration, Snapper snapshot description generation,
//! and automatic rollback metadata management.

use std::fmt;

/// Type of ALPM hook operation trigger
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookOperation {
    Install,
    Upgrade,
    Remove,
}

impl fmt::Display for HookOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Install => write!(f, "Install"),
            Self::Upgrade => write!(f, "Upgrade"),
            Self::Remove => write!(f, "Remove"),
        }
    }
}

/// Execution phase for ALPM hook
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookWhen {
    PreTransaction,
    PostTransaction,
}

impl fmt::Display for HookWhen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PreTransaction => write!(f, "PreTransaction"),
            Self::PostTransaction => write!(f, "PostTransaction"),
        }
    }
}

/// An ALPM transaction hook specification
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlpmHook {
    pub name: String,
    pub description: String,
    pub when: HookWhen,
    pub operations: Vec<HookOperation>,
    pub target_pattern: String,
    pub exec: String,
}

impl AlpmHook {
    pub fn new_snapper_pre(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: "Create pre-transaction Btrfs snapshot".to_string(),
            when: HookWhen::PreTransaction,
            operations: vec![HookOperation::Install, HookOperation::Upgrade, HookOperation::Remove],
            target_pattern: "*".to_string(),
            exec: "/usr/bin/snapper-timeline pre".to_string(),
        }
    }

    pub fn new_snapper_post(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: "Create post-transaction Btrfs snapshot and calculate diff".to_string(),
            when: HookWhen::PostTransaction,
            operations: vec![HookOperation::Install, HookOperation::Upgrade, HookOperation::Remove],
            target_pattern: "*".to_string(),
            exec: "/usr/bin/snapper-timeline post".to_string(),
        }
    }

    /// Render to pacman hook file format (.hook)
    pub fn to_hook_file(&self) -> String {
        let mut out = String::new();
        out.push_str("[Trigger]\n");
        for op in &self.operations {
            out.push_str(&format!("Operation = {}\n", op));
        }
        out.push_str("Type = Package\n");
        out.push_str(&format!("Target = {}\n\n", self.target_pattern));

        out.push_str("[Action]\n");
        out.push_str(&format!("Description = {}\n", self.description));
        out.push_str(&format!("When = {}\n", self.when));
        out.push_str(&format!("Exec = {}\n", self.exec));
        out
    }
}

/// Snapper Snapshot metadata descriptor
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapperSnapshot {
    pub id: u64,
    pub snapshot_type: String, // "pre", "post", "single"
    pub pre_id: Option<u64>,
    pub timestamp: String,
    pub description: String,
    pub cleanup_algorithm: String, // "number", "timeline"
}

impl SnapperSnapshot {
    pub fn new_pre(id: u64, desc: &str, timestamp: &str) -> Self {
        Self {
            id,
            snapshot_type: "pre".to_string(),
            pre_id: None,
            timestamp: timestamp.to_string(),
            description: desc.to_string(),
            cleanup_algorithm: "number".to_string(),
        }
    }

    pub fn new_post(id: u64, pre_id: u64, desc: &str, timestamp: &str) -> Self {
        Self {
            id,
            snapshot_type: "post".to_string(),
            pre_id: Some(pre_id),
            timestamp: timestamp.to_string(),
            description: desc.to_string(),
            cleanup_algorithm: "number".to_string(),
        }
    }

    /// Generate snapper command to create this snapshot
    pub fn create_command(&self) -> String {
        if self.snapshot_type == "post" {
            format!(
                "snapper -c root create -t post --pre-number {} -d \"{}\" -c {}",
                self.pre_id.unwrap_or(0),
                self.description,
                self.cleanup_algorithm
            )
        } else {
            format!(
                "snapper -c root create -t {} -d \"{}\" -c {}",
                self.snapshot_type, self.description, self.cleanup_algorithm
            )
        }
    }

    /// Generate rollback command
    pub fn rollback_command(&self) -> String {
        format!("snapper -c root rollback {}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alpm_hook_rendering_aaa() {
        let hook = AlpmHook::new_snapper_pre("00-snapper-pre.hook");
        let rendered = hook.to_hook_file();

        assert!(rendered.contains("[Trigger]"));
        assert!(rendered.contains("Operation = Install"));
        assert!(rendered.contains("Operation = Upgrade"));
        assert!(rendered.contains("When = PreTransaction"));
        assert!(rendered.contains("Exec = /usr/bin/snapper-timeline pre"));
    }

    #[test]
    fn test_snapper_pre_snapshot_command_aaa() {
        let snap = SnapperSnapshot::new_pre(101, "pacman -Syu rolling upgrade", "2026-11-18T13:45:00Z");
        let cmd = snap.create_command();

        assert_eq!(cmd, "snapper -c root create -t pre -d \"pacman -Syu rolling upgrade\" -c number");
    }

    #[test]
    fn test_snapper_post_snapshot_command_aaa() {
        let snap = SnapperSnapshot::new_post(102, 101, "pacman -Syu complete", "2026-11-18T13:46:00Z");
        let cmd = snap.create_command();

        assert_eq!(cmd, "snapper -c root create -t post --pre-number 101 -d \"pacman -Syu complete\" -c number");
    }

    #[test]
    fn test_snapper_rollback_command_aaa() {
        let snap = SnapperSnapshot::new_pre(101, "before bad update", "2026-11-18T13:00:00Z");
        let cmd = snap.rollback_command();

        assert_eq!(cmd, "snapper -c root rollback 101");
    }
}
