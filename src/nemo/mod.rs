// SPDX-License-Identifier: MIT
// SigmaOS Nemo-Inspired File Manager
// Linux Mint Nemo-inspired file manager with advanced features

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

/// Nemo-inspired file operation types
#[derive(Debug, Clone, PartialEq)]
pub enum NemoFileOperation {
    Copy,
    Move,
    Delete,
    Rename,
    CreateFolder,
    Compress,
    Extract,
    Properties,
}

/// Nemo-inspired view types
#[derive(Debug, Clone, PartialEq)]
pub enum NemoViewType {
    IconView,
    ListView,
    CompactView,
    DetailedView,
}

/// Nemo-inspired file information
#[derive(Debug, Clone)]
pub struct NemoFileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub modified: String,
    pub is_directory: bool,
    pub is_hidden: bool,
    pub permissions: String,
    pub owner: String,
    pub group: String,
    pub mime_type: String,
}

impl NemoFileInfo {
    pub fn new(name: String, path: String) -> Self {
        Self {
            name,
            path,
            size: 0,
            modified: String::new(),
            is_directory: false,
            is_hidden: false,
            permissions: String::from("644"),
            owner: String::from("user"),
            group: String::from("user"),
            mime_type: String::new(),
        }
    }
}

/// Nemo-inspired bookmark entry
#[derive(Debug, Clone)]
pub struct NemoBookmark {
    pub name: String,
    pub uri: String,
    pub icon: String,
    pub metadata: BTreeMap<String, String>,
}

impl NemoBookmark {
    pub fn new(name: String, uri: String) -> Self {
        Self {
            name,
            uri,
            icon: String::from("folder"),
            metadata: BTreeMap::new(),
        }
    }

    /// Add metadata to bookmark
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
}

/// Nemo-inspired file manager
#[derive(Debug, Clone)]
pub struct NemoFileManager {
    pub current_path: String,
    pub view_type: NemoViewType,
    pub files: Vec<NemoFileInfo>,
    pub bookmarks: Vec<NemoBookmark>,
    pub show_hidden: bool,
    pub terminal_enabled: bool,
    pub root_enabled: bool,
    pub navigation_history: Vec<String>,
    pub history_index: usize,
    pub clipboard_files: Vec<String>,
    pub clipboard_operation: Option<NemoFileOperation>,
}

impl NemoFileManager {
    pub fn new() -> Self {
        Self {
            current_path: String::from("/home/user"),
            view_type: NemoViewType::IconView,
            files: Vec::new(),
            bookmarks: Vec::new(),
            show_hidden: false,
            terminal_enabled: true,
            root_enabled: true,
            navigation_history: Vec::new(),
            history_index: 0,
            clipboard_files: Vec::new(),
            clipboard_operation: None,
        }
    }

