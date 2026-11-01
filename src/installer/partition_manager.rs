//! Guided disk partitioning for the SigmaOS installer.
//!
//! Detects available disks, builds a partition plan based on the user's
//! chosen scheme, validates minimum requirements, and executes the plan.
//! The `dry_run` function lets the UI preview what will happen before
//! committing to actual disk changes.

use std::path::PathBuf;

/// The bytes-per-gibibyte constant.
const GIB: u64 = 1024 * 1024 * 1024;
/// Minimum root partition size (20 GiB).
const MIN_ROOT_GIB: u64 = 20;
/// Minimum home partition size when separate (10 GiB).
const MIN_HOME_GIB: u64 = 10;
/// Fixed EFI system partition size (512 MiB).
const EFI_SIZE_BYTES: u64 = 512 * 1024 * 1024;

/// Partition layout schemes offered to the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionScheme {
    /// Single BTRFS root with subvolumes (`@` and `@home`).  Recommended default.
    SingleRootBtrfs,
    /// Single ext4 root, no separate home.
    SingleRootExt4,
    /// Separate `/home` partition on BTRFS.
    SeparateHomeBtrfs,
    /// Encrypted LVM with root + swap logical volumes.
    EncryptedLvm,
    /// User defines their own layout.
    Manual,
}

impl PartitionScheme {
    /// Human-readable description shown in the installer UI.
    pub fn description(self) -> &'static str {
        match self {
            Self::SingleRootBtrfs => "BTRFS root with @ and @home subvolumes (recommended)",
            Self::SingleRootExt4  => "Single ext4 root partition",
            Self::SeparateHomeBtrfs => "Separate /home partition on BTRFS",
            Self::EncryptedLvm    => "Encrypted LVM (LUKS2 + LVM)",
            Self::Manual          => "Manual partitioning",
        }
    }

    /// Whether this scheme uses a separate home partition.
    pub fn has_separate_home(self) -> bool {
        matches!(self, Self::SeparateHomeBtrfs | Self::EncryptedLvm | Self::Manual)
    }
}

/// Information about a physical disk detected on the system.
#[derive(Debug, Clone)]
pub struct DiskInfo {
    /// Block device path, e.g. `/dev/nvme0n1`.
    pub device_path: PathBuf,
    /// Total capacity in bytes.
    pub size_bytes: u64,
    /// Human-readable model name from the kernel.
    pub model: String,
    /// `true` if the device is an SSD or NVMe.
    pub is_ssd: bool,
    /// Existing partition entries (populated at detection time).
    pub existing_partitions: Vec<PartitionInfo>,
}

impl DiskInfo {
    /// Convenience: capacity in GiB (integer, rounded down).
    pub fn size_gib(&self) -> u64 {
        self.size_bytes / GIB
    }

    /// `true` if the disk has enough space for the minimum install.
    pub fn is_large_enough(&self) -> bool {
        self.size_bytes >= (MIN_ROOT_GIB + 1) * GIB + EFI_SIZE_BYTES
    }
}

/// A single existing partition on a disk.
#[derive(Debug, Clone)]
pub struct PartitionInfo {
    /// Partition number (1-based).
    pub number: u32,
    /// Partition size in bytes.
    pub size_bytes: u64,
    /// Detected filesystem, if any.
    pub filesystem: Option<String>,
    /// Mount point hint (e.g. `/boot/efi`).
    pub mount_hint: Option<String>,
}

/// A resolved partition plan ready to execute or preview.
#[derive(Debug, Clone)]
pub struct PartitionPlan {
    /// Target disk.
    pub device_path: PathBuf,
    /// Layout scheme.
    pub scheme: PartitionScheme,
    /// Root partition size in bytes.
    pub root_size_bytes: u64,
    /// Home partition size in bytes (0 if not separate).
    pub home_size_bytes: u64,
    /// Swap size in bytes (0 = no swap, >0 = swap partition or file).
    pub swap_size_bytes: u64,
    /// EFI system partition size in bytes.
    pub efi_size_bytes: u64,
    /// Whether to enable LUKS2 full-disk encryption.
    pub encrypt: bool,
}

impl PartitionPlan {
    /// Total bytes required by this plan.
    pub fn total_required(&self) -> u64 {
        self.root_size_bytes + self.home_size_bytes + self.swap_size_bytes + self.efi_size_bytes
    }
}

/// Human-readable preview of what the plan will do.
#[derive(Debug, Clone)]
pub struct PartitionPreview {
    /// List of operations that will be performed, in order.
    pub operations: Vec<String>,
    /// Warnings about data loss or other concerns.
    pub warnings: Vec<String>,
    /// Whether the plan will erase the entire disk.
    pub will_erase_disk: bool,
}

/// Where the installer will mount partitions after formatting.
#[derive(Debug, Clone)]
pub struct MountPoints {
    /// `/` — root filesystem.
    pub root: PathBuf,
    /// `/home` — home (may be a subvolume of root for BTRFS schemes).
    pub home: PathBuf,
    /// `/boot/efi` — EFI system partition.
    pub efi: PathBuf,
    /// `/boot` — kernel + initramfs.
    pub boot: PathBuf,
    /// Swap device path (empty if no swap).
    pub swap: Option<PathBuf>,
}

