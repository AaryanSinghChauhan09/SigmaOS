// SPDX-License-Identifier: MIT
// SigmaOS Boot Process Manager - Arch Linux Inspired
// Implements mkinitcpio-style initramfs generation and boot process management

use std::collections::BTreeMap;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// Initramfs hook type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitcpioHook {
    Base,
    Udev,
    Autodetect,
    Modconf,
    Block,
    Filesystems,
    Keyboard,
    Fsck,
    Encrypt,
    Lvm2,
    Mdadm,
    Netconf,
    Network,
    Dmraid,
    Resume,
    Custom(String),
}

impl InitcpioHook {
    pub fn as_str(&self) -> &str {
        match self {
            InitcpioHook::Base => "base",
            InitcpioHook::Udev => "udev",
            InitcpioHook::Autodetect => "autodetect",
            InitcpioHook::Modconf => "modconf",
            InitcpioHook::Block => "block",
            InitcpioHook::Filesystems => "filesystems",
            InitcpioHook::Keyboard => "keyboard",
            InitcpioHook::Fsck => "fsck",
            InitcpioHook::Encrypt => "encrypt",
            InitcpioHook::Lvm2 => "lvm2",
            InitcpioHook::Mdadm => "mdadm",
            InitcpioHook::Netconf => "netconf",
            InitcpioHook::Network => "network",
            InitcpioHook::Dmraid => "dmraid",
            InitcpioHook::Resume => "resume",
            InitcpioHook::Custom(s) => s,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "base" => Some(InitcpioHook::Base),
            "udev" => Some(InitcpioHook::Udev),
            "autodetect" => Some(InitcpioHook::Autodetect),
            "modconf" => Some(InitcpioHook::Modconf),
            "block" => Some(InitcpioHook::Block),
            "filesystems" => Some(InitcpioHook::Filesystems),
            "keyboard" => Some(InitcpioHook::Keyboard),
            "fsck" => Some(InitcpioHook::Fsck),
            "encrypt" => Some(InitcpioHook::Encrypt),
            "lvm2" => Some(InitcpioHook::Lvm2),
            "mdadm" => Some(InitcpioHook::Mdadm),
            "netconf" => Some(InitcpioHook::Netconf),
            "network" => Some(InitcpioHook::Network),
            "dmraid" => Some(InitcpioHook::Dmraid),
            "resume" => Some(InitcpioHook::Resume),
            _ => Some(InitcpioHook::Custom(s.to_string())),
        }
    }

    pub fn is_required(&self) -> bool {
        matches!(self, InitcpioHook::Base | InitcpioHook::Udev | InitcpioHook::Filesystems)
    }

    pub fn is_optional(&self) -> bool {
        !self.is_required()
    }
}

/// Initramfs compression algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitcpioCompression {
    None,
    Gzip,
    Zstd(u8),
    Lz4,
    Xz,
    Bzip2,
}

impl InitcpioCompression {
    pub fn as_str(&self) -> &str {
        match self {
            InitcpioCompression::None => "none",
            InitcpioCompression::Gzip => "gzip",
            InitcpioCompression::Zstd(_) => "zstd",
            InitcpioCompression::Lz4 => "lz4",
            InitcpioCompression::Xz => "xz",
            InitcpioCompression::Bzip2 => "bzip2",
        }
    }

    pub fn get_extension(&self) -> &str {
        match self {
            InitcpioCompression::None => ".img",
            InitcpioCompression::Gzip => ".img.gz",
            InitcpioCompression::Zstd(_) => ".img.zst",
            InitcpioCompression::Lz4 => ".img.lz4",
            InitcpioCompression::Xz => ".img.xz",
            InitcpioCompression::Bzip2 => ".img.bz2",
        }
    }

    pub fn get_command(&self) -> Vec<String> {
        match self {
            InitcpioCompression::None => vec![],
            InitcpioCompression::Gzip => vec!["gzip".to_string(), "-9".to_string()],
            InitcpioCompression::Zstd(level) => vec!["zstd".to_string(), format!("-{}", level)],
            InitcpioCompression::Lz4 => vec!["lz4".to_string(), "-9".to_string()],
            InitcpioCompression::Xz => vec!["xz".to_string(), "-9".to_string()],
            InitcpioCompression::Bzip2 => vec!["bzip2".to_string(), "-9".to_string()],
        }
    }
}

/// Kernel module information
#[derive(Debug, Clone, PartialEq)]
pub struct KernelModule {
    pub name: String,
    pub path: PathBuf,
    pub dependencies: Vec<String>,
    pub size: u64,
    pub is_builtin: bool,
    pub is_loaded: bool,
    pub parameters: BTreeMap<String, String>,
}

impl KernelModule {
    /// Create a new kernel module
    pub fn new(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
            dependencies: Vec::new(),
            size: 0,
            is_builtin: false,
            is_loaded: false,
            parameters: BTreeMap::new(),
        }
    }

    /// Add dependency
    pub fn add_dependency(mut self, dep: impl Into<String>) -> Self {
        self.dependencies.push(dep.into());
        self
    }

    /// Set size
    pub fn size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }

    /// Set builtin status
    pub fn builtin(mut self, builtin: bool) -> Self {
        self.is_builtin = builtin;
        self
    }

    /// Set loaded status
    pub fn loaded(mut self, loaded: bool) -> Self {
        self.is_loaded = loaded;
        self
    }

    /// Set parameter
    pub fn parameter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.parameters.insert(key.into(), value.into());
        self
    }
}

/// Initramfs preset configuration
#[derive(Debug, Clone)]
pub struct MkinitcpioPreset {
    pub name: String,
    pub default_config: String,
    pub fallback_config: Option<String>,
    pub kernel_image: PathBuf,
    pub initramfs_image: PathBuf,
    pub fallback_initramfs_image: Option<PathBuf>,
    pub ukify_config: Option<PathBuf>,
}

impl MkinitcpioPreset {
    /// Create a new preset
    pub fn new(name: String) -> Self {
        Self {
            name,
            default_config: String::new(),
            fallback_config: None,
            kernel_image: PathBuf::new(),
            initramfs_image: PathBuf::new(),
            fallback_initramfs_image: None,
            ukify_config: None,
        }
    }

