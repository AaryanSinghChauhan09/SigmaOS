//! File Manager
//!
//! File management system inspired by Linux Mint's Nemo and Omarchy's file utilities,
//! supporting file operations, navigation, and file type handling.

use std::collections::HashMap;
use std::path::PathBuf;

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
    Unknown,
}

impl FileType {
    pub fn as_str(&self) -> &str {
        match self {
            FileType::Regular => "Regular",
            FileType::Directory => "Directory",
            FileType::Symlink => "Symlink",
            FileType::BlockDevice => "Block Device",
            FileType::CharDevice => "Char Device",
            FileType::Fifo => "FIFO",
            FileType::Socket => "Socket",
            FileType::Unknown => "Unknown",
        }
    }
}

/// File information
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub name: String,
    pub file_type: FileType,
    pub size_bytes: u64,
    pub modified_time: u64,
    pub permissions: u32,
    pub is_hidden: bool,
    pub is_executable: bool,
}

impl FileInfo {
    pub fn new(path: PathBuf, name: String, file_type: FileType) -> Self {
        Self {
            path,
            name,
            file_type,
            size_bytes: 0,
            modified_time: 0,
            permissions: 0o644,
            is_hidden: false,
            is_executable: false,
        }
    }

    pub fn set_size(&mut self, size: u64) {
        self.size_bytes = size;
    }

    pub fn set_modified_time(&mut self, time: u64) {
        self.modified_time = time;
    }

    pub fn set_permissions(&mut self, perms: u32) {
        self.permissions = perms;
    }

    pub fn set_hidden(&mut self, hidden: bool) {
        self.is_hidden = hidden;
    }

    pub fn set_executable(&mut self, exec: bool) {
        self.is_executable = exec;
    }

    pub fn size_human(&self) -> String {
        const GB: u64 = 1024 * 1024 * 1024;
        const MB: u64 = 1024 * 1024;
        const KB: u64 = 1024;

        if self.size_bytes >= GB {
            format!("{:.1} GB", self.size_bytes as f64 / GB as f64)
        } else if self.size_bytes >= MB {
            format!("{:.1} MB", self.size_bytes as f64 / MB as f64)
        } else if self.size_bytes >= KB {
            format!("{:.1} KB", self.size_bytes as f64 / KB as f64)
        } else {
            format!("{} B", self.size_bytes)
        }
    }

    pub fn is_directory(&self) -> bool {
        self.file_type == FileType::Directory
    }
}

/// View mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopViewMode {
    Icon,
    List,
    Compact,
    Tree,
}

impl DesktopViewMode {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "icon" => Some(DesktopViewMode::Icon),
            "list" => Some(DesktopViewMode::List),
            "compact" => Some(DesktopViewMode::Compact),
            "tree" => Some(DesktopViewMode::Tree),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            DesktopViewMode::Icon => "Icon",
            DesktopViewMode::List => "List",
            DesktopViewMode::Compact => "Compact",
            DesktopViewMode::Tree => "Tree",
        }
    }
}

/// Sort order
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopSortOrder {
    Name,
    Size,
    Modified,
    Type,
}

impl DesktopSortOrder {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "name" => Some(DesktopSortOrder::Name),
            "size" => Some(DesktopSortOrder::Size),
            "modified" | "date" => Some(DesktopSortOrder::Modified),
            "type" => Some(DesktopSortOrder::Type),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            DesktopSortOrder::Name => "Name",
            DesktopSortOrder::Size => "Size",
            DesktopSortOrder::Modified => "Modified",
            DesktopSortOrder::Type => "Type",
        }
    }
}

/// File manager configuration
#[derive(Debug, Clone)]
pub struct DesktopFileExplorerConfig {
    pub show_hidden: bool,
    pub view_mode: DesktopViewMode,
    pub sort_order: DesktopSortOrder,
    pub sort_reverse: bool,
    pub single_click: bool,
    pub show_thumbnails: bool,
}

impl Default for DesktopFileExplorerConfig {
    fn default() -> Self {
        Self {
            show_hidden: false,
            view_mode: DesktopViewMode::Icon,
            sort_order: DesktopSortOrder::Name,
            sort_reverse: false,
            single_click: false,
            show_thumbnails: true,
        }
    }
}

/// File manager
#[derive(Debug)]
pub struct DesktopFileExplorer {
    current_path: PathBuf,
    files: Vec<FileInfo>,
    history: Vec<PathBuf>,
    history_index: usize,
    config: DesktopFileExplorerConfig,
    bookmarks: HashMap<String, PathBuf>,
}

