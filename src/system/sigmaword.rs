//! SigmaWord - Word Processing System
//!
//! Provides comprehensive word processing management inspired by Word Craft.
//! Supports document creation, text formatting, paragraphs, and document management.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
;

/// Text alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    Justify,
;

/// Font style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontStyle {
    Regular,
    Bold,
    Italic,
    BoldItalic,
;

/// Paragraph
#[derive(Debug, Clone)]
pub struct Paragraph {
    pub id: String,
    pub text: String,
    pub alignment: TextAlignment,
    pub font_size: u32,
    pub font_style: FontStyle,
    pub indent: u32,
    pub line_spacing: f32,
;

impl Paragraph {
    pub fn new(id: String, text: String) -> Self {
        Self {
            id,
            text,
            alignment: TextAlignment::Left,
            font_size: 12,
            font_style: FontStyle::Regular,
            indent: 0,
            line_spacing: 1.0,
        }
    }

    pub fn set_alignment(&mut self, alignment: TextAlignment) {
        self.alignment = alignment;
    }

    pub fn set_font_size(&mut self, size: u32) {
        self.font_size = size;
    }

    pub fn set_font_style(&mut self, style: FontStyle) {
        self.font_style = style;
    }

    pub fn set_indent(&mut self, indent: u32) {
        self.indent = indent;
    }

    pub fn set_line_spacing(&mut self, spacing: f32) {
        self.line_spacing = spacing;
    }

    pub fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }

    pub fn char_count(&self) -> usize {
        self.text.chars().count()
    }
;

/// Document
#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,
    pub name: String,
    pub author: String,
    pub title: String,
    pub paragraphs: BTreeMap<String, Paragraph>,
    pub word_count: usize,
    pub char_count: usize,
;

impl Document {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            author: String::new(),
            title: String::new(),
            paragraphs: BTreeMap::new(),
            word_count: 0,
            char_count: 0,
        }
    }

    pub fn add_paragraph(&mut self, paragraph: Paragraph) {
        let words = paragraph.word_count();
        let chars = paragraph.char_count();
        self.word_count += words;
        self.char_count += chars;
        self.paragraphs.insert(paragraph.id.clone(), paragraph);
    }

    pub fn remove_paragraph(&mut self, id: &str) -> bool {
        if let Some(para) = self.paragraphs.remove(id) {
            self.word_count -= para.word_count();
            self.char_count -= para.char_count();
            true
        } else {
            false
        }
    }

    pub fn get_paragraph(&self, id: &str) -> Option<&Paragraph> {
        self.paragraphs.get(id)
    }

    pub fn get_paragraph_mut(&mut self, id: &str) -> Option<&mut Paragraph> {
        self.paragraphs.get_mut(id)
    }

    pub fn set_metadata(&mut self, author: String, title: String) {
        self.author = author;
        self.title = title;
    }

    pub fn paragraph_count(&self) -> usize {
        self.paragraphs.len()
    }
;

/// SigmaWord - Word processing system
#[derive(Debug)]
pub struct SigmaWord {
    documents: BTreeMap<String, Document>,
    current_document: Option<String>,
;

impl SigmaWord {
    pub fn new() -> Self {
        Self {
            documents: BTreeMap::new(),
            current_document: None,
        }
    }

    pub fn add_document(&mut self, document: Document) {
        self.documents.insert(document.id.clone(), document);
    }

    pub fn remove_document(&mut self, id: &str) -> bool {
        if self.current_document.as_ref() == Some(&id.to_string()) {
            self.current_document = None;
        }
        self.documents.remove(id).is_some()
    }

    pub fn get_document(&self, id: &str) -> Option<&Document> {
        self.documents.get(id)
    }

    pub fn get_document_mut(&mut self, id: &str) -> Option<&mut Document> {
        self.documents.get_mut(id)
    }

    pub fn set_current_document(&mut self, id: &str) -> bool {
        if self.documents.contains_key(id) {
            self.current_document = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_current_document(&self) -> Option<&Document> {
        self.current_document.as_ref()
            .and_then(|id| self.documents.get(id))
    }

    pub fn get_current_document_mut(&mut self) -> Option<&mut Document> {
        self.current_document.as_ref()
            .and_then(|id| self.documents.get_mut(id.as_str()))
    }

    pub fn list_documents(&self) -> Vec<&Document> {
        self.documents.values().collect()
    }

    pub fn document_count(&self) -> usize {
        self.documents.len()
    }
;

impl Default for SigmaWord {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaWord instance
static mut SIGMAWORD_INSTANCE: Option<SigmaWord> = None;

/// Initialize the global SigmaWord instance
pub fn init_sigmaword() {
    unsafe {
        SIGMAWORD_INSTANCE = Some(SigmaWord::new());
    }
;

/// Get the global SigmaWord instance
pub fn get_sigmaword() -> Option<&'static SigmaWord> {
    unsafe {
        SIGMAWORD_INSTANCE.as_ref()
    }
;

/// Get the global SigmaWord instance (mutable)
pub fn get_sigmaword_mut() -> Option<&'static mut SigmaWord> {
    unsafe {
        SIGMAWORD_INSTANCE.as_mut()
    }
;

/// Convenience function to add a document
pub fn add_document(document: Document) -> bool {
    if let Some(word) = get_sigmaword_mut() {
        word.add_document(document);
        true
    } else {
        false
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paragraph_creation() {
        let para = Paragraph::new("para1".to_string(), "Hello world".to_string());
        assert_eq!(para.id, "para1");
        assert_eq!(para.word_count(), 2);
        assert_eq!(para.alignment, TextAlignment::Left);
    }

    #[test]
    fn test_paragraph_modification() {
        let mut para = Paragraph::new("para1".to_string(), "Test".to_string());
        para.set_alignment(TextAlignment::Center);
        para.set_font_size(14);
        para.set_font_style(FontStyle::Bold);
        assert_eq!(para.alignment, TextAlignment::Center);
        assert_eq!(para.font_size, 14);
    }

    #[test]
    fn test_document_creation() {
        let doc = Document::new("doc1".to_string(), "Report".to_string());
        assert_eq!(doc.id, "doc1");
        assert_eq!(doc.word_count, 0);
        assert_eq!(doc.paragraph_count(), 0);
    }

    #[test]
    fn test_document_management() {
        let mut doc = Document::new("doc1".to_string(), "Test".to_string());
        let para = Paragraph::new("para1".to_string(), "Hello world".to_string());
        doc.add_paragraph(para);
        assert_eq!(doc.paragraph_count(), 1);
        assert_eq!(doc.word_count, 2);
        assert!(doc.remove_paragraph("para1"));
        assert_eq!(doc.word_count, 0);
    }

    #[test]
    fn test_sigmaword_creation() {
        let word = SigmaWord::new();
        assert_eq!(word.document_count(), 0);
    }

    #[test]
    fn test_sigmaword_management() {
        let mut word = SigmaWord::new();
        let doc = Document::new("doc1".to_string(), "Test".to_string());
        word.add_document(doc);
        assert_eq!(word.document_count(), 1);
    }
;
