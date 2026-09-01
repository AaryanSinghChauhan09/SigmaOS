#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
extern crate alloc;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

// SigmaOS File Manager
// OOP-based file management with advanced features

use crate::klib::BTreeMap;
// str/String not in no_std

/// File item
#[derive(Debug, Clone)]
pub struct FileItem {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub is_directory: bool,
    pub is_hidden: bool,
    pub is_readonly: bool,
    pub modified_at: u64,
    pub created_at: u64,
    pub file_type: FileType,
}

/// File type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Regular,
    Directory,
    Symlink,
    BlockDevice,
    CharDevice,
    Fifo,
    Socket,
}

/// View mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    List,
    Grid,
    Tree,
    Details,
}

/// Sort order
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Name,
    Size,
    Date,
    Type,
}

/// Clipboard operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardOperation {
    Copy,
    Cut,
}

/// OOP trait for file operations
pub trait FileOperation {
    /// Copy file
    fn copy(&self, source: &str, destination: &str) -> Result<(), FileManagerError>;
    /// Move file
    fn move_file(&self, source: &str, destination: &str) -> Result<(), FileManagerError>;
    /// Delete file
    fn delete(&self, path: &str) -> Result<(), FileManagerError>;
    /// Create directory
    fn create_directory(&self, path: &str) -> Result<(), FileManagerError>;
    /// Get operation name
    fn name(&self) -> &str;
}

/// Standard file operation implementation
pub struct StandardFileOperation;

impl FileOperation for StandardFileOperation {
    fn copy(&self, _source: &str, _destination: &str) -> Result<(), FileManagerError> {
        // Simulated copy operation
        // In real implementation, would use std::fs::copy
        Ok(())
    }

    fn move_file(&self, _source: &str, _destination: &str) -> Result<(), FileManagerError> {
        // Simulated move operation
        // In real implementation, would use std::fs::rename
        Ok(())
    }

    fn delete(&self, _path: &str) -> Result<(), FileManagerError> {
        // Simulated delete operation
        // In real implementation, would use std::fs::remove_file or remove_dir_all
        Ok(())
    }

    fn create_directory(&self, _path: &str) -> Result<(), FileManagerError> {
        // Simulated directory creation
        // In real implementation, would use std::fs::create_dir_all
        Ok(())
    }

    fn name(&self) -> &str {
        "StandardFileOperation"
    }
}

/// OOP-based File Manager
pub struct FileManager {
    current_path: String,
    view_mode: ViewMode,
    sort_order: SortOrder,
    show_hidden: bool,
    file_operation: Box<dyn FileOperation>,
    clipboard: Vec<(String, ClipboardOperation)>,
    bookmarks: BTreeMap<String, String>,
    recent_paths: Vec<String>,
}

impl FileManager {
    pub fn new(current_path: String, file_operation: Box<dyn FileOperation>) -> Self {
        Self {
            current_path,
            view_mode: ViewMode::List,
            sort_order: SortOrder::Name,
            show_hidden: false,
            file_operation,
            clipboard: Vec::new(),
            bookmarks: BTreeMap::new(),
            recent_paths: Vec::new(),
        }
    }

    /// Set view mode
    pub fn with_view_mode(mut self, mode: ViewMode) -> Self {
        self.view_mode = mode;
        self
    }

    /// Set sort order
    pub fn with_sort_order(mut self, order: SortOrder) -> Self {
        self.sort_order = order;
        self
    }

    /// Show hidden files
    pub fn with_show_hidden(mut self, show: bool) -> Self {
        self.show_hidden = show;
        self
    }

    /// List directory contents
    pub fn list_directory(&self, path: &str) -> Result<Vec<FileItem>, FileManagerError> {
        let mut items = Vec::new();

        // Simulated directory listing
        // In real implementation, would use std::fs::read_dir
        let simulated_entries = vec![
            ("Documents", true, 0),
            ("Downloads", true, 0),
            ("Desktop", true, 0),
            ("Pictures", true, 0),
            ("file.txt", false, 1024),
            ("document.pdf", false, 2048),
        ];

        for (name, is_dir, size) in simulated_entries {
            if !self.show_hidden && name.starts_with('.') {
                continue;
            }

            items.push(FileItem {
                name: name.to_string(),
                path: format!("{}/{}", path, name),
                size_bytes: size,
                is_directory: is_dir,
                is_hidden: name.starts_with('.'),
                is_readonly: false,
                modified_at: 1700000000u64,
                created_at: 1700000000u64,
                file_type: if is_dir {
                    FileType::Directory
                } else {
                    FileType::Regular
                },
            });
        }

        // Sort items
        self.sort_items(&mut items);

        Ok(items)
    }

