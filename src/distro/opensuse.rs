use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// openSUSE Tumbleweed / MicroOS Read-Only RootFS Snapshot State
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MicroOsSnapshotState {
    Active,
    PendingReboot,
    Staged,
    Discarded,
}

/// Represents an openSUSE MicroOS Btrfs / S-FS Transactional Snapshot
#[derive(Debug, Clone)]
pub struct MicroOsSnapshot {
    pub id: u64,
    pub description: String,
    pub state: MicroOsSnapshotState,
    pub is_read_only: bool,
    pub staged_packages: Vec<String>,
}

/// openSUSE MicroOS Transactional Update Engine (`transactional-update` CLI parity)
#[derive(Debug, Clone)]
pub struct OpenSuseMicroOsEngine {
    pub current_active_snapshot_id: u64,
    pub snapshots: BTreeMap<u64, MicroOsSnapshot>,
    pub next_snapshot_id: u64,
    pub automatic_reboot_enabled: bool,
}

impl OpenSuseMicroOsEngine {
    pub fn new() -> Self {
        let mut snapshots = BTreeMap::new();
        snapshots.insert(
            1,
            MicroOsSnapshot {
                id: 1,
                description: String::from("Initial MicroOS Base RootFS"),
                state: MicroOsSnapshotState::Active,
                is_read_only: true,
                staged_packages: Vec::new(),
            },
        );

        Self {
            current_active_snapshot_id: 1,
            snapshots,
            next_snapshot_id: 2,
            automatic_reboot_enabled: false,
        }
    }

    /// Creates a writeable target snapshot and stages package additions/updates (`transactional-update pkg install`)
    pub fn apply_transactional_update(
        &mut self,
        description: &str,
        packages: &[&str],
    ) -> Result<u64, &'static str> {
        if packages.is_empty() {
            return Err("No packages provided for transactional update");
        }

        let new_id = self.next_snapshot_id;
        self.next_snapshot_id += 1;

        let snapshot = MicroOsSnapshot {
            id: new_id,
            description: description.to_string(),
            state: MicroOsSnapshotState::PendingReboot,
            is_read_only: true,
            staged_packages: packages.iter().map(|p| p.to_string()).collect(),
        };

        self.snapshots.insert(new_id, snapshot);
        Ok(new_id)
    }

    /// Triggers reboot into the newly staged snapshot (`transactional-update reboot`)
    pub fn reboot_into_new_snapshot(&mut self, target_id: u64) -> Result<String, &'static str> {
        if !self.snapshots.contains_key(&target_id) {
            return Err("Target snapshot ID not found");
        }

        if self.snapshots.get(&target_id).unwrap().state != MicroOsSnapshotState::PendingReboot {
            return Err("Snapshot is not in PendingReboot state");
        }

        // Deactivate old active snapshot
        let old_id = self.current_active_snapshot_id;
        if let Some(old_snap) = self.snapshots.get_mut(&old_id) {
            old_snap.state = MicroOsSnapshotState::Staged;
        }

        let desc = {
            let target_snap = self.snapshots.get_mut(&target_id).unwrap();
            target_snap.state = MicroOsSnapshotState::Active;
            target_snap.description.clone()
        };
        self.current_active_snapshot_id = target_id;

        Ok(format!(
            "Successfully rebooted into openSUSE MicroOS Snapshot {} ('{}')",
            target_id, desc
        ))
    }

    /// Rolls back system state to a target historical snapshot (`transactional-update rollback <ID>`)
    pub fn rollback_snapshot(&mut self, target_id: u64) -> Result<String, &'static str> {
        if !self.snapshots.contains_key(&target_id) {
            return Err("Target rollback snapshot does not exist");
        }

        if let Some(old_snap) = self.snapshots.get_mut(&self.current_active_snapshot_id) {
            old_snap.state = MicroOsSnapshotState::Staged;
        }

        let target_snap = self.snapshots.get_mut(&target_id).unwrap();
        target_snap.state = MicroOsSnapshotState::Active;
        self.current_active_snapshot_id = target_id;

        Ok(format!(
            "Rolled back openSUSE MicroOS rootfs to snapshot {}",
            target_id
        ))
    }
}

