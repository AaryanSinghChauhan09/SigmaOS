// Linux-inspired umask for file creation mask
// Provides umask for controlling default file permissions

use std::sync::{Arc, Mutex};

/// Umask manager for system-wide umask management
pub struct UmaskManager {
    pub umask: Arc<Mutex<u32>>,
}

impl UmaskManager {
    pub fn new() -> Self {
        // Default umask is 0o022 (rwxr-xr-x for files, rwxr-xr-x for directories)
        Self {
            umask: Arc::new(Mutex::new(0o022)),
        }
    }

    /// Get umask
    pub fn get(&self) -> u32 {
        let umask = self.umask.lock().unwrap();
        *umask
    }

    /// Set umask
    pub fn set(&self, mask: u32) -> u32 {
        let mut umask = self.umask.lock().unwrap();
        let old = *umask;
        *umask = mask & 0o777; // Only keep permission bits
        old
    }

    /// Apply umask to mode
    pub fn apply(&self, mode: u32) -> u32 {
        let umask = self.umask.lock().unwrap();
        mode & !(*umask)
    }

    /// Apply umask to file mode (files cannot be executable by default)
    pub fn apply_file_mode(&self, mode: u32) -> u32 {
        let umask = self.umask.lock().unwrap();
        // For files, remove execute bits from the mode before applying umask
        let file_mode = mode & 0o666;
        file_mode & !(*umask)
    }

    /// Apply umask to directory mode
    pub fn apply_directory_mode(&self, mode: u32) -> u32 {
        let umask = self.umask.lock().unwrap();
        // For directories, keep execute bits
        mode & !(*umask)
    }
}

impl Default for UmaskManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_umask_manager() {
        let manager = UmaskManager::new();
        assert_eq!(manager.get(), 0o022);
    }

    #[test]
    fn test_umask_manager_set() {
        let manager = UmaskManager::new();
        let old = manager.set(0o077);
        assert_eq!(old, 0o022);
        assert_eq!(manager.get(), 0o077);
    }

    #[test]
    fn test_umask_manager_set_truncates() {
        let manager = UmaskManager::new();
        manager.set(0o1777);
        assert_eq!(manager.get(), 0o777);
    }

    #[test]
    fn test_umask_manager_apply() {
        let manager = UmaskManager::new();
        let mode = 0o777;
        let result = manager.apply(mode);
        assert_eq!(result, 0o755);
    }

    #[test]
    fn test_umask_manager_apply_file_mode() {
        let manager = UmaskManager::new();
        let mode = 0o777;
        let result = manager.apply_file_mode(mode);
        assert_eq!(result, 0o644);
    }

    #[test]
    fn test_umask_manager_apply_directory_mode() {
        let manager = UmaskManager::new();
        let mode = 0o777;
        let result = manager.apply_directory_mode(mode);
        assert_eq!(result, 0o755);
    }

    #[test]
    fn test_umask_manager_custom() {
        let manager = UmaskManager::new();
        manager.set(0o077);
        let mode = 0o777;
        let result = manager.apply(mode);
        assert_eq!(result, 0o700);
    }

    #[test]
    fn test_umask_manager_file_with_execute() {
        let manager = UmaskManager::new();
        manager.set(0o000);
        let mode = 0o777;
        let result = manager.apply_file_mode(mode);
        // Even with umask 000, files don't get execute bits
        assert_eq!(result, 0o666);
    }

    #[test]
    fn test_umask_manager_directory_with_execute() {
        let manager = UmaskManager::new();
        manager.set(0o000);
        let mode = 0o777;
        let result = manager.apply_directory_mode(mode);
        // Directories keep execute bits
        assert_eq!(result, 0o777);
    }
}
