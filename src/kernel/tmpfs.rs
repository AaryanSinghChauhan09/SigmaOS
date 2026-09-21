// Linux-inspired tmpfs for temporary file storage
// Provides tmpfs for in-memory temporary files

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Tmpfs file
#[derive(Debug, Clone)]
pub struct TmpfsFile {
    pub name: String,
    pub data: Vec<u8>,
    pub size: usize,
    pub permissions: u32,
}

impl TmpfsFile {
    pub fn new(name: String, permissions: u32) -> Self {
        Self {
            name,
            data: Vec::new(),
            size: 0,
            permissions,
        }
    }

    /// Write data
    pub fn write(&mut self, data: Vec<u8>) {
        self.data = data;
        self.size = self.data.len();
    }

    /// Read data
    pub fn read(&self) -> Vec<u8> {
        self.data.clone()
    }

    /// Truncate
    pub fn truncate(&mut self, size: usize) {
        self.data.truncate(size);
        self.size = size;
    }

    /// Get size
    pub fn get_size(&self) -> usize {
        self.size
    }
}

/// Tmpfs directory
#[derive(Debug, Clone)]
pub struct TmpfsDirectory {
    pub name: String,
    pub files: HashMap<String, TmpfsFile>,
    pub subdirectories: HashMap<String, TmpfsDirectory>,
}

impl TmpfsDirectory {
    pub fn new(name: String) -> Self {
        Self {
            name,
            files: HashMap::new(),
            subdirectories: HashMap::new(),
        }
    }

    /// Add file
    pub fn add_file(&mut self, file: TmpfsFile) -> Result<(), String> {
        if self.files.contains_key(&file.name) {
            return Err(format!("File {} already exists", file.name));
        }

        self.files.insert(file.name.clone(), file);
        Ok(())
    }

    /// Remove file
    pub fn remove_file(&mut self, name: &str) -> Result<(), String> {
        match self.files.remove(name) {
            Some(_) => Ok(()),
            None => Err(format!("File {} not found", name)),
        }
    }

    /// Get file
    pub fn get_file(&self, name: &str) -> Option<&TmpfsFile> {
        self.files.get(name)
    }

    /// Get file mutable
    pub fn get_file_mut(&mut self, name: &str) -> Option<&mut TmpfsFile> {
        self.files.get_mut(name)
    }

    /// Add subdirectory
    pub fn add_subdirectory(&mut self, dir: TmpfsDirectory) -> Result<(), String> {
        if self.subdirectories.contains_key(&dir.name) {
            return Err(format!("Directory {} already exists", dir.name));
        }

        self.subdirectories.insert(dir.name.clone(), dir);
        Ok(())
    }

    /// Remove subdirectory
    pub fn remove_subdirectory(&mut self, name: &str) -> Result<(), String> {
        match self.subdirectories.remove(name) {
            Some(_) => Ok(()),
            None => Err(format!("Directory {} not found", name)),
        }
    }

    /// Get subdirectory
    pub fn get_subdirectory(&self, name: &str) -> Option<&TmpfsDirectory> {
        self.subdirectories.get(name)
    }

    /// Get subdirectory mutable
    pub fn get_subdirectory_mut(&mut self, name: &str) -> Option<&mut TmpfsDirectory> {
        self.subdirectories.get_mut(name)
    }

    /// List files
    pub fn list_files(&self) -> Vec<String> {
        self.files.keys().cloned().collect()
    }

    /// List subdirectories
    pub fn list_subdirectories(&self) -> Vec<String> {
        self.subdirectories.keys().cloned().collect()
    }

    /// Get total size
    pub fn get_total_size(&self) -> usize {
        let mut size = 0;
        for file in self.files.values() {
            size += file.get_size();
        }
        for dir in self.subdirectories.values() {
            size += dir.get_total_size();
        }
        size
    }
}

/// Tmpfs instance
#[derive(Debug, Clone)]
pub struct Tmpfs {
    pub root: TmpfsDirectory,
    pub max_size: usize,
}

impl Tmpfs {
    pub fn new(max_size: usize) -> Self {
        Self {
            root: TmpfsDirectory::new("/".to_string()),
            max_size,
        }
    }

