// SPDX-License-Identifier: MIT OR GPL-2.0
//! SigmaOS ISO Architecture Module
//! Inspired by: Linux Mint ISO, Omarchy 4.0.6, Arch Linux archiso, Debian live-build,
//!              FreeBSD bsdinstall, OpenBSD autoinstall, ReactOS, DragonFlyBSD
//!
//! Principles applied:
//! - ARCHITECTURE: Clean layered ISO structure
//! - ABSTRACTION: IsoLayer trait abstracts boot/rootfs/data concerns
//! - SEPARATION OF CONCERNS: Boot, installer, live-system, package layers separated
//! - MODULARITY: Each ISO component is an independent, composable unit
//! - SINGLE RESPONSIBILITY: Each struct/trait does one thing
//! - GALL'S LAW: Start simple, evolve incrementally
//! - BOOTLOADER INTEGRATION: GRUB2/Limine/syslinux support
//! - ZERO-COPY: squashfs overlay via overlayfs (no unnecessary copies)
//! - MEMORY DISCIPLINE: tmpfs sizing, ramdisk management
//! - PORTABILITY: x86_64, aarch64, riscv64 target selection
//! - DEFENSIVE PROGRAMMING: All operations validate before execute
//! - FAIL-FAST: Early error detection in build pipeline
//! - OPEN STANDARD & INTEROPERABILITY: ISO 9660 El Torito / UEFI ESP

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// ISO 9660 compliant volume ID (≤32 chars, uppercase, no spaces)
pub const SIGMA_ISO_VOLUME_ID: &str = "SIGMAOS_DESKTOP_1_0";

/// Supported boot firmware targets (PORTABILITY principle)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirmwareTarget {
    /// UEFI 64-bit (EFI System Partition with BOOTX64.EFI)
    UefiX64,
    /// UEFI 32-bit (arm/riscv EFI)
    UefiAarch64,
    /// Legacy BIOS (El Torito boot catalog)
    BiosLegacy,
    /// Hybrid BIOS + UEFI (recommended for maximum compatibility)
    HybridBiosUefi,
}

/// Architecture target (cross-compilation support)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchTarget {
    X86_64,
    Aarch64,
    Riscv64,
}

impl ArchTarget {
    pub fn as_str(self) -> &'static str {
        match self {
            ArchTarget::X86_64 => "x86_64",
            ArchTarget::Aarch64 => "aarch64",
            ArchTarget::Riscv64 => "riscv64",
        }
    }

    pub fn efi_filename(self) -> &'static str {
        match self {
            ArchTarget::X86_64 => "BOOTX64.EFI",
            ArchTarget::Aarch64 => "BOOTAA64.EFI",
            ArchTarget::Riscv64 => "BOOTRISCV64.EFI",
        }
    }
}

/// Squashfs compression algorithm (CACHE CONSCIOUS DESIGN)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SquashfsCompression {
    Zstd, // Best for modern systems (fast decompress, good ratio)
    Lz4,  // Fastest decompression (cache-friendly, warm boot)
    Xz,   // Best ratio (slow decompress, good for archival ISOs)
    Gzip, // Maximum compatibility fallback
}

impl SquashfsCompression {
    pub fn as_str(self) -> &'static str {
        match self {
            SquashfsCompression::Zstd => "zstd",
            SquashfsCompression::Lz4 => "lz4",
            SquashfsCompression::Xz => "xz",
            SquashfsCompression::Gzip => "gzip",
        }
    }
}

/// ISO layer abstraction (ABSTRACTION + LAYERED DESIGN + INTERFACE SEGREGATION)
pub trait IsoLayer {
    /// Layer name for logging/display
    fn name(&self) -> &str;
    /// Validate layer prerequisites (DEFENSIVE PROGRAMMING)
    fn validate(&self) -> Result<(), IsoError>;
    /// Estimated uncompressed size in bytes
    fn estimated_size_bytes(&self) -> u64;
    /// Layer description for documentation
    fn description(&self) -> &str;
}

