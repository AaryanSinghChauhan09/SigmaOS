// Linux-inspired sysfs for kernel object representation
// Provides sysfs for kernel object introspection

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Sysfs file type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SysfsFileType {
    Regular,
    Directory,
    Symlink,
}

/// Sysfs entry
#[derive(Debug, Clone)]
pub struct SysfsEntry {
    pub name: String,
    pub file_type: SysfsFileType,
    pub data: Vec<u8>,
    pub children: HashMap<String, SysfsEntry>,
}

impl SysfsEntry {
    pub fn new(name: String, file_type: SysfsFileType) -> Self {
        Self {
            name,
            file_type,
            data: Vec::new(),
            children: HashMap::new(),
        }
    }

    /// Add child entry
    pub fn add_child(&mut self, entry: SysfsEntry) -> Result<(), String> {
        if self.children.contains_key(&entry.name) {
            return Err(format!("Child {} already exists", entry.name));
        }

        self.children.insert(entry.name.clone(), entry);
        Ok(())
    }

    /// Remove child entry
    pub fn remove_child(&mut self, name: &str) -> Result<(), String> {
        match self.children.remove(name) {
            Some(_) => Ok(()),
            None => Err(format!("Child {} not found", name)),
        }
    }

    /// Get child entry
    pub fn get_child(&self, name: &str) -> Option<&SysfsEntry> {
        self.children.get(name)
    }

    /// Set data
    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    /// Get data
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    /// Get child count
    pub fn child_count(&self) -> usize {
        self.children.len()
    }
}

/// Sysfs manager for system-wide sysfs management
pub struct SysfsManager {
    pub root: Arc<Mutex<SysfsEntry>>,
}

impl SysfsManager {
    pub fn new() -> Self {
        let root = SysfsEntry::new("sys".to_string(), SysfsFileType::Directory);
        Self {
            root: Arc::new(Mutex::new(root)),
        }
    }

    /// Create directory
    pub fn create_directory(&self, path: &str) -> Result<(), String> {
        let parts: Vec<String> = path.split('/').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
        if parts.is_empty() {
            return Err("Invalid path".to_string());
        }

        let mut root = self.root.lock().unwrap();
        let mut current = &mut *root;

        for (i, part) in parts.iter().enumerate() {
            let is_last = i == parts.len() - 1;

            let exists = current.children.contains_key(part);
            if exists {
                let child = current.children.get_mut(part).unwrap();
                if is_last && child.file_type != SysfsFileType::Directory {
                    return Err(format!("{} is not a directory", path));
                }
                current = child;
            } else {
                if is_last {
                    let new_entry = SysfsEntry::new(part.clone(), SysfsFileType::Directory);
                    current.children.insert(part.clone(), new_entry);
                } else {
                    let new_entry = SysfsEntry::new(part.clone(), SysfsFileType::Directory);
                    current.children.insert(part.clone(), new_entry);
                    current = current.children.get_mut(part).unwrap();
                }
            }
        }

        Ok(())
    }

    /// Create file
    pub fn create_file(&self, path: &str, data: Vec<u8>) -> Result<(), String> {
        let parts: Vec<String> = path.split('/').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
        if parts.is_empty() {
            return Err("Invalid path".to_string());
        }

        let filename = parts.last().unwrap();
        let dir_path: Vec<String> = parts[..parts.len() - 1].to_vec();
        let dir_path_str = dir_path.join("/");

        let mut root = self.root.lock().unwrap();
        let mut current = &mut *root;

        // Navigate to directory
        for part in &dir_path {
            if current.children.contains_key(part) {
                current = current.children.get_mut(part).unwrap();
            } else {
                return Err(format!("Directory {} not found", dir_path_str));
            }
        }

        // Create file
        let mut new_entry = SysfsEntry::new(filename.clone(), SysfsFileType::Regular);
        new_entry.set_data(data);
        current.children.insert(filename.clone(), new_entry);

        Ok(())
    }

    /// Read file
    pub fn read_file(&self, path: &str) -> Result<Vec<u8>, String> {
        let parts: Vec<String> = path.split('/').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
        if parts.is_empty() {
            return Err("Invalid path".to_string());
        }

        let filename = parts.last().unwrap();
        let dir_path: Vec<String> = parts[..parts.len() - 1].to_vec();
        let dir_path_str = dir_path.join("/");

        let root = self.root.lock().unwrap();
        let mut current = &*root;

        // Navigate to directory
        for part in &dir_path {
            if current.children.contains_key(part) {
                current = current.children.get(part).unwrap();
            } else {
                return Err(format!("Directory {} not found", dir_path_str));
            }
        }

        // Read file
        match current.children.get(filename) {
            Some(entry) => {
                if entry.file_type == SysfsFileType::Regular {
                    Ok(entry.get_data().to_vec())
                } else {
                    Err(format!("{} is not a regular file", path))
                }
            }
            None => Err(format!("File {} not found", path)),
        }
    }