/// Errors from the partition manager.
#[derive(Debug, thiserror::Error)]
pub enum PartitionError {
    /// The target disk is smaller than the minimum requirement.
    #[error("disk too small: need {required_gib} GiB, have {available_gib} GiB")]
    DiskTooSmall { required_gib: u64, available_gib: u64 },
    /// Root partition is smaller than the minimum.
    #[error("root partition too small: minimum {MIN_ROOT_GIB} GiB")]
    RootTooSmall,
    /// Home partition is smaller than the minimum.
    #[error("home partition too small: minimum {MIN_HOME_GIB} GiB")]
    HomeTooSmall,
    /// No suitable disk was found.
    #[error("no suitable disk detected")]
    NoDiskFound,
    /// The chosen scheme requires features not available on this disk.
    #[error("scheme incompatible with disk: {0}")]
    IncompatibleScheme(String),
    /// The partition operation failed.
    #[error("partition operation failed: {0}")]
    OperationFailed(String),
}

/// The partition manager orchestrates disk detection, planning, and execution.
pub struct PartitionManager;

impl PartitionManager {
    /// Create a new partition manager.
    pub fn new() -> Self { Self }

    /// Detect available disks.  Returns simulated data in this build;
    /// production code reads `/sys/block` and `lsblk`.
    pub fn detect_disks() -> Result<Vec<DiskInfo>, PartitionError> {
        // Production: parse /sys/block and query udev.
        // This simulation returns a plausible NVMe disk.
        Ok(vec![
            DiskInfo {
                device_path: PathBuf::from("/dev/nvme0n1"),
                size_bytes: 512 * GIB,
                model: "Samsung SSD 980 500GB".to_string(),
                is_ssd: true,
                existing_partitions: vec![],
            },
        ])
    }

    /// Build a partition plan for the given disk and scheme.
    ///
    /// Allocates space proportionally:
    /// - EFI: 512 MiB (fixed)
    /// - Swap: equal to available RAM, capped at 32 GiB
    /// - Home: 40 % of remaining space (if separate)
    /// - Root: remainder
    pub fn create_plan(
        disk: &DiskInfo,
        scheme: PartitionScheme,
        encrypt: bool,
    ) -> PartitionPlan {
        let swap = 8 * GIB; // Simplified: 8 GiB swap
        let usable = disk.size_bytes.saturating_sub(EFI_SIZE_BYTES + swap);
        let (root, home) = if scheme.has_separate_home() {
            let h = usable * 40 / 100;
            let r = usable - h;
            (r, h)
        } else {
            (usable, 0)
        };

        PartitionPlan {
            device_path: disk.device_path.clone(),
            scheme,
            root_size_bytes: root,
            home_size_bytes: home,
            swap_size_bytes: swap,
            efi_size_bytes: EFI_SIZE_BYTES,
            encrypt,
        }
    }

    /// Validate a plan before executing it.
    pub fn validate_plan(plan: &PartitionPlan) -> Result<(), PartitionError> {
        if plan.root_size_bytes < MIN_ROOT_GIB * GIB {
            return Err(PartitionError::RootTooSmall);
        }
        if plan.scheme.has_separate_home() && plan.home_size_bytes < MIN_HOME_GIB * GIB {
            return Err(PartitionError::HomeTooSmall);
        }
        Ok(())
    }

    /// Generate a human-readable preview without touching the disk.
    pub fn dry_run(plan: &PartitionPlan) -> PartitionPreview {
        let mut ops = vec![
            format!(
                "Partition disk {} ({:.0} GiB)",
                plan.device_path.display(),
                plan.total_required() as f64 / GIB as f64,
            ),
            format!(
                "Create EFI partition: {} MiB, FAT32",
                plan.efi_size_bytes / (1024 * 1024),
            ),
            format!(
                "Create root partition: {:.0} GiB, {}",
                plan.root_size_bytes as f64 / GIB as f64,
                match plan.scheme {
                    PartitionScheme::SingleRootBtrfs | PartitionScheme::SeparateHomeBtrfs =>
                        "BTRFS",
                    PartitionScheme::EncryptedLvm => "LVM on LUKS2",
                    _ => "ext4",
                },
            ),
        ];
        if plan.home_size_bytes > 0 {
            ops.push(format!(
                "Create home partition: {:.0} GiB",
                plan.home_size_bytes as f64 / GIB as f64,
            ));
        }
        if plan.swap_size_bytes > 0 {
            ops.push(format!(
                "Create swap: {:.0} GiB",
                plan.swap_size_bytes as f64 / GIB as f64,
            ));
        }
        if plan.encrypt {
            ops.push("Encrypt root partition with LUKS2 (AES-256-XTS)".to_string());
        }

        let warnings = if plan.existing_partitions_on_disk() {
            vec!["WARNING: This will erase all data on the selected disk.".to_string()]
        } else {
            vec![]
        };

        PartitionPreview {
            will_erase_disk: true,
            warnings,
            operations: ops,
        }
    }

