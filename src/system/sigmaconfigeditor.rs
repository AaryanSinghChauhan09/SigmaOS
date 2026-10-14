// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaConfigEditor - System Configuration Editor (/etc Management)
//!
//! Inspired by etckeeper, Linux /etc configuration management, and sysconfig editors.
//! Provides structured parsing, modification, revision tracking, rollback,
//! validation, and diff generation for core operating system configuration files.

#![no_std]
#![allow(dead_code)]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Configuration file format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigFormat {
    /// KEY=VALUE or KEY="VALUE" format (e.g. /etc/os-release, /etc/default/*)
    KeyValue,
    /// INI-style sections [section] with key=value pairs (e.g. systemd.conf, smb.conf)
    Ini,
    /// Space/tab separated table format (e.g. /etc/hosts, /etc/fstab)
    Table,
    /// Single-entry or line-delimited format (e.g. /etc/hostname, /etc/resolv.conf)
    LineDelimited,
    /// Plain unstructured text format
    PlainText,
}

/// Operational state of a managed configuration file
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileState {
    /// File has not been modified since loading
    Pristine,
    /// In-memory edits have been made but not committed to history
    Modified,
    /// Changes validated and committed to revision history
    Committed,
    /// Syntax or semantic error detected in configuration
    Error,
}

/// Validation result for configuration content
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationResult {
    Valid,
    Warning(String),
    Invalid(String),
}

/// A historical revision of a configuration file
#[derive(Debug, Clone)]
pub struct ConfigRevision {
    pub revision_id: u64,
    pub timestamp: u64,
    pub author: String,
    pub message: String,
    pub content: String,
}

/// A managed system configuration file (e.g., in /etc)
#[derive(Debug, Clone)]
pub struct ManagedFile {
    pub path: String,
    pub format: ConfigFormat,
    pub current_content: String,
    pub original_content: String,
    pub state: FileState,
    pub revisions: Vec<ConfigRevision>,
    pub properties: BTreeMap<String, String>,
    next_revision_id: u64,
}

impl ManagedFile {
    /// Create a new managed configuration file
    pub fn new(path: String, format: ConfigFormat, initial_content: String) -> Self {
        let mut file = Self {
            path,
            format,
            current_content: initial_content.clone(),
            original_content: initial_content,
            state: FileState::Pristine,
            revisions: Vec::new(),
            properties: BTreeMap::new(),
            next_revision_id: 1,
        };
        file.parse_properties();
        file
    }