    /// Set default config
    pub fn default_config(mut self, config: impl Into<String>) -> Self {
        self.default_config = config.into();
        self
    }

    /// Set fallback config
    pub fn fallback_config(mut self, config: impl Into<String>) -> Self {
        self.fallback_config = Some(config.into());
        self
    }

    /// Set kernel image
    pub fn kernel_image(mut self, image: impl Into<PathBuf>) -> Self {
        self.kernel_image = image.into();
        self
    }

    /// Set initramfs image
    pub fn initramfs_image(mut self, image: impl Into<PathBuf>) -> Self {
        self.initramfs_image = image.into();
        self
    }

    /// Set fallback initramfs image
    pub fn fallback_initramfs_image(mut self, image: impl Into<PathBuf>) -> Self {
        self.fallback_initramfs_image = Some(image.into());
        self
    }

    /// Set UKI config
    pub fn ukify_config(mut self, config: impl Into<PathBuf>) -> Self {
        self.ukify_config = Some(config.into());
        self
    }
}

/// Boot process configuration
#[derive(Debug, Clone)]
pub struct BootConfig {
    pub hooks: Vec<InitcpioHook>,
    pub modules: Vec<String>,
    pub files: Vec<PathBuf>,
    pub binaries: Vec<String>,
    pub compression: InitcpioCompression,
    pub compression_fallback: Option<InitcpioCompression>,
    pub preset: Option<MkinitcpioPreset>,
    pub kernel_cmdline: Vec<String>,
    pub root_device: Option<String>,
    pub resume_device: Option<String>,
    pub encrypt_device: Option<String>,
    pub lvm_device: Option<String>,
    pub mdadm_device: Option<String>,
    pub dmraid_device: Option<String>,
    pub fsck_mode: String,
    pub keyboard_layout: String,
    pub timezone: String,
    pub locale: String,
}

impl BootConfig {
    /// Create a new boot configuration
    pub fn new() -> Self {
        Self {
            hooks: vec![
                InitcpioHook::Base,
                InitcpioHook::Udev,
                InitcpioHook::Autodetect,
                InitcpioHook::Modconf,
                InitcpioHook::Block,
                InitcpioHook::Filesystems,
                InitcpioHook::Keyboard,
                InitcpioHook::Fsck,
            ],
            modules: Vec::new(),
            files: Vec::new(),
            binaries: Vec::new(),
            compression: InitcpioCompression::Zstd(1),
            compression_fallback: Some(InitcpioCompression::Gzip),
            preset: None,
            kernel_cmdline: Vec::new(),
            root_device: None,
            resume_device: None,
            encrypt_device: None,
            lvm_device: None,
            mdadm_device: None,
            dmraid_device: None,
            fsck_mode: "default".to_string(),
            keyboard_layout: "us".to_string(),
            timezone: "UTC".to_string(),
            locale: "C".to_string(),
        }
    }

    /// Add hook
    pub fn add_hook(mut self, hook: InitcpioHook) -> Self {
        if !self.hooks.contains(&hook) {
            self.hooks.push(hook);
        }
        self
    }

    /// Remove hook
    pub fn remove_hook(&mut self, hook: InitcpioHook) {
        self.hooks.retain(|h| h != &hook);
    }

    /// Add module
    pub fn add_module(mut self, module: impl Into<String>) -> Self {
        let module = module.into();
        if !self.modules.contains(&module) {
            self.modules.push(module);
        }
        self
    }

    /// Add file
    pub fn add_file(mut self, file: impl Into<PathBuf>) -> Self {
        self.files.push(file.into());
        self
    }

    /// Add binary
    pub fn add_binary(mut self, binary: impl Into<String>) -> Self {
        self.binaries.push(binary.into());
        self
    }

    /// Set compression
    pub fn compression(mut self, compression: InitcpioCompression) -> Self {
        self.compression = compression;
        self
    }

    /// Set compression fallback
    pub fn compression_fallback(mut self, fallback: InitcpioCompression) -> Self {
        self.compression_fallback = Some(fallback);
        self
    }

    /// Set preset
    pub fn preset(mut self, preset: MkinitcpioPreset) -> Self {
        self.preset = Some(preset);
        self
    }

    /// Add kernel command line parameter
    pub fn kernel_param(mut self, param: impl Into<String>) -> Self {
        self.kernel_cmdline.push(param.into());
        self
    }

    /// Set root device
    pub fn root_device(mut self, device: impl Into<String>) -> Self {
        self.root_device = Some(device.into());
        self
    }

    /// Set resume device
    pub fn resume_device(mut self, device: impl Into<String>) -> Self {
        self.resume_device = Some(device.into());
        self
    }

    /// Set encrypt device
    pub fn encrypt_device(mut self, device: impl Into<String>) -> Self {
        self.encrypt_device = Some(device.into());
        self
    }

    /// Set LVM device
    pub fn lvm_device(mut self, device: impl Into<String>) -> Self {
        self.lvm_device = Some(device.into());
        self
    }

    /// Set mdadm device
    pub fn mdadm_device(mut self, device: impl Into<String>) -> Self {
        self.mdadm_device = Some(device.into());
        self
    }

    /// Set dmraid device
    pub fn dmraid_device(mut self, device: impl Into<String>) -> Self {
        self.dmraid_device = Some(device.into());
        self
    }

    /// Set fsck mode
    pub fn fsck_mode(mut self, mode: impl Into<String>) -> Self {
        self.fsck_mode = mode.into();
        self
    }

    /// Set keyboard layout
    pub fn keyboard_layout(mut self, layout: impl Into<String>) -> Self {
        self.keyboard_layout = layout.into();
        self
    }

    /// Set timezone
    pub fn timezone(mut self, timezone: impl Into<String>) -> Self {
        self.timezone = timezone.into();
        self
    }

