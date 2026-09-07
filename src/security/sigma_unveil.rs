//! # sigma_unveil - Path Narrowing Framework
//!
//! Inspired by OpenBSD's unveil(), sigma_unveil allows processes to restrict
//! their filesystem access to specific paths. All other paths are inaccessible.
//!
//! ## Usage
//!
//! ```rust,ignore
//! // Allow read access to /etc and read-write access to /tmp
//! sigma_unveil!("/etc", "r");
//! sigma_unveil!("/tmp", "rw");
//! sigma_unveil!(nullptr, nullptr); // Lock the veil
//! ```

use std::vec::Vec;

use crate::klib::HashMap;
use sigma_types::Result;
// Path/PathBuf not in no_std; using std::string::String as path
pub type PathBuf = std::string::String;
pub type Path = str;

/// Access permissions for unveiled paths
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnveilPermissions {
    /// No access
    None,
    /// Read-only access
    Read,
    /// Write-only access
    Write,
    /// Read and write access
    ReadWrite,
    /// Execute access
    Execute,
    /// Full access (read, write, execute)
    Full,
}

impl UnveilPermissions {
    /// Parse permission string
    pub fn from_str(s: &str) -> Self {
        match s {
            "r" => UnveilPermissions::Read,
            "w" => UnveilPermissions::Write,
            "rw" | "wr" => UnveilPermissions::ReadWrite,
            "x" => UnveilPermissions::Execute,
            "rwx" | "rxw" | "xrw" | "xwr" | "wxr" | "wrx" => UnveilPermissions::Full,
            _ => UnveilPermissions::None,
        }
    }

    /// Check if read is allowed
    pub fn allows_read(&self) -> bool {
        matches!(
            self,
            UnveilPermissions::Read | UnveilPermissions::ReadWrite | UnveilPermissions::Full
        )
    }

    /// Check if write is allowed
    pub fn allows_write(&self) -> bool {
        matches!(
            self,
            UnveilPermissions::Write | UnveilPermissions::ReadWrite | UnveilPermissions::Full
        )
    }

    /// Check if execute is allowed
    pub fn allows_execute(&self) -> bool {
        matches!(self, UnveilPermissions::Execute | UnveilPermissions::Full)
    }
}

/// Unveil entry mapping path to permissions
#[derive(Debug, Clone)]
pub struct UnveilEntry {
    path: PathBuf,
    permissions: UnveilPermissions,
}

impl UnveilEntry {
    /// Create a new unveil entry
    pub fn new(path: PathBuf, permissions: UnveilPermissions) -> Self {
        UnveilEntry { path, permissions }
    }

    /// Get the path
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get permissions
    pub fn permissions(&self) -> UnveilPermissions {
        self.permissions
    }

    /// Check if this entry covers the given path
    pub fn covers(&self, path: &Path) -> bool {
        path.starts_with(&self.path)
    }
}

/// Unveil state for a process
#[derive(Debug, Clone)]
pub struct UnveilState {
    entries: Vec<UnveilEntry>,
    is_locked: bool,
}

impl UnveilState {
    /// Create a new unveil state
    pub fn new() -> Self {
        UnveilState {
            entries: Vec::new(),
            is_locked: false,
        }
    }

    /// Add an unveil entry
    pub fn unveil(&mut self, path: PathBuf, permissions: &str) -> Result<()> {
        if self.is_locked {
            return Err("Unveil is locked");
        }

        let perms = UnveilPermissions::from_str(permissions);
        self.entries.push(UnveilEntry::new(path, perms));
        Ok(())
    }

    /// Lock the veil (no more unveil calls allowed)
    pub fn lock(&mut self) -> Result<()> {
        if self.is_locked {
            return Err("Already locked");
        }
        self.is_locked = true;
        Ok(())
    }

    /// Check if a path is accessible with given permission
    pub fn check_access(&self, path: &Path, required_perm: UnveilPermissions) -> Result<()> {
        // Mitigate directory traversal: reject paths containing parent directory segments
        for component in path.split("/") {
            if component == ".." {
                return Err("Directory traversal sequence detected");
            }
        }

        // Find the most specific matching entry
        let mut best_entry: Option<&UnveilEntry> = None;
        let mut best_len = 0;

        for entry in &self.entries {
            if entry.covers(path) {
                let entry_len = entry.path().len();
                if entry_len > best_len {
                    best_len = entry_len;
                    best_entry = Some(entry);
                }
            }
        }

        match best_entry {
            Some(entry) => {
                let has_perm = match required_perm {
                    UnveilPermissions::Read => entry.permissions().allows_read(),
                    UnveilPermissions::Write => entry.permissions().allows_write(),
                    UnveilPermissions::Execute => entry.permissions().allows_execute(),
                    _ => false,
                };

                if has_perm {
                    Ok(())
                } else {
                    Err("Permission not granted")
                }
            }
            None => {
                // No matching entry - deny
                Err("Path not unveiled")
            }
        }
    }

    /// Check if locked
    pub fn is_locked(&self) -> bool {
        self.is_locked
    }
}

impl Default for UnveilState {
    fn default() -> Self {
        Self::new()
    }
}

