//! SigmaOS System & User Data Backup Tool (`mintbackup` counterpart)
//!
//! Backup planning and package-selection manifest model inspired by Linux Mint.
//!
//! Package manifest export/import is implemented. Personal-data archive and
//! restore backends are not, so those operations fail without claiming success.

#![allow(dead_code)]

use std::collections::BTreeSet;
use std::string::String;
use std::vec::Vec;

/// Backup scope type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupScope {
    PersonalDataOnly,
    SoftwareSelectionOnly,
    CompleteHybridBackup,
}

/// Software selection manifest entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageSelectionEntry {
    pub package_name: String,
    pub version: String,
    pub origin_repo: String,
    pub is_user_explicit: bool, // true if user explicitly installed it, false if auto-dependency
}

/// Personal data archive metadata
#[derive(Debug, Clone)]
pub struct BackupArchiveHeader {
    pub archive_id: String,
    pub created_timestamp: u64,
    pub scope: BackupScope,
    pub total_files_count: usize,
    pub uncompressed_bytes: u64,
    pub compressed_bytes: u64,
    pub sha256_checksum: String,
}

/// Linux Mint-Inspired Backup Tool Engine
pub struct MintBackupToolEngine {
    pub default_exclusions: BTreeSet<String>,
    tracked_installed_packages: Vec<PackageSelectionEntry>,
    pub past_backups: Vec<BackupArchiveHeader>,
}

impl MintBackupToolEngine {
    pub fn new() -> Self {
        let mut exclusions = BTreeSet::new();
        exclusions.insert("~/.cache/*".into());
        exclusions.insert("~/.local/share/Trash/*".into());
        exclusions.insert("**/node_modules/*".into());
        exclusions.insert("**/.git/*".into());
        exclusions.insert("**/target/debug/*".into());

        Self {
            default_exclusions: exclusions,
            tracked_installed_packages: Vec::new(),
            past_backups: Vec::new(),
        }
    }

    /// Add a software package to the system manifest
    pub fn register_installed_package(
        &mut self,
        name: &str,
        version: &str,
        repo: &str,
        is_explicit: bool,
    ) -> Result<(), &'static str> {
        if !is_manifest_token(name) || !is_manifest_token(version) || !is_manifest_token(repo) {
            return Err("Package manifest fields must be non-empty safe tokens");
        }
        if self
            .tracked_installed_packages
            .iter()
            .any(|entry| entry.package_name == name)
        {
            return Err("Package is already registered in the manifest");
        }
        self.tracked_installed_packages.push(PackageSelectionEntry {
            package_name: name.to_string(),
            version: version.to_string(),
            origin_repo: repo.to_string(),
            is_user_explicit: is_explicit,
        });
        Ok(())
    }

    /// Export a deterministic, tab-separated software selection manifest.
    pub fn export_software_manifest(&self) -> String {
        let mut out = String::from(
            "# SigmaOS package selection manifest v1\n# package\tversion\trepository\tselection\n",
        );
        for pkg in &self.tracked_installed_packages {
            if !pkg.is_user_explicit {
                continue;
            }
            out.push_str(&format!(
                "{}\t{}\t{}\texplicit\n",
                pkg.package_name, pkg.version, pkg.origin_repo
            ));
        }
        out
    }

    /// Personal-data archive creation requires a real archive backend.
    pub fn create_personal_data_backup(
        &mut self,
        source_dir: &str,
        dest_archive_path: &str,
        _custom_exclusions: &[String],
        _timestamp: u64,
    ) -> Result<BackupArchiveHeader, &'static str> {
        if source_dir.trim().is_empty() || dest_archive_path.trim().is_empty() {
            return Err("Source or destination path is empty");
        }
        Err("Personal-data archive backend is unavailable")
    }

    /// Parse a v1 package manifest and return only explicitly selected packages.
    /// Malformed rows fail the entire import; no partial install list is returned.
    pub fn restore_software_selection(
        &self,
        manifest_content: &str,
    ) -> Result<Vec<String>, &'static str> {
        let mut packages_to_install = Vec::new();
        let mut seen = BTreeSet::new();
        for line in manifest_content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = trimmed.split('\t').collect();
            if parts.len() != 4
                || !is_manifest_token(parts[0])
                || !is_manifest_token(parts[1])
                || !is_manifest_token(parts[2])
                || !matches!(parts[3], "explicit" | "dependency")
            {
                return Err("Malformed package selection manifest row");
            }
            if !seen.insert(parts[0]) {
                return Err("Duplicate package in selection manifest");
            }
            if parts[3] == "explicit" {
                packages_to_install.push(parts[0].to_string());
            }
        }
        Ok(packages_to_install)
    }
}

fn is_manifest_token(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 256
        && value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'+' | b':' | b'@')
        })
}

impl Default for MintBackupToolEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_software_manifest_export_and_restore() {
        let mut tool = MintBackupToolEngine::new();
        tool.register_installed_package("alacritty", "0.13.0", "core", true)
            .unwrap();
        tool.register_installed_package("zenith-compositor", "1.2.0", "desktop", true)
            .unwrap();
        tool.register_installed_package("libssl", "3.2.0", "system", false)
            .unwrap(); // auto-dependency, should be skipped

        let manifest = tool.export_software_manifest();
        assert!(manifest.contains("alacritty"));
        assert!(manifest.contains("zenith-compositor"));
        assert!(!manifest.contains("libssl"));

        let restored = tool.restore_software_selection(&manifest).unwrap();
        assert_eq!(restored.len(), 2);
        assert_eq!(restored[0], "alacritty");
        assert_eq!(restored[1], "zenith-compositor");
    }

    #[test]
    fn test_personal_data_backup_fails_without_backend_without_fabricated_state() {
        let mut tool = MintBackupToolEngine::new();
        let res = tool.create_personal_data_backup(
            "/home/user",
            "/media/backup/user_backup.tar.zst",
            &["*.iso".into()],
            1727260800,
        );
        assert!(matches!(
            res,
            Err("Personal-data archive backend is unavailable")
        ));
        assert!(tool.past_backups.is_empty());
    }

    #[test]
    fn test_manifest_fields_reject_injection_and_duplicate_names() {
        let mut tool = MintBackupToolEngine::new();
        assert!(tool
            .register_installed_package("good\nattacker", "1.0", "core", true)
            .is_err());
        tool.register_installed_package("good", "1.0", "core", true)
            .unwrap();
        assert!(tool
            .register_installed_package("good", "2.0", "core", true)
            .is_err());
    }

    #[test]
    fn test_manifest_import_rejects_malformed_and_duplicate_rows() {
        let tool = MintBackupToolEngine::new();
        assert!(tool
            .restore_software_selection("pkg\t1.0\tcore\texplicit\nmalformed")
            .is_err());
        assert!(tool
            .restore_software_selection("pkg\t1.0\tcore\texplicit\npkg\t2.0\tcore\texplicit")
            .is_err());
    }
}