    /// Set locale
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = locale.into();
        self
    }

    /// Generate mkinitcpio.conf content
    pub fn to_config_file(&self) -> String {
        let mut content = String::new();

        content.push_str("# SigmaOS mkinitcpio configuration\n");
        content.push_str("# Generated by SigmaOS Boot Manager\n\n");

        // Modules
        if !self.modules.is_empty() {
            content.push_str("MODULES=(");
            for module in &self.modules {
                content.push_str(module);
                content.push(' ');
            }
            content.push_str(")\n");
        } else {
            content.push_str("MODULES=()\n");
        }

        // Binaries
        if !self.binaries.is_empty() {
            content.push_str("BINARIES=(");
            for binary in &self.binaries {
                content.push_str(binary);
                content.push(' ');
            }
            content.push_str(")\n");
        } else {
            content.push_str("BINARIES=()\n");
        }

        // Files
        if !self.files.is_empty() {
            content.push_str("FILES=(");
            for file in &self.files {
                content.push_str(&format!("{} ", file.display()));
            }
            content.push_str(")\n");
        } else {
            content.push_str("FILES=()\n");
        }

        // Hooks
        content.push_str("HOOKS=(");
        for hook in &self.hooks {
            content.push_str(hook.as_str());
            content.push(' ');
        }
        content.push_str(")\n\n");

        // Compression
        content.push_str(&format!("COMPRESSION=\"{}\"\n", self.compression.as_str()));
        if let Some(fallback) = self.compression_fallback {
            content.push_str(&format!("COMPRESSION_FALLBACK=\"{}\"\n", fallback.as_str()));
        }

        content.push_str("\n# Use mkinitcpio.conf.d/ to override the following options\n");
        content.push_str("#MODULES_DEPS=()\n");
        content.push_str("#INSTALL=\n");
        content.push_str("#NO_MODULES=0\n");
        content.push_str("#HOOKS=()\n");

        content
    }

    /// Generate kernel command line
    pub fn to_kernel_cmdline(&self) -> String {
        let mut cmdline = String::new();

        // Add root device
        if let Some(root) = &self.root_device {
            cmdline.push_str(&format!("root={} ", root));
        }

        // Add resume device
        if let Some(resume) = &self.resume_device {
            cmdline.push_str(&format!("resume={} ", resume));
        }

        // Add encrypt device
        if let Some(encrypt) = &self.encrypt_device {
            cmdline.push_str(&format!("cryptdevice={} ", encrypt));
        }

        // Add LVM device
        if let Some(lvm) = &self.lvm_device {
            cmdline.push_str(&format!("rd.lvm.lv={} ", lvm));
        }

        // Add mdadm device
        if let Some(mdadm) = &self.mdadm_device {
            cmdline.push_str(&format!("rd.md={} ", mdadm));
        }

        // Add dmraid device
        if let Some(dmraid) = &self.dmraid_device {
            cmdline.push_str(&format!("rd.dm={} ", dmraid));
        }

        // Add fsck mode
        cmdline.push_str(&format!("fsck.mode={} ", self.fsck_mode));

        // Add keyboard layout
        cmdline.push_str(&format!("keyboard={} ", self.keyboard_layout));

        // Add timezone
        cmdline.push_str(&format!("timezone={} ", self.timezone));

        // Add locale
        cmdline.push_str(&format!("locale={} ", self.locale));

        // Add custom parameters
        for param in &self.kernel_cmdline {
            cmdline.push_str(param);
            cmdline.push(' ');
        }

        cmdline.trim().to_string()
    }
}

impl Default for BootConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Initramfs builder
#[derive(Debug, Clone)]
pub struct InitramfsBuilder {
    pub config: BootConfig,
    pub build_dir: PathBuf,
    pub output_dir: PathBuf,
    pub temp_dir: PathBuf,
    pub use_zstd: bool,
    pub parallel_build: bool,
    pub clean_build: bool,
}

impl InitramfsBuilder {
    /// Create a new initramfs builder
    pub fn new(config: BootConfig, build_dir: PathBuf, output_dir: PathBuf) -> Self {
        Self {
            config,
            build_dir,
            output_dir,
            temp_dir: PathBuf::from("/tmp/initramfs-build"),
            use_zstd: true,
            parallel_build: true,
            clean_build: false,
        }
    }

