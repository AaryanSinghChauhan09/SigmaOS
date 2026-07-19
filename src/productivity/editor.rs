// SigmaOS Code Editor
// OOP-based code editor with syntax highlighting and LSP integration

use std::collections::HashMap;
use std::path::PathBuf;

/// Document
#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,
    pub path: PathBuf,
    pub content: String,
    pub language: Language,
    pub is_modified: bool,
    pub cursor_position: CursorPosition,
}

/// Language
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Go,
    C,
    Cpp,
    Java,
    Html,
    Css,
    Markdown,
    PlainText,
}

/// Cursor position
#[derive(Debug, Clone, Copy)]
pub struct CursorPosition {
    pub line: usize,
    pub column: usize,
}

/// Text selection
#[derive(Debug, Clone)]
pub struct TextSelection {
    pub start: CursorPosition,
    pub end: CursorPosition,
}

/// Editor configuration
#[derive(Debug, Clone)]
pub struct EditorConfig {
    pub font_family: String,
    pub font_size: u16,
    pub tab_size: u16,
    pub insert_spaces: bool,
    pub word_wrap: bool,
    pub line_numbers: bool,
    pub auto_save: bool,
    pub auto_save_interval_seconds: u64,
}

/// Syntax highlighting token
#[derive(Debug, Clone)]
pub struct SyntaxToken {
    pub token_type: TokenType,
    pub start: usize,
    pub end: usize,
}

/// Token type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Keyword,
    String,
    Number,
    Comment,
    Function,
    Variable,
    Operator,
    Type,
    Constant,
}

/// OOP trait for syntax highlighters
pub trait SyntaxHighlighter {
    /// Highlight text
    fn highlight(&self, text: &str, language: Language) -> Vec<SyntaxToken>;
    /// Get highlighter name
    fn name(&self) -> &str;
}

/// Simple regex-based highlighter
pub struct RegexHighlighter;

impl SyntaxHighlighter for RegexHighlighter {
    fn highlight(&self, text: &str, language: Language) -> Vec<SyntaxToken> {
        let mut tokens = Vec::new();

        // Simple keyword highlighting based on language
        let keywords = match language {
            Language::Rust => vec!["fn", "let", "mut", "pub", "struct", "impl", "use", "mod"],
            Language::Python => vec!["def", "class", "import", "from", "return", "if", "else"],
            Language::JavaScript => vec!["function", "const", "let", "var", "return", "if", "else"],
            _ => vec![],
        };

        for keyword in keywords {
            let mut start = 0;
            while let Some(pos) = text[start..].find(keyword) {
                let actual_pos = start + pos;
                tokens.push(SyntaxToken {
                    token_type: TokenType::Keyword,
                    start: actual_pos,
                    end: actual_pos + keyword.len(),
                });
                start = actual_pos + keyword.len();
            }
        }

        tokens
    }

    fn name(&self) -> &str {
        "RegexHighlighter"
    }
}

/// LSP feature
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LspFeature {
    Completion,
    Diagnostics,
    Hover,
    GoToDefinition,
    Rename,
}

/// LSP completion item
#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionKind,
    pub detail: Option<String>,
    pub documentation: Option<String>,
}

/// Completion kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionKind {
    Function,
    Variable,
    Class,
    Interface,
    Keyword,
    Snippet,
}

/// OOP trait for LSP clients
pub trait LspClient {
    /// Request completion
    fn request_completion(&self, document: &Document, position: CursorPosition) -> Vec<CompletionItem>;
    /// Request diagnostics
    fn request_diagnostics(&self, document: &Document) -> Vec<Diagnostic>;
    /// Get LSP name
    fn name(&self) -> &str;
}

/// Diagnostic
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub message: String,
    pub severity: DiagnosticSeverity,
    pub range: TextRange,
}

/// Diagnostic severity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

/// Text range
#[derive(Debug, Clone)]
pub struct TextRange {
    pub start: CursorPosition,
    pub end: CursorPosition,
}

/// Mock LSP client
pub struct MockLspClient;