    /// Create file
    pub fn create_file(&mut self, path: &str, permissions: u32) -> Result<(), String> {
        let parts: Vec<String> = path.split('/').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
        if parts.is_empty() {
            return Err("Invalid path".to_string());
        }

        let filename = parts.last().unwrap();
        let dir_path: Vec<String> = parts[..parts.len() - 1].to_vec();

        let current = self.navigate_to_directory_mut(&dir_path)?;

        let file = TmpfsFile::new(filename.clone(), permissions);
        current.add_file(file)
    }

    /// Write file
    pub fn write_file(&mut self, path: &str, data: Vec<u8>) -> Result<(), String> {
        let parts: Vec<String> = path.split('/').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
        if parts.is_empty() {
            return Err("Invalid path".to_string());
        }

        let filename = parts.last().unwrap();
        let dir_path: Vec<String> = parts[..parts.len() - 1].to_vec();

        let current = self.navigate_to_directory_mut(&dir_path)?;

        match current.get_file_mut(filename) {
            Some(file) => {
                file.write(data);
                Ok(())
            }
            None => Err(format!("File {} not found", path)),
        }
    }

    /// Read file
    pub fn read_file(&self, path: &str) -> Result<Vec<u8>, String> {
        let parts: Vec<String> = path.split('/').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
        if parts.is_empty() {
            return Err("Invalid path".to_string());
        }

        let filename = parts.last().unwrap();
        let dir_path: Vec<String> = parts[..parts.len() - 1].to_vec();

        let current = self.navigate_to_directory(&dir_path)?;

        match current.get_file(filename) {
            Some(file) => Ok(file.read()),
            None => Err(format!("File {} not found", path)),
        }
    }

    /// Remove file
    pub fn remove_file(&mut self, path: &str) -> Result<(), String> {
        let parts: Vec<String> = path.split('/').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
        if parts.is_empty() {
            return Err("Invalid path".to_string());
        }

        let filename = parts.last().unwrap();
        let dir_path: Vec<String> = parts[..parts.len() - 1].to_vec();

        let current = self.navigate_to_directory_mut(&dir_path)?;
        current.remove_file(filename)
    }

    /// Create directory
    pub fn create_directory(&mut self, path: &str) -> Result<(), String> {
        let parts: Vec<String> = path.split('/').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
        if parts.is_empty() {
            return Err("Invalid path".to_string());
        }

        let dir_name = parts.last().unwrap();
        let dir_path: Vec<String> = parts[..parts.len() - 1].to_vec();

        let current = self.navigate_to_directory_mut(&dir_path)?;

        let dir = TmpfsDirectory::new(dir_name.clone());
        current.add_subdirectory(dir)
    }

    /// Remove directory
    pub fn remove_directory(&mut self, path: &str) -> Result<(), String> {
        let parts: Vec<String> = path.split('/').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
        if parts.is_empty() {
            return Err("Invalid path".to_string());
        }

        let dir_name = parts.last().unwrap();
        let dir_path: Vec<String> = parts[..parts.len() - 1].to_vec();

        let current = self.navigate_to_directory_mut(&dir_path)?;
        current.remove_subdirectory(dir_name)
    }

    /// Get total size
    pub fn get_total_size(&self) -> usize {
        self.root.get_total_size()
    }

    /// Navigate to directory (immutable)
    fn navigate_to_directory(&self, path: &[String]) -> Result<&TmpfsDirectory, String> {
        let mut current = &self.root;

        for part in path {
            match current.get_subdirectory(part) {
                Some(dir) => current = dir,
                None => return Err(format!("Directory {} not found", part)),
            }
        }

        Ok(current)
    }

    /// Navigate to directory (mutable)
    fn navigate_to_directory_mut(&mut self, path: &[String]) -> Result<&mut TmpfsDirectory, String> {
        let mut current = &mut self.root;

        for part in path {
            match current.get_subdirectory_mut(part) {
                Some(dir) => current = dir,
                None => return Err(format!("Directory {} not found", part)),
            }
        }

        Ok(current)
    }
}

/// Tmpfs manager for system-wide tmpfs management
pub struct TmpfsManager {
    pub instances: Arc<Mutex<HashMap<u64, Tmpfs>>>,
    pub next_instance_id: Arc<Mutex<u64>>,
}

