//! SigmaOS Text Editor — rope-based editor with LSP and tree-sitter.

pub mod buffer;
pub mod lsp;
pub mod syntax;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EditorError {
    #[error("Buffer error: {0}")]
    Buffer(String),
    #[error("LSP error: {0}")]
    Lsp(String),
    #[error("Syntax error: {0}")]
    Syntax(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, EditorError>;

/// Vim-style editor modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
    Command,
}

impl Default for Mode {
    fn default() -> Self { Mode::Normal }
}

/// A cursor position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Cursor {
    pub line: usize,
    pub col: usize,
}

// ── Line-based Buffer (used by pre-existing tests) ────────────────────────────

/// Simple line-based text buffer.
#[derive(Debug, Clone)]
pub struct Buffer {
    lines: Vec<String>,
    pub dirty: bool,
    pub file_path: Option<String>,
}

impl Buffer {
    pub fn new() -> Self {
        Self { lines: vec![String::new()], dirty: false, file_path: None }
    }

    pub fn from_str(s: &str) -> Self {
        let lines: Vec<String> = s.lines().map(|l| l.to_owned()).collect();
        let lines = if lines.is_empty() { vec![String::new()] } else { lines };
        Self { lines, dirty: false, file_path: None }
    }

    pub fn line(&self, index: usize) -> Option<&str> {
        self.lines.get(index).map(|s| s.as_str())
    }

    pub fn line_count(&self) -> usize { self.lines.len() }

    pub fn append_to_line(&mut self, index: usize, text: &str) {
        if let Some(line) = self.lines.get_mut(index) {
            line.push_str(text);
            self.dirty = true;
        }
    }

    pub fn insert_line_after(&mut self, index: usize, text: impl Into<String>) {
        if index < self.lines.len() {
            self.lines.insert(index + 1, text.into());
            self.dirty = true;
        }
    }

    pub fn delete_line(&mut self, index: usize) {
        if self.lines.len() > 1 && index < self.lines.len() {
            self.lines.remove(index);
            self.dirty = true;
        }
    }

    pub fn to_string(&self) -> String {
        self.lines.join("\n")
    }
}

impl Default for Buffer {
    fn default() -> Self { Self::new() }
}

// ── SyntaxHighlighter (used by pre-existing tests) ────────────────────────────

/// Token kind for syntax highlighting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Comment,
    Keyword,
    String,
    Number,
    Identifier,
    Operator,
    Whitespace,
    Other,
}

/// A syntax span within a line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenSpan {
    pub start: usize,
    pub end: usize,
    pub kind: TokenKind,
}

/// Line-level syntax highlighter.
#[derive(Debug)]
pub struct SyntaxHighlighter {
    pub language: String,
}

impl SyntaxHighlighter {
    pub fn new(language: impl Into<String>) -> Self {
        Self { language: language.into() }
    }

    /// Highlight a single line, returning token spans.
    pub fn highlight_line(&self, line: &str) -> Vec<TokenSpan> {
        if line.is_empty() {
            return Vec::new();
        }
        // Comment detection (Rust, C, JS, Python style)
        let trimmed = line.trim_start();
        if trimmed.starts_with("//") || trimmed.starts_with('#') {
            return vec![TokenSpan { start: 0, end: line.len(), kind: TokenKind::Comment }];
        }
        // Fallback: entire line as Other/Identifier
        vec![TokenSpan { start: 0, end: line.len(), kind: TokenKind::Other }]
    }
}

// ── Editor top-level (used by pre-existing tests) ─────────────────────────────

/// Top-level editor instance.
pub struct Editor {
    pub buffer: Buffer,
    pub file_path: Option<String>,
    pub mode: Mode,
}

/// Open an editor, optionally with a file path.
pub fn open_editor(path: Option<&str>) -> Editor {
    let (buffer, file_path) = if let Some(p) = path {
        (Buffer::new(), Some(p.to_string()))
    } else {
        (Buffer::new(), None)
    };
    Editor { buffer, file_path, mode: Mode::Normal }
}
