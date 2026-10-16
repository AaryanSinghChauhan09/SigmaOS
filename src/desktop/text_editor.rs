// SPDX-License-Identifier: MIT
// SigmaOS - Text Editor
// Inspired by VS Code, GNOME Builder, and Kate
// Provides text and code editing capabilities

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

/// Language for syntax highlighting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Plain,
    Rust,
    Python,
    JavaScript,
    TypeScript,
    HTML,
    CSS,
    JSON,
    Markdown,
    C,
    Cpp,
    Go,
    Java,
    Shell,
    SQL,
    XML,
    YAML,
}

/// Theme
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
    HighContrast,
    SolarizedLight,
    SolarizedDark,
    Dracula,
    Nord,
}

/// Tab size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabSize {
    Two,
    Four,
    Eight,
}

/// Line ending
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    LF,
    CRLF,
    CR,
}

/// Encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    UTF8,
    UTF16LE,
    UTF16BE,
    ASCII,
    Latin1,
}

/// Document
#[derive(Debug, Clone)]
pub struct Document {
    pub id: u32,
    pub path: String,
    pub name: String,
    pub content: String,
    pub language: Language,
    pub modified: bool,
    pub read_only: bool,
    pub encoding: Encoding,
    pub line_ending: LineEnding,
}

/// Cursor position
#[derive(Debug, Clone, Copy)]
pub struct CursorPosition {
    pub line: usize,
    pub column: usize,
}

/// Selection
#[derive(Debug, Clone)]
pub struct Selection {
    pub start: CursorPosition,
    pub end: CursorPosition,
}

/// Editor state
#[derive(Debug, Clone)]
pub struct EditorState {
    pub cursor: CursorPosition,
    pub selection: Option<Selection>,
    pub scroll_line: usize,
    pub scroll_column: usize,
}

/// Text editor
pub struct TextEditor {
    pub documents: Vec<Document>,
    pub active_document: Option<usize>,
    pub editor_state: EditorState,
    pub theme: Theme,
    pub font_size: u32,
    pub tab_size: TabSize,
    pub insert_spaces: bool,
    pub show_line_numbers: bool,
    pub word_wrap: bool,
    pub auto_indent: bool,
    pub show_whitespace: bool,
    pub highlight_current_line: bool,
    pub bracket_matching: bool,
    next_document_id: AtomicU32,
}

impl TextEditor {
    pub fn new() -> Self {
        Self {
            documents: Vec::new(),
            active_document: None,
            editor_state: EditorState {
                cursor: CursorPosition { line: 0, column: 0 },
                selection: None,
                scroll_line: 0,
                scroll_column: 0,
            },
            theme: Theme::Dark,
            font_size: 14,
            tab_size: TabSize::Four,
            insert_spaces: true,
            show_line_numbers: true,
            word_wrap: false,
            auto_indent: true,
            show_whitespace: false,
            highlight_current_line: true,
            bracket_matching: true,
            next_document_id: AtomicU32::new(1),
        }
    }

    /// Create new document
    pub fn new_document(&mut self, name: String, language: Language) -> usize {
        let id = self.next_document_id.fetch_add(1, Ordering::Relaxed);
        let document = Document {
            id,
            path: String::new(),
            name,
            content: String::new(),
            language,
            modified: false,
            read_only: false,
            encoding: Encoding::UTF8,
            line_ending: LineEnding::LF,
        };
        self.documents.push(document);
        let index = self.documents.len() - 1;
        self.active_document = Some(index);
        index
    }

    /// Open document
    pub fn open_document(&mut self, path: String, content: String, language: Language) -> usize {
        let id = self.next_document_id.fetch_add(1, Ordering::Relaxed);
        let name = path.split('/').last().unwrap_or("untitled").to_string();
        let document = Document {
            id,
            path: path.clone(),
            name,
            content,
            language,
            modified: false,
            read_only: false,
            encoding: Encoding::UTF8,
            line_ending: LineEnding::LF,
        };
        self.documents.push(document);
        let index = self.documents.len() - 1;
        self.active_document = Some(index);
        index
    }

    /// Close document
    pub fn close_document(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.documents.len() {
            return Err("Document index out of bounds");
        }
        if self.documents[index].modified {
            return Err("Document has unsaved changes");
        }
        self.documents.remove(index);
        if self.active_document == Some(index) {
            self.active_document = if self.documents.is_empty() {
                None
            } else {
                Some(index.saturating_sub(1))
            };
        } else if self.active_document > Some(index) {
            self.active_document = self.active_document.map(|a| a - 1);
        }
        Ok(())
    }

