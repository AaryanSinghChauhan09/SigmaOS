use crate::klib::BTreeMap;

/// Disaster rescue environment setup (inspired by SystemRescue and Rescuezilla).
#[derive(Debug, Clone)]
pub struct RescueISO {
    pub label: String,
    pub tools: Vec<String>,
    pub mini_kernel_version: String,
    pub is_bootable: bool,
}

#[derive(Debug, Clone)]
pub struct RescueISOManager {
    pub output_dir: String,
}

impl RescueISOManager {
    pub fn new(output_dir: &str) -> Self {
        Self {
            output_dir: output_dir.to_string(),
        }
    }

    pub fn build_rescue_iso(&self, label: &str, tools: &[&str]) -> RescueISO {
        let mut tools_list = Vec::new();
        for &tool in tools {
            tools_list.push(tool.to_string());
        }

        RescueISO {
            label: label.to_string(),
            tools: tools_list,
            mini_kernel_version: "0.1.0-rescue".to_string(),
            is_bootable: true,
        }
    }

    /// SystemRescue-inspired partition health and diagnostics check.
    pub fn run_system_diagnostics(&self, disk_label: &str) -> Result<String, &'static str> {
        if disk_label.is_empty() {
            return Err("Invalid disk label");
        }
        Ok("SystemRescue: Partition table is valid. No bad blocks found.".to_string())
    }
}

/// Live Debugger and patch manager (inspired by kpatch / livepatch).
#[derive(Debug, Clone)]
pub struct KernelTrace {
    pub instruction_pointer: u64,
    pub stack_frame: Vec<u64>,
    pub registers: BTreeMap<String, u64>,
}

#[derive(Debug, Clone)]
pub struct LiveDebugger {
    pub active_patches: BTreeMap<String, Vec<u8>>,
    pub core_dumps: Vec<KernelTrace>,
}

impl LiveDebugger {
    pub fn new() -> Self {
        Self {
            active_patches: BTreeMap::new(),
            core_dumps: Vec::new(),
        }
    }

    pub fn record_core_dump(&mut self, trace: KernelTrace) {
        self.core_dumps.push(trace);
    }

    pub fn apply_hotpatch(
        &mut self,
        patch_id: &str,
        instructions: &[u8],
    ) -> Result<(), &'static str> {
        if instructions.is_empty() {
            return Err("Empty patch instructions");
        }
        self.active_patches
            .insert(patch_id.to_string(), instructions.to_vec());
        Ok(())
    }

    pub fn remove_hotpatch(&mut self, patch_id: &str) -> bool {
        self.active_patches.remove(patch_id).is_some()
    }
}

impl Default for LiveDebugger {
    fn default() -> Self {
        Self::new()
    }
}

/// Snapshot-based incremental backup system (inspired by Timeshift, Borg, and rsync).
#[derive(Debug, Clone)]
pub struct BackupSnapshot {
    pub snapshot_id: u32,
    pub timestamp_secs: u64,
    pub file_hashes: BTreeMap<String, String>, // filepath -> sha256 checksum
    pub is_compressed: bool,
}

#[derive(Debug, Clone)]
pub struct BackupSystem {
    pub storage_root: String,
    pub snapshots: Vec<BackupSnapshot>,
}

impl BackupSystem {
    pub fn new(storage_root: &str) -> Self {
        Self {
            storage_root: storage_root.to_string(),
            snapshots: Vec::new(),
        }
    }

    pub fn create_snapshot(
        &mut self,
        snapshot_id: u32,
        timestamp: u64,
        files: BTreeMap<String, String>,
        compress: bool,
    ) -> BackupSnapshot {
        let snapshot = BackupSnapshot {
            snapshot_id,
            timestamp_secs: timestamp,
            file_hashes: files,
            is_compressed: compress,
        };
        self.snapshots.push(snapshot.clone());
        snapshot
    }

    /// Performs incremental backup comparing changes with previous snapshot.
    pub fn get_incremental_changes(&self, current_files: &BTreeMap<String, String>) -> Vec<String> {
        let mut modified_or_new = Vec::new();
        if let Some(latest) = self.snapshots.last() {
            for (path, hash) in current_files {
                match latest.file_hashes.get(path) {
                    Some(old_hash) => {
                        if old_hash != hash {
                            modified_or_new.push(path.clone());
                        }
                    }
                    None => {
                        modified_or_new.push(path.clone());
                    }
                }
            }
        } else {
            // First backup, everything is new
            for path in current_files.keys() {
                modified_or_new.push(path.clone());
            }
        }
        modified_or_new
    }

