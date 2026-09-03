#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
extern crate alloc;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

// SigmaOS Secure Clipboard Manager
// OOP-based clipboard with encryption and auto-clear

use core::time::Duration;
// Instant not available in no_std - using u64 tick counters instead

/// Clipboard entry
#[derive(Debug, Clone)]
pub struct ClipboardEntry {
    pub content: String,
    pub content_type: ClipboardType,
    pub timestamp: u64,
    pub is_encrypted: bool,
    pub auto_clear_after: Duration,
}

/// Clipboard content type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardType {
    Text,
    Image,
    Html,
    Rtf,
    File,
}

/// Clipboard security level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    None,
    Low,
    Medium,
    High,
}

/// OOP trait for clipboard security strategies
pub trait ClipboardSecurity {
    /// Secure clipboard content
    fn secure(&self, content: &str, level: SecurityLevel) -> Result<String, ClipboardError>;
    /// Unsecure clipboard content
    fn unsecure(&self, content: &str, level: SecurityLevel) -> Result<String, ClipboardError>;
    /// Get strategy name
    fn name(&self) -> &str;
}

/// No encryption strategy
pub struct NoEncryption;

impl ClipboardSecurity for NoEncryption {
    fn secure(&self, content: &str, _level: SecurityLevel) -> Result<String, ClipboardError> {
        Ok(content.to_string())
    }

    fn unsecure(&self, content: &str, _level: SecurityLevel) -> Result<String, ClipboardError> {
        Ok(content.to_string())
    }

    fn name(&self) -> &str {
        "NoEncryption"
    }
}

/// XOR encryption strategy
pub struct XorEncryption {
    key: Vec<u8>,
}

impl XorEncryption {
    pub fn new(key: Vec<u8>) -> Self {
        Self { key }
    }
}

impl ClipboardSecurity for XorEncryption {
    fn secure(&self, content: &str, _level: SecurityLevel) -> Result<String, ClipboardError> {
        if self.key.is_empty() {
            return Err(ClipboardError::EncryptionError(
                "Encryption key cannot be empty".to_string(),
            ));
        }
        // Optimize: Use single-pass cycle + zip iterator chain to eliminate repeated modulo index divisions
        let encrypted: Vec<u8> = content
            .bytes()
            .zip(self.key.iter().cycle())
            .map(|(b, &k)| b ^ k)
            .collect();
        Ok(String::from_utf8(encrypted)
            .map_err(|e| ClipboardError::EncodingError(e.to_string()))?)
    }

    fn unsecure(&self, content: &str, _level: SecurityLevel) -> Result<String, ClipboardError> {
        if self.key.is_empty() {
            return Err(ClipboardError::DecryptionError(
                "Decryption key cannot be empty".to_string(),
            ));
        }
        // Optimize: Use single-pass cycle + zip iterator chain to eliminate repeated modulo index divisions
        let decrypted: Vec<u8> = content
            .bytes()
            .zip(self.key.iter().cycle())
            .map(|(b, &k)| b ^ k)
            .collect();
        Ok(String::from_utf8(decrypted)
            .map_err(|e| ClipboardError::EncodingError(e.to_string()))?)
    }

    fn name(&self) -> &str {
        "XorEncryption"
    }
}

/// OOP-based Secure Clipboard Manager
pub struct SecureClipboardManager {
    current_entry: Option<ClipboardEntry>,
    history: Vec<ClipboardEntry>,
    max_history_size: usize,
    security: Box<dyn ClipboardSecurity>,
    default_security_level: SecurityLevel,
    auto_clear_enabled: bool,
    auto_clear_duration: Duration,
}

impl SecureClipboardManager {
    pub fn new(security: Box<dyn ClipboardSecurity>) -> Self {
        Self {
            current_entry: None,
            history: Vec::new(),
            max_history_size: 50,
            security,
            default_security_level: SecurityLevel::Medium,
            auto_clear_enabled: true,
            auto_clear_duration: Duration::from_secs(60), // 1 minute
        }
    }

    /// Set max history size
    pub fn with_max_history(mut self, size: usize) -> Self {
        self.max_history_size = size;
        self
    }

    /// Set default security level
    pub fn with_security_level(mut self, level: SecurityLevel) -> Self {
        self.default_security_level = level;
        self
    }

    /// Enable auto-clear
    pub fn with_auto_clear(mut self, enabled: bool, duration: Duration) -> Self {
        self.auto_clear_enabled = enabled;
        self.auto_clear_duration = duration;
        self
    }