impl DesktopFileExplorer {
    pub fn new() -> Self {
        let home = PathBuf::from("/home/user");

        let mut explorer = Self {
            current_path: home.clone(),
            files: Vec::new(),
            history: vec![home],
            history_index: 0,
            config: DesktopFileExplorerConfig::default(),
            bookmarks: HashMap::new(),
        };
        explorer.load_files();
        explorer
    }

    /// Get current path
    pub fn get_current_path(&self) -> &PathBuf {
        &self.current_path
    }

    /// Get configuration
    pub fn get_config(&self) -> &DesktopFileExplorerConfig {
        &self.config
    }

    /// Set configuration
    pub fn set_config(&mut self, config: DesktopFileExplorerConfig) {
        self.config = config;
    }

    /// Navigate to path
    pub fn navigate(&mut self, path: PathBuf) -> Result<(), String> {
        if !path.is_absolute() {
            return Err("Path must be absolute".to_string());
        }

        self.current_path = path.clone();

        // Add to history
        if self.history_index < self.history.len() - 1 {
            self.history.truncate(self.history_index + 1);
        }
        self.history.push(path);
        self.history_index = self.history.len() - 1;

        // Load files (simulated)
        self.load_files();

        Ok(())
    }

    /// Navigate up
    pub fn navigate_up(&mut self) -> Result<(), String> {
        if let Some(parent) = self.current_path.parent() {
            self.navigate(parent.to_path_buf())
        } else {
            Err("Already at root".to_string())
        }
    }

    /// Navigate back in history
    pub fn navigate_back(&mut self) -> Result<(), String> {
        if self.history_index > 0 {
            self.history_index -= 1;
            self.current_path = self.history[self.history_index].clone();
            self.load_files();
            Ok(())
        } else {
            Err("No more history".to_string())
        }
    }

    /// Navigate forward in history
    pub fn navigate_forward(&mut self) -> Result<(), String> {
        if self.history_index < self.history.len() - 1 {
            self.history_index += 1;
            self.current_path = self.history[self.history_index].clone();
            self.load_files();
            Ok(())
        } else {
            Err("No more history".to_string())
        }
    }

    /// Load files in current directory (simulated)
    fn load_files(&mut self) {
        self.files.clear();

        // Simulate directory contents
        let sample_files = vec![
            ("Documents", FileType::Directory, 0),
            ("Downloads", FileType::Directory, 0),
            ("Pictures", FileType::Directory, 0),
            ("Music", FileType::Directory, 0),
            ("Videos", FileType::Directory, 0),
            ("readme.txt", FileType::Regular, 1024),
            ("script.sh", FileType::Regular, 512),
        ];

        for (name, file_type, size) in sample_files {
            let mut path = self.current_path.clone();
            path.push(name);

            let mut file = FileInfo::new(path.clone(), name.to_string(), file_type);
            file.set_size(size);
            file.set_modified_time(0);
            file.set_executable(name.ends_with(".sh"));

            self.files.push(file);
        }
    }

    /// Get files in current directory
    pub fn get_files(&self) -> Vec<&FileInfo> {
        if self.config.show_hidden {
            self.files.iter().collect()
        } else {
            self.files.iter()
                .filter(|f| !f.is_hidden)
                .collect()
        }
    }

    /// Get a file by name
    pub fn get_file(&self, name: &str) -> Option<&FileInfo> {
        self.files.iter()
            .find(|f| f.name == name)
    }

    /// Create directory
    pub fn create_directory(&mut self, name: String) -> Result<(), String> {
        if name.is_empty() {
            return Err("Directory name cannot be empty".to_string());
        }

        let mut path = self.current_path.clone();
        path.push(&name);

        let mut file = FileInfo::new(path.clone(), name.clone(), FileType::Directory);
        self.files.push(file);

        Ok(())
    }

    /// Create file
    pub fn create_file(&mut self, name: String) -> Result<(), String> {
        if name.is_empty() {
            return Err("File name cannot be empty".to_string());
        }

        let mut path = self.current_path.clone();
        path.push(&name);

        let mut file = FileInfo::new(path.clone(), name.clone(), FileType::Regular);
        self.files.push(file);

        Ok(())
    }

    /// Delete file or directory
    pub fn delete(&mut self, name: &str) -> Result<(), String> {
        let pos = self.files.iter()
            .position(|f| f.name == name)
            .ok_or_else(|| format!("File {} not found", name))?;

        self.files.remove(pos);
        Ok(())
    }

