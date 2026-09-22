// Linux-inspired tmpfs virtual filesystem
// Temporary storage filesystem for SigmaOS

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// tmpfs file type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TmpFileType {
    Directory,
    File,
}

/// tmpfs file
#[derive(Debug, Clone)]
pub struct TmpFile {
    pub name: String,
    pub file_type: TmpFileType,
    pub content: Vec<u8>,
    pub permissions: u32,
    pub children: Vec<String>,
    pub size: usize,
}

impl TmpFile {
    pub fn new(name: String, file_type: TmpFileType) -> Self {
        TmpFile {
            name,
            file_type,
            content: Vec::new(),
            permissions: 0o644,
            children: Vec::new(),
            size: 0,
        }
    }

    pub fn with_permissions(mut self, permissions: u32) -> Self {
        self.permissions = permissions;
        self
    }

    pub fn add_child(&mut self, child: String) {
        self.children.push(child);
    }

    pub fn write(&mut self, data: &[u8]) {
        self.content = data.to_vec();
        self.size = data.len();
    }

    pub fn read(&self) -> Vec<u8> {
        self.content.clone()
    }

    pub fn append(&mut self, data: &[u8]) {
        self.content.extend_from_slice(data);
        self.size = self.content.len();
    }
}

/// tmpfs instance
pub struct Tmpfs {
    files: HashMap<String, Arc<Mutex<TmpFile>>>,
    total_size: usize,
    max_size: usize,
}

impl Tmpfs {
    pub fn new(max_size: usize) -> Self {
        let mut tmpfs = Tmpfs {
            files: HashMap::new(),
            total_size: 0,
            max_size,
        };

        // Create root directory
        tmpfs.create_file("/".to_string(), TmpFileType::Directory);
        tmpfs
    }

    pub fn create_file(&mut self, path: String, file_type: TmpFileType) -> Arc<Mutex<TmpFile>> {
        let file = Arc::new(Mutex::new(TmpFile::new(
            path.split('/').last().unwrap_or(&path).to_string(),
            file_type,
        )));

        self.files.insert(path.clone(), file.clone());

        // Add to parent directory
        if let Some(parent_path) = Self::parent_path(&path) {
            if let Some(parent) = self.files.get_mut(&parent_path) {
                let mut parent_guard = parent.lock().unwrap();
                parent_guard.add_child(path.clone());
            }
        }

        file
    }

    pub fn get_file(&self, path: &str) -> Option<Arc<Mutex<TmpFile>>> {
        self.files.get(path).cloned()
    }

    pub fn write_file(&mut self, path: &str, data: &[u8]) -> Result<(), String> {
        let file = self.files.get(path)
            .ok_or_else(|| format!("File not found: {}", path))?;

        let old_size = {
            let file_guard = file.lock().unwrap();
            file_guard.size
        };

        let new_size = data.len();

        // Check space
        if self.total_size - old_size + new_size > self.max_size {
            return Err("Not enough space".to_string());
        }

        let mut file_guard = file.lock().unwrap();
        file_guard.write(data);

        self.total_size = self.total_size - old_size + new_size;
        Ok(())
    }

    pub fn read_file(&self, path: &str) -> Result<Vec<u8>, String> {
        let file = self.files.get(path)
            .ok_or_else(|| format!("File not found: {}", path))?;

        let file_guard = file.lock().unwrap();
        if file_guard.file_type != TmpFileType::File {
            return Err("Not a file".to_string());
        }

        Ok(file_guard.read())
    }

    pub fn append_file(&mut self, path: &str, data: &[u8]) -> Result<(), String> {
        let file = self.files.get(path)
            .ok_or_else(|| format!("File not found: {}", path))?;

        let new_size = data.len();

        // Check space
        if self.total_size + new_size > self.max_size {
            return Err("Not enough space".to_string());
        }

        let mut file_guard = file.lock().unwrap();
        file_guard.append(data);

        self.total_size += new_size;
        Ok(())
    }

    pub fn delete_file(&mut self, path: &str) -> Result<(), String> {
        let file = self.files.remove(path)
            .ok_or_else(|| format!("File not found: {}", path))?;

        let size = file.lock().unwrap().size;
        self.total_size -= size;

        // Remove from parent directory
        if let Some(parent_path) = Self::parent_path(path) {
            if let Some(parent) = self.files.get_mut(&parent_path) {
                let mut parent_guard = parent.lock().unwrap();
                parent_guard.children.retain(|c| c != path);
            }
        }

        Ok(())
    }