    /// Execute the partition plan.
    ///
    /// In production this calls `parted`, `mkfs.*`, and `mount`.
    /// Returns the mount points that the installer should use.
    pub fn execute(plan: &PartitionPlan) -> Result<MountPoints, PartitionError> {
        Self::validate_plan(plan)?;
        // Production code would run parted / sgdisk here.
        Ok(MountPoints {
            root: PathBuf::from("/mnt"),
            home: PathBuf::from("/mnt/home"),
            efi:  PathBuf::from("/mnt/boot/efi"),
            boot: PathBuf::from("/mnt/boot"),
            swap: if plan.swap_size_bytes > 0 {
                Some(PathBuf::from("/dev/nvme0n1p2"))
            } else {
                None
            },
        })
    }
}

// Helper for dry_run — not part of public API.
impl PartitionPlan {
    fn existing_partitions_on_disk(&self) -> bool {
        // Simplified: always warn about potential erasure.
        true
    }
}

impl Default for PartitionManager {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_plan_single_root_btrfs() {
        // Arrange
        let disk = DiskInfo {
            device_path: PathBuf::from("/dev/sda"),
            size_bytes: 256 * GIB,
            model: "Test SSD".to_string(),
            is_ssd: true,
            existing_partitions: vec![],
        };
        // Act
        let plan = PartitionManager::create_plan(&disk, PartitionScheme::SingleRootBtrfs, false);
        // Assert
        assert_eq!(plan.home_size_bytes, 0);
        assert!(plan.root_size_bytes >= MIN_ROOT_GIB * GIB);
    }

    #[test]
    fn test_validate_plan_rejects_undersized_root() {
        // Arrange
        let plan = PartitionPlan {
            device_path: PathBuf::from("/dev/sda"),
            scheme: PartitionScheme::SingleRootBtrfs,
            root_size_bytes: 10 * GIB, // too small
            home_size_bytes: 0,
            swap_size_bytes: 0,
            efi_size_bytes: EFI_SIZE_BYTES,
            encrypt: false,
        };
        // Act
        let result = PartitionManager::validate_plan(&plan);
        // Assert
        assert!(matches!(result, Err(PartitionError::RootTooSmall)));
    }

    #[test]
    fn test_validate_plan_rejects_undersized_home() {
        // Arrange
        let plan = PartitionPlan {
            device_path: PathBuf::from("/dev/sda"),
            scheme: PartitionScheme::SeparateHomeBtrfs,
            root_size_bytes: 30 * GIB,
            home_size_bytes: 5 * GIB, // too small
            swap_size_bytes: 4 * GIB,
            efi_size_bytes: EFI_SIZE_BYTES,
            encrypt: false,
        };
        // Act
        let result = PartitionManager::validate_plan(&plan);
        // Assert
        assert!(matches!(result, Err(PartitionError::HomeTooSmall)));
    }

    #[test]
    fn test_dry_run_produces_operations() {
        // Arrange
        let plan = PartitionPlan {
            device_path: PathBuf::from("/dev/nvme0n1"),
            scheme: PartitionScheme::SingleRootBtrfs,
            root_size_bytes: 200 * GIB,
            home_size_bytes: 0,
            swap_size_bytes: 8 * GIB,
            efi_size_bytes: EFI_SIZE_BYTES,
            encrypt: false,
        };
        // Act
        let preview = PartitionManager::dry_run(&plan);
        // Assert
        assert!(!preview.operations.is_empty());
        assert!(preview.operations.iter().any(|op| op.contains("EFI")));
        assert!(preview.operations.iter().any(|op| op.contains("root")));
    }

    #[test]
    fn test_execute_returns_mount_points() {
        // Arrange
        let plan = PartitionPlan {
            device_path: PathBuf::from("/dev/nvme0n1"),
            scheme: PartitionScheme::SingleRootBtrfs,
            root_size_bytes: 200 * GIB,
            home_size_bytes: 0,
            swap_size_bytes: 8 * GIB,
            efi_size_bytes: EFI_SIZE_BYTES,
            encrypt: false,
        };
        // Act
        let mounts = PartitionManager::execute(&plan).unwrap();
        // Assert
        assert_eq!(mounts.root, PathBuf::from("/mnt"));
        assert!(mounts.swap.is_some());
    }

    #[test]
    fn test_disk_size_gib() {
        // Arrange
        let disk = DiskInfo {
            device_path: PathBuf::from("/dev/sda"),
            size_bytes: 512 * GIB,
            model: "Test".to_string(),
            is_ssd: false,
            existing_partitions: vec![],
        };
        // Act + Assert
        assert_eq!(disk.size_gib(), 512);
    }

    #[test]
    fn test_separate_home_scheme_has_home() {
        assert!(PartitionScheme::SeparateHomeBtrfs.has_separate_home());
        assert!(!PartitionScheme::SingleRootBtrfs.has_separate_home());
    }
}
