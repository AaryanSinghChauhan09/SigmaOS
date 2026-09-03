// SigmaOS Universal Package Snapshot & Rollback Engine
// Inspired by NixOS generations, openSUSE Snapper Btrfs transaction snapshots, and FreeBSD pkg(8) state recovery
// - Generation-based package database state snapshotting
// - Atomic package rollbacks across version updates, removals, and configuration drifts
// - Snapshot diffing (added, removed, upgraded, or downgraded packages)
// - Automated generation pruning with retention policies

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstalledPackageRecord {
    pub name: String,
    pub version: String,
    pub config_checksum: String,
    pub installed_files: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageSnapshotState {
    pub generation_id: u32,
    pub description: String,
    pub timestamp: u64,
    pub packages: BTreeMap<String, InstalledPackageRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageSnapshotDiff {
    pub added_packages: Vec<String>,
    pub removed_packages: Vec<String>,
    pub modified_packages: Vec<(String, String, String)>, // (name, old_ver, new_ver)
}

/// Sovereign Universal Package Snapshot & Rollback Engine
#[derive(Debug, Clone)]
pub struct SovereignPackageSnapshotRollbackEngine {
    pub current_generation: u32,
    pub snapshots: BTreeMap<u32, PackageSnapshotState>,
    pub active_installed_packages: BTreeMap<String, InstalledPackageRecord>,
}

impl SovereignPackageSnapshotRollbackEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            current_generation: 0,
            snapshots: BTreeMap::new(),
            active_installed_packages: BTreeMap::new(),
        };

        // Seed initial base system packages
        engine.install_package_record(
            "sigma-base",
            "1.0.0",
            "chk_base_v1",
            vec!["/bin/init".to_string()],
        );
        engine.install_package_record(
            "sigma-libc",
            "2.38.0",
            "chk_libc_v1",
            vec!["/lib/libc.so".to_string()],
        );
        engine.create_snapshot("Initial System Installation Base");
        engine
    }

    pub fn install_package_record(
        &mut self,
        name: &str,
        version: &str,
        config_checksum: &str,
        installed_files: Vec<String>,
    ) {
        let record = InstalledPackageRecord {
            name: name.to_string(),
            version: version.to_string(),
            config_checksum: config_checksum.to_string(),
            installed_files,
        };
        self.active_installed_packages
            .insert(name.to_string(), record);
    }

    pub fn remove_package_record(&mut self, name: &str) -> bool {
        self.active_installed_packages.remove(name).is_some()
    }

    pub fn create_snapshot(&mut self, description: &str) -> u32 {
        self.current_generation += 1;
        let gen_id = self.current_generation;

        let snapshot = PackageSnapshotState {
            generation_id: gen_id,
            description: description.to_string(),
            timestamp: gen_id as u64 * 1000,
            packages: self.active_installed_packages.clone(),
        };

        self.snapshots.insert(gen_id, snapshot);
        gen_id
    }

    pub fn create_pre_update_snapshot(&mut self, target_pkg: &str) -> u32 {
        let desc = format!("Timeshift/Snapper Pre-Update Snapshot prior to updating {}", target_pkg);
        self.create_snapshot(&desc)
    }

    pub fn rollback_to_snapshot(&mut self, target_generation: u32) -> Result<(), &'static str> {
        let target_snap = self
            .snapshots
            .get(&target_generation)
            .ok_or("Target package generation snapshot not found")?
            .clone();

        self.active_installed_packages = target_snap.packages;
        self.create_snapshot(&format!("Rollback to Generation {}", target_generation));
        Ok(())
    }

    pub fn diff_snapshots(
        &self,
        gen_a: u32,
        gen_b: u32,
    ) -> Result<PackageSnapshotDiff, &'static str> {
        let snap_a = self.snapshots.get(&gen_a).ok_or("Snapshot A not found")?;
        let snap_b = self.snapshots.get(&gen_b).ok_or("Snapshot B not found")?;

        let mut added = Vec::new();
        let mut removed = Vec::new();
        let mut modified = Vec::new();

        for (name, rec_b) in &snap_b.packages {
            if let Some(rec_a) = snap_a.packages.get(name) {
                if rec_a.version != rec_b.version || rec_a.config_checksum != rec_b.config_checksum
                {
                    modified.push((name.clone(), rec_a.version.clone(), rec_b.version.clone()));
                }
            } else {
                added.push(name.clone());
            }
        }

        for name in snap_a.packages.keys() {
            if !snap_b.packages.contains_key(name) {
                removed.push(name.clone());
            }
        }

        Ok(PackageSnapshotDiff {
            added_packages: added,
            removed_packages: removed,
            modified_packages: modified,
        })
    }

    pub fn prune_snapshots(&mut self, retain_count: usize) -> usize {
        if self.snapshots.len() <= retain_count {
            return 0;
        }

        let keys_to_remove: Vec<u32> = self
            .snapshots
            .keys()
            .cloned()
            .take(self.snapshots.len() - retain_count)
            .collect();

        let removed = keys_to_remove.len();
        for k in keys_to_remove {
            self.snapshots.remove(&k);
        }
        removed
    }

    pub fn active_package_count(&self) -> usize {
        self.active_installed_packages.len()
    }
}

impl Default for SovereignPackageSnapshotRollbackEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_snapshot_creation_and_rollback() {
        let mut engine = SovereignPackageSnapshotRollbackEngine::new();
        assert_eq!(engine.current_generation, 1);

        // Install new package and take generation 2 snapshot
        engine.install_package_record(
            "curl",
            "8.2.1",
            "chk_curl_v1",
            vec!["/bin/curl".to_string()],
        );
        let gen2 = engine.create_snapshot("Installed curl 8.2.1");
        assert_eq!(gen2, 2);
        assert!(engine.active_installed_packages.contains_key("curl"));

        // Rollback to generation 1
        engine.rollback_to_snapshot(1).unwrap();
        assert!(!engine.active_installed_packages.contains_key("curl"));
    }

    #[test]
    fn test_snapshot_diffing() {
        let mut engine = SovereignPackageSnapshotRollbackEngine::new(); // Gen 1
        engine.install_package_record("zsh", "5.9", "chk_zsh", vec!["/bin/zsh".to_string()]);
        let gen2 = engine.create_snapshot("Installed zsh");

        let diff = engine.diff_snapshots(1, gen2).unwrap();
        assert_eq!(diff.added_packages, vec!["zsh".to_string()]);
        assert_eq!(diff.removed_packages.len(), 0);
    }

    #[test]
    fn test_generation_pruning() {
        let mut engine = SovereignPackageSnapshotRollbackEngine::new(); // Gen 1
        engine.create_snapshot("Gen 2");
        engine.create_snapshot("Gen 3");
        engine.create_snapshot("Gen 4");

        assert_eq!(engine.snapshots.len(), 4);
        let pruned = engine.prune_snapshots(2);
        assert_eq!(pruned, 2);
        assert_eq!(engine.snapshots.len(), 2);
    }

    #[test]
    fn test_active_package_count() {
        let mut engine = SovereignPackageSnapshotRollbackEngine::new();
        engine.install_package_record("bash", "5.2", "chk1", vec!["/bin/bash".to_string()]);
        // `new()` seeds sigma-base and sigma-libc, so bash makes three.
        assert_eq!(engine.active_package_count(), 3);
    }
}