/// openSUSE YaST Pattern Group Definition
#[derive(Debug, Clone)]
pub struct YastPattern {
    pub name: String,
    pub category: String,
    pub summary: String,
    pub included_packages: Vec<String>,
    pub is_installed: bool,
}

/// openSUSE YaST Pattern & Meta-Package Manager
#[derive(Debug, Clone)]
pub struct OpenSuseYastPatternEngine {
    pub patterns: BTreeMap<String, YastPattern>,
}

impl OpenSuseYastPatternEngine {
    pub fn new() -> Self {
        let mut patterns = BTreeMap::new();

        patterns.insert(
            "patterns-microos-base".to_string(),
            YastPattern {
                name: "patterns-microos-base".to_string(),
                category: "Base System".to_string(),
                summary: "openSUSE MicroOS Transactional Base System".to_string(),
                included_packages: vec![
                    "kernel-default".to_string(),
                    "transactional-update".to_string(),
                    "health-checker".to_string(),
                    "btrfsprogs".to_string(),
                ],
                is_installed: true,
            },
        );

        patterns.insert(
            "patterns-gnome-desktop".to_string(),
            YastPattern {
                name: "patterns-gnome-desktop".to_string(),
                category: "Graphical Environments".to_string(),
                summary: "openSUSE GNOME Desktop Pattern".to_string(),
                included_packages: vec![
                    "gnome-shell".to_string(),
                    "nautilus".to_string(),
                    "gdm".to_string(),
                    "pipewire".to_string(),
                ],
                is_installed: false,
            },
        );

        patterns.insert(
            "patterns-devel-base".to_string(),
            YastPattern {
                name: "patterns-devel-base".to_string(),
                category: "Development".to_string(),
                summary: "openSUSE Base Development Tools".to_string(),
                included_packages: vec![
                    "gcc".to_string(),
                    "make".to_string(),
                    "gdb".to_string(),
                    "git".to_string(),
                ],
                is_installed: false,
            },
        );

        Self { patterns }
    }

    /// Installs a YaST pattern group and resolves nested packages
    pub fn install_pattern(&mut self, pattern_name: &str) -> Result<Vec<String>, &'static str> {
        let pattern = self
            .patterns
            .get_mut(pattern_name)
            .ok_or("Pattern not found in openSUSE pattern registry")?;

        if pattern.is_installed {
            return Err("YaST pattern is already installed");
        }

        pattern.is_installed = true;
        Ok(pattern.included_packages.clone())
    }

    /// Verifies pattern package installation integrity
    pub fn verify_pattern_integrity(&self, pattern_name: &str) -> Result<bool, &'static str> {
        let pattern = self
            .patterns
            .get(pattern_name)
            .ok_or("Pattern not found in registry")?;
        Ok(pattern.is_installed)
    }

    /// Returns all available openSUSE patterns
    pub fn list_available_patterns(&self) -> Vec<(&str, &str, bool)> {
        self.patterns
            .values()
            .map(|p| (p.name.as_str(), p.summary.as_str(), p.is_installed))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opensuse_microos_transactional_update() {
        let mut engine = OpenSuseMicroOsEngine::new();
        assert_eq!(engine.current_active_snapshot_id, 1);

        let snap_id = engine
            .apply_transactional_update("Update kernel & glibc", &["kernel-default", "glibc"])
            .unwrap();
        assert_eq!(snap_id, 2);

        let reboot_msg = engine.reboot_into_new_snapshot(snap_id).unwrap();
        assert!(reboot_msg.contains("Successfully rebooted"));
        assert_eq!(engine.current_active_snapshot_id, 2);

        let rollback_msg = engine.rollback_snapshot(1).unwrap();
        assert!(rollback_msg.contains("Rolled back"));
        assert_eq!(engine.current_active_snapshot_id, 1);
    }

    #[test]
    fn test_opensuse_yast_patterns() {
        let mut pattern_engine = OpenSuseYastPatternEngine::new();
        let avail = pattern_engine.list_available_patterns();
        assert!(avail.len() >= 3);

        let installed_pkgs = pattern_engine
            .install_pattern("patterns-devel-base")
            .unwrap();
        assert!(installed_pkgs.contains(&"gcc".to_string()));
        assert!(pattern_engine
            .verify_pattern_integrity("patterns-devel-base")
            .unwrap());
    }
}