    /// Rename file
    pub fn rename(&mut self, old_name: &str, new_name: String) -> Result<(), String> {
        let file = self.files.iter_mut()
            .find(|f| f.name == old_name)
            .ok_or_else(|| format!("File {} not found", old_name))?;

        file.name = new_name.clone();
        let mut new_path = self.current_path.clone();
        new_path.push(&new_name);
        file.path = new_path;

        Ok(())
    }

    /// Add bookmark
    pub fn add_bookmark(&mut self, name: String, path: PathBuf) {
        self.bookmarks.insert(name, path);
    }

    /// Get bookmark
    pub fn get_bookmark(&self, name: &str) -> Option<&PathBuf> {
        self.bookmarks.get(name)
    }

    /// List bookmarks
    pub fn list_bookmarks(&self) -> Vec<(&String, &PathBuf)> {
        self.bookmarks.iter().collect()
    }

    /// Remove bookmark
    pub fn remove_bookmark(&mut self, name: &str) {
        self.bookmarks.remove(name);
    }

    /// Get statistics
    pub fn get_statistics(&self) -> DesktopFileExplorerStatistics {
        let total_files = self.files.len();
        let directories = self.files.iter()
            .filter(|f| f.is_directory())
            .count();
        let regular_files = total_files - directories;
        let total_size: u64 = self.files.iter()
            .map(|f| f.size_bytes)
            .sum();

        DesktopFileExplorerStatistics {
            current_path: self.current_path.display().to_string(),
            total_files,
            directories,
            regular_files,
            total_size,
            bookmarks: self.bookmarks.len(),
        }
    }
}

impl Default for DesktopFileExplorer {
    fn default() -> Self {
        Self::new()
    }
}

/// File manager statistics
#[derive(Debug, Clone)]
pub struct DesktopFileExplorerStatistics {
    pub current_path: String,
    pub total_files: usize,
    pub directories: usize,
    pub regular_files: usize,
    pub total_size: u64,
    pub bookmarks: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_type_as_str() {
        assert_eq!(FileType::Directory.as_str(), "Directory");
        assert_eq!(FileType::Regular.as_str(), "Regular");
    }

    #[test]
    fn test_view_mode_from_str() {
        assert_eq!(DesktopViewMode::from_str("icon"), Some(DesktopViewMode::Icon));
        assert_eq!(DesktopViewMode::from_str("list"), Some(DesktopViewMode::List));
    }

    #[test]
    fn test_sort_order_from_str() {
        assert_eq!(DesktopSortOrder::from_str("name"), Some(DesktopSortOrder::Name));
        assert_eq!(DesktopSortOrder::from_str("size"), Some(DesktopSortOrder::Size));
    }

    #[test]
    fn test_file_info_creation() {
        let file = FileInfo::new(
            PathBuf::from("/test"),
            "test.txt".to_string(),
            FileType::Regular,
        );
        assert_eq!(file.name, "test.txt");
    }

    #[test]
    fn test_file_manager_creation() {
        let manager = DesktopFileExplorer::new();
        assert_eq!(manager.get_current_path(), &PathBuf::from("/home/user"));
    }

    #[test]
    fn test_navigate() {
        let mut manager = DesktopFileExplorer::new();
        assert!(manager.navigate(PathBuf::from("/tmp")).is_ok());
        assert_eq!(manager.get_current_path(), &PathBuf::from("/tmp"));
    }

    #[test]
    fn test_navigate_up() {
        let mut manager = DesktopFileExplorer::new();
        manager.navigate(PathBuf::from("/home/user/Documents")).ok();
        assert!(manager.navigate_up().is_ok());
        assert_eq!(manager.get_current_path(), &PathBuf::from("/home/user"));
    }

    #[test]
    fn test_create_directory() {
        let mut manager = DesktopFileExplorer::new();
        assert!(manager.create_directory("test".to_string()).is_ok());
        assert!(manager.get_file("test").is_some());
    }

    #[test]
    fn test_delete() {
        let mut manager = DesktopFileExplorer::new();
        manager.create_directory("test".to_string()).ok();
        assert!(manager.delete("test").is_ok());
        assert!(manager.get_file("test").is_none());
    }

    #[test]
    fn test_bookmarks() {
        let mut manager = DesktopFileExplorer::new();
        manager.add_bookmark("Home".to_string(), PathBuf::from("/home/user"));
        assert!(manager.get_bookmark("Home").is_some());
    }

    #[test]
    fn test_statistics() {
        let manager = DesktopFileExplorer::new();
        let stats = manager.get_statistics();
        assert!(stats.total_files > 0);
    }
}
