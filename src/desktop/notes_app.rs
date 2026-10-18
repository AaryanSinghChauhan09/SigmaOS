// SPDX-License-Identifier: MIT
// SigmaOS - Notes Application
// Inspired by GNOME Notes and Joplin
// Provides note-taking and organization

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

/// Note color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteColor {
    Default,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Brown,
    Gray,
}

/// Note
#[derive(Debug, Clone)]
pub struct Note {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub folder_id: Option<u32>,
    pub color: NoteColor,
    pub created: i64,
    pub modified: i64,
    pub pinned: bool,
    pub tags: Vec<String>,
}

/// Folder
#[derive(Debug, Clone)]
pub struct NoteFolder {
    pub id: u32,
    pub name: String,
    pub parent_id: Option<u32>,
}

/// Zero-allocation case-insensitive substring search helper.
fn contains_ignore_case(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    if needle.is_ascii() && haystack.is_ascii() {
        let h_bytes = haystack.as_bytes();
        let n_bytes = needle.as_bytes();
        if n_bytes.len() > h_bytes.len() {
            return false;
        }
        h_bytes.windows(n_bytes.len()).any(|window| {
            window
                .iter()
                .zip(n_bytes)
                .all(|(&h, &n)| h.to_ascii_lowercase() == n.to_ascii_lowercase())
        })
    } else {
        haystack.to_lowercase().contains(&needle.to_lowercase())
    }
}

/// Notes application
pub struct NotesApp {
    pub notes: BTreeMap<u32, Note>,
    pub folders: BTreeMap<u32, NoteFolder>,
    pub active_note: Option<u32>,
    pub next_note_id: AtomicU32,
    next_folder_id: AtomicU32,
}

impl NotesApp {
    pub fn new() -> Self {
        let mut folders = BTreeMap::new();

        // Default folder
        folders.insert(
            1,
            NoteFolder {
                id: 1,
                name: "All Notes".to_string(),
                parent_id: None,
            },
        );

        Self {
            notes: BTreeMap::new(),
            folders,
            active_note: None,
            next_note_id: AtomicU32::new(1),
            next_folder_id: AtomicU32::new(2),
        }
    }

    /// Add folder
    pub fn add_folder(&mut self, name: String, parent_id: Option<u32>) -> u32 {
        let id = self.next_folder_id.fetch_add(1, Ordering::Relaxed);
        let folder = NoteFolder {
            id,
            name,
            parent_id,
        };
        self.folders.insert(id, folder);
        id
    }