impl TmpfsManager {
    pub fn new() -> Self {
        Self {
            instances: Arc::new(Mutex::new(HashMap::new())),
            next_instance_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create tmpfs instance
    pub fn create_instance(&self, max_size: usize) -> u64 {
        let mut next_id = self.next_instance_id.lock().unwrap();
        let instance_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let tmpfs = Tmpfs::new(max_size);
        let mut instances = self.instances.lock().unwrap();
        instances.insert(instance_id, tmpfs);

        instance_id
    }

    /// Get instance
    pub fn get_instance(&self, instance_id: u64) -> Option<Tmpfs> {
        let instances = self.instances.lock().unwrap();
        instances.get(&instance_id).cloned()
    }

    /// Remove instance
    pub fn remove_instance(&self, instance_id: u64) -> Result<(), String> {
        let mut instances = self.instances.lock().unwrap();
        match instances.remove(&instance_id) {
            Some(_) => Ok(()),
            None => Err(format!("Tmpfs instance {} not found", instance_id)),
        }
    }

    /// Get instance count
    pub fn instance_count(&self) -> usize {
        let instances = self.instances.lock().unwrap();
        instances.len()
    }
}

impl Default for TmpfsManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tmpfs_file() {
        let file = TmpfsFile::new("test.txt".to_string(), 0o644);
        assert_eq!(file.name, "test.txt");
        assert_eq!(file.size, 0);
    }

    #[test]
    fn test_tmpfs_file_write() {
        let mut file = TmpfsFile::new("test.txt".to_string(), 0o644);
        file.write(b"Hello".to_vec());
        assert_eq!(file.size, 5);
    }

    #[test]
    fn test_tmpfs_file_read() {
        let mut file = TmpfsFile::new("test.txt".to_string(), 0o644);
        file.write(b"Hello".to_vec());
        let data = file.read();
        assert_eq!(data, b"Hello");
    }

    #[test]
    fn test_tmpfs_directory() {
        let dir = TmpfsDirectory::new("test".to_string());
        assert_eq!(dir.name, "test");
    }

    #[test]
    fn test_tmpfs_directory_add_file() {
        let mut dir = TmpfsDirectory::new("test".to_string());
        let file = TmpfsFile::new("test.txt".to_string(), 0o644);
        dir.add_file(file).unwrap();
        assert_eq!(dir.list_files().len(), 1);
    }

    #[test]
    fn test_tmpfs() {
        let tmpfs = Tmpfs::new(1024);
        assert_eq!(tmpfs.max_size, 1024);
    }

    #[test]
    fn test_tmpfs_create_file() {
        let mut tmpfs = Tmpfs::new(1024);
        tmpfs.create_file("/test.txt", 0o644).unwrap();
        assert!(tmpfs.read_file("/test.txt").is_ok());
    }

    #[test]
    fn test_tmpfs_write_read_file() {
        let mut tmpfs = Tmpfs::new(1024);
        tmpfs.create_file("/test.txt", 0o644).unwrap();
        tmpfs.write_file("/test.txt", b"Hello".to_vec()).unwrap();
        let data = tmpfs.read_file("/test.txt").unwrap();
        assert_eq!(data, b"Hello");
    }

    #[test]
    fn test_tmpfs_remove_file() {
        let mut tmpfs = Tmpfs::new(1024);
        tmpfs.create_file("/test.txt", 0o644).unwrap();
        tmpfs.remove_file("/test.txt").unwrap();
        assert!(tmpfs.read_file("/test.txt").is_err());
    }

    #[test]
    fn test_tmpfs_create_directory() {
        let mut tmpfs = Tmpfs::new(1024);
        tmpfs.create_directory("/test").unwrap();
        assert!(tmpfs.create_directory("/test/subdir").is_ok());
    }

    #[test]
    fn test_tmpfs_manager() {
        let manager = TmpfsManager::new();
        let instance_id = manager.create_instance(1024);
        assert_eq!(instance_id, 1);
        assert_eq!(manager.instance_count(), 1);
    }

    #[test]
    fn test_tmpfs_manager_invalid() {
        let manager = TmpfsManager::new();
        assert!(manager.get_instance(999).is_none());
    }
}
