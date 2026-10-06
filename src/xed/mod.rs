// SPDX-License-Identifier: MIT
// SigmaOS Xed-Inspired Text Editor
// Linux Mint Xed-inspired text editor with plugin system and advanced features

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

/// Xed-inspired plugin types
#[derive(Debug, Clone, PartialEq)]
pub enum XedPluginType {
    ChangeCase,
    DocumentStatistics,
    FileBrowser,
    IndentLines,
    InsertDateTime,
    Modelines,
    SaveWithoutTrailingSpaces,
    Sort,
    SpellChecker,
    TagList,
    WordCompletion,
    Custom,
}

/// Xed-inspired text editor plugin
#[derive(Debug, Clone)]
pub struct XedPlugin {
    pub id: String,
    pub name: String,
    pub plugin_type: XedPluginType,
    pub enabled: bool,
    pub config: BTreeMap<String, String>,
    pub python_support: bool,
}

impl XedPlugin {
    pub fn new(id: String, name: String, plugin_type: XedPluginType) -> Self {
        Self {
            id,
            name,
            plugin_type,
            enabled: true,
            config: BTreeMap::new(),
            python_support: false,
        }
    }

    /// Set plugin configuration
    pub fn set_config(&mut self, key: String, value: String) {
        self.config.insert(key, value);
    }

    /// Enable Python support
    pub fn enable_python_support(&mut self) {
        self.python_support = true;
    }
}

/// Xed-inspired text document
#[derive(Debug, Clone)]
pub struct XedDocument {
    pub name: String,
    pub path: String,
    pub content: String,
    pub modified: bool,
    pub encoding: String,
    pub language: String,
    pub cursor_position: usize,
    pub selection_start: usize,
    pub selection_end: usize,
    pub undo_stack: Vec<String>,
    pub redo_stack: Vec<String>,
}

impl XedDocument {
    pub fn new(name: String, path: String) -> Self {
        Self {
            name,
            path,
            content: String::new(),
            modified: false,
            encoding: String::from("UTF-8"),
            language: String::from("Plain Text"),
            cursor_position: 0,
            selection_start: 0,
            selection_end: 0,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    /// Insert text at cursor position
    pub fn insert_text(&mut self, text: String) {
        if text.is_empty() {
            return;
        }
        
        // Save state for undo before modification
        self.undo_stack.push(self.content.clone());
        
        let before = &self.content[..self.cursor_position];
        let after = &self.content[self.cursor_position..];
        
        self.content = format!("{}{}{}", before, text, after);
        self.cursor_position += text.len();
        self.modified = true;
    }

    /// Delete text at cursor position
    pub fn delete_text(&mut self, count: usize) {
        if self.cursor_position + count > self.content.len() {
            return;
        }
        
        // Save state for undo before modification
        self.undo_stack.push(self.content.clone());
        
        let before = &self.content[..self.cursor_position];
        let after = &self.content[self.cursor_position + count..];
        
        self.content = format!("{}{}", before, after);
        self.modified = true;
    }

    /// Undo last operation
    pub fn undo(&mut self) -> Result<(), &'static str> {
        if let Some(previous) = self.undo_stack.pop() {
            self.redo_stack.push(self.content.clone());
            self.content = previous;
            Ok(())
        } else {
            Err("Nothing to undo")
        }
    }

    /// Redo last undone operation
    pub fn redo(&mut self) -> Result<(), &'static str> {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(self.content.clone());
            self.content = next;
            Ok(())
        } else {
            Err("Nothing to redo")
        }
    }

    /// Set cursor position
    pub fn set_cursor_position(&mut self, position: usize) {
        if position <= self.content.len() {
            self.cursor_position = position;
        }
    }

    /// Set selection
    pub fn set_selection(&mut self, start: usize, end: usize) {
        if start <= end && end <= self.content.len() {
            self.selection_start = start;
            self.selection_end = end;
        }
    }

    /// Get selected text
    pub fn get_selected_text(&self) -> String {
        if self.selection_start < self.selection_end {
            self.content[self.selection_start..self.selection_end].to_string()
        } else {
            String::new()
        }
    }

    /// Search for text
    pub fn search(&self, pattern: &str) -> Vec<usize> {
        let mut matches = Vec::new();
        let mut start = 0;
        
        while let Some(pos) = self.content[start..].find(pattern) {
            matches.push(start + pos);
            start += pos + pattern.len();
        }
        
        matches
    }

    /// Replace text
    pub fn replace(&mut self, pattern: &str, replacement: &str) -> usize {
        let count = self.content.matches(pattern).count();
        self.content = self.content.replace(pattern, replacement);
        self.modified = true;
        count
    }

    /// Get document statistics
    pub fn get_statistics(&self) -> DocumentStatistics {
        let lines = self.content.lines().count();
        let words = self.content.split_whitespace().count();
        let chars = self.content.chars().count();
        
        DocumentStatistics {
            lines,
            words,
            chars,
            bytes: self.content.len(),
        }
    }

    /// Save document
    pub fn save(&mut self) -> Result<(), &'static str> {
        // In real implementation, would save to file
        self.modified = false;
        Ok(())
    }

    /// Revert document
    pub fn revert(&mut self) -> Result<(), &'static str> {
        // In real implementation, would reload from file
        self.modified = false;
        Ok(())
    }
}