    /// Save document
    pub fn save_document(&mut self, index: usize) -> Result<(), &'static str> {
        let document = self.documents.get_mut(index).ok_or("Document not found")?;
        document.modified = false;
        // In real implementation would write to file
        Ok(())
    }

    /// Save document as
    pub fn save_document_as(&mut self, index: usize, path: String) -> Result<(), &'static str> {
        let document = self.documents.get_mut(index).ok_or("Document not found")?;
        document.path = path.clone();
        document.name = path.split('/').last().unwrap_or("untitled").to_string();
        document.modified = false;
        // In real implementation would write to file
        Ok(())
    }

    /// Get document content
    pub fn get_content(&self, index: usize) -> Result<&str, &'static str> {
        let document = self.documents.get(index).ok_or("Document not found")?;
        Ok(&document.content)
    }

    /// Set document content
    pub fn set_content(&mut self, index: usize, content: String) -> Result<(), &'static str> {
        let document = self.documents.get_mut(index).ok_or("Document not found")?;
        document.content = content;
        document.modified = true;
        Ok(())
    }

    /// Insert text at cursor
    pub fn insert_text(&mut self, index: usize, text: String) -> Result<(), &'static str> {
        let document = self.documents.get_mut(index).ok_or("Document not found")?;
        let line_start = document
            .content
            .lines()
            .take(self.editor_state.cursor.line)
            .fold(0, |acc, line| acc + line.len() + 1);
        let column = self.editor_state.cursor.column;
        document.content.insert_str(line_start + column, &text);
        document.modified = true;
        self.editor_state.cursor.column += text.len();
        Ok(())
    }

    /// Delete text at cursor
    pub fn delete_text(&mut self, index: usize, length: usize) -> Result<(), &'static str> {
        let document = self.documents.get_mut(index).ok_or("Document not found")?;
        let line_start = document
            .content
            .lines()
            .take(self.editor_state.cursor.line)
            .fold(0, |acc, line| acc + line.len() + 1);
        let column = self.editor_state.cursor.column;
        let start = line_start + column;
        let end = start + length;
        if end <= document.content.len() {
            document.content.replace_range(start..end, "");
            document.modified = true;
        }
        Ok(())
    }

    /// Move cursor
    pub fn move_cursor(&mut self, line: usize, column: usize) {
        self.editor_state.cursor.line = line;
        self.editor_state.cursor.column = column;
    }

    /// Set selection
    pub fn set_selection(&mut self, start: CursorPosition, end: CursorPosition) {
        self.editor_state.selection = Some(Selection { start, end });
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.editor_state.selection = None;
    }

    /// Get selected text
    pub fn get_selected_text(&self, index: usize) -> Result<String, &'static str> {
        let document = self.documents.get(index).ok_or("Document not found")?;
        if let Some(ref selection) = self.editor_state.selection {
            // Simplified - in real implementation would extract selected range
            Ok(String::new())
        } else {
            Ok(String::new())
        }
    }

    /// Set active document
    pub fn set_active_document(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.documents.len() {
            return Err("Document index out of bounds");
        }
        self.active_document = Some(index);
        Ok(())
    }

    /// Get active document
    pub fn get_active_document(&self) -> Option<&Document> {
        self.active_document
            .and_then(|index| self.documents.get(index))
    }

    /// Get line count
    pub fn get_line_count(&self, index: usize) -> Result<usize, &'static str> {
        let document = self.documents.get(index).ok_or("Document not found")?;
        Ok(document.content.lines().count())
    }

    /// Set theme
    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    /// Set font size
    pub fn set_font_size(&mut self, size: u32) {
        self.font_size = size;
    }

    /// Set tab size
    pub fn set_tab_size(&mut self, size: TabSize) {
        self.tab_size = size;
    }

    /// Set language
    pub fn set_language(&mut self, index: usize, language: Language) -> Result<(), &'static str> {
        let document = self.documents.get_mut(index).ok_or("Document not found")?;
        document.language = language;
        Ok(())
    }

    /// Toggle word wrap
    pub fn toggle_word_wrap(&mut self) {
        self.word_wrap = !self.word_wrap;
    }

    /// Toggle line numbers
    pub fn toggle_line_numbers(&mut self) {
        self.show_line_numbers = !self.show_line_numbers;
    }

    /// Document count
    pub fn document_count(&self) -> usize {
        self.documents.len()
    }

    /// Has unsaved changes
    pub fn has_unsaved_changes(&self) -> bool {
        self.documents.iter().any(|d| d.modified)
    }
}

/// Global text editor
static mut TEXT_EDITOR: Option<TextEditor> = None;

/// Initialize text editor
pub fn init_text_editor() {
    unsafe {
        TEXT_EDITOR = Some(TextEditor::new());
    }
}

/// Get text editor
pub fn get_text_editor() -> &'static mut TextEditor {
    unsafe { TEXT_EDITOR.as_mut().expect("Text editor not initialized") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_editor_creation() {
        let editor = TextEditor::new();
        assert_eq!(editor.document_count(), 0);
    }

    #[test]
    fn test_new_document() {
        let mut editor = TextEditor::new();
        let index = editor.new_document("test.txt".to_string(), Language::Plain);
        assert_eq!(editor.document_count(), 1);
        assert_eq!(editor.active_document, Some(0));
    }

    #[test]
    fn test_open_document() {
        let mut editor = TextEditor::new();
        let index = editor.open_document(
            "/path/to/file.txt".to_string(),
            "content".to_string(),
            Language::Plain,
        );
        assert_eq!(editor.document_count(), 1);
    }

    #[test]
    fn test_set_content() {
        let mut editor = TextEditor::new();
        let index = editor.new_document("test.txt".to_string(), Language::Plain);
        editor
            .set_content(index, "Hello, World!".to_string())
            .unwrap();
        assert!(editor.documents[index].modified);
    }

    #[test]
    fn test_save_document() {
        let mut editor = TextEditor::new();
        let index = editor.new_document("test.txt".to_string(), Language::Plain);
        editor.set_content(index, "content".to_string()).unwrap();
        editor.save_document(index).unwrap();
        assert!(!editor.documents[index].modified);
    }

    #[test]
    fn test_move_cursor() {
        let mut editor = TextEditor::new();
        editor.move_cursor(5, 10);
        assert_eq!(editor.editor_state.cursor.line, 5);
        assert_eq!(editor.editor_state.cursor.column, 10);
    }
}
