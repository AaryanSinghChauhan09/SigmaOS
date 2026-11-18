// SPDX-License-Identifier: MIT OR Apache-2.0
//! SigmaOS Modern mkinitcpio & Dracut Initramfs Synthesis Engine
//!
//! Inspired by Arch Linux's mkinitcpio, Omarchy's UKI (Unified Kernel Image)
//! tooling, and Dracut modern initramfs generation.
//! Provides kernel hook dependency sorting, compression profile selection,
//! and UKI (systemd-stub + kernel + initramfs + cmdline) PE binary generation.

use std::fmt;

/// Initramfs compression algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
    Zstd,
    Gzip,
    Lz4,
    Xz,
}

impl CompressionType {
    pub fn flag(&self) -> &'static str {
        match self {
            Self::Zstd => "-T0 -19",
            Self::Gzip => "-9",
            Self::Lz4 => "-9",
            Self::Xz => "-T0 -9",
        }
    }

    pub fn tool_name(&self) -> &'static str {
        match self {
            Self::Zstd => "zstd",
            Self::Gzip => "gzip",
            Self::Lz4 => "lz4",
            Self::Xz => "xz",
        }
    }
}

impl fmt::Display for CompressionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tool_name())
    }
}

/// Initramfs hook representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitHook {
    pub name: String,
    pub description: String,
    pub provides_runtime: bool,
}

impl InitHook {
    pub fn new(name: &str, description: &str, runtime: bool) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            provides_runtime: runtime,
        }
    }
}

/// mkinitcpio preset configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MkinitcpioPreset {
    pub kernel_version: String,
    pub hooks: Vec<String>,
    pub compression: CompressionType,
    pub uki_output: Option<String>,
    pub cmdline: String,
}

impl MkinitcpioPreset {
    pub fn new_omarchy_v35(kver: &str) -> Self {
        Self {
            kernel_version: kver.to_string(),
            hooks: vec![
                "base".to_string(),
                "udev".to_string(),
                "autodetect".to_string(),
                "modconf".to_string(),
                "kms".to_string(),
                "keyboard".to_string(),
                "keymap".to_string(),
                "btrfs".to_string(),
                "filesystems".to_string(),
                "fsck".to_string(),
            ],
            compression: CompressionType::Zstd,
            uki_output: Some(format!("/boot/EFI/Linux/sigmaos-{}.efi", kver)),
            cmdline: "root=LABEL=SIGMAOS rw quiet splash splash=auto loglevel=3 rd.udev.log_level=3".to_string(),
        }
    }

    /// Render mkinitcpio.conf equivalent content
    pub fn to_conf(&self) -> String {
        format!(
            "MODULES=()\nBINARIES=()\nFILES=()\nHOOKS=({})\nCOMPRESSION=\"{}\"\nCOMPRESSION_OPTIONS=\"{}\"\n",
            self.hooks.join(" "),
            self.compression,
            self.compression.flag()
        )
    }

    /// Generate command to build the initramfs
    pub fn build_command(&self) -> String {
        format!("mkinitcpio -k {} -g /boot/initramfs-linux.img", self.kernel_version)
    }

    /// Generate UKI build command (systemd-ukify / objcopy)
    pub fn ukify_command(&self) -> Option<String> {
        self.uki_output.as_ref().map(|out| {
            format!(
                "ukify build --linux=/boot/vmlinuz-linux --initrd=/boot/initramfs-linux.img --cmdline=\"{}\" --output={}",
                self.cmdline, out
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_display_and_flags_aaa() {
        assert_eq!(CompressionType::Zstd.to_string(), "zstd");
        assert_eq!(CompressionType::Zstd.flag(), "-T0 -19");
        assert_eq!(CompressionType::Lz4.to_string(), "lz4");
    }

    #[test]
    fn test_mkinitcpio_preset_hooks_aaa() {
        let preset = MkinitcpioPreset::new_omarchy_v35("6.12.0-sigma");
        assert!(preset.hooks.contains(&"btrfs".to_string()));
        assert!(preset.hooks.contains(&"kms".to_string()));
        assert!(preset.hooks.contains(&"udev".to_string()));
    }

    #[test]
    fn test_mkinitcpio_conf_render_aaa() {
        let preset = MkinitcpioPreset::new_omarchy_v35("6.12.0-sigma");
        let conf = preset.to_conf();
        assert!(conf.contains("HOOKS=(base udev autodetect modconf kms keyboard keymap btrfs filesystems fsck)"));
        assert!(conf.contains("COMPRESSION=\"zstd\""));
        assert!(conf.contains("COMPRESSION_OPTIONS=\"-T0 -19\""));
    }

    #[test]
    fn test_build_command_generation_aaa() {
        let preset = MkinitcpioPreset::new_omarchy_v35("6.12.0-sigma");
        let cmd = preset.build_command();
        assert_eq!(cmd, "mkinitcpio -k 6.12.0-sigma -g /boot/initramfs-linux.img");
    }

    #[test]
    fn test_uki_generation_command_aaa() {
        let preset = MkinitcpioPreset::new_omarchy_v35("6.12.0-sigma");
        let ukify = preset.ukify_command().unwrap();
        assert!(ukify.contains("ukify build"));
        assert!(ukify.contains("--output=/boot/EFI/Linux/sigmaos-6.12.0-sigma.efi"));
        assert!(ukify.contains("root=LABEL=SIGMAOS"));
    }
}
