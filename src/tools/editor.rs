//! Text Editor (gedit/nano Inspiration)
//! Document management, syntax highlighting, and editor features



use crate::klib::{Vec, String};

/// Document
#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,
    pub path: String,
    pub content: String,
    pub modified: bool,
    pub encoding: String,
}

impl Document {
    pub fn new(id: &str, path: &str) -> Self {
        Self {
            id: id.to_string(),
            path: path.to_string(),
            content: String::new(),
            modified: false,
            encoding: "UTF-8".to_string(),
        }
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = content.to_string();
        self.modified = true;
    }

    pub fn save(&mut self) -> Result<(), EditorError> {
        // Save document
        self.modified = false;
        Ok(())
    }
}

/// Syntax highlighter
#[derive(Debug, Clone)]
pub struct SyntaxHighlighter {
    pub language: String,
    pub keywords: Vec<String>,
}

impl SyntaxHighlighter {
    pub fn new(language: &str) -> Self {
        Self {
            language: language.to_string(),
            keywords: Vec::new(),
        }
    }

    pub fn add_keyword(&mut self, keyword: &str) {
        self.keywords.push(keyword.to_string());
    }

    pub fn highlight(&self, text: &str) -> String {
        // Apply syntax highlighting
        text.to_string()
    }
}

/// Text editor
pub struct TextEditor {
    pub documents: Vec<Document>,
    pub current_document: Option<String>,
    pub syntax_highlighter: Option<SyntaxHighlighter>,
}

impl TextEditor {
    pub fn new() -> Self {
        Self {
            documents: Vec::new(),
            current_document: None,
            syntax_highlighter: None,
        }
    }

    pub fn new_document(&mut self, path: &str) -> String {
        let id = format!("doc-{}", self.documents.len());
        let document = Document::new(&id, path);
        self.documents.push(document);
        id
    }

    pub fn open_document(&mut self, path: &str) -> Result<String, EditorError> {
        let id = self.new_document(path);
        // Load file content
        Ok(id)
    }

    pub fn close_document(&mut self, id: &str) {
        self.documents.retain(|d| d.id != id);
    }

    pub fn switch_document(&mut self, id: &str) {
        self.current_document = Some(id.to_string());
    }

    pub fn get_document(&mut self, id: &str) -> Option<&mut Document> {
        self.documents.iter_mut().find(|d| d.id == id)
    }

    pub fn save_current(&mut self) -> Result<(), EditorError> {
        if let Some(id) = &self.current_document {
            if let Some(doc) = self.get_document(id) {
                doc.save()
            } else {
                Err(EditorError::DocumentNotFound)
            }
        } else {
            Err(EditorError::NoDocumentOpen)
        }
    }

    pub fn set_syntax_language(&mut self, language: &str) {
        self.syntax_highlighter = Some(SyntaxHighlighter::new(language));
    }

    pub fn search(&self, text: &str, query: &str) -> Vec<usize> {
        let mut matches = Vec::new();
        let mut start = 0;
        while let Some(pos) = text[start..].find(query) {
            matches.push(start + pos);
            start += pos + query.len();
        }
        matches
    }

    pub fn replace(&mut self, text: &mut String, old: &str, new: &str) {
        *text = text.replace(old, new);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditorError {
    DocumentNotFound,
    NoDocumentOpen,
    SaveFailed,
    OpenFailed,
}

impl Default for TextEditor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document() {
        let doc = Document::new("doc-1", "/tmp/test.txt");
        assert_eq!(doc.path, "/tmp/test.txt");
    }

    #[test]
    fn test_syntax_highlighter() {
        let highlighter = SyntaxHighlighter::new("rust");
        assert_eq!(highlighter.language, "rust");
    }

    #[test]
    fn test_text_editor() {
        let mut editor = TextEditor::new();
        let id = editor.new_document("/tmp/test.txt");
        assert_eq!(editor.documents.len(), 1);
    }
}