    /// Sort items
    fn sort_items(&self, items: &mut Vec<FileItem>) {
        match self.sort_order {
            SortOrder::Name => items.sort_by(|a, b| a.name.cmp(&b.name)),
            SortOrder::Size => items.sort_by(|a, b| a.size_bytes.cmp(&b.size_bytes)),
            SortOrder::Date => items.sort_by(|a, b| a.modified_at.cmp(&b.modified_at)),
            SortOrder::Type => items.sort_by(|a, b| {
                a.is_directory
                    .cmp(&b.is_directory)
                    .then_with(|| a.name.cmp(&b.name))
            }),
        }
    }

    /// Navigate to path
    pub fn navigate(&mut self, path: &str) -> Result<(), FileManagerError> {
        if !path.starts_with('/') {
            return Err(FileManagerError::InvalidPath(path.to_string()));
        }

        self.current_path = path.to_string();
        self.add_to_recent(path);
        Ok(())
    }

    /// Navigate up
    pub fn navigate_up(&mut self) -> Result<(), FileManagerError> {
        if let Some(idx) = self.current_path.rfind('/') {
            if idx == 0 {
                self.current_path = String::from("/");
            } else {
                self.current_path = self.current_path[..idx].to_string();
            }
            Ok(())
        } else {
            Err(FileManagerError::AlreadyAtRoot)
        }
    }

    /// Navigate to home
    pub fn navigate_home(&mut self) {
        self.current_path = String::from("/home/user");
    }

    /// Get current path
    pub fn current_path(&self) -> &str {
        &self.current_path
    }

    /// Copy file to clipboard
    pub fn copy_to_clipboard(&mut self, path: String) {
        self.clipboard.push((path, ClipboardOperation::Copy));
    }

    /// Cut file to clipboard
    pub fn cut_to_clipboard(&mut self, path: String) {
        self.clipboard.push((path, ClipboardOperation::Cut));
    }

    /// Paste from clipboard
    pub fn paste(&mut self) -> Result<(), FileManagerError> {
        let current_path = self.current_path.clone();
        for (path, operation) in self.clipboard.drain(..) {
            let filename = if let Some(idx) = path.rfind('/') {
                &path[idx + 1..]
            } else {
                &path
            };
            let destination = format!("{}/{}", current_path, filename);

            match operation {
                ClipboardOperation::Copy => {
                    self.file_operation.copy(&path, &destination)?;
                }
                ClipboardOperation::Cut => {
                    self.file_operation.move_file(&path, &destination)?;
                }
            }
        }
        self.clipboard.clear();
        Ok(())
    }

    /// Clear clipboard
    pub fn clear_clipboard(&mut self) {
        self.clipboard.clear();
    }

    /// Delete file
    pub fn delete(&self, path: &str) -> Result<(), FileManagerError> {
        self.file_operation.delete(path)
    }

    /// Create directory
    pub fn create_directory(&self, name: &str) -> Result<(), FileManagerError> {
        let path = format!("{}/{}", self.current_path, name);
        self.file_operation.create_directory(&path)
    }

    /// Rename file
    pub fn rename(&self, old_path: &str, new_name: &str) -> Result<(), FileManagerError> {
        let parent = if let Some(idx) = old_path.rfind('/') {
            &old_path[..idx]
        } else {
            "."
        };
        let new_path = format!("{}/{}", parent, new_name);
        self.file_operation.move_file(old_path, &new_path)
    }

    /// Add bookmark
    pub fn add_bookmark(&mut self, name: String, path: String) {
        self.bookmarks.insert(name, path);
    }

    /// Remove bookmark
    pub fn remove_bookmark(&mut self, name: &str) {
        self.bookmarks.remove(&name.to_string());
    }

    /// Get bookmarks
    pub fn bookmarks(&self) -> Vec<(&String, &String)> {
        self.bookmarks.iter().collect()
    }

