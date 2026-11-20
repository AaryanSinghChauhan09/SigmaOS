// installer/tests/install_dry_run.rs
// Non-destructive installer dry-run validation test suite

#[derive(Debug, PartialEq, Eq)]
pub enum InstallMode {
    DryRun,
    LiveInstall,
}

#[derive(Debug)]
pub struct PartitionTarget {
    pub mount_point: String,
    pub fstype: String,
    pub size_mb: u64,
}

#[derive(Debug)]
pub struct DryRunInstallerConfig {
    pub target_disk: String,
    pub partitions: Vec<PartitionTarget>,
    pub mode: InstallMode,
}

pub struct DryRunInstallerEngine {
    pub config: DryRunInstallerConfig,
    pub simulated_ops: Vec<String>,
    pub modified_disks: Vec<String>,
}

impl DryRunInstallerEngine {
    pub fn new(config: DryRunInstallerConfig) -> Self {
        Self {
            config,
            simulated_ops: Vec::new(),
            modified_disks: Vec::new(),
        }
    }

    pub fn execute(&mut self) -> Result<(), &'static str> {
        if self.config.mode == InstallMode::DryRun {
            self.simulated_ops.push(format!("DRY_RUN: Inspect partition layout on {}", self.config.target_disk));
            for part in &self.config.partitions {
                self.simulated_ops.push(format!(
                    "DRY_RUN: Validate filesystem creation for {} ({}, {}MB)",
                    part.mount_point, part.fstype, part.size_mb
                ));
            }
            self.simulated_ops.push("DRY_RUN: Bootloader EFI staging check passed".to_string());
            Ok(())
        } else {
            self.modified_disks.push(self.config.target_disk.clone());
            Ok(())
        }
    }
}

#[test]
fn test_install_dry_run_non_destructive_invariant() {
    let config = DryRunInstallerConfig {
        target_disk: "/dev/nvme0n1".to_string(),
        partitions: vec![
            PartitionTarget {
                mount_point: "/boot/efi".to_string(),
                fstype: "vfat".to_string(),
                size_mb: 512,
            },
            PartitionTarget {
                mount_point: "/".to_string(),
                fstype: "ext4".to_string(),
                size_mb: 32768,
            },
        ],
        mode: InstallMode::DryRun,
    };

    let mut engine = DryRunInstallerEngine::new(config);
    let status = engine.execute();

    assert!(status.is_ok(), "Installer dry run should return Ok");
    assert!(
        engine.modified_disks.is_empty(),
        "Dry run must not modify disk state"
    );
    assert_eq!(engine.simulated_ops.len(), 4);
}