    /// Set temp directory
    pub fn temp_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.temp_dir = dir.into();
        self
    }

    /// Enable/disable zstd compression
    pub fn zstd(mut self, enable: bool) -> Self {
        self.use_zstd = enable;
        self
    }

    /// Enable/disable parallel build
    pub fn parallel(mut self, enable: bool) -> Self {
        self.parallel_build = enable;
        self
    }

    /// Enable/disable clean build
    pub fn clean_build(mut self, enable: bool) -> Self {
        self.clean_build = enable;
        self
    }

    /// Build initramfs
    pub fn build(&mut self) -> Result<PathBuf, String> {
        // Create directories
        fs::create_dir_all(&self.build_dir).map_err(|e| format!("Failed to create build dir: {}", e))?;
        fs::create_dir_all(&self.output_dir).map_err(|e| format!("Failed to create output dir: {}", e))?;
        fs::create_dir_all(&self.temp_dir).map_err(|e| format!("Failed to create temp dir: {}", e))?;

        // Clean build directory if requested
        if self.clean_build {
            self.clean_build_dir()?;
        }

        // Generate configuration file
        let config_file = self.build_dir.join("mkinitcpio.conf");
        let config_content = self.config.to_config_file();
        fs::write(&config_file, config_content).map_err(|e| format!("Failed to write config: {}", e))?;

        // Execute hooks
        self.execute_hooks()?;

        // Copy files
        self.copy_files()?;

        // Copy binaries
        self.copy_binaries()?;

        // Create initramfs image
        let output_file = self.create_initramfs_image()?;

        Ok(output_file)
    }

    /// Clean build directory
    fn clean_build_dir(&self) -> Result<(), String> {
        if self.build_dir.exists() {
            fs::remove_dir_all(&self.build_dir).map_err(|e| format!("Failed to clean build dir: {}", e))?;
        }
        fs::create_dir_all(&self.build_dir).map_err(|e| format!("Failed to create build dir: {}", e))?;
        Ok(())
    }

    /// Execute initramfs hooks
    fn execute_hooks(&self) -> Result<(), String> {
        for hook in &self.config.hooks {
            match hook {
                InitcpioHook::Base => self.execute_base_hook()?,
                InitcpioHook::Udev => self.execute_udev_hook()?,
                InitcpioHook::Autodetect => self.execute_autodetect_hook()?,
                InitcpioHook::Modconf => self.execute_modconf_hook()?,
                InitcpioHook::Block => self.execute_block_hook()?,
                InitcpioHook::Filesystems => self.execute_filesystems_hook()?,
                InitcpioHook::Keyboard => self.execute_keyboard_hook()?,
                InitcpioHook::Fsck => self.execute_fsck_hook()?,
                InitcpioHook::Encrypt => self.execute_encrypt_hook()?,
                InitcpioHook::Lvm2 => self.execute_lvm2_hook()?,
                InitcpioHook::Mdadm => self.execute_mdadm_hook()?,
                InitcpioHook::Netconf => self.execute_netconf_hook()?,
                InitcpioHook::Network => self.execute_network_hook()?,
                InitcpioHook::Dmraid => self.execute_dmraid_hook()?,
                InitcpioHook::Resume => self.execute_resume_hook()?,
                InitcpioHook::Custom(name) => self.execute_custom_hook(name)?,
            }
        }
        Ok(())
    }

    /// Execute base hook
    fn execute_base_hook(&self) -> Result<(), String> {
        // Create basic directory structure
        let dirs = ["bin", "sbin", "lib", "usr", "etc", "var", "tmp", "dev", "proc", "sys", "run"];
        for dir in dirs {
            let dir_path = self.temp_dir.join(dir);
            fs::create_dir_all(&dir_path).map_err(|e| format!("Failed to create {}: {}", dir, e))?;
        }

        // Copy essential files
        let essential_files = [
            "/etc/passwd", "/etc/group", "/etc/shadow", "/etc/hosts", "/etc/resolv.conf"
        ];

        for file in essential_files {
            let src = Path::new(file);
            if src.exists() {
                let dst = self.temp_dir.join("etc").join(src.file_name().unwrap());
                fs::copy(src, dst).map_err(|e| format!("Failed to copy {}: {}", file, e))?;
            }
        }

        Ok(())
    }

    /// Execute udev hook
    fn execute_udev_hook(&self) -> Result<(), String> {
        // Copy udev rules and binaries
        let udev_dirs = [
            "/etc/udev",
            "/lib/udev",
            "/usr/lib/udev",
        ];

        for dir in udev_dirs {
            let src = Path::new(dir);
            if src.exists() {
                let dst = self.temp_dir.join(dir.trim_start_matches('/'));
                self.copy_directory_recursive(src, &dst)?;
            }
        }

        Ok(())
    }

    /// Execute autodetect hook
    fn execute_autodetect_hook(&self) -> Result<(), String> {
        // Detect hardware and load appropriate modules
        // This would use the hardware detection system
        let modules = self.detect_required_modules()?;

        for module in modules {
            self.copy_module(&module)?;
        }

        Ok(())
    }

    /// Execute modconf hook
    fn execute_modconf_hook(&self) -> Result<(), String> {
        // Load modules from modconf
        let modconf_path = Path::new("/etc/modprobe.d");
        if modconf_path.exists() {
            for entry in fs::read_dir(modconf_path)? {
                let entry = entry?;
                let path = entry.path();

                if path.extension().map_or(false, |ext| ext == "conf") {
                    let dst = self.temp_dir.join("etc/modprobe.d").join(entry.file_name());
                    fs::create_dir_all(dst.parent().unwrap()).map_err(|e| format!("Failed to create modprobe.d: {}", e))?;
                    fs::copy(&path, dst).map_err(|e| format!("Failed to copy modconf: {}", e))?;
                }
            }
        }

        Ok(())
    }

    /// Execute block hook
    fn execute_block_hook(&self) -> Result<(), String> {
        // Copy block device modules and tools
        let block_modules = ["sd_mod", "sr_mod", "loop", "nbd", "floppy"];

        for module in block_modules {
            self.copy_module(module)?;
        }

        // Copy block utilities
        let block_bins = ["blkid", "fdisk", "parted", "sfdisk"];
        for bin in block_bins {
            self.copy_binary(bin)?;
        }

        Ok(())
    }

    /// Execute filesystems hook
    fn execute_filesystems_hook(&self) -> Result<(), String> {
        // Copy filesystem modules
        let fs_modules = ["ext4", "xfs", "btrfs", "f2fs", "ntfs", "vfat", "exfat"];

        for module in fs_modules {
            self.copy_module(module)?;
        }

        // Copy filesystem utilities
        let fs_bins = ["fsck", "mkfs", "mount", "umount", "swapon", "swapoff"];
        for bin in fs_bins {
            self.copy_binary(bin)?;
        }

        Ok(())
    }

    /// Execute keyboard hook
    fn execute_keyboard_hook(&self) -> Result<(), String> {
        // Copy keyboard modules and configuration
        let keyboard_modules = ["atkbd", "ps2", "usbkbd"];

        for module in keyboard_modules {
            self.copy_module(module)?;
        }

        // Copy keyboard configuration
        let keyboard_files = [
            "/etc/vconsole.conf",
            "/etc/keymap.conf",
        ];

        for file in keyboard_files {
            let src = Path::new(file);
            if src.exists() {
                let dst = self.temp_dir.join("etc").join(src.file_name().unwrap());
                fs::copy(src, dst).map_err(|e| format!("Failed to copy {}: {}", file, e))?;
            }
        }

        Ok(())
    }

    /// Execute fsck hook
    fn execute_fsck_hook(&self) -> Result<(), String> {
        // Copy fsck utilities
        let fsck_bins = ["fsck", "fsck.ext4", "fsck.xfs", "fsck.btrfs"];
        for bin in fsck_bins {
            self.copy_binary(bin)?;
        }

        Ok(())
    }

    /// Execute encrypt hook
    fn execute_encrypt_hook(&self) -> Result<(), String> {
        // Copy encryption modules and tools
        let encrypt_modules = ["dm_crypt", "aes", "xts", "cbc", "sha256"];

        for module in encrypt_modules {
            self.copy_module(module)?;
        }

        // Copy encryption utilities
        let encrypt_bins = ["cryptsetup", "dmsetup"];
        for bin in encrypt_bins {
            self.copy_binary(bin)?;
        }

        Ok(())
    }

    /// Execute LVM2 hook
    fn execute_lvm2_hook(&self) -> Result<(), String> {
        // Copy LVM modules and tools
        let lvm_modules = ["dm_mod", "dm_mirror", "dm_snapshot", "dm_thin_pool"];

        for module in lvm_modules {
            self.copy_module(module)?;
        }

        // Copy LVM utilities
        let lvm_bins = ["lvm", "pvcreate", "vgcreate", "lvcreate"];
        for bin in lvm_bins {
            self.copy_binary(bin)?;
        }

        Ok(())
    }

    /// Execute mdadm hook
    fn execute_mdadm_hook(&self) -> Result<(), String> {
        // Copy mdadm module and tools
        self.copy_module("md_mod")?;
        self.copy_binary("mdadm")?;

        // Copy mdadm configuration
        let mdadm_conf = Path::new("/etc/mdadm.conf");
        if mdadm_conf.exists() {
            let dst = self.temp_dir.join("etc/mdadm.conf");
            fs::copy(mdadm_conf, dst).map_err(|e| format!("Failed to copy mdadm.conf: {}", e))?;
        }

        Ok(())
    }

    /// Execute netconf hook
    fn execute_netconf_hook(&self) -> Result<(), String> {
        // Copy network configuration files
        let net_files = [
            "/etc/network/interfaces",
            "/etc/netctl.conf",
            "/etc/systemd/network",
        ];

        for file in net_files {
            let src = Path::new(file);
            if src.exists() {
                let dst = self.temp_dir.join("etc").join(src.file_name().unwrap());
                fs::copy(src, dst).map_err(|e| format!("Failed to copy {}: {}", file, e))?;
            }
        }

        Ok(())
    }

    /// Execute network hook
    fn execute_network_hook(&self) -> Result<(), String> {
        // Copy network modules
        let net_modules = ["loop", "bonding", "8021q", "tun", "tap"];

        for module in net_modules {
            self.copy_module(module)?;
        }

        // Copy network utilities
        let net_bins = ["ip", "ifconfig", "route", "ping", "dhclient", "dhcpcd"];
        for bin in net_bins {
            self.copy_binary(bin)?;
        }

        Ok(())
    }

    /// Execute dmraid hook
    fn execute_dmraid_hook(&self) -> Result<(), String> {
        // Copy dmraid module and tools
        self.copy_module("dm_mod")?;
        self.copy_binary("dmraid")?;

        Ok(())
    }

    /// Execute resume hook
    fn execute_resume_hook(&self) -> Result<(), String> {
        // Copy resume-related modules
        let resume_modules = ["ext4", "xfs", "btrfs", "swap"];

        for module in resume_modules {
            self.copy_module(module)?;
        }

        Ok(())
    }

    /// Execute custom hook
    fn execute_custom_hook(&self, name: &str) -> Result<(), String> {
        // Execute custom hook script
        let hook_script = Path::new("/usr/lib/initcpio/hooks").join(format!("{}.install", name));
        if hook_script.exists() {
            // In real implementation, would execute the hook script
            println!("Executing custom hook: {}", name);
        }
        Ok(())
    }

    /// Detect required modules
    fn detect_required_modules(&self) -> Result<Vec<String>, String> {
        let mut modules = Vec::new();

        // Add explicitly configured modules
        modules.extend(self.config.modules.clone());

        // Add modules for detected hardware
        // This would use the hardware detection system
        let hardware_modules = self.detect_hardware_modules()?;
        modules.extend(hardware_modules);

        // Add filesystem modules based on root device
        if let Some(root) = &self.config.root_device {
            let fs_modules = self.get_filesystem_modules(root)?;
            modules.extend(fs_modules);
        }

        // Add storage modules
        let storage_modules = self.get_storage_modules()?;
        modules.extend(storage_modules);

        // Remove duplicates
        modules.sort();
        modules.dedup();

        Ok(modules)
    }

    /// Detect hardware modules
    fn detect_hardware_modules(&self) -> Result<Vec<String>, String> {
        let mut modules = Vec::new();

        // This would use the hardware detection system to determine required modules
        // For now, return common modules
        modules.extend(vec![
            "ahci".to_string(),
            "usb_storage".to_string(),
            "sd_mod".to_string(),
            "sr_mod".to_string(),
        ]);

        Ok(modules)
    }

    /// Get filesystem modules for root device
    fn get_filesystem_modules(&self, root: &str) -> Result<Vec<String>, String> {
        let mut modules = Vec::new();

        // Parse root device to determine filesystem
        if root.contains("ext4") {
            modules.push("ext4".to_string());
        } else if root.contains("xfs") {
            modules.push("xfs".to_string());
        } else if root.contains("btrfs") {
            modules.push("btrfs".to_string());
        } else if root.contains("f2fs") {
            modules.push("f2fs".to_string());
        } else if root.contains("ntfs") {
            modules.push("ntfs".to_string());
        } else if root.contains("vfat") {
            modules.push("vfat".to_string());
        } else if root.contains("exfat") {
            modules.push("exfat".to_string());
        }

        Ok(modules)
    }

    /// Get storage modules
    fn get_storage_modules(&self) -> Result<Vec<String>, String> {
        let mut modules = Vec::new();

        // Add common storage modules
        modules.extend(vec![
            "sd_mod".to_string(),
            "sr_mod".to_string(),
            "loop".to_string(),
            "nbd".to_string(),
        ]);

        Ok(modules)
    }

    /// Copy files to initramfs
    fn copy_files(&self) -> Result<(), String> {
        for file in &self.config.files {
            if file.exists() {
                let dst = self.temp_dir.join(file.strip_prefix("/").unwrap_or(file));
                if let Some(parent) = dst.parent() {
                    fs::create_dir_all(parent).map_err(|e| format!("Failed to create parent dir: {}", e))?;
                }
                fs::copy(file, &dst).map_err(|e| format!("Failed to copy {}: {}", file.display(), e))?;
            }
        }
        Ok(())
    }

    /// Copy binaries to initramfs
    fn copy_binaries(&self) -> Result<(), String> {
        for binary in &self.config.binaries {
            self.copy_binary(binary)?;
        }
        Ok(())
    }

    /// Copy a single module
    fn copy_module(&self, module: &str) -> Result<(), String> {
        // Find module in kernel modules directory
        let modules_dir = Path::new("/lib/modules");
        if !modules_dir.exists() {
            return Ok(()); // No modules directory
        }

        // Find the latest kernel version
        let mut kernel_versions = Vec::new();
        for entry in fs::read_dir(modules_dir)? {
            let entry = entry?;
            let version = entry.file_name().to_string_lossy().to_string();
            if version.starts_with("6.") || version.starts_with("5.") {
                kernel_versions.push(version);
            }
        }

        kernel_versions.sort();
        kernel_versions.reverse(); // Latest first

        if let Some(version) = kernel_versions.first() {
            let module_path = modules_dir.join(version).join("kernel").join(format!("{}.ko", module));

            if module_path.exists() {
                let dst = self.temp_dir.join("lib/modules").join(version).join("kernel").join(format!("{}.ko", module));
                if let Some(parent) = dst.parent() {
                    fs::create_dir_all(parent).map_err(|e| format!("Failed to create module dir: {}", e))?;
                }
                fs::copy(&module_path, &dst).map_err(|e| format!("Failed to copy module {}: {}", module, e))?;
            }
        }

        Ok(())
    }

    /// Copy a single binary
    fn copy_binary(&self, binary: &str) -> Result<(), String> {
        // Try to find binary in common locations
        let search_paths = [
            format!("/usr/bin/{}", binary),
            format!("/usr/sbin/{}", binary),
            format!("/bin/{}", binary),
            format!("/sbin/{}", binary),
        ];

        for path in search_paths {
            let src = Path::new(&path);
            if src.exists() {
                let dst = self.temp_dir.join(path.trim_start_matches('/'));
                if let Some(parent) = dst.parent() {
                    fs::create_dir_all(parent).map_err(|e| format!("Failed to create binary dir: {}", e))?;
                }
                fs::copy(src, &dst).map_err(|e| format!("Failed to copy binary {}: {}", binary, e))?;
                return Ok(());
            }
        }

        // Binary not found, but don't fail - just warn
        eprintln!("Warning: Binary {} not found", binary);
        Ok(())
    }

    /// Copy directory recursively
    fn copy_directory_recursive(&self, src: &Path, dst: &Path) -> Result<(), String> {
        fs::create_dir_all(dst).map_err(|e| format!("Failed to create directory {}: {}", dst.display(), e))?;

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if src_path.is_dir() {
                self.copy_directory_recursive(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path).map_err(|e| format!("Failed to copy {}: {}", src_path.display(), e))?;
            }
        }

        Ok(())
    }

    /// Create initramfs image
    fn create_initramfs_image(&self) -> Result<PathBuf, String> {
        let image_name = format!("initramfs-{}.img{}", self.config.preset.as_ref().map(|p| p.name.as_str()).unwrap_or("default"), self.config.compression.get_extension());
        let output_path = self.output_dir.join(&image_name);

        // Create CPIO archive
        let cpio_path = self.temp_dir.join("initramfs.cpio");
        self.create_cpio_archive(&self.temp_dir, &cpio_path)?;

        // Compress the archive
        if self.config.compression != InitcpioCompression::None {
            self.compress_archive(&cpio_path, &output_path)?;
        } else {
            fs::copy(&cpio_path, &output_path).map_err(|e| format!("Failed to copy initramfs: {}", e))?;
        }

        Ok(output_path)
    }

    /// Create CPIO archive
    fn create_cpio_archive(&self, source_dir: &Path, output_path: &Path) -> Result<(), String> {
        // This would use the cpio command or implement CPIO format
        // For now, create a simple archive structure

        let mut archive = String::new();
        archive.push_str("070701"); // CPIO newc format magic
        archive.push('\n');

        // Add files to archive
        self.add_files_to_cpio(source_dir, source_dir, &mut archive)?;

        // Add end-of-archive marker
        archive.push_str("TRAILER!!!");
        archive.push('\n');

        fs::write(output_path, archive).map_err(|e| format!("Failed to write CPIO archive: {}", e))?;

        Ok(())
    }

    /// Add files to CPIO archive
    fn add_files_to_cpio(&self, base_dir: &Path, current_dir: &Path, archive: &mut String) -> Result<(), String> {
        for entry in fs::read_dir(current_dir)? {
            let entry = entry?;
            let path = entry.path();
            let name = path.strip_prefix(base_dir).unwrap_or(&path);
            let name_str = name.to_string_lossy().to_string();

            if path.is_dir() {
                self.add_files_to_cpio(base_dir, &path, archive)?;
            } else if path.is_file() {
                // Add file header (simplified)
                archive.push_str(&format!("{}\n", name_str));
            }
        }

        Ok(())
    }

    /// Compress archive
    fn compress_archive(&self, input_path: &Path, output_path: &Path) -> Result<(), String> {
        let compression_cmd = self.config.compression.get_command();

        if compression_cmd.is_empty() {
            fs::copy(input_path, output_path).map_err(|e| format!("Failed to copy archive: {}", e))?;
            return Ok(());
        }

        // Use external compression command
        let mut cmd = Command::new(&compression_cmd[0]);
        cmd.args(&compression_cmd[1..]);
        cmd.arg("-c"); // Compress to stdout
        cmd.arg(input_path);

        let output = cmd.output().map_err(|e| format!("Failed to run compression: {}", e))?;

        if !output.status.success() {
            return Err(format!("Compression failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        fs::write(output_path, &output.stdout).map_err(|e| format!("Failed to write compressed archive: {}", e))?;

        Ok(())
    }

    /// Get build statistics
    pub fn get_statistics(&self) -> BuildStatistics {
        let mut total_size = 0;
        let mut file_count = 0;

        if self.temp_dir.exists() {
            self.calculate_dir_size(&self.temp_dir, &mut total_size, &mut file_count);
        }

        BuildStatistics {
            total_size,
            file_count,
            hooks_count: self.config.hooks.len(),
            modules_count: self.config.modules.len(),
            compression: self.config.compression.as_str().to_string(),
        }
    }

    /// Calculate directory size
    fn calculate_dir_size(&self, dir: &Path, total_size: &mut u64, file_count: &mut usize) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    self.calculate_dir_size(&path, total_size, file_count);
                } else if path.is_file() {
                    if let Ok(metadata) = entry.metadata() {
                        *total_size += metadata.len();
                        *file_count += 1;
                    }
                }
            }
        }
    }
}