    /// Navigate to bookmark
    pub fn navigate_to_bookmark(&mut self, name: &str) -> Result<(), FileManagerError> {
        if let Some(path) = self.bookmarks.get(&name.to_string()).cloned() {
            self.navigate(&path)
        } else {
            Err(FileManagerError::BookmarkNotFound(name.to_string()))
        }
    }

    /// Add to recent paths
    fn add_to_recent(&mut self, path: &str) {
        let path_str = path.to_string();
        if !self.recent_paths.contains(&path_str) {
            self.recent_paths.push(path_str);
            if self.recent_paths.len() > 10 {
                self.recent_paths.remove(0);
            }
        }
    }

    /// Get recent paths
    pub fn recent_paths(&self) -> &[String] {
        &self.recent_paths
    }

    /// Search files
    pub fn search(&self, query: &str) -> Vec<FileItem> {
        let mut results = Vec::new();

        if let Ok(items) = self.list_directory(&self.current_path) {
            let query_lower = query.to_lowercase();
            for item in items {
                if item.name.to_lowercase().contains(&query_lower) {
                    results.push(item);
                }
            }
        }

        results
    }

    /// Get file info
    pub fn get_file_info(&self, path: &str) -> Result<FileItem, FileManagerError> {
        let filename = if let Some(idx) = path.rfind('/') {
            &path[idx + 1..]
        } else {
            path
        };
        Ok(FileItem {
            name: filename.to_string(),
            path: path.to_string(),
            size_bytes: 1024,
            is_directory: false,
            is_hidden: filename.starts_with('.'),
            is_readonly: false,
            modified_at: 1700000000u64,
            created_at: 1700000000u64,
            file_type: FileType::Regular,
        })
    }

    /// Get view mode
    pub fn view_mode(&self) -> ViewMode {
        self.view_mode
    }

    /// Set view mode
    pub fn set_view_mode(&mut self, mode: ViewMode) {
        self.view_mode = mode;
    }

    /// Get sort order
    pub fn sort_order(&self) -> SortOrder {
        self.sort_order
    }

    /// Set sort order
    pub fn set_sort_order(&mut self, order: SortOrder) {
        self.sort_order = order;
    }

    /// Toggle hidden files
    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
    }

    /// Is showing hidden
    pub fn is_showing_hidden(&self) -> bool {
        self.show_hidden
    }
}

impl Default for FileManager {
    fn default() -> Self {
        Self::new(String::from("/home/user"), Box::new(StandardFileOperation))
            .with_view_mode(ViewMode::List)
            .with_sort_order(SortOrder::Name)
            .with_show_hidden(false)
    }
}

/// File manager errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileManagerError {
    FileNotFound(String),
    PermissionDenied(String),
    InvalidPath(String),
    AlreadyAtRoot,
    BookmarkNotFound(String),
    OperationFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_item() {
        let item = FileItem {
            name: "test.txt".to_string(),
            path: String::from("/test/test.txt"),
            size_bytes: 1024,
            is_directory: false,
            is_hidden: false,
            is_readonly: false,
            modified_at: 1234567890,
            created_at: 1234567890,
            file_type: FileType::Regular,
        };
        assert_eq!(item.name, "test.txt");
    }

    #[test]
    fn test_standard_file_operation() {
        let op = StandardFileOperation;
        assert_eq!(op.name(), "StandardFileOperation");
    }

    #[test]
    fn test_file_manager() {
        let manager = FileManager::default();
        assert_eq!(manager.current_path(), String::from("/home/user"));
    }

    #[test]
    fn test_list_directory() {
        let manager = FileManager::default();
        let path = String::from("/home/user");
        let items = manager.list_directory(&path).unwrap();
        assert!(!items.is_empty());
    }

    #[test]
    fn test_navigate() {
        let mut manager = FileManager::default();
        let path = String::from("/home/user/Documents");
        manager.navigate(&path).unwrap();
        assert_eq!(manager.current_path(), String::from("/home/user/Documents"));
    }

    #[test]
    fn test_add_bookmark() {
        let mut manager = FileManager::default();
        manager.add_bookmark(
            "Documents".to_string(),
            String::from("/home/user/Documents"),
        );
        assert_eq!(manager.bookmarks().len(), 1);
    }
}