/// Unveil manager for all processes
pub struct UnveilManager {
    process_states: HashMap<u64, UnveilState>,
}

impl UnveilManager {
    /// Create a new unveil manager
    pub fn new() -> Self {
        UnveilManager {
            process_states: HashMap::new(),
        }
    }

    /// Register a process state
    pub fn register_state(&mut self, process_id: u64, state: UnveilState) {
        self.process_states.insert(process_id, state);
    }

    /// Unveil a path for a process
    pub fn unveil(&mut self, process_id: u64, path: PathBuf, permissions: &str) -> Result<()> {
        match self.process_states.get_mut(&process_id) {
            Some(state) => state.unveil(path, permissions),
            None => {
                // Create new state for this process
                let mut state = UnveilState::new();
                let result = state.unveil(path, permissions);
                self.process_states.insert(process_id, state);
                result
            }
        }
    }

    /// Lock the veil for a process
    pub fn lock(&mut self, process_id: u64) -> Result<()> {
        match self.process_states.get_mut(&process_id) {
            Some(state) => state.lock(),
            None => Err("Process not found"),
        }
    }

    /// Check if a path is accessible for a process
    pub fn check_access(
        &self,
        process_id: u64,
        path: &Path,
        required_perm: UnveilPermissions,
    ) -> Result<()> {
        match self.process_states.get(&process_id) {
            Some(state) => state.check_access(path, required_perm),
            None => Err("Process not found"),
        }
    }

    /// Remove process state
    pub fn remove_state(&mut self, process_id: u64) {
        self.process_states.remove(&process_id);
    }
}

impl Default for UnveilManager {
    fn default() -> Self {
        Self::new()
    }
}

/// sigma_unveil macro for easy unveil declaration
#[macro_export]
macro_rules! sigma_unveil {
    ($path:expr, $perms:expr) => {{
        let path = std::string::String::from($path);
        // In real implementation, this would call the global unveil manager
        // For now, we'll just create the state
        let mut state = $crate::security::sigma_unveil::UnveilState::new();
        state.unveil(path, $perms).expect("Failed to unveil");
        state
    }};
    (nullptr, nullptr) => {{
        // Lock the veil - in real implementation, this would call the global manager
        // For now, this is a placeholder
    }};
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_permissions_from_str() {
        assert_eq!(UnveilPermissions::from_str("r"), UnveilPermissions::Read);
        assert_eq!(UnveilPermissions::from_str("w"), UnveilPermissions::Write);
        assert_eq!(
            UnveilPermissions::from_str("rw"),
            UnveilPermissions::ReadWrite
        );
        assert_eq!(UnveilPermissions::from_str("x"), UnveilPermissions::Execute);
        assert_eq!(UnveilPermissions::from_str("rwx"), UnveilPermissions::Full);
    }

    #[test]
    fn test_unveil_state() {
        let mut state = UnveilState::new();

        state
            .unveil(std::string::String::from("/etc"), "r")
            .unwrap();
        state
            .unveil(std::string::String::from("/tmp"), "rw")
            .unwrap();

        assert!(!state.is_locked());
        state.lock().unwrap();
        assert!(state.is_locked());

        // Second lock should fail
        assert!(state.lock().is_err());
    }

    #[test]
    fn test_access_check() {
        let mut state = UnveilState::new();

        state
            .unveil(std::string::String::from("/etc"), "r")
            .unwrap();
        state
            .unveil(std::string::String::from("/tmp"), "rw")
            .unwrap();

        // Read access to /etc should be allowed
        assert!(state
            .check_access("/etc/passwd", UnveilPermissions::Read)
            .is_ok());

        // Write access to /etc should be denied
        assert!(state
            .check_access("/etc/passwd", UnveilPermissions::Write)
            .is_err());

        // Read access to /tmp should be allowed
        assert!(state
            .check_access("/tmp/file", UnveilPermissions::Read)
            .is_ok());

        // Write access to /tmp should be allowed
        assert!(state
            .check_access("/tmp/file", UnveilPermissions::Write)
            .is_ok());

        // Access to /var should be denied (not unveiled)
        assert!(state
            .check_access("/var/log", UnveilPermissions::Read)
            .is_err());

        // Traversal sequences should be immediately blocked and return Err
        assert!(state
            .check_access("/etc/../tmp/file", UnveilPermissions::Read)
            .is_err());
    }

    #[test]
    fn test_unveil_manager() {
        let mut manager = UnveilManager::new();

        manager
            .unveil(1, std::string::String::from("/home"), "rw")
            .unwrap();
        manager.lock(1).unwrap();

        // Check access
        assert!(manager
            .check_access(1, "/home/user/file", UnveilPermissions::Read)
            .is_ok());

        // Unregistered process
        assert!(manager
            .check_access(2, "/home/user/file", UnveilPermissions::Read)
            .is_err());
    }
}

// Placeholder types for compilation
mod sigma_types {
    pub type Result<T> = core::result::Result<T, &'static str>;

    #[derive(Debug, Clone)]
    pub struct CapabilityToken {
        pub id: u64,
    }
}
