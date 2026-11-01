//! Rope-based text buffer for SigmaOS-text-editor.

use ropey::Rope;
use crate::{Cursor, Result, EditorError};

/// An edit action stored in the undo stack.
#[derive(Debug, Clone)]
pub enum EditAction {
    Insert { pos: usize, text: String },
    Delete { pos: usize, len: usize, deleted: String },
}

/// A rope-backed mutable text buffer with undo/redo.
#[derive(Debug)]
pub struct TextBuffer {
    rope: Rope,
    undo_stack: Vec<EditAction>,
    redo_stack: Vec<EditAction>,
    pub path: Option<std::path::PathBuf>,
    pub dirty: bool,
    pub encoding: String,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            path: None,
            dirty: false,
            encoding: "UTF-8".to_string(),
        }
    }

    pub fn from_str(s: &str) -> Self {
        let mut buf = Self::new();
        buf.rope = Rope::from_str(s);
        buf
    }

    pub fn insert(&mut self, char_idx: usize, text: &str) -> Result<()> {
        if char_idx > self.rope.len_chars() {
            return Err(EditorError::Buffer(format!("index {} out of range", char_idx)));
        }
        self.undo_stack.push(EditAction::Insert { pos: char_idx, text: text.to_string() });
        self.redo_stack.clear();
        self.rope.insert(char_idx, text);
        self.dirty = true;
        Ok(())
    }

    pub fn delete(&mut self, char_idx: usize, len: usize) -> Result<()> {
        let end = char_idx + len;
        if end > self.rope.len_chars() {
            return Err(EditorError::Buffer("delete range out of bounds".into()));
        }
        let deleted: String = self.rope.slice(char_idx..end).to_string();
        self.undo_stack.push(EditAction::Delete { pos: char_idx, len, deleted });
        self.redo_stack.clear();
        self.rope.remove(char_idx..end);
        self.dirty = true;
        Ok(())
    }

    pub fn undo(&mut self) -> bool {
        if let Some(action) = self.undo_stack.pop() {
            match &action {
                EditAction::Insert { pos, text } => {
                    let len = text.chars().count();
                    self.rope.remove(*pos..*pos + len);
                }
                EditAction::Delete { pos, deleted, .. } => {
                    self.rope.insert(*pos, deleted);
                }
            }
            self.redo_stack.push(action);
            self.dirty = true;
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if let Some(action) = self.redo_stack.pop() {
            match &action {
                EditAction::Insert { pos, text } => {
                    self.rope.insert(*pos, text);
                }
                EditAction::Delete { pos, len, .. } => {
                    self.rope.remove(*pos..*pos + *len);
                }
            }
            self.undo_stack.push(action);
            self.dirty = true;
            true
        } else {
            false
        }
    }

    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    pub fn to_string(&self) -> String {
        self.rope.to_string()
    }

    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    pub fn search(&self, pattern: &str) -> Vec<usize> {
        let text = self.rope.to_string();
        let mut positions = Vec::new();
        let mut start = 0;
        while let Some(pos) = text[start..].find(pattern) {
            positions.push(start + pos);
            start += pos + pattern.len();
        }
        positions
    }

    pub fn replace_all(&mut self, pattern: &str, replacement: &str) -> usize {
        let text = self.rope.to_string();
        let count = text.matches(pattern).count();
        let new_text = text.replace(pattern, replacement);
        self.rope = Rope::from_str(&new_text);
        if count > 0 { self.dirty = true; }
        count
    }
}

impl Default for TextBuffer {
    fn default() -> Self { Self::new() }
}
