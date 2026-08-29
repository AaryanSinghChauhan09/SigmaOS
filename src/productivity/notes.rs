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
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;

// SigmaOS Note-taking App
// OOP-based note management with rich text and organization

use crate::klib::BTreeMap;
// PathBuf not in no_std

/// Note
#[derive(Debug, Clone)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub content_type: ContentType,
    pub created_at: u64,
    pub modified_at: u64,
    pub tags: Vec<String>,
    pub folder_id: Option<String>,
    pub is_pinned: bool,
    pub is_archived: bool,
}

/// Content type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    PlainText,
    Markdown,
    RichText,
    Code,
}

/// Folder
#[derive(Debug, Clone)]
pub struct Folder {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub note_ids: Vec<String>,
    pub created_at: u64,
}

/// Notebook
#[derive(Debug, Clone)]
pub struct Notebook {
    pub id: String,
    pub name: String,
    pub description: String,
    pub color: String,
    pub folder_ids: Vec<String>,
}

/// Note search result
#[derive(Debug, Clone)]
pub struct NoteSearchResult {
    pub note_id: String,
    pub title: String,
    pub snippet: String,
    pub relevance_score: f64,
}

/// OOP trait for note storage strategies
pub trait NoteStorage {
    /// Save note
    fn save_note(&mut self, note: &Note) -> Result<(), NoteError>;
    /// Load note
    fn load_note(&self, note_id: &str) -> Result<Note, NoteError>;
    /// Delete note
    fn delete_note(&mut self, note_id: &str) -> Result<(), NoteError>;
    /// List all notes
    fn list_notes(&self) -> Result<Vec<Note>, NoteError>;
    /// Get storage name
    fn name(&self) -> &str;
}

/// In-memory note storage
pub struct InMemoryNoteStorage {
    notes: BTreeMap<String, Note>,
}

impl InMemoryNoteStorage {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            notes: BTreeMap::new(),
        }
    }
}

impl NoteStorage for InMemoryNoteStorage {
    fn save_note(&mut self, note: &Note) -> Result<(), NoteError> {
        self.notes.insert(note.id.clone(), note.clone());
        Ok(())
    }

    fn load_note(&self, note_id: &str) -> Result<Note, NoteError> {
        self.notes
            .get(note_id)
            .cloned()
            .ok_or_else(|| NoteError::NoteNotFound(note_id.to_string()))
    }

    fn delete_note(&mut self, note_id: &str) -> Result<(), NoteError> {
        self.notes
            .remove(note_id)
            .ok_or_else(|| NoteError::NoteNotFound(note_id.to_string()))?;
        Ok(())
    }

    fn list_notes(&self) -> Result<Vec<Note>, NoteError> {
        Ok(self.notes.values().cloned().collect())
    }

    fn name(&self) -> &str {
        "InMemoryNoteStorage"
    }
}

/// OOP-based Note-taking App
pub struct NoteTakingApp {
    storage: Box<dyn NoteStorage>,
    folders: BTreeMap<String, Folder>,
    notebooks: BTreeMap<String, Notebook>,
    active_note: Option<String>,
    search_index: BTreeMap<String, Vec<String>>,
}

impl NoteTakingApp {
    pub fn new(storage: Box<dyn NoteStorage>) -> Self {
        Self {
            storage,
            folders: BTreeMap::new(),
            notebooks: BTreeMap::new(),
            active_note: None,
            search_index: BTreeMap::new(),
        }
    }