    /// Parse key-value properties from current content
    fn parse_properties(&mut self) {
        self.properties.clear();
        match self.format {
            ConfigFormat::KeyValue | ConfigFormat::Ini => {
                for line in self.current_content.lines() {
                    let trimmed = line.trim();
                    if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
                        continue;
                    }
                    if trimmed.starts_with('[') && trimmed.ends_with(']') {
                        continue;
                    }
                    if let Some(pos) = trimmed.find('=') {
                        let key = trimmed[..pos].trim().to_string();
                        let mut val = trimmed[pos + 1..].trim();
                        if (val.starts_with('"') && val.ends_with('"'))
                            || (val.starts_with('\'') && val.ends_with('\''))
                        {
                            if val.len() >= 2 {
                                val = &val[1..val.len() - 1];
                            }
                        }
                        self.properties.insert(key, val.to_string());
                    }
                }
            }
            ConfigFormat::Table => {
                for (idx, line) in self.current_content.lines().enumerate() {
                    let trimmed = line.trim();
                    if trimmed.is_empty() || trimmed.starts_with('#') {
                        continue;
                    }
                    let key = format!("entry_{}", idx);
                    self.properties.insert(key, trimmed.to_string());
                }
            }
            ConfigFormat::LineDelimited | ConfigFormat::PlainText => {
                for (idx, line) in self.current_content.lines().enumerate() {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        self.properties.insert(format!("line_{}", idx), trimmed.to_string());
                    }
                }
            }
        }
    }

    /// Rebuild content from properties for KeyValue files
    fn rebuild_key_value_content(&mut self) {
        let mut new_content = String::new();
        for (key, val) in &self.properties {
            new_content.push_str(key);
            new_content.push('=');
            if val.contains(' ') || val.is_empty() {
                new_content.push('"');
                new_content.push_str(val);
                new_content.push('"');
            } else {
                new_content.push_str(val);
            }
            new_content.push('\n');
        }
        self.current_content = new_content;
    }

    /// Get a property value by key
    pub fn get_property(&self, key: &str) -> Option<&str> {
        self.properties.get(key).map(|s| s.as_str())
    }

    /// Set a property value by key
    pub fn set_property(&mut self, key: &str, value: &str) {
        self.properties.insert(key.to_string(), value.to_string());
        if self.format == ConfigFormat::KeyValue {
            self.rebuild_key_value_content();
        }
        self.state = FileState::Modified;
    }

    /// Remove a property by key
    pub fn remove_property(&mut self, key: &str) -> Option<String> {
        let removed = self.properties.remove(key);
        if removed.is_some() {
            if self.format == ConfigFormat::KeyValue {
                self.rebuild_key_value_content();
            }
            self.state = FileState::Modified;
        }
        removed
    }

    /// Set raw content directly
    pub fn set_content(&mut self, new_content: String) {
        self.current_content = new_content;
        self.parse_properties();
        self.state = FileState::Modified;
    }

    /// Commit the current state as a historical revision
    pub fn commit_revision(&mut self, author: String, message: String, timestamp: u64) -> u64 {
        let rev_id = self.next_revision_id;
        self.next_revision_id += 1;

        let revision = ConfigRevision {
            revision_id: rev_id,
            timestamp,
            author,
            message,
            content: self.current_content.clone(),
        };

        self.revisions.push(revision);
        self.state = FileState::Committed;
        rev_id
    }

    /// Roll back content to a previous revision
    pub fn rollback(&mut self, revision_id: u64) -> bool {
        if let Some(rev) = self.revisions.iter().find(|r| r.revision_id == revision_id) {
            self.current_content = rev.content.clone();
            self.parse_properties();
            self.state = FileState::Modified;
            true
        } else {
            false
        }
    }

    /// Reset content to original content at initialization
    pub fn reset_to_original(&mut self) {
        self.current_content = self.original_content.clone();
        self.parse_properties();
        self.state = FileState::Pristine;
    }

    /// Generate unified-style diff between original and current content
    pub fn generate_diff(&self) -> String {
        let mut diff = String::new();
        diff.push_str("--- a");
        diff.push_str(&self.path);
        diff.push('\n');
        diff.push_str("+++ b");
        diff.push_str(&self.path);
        diff.push('\n');

        let orig_lines: Vec<&str> = self.original_content.lines().collect();
        let curr_lines: Vec<&str> = self.current_content.lines().collect();

        for line in &orig_lines {
            if !curr_lines.contains(line) {
                diff.push('-');
                diff.push_str(line);
                diff.push('\n');
            }
        }
        for line in &curr_lines {
            if !orig_lines.contains(line) {
                diff.push('+');
                diff.push_str(line);
                diff.push('\n');
            }
        }

        diff
    }

    /// Validate syntax based on file format
    pub fn validate(&self) -> ValidationResult {
        match self.format {
            ConfigFormat::KeyValue => {
                for (idx, line) in self.current_content.lines().enumerate() {
                    let trimmed = line.trim();
                    if trimmed.is_empty() || trimmed.starts_with('#') {
                        continue;
                    }
                    if !trimmed.contains('=') {
                        return ValidationResult::Invalid(format!(
                            "Line {} missing '=' delimiter: {}",
                            idx + 1,
                            trimmed
                        ));
                    }
                }
                ValidationResult::Valid
            }
            ConfigFormat::Ini => {
                let mut in_section = false;
                for (idx, line) in self.current_content.lines().enumerate() {
                    let trimmed = line.trim();
                    if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
                        continue;
                    }
                    if trimmed.starts_with('[') {
                        if !trimmed.ends_with(']') {
                            return ValidationResult::Invalid(format!(
                                "Line {} unclosed section bracket: {}",
                                idx + 1,
                                trimmed
                            ));
                        }
                        in_section = true;
                    } else if in_section && !trimmed.contains('=') {
                        return ValidationResult::Invalid(format!(
                            "Line {} invalid entry outside key=value: {}",
                            idx + 1,
                            trimmed
                        ));
                    }
                }
                ValidationResult::Valid
            }
            ConfigFormat::Table => {
                for line in self.current_content.lines() {
                    let trimmed = line.trim();
                    if trimmed.is_empty() || trimmed.starts_with('#') {
                        continue;
                    }
                    let cols: Vec<&str> = trimmed.split_whitespace().collect();
                    if cols.is_empty() {
                        return ValidationResult::Invalid("Malformed table entry".to_string());
                    }
                }
                ValidationResult::Valid
            }
            ConfigFormat::LineDelimited | ConfigFormat::PlainText => {
                if self.current_content.is_empty() {
                    ValidationResult::Warning("Empty configuration file".to_string())
                } else {
                    ValidationResult::Valid
                }
            }
        }
    }
}

/// System Configuration Editor (/etc Manager)
#[derive(Debug)]
pub struct SigmaConfigEditor {
    files: BTreeMap<String, ManagedFile>,
    backup_path: String,
    running: bool,
}