/// ISO build error (FAIL-FAST system)
#[derive(Debug, Clone)]
pub enum IsoError {
    MissingBootloader(String),
    MissingKernel(String),
    InvalidProfile(String),
    FilesystemError(String),
    UnsupportedArch(String),
    InsufficientSpace { needed: u64, available: u64 },
    CompressionFailed(String),
    SigningFailed(String),
    ChecksumMismatch { expected: String, actual: String },
}

impl IsoError {
    pub fn as_str(&self) -> String {
        match self {
            IsoError::MissingBootloader(b) => format!("Missing bootloader: {}", b),
            IsoError::MissingKernel(k) => format!("Missing kernel: {}", k),
            IsoError::InvalidProfile(p) => format!("Invalid profile: {}", p),
            IsoError::FilesystemError(e) => format!("Filesystem error: {}", e),
            IsoError::UnsupportedArch(a) => format!("Unsupported arch: {}", a),
            IsoError::InsufficientSpace { needed, available } => {
                format!(
                    "Insufficient space: need {}MB, have {}MB",
                    needed / 1_048_576,
                    available / 1_048_576
                )
            }
            IsoError::CompressionFailed(e) => format!("Compression failed: {}", e),
            IsoError::SigningFailed(e) => format!("Signing failed: {}", e),
            IsoError::ChecksumMismatch { expected, actual } => {
                format!("Checksum mismatch: expected {}, got {}", expected, actual)
            }
        }
    }
}

/// Boot layer: GRUB2 + Limine + syslinux configuration
/// (BOOTLOADER INTEGRATION + SEPARATION OF MECHANISM & POLICY)
pub struct BootLayer {
    pub firmware: FirmwareTarget,
    pub arch: ArchTarget,
    pub timeout_secs: u32,
    pub splash_enabled: bool,
    pub kernel_cmdline: String,
}

impl IsoLayer for BootLayer {
    fn name(&self) -> &str {
        "boot"
    }

    fn validate(&self) -> Result<(), IsoError> {
        if self.timeout_secs > 60 {
            return Err(IsoError::InvalidProfile(
                "Boot timeout > 60s is unreasonable".to_string(),
            ));
        }
        Ok(())
    }

    fn estimated_size_bytes(&self) -> u64 {
        // GRUB2 ~5MB, Limine ~1MB, EFI partition ~50MB
        56 * 1_048_576
    }

    fn description(&self) -> &str {
        "Bootloader layer: GRUB2/Limine/syslinux + EFI/BIOS support"
    }
}

impl BootLayer {
    pub fn new(arch: ArchTarget) -> Self {
        Self {
            firmware: FirmwareTarget::HybridBiosUefi,
            arch,
            timeout_secs: 10,
            splash_enabled: true,
            kernel_cmdline: "quiet splash loglevel=3 systemd.show_status=false".to_string(),
        }
    }

    /// Generate GRUB2 configuration (Linux Mint / Omarchy style)
    pub fn generate_grub_cfg(&self) -> String {
        format!(
            r#"set default=0
set timeout={timeout}

if loadfont /boot/grub/fonts/unicode.pf2; then
  set gfxmode=auto
  insmod all_video
  insmod gfxterm
  insmod png
  terminal_output gfxterm
fi

if background_image /boot/grub/splash.png; then
  set color_normal=light-gray/black
  set color_highlight=magenta/black
else
  set menu_color_normal=cyan/blue
  set menu_color_highlight=white/blue
fi

menuentry "SigmaOS Desktop (Live)" --class sigmaos {{
  set gfxpayload=keep
  linux /boot/vmlinuz {cmdline} boot=live
  initrd /boot/initrd.img
}}

menuentry "SigmaOS Desktop (Safe Mode)" --class sigmaos {{
  set gfxpayload=keep
  linux /boot/vmlinuz {cmdline} nomodeset systemd.unit=multi-user.target
  initrd /boot/initrd.img
}}

menuentry "SigmaOS Installer" --class sigmaos {{
  set gfxpayload=keep
  linux /boot/vmlinuz {cmdline} boot=installer
  initrd /boot/initrd.img
}}

menuentry "Memory Test (memtest86+)" --class memtest {{
  linux16 /boot/memtest86+.bin
}}

menuentry "Boot from next device" {{
  boot
}}
"#,
            timeout = self.timeout_secs,
            cmdline = self.kernel_cmdline,
        )
    }

