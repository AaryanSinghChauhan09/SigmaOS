//! File Manager (Nautilus/Thunar Inspiration)
//! File navigation, operations, and file properties

// #![no_std]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;

const MAX_CLIPBOARD_ITEMS: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    RegularFile,
    Directory,
}

#[derive(Debug, Clone, Copy)]
pub struct FileEntry {
    pub inode_id: u32,
    pub name_hash: u32,
    pub size: u32,
    pub file_type: FileType,
}

pub struct Pane {
    pub current_directory_inode: u32,
    pub entries: [Option<FileEntry>; 16],
    pub selected_idx: usize,
}

impl Pane {
    pub fn new(root_inode: u32) -> Self {
        const EMPTY_ENTRY: Option<FileEntry> = None;
        Self {
            current_directory_inode: root_inode,
            entries: [EMPTY_ENTRY; 16],
            selected_idx: 0,
        }
    }
}

pub struct ClipboardBuffer {
    pub items: [Option<FileEntry>; MAX_CLIPBOARD_ITEMS],
    pub is_cut: bool,
}

impl ClipboardBuffer {
    pub fn new() -> Self {
        const EMPTY_ENTRY: Option<FileEntry> = None;
        Self {
            items: [EMPTY_ENTRY; MAX_CLIPBOARD_ITEMS],
            is_cut: false,
        }
    }

    pub fn clear(&mut self) {
        self.items.fill(None);
        self.is_cut = false;
    }
}

impl Default for ClipboardBuffer {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SovereignFileManager {
    pub active_pane: Pane,
    pub clipboard: ClipboardBuffer,
}

impl SovereignFileManager {
    pub fn new() -> Self {
        Self {
            active_pane: Pane::new(0),
            clipboard: ClipboardBuffer::new(),
        }
    }
}

impl Default for SovereignFileManager {
    fn default() -> Self {
        Self::new()
    }
}

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

impl Default for Clipboard {
    fn default() -> Self {
        Self::new()
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
        Ok(())
    }

    pub fn create_file(&mut self, _name: &str) -> Result<(), FMError> {
        Ok(())
    }

    pub fn create_directory(&mut self, _name: &str) -> Result<(), FMError> {
        Ok(())
    }

    pub fn delete_files(&mut self) -> Result<(), FMError> {
        Ok(())
    }

    pub fn rename_file(&mut self, _old_name: &str, _new_name: &str) -> Result<(), FMError> {
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
    fn test_sovereign_file_manager() {
        let mut sfm = SovereignFileManager::new();
        assert_eq!(sfm.active_pane.current_directory_inode, 0);
        sfm.clipboard.is_cut = true;
        assert!(sfm.clipboard.is_cut);
    }

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
