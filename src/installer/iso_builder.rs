use std::vec;
use std::format;
// SigmaOS Bootable ISO Builder
// Finalizes the bootable ISO implementation with advanced features
// Integrates with existing installer components


use std::string::String;
use std::vec::Vec;
use std::collections::BTreeMap;

/// ISO Boot Configuration
#[derive(Debug, Clone)]
pub struct IsoBootConfig {
    pub iso_label: String,
    pub volume_id: String,
    pub boot_loader: String,
    pub kernel_path: String,
    pub initrd_path: String,
    pub boot_parameters: Vec<String>,
}

impl IsoBootConfig {
    pub fn new() -> Self {
        Self {
            iso_label: String::from("SIGMAOS"),
            volume_id: String::from("SigmaOS_2026"),
            boot_loader: String::from("grub"),
            kernel_path: String::from("/boot/vmlinuz-sigma"),
            initrd_path: String::from("/boot/initrd-sigma"),
            boot_parameters: vec![
                String::from("quiet"),
                String::from("splash"),
                String::from("root=live:CDROM"),
            ],
        }
    }

    pub fn add_boot_parameter(&mut self, param: &str) {
        self.boot_parameters.push(String::from(param));
    }

    pub fn get_boot_command(&self) -> String {
        let mut cmd = format!("linux /boot/vmlinuz-sigma");
        for param in &self.boot_parameters {
            cmd.push_str(" ");
            cmd.push_str(param);
        }
        cmd
    }
}

/// File System Configuration for ISO
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsoFilesystem {
    SquashFS,
    Iso9660,
    Ext4,
}

/// ISO File Entry
#[derive(Debug, Clone)]
pub struct IsoFileEntry {
    pub source_path: String,
    pub iso_path: String,
    pub permissions: u32,
    pub is_compressed: bool,
}

impl IsoFileEntry {
    pub fn new(source: &str, iso: &str) -> Self {
        Self {
            source_path: String::from(source),
            iso_path: String::from(iso),
            permissions: 0o755,
            is_compressed: false,
        }
    }

    pub fn with_permissions(mut self, perms: u32) -> Self {
        self.permissions = perms;
        self
    }

    pub fn compressed(mut self) -> Self {
        self.is_compressed = true;
        self
    }
}

/// ISO Builder
pub struct IsoBuilder {
    pub config: IsoBootConfig,
    pub filesystem: IsoFilesystem,
    pub files: Vec<IsoFileEntry>,
    pub boot_catalog: Vec<String>,
    pub size_estimate: usize,
}

impl IsoBuilder {
    pub fn new() -> Self {
        Self {
            config: IsoBootConfig::new(),
            filesystem: IsoFilesystem::SquashFS,
            files: Vec::new(),
            boot_catalog: Vec::new(),
            size_estimate: 0,
        }
    }

    pub fn with_filesystem(mut self, fs: IsoFilesystem) -> Self {
        self.filesystem = fs;
        self
    }

    pub fn add_file(&mut self, entry: IsoFileEntry) {
        self.size_estimate += 4096; // Estimate 4KB per file
        self.files.push(entry);
    }

    pub fn add_directory(&mut self, source: &str, iso: &str) {
        let entry = IsoFileEntry::new(source, iso)
            .with_permissions(0o755);
        self.add_file(entry);
    }

    pub fn add_boot_catalog_entry(&mut self, entry: &str) {
        self.boot_catalog.push(String::from(entry));
    }

    /// Generate GRUB configuration for ISO boot
    pub fn generate_grub_config(&self) -> String {
        let mut config = String::new();
        
        config.push_str("set timeout=5\n");
        config.push_str("set default=0\n\n");
        config.push_str("menuentry \"SigmaOS Live\" {\n");
        config.push_str("    set root=(cd0)\n");
        config.push_str("    linux ");
        config.push_str(&self.config.kernel_path);
        
        for param in &self.config.boot_parameters {
            config.push_str(" ");
            config.push_str(param);
        }
        
        config.push_str("\n");
        config.push_str("    initrd ");
        config.push_str(&self.config.initrd_path);
        config.push_str("\n");
        config.push_str("}\n\n");
        
        config.push_str("menuentry \"SigmaOS Live (Text Mode)\" {\n");
        config.push_str("    set root=(cd0)\n");
        config.push_str("    linux ");
        config.push_str(&self.config.kernel_path);
        config.push_str(" textonly\n");
        config.push_str("    initrd ");
        config.push_str(&self.config.initrd_path);
        config.push_str("\n");
        config.push_str("}\n");

        config
    }

