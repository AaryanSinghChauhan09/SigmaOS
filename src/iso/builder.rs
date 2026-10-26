//! SigmaOS ISO Builder — Inspired by Omarchy ISO, Linux Mint ISO, Ubuntu casper.
//! Follows: Layered Design, Separation of Mechanism & Policy, Modularity, KISS.
//! Principles: Zero-Copy (mmap), Fail-Fast, Least Privilege, Memory Safety.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// ISO build profile variants inspired by real-world distro ISOs.
#[derive(Debug, Clone, PartialEq)]
pub enum IsoVariant {
    /// Full desktop ISO (like Linux Mint Cinnamon)
    Desktop,
    /// Minimal server ISO (no GUI, base utilities only)
    Minimal,
    /// Live-session only ISO (try-before-install, read-only squashfs)
    Live,
    /// Recovery ISO (rescue shell, fsck, bootloader repair)
    Recovery,
    /// Developer ISO (includes compilers, debuggers, dev tools)
    Developer,
}

/// Architecture targets (cross-compilation aware).
#[derive(Debug, Clone, PartialEq)]
pub enum TargetArch {
    X86_64,
    Aarch64,
    Riscv64,
    Arm32,
    LoongArch64,
}

impl TargetArch {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::X86_64 => "x86_64",
            Self::Aarch64 => "aarch64",
            Self::Riscv64 => "riscv64",
            Self::Arm32 => "armv7",
            Self::LoongArch64 => "loongarch64",
        }
    }
}

/// Bootloader type for the ISO (GRUB2, systemd-boot, syslinux, rEFInd).
#[derive(Debug, Clone, PartialEq)]
pub enum Bootloader {
    Grub2,
    SystemdBoot,
    Syslinux,
    Refind,
    M1n1, // Apple Silicon (inspired by omacom/m1n1)
}

/// SigmaOS ISO build configuration (inspired by omarchy-iso, mint-iso tools).
#[derive(Debug, Clone)]
pub struct IsoBuildConfig {
    /// Output ISO filename (e.g. "sigmaos-1.0-x86_64.iso")
    pub output_name: String,
    /// ISO volume label (max 32 chars, uppercase)
    pub volume_label: String,
    /// Target architecture
    pub arch: TargetArch,
    /// Variant (Desktop / Minimal / Live / Recovery / Developer)
    pub variant: IsoVariant,
    /// Bootloader to embed
    pub bootloader: Bootloader,
    /// Packages to include in squashfs filesystem
    pub packages: Vec<String>,
    /// Extra files to copy into ISO root (source → dest)
    pub extra_files: HashMap<PathBuf, PathBuf>,
    /// Whether to include live persistence support
    pub persistence: bool,
    /// Squashfs compression algorithm
    pub squashfs_comp: SquashfsCompression,
    /// EFI partition size in MiB (0 = auto)
    pub efi_partition_mb: u32,
    /// Whether to sign the ISO with a GPG key
    pub gpg_sign: bool,
}

/// Squashfs compression algorithms (Zero-Copy friendly algorithms first).
#[derive(Debug, Clone, PartialEq)]
pub enum SquashfsCompression {
    Zstd, // Best speed/ratio balance (recommended for SigmaOS)
    Lz4,  // Fastest decompression (for Live/Recovery)
    Xz,   // Highest compression (for Minimal/release builds)
    Lzo,  // Legacy compatibility
}

impl Default for IsoBuildConfig {
    fn default() -> Self {
        let mut packages = vec![
            "sigma-base".to_string(),
            "sigma-kernel".to_string(),
            "sigma-desktop".to_string(),
            "sigma-drivers".to_string(),
            "sigma-installer".to_string(),
            "sigma-welcome".to_string(),
        ];
        Self {
            output_name: "sigmaos-1.0-x86_64.iso".to_string(),
            volume_label: "SIGMAOS_1_0".to_string(),
            arch: TargetArch::X86_64,
            variant: IsoVariant::Desktop,
            bootloader: Bootloader::Grub2,
            packages,
            extra_files: HashMap::new(),
            persistence: true,
            squashfs_comp: SquashfsCompression::Zstd,
            efi_partition_mb: 512,
            gpg_sign: true,
        }
    }
}

/// ISO builder — orchestrates squashfs creation, bootloader integration, ISO 9660 packing.
/// Follows: Command-Query Separation (build() = command, verify() = query).
pub struct DistroIsoBuilder {
    config: IsoBuildConfig,
    work_dir: PathBuf,
    build_log: Vec<String>,
}

impl DistroIsoBuilder {
    pub fn new(config: IsoBuildConfig, work_dir: impl AsRef<Path>) -> Self {
        Self {
            config,
            work_dir: work_dir.as_ref().to_path_buf(),
            build_log: Vec::new(),
        }
    }

    fn log(&mut self, msg: impl Into<String>) {
        let entry = msg.into();
        eprintln!("[sigma-iso] {}", entry);
        self.build_log.push(entry);
    }

    /// Validate configuration before building (Fail-Fast principle).
    pub fn validate(&self) -> Result<(), IsoError> {
        if self.config.volume_label.len() > 32 {
            return Err(IsoError::InvalidConfig(
                "Volume label exceeds 32 characters".into(),
            ));
        }
        if self.config.packages.is_empty() {
            return Err(IsoError::InvalidConfig("Package list is empty".into()));
        }
        if self.config.output_name.is_empty() {
            return Err(IsoError::InvalidConfig(
                "Output name cannot be empty".into(),
            ));
        }
        Ok(())
    }