    pub fn list_directory(&self, path: &str) -> Result<Vec<String>, String> {
        let file = self.files.get(path)
            .ok_or_else(|| format!("Directory not found: {}", path))?;

        let file_guard = file.lock().unwrap();
        if file_guard.file_type != TmpFileType::Directory {
            return Err("Not a directory".to_string());
        }

        Ok(file_guard.children.clone())
    }

    pub fn get_file_size(&self, path: &str) -> Result<usize, String> {
        let file = self.files.get(path)
            .ok_or_else(|| format!("File not found: {}", path))?;

        let file_guard = file.lock().unwrap();
        Ok(file_guard.size)
    }

    pub fn get_total_size(&self) -> usize {
        self.total_size
    }

    pub fn get_max_size(&self) -> usize {
        self.max_size
    }

    pub fn get_free_space(&self) -> usize {
        self.max_size - self.total_size
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
}

impl Default for Tmpfs {
    fn default() -> Self {
        Self::new(100 * 1024 * 1024) // 100 MB default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tmpfs_creation() {
        let tmpfs = Tmpfs::new(1024);
        assert_eq!(tmpfs.get_max_size(), 1024);
        assert_eq!(tmpfs.get_total_size(), 0);
    }

    #[test]
    fn test_tmpfs_create_file() {
        let mut tmpfs = Tmpfs::new(1024);
        tmpfs.create_file("/test".to_string(), TmpFileType::File);

        let file = tmpfs.get_file("/test");
        assert!(file.is_some());
    }

    #[test]
    fn test_tmpfs_write_file() {
        let mut tmpfs = Tmpfs::new(1024);
        tmpfs.create_file("/test".to_string(), TmpFileType::File);

        tmpfs.write_file("/test", b"hello").unwrap();

        let content = tmpfs.read_file("/test").unwrap();
        assert_eq!(content, b"hello");
    }

    #[test]
    fn test_tmpfs_append_file() {
        let mut tmpfs = Tmpfs::new(1024);
        tmpfs.create_file("/test".to_string(), TmpFileType::File);

        tmpfs.write_file("/test", b"hello").unwrap();
        tmpfs.append_file("/test", b" world").unwrap();

        let content = tmpfs.read_file("/test").unwrap();
        assert_eq!(content, b"hello world");
    }

    #[test]
    fn test_tmpfs_delete_file() {
        let mut tmpfs = Tmpfs::new(1024);
        tmpfs.create_file("/test".to_string(), TmpFileType::File);
        tmpfs.write_file("/test", b"hello").unwrap();

        tmpfs.delete_file("/test").unwrap();

        assert!(tmpfs.get_file("/test").is_none());
        assert_eq!(tmpfs.get_total_size(), 0);
    }

    #[test]
    fn test_tmpfs_space_limit() {
        let mut tmpfs = Tmpfs::new(10);
        tmpfs.create_file("/test".to_string(), TmpFileType::File);

        let result = tmpfs.write_file("/test", b"this is too long");
        assert!(result.is_err());
    }

    #[test]
    fn test_tmpfs_list_directory() {
        let mut tmpfs = Tmpfs::new(1024);
        tmpfs.create_file("/dir".to_string(), TmpFileType::Directory);
        tmpfs.create_file("/dir/file1".to_string(), TmpFileType::File);
        tmpfs.create_file("/dir/file2".to_string(), TmpFileType::File);

        let children = tmpfs.list_directory("/dir").unwrap();
        assert_eq!(children.len(), 2);
    }

    #[test]
    fn test_tmpfs_get_file_size() {
        let mut tmpfs = Tmpfs::new(1024);
        tmpfs.create_file("/test".to_string(), TmpFileType::File);
        tmpfs.write_file("/test", b"hello").unwrap();

        let size = tmpfs.get_file_size("/test").unwrap();
        assert_eq!(size, 5);
    }

    #[test]
    fn test_tmpfs_get_free_space() {
        let mut tmpfs = Tmpfs::new(1024);
        tmpfs.create_file("/test".to_string(), TmpFileType::File);
        tmpfs.write_file("/test", b"hello").unwrap();

        let free = tmpfs.get_free_space();
        assert_eq!(free, 1019);
    }
}
