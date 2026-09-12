#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use std::boxed::Box;
use std::string::{String, ToString};
use std::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

/// OOP-based File Manager for SigmaOS
/// Based on Ideas-999-Structured: User Experience & Desktop Item 766
/// Implements file browser and management

pub type FileID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType { Directory = 0, File = 1, Symlink = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum FileManagerError { Success = 0, NotFound = 1, PermissionDenied = 2 }

pub trait FileEntry {
    fn id(&self) -> FileID;
    fn name(&self) -> &[u8];
    fn file_type(&self) -> FileType;
    fn size(&self) -> u64;
    fn is_hidden(&self) -> bool;
}

#[repr(C)]
pub struct SimpleFileEntry {
    pub id: FileID,
    pub name: [u8; 256],
    pub file_type: AtomicUsize,
    pub size: AtomicUsize,
    pub hidden: AtomicUsize,
}

impl SimpleFileEntry {
    pub fn new(id: FileID, name: &[u8], file_type: FileType, size: u64) -> Self {
        let mut name_array = [0u8; 256];
        let name_len = name.len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleFileEntry {
            id,
            name: name_array,
            file_type: AtomicUsize::new(file_type as usize),
            size: AtomicUsize::new(size as usize),
            hidden: AtomicUsize::new(0),
        }
    }
}

impl FileEntry for SimpleFileEntry {
    fn id(&self) -> FileID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(256);
        &self.name[..len]
    }
    fn file_type(&self) -> FileType {
        match self.file_type.load(Ordering::SeqCst) {
            0 => FileType::Directory,
            1 => FileType::File,
            _ => FileType::Symlink,
        }
    }
    fn size(&self) -> u64 { self.size.load(Ordering::SeqCst) as u64 }
    fn is_hidden(&self) -> bool { self.hidden.load(Ordering::SeqCst) == 1 }
}

pub trait FileManager {
    fn list_directory(&self, path: &[u8]) -> Result<Vec<&dyn FileEntry>, FileManagerError>;
    fn create_directory(&mut self, path: &[u8], name: &[u8]) -> Result<FileID, FileManagerError>;
    fn delete_file(&mut self, id: FileID) -> Result<(), FileManagerError>;
    fn copy_file(&mut self, source_id: FileID, dest_path: &[u8]) -> Result<(), FileManagerError>;
}

#[repr(C)]
pub struct SimpleFileManager {
    pub files: Vec<Option<Box<dyn FileEntry>>>,
    pub next_id: AtomicUsize,
}

impl SimpleFileManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleFileManager {
            files: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }

    pub fn get_file(&self, id: FileID) -> Option<&dyn FileEntry> {
        for file_option in &self.files {
            if let Some(ref file) = *file_option {
                if file.id() == id {
                    return Some(file.as_ref());
                }
            }
        }
        None
    }
}

impl FileManager for SimpleFileManager {
    fn list_directory(&self, _path: &[u8]) -> Result<Vec<&dyn FileEntry>, FileManagerError> {
        let mut entries = Vec::new();
        for file_option in &self.files {
            if let Some(ref file) = *file_option {
                entries.push(file.as_ref());
            }
        }
        Ok(entries)
    }

    fn create_directory(&mut self, _path: &[u8], name: &[u8]) -> Result<FileID, FileManagerError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let entry = SimpleFileEntry::new(id, name, FileType::Directory, 4096);
        self.files.push(Some(Box::new(entry)));
        Ok(id)
    }

    fn delete_file(&mut self, id: FileID) -> Result<(), FileManagerError> {
        for file_option in &mut self.files {
            if let Some(ref file) = *file_option {
                if file.id() == id {
                    return Ok(());
                }
            }
        }
        Err(FileManagerError::NotFound)
    }

    fn copy_file(&mut self, source_id: FileID, _dest_path: &[u8]) -> Result<(), FileManagerError> {
        if self.get_file(source_id).is_some() {
            Ok(())
        } else {
            Err(FileManagerError::NotFound)
        }
    }
}

/// Midnight Commander / Ranger inspired Dual-Pane Split Navigation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePane {
    Left,
    Right,
}

pub struct DualPaneView {
    pub left_path: [u8; 256],
    pub right_path: [u8; 256],
    pub active_pane: ActivePane,
}

impl DualPaneView {
    pub fn new(left_path: &[u8], right_path: &[u8]) -> Self {
        let mut l = [0u8; 256];
        let mut r = [0u8; 256];
        let llen = left_path.len().min(255);
        let rlen = right_path.len().min(255);
        l[..llen].copy_from_slice(&left_path[..llen]);
        r[..rlen].copy_from_slice(&right_path[..rlen]);

        DualPaneView {
            left_path: l,
            right_path: r,
            active_pane: ActivePane::Left,
        }
    }

    pub fn switch_active_pane(&mut self) {
        self.active_pane = match self.active_pane {
            ActivePane::Left => ActivePane::Right,
            ActivePane::Right => ActivePane::Left,
        };
    }

