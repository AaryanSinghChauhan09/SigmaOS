//! SigmaOS system bootloader integration.
//!
//! Provides in-process backends for systemd-boot (Linux) and BSD loader.conf,
//! following the platform-contract installation-flow step 5.
//!
//! Upstream inspiration:
//! - systemd-boot Boot Loader Specification: <https://systemd.io/BOOT_LOADER_SPECIFICATION/>
//! - GRUB:         <https://www.gnu.org/software/grub/>
//! - archinstall:  <https://github.com/archlinux/archinstall>

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

// ---------------------------------------------------------------------------
// Core types
// ---------------------------------------------------------------------------

/// A single boot entry in the bootloader menu.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootEntry {
    /// Display name shown in the menu.
    pub title: String,
    /// Path to the kernel image (e.g. `/boot/vmlinuz-linux`).
    pub kernel: String,
    /// Optional initramfs path (e.g. `/boot/initramfs-linux.img`).
    pub initramfs: Option<String>,
    /// Kernel command-line arguments.
    pub cmdline: String,
    /// Whether this is the default entry.
    pub is_default: bool,
}

impl BootEntry {
    /// Create a standard Linux/BSD boot entry.
    pub fn new(title: &str, kernel: &str, initramfs: Option<&str>, cmdline: &str) -> Self {
        Self {
            title: title.to_string(),
            kernel: kernel.to_string(),
            initramfs: initramfs.map(|s| s.to_string()),
            cmdline: cmdline.to_string(),
            is_default: false,
        }
    }

    /// Render as a systemd-boot `.conf` snippet.
    pub fn as_systemd_boot_conf(&self) -> String {
        let mut out = format!("title   {}\nlinux   {}\n", self.title, self.kernel);
        if let Some(ref initrd) = self.initramfs {
            out.push_str(&format!("initrd  {}\n", initrd));
        }
        out.push_str(&format!("options {}\n", self.cmdline));
        out
    }
}

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Errors returned by bootloader operations.
#[derive(Debug, PartialEq, Eq)]
pub enum BootError {
    /// The bootloader config could not be parsed.
    ParseError(String),
    /// The bootloader was not installed.
    NotInstalled,
    /// The requested entry was not found.
    EntryNotFound,
    /// A duplicate entry title was detected.
    DuplicateEntry(String),
}

impl std::fmt::Display for BootError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BootError::ParseError(msg) => write!(f, "parse error: {msg}"),
            BootError::NotInstalled => write!(f, "bootloader not installed"),
            BootError::EntryNotFound => write!(f, "entry not found"),
            BootError::DuplicateEntry(t) => write!(f, "duplicate entry: {t}"),
        }
    }
}

impl std::error::Error for BootError {}

// ---------------------------------------------------------------------------
// Shared trait
// ---------------------------------------------------------------------------

/// Shared interface for all bootloader backends.
pub trait BootloaderConfig {
    /// Read all configured boot entries.
    fn entries(&self) -> Result<Vec<BootEntry>, BootError>;
    /// Set the default entry by title.
    fn set_default(&mut self, title: &str) -> Result<(), BootError>;
    /// Write the bootloader configuration (clears dirty flag).
    fn update(&mut self) -> Result<(), BootError>;
    /// Add a new boot entry; returns `DuplicateEntry` if title exists.
    fn add_entry(&mut self, entry: BootEntry) -> Result<(), BootError>;
    /// Remove a boot entry by title.
    fn remove_entry(&mut self, title: &str) -> Result<(), BootError>;
}

// ---------------------------------------------------------------------------
// systemd-boot backend (Linux)
// ---------------------------------------------------------------------------

/// In-process backend for systemd-boot (Boot Loader Specification).
pub struct SystemdBootBackend {
    entries: BTreeMap<String, BootEntry>,
    default: Option<String>,
    dirty: bool,
}

impl SystemdBootBackend {
    /// Create a new empty backend.
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            default: None,
            dirty: false,
        }
    }

    /// Return the default entry title, if set.
    pub fn default_title(&self) -> Option<&str> {
        self.default.as_deref()
    }

    /// Return `true` if there are unsaved changes.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Render `loader.conf` content (timeout + default entry).
    pub fn loader_conf(&self) -> String {
        let mut out = "timeout 5\n".to_string();
        if let Some(ref d) = self.default {
            out.push_str(&format!("default {}\n", d));
        }
        out
    }
}

