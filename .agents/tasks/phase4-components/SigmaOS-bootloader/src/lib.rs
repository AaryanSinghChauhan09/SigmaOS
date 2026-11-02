//! SigmaOS system bootloader integration.
//!
//! Manages GRUB configuration, kernel selection, and boot entries.

#![warn(missing_docs)]

/// A single boot entry in the bootloader menu.
#[derive(Debug, Clone)]
pub struct BootEntry {
    /// Display name of the entry.
    pub title: String,
    /// Path to the kernel image.
    pub kernel: String,
    /// Optional initramfs path.
    pub initramfs: Option<String>,
    /// Kernel command-line arguments.
    pub cmdline: String,
}

/// Trait for bootloader configuration backends.
pub trait BootloaderConfig {
    /// Read all configured boot entries.
    fn entries(&self) -> Result<Vec<BootEntry>, BootError>;
    /// Set the default entry by title.
    fn set_default(&mut self, title: &str) -> Result<(), BootError>;
    /// Regenerate the bootloader config.
    fn update(&mut self) -> Result<(), BootError>;
}

/// Errors returned by the bootloader integration.
#[derive(Debug)]
pub enum BootError {
    /// The bootloader config could not be parsed.
    ParseError(String),
    /// The bootloader was not installed.
    NotInstalled,
    /// The requested entry was not found.
    EntryNotFound,
}

impl std::fmt::Display for BootError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BootError::ParseError(msg) => write!(f, "parse error: {msg}"),
            BootError::NotInstalled => write!(f, "bootloader not installed"),
            BootError::EntryNotFound => write!(f, "entry not found"),
        }
    }
}

impl std::error::Error for BootError {}
