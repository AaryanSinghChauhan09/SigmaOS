// Core Document Engine
// Supports reading and writing document formats

#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentFormat {
    PlainText,
    Markdown,
    Odt,
    Ods,
    Rtf,
    Epub,
    Asciidoc,
    Latex,
    Texinfo,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct DocumentMetadata {
    pub format: DocumentFormat,
    pub title: String,
    pub author: String,
    pub created_at: u64,
    pub modified_at: u64,
    pub word_count: usize,
    pub character_count: usize,
}

#[derive(Debug, Clone)]
pub struct Document {
    pub metadata: DocumentMetadata,
    pub content: String,
}

pub struct DocumentEngine;

impl DocumentEngine {
    pub fn new() -> Self {
        Self
    }

    /// Detect document format from file extension or content
    pub fn detect_format(filename: &str, content: &str) -> DocumentFormat {
        if filename.ends_with(".md") {
            return DocumentFormat::Markdown;
        }
        if filename.ends_with(".odt") {
            return DocumentFormat::Odt;
        }
        if filename.ends_with(".ods") {
            return DocumentFormat::Ods;
        }
        if filename.ends_with(".rtf") {
            return DocumentFormat::Rtf;
        }
        if filename.ends_with(".epub") {
            return DocumentFormat::Epub;
        }
        if filename.ends_with(".adoc") || filename.ends_with(".asciidoc") {
            return DocumentFormat::Asciidoc;
        }
        if filename.ends_with(".tex") || filename.ends_with(".latex") {
            return DocumentFormat::Latex;
        }
        if filename.ends_with(".texi") || filename.ends_with(".texinfo") {
            return DocumentFormat::Texinfo;
        }

        // Try to detect from content
        if content.starts_with("# ") {
            return DocumentFormat::Markdown;
        }
        if content.starts_with("{\\rtf") {
            return DocumentFormat::Rtf;
        }
        if content.starts_with("\\documentclass") {
            return DocumentFormat::Latex;
        }

        DocumentFormat::PlainText
    }

    /// Create a new document
    pub fn create_document(&self, title: String, author: String, content: String) -> Document {
        let word_count = content.split_whitespace().count();
        let character_count = content.chars().count();
        let timestamp = 0; // Would use actual timestamp

        let metadata = DocumentMetadata {
            format: DocumentFormat::PlainText,
            title,
            author,
            created_at: timestamp,
            modified_at: timestamp,
            word_count,
            character_count,
        };

        Document { metadata, content }
    }

    /// Load document from string
    pub fn load_document(&self, filename: String, content: String) -> Document {
        let format = Self::detect_format(&filename, &content);
        let word_count = content.split_whitespace().count();
        let character_count = content.chars().count();
        let timestamp = 0;

        let title = filename.split('.').next().unwrap_or("Untitled").to_string();

        let metadata = DocumentMetadata {
            format,
            title: title.clone(),
            author: String::new(),
            created_at: timestamp,
            modified_at: timestamp,
            word_count,
            character_count,
        };

        Document { metadata, content }
    }

    /// Convert document to different format
    pub fn convert_format(
        document: &Document,
        target_format: DocumentFormat,
    ) -> Result<String, &'static str> {
        match target_format {
            DocumentFormat::Markdown => Self::to_markdown(document),
            DocumentFormat::PlainText => Ok(document.content.clone()),
            DocumentFormat::Latex => Self::to_latex(document),
            _ => Err("Format conversion not implemented"),
        }
    }

    /// Convert to Markdown
    fn to_markdown(document: &Document) -> Result<String, &'static str> {
        let mut markdown = String::new();

        // Add title as H1
        if !document.metadata.title.is_empty() {
            markdown.push_str(&format!("# {}\n\n", document.metadata.title));
        }

        // Add content
        markdown.push_str(&document.content);

        Ok(markdown)
    }

    /// Convert to LaTeX
    fn to_latex(document: &Document) -> Result<String, &'static str> {
        let mut latex = String::new();

        latex.push_str("\\documentclass{article}\n");
        latex.push_str("\\begin{document}\n");

        if !document.metadata.title.is_empty() {
            latex.push_str(&format!("\\title{{{}}}\n", document.metadata.title));
            latex.push_str("\\maketitle\n\n");
        }

        // Simple conversion - would need proper escaping in production
        for line in document.content.lines() {
            latex.push_str(&format!("{}\n", line));
        }

        latex.push_str("\\end{document}\n");

        Ok(latex)
    }

    /// Search text in document
    pub fn search_text(document: &Document, query: &str) -> Vec<usize> {
        let mut positions = Vec::new();
        let content_lower = document.content.to_lowercase();
        let query_lower = query.to_lowercase();

        let mut start = 0;
        while let Some(pos) = content_lower[start..].find(&query_lower) {
            positions.push(start + pos);
            start += pos + query.len();
        }

        positions
    }

    /// Replace text in document
    pub fn replace_text(document: &mut Document, search: &str, replace: &str) -> usize {
        let mut count = 0;
        document.content = document.content.replace(search, replace);
        count = document.content.matches(replace).count();

        // Update metadata
        document.metadata.character_count = document.content.chars().count();
        document.metadata.word_count = document.content.split_whitespace().count();

        count
    }
}

impl Default for DocumentEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_format_detection() {
        let content = "# Title\nSome content";
        assert_eq!(
            DocumentEngine::detect_format("test.md", content),
            DocumentFormat::Markdown
        );
    }

    #[test]
    fn test_latex_format_detection() {
        let content = "\\documentclass{article}";
        assert_eq!(
            DocumentEngine::detect_format("test.tex", content),
            DocumentFormat::Latex
        );
    }

    #[test]
    fn test_rtf_format_detection() {
        let content = "{\\rtf1\\ansi";
        assert_eq!(
            DocumentEngine::detect_format("test.rtf", content),
            DocumentFormat::Rtf
        );
    }

    #[test]
    fn test_create_document() {
        let engine = DocumentEngine::new();
        let doc = engine.create_document(
            "Test Document".to_string(),
            "Author".to_string(),
            "Hello World".to_string(),
        );

        assert_eq!(doc.metadata.title, "Test Document");
        assert_eq!(doc.metadata.word_count, 2);
        assert_eq!(doc.metadata.character_count, 11);
    }

    #[test]
    fn test_load_document() {
        let engine = DocumentEngine::new();
        let doc = engine.load_document("test.md".to_string(), "# Title\nContent".to_string());

        assert_eq!(doc.metadata.format, DocumentFormat::Markdown);
        assert_eq!(doc.content, "# Title\nContent");
    }

    #[test]
    fn test_convert_to_markdown() {
        let engine = DocumentEngine::new();
        let doc = engine.create_document(
            "Test".to_string(),
            "Author".to_string(),
            "Content".to_string(),
        );

        let result = DocumentEngine::convert_format(&doc, DocumentFormat::Markdown);
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("# Test"));
    }

    #[test]
    fn test_search_text() {
        let engine = DocumentEngine::new();
        let doc = engine.create_document(
            "Test".to_string(),
            "Author".to_string(),
            "Hello World Hello".to_string(),
        );

        let positions = DocumentEngine::search_text(&doc, "Hello");
        assert_eq!(positions.len(), 2);
    }

    #[test]
    fn test_replace_text() {
        let engine = DocumentEngine::new();
        let mut doc = engine.create_document(
            "Test".to_string(),
            "Author".to_string(),
            "Hello World".to_string(),
        );

        let count = DocumentEngine::replace_text(&mut doc, "Hello", "Hi");
        assert_eq!(count, 1);
        assert_eq!(doc.content, "Hi World");
    }
}
