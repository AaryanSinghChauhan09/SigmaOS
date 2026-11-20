// installer/tests/dry_run.rs
// Standalone integration test for non-destructive installer dry-run mode

#[derive(Debug, PartialEq, Eq)]
pub enum InstallerMode {
    DryRun,
    LiveInstall,
}

#[derive(Debug)]
pub struct PartitionSpec {
    pub mount_point: String,
    pub filesystem: String,
    pub size_mb: u64,
}

#[derive(Debug)]
pub struct InstallerPlan {
    pub target_disk: String,
    pub partitions: Vec<PartitionSpec>,
    pub mode: InstallerMode,
}

pub struct InstallerEngine {
    pub plan: InstallerPlan,
    pub executed_commands: Vec<String>,
    pub mutated_disks: Vec<String>,
}

impl InstallerEngine {
    pub fn new(plan: InstallerPlan) -> Self {
        Self {
            plan,
            executed_commands: Vec::new(),
            mutated_disks: Vec::new(),
        }
    }

    pub fn execute(&mut self) -> Result<(), &'static str> {
        if self.plan.mode == InstallerMode::DryRun {
            self.executed_commands.push(format!("SIMULATED: Format disk {}", self.plan.target_disk));
            for part in &self.plan.partitions {
                self.executed_commands.push(format!(
                    "SIMULATED: Create partition {} ({}, {}MB)",
                    part.mount_point, part.filesystem, part.size_mb
                ));
            }
            self.executed_commands.push("SIMULATED: Install GRUB2 bootloader".to_string());
            // In dry run, NO disk mutation should occur
            Ok(())
        } else {
            self.mutated_disks.push(self.plan.target_disk.clone());
            self.executed_commands.push(format!("FORMAT {}", self.plan.target_disk));
            Ok(())
        }
    }
}

#[test]
fn test_installer_dry_run_safety() {
    let plan = InstallerPlan {
        target_disk: "/dev/sda".to_string(),
        partitions: vec![
            PartitionSpec {
                mount_point: "/boot".to_string(),
                filesystem: "fat32".to_string(),
                size_mb: 512,
            },
            PartitionSpec {
                mount_point: "/".to_string(),
                filesystem: "ext4".to_string(),
                size_mb: 20480,
            },
        ],
        mode: InstallerMode::DryRun,
    };

    let mut engine = InstallerEngine::new(plan);
    let result = engine.execute();

    assert!(result.is_ok(), "Dry run execution failed");
    assert!(
        engine.mutated_disks.is_empty(),
        "CRITICAL: Dry run mutated disks! Violates non-destructive safety rule."
    );
    assert_eq!(engine.executed_commands.len(), 4, "Expected 4 simulated commands");
    println!("Installer dry run safety test passed cleanly.");
}