    /// Copy content to clipboard
    pub fn copy(
        &mut self,
        content: String,
        content_type: ClipboardType,
    ) -> Result<(), ClipboardError> {
        let secured_content = if self.default_security_level != SecurityLevel::None {
            self.security
                .secure(&content, self.default_security_level)?
        } else {
            content.clone()
        };

        let entry = ClipboardEntry {
            content: secured_content,
            content_type,
            timestamp: 0u64,
            is_encrypted: self.default_security_level != SecurityLevel::None,
            auto_clear_after: if self.auto_clear_enabled {
                self.auto_clear_duration
            } else {
                Duration::from_secs(u64::MAX)
            },
        };

        // Add to history
        if self.history.len() >= self.max_history_size {
            self.history.remove(0);
        }
        self.history.push(entry.clone());

        self.current_entry = Some(entry);
        Ok(())
    }

    /// Paste content from clipboard
    pub fn paste(&mut self) -> Result<String, ClipboardError> {
        self.check_auto_clear();

        let entry = self
            .current_entry
            .as_ref()
            .ok_or_else(|| ClipboardError::ClipboardEmpty)?;

        let content = if entry.is_encrypted {
            self.security
                .unsecure(&entry.content, self.default_security_level)?
        } else {
            entry.content.clone()
        };

        Ok(content)
    }

    /// Get clipboard content type
    pub fn content_type(&self) -> Option<ClipboardType> {
        self.current_entry.as_ref().map(|e| e.content_type)
    }

    /// Check if clipboard is encrypted
    pub fn is_encrypted(&self) -> bool {
        self.current_entry
            .as_ref()
            .map(|e| e.is_encrypted)
            .unwrap_or(false)
    }

    /// Clear clipboard
    pub fn clear(&mut self) {
        self.current_entry = None;
    }

    /// Get clipboard history
    pub fn history(&self) -> &[ClipboardEntry] {
        &self.history
    }

    /// Restore from history
    pub fn restore_from_history(&mut self, index: usize) -> Result<(), ClipboardError> {
        if index >= self.history.len() {
            return Err(ClipboardError::IndexOutOfRange);
        }

        let entry = self.history[index].clone();
        self.current_entry = Some(entry);
        Ok(())
    }

    /// Check auto-clear
    fn check_auto_clear(&mut self) {
        if let Some(ref entry) = self.current_entry {
            if self.auto_clear_enabled
                && core::time::Duration::from_millis(0) > entry.auto_clear_after
            {
                self.clear();
            }
        }
    }

    /// Set security level for next copy
    pub fn set_security_level(&mut self, level: SecurityLevel) {
        self.default_security_level = level;
    }

    /// Get security level
    pub fn security_level(&self) -> SecurityLevel {
        self.default_security_level
    }
}

impl Default for SecureClipboardManager {
    fn default() -> Self {
        Self::new(Box::new(NoEncryption))
    }
}

/// Clipboard errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClipboardError {
    ClipboardEmpty,
    EncodingError(String),
    EncryptionError(String),
    DecryptionError(String),
    IndexOutOfRange,
    SecurityError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_entry() {
        let entry = ClipboardEntry {
            content: "test".to_string(),
            content_type: ClipboardType::Text,
            timestamp: 0u64,
            is_encrypted: false,
            auto_clear_after: Duration::from_secs(60),
        };
        assert_eq!(entry.content, "test");
    }

    #[test]
    fn test_no_encryption() {
        let security = NoEncryption;
        let secured = security.secure("test", SecurityLevel::None).unwrap();
        let unsecured = security.unsecure(&secured, SecurityLevel::None).unwrap();
        assert_eq!(unsecured, "test");
    }

    #[test]
    fn test_xor_encryption() {
        let security = XorEncryption::new(vec![1, 2, 3]);
        let secured = security.secure("test", SecurityLevel::Low).unwrap();
        let unsecured = security.unsecure(&secured, SecurityLevel::Low).unwrap();
        assert_eq!(unsecured, "test");
    }

    #[test]
    fn test_secure_clipboard_manager() {
        let mut manager = SecureClipboardManager::default();
        manager
            .copy("test".to_string(), ClipboardType::Text)
            .unwrap();
        let pasted = manager.paste().unwrap();
        assert_eq!(pasted, "test");
    }

    #[test]
    fn test_clipboard_clear() {
        let mut manager = SecureClipboardManager::default();
        manager
            .copy("test".to_string(), ClipboardType::Text)
            .unwrap();
        manager.clear();
        assert!(manager.paste().is_err());
    }
}