impl LspClient for MockLspClient {
    fn request_completion(&self, document: &Document, _position: CursorPosition) -> Vec<CompletionItem> {
        match document.language {
            Language::Rust => vec![
                CompletionItem {
                    label: "println!".to_string(),
                    kind: CompletionKind::Function,
                    detail: Some("Prints to stdout".to_string()),
                    documentation: Some("Macro for printing".to_string()),
                },
                CompletionItem {
                    label: "Vec".to_string(),
                    kind: CompletionKind::Class,
                    detail: Some("Vector type".to_string()),
                    documentation: None,
                },
            ],
            Language::Python => vec![
                CompletionItem {
                    label: "print".to_string(),
                    kind: CompletionKind::Function,
                    detail: Some("Print to stdout".to_string()),
                    documentation: None,
                },
            ],
            _ => Vec::new(),
        }
    }

    fn request_diagnostics(&self, _document: &Document) -> Vec<Diagnostic> {
        Vec::new()
    }

    fn name(&self) -> &str {
        "MockLspClient"
    }
}

/// OOP-based Code Editor
pub struct CodeEditor {
    documents: HashMap<String, Document>,
    active_document: Option<String>,
    highlighter: Box<dyn SyntaxHighlighter>,
    lsp_client: Option<Box<dyn LspClient>>,
    config: EditorConfig,
    unsaved_changes: bool,
}

impl CodeEditor {
    pub fn new(highlighter: Box<dyn SyntaxHighlighter>, config: EditorConfig) -> Self {
        Self {
            documents: HashMap::new(),
            active_document: None,
            highlighter,
            lsp_client: None,
            config,
            unsaved_changes: false,
        }
    }

    /// Set LSP client
    pub fn with_lsp_client(mut self, lsp_client: Box<dyn LspClient>) -> Self {
        self.lsp_client = Some(lsp_client);
        self
    }

    /// Create new document
    pub fn new_document(&mut self, language: Language) -> String {
        let doc_id = format!("doc_{}", self.documents.len());
        let document = Document {
            id: doc_id.clone(),
            path: PathBuf::from("untitled"),
            content: String::new(),
            language,
            is_modified: false,
            cursor_position: CursorPosition { line: 0, column: 0 },
        };

        self.documents.insert(doc_id.clone(), document);
        self.active_document = Some(doc_id.clone());
        doc_id
    }

    /// Open document
    pub fn open_document(&mut self, path: PathBuf, content: String, language: Language) -> String {
        let doc_id = path.to_string_lossy().to_string();
        let document = Document {
            id: doc_id.clone(),
            path: path.clone(),
            content,
            language,
            is_modified: false,
            cursor_position: CursorPosition { line: 0, column: 0 },
        };

        self.documents.insert(doc_id.clone(), document);
        self.active_document = Some(doc_id.clone());
        doc_id
    }

    /// Close document
    pub fn close_document(&mut self, doc_id: &str) -> Result<(), EditorError> {
        if let Some(doc) = self.documents.get(doc_id) {
            if doc.is_modified {
                return Err(EditorError::UnsavedChanges(doc_id.to_string()));
            }
        }

        if self.active_document.as_ref() == Some(doc_id) {
            self.active_document = None;
        }

        self.documents.remove(doc_id);
        Ok(())
    }

    /// Save document
    pub fn save_document(&mut self, doc_id: &str) -> Result<(), EditorError> {
        if let Some(doc) = self.documents.get_mut(doc_id) {
            doc.is_modified = false;
            self.unsaved_changes = false;
            Ok(())
        } else {
            Err(EditorError::DocumentNotFound(doc_id.to_string()))
        }
    }

    /// Get active document
    pub fn active_document(&self) -> Option<&Document> {
        self.active_document.and_then(|id| self.documents.get(&id))
    }

    /// Get document by ID
    pub fn get_document(&self, doc_id: &str) -> Option<&Document> {
        self.documents.get(doc_id)
    }

    /// Insert text
    pub fn insert_text(&mut self, doc_id: &str, text: String, position: CursorPosition) -> Result<(), EditorError> {
        if let Some(doc) = self.documents.get_mut(doc_id) {
            // Simple insertion (in real implementation, would handle position properly)
            doc.content.push_str(&text);
            doc.is_modified = true;
            doc.cursor_position = position;
            self.unsaved_changes = true;
            Ok(())
        } else {
            Err(EditorError::DocumentNotFound(doc_id.to_string()))
        }
    }

