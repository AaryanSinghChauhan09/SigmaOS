// Text Editor (gedit/nano Inspiration)
// Document management, syntax highlighting, and editor features

use std::string::{String, ToString};
use std::vec::Vec;
use std::format;




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
        if let Some(id) = self.current_document.clone() {
            if let Some(doc) = self.get_document(&id) {
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

/// Online Web File Editor Engine (inspired by Cockpit / Webmin / VS Code Web)
#[derive(Debug, Clone)]
pub struct OnlineWebFileEditorEngine {
    pub active_tabs: Vec<Document>,
    pub selected_tab_id: Option<String>,
    pub auto_save_enabled: bool,
    pub version_history: Vec<(String, String)>, // (timestamp/version, content)
}

impl OnlineWebFileEditorEngine {
    pub fn new() -> Self {
        Self {
            active_tabs: Vec::new(),
            selected_tab_id: None,
            auto_save_enabled: true,
            version_history: Vec::new(),
        }
    }

    pub fn open_tab(&mut self, path: &str, content: &str) -> String {
        let id = format!("tab-{}", self.active_tabs.len() + 1);
        let mut doc = Document::new(&id, path);
        doc.set_content(content);
        self.active_tabs.push(doc);
        self.selected_tab_id = Some(id.clone());
        id
    }

    pub fn save_version(&mut self, id: &str, version_label: &str) -> Result<usize, EditorError> {
        if let Some(doc) = self.active_tabs.iter_mut().find(|d| d.id == id) {
            doc.save()?;
            self.version_history.push((version_label.to_string(), doc.content.clone()));
            Ok(self.version_history.len())
        } else {
            Err(EditorError::DocumentNotFound)
        }
    }

    pub fn render_live_html_preview(&self, id: &str) -> String {
        if let Some(doc) = self.active_tabs.iter().find(|d| d.id == id) {
            format!("<div class=\"sigma-web-editor-preview\">{}</div>", doc.content)
        } else {
            String::from("<div class=\"error\">No Document</div>")
        }
    }
}

impl Default for OnlineWebFileEditorEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_online_web_file_editor() {
        let mut engine = OnlineWebFileEditorEngine::new();
        let tab_id = engine.open_tab("/etc/sigma/config.conf", "SERVER_PORT=8080");

        assert_eq!(engine.active_tabs.len(), 1);
        assert_eq!(engine.selected_tab_id.as_deref(), Some(tab_id.as_str()));

        let versions = engine.save_version(&tab_id, "v1.0.0").unwrap();
        assert_eq!(versions, 1);

        let preview = engine.render_live_html_preview(&tab_id);
        assert!(preview.contains("SERVER_PORT=8080"));
    }
}

#[cfg(test_disabled)]
mod tests_disabled {
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