/// Document statistics
#[derive(Debug, Clone)]
pub struct DocumentStatistics {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
    pub bytes: usize,
}

/// Xed-inspired text editor
#[derive(Debug, Clone)]
pub struct XedTextEditor {
    pub documents: BTreeMap<String, XedDocument>,
    pub active_document: Option<String>,
    pub plugins: BTreeMap<String, XedPlugin>,
    pub preferences: BTreeMap<String, String>,
    pub syntax_highlighting: bool,
    pub line_numbers: bool,
    pub auto_indent: bool,
    pub tab_width: u32,
}

impl XedTextEditor {
    pub fn new() -> Self {
        Self {
            documents: BTreeMap::new(),
            active_document: None,
            plugins: BTreeMap::new(),
            preferences: BTreeMap::new(),
            syntax_highlighting: true,
            line_numbers: true,
            auto_indent: true,
            tab_width: 4,
        }
    }

    /// Create new document
    pub fn new_document(&mut self, name: String, path: String) {
        let name_copy = name.clone();
        let document = XedDocument::new(name, path);
        self.documents.insert(name_copy.clone(), document);
        self.active_document = Some(name_copy);
    }

    /// Open document
    pub fn open_document(&mut self, path: String) -> Result<(), &'static str> {
        let name = path.split('/').last().unwrap_or("untitled").to_string();
        let document = XedDocument::new(name.clone(), path);
        self.documents.insert(name.clone(), document);
        self.active_document = Some(name);
        Ok(())
    }

    /// Close document
    pub fn close_document(&mut self, name: &str) -> Result<(), &'static str> {
        if let Some(active) = &self.active_document {
            if active == name {
                return Err("Cannot close active document");
            }
        }
        
        if self.documents.remove(name).is_some() {
            Ok(())
        } else {
            Err("Document not found")
        }
    }

    /// Get active document
    pub fn get_active_document(&self) -> Option<&XedDocument> {
        if let Some(ref name) = self.active_document {
            self.documents.get(name)
        } else {
            None
        }
    }

    /// Get active document (mutable)
    pub fn get_active_document_mut(&mut self) -> Option<&mut XedDocument> {
        if let Some(ref name) = self.active_document.clone() {
            self.documents.get_mut(name)
        } else {
            None
        }
    }

    /// Add plugin
    pub fn add_plugin(&mut self, plugin: XedPlugin) {
        let id = plugin.id.clone();
        self.plugins.insert(id, plugin);
    }

    /// Enable plugin
    pub fn enable_plugin(&mut self, id: String) -> Result<(), &'static str> {
        if let Some(plugin) = self.plugins.get_mut(&id) {
            plugin.enabled = true;
            Ok(())
        } else {
            Err("Plugin not found")
        }
    }

    /// Disable plugin
    pub fn disable_plugin(&mut self, id: String) -> Result<(), &'static str> {
        if let Some(plugin) = self.plugins.get_mut(&id) {
            plugin.enabled = false;
            Ok(())
        } else {
            Err("Plugin not found")
        }
    }

    /// Set preference
    pub fn set_preference(&mut self, key: String, value: String) {
        self.preferences.insert(key, value);
    }

    /// Get preference
    pub fn get_preference(&self, key: &str) -> Option<&String> {
        self.preferences.get(key)
    }

    /// List all documents
    pub fn list_documents(&self) -> Vec<String> {
        self.documents.keys().cloned().collect()
    }

    /// List all plugins
    pub fn list_plugins(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }
}