    /// Write file
    pub fn write_file(&self, path: &str, data: Vec<u8>) -> Result<(), String> {
        let parts: Vec<String> = path.split('/').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
        if parts.is_empty() {
            return Err("Invalid path".to_string());
        }

        let filename = parts.last().unwrap();
        let dir_path: Vec<String> = parts[..parts.len() - 1].to_vec();
        let dir_path_str = dir_path.join("/");

        let mut root = self.root.lock().unwrap();
        let mut current = &mut *root;

        // Navigate to directory
        for part in &dir_path {
            if current.children.contains_key(part) {
                current = current.children.get_mut(part).unwrap();
            } else {
                return Err(format!("Directory {} not found", dir_path_str));
            }
        }

        // Write file
        match current.children.get_mut(filename) {
            Some(entry) => {
                if entry.file_type == SysfsFileType::Regular {
                    entry.set_data(data);
                    Ok(())
                } else {
                    Err(format!("{} is not a regular file", path))
                }
            }
            None => Err(format!("File {} not found", path)),
        }
    }

    /// Remove entry
    pub fn remove(&self, path: &str) -> Result<(), String> {
        let parts: Vec<String> = path.split('/').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
        if parts.is_empty() {
            return Err("Invalid path".to_string());
        }

        let filename = parts.last().unwrap();
        let dir_path: Vec<String> = parts[..parts.len() - 1].to_vec();
        let dir_path_str = dir_path.join("/");

        let mut root = self.root.lock().unwrap();
        let mut current = &mut *root;

        // Navigate to directory
        for part in &dir_path {
            if current.children.contains_key(part) {
                current = current.children.get_mut(part).unwrap();
            } else {
                return Err(format!("Directory {} not found", dir_path_str));
            }
        }

        // Remove entry
        current.remove_child(filename)
    }

    /// List directory
    pub fn list_directory(&self, path: &str) -> Result<Vec<String>, String> {
        let parts: Vec<String> = path.split('/').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();

        let root = self.root.lock().unwrap();
        let mut current = &*root;

        // Navigate to directory
        for part in &parts {
            if current.children.contains_key(part) {
                current = current.children.get(part).unwrap();
            } else {
                return Err(format!("Directory {} not found", path));
            }
        }

        if current.file_type != SysfsFileType::Directory {
            return Err(format!("{} is not a directory", path));
        }

        Ok(current.children.keys().cloned().collect())
    }
}

impl Default for SysfsManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sysfs_entry() {
        let entry = SysfsEntry::new("test".to_string(), SysfsFileType::Regular);
        assert_eq!(entry.name, "test");
        assert_eq!(entry.file_type, SysfsFileType::Regular);
    }

    #[test]
    fn test_sysfs_entry_add_child() {
        let mut entry = SysfsEntry::new("parent".to_string(), SysfsFileType::Directory);
        let child = SysfsEntry::new("child".to_string(), SysfsFileType::Regular);

        entry.add_child(child).unwrap();
        assert_eq!(entry.child_count(), 1);
    }

    #[test]
    fn test_sysfs_entry_set_data() {
        let mut entry = SysfsEntry::new("test".to_string(), SysfsFileType::Regular);
        entry.set_data(b"Hello".to_vec());

        assert_eq!(entry.get_data(), b"Hello");
    }

    #[test]
    fn test_sysfs_manager() {
        let manager = SysfsManager::new();
        assert_eq!(manager.root.lock().unwrap().name, "sys");
    }

    #[test]
    fn test_sysfs_manager_create_directory() {
        let manager = SysfsManager::new();
        manager.create_directory("kernel").unwrap();

        let root = manager.root.lock().unwrap();
        assert!(root.children.contains_key("kernel"));
    }

    #[test]
    fn test_sysfs_manager_create_file() {
        let manager = SysfsManager::new();
        manager.create_directory("kernel").unwrap();
        manager.create_file("kernel/version", b"1.0".to_vec()).unwrap();

        let data = manager.read_file("kernel/version").unwrap();
        assert_eq!(data, b"1.0");
    }

    #[test]
    fn test_sysfs_manager_read_file() {
        let manager = SysfsManager::new();
        manager.create_directory("kernel").unwrap();
        manager.create_file("kernel/version", b"1.0".to_vec()).unwrap();

        let data = manager.read_file("kernel/version").unwrap();
        assert_eq!(data, b"1.0");
    }

    #[test]
    fn test_sysfs_manager_write_file() {
        let manager = SysfsManager::new();
        manager.create_directory("kernel").unwrap();
        manager.create_file("kernel/version", b"1.0".to_vec()).unwrap();
        manager.write_file("kernel/version", b"2.0".to_vec()).unwrap();

        let data = manager.read_file("kernel/version").unwrap();
        assert_eq!(data, b"2.0");
    }

    #[test]
    fn test_sysfs_manager_list_directory() {
        let manager = SysfsManager::new();
        manager.create_directory("kernel").unwrap();
        manager.create_file("kernel/version", b"1.0".to_vec()).unwrap();

        let entries = manager.list_directory("kernel").unwrap();
        assert!(entries.contains(&"version".to_string()));
    }

    #[test]
    fn test_sysfs_manager_remove() {
        let manager = SysfsManager::new();
        manager.create_directory("kernel").unwrap();
        manager.create_file("kernel/version", b"1.0".to_vec()).unwrap();
        manager.remove("kernel/version").unwrap();

        let entries = manager.list_directory("kernel").unwrap();
        assert!(!entries.contains(&"version".to_string()));
    }

    #[test]
    fn test_sysfs_manager_invalid() {
        let manager = SysfsManager::new();
        assert!(manager.read_file("nonexistent").is_err());
        assert!(manager.write_file("nonexistent", b"test".to_vec()).is_err());
    }
}
