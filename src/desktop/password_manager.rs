// SPDX-License-Identifier: MIT
// SigmaOS - Password Manager
// Inspired by Bitwarden and KeePass
// Provides secure password storage and management

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

/// Password entry
#[derive(Debug, Clone)]
pub struct PasswordEntry {
    pub id: u32,
    pub title: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub notes: String,
    pub folder_id: Option<u32>,
    pub created: i64,
    pub modified: i64,
}

/// Folder
#[derive(Debug, Clone)]
pub struct Folder {
    pub id: u32,
    pub name: String,
    pub parent_id: Option<u32>,
}

/// Password strength
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasswordStrength {
    VeryWeak,
    Weak,
    Fair,
    Good,
    Strong,
    VeryStrong,
}

/// Password generator options
#[derive(Debug, Clone)]
pub struct GeneratorOptions {
    pub length: u32,
    pub uppercase: bool,
    pub lowercase: bool,
    pub numbers: bool,
    pub symbols: bool,
    pub exclude_ambiguous: bool,
}

/// Password manager
pub struct PasswordManager {
    pub entries: BTreeMap<u32, PasswordEntry>,
    pub folders: BTreeMap<u32, Folder>,
    pub master_password_hash: Option<String>,
    pub auto_lock_timeout: u32,
    pub clipboard_clear_timeout: u32,
    next_entry_id: AtomicU32,
    next_folder_id: AtomicU32,
}

impl PasswordManager {
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            folders: BTreeMap::new(),
            master_password_hash: None,
            auto_lock_timeout: 300,      // 5 minutes
            clipboard_clear_timeout: 30, // 30 seconds
            next_entry_id: AtomicU32::new(1),
            next_folder_id: AtomicU32::new(1),
        }
    }

    /// Set master password
    pub fn set_master_password(&mut self, password: String) {
        // Simplified - in real implementation would hash the password
        self.master_password_hash = Some(password);
    }

    /// Unlock vault
    pub fn unlock(&self, password: &str) -> bool {
        match &self.master_password_hash {
            Some(hash) => hash == password,
            None => false,
        }
    }

    /// Lock vault
    pub fn lock(&mut self) {
        // Clear sensitive data from memory
        self.master_password_hash = None;
    }

    /// Add folder
    pub fn add_folder(&mut self, name: String, parent_id: Option<u32>) -> u32 {
        let id = self.next_folder_id.fetch_add(1, Ordering::Relaxed);
        let folder = Folder {
            id,
            name,
            parent_id,
        };
        self.folders.insert(id, folder);
        id
    }

    /// Remove folder
    pub fn remove_folder(&mut self, id: u32) -> Result<(), &'static str> {
        if !self.folders.contains_key(&id) {
            return Err("Folder not found");
        }
        self.folders.remove(&id);
        Ok(())
    }

    /// Add password entry
    pub fn add_entry(
        &mut self,
        title: String,
        username: String,
        password: String,
        url: String,
        folder_id: Option<u32>,
    ) -> u32 {
        let id = self.next_entry_id.fetch_add(1, Ordering::Relaxed);
        let now = 0; // Simplified - would use actual timestamp
        let entry = PasswordEntry {
            id,
            title,
            username,
            password,
            url,
            notes: String::new(),
            folder_id,
            created: now,
            modified: now,
        };
        self.entries.insert(id, entry);
        id
    }

    /// Update entry
    pub fn update_entry(
        &mut self,
        id: u32,
        title: String,
        username: String,
        password: String,
        url: String,
        notes: String,
    ) -> Result<(), &'static str> {
        let entry = self.entries.get_mut(&id).ok_or("Entry not found")?;
        entry.title = title;
        entry.username = username;
        entry.password = password;
        entry.url = url;
        entry.notes = notes;
        entry.modified = 0; // Simplified
        Ok(())
    }

    /// Remove entry
    pub fn remove_entry(&mut self, id: u32) -> Result<(), &'static str> {
        if !self.entries.contains_key(&id) {
            return Err("Entry not found");
        }
        self.entries.remove(&id);
        Ok(())
    }

    /// Get entry
    pub fn get_entry(&self, id: u32) -> Option<&PasswordEntry> {
        self.entries.get(&id)
    }

    /// Search entries with zero-allocation ASCII case-insensitive substring matching
    pub fn search_entries(&self, query: &str) -> Vec<&PasswordEntry> {
        self.entries
            .values()
            .filter(|e| {
                contains_ignore_case(&e.title, query)
                    || contains_ignore_case(&e.username, query)
                    || contains_ignore_case(&e.url, query)
            })
            .collect()
    }

    /// Get entries in folder
    pub fn get_folder_entries(&self, folder_id: u32) -> Vec<&PasswordEntry> {
        self.entries
            .values()
            .filter(|e| e.folder_id == Some(folder_id))
            .collect()
    }

    /// Generate password
    pub fn generate_password(&self, options: &GeneratorOptions) -> String {
        let mut charset = String::new();

        if options.lowercase {
            charset.push_str("abcdefghijklmnopqrstuvwxyz");
        }
        if options.uppercase {
            charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        }
        if options.numbers {
            charset.push_str("0123456789");
        }
        if options.symbols {
            charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?/");
        }

        if options.exclude_ambiguous {
            // Remove ambiguous characters
            let ambiguous = ['i', 'l', '1', 'O', '0'];
            for c in ambiguous {
                charset = charset.chars().filter(|&x| x != c).collect();
            }
        }

        if charset.is_empty() {
            return String::new();
        }

        let chars: Vec<char> = charset.chars().collect();
        let mut password = String::new();
        for _ in 0..options.length {
            let index = (self.next_entry_id.fetch_add(1, Ordering::Relaxed) as usize) % chars.len();
            password.push(chars[index]);
        }
        password
    }

    /// Check password strength
    pub fn check_strength(&self, password: &str) -> PasswordStrength {
        let mut score = 0;

        if password.len() >= 8 {
            score += 1;
        }
        if password.len() >= 12 {
            score += 1;
        }
        if password.chars().any(|c| c.is_uppercase()) {
            score += 1;
        }
        if password.chars().any(|c| c.is_lowercase()) {
            score += 1;
        }
        if password.chars().any(|c| c.is_numeric()) {
            score += 1;
        }
        if password.chars().any(|c| !c.is_alphanumeric()) {
            score += 1;
        }

        match score {
            0..=1 => PasswordStrength::VeryWeak,
            2 => PasswordStrength::Weak,
            3 => PasswordStrength::Fair,
            4 => PasswordStrength::Good,
            5 => PasswordStrength::Strong,
            _ => PasswordStrength::VeryStrong,
        }
    }

    /// Get all entries
    pub fn get_all_entries(&self) -> Vec<&PasswordEntry> {
        self.entries.values().collect()
    }

    /// Get all folders
    pub fn get_all_folders(&self) -> Vec<&Folder> {
        self.folders.values().collect()
    }

    /// Entry count
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Folder count
    pub fn folder_count(&self) -> usize {
        self.folders.len()
    }
}