impl SigmaConfigEditor {
    /// Create a new configuration editor
    pub fn new() -> Self {
        Self {
            files: BTreeMap::new(),
            backup_path: "/etc/.sigma_backup".to_string(),
            running: false,
        }
    }

    /// Start the editor daemon
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the editor daemon
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Register a configuration file for tracking and editing
    pub fn register_file(
        &mut self,
        path: &str,
        format: ConfigFormat,
        initial_content: &str,
    ) -> bool {
        if self.files.contains_key(path) {
            return false;
        }
        let file = ManagedFile::new(path.to_string(), format, initial_content.to_string());
        self.files.insert(path.to_string(), file);
        true
    }

    /// Unregister a configuration file
    pub fn unregister_file(&mut self, path: &str) -> bool {
        self.files.remove(path).is_some()
    }

    /// Get managed file reference
    pub fn get_file(&self, path: &str) -> Option<&ManagedFile> {
        self.files.get(path)
    }

    /// Get managed file mutable reference
    pub fn get_file_mut(&mut self, path: &str) -> Option<&mut ManagedFile> {
        self.files.get_mut(path)
    }

    /// Set file content and automatically commit revision
    pub fn update_file_content(
        &mut self,
        path: &str,
        content: &str,
        author: &str,
        message: &str,
        timestamp: u64,
    ) -> bool {
        if let Some(file) = self.files.get_mut(path) {
            file.set_content(content.to_string());
            file.commit_revision(author.to_string(), message.to_string(), timestamp);
            true
        } else {
            false
        }
    }

    /// Set a key-value property in a configuration file
    pub fn set_property(
        &mut self,
        path: &str,
        key: &str,
        value: &str,
        author: &str,
        message: &str,
        timestamp: u64,
    ) -> bool {
        if let Some(file) = self.files.get_mut(path) {
            file.set_property(key, value);
            file.commit_revision(author.to_string(), message.to_string(), timestamp);
            true
        } else {
            false
        }
    }

    /// Roll back a file to a specific revision
    pub fn rollback_file(&mut self, path: &str, revision_id: u64) -> bool {
        if let Some(file) = self.files.get_mut(path) {
            file.rollback(revision_id)
        } else {
            false
        }
    }

    /// Generate diff for a managed file
    pub fn diff_file(&self, path: &str) -> Option<String> {
        self.files.get(path).map(|f| f.generate_diff())
    }

    /// Validate a managed file
    pub fn validate_file(&self, path: &str) -> Option<ValidationResult> {
        self.files.get(path).map(|f| f.validate())
    }

    /// Count tracked files
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    /// List all tracked file paths
    pub fn list_files(&self) -> Vec<&str> {
        self.files.keys().map(|k| k.as_str()).collect()
    }

    /// List modified files that have uncommitted changes
    pub fn list_modified_files(&self) -> Vec<&str> {
        self.files
            .iter()
            .filter(|(_, f)| f.state == FileState::Modified)
            .map(|(k, _)| k.as_str())
            .collect()
    }
}

impl Default for SigmaConfigEditor {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// Global Singleton & Convenience Functions
// ============================================================

static mut CONFIG_EDITOR_INSTANCE: Option<SigmaConfigEditor> = None;

/// Initialize global SigmaConfigEditor instance
pub fn init_sigmaconfigeditor() {
    unsafe {
        CONFIG_EDITOR_INSTANCE = Some(SigmaConfigEditor::new());
        if let Some(ref mut editor) = CONFIG_EDITOR_INSTANCE {
            editor.start();
        }
    }
}

/// Get global SigmaConfigEditor reference
pub fn get_sigmaconfigeditor() -> Option<&'static SigmaConfigEditor> {
    unsafe { CONFIG_EDITOR_INSTANCE.as_ref() }
}

/// Get mutable global SigmaConfigEditor reference
pub fn get_sigmaconfigeditor_mut() -> Option<&'static mut SigmaConfigEditor> {
    unsafe { CONFIG_EDITOR_INSTANCE.as_mut() }
}

/// Convenience function to register a file
pub fn register_config_file(path: &str, format: ConfigFormat, content: &str) -> bool {
    if let Some(editor) = get_sigmaconfigeditor_mut() {
        editor.register_file(path, format, content)
    } else {
        false
    }
}

/// Convenience function to get a property from a file
pub fn get_config_property(path: &str, key: &str) -> Option<String> {
    if let Some(editor) = get_sigmaconfigeditor() {
        editor.get_file(path).and_then(|f| f.get_property(key).map(|s| s.to_string()))
    } else {
        None
    }
}