impl Default for XedTextEditor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xed_plugin() {
        let mut plugin = XedPlugin::new(
            String::from("spell-check"),
            String::from("Spell Checker"),
            XedPluginType::SpellChecker
        );
        plugin.set_config(String::from("language"), String::from("en_US"));
        plugin.enable_python_support();
        
        assert_eq!(plugin.plugin_type, XedPluginType::SpellChecker);
        assert!(plugin.python_support);
    }

    #[test]
    fn test_xed_document() {
        let mut doc = XedDocument::new(String::from("test.txt"), String::from("/home/user/test.txt"));
        doc.insert_text(String::from("Hello, World!"));
        
        assert_eq!(doc.content, "Hello, World!");
        assert!(doc.modified);
        assert_eq!(doc.cursor_position, 13);
    }

    #[test]
    fn test_undo_redo() {
        let mut doc = XedDocument::new(String::from("test.txt"), String::from("/home/user/test.txt"));
        doc.insert_text(String::from("First"));
        doc.insert_text(String::from(" Second"));
        
        // After two inserts, undo should go back to first state
        assert!(doc.undo().is_ok());
        assert_eq!(doc.content, "First");
        assert!(doc.redo().is_ok());
        assert_eq!(doc.content, "First Second");
    }

    #[test]
    fn test_search_replace() {
        let mut doc = XedDocument::new(String::from("test.txt"), String::from("/home/user/test.txt"));
        doc.insert_text(String::from("Hello World, Hello Universe"));
        
        let matches = doc.search("Hello");
        assert_eq!(matches.len(), 2);
        
        let count = doc.replace("Hello", "Hi");
        assert_eq!(count, 2);
        assert_eq!(doc.content, "Hi World, Hi Universe");
    }

    #[test]
    fn test_document_statistics() {
        let mut doc = XedDocument::new(String::from("test.txt"), String::from("/home/user/test.txt"));
        doc.insert_text(String::from("Hello World\nThis is a test"));
        
        let stats = doc.get_statistics();
        assert_eq!(stats.lines, 2);
        assert_eq!(stats.words, 6); // "Hello", "World", "This", "is", "a", "test"
    }

    #[test]
    fn test_xed_text_editor() {
        let mut editor = XedTextEditor::new();
        
        editor.new_document(String::from("new.txt"), String::from("/home/user/new.txt"));
        assert!(editor.get_active_document().is_some());
        
        let docs = editor.list_documents();
        assert!(docs.contains(&String::from("new.txt")));
    }

    #[test]
    fn test_plugin_management() {
        let mut editor = XedTextEditor::new();
        let plugin = XedPlugin::new(
            String::from("indent-lines"),
            String::from("Indent Lines"),
            XedPluginType::IndentLines
        );
        editor.add_plugin(plugin);
        
        assert!(editor.enable_plugin(String::from("indent-lines")).is_ok());
        assert!(editor.disable_plugin(String::from("indent-lines")).is_ok());
    }
}