impl Default for SystemdBootBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl BootloaderConfig for SystemdBootBackend {
    fn entries(&self) -> Result<Vec<BootEntry>, BootError> {
        Ok(self.entries.values().cloned().collect())
    }

    fn set_default(&mut self, title: &str) -> Result<(), BootError> {
        if !self.entries.contains_key(title) {
            return Err(BootError::EntryNotFound);
        }
        for e in self.entries.values_mut() {
            e.is_default = false;
        }
        if let Some(e) = self.entries.get_mut(title) {
            e.is_default = true;
        }
        self.default = Some(title.to_string());
        self.dirty = true;
        Ok(())
    }

    fn update(&mut self) -> Result<(), BootError> {
        self.dirty = false;
        Ok(())
    }

    fn add_entry(&mut self, entry: BootEntry) -> Result<(), BootError> {
        if self.entries.contains_key(&entry.title) {
            return Err(BootError::DuplicateEntry(entry.title.clone()));
        }
        self.entries.insert(entry.title.clone(), entry);
        self.dirty = true;
        Ok(())
    }

    fn remove_entry(&mut self, title: &str) -> Result<(), BootError> {
        if self.entries.remove(title).is_none() {
            return Err(BootError::EntryNotFound);
        }
        if self.default.as_deref() == Some(title) {
            self.default = None;
        }
        self.dirty = true;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// BSD loader.conf backend
// ---------------------------------------------------------------------------

/// In-process backend for BSD's `loader.conf` boot configuration.
pub struct BsdLoaderBackend {
    entries: BTreeMap<String, BootEntry>,
    default: Option<String>,
    dirty: bool,
}

impl BsdLoaderBackend {
    /// Create a new empty BSD loader backend.
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            default: None,
            dirty: false,
        }
    }

    /// Render a `loader.conf` snippet for this configuration.
    pub fn loader_conf(&self) -> String {
        let mut out = String::from("# SigmaOS BSD bootloader config\n");
        if let Some(ref d) = self.default {
            if let Some(e) = self.entries.get(d) {
                out.push_str(&format!("kernel=\"{}\"\n", e.kernel));
            }
        }
        out
    }
}

impl Default for BsdLoaderBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl BootloaderConfig for BsdLoaderBackend {
    fn entries(&self) -> Result<Vec<BootEntry>, BootError> {
        Ok(self.entries.values().cloned().collect())
    }

    fn set_default(&mut self, title: &str) -> Result<(), BootError> {
        if !self.entries.contains_key(title) {
            return Err(BootError::EntryNotFound);
        }
        for e in self.entries.values_mut() {
            e.is_default = false;
        }
        if let Some(e) = self.entries.get_mut(title) {
            e.is_default = true;
        }
        self.default = Some(title.to_string());
        self.dirty = true;
        Ok(())
    }

    fn update(&mut self) -> Result<(), BootError> {
        self.dirty = false;
        Ok(())
    }

    fn add_entry(&mut self, entry: BootEntry) -> Result<(), BootError> {
        if self.entries.contains_key(&entry.title) {
            return Err(BootError::DuplicateEntry(entry.title.clone()));
        }
        self.entries.insert(entry.title.clone(), entry);
        self.dirty = true;
        Ok(())
    }

