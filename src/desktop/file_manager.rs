// SPDX-License-Identifier: MIT
// SigmaOS - File Manager
// Inspired by Nautilus (GNOME Files)
// Provides file browsing and management

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

/// File type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Regular,
    Directory,
    Symlink,
    BlockDevice,
    CharDevice,
    FIFO,
    Socket,
}

/// File permission
#[derive(Debug, Clone, Copy)]
pub struct FilePermission {
    pub user_read: bool,
    pub user_write: bool,
    pub user_execute: bool,
    pub group_read: bool,
    pub group_write: bool,
    pub group_execute: bool,
    pub other_read: bool,
    pub other_write: bool,
    pub other_execute: bool,
}

/// File metadata
#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub size: u64,
    pub modified: i64,
    pub accessed: i64,
    pub created: i64,
    pub permissions: FilePermission,
    pub owner: u32,
    pub group: u32,
}

/// File
#[derive(Debug, Clone)]
pub struct File {
    pub path: String,
    pub name: String,
    pub file_type: FileType,
    pub metadata: FileMetadata,
    pub is_hidden: bool,
}

/// View mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Icon,
    List,
    Compact,
    Tree,
}

/// Sort order
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Name,
    Size,
    Type,
    Modified,
    Created,
}

/// Selection mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionMode {
    Single,
    Multiple,
}

/// File manager
pub struct FileManager {
    pub current_path: String,
    pub files: Vec<File>,
    pub selected_files: Vec<String>,
    pub view_mode: ViewMode,
    pub sort_order: SortOrder,
    pub show_hidden: bool,
    pub history: Vec<String>,
    pub history_index: usize,
    bookmarks: Vec<String>,
    next_file_id: AtomicU32,
}

impl FileManager {
    pub fn new(initial_path: String) -> Self {
        Self {
            current_path: initial_path.clone(),
            files: Vec::new(),
            selected_files: Vec::new(),
            view_mode: ViewMode::Icon,
            sort_order: SortOrder::Name,
            show_hidden: false,
            history: vec![initial_path],
            history_index: 0,
            bookmarks: Vec::new(),
            next_file_id: AtomicU32::new(1),
        }
    }