    /// Generate Limine bootloader configuration (modern UEFI alternative)
    pub fn generate_limine_conf(&self) -> String {
        format!(
            r#"# SigmaOS Limine Bootloader Configuration
# Inspired by Arch Linux limine-cd.cfg

TIMEOUT={timeout}
VERBOSE=no
GRAPHICS=yes

/SigmaOS Desktop (Live)
    PROTOCOL=linux
    KERNEL_PATH=boot:///boot/vmlinuz
    CMDLINE={cmdline} boot=live
    MODULE_PATH=boot:///boot/initrd.img
    RESOLUTION=auto

/SigmaOS Installer
    PROTOCOL=linux
    KERNEL_PATH=boot:///boot/vmlinuz
    CMDLINE={cmdline} boot=installer
    MODULE_PATH=boot:///boot/initrd.img
    RESOLUTION=auto

/SigmaOS Safe Mode
    PROTOCOL=linux
    KERNEL_PATH=boot:///boot/vmlinuz
    CMDLINE={cmdline} nomodeset
    MODULE_PATH=boot:///boot/initrd.img
"#,
            timeout = self.timeout_secs,
            cmdline = self.kernel_cmdline,
        )
    }
}

/// Live filesystem layer (squashfs rootfs, overlayfs RAM layer)
/// (VIRTUAL MEMORY ABSTRACTION + ZERO-COPY + MEMORY DISCIPLINE)
pub struct LiveFilesystemLayer {
    pub compression: SquashfsCompression,
    pub block_size: u32,    // squashfs block size (128K default)
    pub tmpfs_size_pct: u8, // % of RAM for overlayfs write layer (50% default)
    pub persistence_enabled: bool,
    pub overlay_backend: OverlayBackend,
}

/// Overlay filesystem backend (Debian live / Arch mkinitcpio style)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlayBackend {
    Overlayfs, // Linux kernel overlayfs (modern, recommended)
    Aufs,      // Legacy Debian live-boot (compatibility)
    Tmpfs,     // Pure RAM (no persistence, fastest boot)
}

impl IsoLayer for LiveFilesystemLayer {
    fn name(&self) -> &str {
        "live-filesystem"
    }

    fn validate(&self) -> Result<(), IsoError> {
        if self.block_size < 4096 || self.block_size > 1_048_576 {
            return Err(IsoError::InvalidProfile(
                "squashfs block_size must be 4K–1M".to_string(),
            ));
        }
        if self.tmpfs_size_pct > 75 {
            return Err(IsoError::InvalidProfile(
                "tmpfs size > 75% RAM may cause OOM on low-memory systems".to_string(),
            ));
        }
        Ok(())
    }

    fn estimated_size_bytes(&self) -> u64 {
        // Typical SigmaOS rootfs: ~3GB uncompressed → ~900MB with zstd
        3_000 * 1_048_576
    }

    fn description(&self) -> &str {
        "Live squashfs filesystem with overlayfs RAM write layer"
    }
}

impl Default for LiveFilesystemLayer {
    fn default() -> Self {
        Self {
            compression: SquashfsCompression::Zstd,
            block_size: 131_072, // 128K: optimal for sequential reads
            tmpfs_size_pct: 50,
            persistence_enabled: false,
            overlay_backend: OverlayBackend::Overlayfs,
        }
    }
}