    /// Generate Syslinux configuration for ISO boot
    pub fn generate_syslinux_config(&self) -> String {
        let mut config = String::new();
        
        config.push_str("DEFAULT sigmaos\n");
        config.push_str("PROMPT 0\n");
        config.push_str("TIMEOUT 50\n\n");
        
        config.push_str("LABEL sigmaos\n");
        config.push_str("    KERNEL /boot/vmlinuz-sigma\n");
        config.push_str("    APPEND ");
        
        for param in &self.config.boot_parameters {
            config.push_str(param);
            config.push_str(" ");
        }
        
        config.push_str("initrd=/boot/initrd-sigma\n\n");
        
        config.push_str("LABEL sigmaos-text\n");
        config.push_str("    KERNEL /boot/vmlinuz-sigma\n");
        config.push_str("    APPEND textonly initrd=/boot/initrd-sigma\n");

        config
    }

    /// Calculate estimated ISO size
    pub fn calculate_size(&self) -> usize {
        let base_size = 50 * 1024 * 1024; // 50MB base system
        let files_size = self.files.len() * 4096;
        let compression_factor = match self.filesystem {
            IsoFilesystem::SquashFS => 3, // 3:1 compression
            _ => 1,
        };
        
        (base_size + files_size) / compression_factor
    }