    /// Compute the approximate ISO size in MiB (query — no side effects).
    pub fn estimate_size_mb(&self) -> u32 {
        let base_mb = match self.config.variant {
            IsoVariant::Minimal => 512,
            IsoVariant::Live => 1024,
            IsoVariant::Desktop => 2048,
            IsoVariant::Developer => 3072,
            IsoVariant::Recovery => 768,
        };
        base_mb + self.config.efi_partition_mb
    }

    /// Return build plan as a structured list (query — no side effects).
    pub fn build_plan(&self) -> Vec<BuildStep> {
        vec![
            BuildStep {
                name: "validate-config".into(),
                estimated_ms: 10,
            },
            BuildStep {
                name: "prepare-work-dir".into(),
                estimated_ms: 50,
            },
            BuildStep {
                name: "install-packages".into(),
                estimated_ms: 60_000,
            },
            BuildStep {
                name: "strip-docs".into(),
                estimated_ms: 2_000,
            },
            BuildStep {
                name: "build-squashfs".into(),
                estimated_ms: 120_000,
            },
            BuildStep {
                name: "install-bootloader".into(),
                estimated_ms: 5_000,
            },
            BuildStep {
                name: "create-efi-image".into(),
                estimated_ms: 3_000,
            },
            BuildStep {
                name: "pack-iso9660".into(),
                estimated_ms: 30_000,
            },
            BuildStep {
                name: "compute-checksums".into(),
                estimated_ms: 5_000,
            },
            BuildStep {
                name: "gpg-sign".into(),
                estimated_ms: 2_000,
            },
        ]
    }

    /// Stub build entry point — real implementation calls out to system tools.
    /// Returns path to the built ISO on success.
    pub fn build(&mut self) -> Result<PathBuf, IsoError> {
        self.validate()?;
        self.log(format!("Starting ISO build: {}", self.config.output_name));
        self.log(format!(
            "Arch: {:?}  Variant: {:?}  Bootloader: {:?}",
            self.config.arch, self.config.variant, self.config.bootloader
        ));
        self.log(format!(
            "Packages: {} | Compression: {:?}",
            self.config.packages.len(),
            self.config.squashfs_comp
        ));

        for step in self.build_plan() {
            self.log(format!("  [{}] ~{}ms", step.name, step.estimated_ms));
        }

        let out = self.work_dir.join(&self.config.output_name);
        self.log(format!("ISO would be written to: {}", out.display()));
        self.log(format!("Estimated size: {} MiB", self.estimate_size_mb()));
        // In a full implementation: call mksquashfs, xorriso, grub-mkrescue, etc.
        Ok(out)
    }

    /// Return accumulated build log (Instrumentation principle).
    pub fn log_entries(&self) -> &[String] {
        &self.build_log
    }
}

/// A single step in the ISO build pipeline.
#[derive(Debug, Clone)]
pub struct BuildStep {
    pub name: String,
    pub estimated_ms: u64,
}

/// ISO build errors (Fail-Fast, Defensive Programming).
#[derive(Debug)]
pub enum IsoError {
    InvalidConfig(String),
    IoError(String),
    ToolNotFound(String),
    BuildFailed(String),
}

impl std::fmt::Display for IsoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidConfig(m) => write!(f, "invalid ISO config: {m}"),
            Self::IoError(m) => write!(f, "I/O error: {m}"),
            Self::ToolNotFound(m) => write!(f, "required tool not found: {m}"),
            Self::BuildFailed(m) => write!(f, "build failed: {m}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn default_builder() -> DistroIsoBuilder {
        DistroIsoBuilder::new(IsoBuildConfig::default(), "/tmp/sigma-iso-test")
    }

    #[test]
    fn test_validate_ok() {
        assert!(default_builder().validate().is_ok());
    }

    #[test]
    fn test_validate_empty_packages() {
        let mut cfg = IsoBuildConfig::default();
        cfg.packages.clear();
        let mut b = DistroIsoBuilder::new(cfg, "/tmp");
        assert!(matches!(b.validate(), Err(IsoError::InvalidConfig(_))));
    }

    #[test]
    fn test_validate_label_too_long() {
        let mut cfg = IsoBuildConfig::default();
        cfg.volume_label = "A".repeat(33);
        let b = DistroIsoBuilder::new(cfg, "/tmp");
        assert!(matches!(b.validate(), Err(IsoError::InvalidConfig(_))));
    }

    #[test]
    fn test_estimate_size_desktop() {
        let b = default_builder();
        assert!(b.estimate_size_mb() >= 2048);
    }

    #[test]
    fn test_build_plan_steps() {
        let b = default_builder();
        let plan = b.build_plan();
        assert!(!plan.is_empty());
        assert!(plan.iter().any(|s| s.name.contains("squashfs")));
        assert!(plan.iter().any(|s| s.name.contains("bootloader")));
    }

    #[test]
    fn test_arch_str() {
        assert_eq!(TargetArch::X86_64.as_str(), "x86_64");
        assert_eq!(TargetArch::Aarch64.as_str(), "aarch64");
        assert_eq!(TargetArch::Riscv64.as_str(), "riscv64");
    }

    #[test]
    fn test_build_returns_output_path() {
        let mut b = default_builder();
        let result = b.build();
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.to_str().unwrap().ends_with(".iso"));
        assert!(!b.log_entries().is_empty());
    }
}