    /// Set current path
    pub fn set_path(&mut self, path: String) -> Result<(), &'static str> {
        self.current_path = path.clone();
        if self.history_index == self.history.len() - 1 {
            self.history.push(path);
            self.history_index = self.history.len() - 1;
        } else {
            self.history.truncate(self.history_index + 1);
            self.history.push(path);
            self.history_index = self.history.len() - 1;
        }
        self.load_files();
        Ok(())
    }

    /// Load files from current path
    pub fn load_files(&mut self) {
        // Simplified - in real implementation would read actual filesystem
        self.files.clear();
    }

    /// Navigate to parent directory
    pub fn go_up(&mut self) -> Result<(), &'static str> {
        let parent = self.get_parent_path(&self.current_path);
        self.set_path(parent)
    }

    /// Navigate back in history
    pub fn go_back(&mut self) -> Result<(), &'static str> {
        if self.history_index > 0 {
            self.history_index -= 1;
            self.current_path = self.history[self.history_index].clone();
            self.load_files();
            Ok(())
        } else {
            Err("No more history")
        }
    }

    /// Navigate forward in history
    pub fn go_forward(&mut self) -> Result<(), &'static str> {
        if self.history_index < self.history.len() - 1 {
            self.history_index += 1;
            self.current_path = self.history[self.history_index].clone();
            self.load_files();
            Ok(())
        } else {
            Err("No forward history")
        }
    }

    /// Get parent path
    fn get_parent_path(&self, path: &str) -> String {
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() <= 1 {
            return "/".to_string();
        }
        let parent = parts[..parts.len() - 1].join("/");
        if parent.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", parent)
        }
    }

    /// Create directory
    pub fn create_directory(&mut self, name: String) -> Result<(), &'static str> {
        let path = format!("{}/{}", self.current_path, name);
        // Simplified - in real implementation would create actual directory
        Ok(())
    }

    /// Create file
    pub fn create_file(&mut self, name: String) -> Result<(), &'static str> {
        let path = format!("{}/{}", self.current_path, name);
        // Simplified - in real implementation would create actual file
        Ok(())
    }

    /// Delete file
    pub fn delete_file(&mut self, path: String) -> Result<(), &'static str> {
        // Simplified - in real implementation would delete actual file
        Ok(())
    }

    /// Copy file
    pub fn copy_file(&mut self, src: String, dst: String) -> Result<(), &'static str> {
        // Simplified - in real implementation would copy actual file
        Ok(())
    }

    /// Move file
    pub fn move_file(&mut self, src: String, dst: String) -> Result<(), &'static str> {
        // Simplified - in real implementation would move actual file
        Ok(())
    }

    /// Rename file
    pub fn rename_file(&mut self, old_name: String, new_name: String) -> Result<(), &'static str> {
        let old_path = format!("{}/{}", self.current_path, old_name);
        let new_path = format!("{}/{}", self.current_path, new_name);
        self.move_file(old_path, new_path)
    }

    /// Select file
    pub fn select_file(&mut self, path: String) {
        if !self.selected_files.contains(&path) {
            self.selected_files.push(path);
        }
    }

    /// Deselect file
    pub fn deselect_file(&mut self, path: String) {
        self.selected_files.retain(|p| p != &path);
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selected_files.clear();
    }

    /// Set view mode
    pub fn set_view_mode(&mut self, mode: ViewMode) {
        self.view_mode = mode;
    }

    /// Set sort order
    pub fn set_sort_order(&mut self, order: SortOrder) {
        self.sort_order = order;
        self.sort_files();
    }

    /// Sort files
    fn sort_files(&mut self) {
        match self.sort_order {
            SortOrder::Name => {
                self.files.sort_by(|a, b| a.name.cmp(&b.name));
            }
            SortOrder::Size => {
                self.files
                    .sort_by(|a, b| a.metadata.size.cmp(&b.metadata.size));
            }
            SortOrder::Type => {
                self.files.sort_by(|a, b| {
                    let a_type = format!("{:?}", a.file_type);
                    let b_type = format!("{:?}", b.file_type);
                    a_type.cmp(&b_type)
                });
            }
            SortOrder::Modified => {
                self.files
                    .sort_by(|a, b| a.metadata.modified.cmp(&b.metadata.modified));
            }
            SortOrder::Created => {
                self.files
                    .sort_by(|a, b| a.metadata.created.cmp(&b.metadata.created));
            }
        }
    }

    /// Toggle hidden files
    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        self.load_files();
    }

    /// Add bookmark
    pub fn add_bookmark(&mut self, path: String) {
        if !self.bookmarks.contains(&path) {
            self.bookmarks.push(path);
        }
    }

    /// Remove bookmark
    pub fn remove_bookmark(&mut self, path: String) {
        self.bookmarks.retain(|p| p != &path);
    }

    /// Get bookmarks
    pub fn get_bookmarks(&self) -> &[String] {
        &self.bookmarks
    }

    /// Search files
    pub fn search_files(&self, query: &str) -> Vec<&File> {
        self.files
            .iter()
            .filter(|f| f.name.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }

    /// Get file info
    pub fn get_file_info(&self, path: &str) -> Option<&File> {
        self.files.iter().find(|f| f.path == path)
    }

    /// Can go back
    pub fn can_go_back(&self) -> bool {
        self.history_index > 0
    }

    /// Can go forward
    pub fn can_go_forward(&self) -> bool {
        self.history_index < self.history.len() - 1
    }
}

/// Global file manager
static mut FILE_MANAGER: Option<FileManager> = None;

/// Initialize file manager
pub fn init_file_manager(initial_path: String) {
    unsafe {
        FILE_MANAGER = Some(FileManager::new(initial_path));
    }
}

/// Get file manager
pub fn get_file_manager() -> &'static mut FileManager {
    unsafe { FILE_MANAGER.as_mut().expect("File manager not initialized") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_manager_creation() {
        let fm = FileManager::new("/home".to_string());
        assert_eq!(fm.current_path, "/home");
    }

    #[test]
    fn test_set_path() {
        let mut fm = FileManager::new("/home".to_string());
        fm.set_path("/home/user".to_string()).unwrap();
        assert_eq!(fm.current_path, "/home/user");
    }

    #[test]
    fn test_go_up() {
        let mut fm = FileManager::new("/home/user".to_string());
        fm.go_up().unwrap();
        assert_eq!(fm.current_path, "/home");
    }

    #[test]
    fn test_go_back() {
        let mut fm = FileManager::new("/home".to_string());
        fm.set_path("/home/user".to_string()).unwrap();
        fm.go_back().unwrap();
        assert_eq!(fm.current_path, "/home");
    }

    #[test]
    fn test_select_file() {
        let mut fm = FileManager::new("/home".to_string());
        fm.select_file("/home/file.txt".to_string());
        assert_eq!(fm.selected_files.len(), 1);
    }

    #[test]
    fn test_bookmarks() {
        let mut fm = FileManager::new("/home".to_string());
        fm.add_bookmark("/home/user".to_string());
        assert_eq!(fm.bookmarks.len(), 1);
        fm.remove_bookmark("/home/user".to_string());
        assert_eq!(fm.bookmarks.len(), 0);
    }
}