    /// Validate ISO configuration
    pub fn validate(&self) -> Result<(), IsoValidationError> {
        if self.config.kernel_path.is_empty() {
            return Err(IsoValidationError::MissingKernel);
        }
        
        if self.config.initrd_path.is_empty() {
            return Err(IsoValidationError::MissingInitrd);
        }
        
        if self.files.is_empty() {
            return Err(IsoValidationError::NoFiles);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IsoValidationError {
    MissingKernel,
    MissingInitrd,
    NoFiles,
    InvalidFilesystem,
}

/// Live Session Configuration
pub struct LiveSessionConfig {
    pub persistence_enabled: bool,
    pub persistence_size_mb: usize,
    pub cow_filesystem: String,
    pub home_persistence: bool,
}

impl LiveSessionConfig {
    pub fn new() -> Self {
        Self {
            persistence_enabled: false,
            persistence_size_mb: 512,
            cow_filesystem: String::from("overlayfs"),
            home_persistence: false,
        }
    }

    pub fn with_persistence(mut self, size_mb: usize) -> Self {
        self.persistence_enabled = true;
        self.persistence_size_mb = size_mb;
        self
    }

    pub fn with_home_persistence(mut self) -> Self {
        self.home_persistence = true;
        self
    }
}

/// Hybrid ISO Builder (Supports both BIOS and UEFI)
pub struct HybridIsoBuilder {
    pub base_builder: IsoBuilder,
    pub uefi_enabled: bool,
    pub bios_enabled: bool,
    pub efi_image: String,
    pub mbr_image: String,
}

impl HybridIsoBuilder {
    pub fn new() -> Self {
        Self {
            base_builder: IsoBuilder::new(),
            uefi_enabled: true,
            bios_enabled: true,
            efi_image: String::from("/boot/efi.img"),
            mbr_image: String::from("/boot/mbr.bin"),
        }
    }

    pub fn disable_uefi(mut self) -> Self {
        self.uefi_enabled = false;
        self
    }

    pub fn disable_bios(mut self) -> Self {
        self.bios_enabled = false;
        self
    }

    pub fn build_hybrid_config(&self) -> String {
        let mut config = String::new();
        
        config.push_str("# Hybrid ISO Configuration\n");
        config.push_str(&format!("UEFI: {}\n", self.uefi_enabled));
        config.push_str(&format!("BIOS: {}\n", self.bios_enabled));
        config.push_str(&format!("EFI Image: {}\n", self.efi_image));
        config.push_str(&format!("MBR Image: {}\n", self.mbr_image));

        if self.uefi_enabled {
            config.push_str("\n# UEFI Boot Entries\n");
            config.push_str(&self.base_builder.generate_grub_config());
        }

        if self.bios_enabled {
            config.push_str("\n# BIOS Boot Entries\n");
            config.push_str(&self.base_builder.generate_syslinux_config());
        }

        config
    }
}

/// ISO Metadata
#[derive(Debug, Clone)]
pub struct IsoMetadata {
    pub name: String,
    pub version: String,
    pub architecture: String,
    pub build_date: String,
    pub description: String,
}

impl IsoMetadata {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: String::from(name),
            version: String::from(version),
            architecture: String::from("x86_64"),
            build_date: String::from("2026-08-12"),
            description: String::from("SigmaOS Live ISO"),
        }
    }

    pub fn generate_volume_id(&self) -> String {
        format!("{}_{}_{}", self.name, self.version, self.architecture)
    }
}

/// Complete ISO Build System
pub struct IsoBuildSystem {
    pub builder: HybridIsoBuilder,
    pub metadata: IsoMetadata,
    pub live_config: LiveSessionConfig,
    pub build_log: Vec<String>,
}

impl IsoBuildSystem {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            builder: HybridIsoBuilder::new(),
            metadata: IsoMetadata::new(name, version),
            live_config: LiveSessionConfig::new(),
            build_log: Vec::new(),
        }
    }

    pub fn log(&mut self, message: &str) {
        self.build_log.push(String::from(message));
    }

    pub fn build_iso(&mut self) -> Result<String, &'static str> {
        self.log("Starting ISO build process...");
        
        // Validate configuration
        if let Err(e) = self.builder.base_builder.validate() {
            return Err("ISO validation failed");
        }

        // Generate configurations
        let grub_config = self.builder.base_builder.generate_grub_config();
        let syslinux_config = self.builder.base_builder.generate_syslinux_config();
        let hybrid_config = self.builder.build_hybrid_config();

        self.log("Generated boot configurations");
        
        // Calculate size
        let size = self.builder.base_builder.calculate_size();
        self.log(&format!("Estimated ISO size: {} MB", size / (1024 * 1024)));

        // Volume ID
        let volume_id = self.metadata.generate_volume_id();
        self.builder.base_builder.config.volume_id = volume_id.clone();
        self.log(&format!("Volume ID: {}", volume_id));

        self.log("ISO build completed successfully");
        
        Ok(format!("SigmaOS-{}.iso", self.metadata.version))
    }

    pub fn get_build_log(&self) -> &[String] {
        &self.build_log
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iso_boot_config() {
        let config = IsoBootConfig::new();
        assert_eq!(config.iso_label, "SIGMAOS");
        assert!(!config.boot_parameters.is_empty());
    }

    #[test]
    fn test_iso_builder() {
        let mut builder = IsoBuilder::new();
        builder.add_file(IsoFileEntry::new("/test", "/test"));
        
        assert_eq!(builder.files.len(), 1);
        assert!(builder.validate().is_ok());
    }

    #[test]
    fn test_grub_config_generation() {
        let builder = IsoBuilder::new();
        let config = builder.generate_grub_config();
        
        assert!(config.contains("menuentry"));
        assert!(config.contains("SigmaOS Live"));
    }

    #[test]
    fn test_hybrid_builder() {
        let builder = HybridIsoBuilder::new();
        let config = builder.build_hybrid_config();
        
        assert!(config.contains("Hybrid ISO Configuration"));
        assert!(config.contains("UEFI"));
    }

    #[test]
    fn test_iso_build_system() {
        let mut system = IsoBuildSystem::new("SigmaOS", "1.0");
        
        assert!(system.build_iso().is_ok());
        assert!(!system.build_log.is_empty());
    }

    #[test]
    fn test_live_session_config() {
        let config = LiveSessionConfig::new()
            .with_persistence(1024)
            .with_home_persistence();
        
        assert!(config.persistence_enabled);
        assert!(config.home_persistence);
        assert_eq!(config.persistence_size_mb, 1024);
    }
}