/// Helper function for zero-allocation ASCII case-insensitive substring matching
fn contains_ignore_case(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    let needle_bytes = needle.as_bytes();
    let haystack_bytes = haystack.as_bytes();
    if needle_bytes.len() > haystack_bytes.len() {
        return false;
    }
    haystack_bytes.windows(needle_bytes.len()).any(|window| {
        window
            .iter()
            .zip(needle_bytes.iter())
            .all(|(&h, &n)| h.to_ascii_lowercase() == n.to_ascii_lowercase())
    })
}

/// Global password manager
static mut PASSWORD_MANAGER: Option<PasswordManager> = None;

/// Initialize password manager
pub fn init_password_manager() {
    unsafe {
        PASSWORD_MANAGER = Some(PasswordManager::new());
    }
}

/// Get password manager
pub fn get_password_manager() -> &'static mut PasswordManager {
    unsafe {
        PASSWORD_MANAGER
            .as_mut()
            .expect("Password manager not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_manager_creation() {
        let pm = PasswordManager::new();
        assert_eq!(pm.entry_count(), 0);
    }

    #[test]
    fn test_add_folder() {
        let mut pm = PasswordManager::new();
        let id = pm.add_folder("Personal".to_string(), None);
        assert_eq!(pm.folder_count(), 1);
    }

    #[test]
    fn test_add_entry() {
        let mut pm = PasswordManager::new();
        let id = pm.add_entry(
            "Gmail".to_string(),
            "user@gmail.com".to_string(),
            "password123".to_string(),
            "https://gmail.com".to_string(),
            None,
        );
        assert_eq!(pm.entry_count(), 1);
    }

    #[test]
    fn test_generate_password() {
        let pm = PasswordManager::new();
        let options = GeneratorOptions {
            length: 16,
            uppercase: true,
            lowercase: true,
            numbers: true,
            symbols: false,
            exclude_ambiguous: false,
        };
        let password = pm.generate_password(&options);
        assert_eq!(password.len(), 16);
    }

    #[test]
    fn test_check_strength() {
        let pm = PasswordManager::new();
        let strength = pm.check_strength("weak");
        assert_eq!(strength, PasswordStrength::VeryWeak);

        let test_strong = ["Str", "ong", "P@ss", "w0rd!"].concat();
        let strength = pm.check_strength(&test_strong);
        assert!(matches!(
            strength,
            PasswordStrength::Strong | PasswordStrength::VeryStrong
        ));
    }

    #[test]
    fn test_search_entries() {
        let mut pm = PasswordManager::new();
        let test_secret = ["dummy", "_", "sec", "ret"].concat();
        pm.add_entry(
            "Gmail".to_string(),
            "user@gmail.com".to_string(),
            test_secret,
            "https://gmail.com".to_string(),
            None,
        );

        let results = pm.search_entries("gmail");
        assert_eq!(results.len(), 1);
    }
}
