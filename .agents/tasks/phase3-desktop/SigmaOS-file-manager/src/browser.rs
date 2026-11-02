//! Directory browser for SigmaOS-file-manager.

use std::path::{Path, PathBuf};
use crate::{Bookmark, DirectoryView, EntryKind, FileEntry, FmError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder { Name, Size, Date, Type }

impl Default for SortOrder { fn default() -> Self { SortOrder::Name } }

/// Stateful file browser with navigation history.
#[derive(Debug, Default)]
pub struct FileBrowser {
    pub current_dir: PathBuf,
    pub view: DirectoryView,
    pub bookmarks: Vec<Bookmark>,
    pub sort: SortOrder,
    pub show_hidden: bool,
    pub selected: Vec<usize>,
}

impl FileBrowser {
    pub fn new() -> Self { Self::default() }

    pub fn navigate(&mut self, path: &Path) -> Result<()> {
        if !path.exists() {
            return Err(FmError::NotFound(path.display().to_string()));
        }
        self.current_dir = path.to_path_buf();
        self.refresh()
    }

    pub fn refresh(&mut self) -> Result<()> {
        let dir = std::fs::read_dir(&self.current_dir)?;
        let mut entries = Vec::new();
        for entry in dir.flatten() {
            let meta = entry.metadata()?;
            let name = entry.file_name().to_string_lossy().into_owned();
            if !self.show_hidden && name.starts_with('.') {
                continue;
            }
            let kind = if meta.is_symlink() {
                EntryKind::Symlink
            } else if meta.is_dir() {
                EntryKind::Directory
            } else {
                EntryKind::File
            };
            entries.push(FileEntry {
                name: name.clone(),
                path: entry.path(),
                size: meta.len(),
                kind,
                mime_type: if meta.is_dir() {
                    "inode/directory".into()
                } else {
                    "application/octet-stream".into()
                },
            });
        }
        // Sort
        match self.sort {
            SortOrder::Name => entries.sort_by(|a, b| a.name.cmp(&b.name)),
            SortOrder::Size => entries.sort_by_key(|e| e.size),
            SortOrder::Date => {}
            SortOrder::Type => entries.sort_by(|a, b| a.mime_type.cmp(&b.mime_type)),
        }
        self.view.load(self.current_dir.to_string_lossy().into_owned(), entries);
        Ok(())
    }

    pub fn go_up(&mut self) -> Result<()> {
        if let Some(parent) = self.current_dir.parent() {
            let parent = parent.to_path_buf();
            self.navigate(&parent)
        } else {
            Ok(())
        }
    }

    pub fn add_bookmark(&mut self, name: impl Into<String>, path: PathBuf) {
        self.bookmarks.push(Bookmark { name: name.into(), path });
    }

    pub fn remove_bookmark(&mut self, path: &Path) {
        self.bookmarks.retain(|b| b.path != path);
    }

    pub fn select(&mut self, index: usize) {
        if !self.selected.contains(&index) {
            self.selected.push(index);
        }
    }

    pub fn deselect_all(&mut self) {
        self.selected.clear();
    }

    pub fn selected_entries(&self) -> Vec<&FileEntry> {
        self.selected
            .iter()
            .filter_map(|&i| self.view.entries().get(i))
            .collect()
    }
}