/// Installer layer: Calamares / archinstall / custom TUI
/// (PLUG-IN DRIVER MODEL + SEPARATION OF MECHANISM & POLICY)
pub struct InstallerLayer {
    pub installer_type: InstallerType,
    pub unattended_config: Option<UnattendedConfig>,
    pub partition_strategies: Vec<PartitionStrategy>,
    pub supported_filesystems: Vec<TargetFilesystem>,
    pub encryption_support: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallerType {
    CalamaresGui,        // Friendly GUI installer (Linux Mint style)
    ArchinstallTui,      // TUI JSON-driven (Arch Linux style)
    SigmaTuiNative,      // Native Rust TUI installer (SigmaOS own)
    UnattendedAutomatic, // Cloud-init / autoinstall (Server automation)
}

/// Unattended installation config (inspired by Ubuntu autoinstall + Alpine setup-alpine)
#[derive(Debug, Clone)]
pub struct UnattendedConfig {
    pub target_disk: String,
    pub hostname: String,
    pub username: String,
    pub locale: String,
    pub timezone: String,
    pub keyboard_layout: String,
    pub filesystem: TargetFilesystem,
    pub swap_size_mb: u64,
    pub encrypt_root: bool,
    pub install_bootloader: bool,
    pub extra_packages: Vec<String>,
    pub post_install_hooks: Vec<String>,
}

impl Default for UnattendedConfig {
    fn default() -> Self {
        Self {
            target_disk: "/dev/sda".to_string(),
            hostname: "sigmaos".to_string(),
            username: "sigma".to_string(),
            locale: "en_US.UTF-8".to_string(),
            timezone: "UTC".to_string(),
            keyboard_layout: "us".to_string(),
            filesystem: TargetFilesystem::Btrfs,
            swap_size_mb: 4096,
            encrypt_root: false,
            install_bootloader: true,
            extra_packages: Vec::new(),
            post_install_hooks: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetFilesystem {
    Btrfs,   // Recommended: snapshots, subvolumes, compression
    Ext4,    // Compatibility default
    Xfs,     // High-throughput (RHEL/Fedora style)
    Zfs,     // Enterprise: CoW, dedup (FreeBSD style)
    F2fs,    // SSD/NVMe optimized (Android/ChromeOS inspired)
    Hammer2, // DragonFlyBSD transactional FS
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionStrategy {
    WholeDisc,          // Erase and use entire disk
    SideBySide,         // Shrink and dual-boot
    ManualPartitioning, // Expert: user configures manually
    LvmEncrypted,       // LVM on LUKS (SECURITY)
    Zfs,                // ZFS zroot pool (FreeBSD style)
    BtrfsSubvolumes,    // Btrfs @root @home @snapshots (Arch/openSUSE style)
}

impl IsoLayer for InstallerLayer {
    fn name(&self) -> &str {
        "installer"
    }

    fn validate(&self) -> Result<(), IsoError> {
        if self.partition_strategies.is_empty() {
            return Err(IsoError::InvalidProfile(
                "At least one partition strategy required".to_string(),
            ));
        }
        Ok(())
    }

    fn estimated_size_bytes(&self) -> u64 {
        // Calamares ~30MB, archinstall ~5MB, config files ~1MB
        36 * 1_048_576
    }

    fn description(&self) -> &str {
        "Installer layer: Calamares/TUI installer with partition strategies"
    }
}

/// Package manifest layer: packages included in the ISO squashfs
/// (MODULARITY + COMPOSITION OVER INHERITANCE + SOFTWARE SUPPORT PROGRAM)
#[derive(Debug, Clone)]
pub struct PackageManifestLayer {
    pub essential_packages: Vec<String>,
    pub desktop_packages: Vec<String>,
    pub media_packages: Vec<String>,
    pub dev_tools: Vec<String>,
    pub security_packages: Vec<String>,
    pub locale_packages: Vec<String>,
}

impl Default for PackageManifestLayer {
    fn default() -> Self {
        Self {
            essential_packages: vec![
                "systemd".to_string(),
                "dbus".to_string(),
                "udev".to_string(),
                "network-manager".to_string(),
                "openssh".to_string(),
                "sudo".to_string(),
                "bash".to_string(),
                "zsh".to_string(),
                "coreutils".to_string(),
                "util-linux".to_string(),
                "procps".to_string(),
                "findutils".to_string(),
            ],
            desktop_packages: vec![
                "wayland".to_string(),
                "weston".to_string(),
                "hyprland".to_string(),
                "waybar".to_string(),
                "wofi".to_string(),
                "foot".to_string(),
                "pipewire".to_string(),
                "wireplumber".to_string(),
                "xdg-utils".to_string(),
                "xdg-user-dirs".to_string(),
            ],
            media_packages: vec![
                "ffmpeg".to_string(),
                "gstreamer".to_string(),
                "vlc".to_string(),
                "mpv".to_string(),
            ],
            dev_tools: vec![
                "rustup".to_string(),
                "gcc".to_string(),
                "make".to_string(),
                "git".to_string(),
                "python3".to_string(),
            ],
            security_packages: vec![
                "apparmor".to_string(),
                "nftables".to_string(),
                "gnupg".to_string(),
                "age".to_string(),
            ],
            locale_packages: vec!["locales".to_string(), "tzdata".to_string()],
        }
    }
}

impl IsoLayer for PackageManifestLayer {
    fn name(&self) -> &str {
        "package-manifest"
    }

    fn validate(&self) -> Result<(), IsoError> {
        if self.essential_packages.is_empty() {
            return Err(IsoError::InvalidProfile(
                "essential_packages cannot be empty".to_string(),
            ));
        }
        Ok(())
    }

    fn estimated_size_bytes(&self) -> u64 {
        // Rough estimate: avg 20MB per package
        let total = self.essential_packages.len()
            + self.desktop_packages.len()
            + self.media_packages.len()
            + self.dev_tools.len()
            + self.security_packages.len()
            + self.locale_packages.len();
        (total as u64) * 20 * 1_048_576
    }

    fn description(&self) -> &str {
        "Package manifest: all packages included in the live ISO squashfs"
    }
}

/// Signing and verification layer (SECURE DEFAULTS + MEMORY SAFETY)
/// Inspired by: Debian apt-key, Arch Linux pacman-key, FreeBSD pkg sign
#[derive(Debug, Clone)]
pub struct SigningLayer {
    pub gpg_key_id: Option<String>,
    pub sha256_checksums: bool,
    pub sha512_checksums: bool,
    pub sbom_enabled: bool,   // Software Bill of Materials (SLSA compliance)
    pub cosign_enabled: bool, // Sigstore cosign for OCI/ISO attestation
}

impl Default for SigningLayer {
    fn default() -> Self {
        Self {
            gpg_key_id: None,
            sha256_checksums: true,
            sha512_checksums: true,
            sbom_enabled: true,
            cosign_enabled: false,
        }
    }
}

impl IsoLayer for SigningLayer {
    fn name(&self) -> &str {
        "signing"
    }

    fn validate(&self) -> Result<(), IsoError> {
        if !self.sha256_checksums && !self.sha512_checksums {
            return Err(IsoError::InvalidProfile(
                "At least one checksum algorithm required for integrity".to_string(),
            ));
        }
        Ok(())
    }

    fn estimated_size_bytes(&self) -> u64 {
        4096
    } // Just checksum files

    fn description(&self) -> &str {
        "Signing layer: GPG, SHA256/512 checksums, SBOM generation"
    }
}

/// Complete ISO build profile (COMPOSITION OVER INHERITANCE)
/// This is the top-level config that drives the ISO builder
pub struct SigmaIsoProfile {
    pub name: String,
    pub version: String,
    pub arch: ArchTarget,
    pub boot: BootLayer,
    pub live_fs: LiveFilesystemLayer,
    pub installer: InstallerLayer,
    pub packages: PackageManifestLayer,
    pub signing: SigningLayer,
    pub output_path: String,
}

impl SigmaIsoProfile {
    /// Create a standard desktop ISO profile (KEEP IT SIMPLE + SECURE DEFAULTS)
    pub fn desktop_standard(arch: ArchTarget) -> Self {
        Self {
            name: "SigmaOS Desktop".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            arch,
            boot: BootLayer::new(arch),
            live_fs: LiveFilesystemLayer::default(),
            installer: InstallerLayer {
                installer_type: InstallerType::CalamaresGui,
                unattended_config: None,
                partition_strategies: vec![
                    PartitionStrategy::WholeDisc,
                    PartitionStrategy::BtrfsSubvolumes,
                    PartitionStrategy::LvmEncrypted,
                    PartitionStrategy::ManualPartitioning,
                ],
                supported_filesystems: vec![
                    TargetFilesystem::Btrfs,
                    TargetFilesystem::Ext4,
                    TargetFilesystem::Xfs,
                ],
                encryption_support: true,
            },
            packages: PackageManifestLayer::default(),
            signing: SigningLayer::default(),
            output_path: format!("target/sigmaos-desktop-{}.iso", arch.as_str()),
        }
    }

    /// Create a minimal server ISO profile (MINIMAL KERNEL PHILOSOPHY)
    pub fn server_minimal(arch: ArchTarget) -> Self {
        let mut profile = Self::desktop_standard(arch);
        profile.name = "SigmaOS Server".to_string();
        profile.boot.splash_enabled = false;
        profile.boot.timeout_secs = 3;
        profile.boot.kernel_cmdline = "quiet loglevel=3 net.ifnames=0 biosdevname=0".to_string();
        profile.live_fs.compression = SquashfsCompression::Xz; // Best ratio for server
        profile.installer.installer_type = InstallerType::UnattendedAutomatic;
        profile.output_path = format!("target/sigmaos-server-{}.iso", arch.as_str());
        profile
    }

    /// Validate entire profile (DEFENSIVE PROGRAMMING + FAIL-FAST)
    pub fn validate(&self) -> Result<(), Vec<IsoError>> {
        let mut errors = Vec::new();
        if let Err(e) = self.boot.validate() {
            errors.push(e);
        }
        if let Err(e) = self.live_fs.validate() {
            errors.push(e);
        }
        if let Err(e) = self.installer.validate() {
            errors.push(e);
        }
        if let Err(e) = self.packages.validate() {
            errors.push(e);
        }
        if let Err(e) = self.signing.validate() {
            errors.push(e);
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Estimate total ISO size in bytes
    pub fn estimated_iso_size_bytes(&self) -> u64 {
        self.boot.estimated_size_bytes()
            + self.live_fs.estimated_size_bytes()
            + self.installer.estimated_size_bytes()
            + self.signing.estimated_size_bytes()
    }
}

/// ISO update delta mechanism (SOFTWARE UPDATING PROGRAM JUST LIKE LINUX DISTROS)
/// Inspired by: apt delta-updates, pacman's delta support, casync
pub struct IsoDeltaUpdater {
    pub current_version: String,
    pub target_version: String,
    pub delta_url: String,
    pub signature_url: String,
    pub checksum: String,
}

impl IsoDeltaUpdater {
    pub fn new(current: &str, target: &str, base_url: &str) -> Self {
        Self {
            current_version: current.to_string(),
            target_version: target.to_string(),
            delta_url: format!("{}/{}-to-{}.delta", base_url, current, target),
            signature_url: format!("{}/{}-to-{}.delta.asc", base_url, current, target),
            checksum: String::new(),
        }
    }

    /// Generate casync-compatible delta manifest (PRISTINE SOURCES principle)
    pub fn generate_manifest_entry(&self) -> String {
        format!(
            "FROM:{} TO:{} DELTA:{} SIG:{} SHA256:{}",
            self.current_version,
            self.target_version,
            self.delta_url,
            self.signature_url,
            self.checksum,
        )
    }
}

/// Software Support Program — LTS channel management
/// (SOFTWARE SUPPORT PROGRAM JUST LIKE LINUX DISTROS)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportChannel {
    /// Long-Term Support: 5 years security updates (Ubuntu LTS style)
    Lts,
    /// Standard release: 9-month support window (Ubuntu interim style)  
    Standard,
    /// Rolling release: always latest (Arch/Omarchy style)
    Rolling,
    /// Edge / Nightly: bleeding edge, no stability guarantee
    Edge,
}

impl SupportChannel {
    pub fn support_duration_months(self) -> Option<u32> {
        match self {
            SupportChannel::Lts => Some(60),
            SupportChannel::Standard => Some(9),
            SupportChannel::Rolling => None, // Indefinite
            SupportChannel::Edge => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            SupportChannel::Lts => "lts",
            SupportChannel::Standard => "standard",
            SupportChannel::Rolling => "rolling",
            SupportChannel::Edge => "edge",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_desktop_profile_validates() {
        let profile = SigmaIsoProfile::desktop_standard(ArchTarget::X86_64);
        assert!(profile.validate().is_ok());
    }

    #[test]
    fn test_server_profile_validates() {
        let profile = SigmaIsoProfile::server_minimal(ArchTarget::X86_64);
        assert!(profile.validate().is_ok());
    }

    #[test]
    fn test_grub_cfg_generation() {
        let boot = BootLayer::new(ArchTarget::X86_64);
        let cfg = boot.generate_grub_cfg();
        assert!(cfg.contains("SigmaOS Desktop (Live)"));
        assert!(cfg.contains("SigmaOS Installer"));
        assert!(cfg.contains("SigmaOS Desktop (Safe Mode)"));
    }

    #[test]
    fn test_limine_conf_generation() {
        let boot = BootLayer::new(ArchTarget::X86_64);
        let conf = boot.generate_limine_conf();
        assert!(conf.contains("TIMEOUT="));
        assert!(conf.contains("/SigmaOS Desktop (Live)"));
    }

    #[test]
    fn test_arch_target_efi_filenames() {
        assert_eq!(ArchTarget::X86_64.efi_filename(), "BOOTX64.EFI");
        assert_eq!(ArchTarget::Aarch64.efi_filename(), "BOOTAA64.EFI");
        assert_eq!(ArchTarget::Riscv64.efi_filename(), "BOOTRISCV64.EFI");
    }

    #[test]
    fn test_delta_updater_manifest() {
        let updater = IsoDeltaUpdater::new("1.0", "1.1", "https://updates.sigmaos.dev");
        let manifest = updater.generate_manifest_entry();
        assert!(manifest.contains("FROM:1.0"));
        assert!(manifest.contains("TO:1.1"));
    }

    #[test]
    fn test_lts_channel_duration() {
        assert_eq!(SupportChannel::Lts.support_duration_months(), Some(60));
        assert_eq!(SupportChannel::Rolling.support_duration_months(), None);
    }

    #[test]
    fn test_squashfs_compression_str() {
        assert_eq!(SquashfsCompression::Zstd.as_str(), "zstd");
        assert_eq!(SquashfsCompression::Lz4.as_str(), "lz4");
    }

    #[test]
    fn test_invalid_boot_timeout() {
        let mut boot = BootLayer::new(ArchTarget::X86_64);
        boot.timeout_secs = 100; // > 60, invalid
        assert!(boot.validate().is_err());
    }

    #[test]
    fn test_invalid_tmpfs_pct() {
        let mut lfs = LiveFilesystemLayer::default();
        lfs.tmpfs_size_pct = 80; // > 75%, invalid
        assert!(lfs.validate().is_err());
    }
}