    /// Navigate to path
    pub fn navigate_to(&mut self, path: String) -> Result<(), &'static str> {
        if self.current_path != path {
            self.navigation_history.push(self.current_path.clone());
            self.history_index = self.navigation_history.len() - 1;
            self.current_path = path;
            self.refresh_files();
        }
        Ok(())
    }

    /// Navigate back
    pub fn navigate_back(&mut self) -> Result<(), &'static str> {
        if self.history_index > 0 {
            self.history_index -= 1;
            self.current_path = self.navigation_history[self.history_index].clone();
            self.refresh_files();
            Ok(())
        } else {
            Err("No previous location in history")
        }
    }

    /// Navigate forward
    pub fn navigate_forward(&mut self) -> Result<(), &'static str> {
        if self.history_index < self.navigation_history.len() - 1 {
            self.history_index += 1;
            self.current_path = self.navigation_history[self.history_index].clone();
            self.refresh_files();
            Ok(())
        } else {
            Err("No next location in history")
        }
    }

    /// Navigate up
    pub fn navigate_up(&mut self) -> Result<(), &'static str> {
        let parent_path = self.get_parent_path(&self.current_path);
        self.navigate_to(parent_path)
    }

    /// Refresh files
    pub fn refresh_files(&mut self) {
        // In real implementation, would scan filesystem
        self.files.clear();
    }

    /// Toggle hidden files
    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
    }

    /// Open in terminal
    pub fn open_in_terminal(&self) -> Result<(), &'static str> {
        if !self.terminal_enabled {
            return Err("Terminal not enabled");
        }
        // In real implementation, would launch terminal in current path
        Ok(())
    }

    /// Open as root
    pub fn open_as_root(&self) -> Result<(), &'static str> {
        if !self.root_enabled {
            return Err("Root access not enabled");
        }
        // In real implementation, would launch with elevated privileges
        Ok(())
    }

    /// Add bookmark
    pub fn add_bookmark(&mut self, bookmark: NemoBookmark) {
        self.bookmarks.push(bookmark);
    }

    /// Remove bookmark
    pub fn remove_bookmark(&mut self, index: usize) {
        if index < self.bookmarks.len() {
            self.bookmarks.remove(index);
        }
    }

    /// Copy files to clipboard
    pub fn copy_to_clipboard(&mut self, files: Vec<String>) {
        self.clipboard_files = files;
        self.clipboard_operation = Some(NemoFileOperation::Copy);
    }

    /// Cut files to clipboard
    pub fn cut_to_clipboard(&mut self, files: Vec<String>) {
        self.clipboard_files = files;
        self.clipboard_operation = Some(NemoFileOperation::Move);
    }

    /// Paste from clipboard
    pub fn paste_from_clipboard(&mut self) -> Result<(), &'static str> {
        if self.clipboard_files.is_empty() {
            return Err("Clipboard is empty");
        }
        if let Some(operation) = &self.clipboard_operation {
            match operation {
                NemoFileOperation::Copy => {
                    // In real implementation, would copy files
                }
                NemoFileOperation::Move => {
                    // In real implementation, would move files
                }
                _ => return Err("Invalid clipboard operation"),
            }
        }
        Ok(())
    }

    /// Set view type
    pub fn set_view_type(&mut self, view_type: NemoViewType) {
        self.view_type = view_type;
    }

    /// Get parent path
    fn get_parent_path(&self, path: &str) -> String {
        if let Some(last_slash) = path.rfind('/') {
            if last_slash == 0 {
                String::from("/")
            } else {
                path[..last_slash].to_string()
            }
        } else {
            String::from("/")
        }
    }

    /// SSH into remote server
    pub fn ssh_connect(&self, _server: String) -> Result<(), &'static str> {
        // In real implementation, would establish SSH connection
        Ok(())
    }

    /// Connect to FTP server
    pub fn ftp_connect(&self, _server: String) -> Result<(), &'static str> {
        // In real implementation, would establish FTP connection
        Ok(())
    }

    /// Get file operations progress
    pub fn get_operation_progress(&self) -> f32 {
        // In real implementation, would return actual progress
        0.0
    }
}

impl Default for NemoFileManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nemo_file_info() {
        let file = NemoFileInfo::new(String::from("test.txt"), String::from("/home/user/test.txt"));
        assert_eq!(file.name, String::from("test.txt"));
        assert_eq!(file.is_directory, false);
    }

    #[test]
    fn test_nemo_bookmark() {
        let bookmark = NemoBookmark::new(String::from("Home"), String::from("file:///home/user"));
        assert_eq!(bookmark.name, String::from("Home"));
        assert_eq!(bookmark.uri, String::from("file:///home/user"));
    }

    #[test]
    fn test_nemo_file_manager() {
        let mut manager = NemoFileManager::new();
        assert_eq!(manager.current_path, String::from("/home/user"));
        
        manager.navigate_to(String::from("/tmp")).unwrap();
        assert_eq!(manager.current_path, String::from("/tmp"));
    }

    #[test]
    fn test_navigation() {
        let mut manager = NemoFileManager::new();
        // Test basic navigation
        manager.navigate_to(String::from("/tmp")).unwrap();
        assert_eq!(manager.current_path, String::from("/tmp"));
        
        manager.navigate_to(String::from("/var")).unwrap();
        assert_eq!(manager.current_path, String::from("/var"));
        
        // Test navigate up
        manager.navigate_up().unwrap();
        assert_eq!(manager.current_path, String::from("/"));
    }

    #[test]
    fn test_bookmarks() {
        let mut manager = NemoFileManager::new();
        let mut bookmark = NemoBookmark::new(String::from("Documents"), String::from("file:///home/user/Documents"));
        bookmark.add_metadata(String::from("type"), String::from("folder"));
        manager.add_bookmark(bookmark);
        
        assert_eq!(manager.bookmarks.len(), 1);
        manager.remove_bookmark(0);
        assert_eq!(manager.bookmarks.len(), 0);
    }

    #[test]
    fn test_clipboard_operations() {
        let mut manager = NemoFileManager::new();
        let files = vec![String::from("/home/user/test.txt")];
        
        manager.copy_to_clipboard(files.clone());
        assert!(manager.clipboard_operation == Some(NemoFileOperation::Copy));
        
        manager.cut_to_clipboard(files);
        assert!(manager.clipboard_operation == Some(NemoFileOperation::Move));
    }

    #[test]
    fn test_terminal_and_root() {
        let manager = NemoFileManager::new();
        assert!(manager.open_in_terminal().is_ok());
        assert!(manager.open_as_root().is_ok());
    }

    #[test]
    fn test_view_type() {
        let mut manager = NemoFileManager::new();
        manager.set_view_type(NemoViewType::ListView);
        assert_eq!(manager.view_type, NemoViewType::ListView);
    }
}