    /// Delete text
    pub fn delete_text(&mut self, doc_id: &str, selection: TextSelection) -> Result<(), EditorError> {
        if let Some(doc) = self.documents.get_mut(doc_id) {
            // Simple deletion (in real implementation, would handle selection properly)
            doc.is_modified = true;
            self.unsaved_changes = true;
            Ok(())
        } else {
            Err(EditorError::DocumentNotFound(doc_id.to_string()))
        }
    }

    /// Get syntax highlighting
    pub fn get_syntax_highlighting(&self, doc_id: &str) -> Vec<SyntaxToken> {
        if let Some(doc) = self.documents.get(doc_id) {
            self.highlighter.highlight(&doc.content, doc.language)
        } else {
            Vec::new()
        }
    }

    /// Request completion
    pub fn request_completion(&self, doc_id: &str, position: CursorPosition) -> Vec<CompletionItem> {
        if let (Some(doc), Some(lsp)) = (self.documents.get(doc_id), self.lsp_client.as_ref()) {
            lsp.request_completion(doc, position)
        } else {
            Vec::new()
        }
    }

    /// Request diagnostics
    pub fn request_diagnostics(&self, doc_id: &str) -> Vec<Diagnostic> {
        if let (Some(doc), Some(lsp)) = (self.documents.get(doc_id), self.lsp_client.as_ref()) {
            lsp.request_diagnostics(doc)
        } else {
            Vec::new()
        }
    }

    /// Get all documents
    pub fn documents(&self) -> Vec<&Document> {
        self.documents.values().collect()
    }

    /// Switch document
    pub fn switch_document(&mut self, doc_id: &str) -> Result<(), EditorError> {
        if self.documents.contains_key(doc_id) {
            self.active_document = Some(doc_id.to_string());
            Ok(())
        } else {
            Err(EditorError::DocumentNotFound(doc_id.to_string()))
        }
    }

    /// Has unsaved changes
    pub fn has_unsaved_changes(&self) -> bool {
        self.unsaved_changes || self.documents.values().any(|d| d.is_modified)
    }

    /// Get configuration
    pub fn config(&self) -> &EditorConfig {
        &self.config
    }

    /// Update configuration
    pub fn update_config(&mut self, config: EditorConfig) {
        self.config = config;
    }
}

impl Default for CodeEditor {
    fn default() -> Self {
        let config = EditorConfig {
            font_family: "JetBrains Mono".to_string(),
            font_size: 14,
            tab_size: 4,
            insert_spaces: true,
            word_wrap: false,
            line_numbers: true,
            auto_save: true,
            auto_save_interval_seconds: 300,
        };

        Self::new(Box::new(RegexHighlighter), config)
            .with_lsp_client(Box::new(MockLspClient))
    }
}

/// Editor errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditorError {
    DocumentNotFound(String),
    UnsavedChanges(String),
    SaveError(String),
    LspError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_highlighter() {
        let highlighter = RegexHighlighter;
        assert_eq!(highlighter.name(), "RegexHighlighter");
    }

    #[test]
    fn test_mock_lsp_client() {
        let lsp = MockLspClient;
        assert_eq!(lsp.name(), "MockLspClient");
    }

    #[test]
    fn test_code_editor() {
        let editor = CodeEditor::default();
        assert_eq!(editor.config.font_family, "JetBrains Mono");
    }

    #[test]
    fn test_new_document() {
        let mut editor = CodeEditor::default();
        let doc_id = editor.new_document(Language::Rust);
        assert!(!doc_id.is_empty());
        assert!(editor.active_document().is_some());
    }

    #[test]
    fn test_open_document() {
        let editor = CodeEditor::default();
        let doc_id = editor.open_document(
            PathBuf::from("/test/main.rs"),
            "fn main() {}".to_string(),
            Language::Rust,
        );
        assert_eq!(doc_id, "/test/main.rs");
    }
}