    /// Create note
    pub fn create_note(
        &mut self,
        title: String,
        content: String,
        content_type: ContentType,
    ) -> Result<String, NoteError> {
        let note_id = format!(
            "note_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );

        let now = 1700000000u64;

        let note = Note {
            id: note_id.clone(),
            title,
            content,
            content_type,
            created_at: now,
            modified_at: now,
            tags: Vec::new(),
            folder_id: None,
            is_pinned: false,
            is_archived: false,
        };

        self.storage.save_note(&note)?;
        self.update_search_index(&note);
        Ok(note_id)
    }

    /// Update note
    pub fn update_note(&mut self, note: Note) -> Result<(), NoteError> {
        let mut updated_note = note;
        updated_note.modified_at = 1700000000u64;

        self.storage.save_note(&updated_note)?;
        self.update_search_index(&updated_note);
        Ok(())
    }

    /// Get note
    pub fn get_note(&self, note_id: &str) -> Result<Note, NoteError> {
        self.storage.load_note(note_id)
    }

    /// Delete note
    pub fn delete_note(&mut self, note_id: &str) -> Result<(), NoteError> {
        self.storage.delete_note(note_id)?;
        self.search_index.remove(note_id);
        Ok(())
    }

    /// List all notes
    pub fn list_notes(&self) -> Result<Vec<Note>, NoteError> {
        self.storage.list_notes()
    }

    /// Search notes
    pub fn search_notes(&self, query: &str) -> Vec<NoteSearchResult> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();

        if let Ok(notes) = self.storage.list_notes() {
            for note in notes {
                let title_match = note.title.to_lowercase().contains(&query_lower);
                let content_match = note.content.to_lowercase().contains(&query_lower);
                let tag_match = note
                    .tags
                    .iter()
                    .any(|t| t.to_lowercase().contains(&query_lower));

                if title_match || content_match || tag_match {
                    let relevance_score = if title_match { 1.0 } else { 0.5 };

                    // Create snippet
                    let snippet = if content_match {
                        let pos = note.content.to_lowercase().find(&query_lower).unwrap_or(0);
                        let start = if pos > 20 { pos - 20 } else { 0 };
                        let end = core::cmp::min(pos + 40, note.content.len());
                        format!("...{}...", &note.content[start..end])
                    } else {
                        note.content.chars().take(50).collect()
                    };

                    results.push(NoteSearchResult {
                        note_id: note.id.clone(),
                        title: note.title.clone(),
                        snippet,
                        relevance_score,
                    });
                }
            }
        }

        results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
        results
    }

    /// Update search index
    fn update_search_index(&mut self, note: &Note) {
        let mut words = Vec::new();

        // Add title words
        for word in note.title.split_whitespace() {
            words.push(word.to_lowercase());
        }

        // Add content words
        for word in note.content.split_whitespace() {
            words.push(word.to_lowercase());
        }

        // Add tags
        for tag in &note.tags {
            words.push(tag.to_lowercase());
        }

        self.search_index.insert(note.id.clone(), words);
    }

    /// Create folder
    pub fn create_folder(&mut self, name: String, parent_id: Option<String>) -> String {
        let folder_id = format!("folder_{}", self.folders.len());
        let folder = Folder {
            id: folder_id.clone(),
            name,
            parent_id,
            note_ids: Vec::new(),
            created_at: 1700000000u64,
        };
        self.folders.insert(folder_id.clone(), folder);
        folder_id
    }

    /// Add note to folder
    pub fn add_note_to_folder(&mut self, note_id: &str, folder_id: &str) -> Result<(), NoteError> {
        if let Some(folder) = self.folders.get_mut(folder_id) {
            folder.note_ids.push(note_id.to_string());
            if let Ok(mut note) = self.storage.load_note(note_id) {
                note.folder_id = Some(folder_id.to_string());
                self.storage.save_note(&note)?;
            }
            Ok(())
        } else {
            Err(NoteError::FolderNotFound(folder_id.to_string()))
        }
    }

    /// Create notebook
    pub fn create_notebook(&mut self, name: String, description: String, color: String) -> String {
        let notebook_id = format!("notebook_{}", self.notebooks.len());
        let notebook = Notebook {
            id: notebook_id.clone(),
            name,
            description,
            color,
            folder_ids: Vec::new(),
        };
        self.notebooks.insert(notebook_id.clone(), notebook);
        notebook_id
    }

    /// Add folder to notebook
    pub fn add_folder_to_notebook(
        &mut self,
        notebook_id: &str,
        folder_id: &str,
    ) -> Result<(), NoteError> {
        if let Some(notebook) = self.notebooks.get_mut(notebook_id) {
            notebook.folder_ids.push(folder_id.to_string());
            Ok(())
        } else {
            Err(NoteError::NotebookNotFound(notebook_id.to_string()))
        }
    }

    /// Pin note
    pub fn pin_note(&mut self, note_id: &str) -> Result<(), NoteError> {
        let mut note = self.storage.load_note(note_id)?;
        note.is_pinned = true;
        self.storage.save_note(&note)
    }

    /// Unpin note
    pub fn unpin_note(&mut self, note_id: &str) -> Result<(), NoteError> {
        let mut note = self.storage.load_note(note_id)?;
        note.is_pinned = false;
        self.storage.save_note(&note)
    }

    /// Archive note
    pub fn archive_note(&mut self, note_id: &str) -> Result<(), NoteError> {
        let mut note = self.storage.load_note(note_id)?;
        note.is_archived = true;
        self.storage.save_note(&note)
    }

    /// Unarchive note
    pub fn unarchive_note(&mut self, note_id: &str) -> Result<(), NoteError> {
        let mut note = self.storage.load_note(note_id)?;
        note.is_archived = false;
        self.storage.save_note(&note)
    }

