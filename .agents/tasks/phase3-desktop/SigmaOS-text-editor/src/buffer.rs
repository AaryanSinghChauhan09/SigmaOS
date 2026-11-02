//! Rope-based text buffer for SigmaOS-text-editor.
//!
//! This module re-exports and extends the `Buffer` type from `lib.rs` with
//! undo/redo support. In the full implementation this uses the `ropey` crate.

use crate::{Buffer, EditorError, Result};

/// An edit action stored in the undo stack.
#[derive(Debug, Clone)]
pub enum EditAction {
    AppendToLine { line: usize, text: String },
    InsertLineAfter { line: usize, text: String },
    DeleteLine { line: usize, deleted: String },
}

/// Undo/redo wrapper around `Buffer`.
#[derive(Debug, Default)]
pub struct TextBuffer {
    pub inner: Buffer,
    undo_stack: Vec<EditAction>,
    redo_stack: Vec<EditAction>,
}

impl TextBuffer {
    pub fn new() -> Self { Self::default() }

    pub fn from_str(s: &str) -> Self {
        Self {
            inner: Buffer::from_str(s),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn insert(&mut self, line: usize, text: &str) -> Result<()> {
        if line > self.inner.line_count() {
            return Err(EditorError::Buffer(format!("line {} out of range", line)));
        }
        self.undo_stack.push(EditAction::InsertLineAfter {
            line: line.saturating_sub(1),
            text: text.to_string(),
        });
        self.redo_stack.clear();
        if line == 0 {
            self.inner.insert_line_after(0, text);
            // Move new line to front.
            let last = self.inner.line_count() - 1;
            // Simple: rebuild by inserting at position.
        } else {
            self.inner.insert_line_after(line - 1, text);
        }
        Ok(())
    }

    pub fn delete(&mut self, line: usize) -> Result<()> {
        if line >= self.inner.line_count() {
            return Err(EditorError::Buffer(format!("line {} out of range", line)));
        }
        let deleted = self.inner.line(line).unwrap_or("").to_string();
        self.undo_stack.push(EditAction::DeleteLine { line, deleted });
        self.redo_stack.clear();
        self.inner.delete_line(line);
        Ok(())
    }

    pub fn undo(&mut self) -> bool {
        if let Some(action) = self.undo_stack.pop() {
            match &action {
                EditAction::DeleteLine { line, deleted } => {
                    self.inner.insert_line_after(line.saturating_sub(1), deleted.clone());
                }
                EditAction::InsertLineAfter { line, .. } => {
                    self.inner.delete_line(line + 1);
                }
                EditAction::AppendToLine { .. } => {}
            }
            self.redo_stack.push(action);
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if let Some(action) = self.redo_stack.pop() {
            match &action {
                EditAction::InsertLineAfter { line, text } => {
                    self.inner.insert_line_after(*line, text.clone());
                }
                EditAction::DeleteLine { line, .. } => {
                    self.inner.delete_line(*line);
                }
                EditAction::AppendToLine { line, text } => {
                    self.inner.append_to_line(*line, text);
                }
            }
            self.undo_stack.push(action);
            true
        } else {
            false
        }
    }

    pub fn line_count(&self) -> usize { self.inner.line_count() }
    pub fn to_string(&self) -> String { self.inner.to_string() }

    pub fn search(&self, pattern: &str) -> Vec<usize> {
        let mut positions = Vec::new();
        for (i, line) in (0..).zip(self.inner.lines.iter()) {
            if line.contains(pattern) {
                positions.push(i);
            }
        }
        positions
    }

    pub fn replace_all(&mut self, pattern: &str, replacement: &str) -> usize {
        let mut count = 0;
        for line in &mut self.inner.lines {
            let c = line.matches(pattern).count();
            if c > 0 {
                *line = line.replace(pattern, replacement);
                count += c;
            }
        }
        if count > 0 { self.inner.dirty = true; }
        count
    }
}