impl Default for InitramfsBuilder {
    fn default() -> Self {
        Self::new(BootConfig::new(), PathBuf::from("/boot/initramfs-build"), PathBuf::from("/boot"))
    }
}

/// Build statistics
#[derive(Debug, Clone)]
pub struct BuildStatistics {
    pub total_size: u64,
    pub file_count: usize,
    pub hooks_count: usize,
    pub modules_count: usize,
    pub compression: String,
}

/// Boot manager
#[derive(Debug, Clone)]
pub struct BootManager {
    pub config: BootConfig,
    pub builder: InitramfsBuilder,
    pub presets: Vec<MkinitcpioPreset>,
    pub current_preset: Option<String>,
    pub last_build: Option<u64>,
}

impl BootManager {
    /// Create a new boot manager
    pub fn new() -> Self {
        let config = BootConfig::new();
        let builder = InitramfsBuilder::new(
            config.clone(),
            PathBuf::from("/boot/initramfs-build"),
            PathBuf::from("/boot"),
        );

        Self {
            config,
            builder,
            presets: Vec::new(),
            current_preset: None,
            last_build: None,
        }
    }

    /// Create boot manager with custom config
    pub fn with_config(config: BootConfig) -> Self {
        let builder = InitramfsBuilder::new(
            config.clone(),
            PathBuf::from("/boot/initramfs-build"),
            PathBuf::from("/boot"),
        );

        Self {
            config,
            builder,
            presets: Vec::new(),
            current_preset: None,
            last_build: None,
        }
    }