    fn remove_entry(&mut self, title: &str) -> Result<(), BootError> {
        if self.entries.remove(title).is_none() {
            return Err(BootError::EntryNotFound);
        }
        if self.default.as_deref() == Some(title) {
            self.default = None;
        }
        self.dirty = true;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Inline tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sigma_entry() -> BootEntry {
        BootEntry::new(
            "SigmaOS Linux",
            "/boot/vmlinuz-linux",
            Some("/boot/initramfs-linux.img"),
            "root=/dev/sda2 rw quiet",
        )
    }

    fn fallback_entry() -> BootEntry {
        BootEntry::new(
            "SigmaOS Fallback",
            "/boot/vmlinuz-linux",
            Some("/boot/initramfs-linux-fallback.img"),
            "root=/dev/sda2 rw",
        )
    }

    #[test]
    fn systemd_add_and_list_entries() {
        let mut bl = SystemdBootBackend::new();
        bl.add_entry(sigma_entry()).unwrap();
        bl.add_entry(fallback_entry()).unwrap();
        assert_eq!(bl.entries().unwrap().len(), 2);
    }

    #[test]
    fn systemd_duplicate_entry_errors() {
        let mut bl = SystemdBootBackend::new();
        bl.add_entry(sigma_entry()).unwrap();
        assert!(matches!(
            bl.add_entry(sigma_entry()).unwrap_err(),
            BootError::DuplicateEntry(_)
        ));
    }

    #[test]
    fn systemd_set_default_marks_entry() {
        let mut bl = SystemdBootBackend::new();
        bl.add_entry(sigma_entry()).unwrap();
        bl.add_entry(fallback_entry()).unwrap();
        bl.set_default("SigmaOS Linux").unwrap();
        assert_eq!(bl.default_title(), Some("SigmaOS Linux"));
        let entries = bl.entries().unwrap();
        let main = entries.iter().find(|e| e.title == "SigmaOS Linux").unwrap();
        assert!(main.is_default);
    }

    #[test]
    fn systemd_set_default_unknown_errors() {
        let mut bl = SystemdBootBackend::new();
        bl.add_entry(sigma_entry()).unwrap();
        assert_eq!(bl.set_default("nonexistent"), Err(BootError::EntryNotFound));
    }

    #[test]
    fn systemd_remove_entry() {
        let mut bl = SystemdBootBackend::new();
        bl.add_entry(sigma_entry()).unwrap();
        bl.add_entry(fallback_entry()).unwrap();
        bl.remove_entry("SigmaOS Fallback").unwrap();
        assert_eq!(bl.entries().unwrap().len(), 1);
    }

    #[test]
    fn systemd_remove_default_clears_default() {
        let mut bl = SystemdBootBackend::new();
        bl.add_entry(sigma_entry()).unwrap();
        bl.set_default("SigmaOS Linux").unwrap();
        bl.remove_entry("SigmaOS Linux").unwrap();
        assert_eq!(bl.default_title(), None);
    }

    #[test]
    fn systemd_update_clears_dirty() {
        let mut bl = SystemdBootBackend::new();
        bl.add_entry(sigma_entry()).unwrap();
        assert!(bl.is_dirty());
        bl.update().unwrap();
        assert!(!bl.is_dirty());
    }

    #[test]
    fn systemd_boot_conf_format() {
        let conf = sigma_entry().as_systemd_boot_conf();
        assert!(conf.contains("title   SigmaOS Linux"));
        assert!(conf.contains("linux   /boot/vmlinuz-linux"));
        assert!(conf.contains("initrd  /boot/initramfs-linux.img"));
        assert!(conf.contains("options root=/dev/sda2"));
    }

    #[test]
    fn systemd_loader_conf_includes_default() {
        let mut bl = SystemdBootBackend::new();
        bl.add_entry(sigma_entry()).unwrap();
        bl.set_default("SigmaOS Linux").unwrap();
        let conf = bl.loader_conf();
        assert!(conf.contains("default SigmaOS Linux"));
        assert!(conf.contains("timeout 5"));
    }

    #[test]
    fn bsd_add_and_list() {
        let mut bl = BsdLoaderBackend::new();
        bl.add_entry(BootEntry::new("FreeBSD", "/boot/kernel/kernel", None, ""))
            .unwrap();
        assert_eq!(bl.entries().unwrap().len(), 1);
    }

    #[test]
    fn bsd_loader_conf_renders_kernel() {
        let mut bl = BsdLoaderBackend::new();
        bl.add_entry(BootEntry::new("FreeBSD", "/boot/kernel/kernel", None, ""))
            .unwrap();
        bl.set_default("FreeBSD").unwrap();
        assert!(bl.loader_conf().contains("kernel=\"/boot/kernel/kernel\""));
    }

    #[test]
    fn boot_error_display() {
        assert!(BootError::ParseError("bad".into()).to_string().contains("parse error"));
        assert!(BootError::NotInstalled.to_string().contains("not installed"));
        assert!(BootError::EntryNotFound.to_string().contains("not found"));
        assert!(BootError::DuplicateEntry("x".into()).to_string().contains("duplicate"));
    }
}