    /// Add tag to note
    pub fn add_tag(&mut self, note_id: &str, tag: String) -> Result<(), NoteError> {
        let mut note = self.storage.load_note(note_id)?;
        if !note.tags.contains(&tag) {
            note.tags.push(tag);
            self.storage.save_note(&note)?;
            self.update_search_index(&note);
        }
        Ok(())
    }

    /// Remove tag from note
    pub fn remove_tag(&mut self, note_id: &str, tag: &str) -> Result<(), NoteError> {
        let mut note = self.storage.load_note(note_id)?;
        note.tags.retain(|t| t != tag);
        self.storage.save_note(&note)?;
        self.update_search_index(&note);
        Ok(())
    }

    /// Get notes by tag
    pub fn get_notes_by_tag(&self, tag: &str) -> Result<Vec<Note>, NoteError> {
        let notes = self.storage.list_notes()?;
        Ok(notes
            .into_iter()
            .filter(|n| n.tags.contains(&tag.to_string()))
            .collect())
    }

    /// Get pinned notes
    pub fn get_pinned_notes(&self) -> Result<Vec<Note>, NoteError> {
        let notes = self.storage.list_notes()?;
        Ok(notes.into_iter().filter(|n| n.is_pinned).collect())
    }

    /// Get recent notes
    pub fn get_recent_notes(&self, count: usize) -> Result<Vec<Note>, NoteError> {
        let mut notes = self.storage.list_notes()?;
        notes.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
        Ok(notes.into_iter().take(count).collect())
    }

    /// Set active note
    pub fn set_active_note(&mut self, note_id: &str) {
        self.active_note = Some(note_id.to_string());
    }

    /// Get active note
    pub fn active_note(&self) -> Option<Note> {
        self.active_note
            .as_ref()
            .and_then(|id| self.storage.load_note(id).ok())
    }

    /// Get folders
    pub fn folders(&self) -> Vec<&Folder> {
        self.folders.values().collect()
    }

    /// Get notebooks
    pub fn notebooks(&self) -> Vec<&Notebook> {
        self.notebooks.values().collect()
    }

    /// Export note to markdown
    pub fn export_to_markdown(&self, note_id: &str) -> Result<String, NoteError> {
        let note = self.storage.load_note(note_id)?;
        let markdown = format!("# {}\n\n{}", note.title, note.content);
        Ok(markdown)
    }

    /// Import note from markdown
    pub fn import_from_markdown(&mut self, markdown: &str) -> Result<String, NoteError> {
        let lines: Vec<&str> = markdown.lines().collect();
        let title = lines
            .first()
            .map(|l| l.trim_start_matches('#').trim())
            .unwrap_or("Untitled")
            .to_string();

        let content = lines[1..].join("\n");
        self.create_note(title, content, ContentType::Markdown)
    }
}

impl Default for NoteTakingApp {
    fn default() -> Self {
        Self::new(Box::new(InMemoryNoteStorage::new()))
    }
}

/// Note errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoteError {
    NoteNotFound(String),
    FolderNotFound(String),
    NotebookNotFound(String),
    StorageError(String),
    InvalidFormat(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note() {
        let note = Note {
            id: "test".to_string(),
            title: "Test Note".to_string(),
            content: "Test content".to_string(),
            content_type: ContentType::PlainText,
            created_at: 1234567890,
            modified_at: 1234567890,
            tags: Vec::new(),
            folder_id: None,
            is_pinned: false,
            is_archived: false,
        };
        assert_eq!(note.title, "Test Note");
    }

    #[test]
    fn test_in_memory_note_storage() {
        let storage = InMemoryNoteStorage::new();
        assert_eq!(storage.name(), "InMemoryNoteStorage");
    }

    #[test]
    fn test_note_taking_app() {
        let app = NoteTakingApp::default();
        assert!(app.folders.is_empty());
    }

    #[test]
    fn test_create_note() {
        let mut app = NoteTakingApp::default();
        let note_id = app
            .create_note(
                "Test".to_string(),
                "Content".to_string(),
                ContentType::PlainText,
            )
            .unwrap();
        assert!(!note_id.is_empty());
    }

    #[test]
    fn test_search_notes() {
        let mut app = NoteTakingApp::default();
        app.create_note(
            "Test Note".to_string(),
            "This is a test".to_string(),
            ContentType::PlainText,
        )
        .unwrap();
        let results = app.search_notes("test");
        assert!(!results.is_empty());
    }
}