    /// Load configuration from file
    pub fn load_config(&mut self, path: &Path) -> Result<(), String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config: {}", e))?;

        // Parse mkinitcpio.conf
        self.parse_config_file(&content)?;

        Ok(())
    }

    /// Parse configuration file
    fn parse_config_file(&mut self, content: &str) -> Result<(), String> {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "MODULES" => {
                        self.config.modules = self.parse_array(value);
                    }
                    "BINARIES" => {
                        self.config.binaries = self.parse_array(value);
                    }
                    "FILES" => {
                        self.config.files = self.parse_array(value).iter().map(PathBuf::from).collect();
                    }
                    "HOOKS" => {
                        self.config.hooks = self.parse_array(value).iter()
                            .filter_map(|s| InitcpioHook::from_str(s))
                            .collect();
                    }
                    "COMPRESSION" => {
                        if let Some(compression) = self.parse_compression(value) {
                            self.config.compression = compression;
                        }
                    }
                    "COMPRESSION_FALLBACK" => {
                        if let Some(fallback) = self.parse_compression(value) {
                            self.config.compression_fallback = Some(fallback);
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    /// Parse array from string
    fn parse_array(&self, value: &str) -> Vec<String> {
        let value = value.trim();
        if value.starts_with('(') && value.ends_with(')') {
            let inner = &value[1..value.len()-1];
            inner.split_whitespace().map(String::from).collect()
        } else {
            Vec::new()
        }
    }

    /// Parse compression from string
    fn parse_compression(&self, value: &str) -> Option<InitcpioCompression> {
        let value = value.trim().trim_matches('"');
        match value {
            "none" => Some(InitcpioCompression::None),
            "gzip" => Some(InitcpioCompression::Gzip),
            "zstd" => Some(InitcpioCompression::Zstd(1)),
            "lz4" => Some(InitcpioCompression::Lz4),
            "xz" => Some(InitcpioCompression::Xz),
            "bzip2" => Some(InitcpioCompression::Bzip2),
            _ => None,
        }
    }

    /// Save configuration to file
    pub fn save_config(&self, path: &Path) -> Result<(), String> {
        let content = self.config.to_config_file();
        fs::write(path, content).map_err(|e| format!("Failed to write config: {}", e))?;
        Ok(())
    }

    /// Build initramfs
    pub fn build(&mut self) -> Result<PathBuf, String> {
        let result = self.builder.build()?;
        self.last_build = Some(SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs());
        Ok(result)
    }

    /// Add preset
    pub fn add_preset(&mut self, preset: MkinitcpioPreset) {
        self.presets.push(preset);
    }

    /// Set current preset
    pub fn set_preset(&mut self, name: &str) -> Result<(), String> {
        if self.presets.iter().any(|p| p.name == name) {
            self.current_preset = Some(name.to_string());
            Ok(())
        } else {
            Err(format!("Preset '{}' not found", name))
        }
    }

    /// Get current preset
    pub fn get_preset(&self) -> Option<&MkinitcpioPreset> {
        self.current_preset.as_ref()
            .and_then(|name| self.presets.iter().find(|p| &p.name == name))
    }

    /// List presets
    pub fn list_presets(&self) -> Vec<&MkinitcpioPreset> {
        self.presets.iter().collect()
    }

    /// Get build statistics
    pub fn get_statistics(&self) -> BuildStatistics {
        self.builder.get_statistics()
    }

    /// Check if build is needed
    pub fn needs_build(&self) -> bool {
        if let Some(last_build) = self.last_build {
            // Check if config files have been modified since last build
            // For now, always return true if last_build is None
            false
        } else {
            true
        }
    }

    /// Get kernel command line
    pub fn get_kernel_cmdline(&self) -> String {
        self.config.to_kernel_cmdline()
    }

    /// Set root device
    pub fn set_root_device(&mut self, device: impl Into<String>) {
        self.config.root_device = Some(device.into());
    }

    /// Set resume device
    pub fn set_resume_device(&mut self, device: impl Into<String>) {
        self.config.resume_device = Some(device.into());
    }

    /// Add kernel parameter
    pub fn add_kernel_param(&mut self, param: impl Into<String>) {
        self.config.kernel_cmdline.push(param.into());
    }

    /// Remove kernel parameter
    pub fn remove_kernel_param(&mut self, param: &str) {
        self.config.kernel_cmdline.retain(|p| p != param);
    }

    /// Export configuration
    pub fn export_config(&self) -> String {
        let mut export = String::new();
        export.push_str("# SigmaOS Boot Configuration Export\n\n");
        export.push_str(&self.config.to_config_file());
        export.push_str("\n# Kernel Command Line\n");
        export.push_str(&self.config.to_kernel_cmdline());
        export
    }

    /// Import configuration
    pub fn import_config(&mut self, content: &str) -> Result<(), String> {
        self.parse_config_file(content)?;
        Ok(())
    }
}

impl Default for BootManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initcpio_hook() {
        assert_eq!(InitcpioHook::Base.as_str(), "base");
        assert_eq!(InitcpioHook::from_str("udev"), Some(InitcpioHook::Udev));
        assert!(InitcpioHook::Base.is_required());
        assert!(InitcpioHook::Udev.is_optional());
    }

    #[test]
    fn test_initcpio_compression() {
        assert_eq!(InitcpioCompression::Gzip.as_str(), "gzip");
        assert_eq!(InitcpioCompression::Zstd(1).as_str(), "zstd");
        assert_eq!(InitcpioCompression::Gzip.get_extension(), ".img.gz");
    }

    #[test]
    fn test_boot_config_builder() {
        let config = BootConfig::new()
            .add_hook(InitcpioHook::Encrypt)
            .add_module("aes")
            .add_binary("cryptsetup")
            .compression(InitcpioCompression::Zstd(3))
            .root_device("/dev/sda1")
            .keyboard_layout("de")
            .timezone("Europe/Berlin");

        assert!(config.hooks.contains(&InitcpioHook::Encrypt));
        assert!(config.modules.contains(&"aes".to_string()));
        assert!(config.binaries.contains(&"cryptsetup".to_string()));
        assert_eq!(config.compression, InitcpioCompression::Zstd(3));
        assert_eq!(config.root_device, Some("/dev/sda1".to_string()));
    }

    #[test]
    fn test_boot_config_to_config_file() {
        let config = BootConfig::new()
            .add_module("ext4")
            .add_binary("fsck")
            .compression(InitcpioCompression::Gzip);

        let config_file = config.to_config_file();
        assert!(config_file.contains("MODULES=(ext4"));
        assert!(config_file.contains("BINARIES=(fsck"));
        assert!(config_file.contains("COMPRESSION=\"gzip\""));
    }

    #[test]
    fn test_boot_config_to_kernel_cmdline() {
        let config = BootConfig::new()
            .root_device("/dev/sda1")
            .resume_device("/dev/sda2")
            .keyboard_layout("us")
            .timezone("UTC");

        let cmdline = config.to_kernel_cmdline();
        assert!(cmdline.contains("root=/dev/sda1"));
        assert!(cmdline.contains("resume=/dev/sda2"));
        assert!(cmdline.contains("keyboard=us"));
        assert!(cmdline.contains("timezone=UTC"));
    }

    #[test]
    fn test_initramfs_builder() {
        let config = BootConfig::new();
        let builder = InitramfsBuilder::new(
            config,
            PathBuf::from("/tmp/test-build"),
            PathBuf::from("/tmp/test-output"),
        );

        assert_eq!(builder.config.hooks.len(), 8);
        assert_eq!(builder.config.compression, InitcpioCompression::Zstd(1));
    }

    #[test]
    fn test_boot_manager() {
        let mut manager = BootManager::new();

        // Test preset management
        let preset = MkinitcpioPreset::new("default".to_string())
            .kernel_image(PathBuf::from("/boot/vmlinuz-linux"))
            .initramfs_image(PathBuf::from("/boot/initramfs-linux.img"));

        manager.add_preset(preset);
        assert!(manager.set_preset("default").is_ok());
        assert!(manager.get_preset().is_some());

        // Test kernel command line
        manager.set_root_device("/dev/sda1");
        manager.add_kernel_param("quiet");
        let cmdline = manager.get_kernel_cmdline();
        assert!(cmdline.contains("root=/dev/sda1"));
        assert!(cmdline.contains("quiet"));
    }

    #[test]
    fn test_boot_manager_config() {
        let mut manager = BootManager::new();

        // Test configuration import/export
        let config_content = r#"
MODULES=(ext4 aes)
BINARIES=(fsck cryptsetup)
HOOKS=(base udev block filesystems)
COMPRESSION="gzip"
"#;

        assert!(manager.import_config(config_content).is_ok());
        assert!(manager.config.modules.contains(&"ext4".to_string()));
        assert!(manager.config.binaries.contains(&"fsck".to_string()));
        assert!(manager.config.hooks.contains(&InitcpioHook::Base));
    }

    #[test]
    fn test_initcpio_hook_parsing() {
        let content = r#"
MODULES=(ext4)
HOOKS=(base udev block filesystems encrypt)
COMPRESSION="zstd"
"#;

        let mut manager = BootManager::new();
        manager.import_config(content).unwrap();

        assert!(manager.config.hooks.contains(&InitcpioHook::Encrypt));
        assert_eq!(manager.config.compression, InitcpioCompression::Zstd(1));
    }
}
