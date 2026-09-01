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

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

/// OOP-based File Manager for SigmaOS
/// Based on Ideas-999-Structured: User Experience & Desktop Item 766
/// Implements file browser and management

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

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

pub struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    pub fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    pub fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8, self.capacity * mem::size_of::<T>()); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8, self.capacity * mem::size_of::<T>());
            }
        }
    }
}

impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data.add(index) }
    }
}

#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    if size == 0 { return core::ptr::null_mut(); }
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8, size: usize) {
    if !ptr.is_null() && size > 0 {
        use std::alloc::{dealloc, Layout};
        let layout = Layout::from_size_align(size, 8).unwrap();
        dealloc(ptr, layout);
    }
}

#[cfg(target_os = "none")]
extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8, size: usize); }

impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dual_pane_view() {
        let mut dp = DualPaneView::new(b"/home/user", b"/var/log");
        assert_eq!(dp.active_pane, ActivePane::Left);
        dp.switch_active_pane();
        assert_eq!(dp.active_pane, ActivePane::Right);
    }

    #[test]
    fn test_file_tag_store() {
        let mut store = FileTagStore::new();
        store.add_tag(101, b"important");
        assert!(store.has_tag(101, b"important"));
        assert!(!store.has_tag(101, b"work"));
    }

    #[test]
    fn test_miller_columns_navigation() {
        let mut mc = MillerColumnsView::new();
        mc.push_column(0);
        mc.select_item(0, 42);
        assert_eq!(mc.columns[0].selected_id, Some(42));
    }

    #[test]
    fn test_file_snapshot_diff() {
        let diff = FileSnapshotDiff::compare(1, 2, 10, 1024, 2048);
        assert!(diff.is_modified);
    }
}