    pub fn active_path(&self) -> &[u8] {
        match self.active_pane {
            ActivePane::Left => {
                let len = self.left_path.iter().position(|&b| b == 0).unwrap_or(256);
                &self.left_path[..len]
            }
            ActivePane::Right => {
                let len = self.right_path.iter().position(|&b| b == 0).unwrap_or(256);
                &self.right_path[..len]
            }
        }
    }
}

/// KDE Dolphin / GNOME Nautilus File Tagging Entry
pub struct FileTagEntry {
    pub file_id: FileID,
    pub tag_name: [u8; 32],
}

pub struct FileTagStore {
    pub tags: Vec<FileTagEntry>,
}

impl FileTagStore {
    pub fn new() -> Self {
        FileTagStore { tags: Vec::new() }
    }

    pub fn add_tag(&mut self, file_id: FileID, tag: &[u8]) {
        let mut tag_arr = [0u8; 32];
        let len = tag.len().min(31);
        tag_arr[..len].copy_from_slice(&tag[..len]);
        self.tags.push(FileTagEntry {
            file_id,
            tag_name: tag_arr,
        });
    }

    pub fn has_tag(&self, file_id: FileID, tag: &[u8]) -> bool {
        let len = tag.len().min(31);
        for entry in self.tags.iter() {
            if entry.file_id == file_id {
                let mut matches = true;
                for i in 0..len {
                    if entry.tag_name[i] != tag[i] {
                        matches = false;
                        break;
                    }
                }
                if matches && (entry.tag_name[len] == 0 || len == 31) {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_files_with_tag(&self, tag: &[u8]) -> Vec<FileID> {
        let mut results = Vec::new();
        for entry in self.tags.iter() {
            let tag_len = entry.tag_name.iter().position(|&b| b == 0).unwrap_or(32);
            if &entry.tag_name[..tag_len] == tag {
                results.push(entry.file_id);
            }
        }
        results
    }
}

/// macOS Finder / Pantheon Miller Columns View Column
pub struct MillerColumn {
    pub depth: usize,
    pub parent_id: FileID,
    pub selected_id: Option<FileID>,
}

pub struct MillerColumnsView {
    pub columns: Vec<MillerColumn>,
}

impl MillerColumnsView {
    pub fn new() -> Self {
        MillerColumnsView {
            columns: Vec::new(),
        }
    }

    pub fn push_column(&mut self, parent_id: FileID) {
        let depth = self.columns.len();
        self.columns.push(MillerColumn {
            depth,
            parent_id,
            selected_id: None,
        });
    }

    pub fn select_item(&mut self, depth: usize, file_id: FileID) {
        if depth < self.columns.len() {
            self.columns[depth].selected_id = Some(file_id);
        }
    }

    pub fn active_selection(&self) -> Option<FileID> {
        self.columns.last().and_then(|col| col.selected_id)
    }
}

/// Yazi / Ranger Inspired Directory Entry Cache for Ultra-Fast Preloading
pub struct DirectoryCache {
    pub cached_path: [u8; 256],
    pub file_ids: Vec<FileID>,
    pub valid: bool,
}

impl DirectoryCache {
    pub fn new(path: &[u8], file_ids: Vec<FileID>) -> Self {
        let mut cached_path = [0u8; 256];
        let len = path.len().min(255);
        cached_path[..len].copy_from_slice(&path[..len]);
        DirectoryCache {
            cached_path,
            file_ids,
            valid: true,
        }
    }

    pub fn matches(&self, path: &[u8]) -> bool {
        if !self.valid {
            return false;
        }
        let len = self.cached_path.iter().position(|&b| b == 0).unwrap_or(256);
        &self.cached_path[..len] == path
    }

    pub fn invalidate(&mut self) {
        self.valid = false;
    }
}

/// KDE Dolphin / GNOME Nautilus Inspired File Metadata Classifier
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MimeType {
    Text,
    Image,
    Audio,
    Video,
    Archive,
    Executable,
    Unknown,
}

pub struct FileMetadata {
    pub name: String,
    pub mime_type: MimeType,
    pub size: u64,
    pub is_hidden: bool,
    pub is_readonly: bool,
}

impl FileMetadata {
    pub fn extract(name: &str, size: u64, is_readonly: bool) -> Self {
        let is_hidden = name.starts_with('.');
        let mime_type = if name.ends_with(".txt") || name.ends_with(".md") || name.ends_with(".rs") {
            MimeType::Text
        } else if name.ends_with(".png") || name.ends_with(".jpg") || name.ends_with(".svg") {
            MimeType::Image
        } else if name.ends_with(".mp3") || name.ends_with(".wav") || name.ends_with(".flac") {
            MimeType::Audio
        } else if name.ends_with(".mp4") || name.ends_with(".mkv") || name.ends_with(".webm") {
            MimeType::Video
        } else if name.ends_with(".tar") || name.ends_with(".gz") || name.ends_with(".zip") || name.ends_with(".sigpkg") {
            MimeType::Archive
        } else if name.ends_with(".sh") || name.ends_with(".bin") || !name.contains('.') {
            MimeType::Executable
        } else {
            MimeType::Unknown
        };

        FileMetadata {
            name: name.to_string(),
            mime_type,
            size,
            is_hidden,
            is_readonly,
        }
    }
}

/// OpenBSD Pledge/Unveil Inspired Sandbox Path Guard
pub struct PathSandboxGuard {
    pub allowed_root: String,
}

impl PathSandboxGuard {
    pub fn new(allowed_root: &str) -> Self {
        PathSandboxGuard {
            allowed_root: allowed_root.to_string(),
        }
    }

    pub fn is_path_safe(&self, target_path: &str) -> bool {
        if target_path.contains("..") || target_path.contains('\0') {
            return false;
        }
        target_path.starts_with(&self.allowed_root)
    }
}

/// openSUSE Snapper CoW File Snapshot Comparison
pub struct FileSnapshotDiff {
    pub snapshot_a_id: u64,
    pub snapshot_b_id: u64,
    pub file_id: FileID,
    pub is_modified: bool,
}

impl FileSnapshotDiff {
    pub fn compare(snapshot_a_id: u64, snapshot_b_id: u64, file_id: FileID, size_a: u64, size_b: u64) -> Self {
        FileSnapshotDiff {
            snapshot_a_id,
            snapshot_b_id,
            file_id,
            is_modified: size_a != size_b,
        }
    }
}

pub trait FileSearch {
    fn search(&self, query: &[u8]) -> Vec<FileID>;
    fn filter_by_type(&self, file_type: FileType) -> Vec<FileID>;
}

#[repr(C)]
pub struct SimpleFileSearch {
    pub manager: SimpleFileManager,
}

impl SimpleFileSearch {
    pub fn new(manager: SimpleFileManager) -> Self {
        SimpleFileSearch { manager }
    }
}

impl FileSearch for SimpleFileSearch {
    fn search(&self, query: &[u8]) -> Vec<FileID> {
        let mut results = Vec::new();
        for file_option in &self.manager.files {
            if let Some(ref file) = *file_option {
                let name = file.name();
                if query.len() <= name.len() {
                    for i in 0..=(name.len() - query.len()) {
                        if &name[i..i + query.len()] == query {
                            results.push(file.id());
                            break;
                        }
                    }
                }
            }
        }
        results
    }

    fn filter_by_type(&self, file_type: FileType) -> Vec<FileID> {
        let mut results = Vec::new();
        for file_option in &self.manager.files {
            if let Some(ref file) = *file_option {
                if file.file_type() == file_type {
                    results.push(file.id());
                }
            }
        }
        results
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dual_pane_view() {
        let mut dp = DualPaneView::new(b"/home/user", b"/var/log");
        assert_eq!(dp.active_pane, ActivePane::Left);
        assert_eq!(dp.active_path(), b"/home/user");
        dp.switch_active_pane();
        assert_eq!(dp.active_pane, ActivePane::Right);
        assert_eq!(dp.active_path(), b"/var/log");
    }

    #[test]
    fn test_file_tag_store_filtering() {
        let mut store = FileTagStore::new();
        store.add_tag(101, b"important");
        store.add_tag(102, b"important");
        store.add_tag(103, b"work");
        assert!(store.has_tag(101, b"important"));
        assert!(!store.has_tag(101, b"work"));

        let tagged = store.get_files_with_tag(b"important");
        assert_eq!(tagged.len(), 2);
        assert!(tagged.contains(&101));
        assert!(tagged.contains(&102));
    }

    #[test]
    fn test_miller_columns_navigation() {
        let mut mc = MillerColumnsView::new();
        mc.push_column(0);
        mc.select_item(0, 42);
        assert_eq!(mc.columns[0].selected_id, Some(42));
        assert_eq!(mc.active_selection(), Some(42));
    }

    #[test]
    fn test_file_snapshot_diff() {
        let diff = FileSnapshotDiff::compare(1, 2, 10, 1024, 2048);
        assert!(diff.is_modified);
    }

    #[test]
    fn test_directory_cache() {
        let mut cache = DirectoryCache::new(b"/home/user", vec![1, 2, 3]);
        assert!(cache.matches(b"/home/user"));
        assert!(!cache.matches(b"/var/log"));
        assert_eq!(cache.file_ids.len(), 3);
        cache.invalidate();
        assert!(!cache.matches(b"/home/user"));
    }

    #[test]
    fn test_file_metadata_extraction() {
        let meta_text = FileMetadata::extract("notes.md", 1024, false);
        assert_eq!(meta_text.mime_type, MimeType::Text);
        assert!(!meta_text.is_hidden);

        let meta_img = FileMetadata::extract(".photo.png", 2048, true);
        assert_eq!(meta_img.mime_type, MimeType::Image);
        assert!(meta_img.is_hidden);
        assert!(meta_img.is_readonly);

        let meta_exec = FileMetadata::extract("script.sh", 512, false);
        assert_eq!(meta_exec.mime_type, MimeType::Executable);
    }

    #[test]
    fn test_path_sandbox_guard() {
        let guard = PathSandboxGuard::new("/home/user");
        assert!(guard.is_path_safe("/home/user/documents/file.txt"));
        assert!(!guard.is_path_safe("/home/user/../etc/passwd"));
        assert!(!guard.is_path_safe("/etc/passwd"));
    }
}
