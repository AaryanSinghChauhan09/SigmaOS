// Linux-inspired sysfs virtual filesystem
// Kernel and hardware information interface for SigmaOS

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// sysfs entry type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SysfsEntryType {
    Directory,
    File,
    Symlink,
}

/// sysfs entry
#[derive(Debug, Clone)]
pub struct SysfsEntry {
    pub name: String,
    pub entry_type: SysfsEntryType,
    pub value: String,
    pub permissions: u32,
    pub children: Vec<String>,
}

impl SysfsEntry {
    pub fn new(name: String, entry_type: SysfsEntryType, value: String) -> Self {
        SysfsEntry {
            name,
            entry_type,
            value,
            permissions: 0o644,
            children: Vec::new(),
        }
    }

    pub fn with_permissions(mut self, permissions: u32) -> Self {
        self.permissions = permissions;
        self
    }

    pub fn add_child(&mut self, child: String) {
        self.children.push(child);
    }
}

/// sysfs instance
pub struct Sysfs {
    pub entries: HashMap<String, Arc<Mutex<SysfsEntry>>>,
}

impl Sysfs {
    pub fn new() -> Self {
        let mut sysfs = Sysfs {
            entries: HashMap::new(),
        };

        // Create standard sysfs structure
        sysfs.create_standard_structure();
        sysfs
    }

    fn create_standard_structure(&mut self) {
        // /sys/devices
        self.create_entry("/sys/devices".to_string(), SysfsEntryType::Directory, "".to_string());
        
        // /sys/kernel
        self.create_entry("/sys/kernel".to_string(), SysfsEntryType::Directory, "".to_string());
        self.create_entry("/sys/kernel/version".to_string(), SysfsEntryType::File, "SigmaOS 1.0.0".to_string());
        self.create_entry("/sys/kernel/hostname".to_string(), SysfsEntryType::File, "sigmaos".to_string());
        
        // /sys/module
        self.create_entry("/sys/module".to_string(), SysfsEntryType::Directory, "".to_string());
        
        // /sys/fs
        self.create_entry("/sys/fs".to_string(), SysfsEntryType::Directory, "".to_string());
        self.create_entry("/sys/fs/cgroup".to_string(), SysfsEntryType::Directory, "".to_string());
        self.create_entry("/sys/fs/proc".to_string(), SysfsEntryType::Directory, "".to_string());
        
        // /sys/class
        self.create_entry("/sys/class".to_string(), SysfsEntryType::Directory, "".to_string());
        self.create_entry("/sys/class/net".to_string(), SysfsEntryType::Directory, "".to_string());
        self.create_entry("/sys/class/block".to_string(), SysfsEntryType::Directory, "".to_string());
        
        // /sys/block
        self.create_entry("/sys/block".to_string(), SysfsEntryType::Directory, "".to_string());
        
        // /sys/bus
        self.create_entry("/sys/bus".to_string(), SysfsEntryType::Directory, "".to_string());
        self.create_entry("/sys/bus/pci".to_string(), SysfsEntryType::Directory, "".to_string());
        self.create_entry("/sys/bus/usb".to_string(), SysfsEntryType::Directory, "".to_string());
    }

    pub fn create_entry(&mut self, path: String, entry_type: SysfsEntryType, value: String) -> Arc<Mutex<SysfsEntry>> {
        let entry = Arc::new(Mutex::new(SysfsEntry::new(
            path.split('/').last().unwrap_or(&path).to_string(),
            entry_type,
            value,
        )));
        
        self.entries.insert(path.clone(), entry.clone());
        
        // Add to parent directory
        if let Some(parent_path) = Self::parent_path(&path) {
            if let Some(parent) = self.entries.get_mut(&parent_path) {
                let mut parent_guard = parent.lock().unwrap();
                parent_guard.add_child(path.clone());
            }
        }
        
        entry
    }

    pub fn get_entry(&self, path: &str) -> Option<Arc<Mutex<SysfsEntry>>> {
        self.entries.get(path).cloned()
    }

    pub fn read_entry(&self, path: &str) -> Result<String, String> {
        let entry = self.entries.get(path)
            .ok_or_else(|| format!("Entry not found: {}", path))?;
        
        let entry_guard = entry.lock().unwrap();
        if entry_guard.entry_type != SysfsEntryType::File {
            return Err("Not a file".to_string());
        }
        
        Ok(entry_guard.value.clone())
    }

    pub fn write_entry(&self, path: &str, value: String) -> Result<(), String> {
        let entry = self.entries.get(path)
            .ok_or_else(|| format!("Entry not found: {}", path))?;
        
        let mut entry_guard = entry.lock().unwrap();
        if entry_guard.entry_type != SysfsEntryType::File {
            return Err("Not a file".to_string());
        }
        
        entry_guard.value = value;
        Ok(())
    }

    pub fn list_directory(&self, path: &str) -> Result<Vec<String>, String> {
        let entry = self.entries.get(path)
            .ok_or_else(|| format!("Entry not found: {}", path))?;
        
        let entry_guard = entry.lock().unwrap();
        if entry_guard.entry_type != SysfsEntryType::Directory {
            return Err("Not a directory".to_string());
        }
        
        Ok(entry_guard.children.clone())
    }

    fn parent_path(path: &str) -> Option<String> {
        if path == "/" || !path.contains('/') {
            return None;
        }
        
        let last_slash = path.rfind('/');
        if let Some(pos) = last_slash {
            if pos == 0 {
                Some("/".to_string())
            } else {
                Some(path[..pos].to_string())
            }
        } else {
            None
        }
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}

impl Default for Sysfs {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sysfs_creation() {
        let sysfs = Sysfs::new();
        assert!(sysfs.entry_count() > 0);
    }

    #[test]
    fn test_sysfs_create_entry() {
        let mut sysfs = Sysfs::new();
        let entry = sysfs.create_entry("/sys/test".to_string(), SysfsEntryType::File, "test_value".to_string());
        
        let entry_guard = entry.lock().unwrap();
        assert_eq!(entry_guard.value, "test_value");
    }

    #[test]
    fn test_sysfs_read_entry() {
        let sysfs = Sysfs::new();
        let value = sysfs.read_entry("/sys/kernel/version").unwrap();
        assert_eq!(value, "SigmaOS 1.0.0");
    }

    #[test]
    fn test_sysfs_read_nonexistent() {
        let sysfs = Sysfs::new();
        let result = sysfs.read_entry("/sys/nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_sysfs_write_entry() {
        let sysfs = Sysfs::new();
        sysfs.write_entry("/sys/kernel/hostname", "newhostname".to_string()).unwrap();
        
        let value = sysfs.read_entry("/sys/kernel/hostname").unwrap();
        assert_eq!(value, "newhostname");
    }

    #[test]
    fn test_sysfs_list_directory() {
        let sysfs = Sysfs::new();
        let children = sysfs.list_directory("/sys").unwrap();
        assert!(children.len() > 0);
    }

    #[test]
    fn test_sysfs_list_directory_nonexistent() {
        let sysfs = Sysfs::new();
        let result = sysfs.list_directory("/sys/nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_sysfs_entry_permissions() {
        let mut sysfs = Sysfs::new();
        let mut entry = SysfsEntry::new("test".to_string(), SysfsEntryType::File, "test".to_string());
        entry.permissions = 0o755;
        
        let entry_arc = Arc::new(Mutex::new(entry));
        sysfs.entries.insert("/sys/test".to_string(), entry_arc.clone());
        
        let entry_guard = entry_arc.lock().unwrap();
        assert_eq!(entry_guard.permissions, 0o755);
    }
}
