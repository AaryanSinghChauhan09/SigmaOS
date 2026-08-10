//! File Manager (Nautilus/Thunar Inspiration)
//! File navigation, operations, and file properties

#![no_std]

extern crate alloc;

use crate::klib::{Vec, String};
use alloc::vec::Vec;
use alloc::string::String;

/// File
#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub is_directory: bool,
    pub is_hidden: bool,
}

impl File {
    pub fn new(name: &str, path: &str, is_directory: bool) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
            size: 0,
            is_directory,
            is_hidden: name.starts_with('.'),
        }
    }

    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }
}

/// Clipboard
#[derive(Debug, Clone)]
pub struct Clipboard {
    pub files: Vec<String>,
    pub operation: ClipboardOperation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardOperation {
    Copy,
    Cut,
}

impl Clipboard {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            operation: ClipboardOperation::Copy,
        }
    }

    pub fn add_file(&mut self, path: &str) {
        self.files.push(path.to_string());
    }

    pub fn set_operation(&mut self, operation: ClipboardOperation) {
        self.operation = operation;
    }

    pub fn clear(&mut self) {
        self.files.clear();
    }
}

/// File manager
pub struct FileManager {
    pub current_directory: String,
    pub selected_files: Vec<File>,
    pub clipboard: Clipboard,
    pub bookmarks: Vec<String>,
}

impl FileManager {
    pub fn new() -> Self {
        Self {
            current_directory: "/home/user".to_string(),
            selected_files: Vec::new(),
            clipboard: Clipboard::new(),
            bookmarks: vec![
                "/home/user".to_string(),
                "/".to_string(),
                "/tmp".to_string(),
            ],
        }
    }

    pub fn navigate(&mut self, path: &str) {
        self.current_directory = path.to_string();
    }

    pub fn select_file(&mut self, file: File) {
        self.selected_files.push(file);
    }

    pub fn clear_selection(&mut self) {
        self.selected_files.clear();
    }

    pub fn copy_files(&mut self) {
        self.clipboard.set_operation(ClipboardOperation::Copy);
        for file in &self.selected_files {
            self.clipboard.add_file(&file.path);
        }
    }

    pub fn cut_files(&mut self) {
        self.clipboard.set_operation(ClipboardOperation::Cut);
        for file in &self.selected_files {
            self.clipboard.add_file(&file.path);
        }
    }

    pub fn paste(&mut self) -> Result<(), FMError> {
        // Paste files from clipboard
        Ok(())
    }

    pub fn create_file(&mut self, name: &str) -> Result<(), FMError> {
        // Create new file
        Ok(())
    }

    pub fn create_directory(&mut self, name: &str) -> Result<(), FMError> {
        // Create new directory
        Ok(())
    }

    pub fn delete_files(&mut self) -> Result<(), FMError> {
        // Delete selected files
        Ok(())
    }

    pub fn rename_file(&mut self, old_name: &str, new_name: &str) -> Result<(), FMError> {
        // Rename file
        Ok(())
    }

    pub fn add_bookmark(&mut self, path: &str) {
        self.bookmarks.push(path.to_string());
    }

    pub fn remove_bookmark(&mut self, path: &str) {
        self.bookmarks.retain(|b| b != path);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FMError {
    FileNotFound,
    PermissionDenied,
    OperationFailed,
}

impl Default for FileManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file() {
        let file = File::new("test.txt", "/home/user/test.txt", false);
        assert_eq!(file.name, "test.txt");
    }

    #[test]
    fn test_clipboard() {
        let mut clipboard = Clipboard::new();
        clipboard.add_file("/test/file.txt");
        assert_eq!(clipboard.files.len(), 1);
    }

    #[test]
    fn test_file_manager() {
        let mut fm = FileManager::new();
        fm.navigate("/tmp");
        assert_eq!(fm.current_directory, "/tmp");
    }
}