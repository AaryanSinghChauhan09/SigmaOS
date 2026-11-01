//! SigmaOS Text Editor — core editing engine.
//!
//! Upstream inspiration:
//! - xed:     <https://github.com/linuxmint/xed>
//! - LazyVim: <https://github.com/LazyVim/LazyVim>

use std::fmt;

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum EditorError {
    FileNotFound(String),
    PermissionDenied(String),
    EncodingError(String),
}

impl fmt::Display for EditorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EditorError::FileNotFound(p) => write!(f, "file not found: {p}"),
            EditorError::PermissionDenied(p) => write!(f, "permission denied: {p}"),
            EditorError::EncodingError(msg) => write!(f, "encoding error: {msg}"),
        }
    }
}

impl std::error::Error for EditorError {}

// ---------------------------------------------------------------------------
// Buffer
// ---------------------------------------------------------------------------

/// A text buffer holding lines of content.
#[derive(Debug, Default)]
pub struct Buffer {
    lines: Vec<String>,
    /// `true` if the buffer has unsaved modifications.
    pub dirty: bool,
}

impl Buffer {
    /// Create a buffer from a string (splits on `\n`).
    pub fn from_str(content: &str) -> Self {
        Self {
            lines: content.split('\n').map(str::to_owned).collect(),
            dirty: false,
        }
    }

    /// Returns the number of lines.
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Returns a reference to line `index`, or `None` if out of range.
    pub fn line(&self, index: usize) -> Option<&str> {
        self.lines.get(index).map(String::as_str)
    }

    /// Insert `text` at the end of line `index`.
    ///
    /// Marks the buffer dirty. Does nothing if `index` is out of range.
    pub fn append_to_line(&mut self, index: usize, text: &str) {
        if let Some(line) = self.lines.get_mut(index) {
            line.push_str(text);
            self.dirty = true;
        }
    }

    /// Insert a new line after `index`.
    pub fn insert_line_after(&mut self, index: usize, content: impl Into<String>) {
        let insert_at = (index + 1).min(self.lines.len());
        self.lines.insert(insert_at, content.into());
        self.dirty = true;
    }

    /// Delete line at `index`. No-op if out of range.
    pub fn delete_line(&mut self, index: usize) {
        if index < self.lines.len() {
            self.lines.remove(index);
            self.dirty = true;
        }
    }

    /// Render the buffer to a single string.
    pub fn to_string(&self) -> String {
        self.lines.join("\n")
    }
}

// ---------------------------------------------------------------------------
// Cursor
// ---------------------------------------------------------------------------

/// Tracks the editing cursor position.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
}

impl Cursor {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    /// Move cursor down by `n` rows, clamped to `max_row`.
    pub fn move_down(&mut self, n: usize, max_row: usize) {
        self.row = (self.row + n).min(max_row);
    }

    /// Move cursor up by `n` rows (clamped to 0).
    pub fn move_up(&mut self, n: usize) {
        self.row = self.row.saturating_sub(n);
    }
}

// ---------------------------------------------------------------------------
// Syntax highlighter
// ---------------------------------------------------------------------------

/// Token kind produced by the syntax highlighter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Keyword,
    Identifier,
    StringLiteral,
    Comment,
    Number,
    Punctuation,
    Whitespace,
}

/// A highlighted span within a line.
#[derive(Debug, Clone)]
pub struct HighlightSpan {
    pub start: usize,
    pub end: usize,
    pub kind: TokenKind,
}

/// Minimal syntax highlighter for Rust-like source code (scaffold).
#[derive(Debug, Default)]
pub struct SyntaxHighlighter {
    /// Language identifier, e.g. `"rust"`, `"python"`.
    pub language: String,
}

impl SyntaxHighlighter {
    pub fn new(language: impl Into<String>) -> Self {
        Self {
            language: language.into(),
        }
    }

    /// Highlight a single `line` and return its spans.
    pub fn highlight_line(&self, line: &str) -> Vec<HighlightSpan> {
        if line.trim_start().starts_with("//") {
            vec![HighlightSpan {
                start: 0,
                end: line.len(),
                kind: TokenKind::Comment,
            }]
        } else if !line.is_empty() {
            vec![HighlightSpan {
                start: 0,
                end: line.len(),
                kind: TokenKind::Identifier,
            }]
        } else {
            vec![]
        }
    }
}

// ---------------------------------------------------------------------------
// Editor
// ---------------------------------------------------------------------------

/// Top-level editor instance.
#[derive(Debug)]
pub struct Editor {
    pub buffer: Buffer,
    pub cursor: Cursor,
    pub highlighter: SyntaxHighlighter,
    /// Path of the open file, if any.
    pub file_path: Option<String>,
}

impl Editor {
    fn new(file: Option<&str>) -> Self {
        Self {
            buffer: Buffer::default(),
            cursor: Cursor::default(),
            highlighter: SyntaxHighlighter::new("plain"),
            file_path: file.map(str::to_owned),
        }
    }
}

// ---------------------------------------------------------------------------
// Public entry-point
// ---------------------------------------------------------------------------

/// Open an editor, optionally with a file path pre-set.
pub fn open_editor(file: Option<&str>) -> Editor {
    Editor::new(file)
}

// ---------------------------------------------------------------------------
// Inline tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_from_str_line_count() {
        let buf = Buffer::from_str("line1\nline2\nline3");
        assert_eq!(buf.line_count(), 3);
    }

    #[test]
    fn cursor_clamps_to_max_row() {
        let mut c = Cursor::default();
        c.move_down(100, 5);
        assert_eq!(c.row, 5);
    }
}