    /// Remove folder
    pub fn remove_folder(&mut self, id: u32) -> Result<(), &'static str> {
        if id == 1 {
            return Err("Cannot remove default folder");
        }
        if !self.folders.contains_key(&id) {
            return Err("Folder not found");
        }
        self.folders.remove(&id);
        Ok(())
    }

    /// Add note
    pub fn add_note(
        &mut self,
        title: String,
        content: String,
        folder_id: Option<u32>,
        color: NoteColor,
    ) -> u32 {
        let id = self.next_note_id.fetch_add(1, Ordering::Relaxed);
        let now = 0; // Simplified
        let note = Note {
            id,
            title,
            content,
            folder_id,
            color,
            created: now,
            modified: now,
            pinned: false,
            tags: Vec::new(),
        };
        self.notes.insert(id, note);
        id
    }

    /// Update note
    pub fn update_note(
        &mut self,
        id: u32,
        title: String,
        content: String,
        color: NoteColor,
    ) -> Result<(), &'static str> {
        let note = self.notes.get_mut(&id).ok_or("Note not found")?;
        note.title = title;
        note.content = content;
        note.color = color;
        note.modified = 0; // Simplified
        Ok(())
    }

    /// Remove note
    pub fn remove_note(&mut self, id: u32) -> Result<(), &'static str> {
        if !self.notes.contains_key(&id) {
            return Err("Note not found");
        }
        if self.active_note == Some(id) {
            self.active_note = None;
        }
        self.notes.remove(&id);
        Ok(())
    }

    /// Get note
    pub fn get_note(&self, id: u32) -> Option<&Note> {
        self.notes.get(&id)
    }

    /// Set active note
    pub fn set_active_note(&mut self, id: u32) -> Result<(), &'static str> {
        if !self.notes.contains_key(&id) {
            return Err("Note not found");
        }
        self.active_note = Some(id);
        Ok(())
    }

    /// Get active note
    pub fn get_active_note(&self) -> Option<&Note> {
        self.active_note.and_then(|id| self.notes.get(&id))
    }

    /// Pin note
    pub fn pin_note(&mut self, id: u32) -> Result<(), &'static str> {
        let note = self.notes.get_mut(&id).ok_or("Note not found")?;
        note.pinned = true;
        Ok(())
    }

    /// Unpin note
    pub fn unpin_note(&mut self, id: u32) -> Result<(), &'static str> {
        let note = self.notes.get_mut(&id).ok_or("Note not found")?;
        note.pinned = false;
        Ok(())
    }

    /// Add tag to note
    pub fn add_tag(&mut self, id: u32, tag: String) -> Result<(), &'static str> {
        let note = self.notes.get_mut(&id).ok_or("Note not found")?;
        if !note.tags.contains(&tag) {
            note.tags.push(tag);
        }
        Ok(())
    }

    /// Remove tag from note
    pub fn remove_tag(&mut self, id: u32, tag: &str) -> Result<(), &'static str> {
        let note = self.notes.get_mut(&id).ok_or("Note not found")?;
        note.tags.retain(|t| t != tag);
        Ok(())
    }

    /// Search notes using case-insensitive substring matching
    pub fn search_notes(&self, query: &str) -> Vec<&Note> {
        let query_lower = query.to_lowercase();
        self.notes
            .values()
            .filter(|n| {
                n.title.to_lowercase().contains(&query_lower)
                    || n.content.to_lowercase().contains(&query_lower)
                    || n.tags
                        .iter()
                        .any(|t| t.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// Get notes in folder
    pub fn get_folder_notes(&self, folder_id: u32) -> Vec<&Note> {
        self.notes
            .values()
            .filter(|n| n.folder_id == Some(folder_id))
            .collect()
    }

    /// Get pinned notes
    pub fn get_pinned_notes(&self) -> Vec<&Note> {
        self.notes.values().filter(|n| n.pinned).collect()
    }

    /// Get all notes
    pub fn get_all_notes(&self) -> Vec<&Note> {
        self.notes.values().collect()
    }

    /// Get all folders
    pub fn get_all_folders(&self) -> Vec<&NoteFolder> {
        self.folders.values().collect()
    }

    /// Note count
    pub fn note_count(&self) -> usize {
        self.notes.len()
    }

    /// Folder count
    pub fn folder_count(&self) -> usize {
        self.folders.len()
    }
}

/// Global notes app
static mut NOTES_APP: Option<NotesApp> = None;

/// Initialize notes app
pub fn init_notes_app() {
    unsafe {
        NOTES_APP = Some(NotesApp::new());
    }
}

/// Get notes app
pub fn get_notes_app() -> &'static mut NotesApp {
    unsafe { NOTES_APP.as_mut().expect("Notes app not initialized") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notes_app_creation() {
        let app = NotesApp::new();
        assert_eq!(app.note_count(), 0);
        assert_eq!(app.folder_count(), 1); // Default folder
    }

    #[test]
    fn test_add_note() {
        let mut app = NotesApp::new();
        let id = app.add_note(
            "Test Note".to_string(),
            "Content".to_string(),
            Some(1),
            NoteColor::Blue,
        );
        assert_eq!(app.note_count(), 1);
    }

    #[test]
    fn test_pin_note() {
        let mut app = NotesApp::new();
        let id = app.add_note(
            "Test".to_string(),
            "Content".to_string(),
            Some(1),
            NoteColor::Default,
        );
        app.pin_note(id).unwrap();
        assert!(app.get_note(id).unwrap().pinned);
    }

    #[test]
    fn test_search_notes() {
        let mut app = NotesApp::new();
        app.add_note(
            "Shopping List".to_string(),
            "Milk, Eggs".to_string(),
            Some(1),
            NoteColor::Default,
        );
        let results = app.search_notes("shopping");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_add_tag() {
        let mut app = NotesApp::new();
        let id = app.add_note(
            "Test".to_string(),
            "Content".to_string(),
            Some(1),
            NoteColor::Default,
        );
        app.add_tag(id, "important".to_string()).unwrap();
        assert!(app
            .get_note(id)
            .unwrap()
            .tags
            .contains(&"important".to_string()));
    }
}