/// Convenience function to set a property in a file
pub fn set_config_property(
    path: &str,
    key: &str,
    value: &str,
    author: &str,
    message: &str,
) -> bool {
    if let Some(editor) = get_sigmaconfigeditor_mut() {
        editor.set_property(path, key, value, author, message, 0)
    } else {
        false
    }
}

/// Convenience function to rollback a file
pub fn rollback_config_file(path: &str, revision_id: u64) -> bool {
    if let Some(editor) = get_sigmaconfigeditor_mut() {
        editor.rollback_file(path, revision_id)
    } else {
        false
    }
}

// ============================================================
// Unit Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_managed_file_key_value() {
        let content = "HOSTNAME=sigmaos-node1\nDOMAIN=\"localdomain\"\nDEBUG=true\n";
        let mut file = ManagedFile::new("/etc/sysconfig/network".to_string(), ConfigFormat::KeyValue, content.to_string());

        assert_eq!(file.get_property("HOSTNAME"), Some("sigmaos-node1"));
        assert_eq!(file.get_property("DOMAIN"), Some("localdomain"));
        assert_eq!(file.get_property("DEBUG"), Some("true"));
        assert_eq!(file.state, FileState::Pristine);

        file.set_property("HOSTNAME", "sigmaos-master");
        assert_eq!(file.get_property("HOSTNAME"), Some("sigmaos-master"));
        assert_eq!(file.state, FileState::Modified);

        let rev_id = file.commit_revision("admin".to_string(), "Update hostname".to_string(), 100);
        assert_eq!(rev_id, 1);
        assert_eq!(file.state, FileState::Committed);

        assert!(file.rollback(rev_id));
        assert_eq!(file.get_property("HOSTNAME"), Some("sigmaos-master"));
    }

    #[test]
    fn test_managed_file_validation() {
        let valid_kv = "KEY1=VAL1\nKEY2=VAL2\n";
        let file = ManagedFile::new("/etc/test.conf".to_string(), ConfigFormat::KeyValue, valid_kv.to_string());
        assert_eq!(file.validate(), ValidationResult::Valid);

        let invalid_kv = "KEY1=VAL1\nINVALID_LINE_NO_EQUALS\n";
        let bad_file = ManagedFile::new("/etc/test.conf".to_string(), ConfigFormat::KeyValue, invalid_kv.to_string());
        match bad_file.validate() {
            ValidationResult::Invalid(_) => {}
            _ => panic!("Expected invalid result"),
        }
    }

    #[test]
    fn test_managed_file_diff() {
        let orig = "LINE1\nLINE2\n";
        let mut file = ManagedFile::new("/etc/sample.conf".to_string(), ConfigFormat::PlainText, orig.to_string());
        file.set_content("LINE1\nLINE3\n".to_string());

        let diff = file.generate_diff();
        assert!(diff.contains("-LINE2"));
        assert!(diff.contains("+LINE3"));
    }

    #[test]
    fn test_editor_lifecycle() {
        let mut editor = SigmaConfigEditor::new();
        editor.start();
        assert!(editor.is_running());

        let reg = editor.register_file(
            "/etc/hostname",
            ConfigFormat::LineDelimited,
            "sigma-box\n",
        );
        assert!(reg);
        assert_eq!(editor.file_count(), 1);

        let dup = editor.register_file(
            "/etc/hostname",
            ConfigFormat::LineDelimited,
            "sigma-box\n",
        );
        assert!(!dup);

        let updated = editor.update_file_content(
            "/etc/hostname",
            "sigma-server\n",
            "root",
            "Change hostname",
            100,
        );
        assert!(updated);

        let file = editor.get_file("/etc/hostname").unwrap();
        assert_eq!(file.current_content, "sigma-server\n");
        assert_eq!(file.revisions.len(), 1);

        assert!(editor.rollback_file("/etc/hostname", 1));

        editor.stop();
        assert!(!editor.is_running());
    }

    #[test]
    fn test_global_singleton() {
        init_sigmaconfigeditor();
        assert!(get_sigmaconfigeditor().is_some());
        assert!(get_sigmaconfigeditor_mut().is_some());

        assert!(register_config_file(
            "/etc/os-release",
            ConfigFormat::KeyValue,
            "NAME=SigmaOS\nVERSION=1.0\n",
        ));

        assert_eq!(
            get_config_property("/etc/os-release", "NAME"),
            Some("SigmaOS".to_string())
        );

        assert!(set_config_property(
            "/etc/os-release",
            "VERSION",
            "2.0",
            "builder",
            "Bump version",
        ));

        assert_eq!(
            get_config_property("/etc/os-release", "VERSION"),
            Some("2.0".to_string())
        );
    }
}