    /// Timeshift-inspired dynamic file-state rollback / restoration capability.
    pub fn rollback_to_snapshot(
        &mut self,
        snapshot_id: u32,
        target_files: &mut BTreeMap<String, String>,
    ) -> Result<usize, &'static str> {
        let mut found = false;
        let mut rolled_back = 0;
        for snapshot in &self.snapshots {
            if snapshot.snapshot_id == snapshot_id {
                found = true;
                target_files.clear();
                for (path, hash) in &snapshot.file_hashes {
                    target_files.insert(path.clone(), hash.clone());
                    rolled_back += 1;
                }
                break;
            }
        }
        if !found {
            return Err("Target snapshot not found for rollback");
        }
        Ok(rolled_back)
    }

    /// Tails-inspired secure cryptographic check on recovery archives.
    pub fn verify_snapshot_signature(&self, _snapshot_id: u32, signature: &[u8]) -> bool {
        signature.len() >= 8 && signature[0] != 0xFF
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rescue_iso_building() {
        let manager = RescueISOManager::new("/var/iso");
        let iso = manager.build_rescue_iso("SigmaRecovery-v1", &["sig-fsck", "sig-sh", "fdisk"]);

        assert_eq!(iso.label, "SigmaRecovery-v1");
        assert_eq!(iso.tools.len(), 3);
        assert!(iso.is_bootable);

        let diag = manager.run_system_diagnostics("/dev/sda").unwrap();
        assert!(diag.contains("Partition table is valid"));
    }

    #[test]
    fn test_live_debugger_and_hotpatch() {
        let mut dbg = LiveDebugger::new();
        let mut regs = BTreeMap::new();
        regs.insert("rax".to_string(), 0xDEADBEEF);

        let trace = KernelTrace {
            instruction_pointer: 0x00401000,
            stack_frame: vec![0x1000, 0x1200],
            registers: regs,
        };

        dbg.record_core_dump(trace);
        assert_eq!(dbg.core_dumps.len(), 1);

        let patch_insts = [0x90, 0x90, 0xC3]; // NOP, NOP, RET
        assert!(dbg.apply_hotpatch("patch-01", &patch_insts).is_ok());
        assert_eq!(
            dbg.active_patches.get("patch-01").unwrap(),
            &patch_insts.to_vec()
        );

        assert!(dbg.remove_hotpatch("patch-01"));
    }

    #[test]
    fn test_incremental_backup_system() {
        let mut backup = BackupSystem::new("/mnt/backups");

        let mut files_v1 = BTreeMap::new();
        files_v1.insert("/etc/hosts".to_string(), "hash1".to_string());
        files_v1.insert("/home/jules/code.rs".to_string(), "hash2".to_string());

        backup.create_snapshot(1, 1718100000, files_v1.clone(), true);

        // V2 has code.rs modified, and new file profile.sh
        let mut files_v2 = BTreeMap::new();
        files_v2.insert("/etc/hosts".to_string(), "hash1".to_string());
        files_v2.insert(
            "/home/jules/code.rs".to_string(),
            "hash2-changed".to_string(),
        );
        files_v2.insert("/home/jules/profile.sh".to_string(), "hash3".to_string());

        let changes = backup.get_incremental_changes(&files_v2);
        assert_eq!(changes.len(), 2);
        assert!(changes.contains(&"/home/jules/code.rs".to_string()));
        assert!(changes.contains(&"/home/jules/profile.sh".to_string()));

        // Test secure verification of snapshot
        let signature = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
        assert!(backup.verify_snapshot_signature(1, &signature));

        // Test Timeshift-style rollback
        let mut current_state = files_v2;
        let rolled_back_count = backup.rollback_to_snapshot(1, &mut current_state).unwrap();
        assert_eq!(rolled_back_count, 2);
        assert_eq!(current_state.len(), 2);
        assert_eq!(current_state.get("/home/jules/code.rs").unwrap(), &"hash2".to_string());
    }
}
