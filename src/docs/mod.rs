pub mod knowledgebase;
pub use knowledgebase::*;

//! SigmaOS Documentation Generation System
//!
//! This module provides automatic documentation generation from source code,
//! including API documentation, architecture diagrams, and user guides.

use std::collections::BTreeMap;
use std::format;
use std::string::String;
use std::string::ToString;
use std::vec::Vec;

/// Safely escapes HTML special characters to prevent DOM text reinterpretation / XSS
pub fn escape_html(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '&' => escaped.push_str("&amp;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(c),
        }
    }
    escaped
}

/// Documentation format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocFormat {
    Markdown,
    Html,
    Pdf,
    AsciiDoc,
    PlainText,
}

/// Documentation section type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionType {
    Overview,
    API,
    Examples,
    Architecture,
    Troubleshooting,
    Reference,
}

/// Documentation entry
#[derive(Debug, Clone)]
pub struct DocEntry {
    pub title: String,
    pub content: String,
    pub section_type: SectionType,
    pub order: u32,
}

impl DocEntry {
    pub fn new(title: String, content: String, section_type: SectionType, order: u32) -> Self {
        Self {
            title,
            content,
            section_type,
            order,
        }
    }
}

/// Documentation generator
pub struct DocGenerator {
    entries: Vec<DocEntry>,
    metadata: BTreeMap<String, String>,
}

impl DocGenerator {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Add metadata
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Add a documentation entry
    pub fn add_entry(&mut self, entry: DocEntry) {
        self.entries.push(entry);
    }

    /// Generate documentation in specified format
    pub fn generate(&self, format: DocFormat) -> Result<String, String> {
        match format {
            DocFormat::Markdown => self.generate_markdown(),
            DocFormat::Html => self.generate_html(),
            DocFormat::Pdf => self.generate_pdf(),
            DocFormat::AsciiDoc => self.generate_asciidoc(),
            DocFormat::PlainText => self.generate_plain_text(),
        }
    }

    /// Generate Plain Text documentation
    fn generate_plain_text(&self) -> Result<String, String> {
        let mut output = String::new();

        if !self.metadata.is_empty() {
            output.push_str("=== METADATA ===\n");
            for (key, value) in &self.metadata {
                output.push_str(&format!("{}: {}\n", key, value));
            }
            output.push_str("\n");
        }

        let mut sorted_entries = self.entries.clone();
        sorted_entries.sort_by_key(|e| e.order);

        for entry in &sorted_entries {
            output.push_str(&format!("[ SECTION: {} ]\n", entry.title.to_uppercase()));
            output.push_str(&entry.content);
            output.push_str("\n\n");
        }

        Ok(output)
    }

    /// Generate PDF documentation (Simulated PDF document layout structure)
    fn generate_pdf(&self) -> Result<String, String> {
        let mut output = String::new();
        output.push_str("%PDF-1.4\n");
        output.push_str("1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n");
        output.push_str("2 0 obj\n<< /Type /Pages /Kids [ 3 0 R ] /Count 1 >>\nendobj\n");

        let mut content_stream = String::new();
        content_stream.push_str("BT /F1 12 Tf 50 700 Td ");

        // Sort entries by order and stream text labels
        let mut sorted_entries = self.entries.clone();
        sorted_entries.sort_by_key(|e| e.order);

        for entry in &sorted_entries {
            content_stream.push_str(&format!("({}) Tj T* ", entry.title));
        }
        content_stream.push_str("ET");

        output.push_str("3 0 obj\n<< /Type /Page /Parent 2 0 R /Contents 4 0 R >>\nendobj\n");
        output.push_str(&format!(
            "4 0 obj\n<< /Length {} >>\nstream\n{}\nendstream\nendobj\n",
            content_stream.len(),
            content_stream
        ));
        output.push_str("xref\n0 5\n0000000000 65535 f\n");
        output.push_str("trailer\n<< /Size 5 /Root 1 0 R >>\nstartxref\n%%EOF");

        Ok(output)
    }

    /// Generate Markdown documentation
    fn generate_markdown(&self) -> Result<String, String> {
        let mut output = String::new();

        // Add metadata as front matter
        if !self.metadata.is_empty() {
            output.push_str("---\n");
            for (key, value) in &self.metadata {
                output.push_str(&format!("{}: {}\n", key, value));
            }
            output.push_str("---\n\n");
        }

        // Sort entries by order
        let mut sorted_entries = self.entries.clone();
        sorted_entries.sort_by_key(|e| e.order);

        // Generate content
        for entry in &sorted_entries {
            output.push_str(&format!("## {}\n\n", entry.title));
            output.push_str(&entry.content);
            output.push_str("\n\n");
        }

        Ok(output)
    }

    /// Generate HTML documentation with XSS sanitization to prevent DOM text reinterpretation as HTML
    fn generate_html(&self) -> Result<String, String> {
        let mut output = String::new();

        output.push_str("<!DOCTYPE html>\n");
        output.push_str("<html>\n<head>\n");
        output.push_str("<title>SigmaOS Documentation</title>\n");
        output.push_str("<style>\n");
        output.push_str(
            "body { font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }\n",
        );
        output.push_str("h1 { color: #333; }\n");
        output.push_str("h2 { color: #666; border-bottom: 1px solid #eee; }\n");
        output.push_str("code { background: #f4f4f4; padding: 2px 4px; border-radius: 3px; }\n");
        output.push_str(
            "pre { background: #f4f4f4; padding: 10px; border-radius: 5px; overflow-x: auto; }\n",
        );
        output.push_str("</style>\n");
        output.push_str("</head>\n<body>\n");

        // Sort entries by order
        let mut sorted_entries = self.entries.clone();
        sorted_entries.sort_by_key(|e| e.order);

        // Generate content with html entity escaping
        for entry in &sorted_entries {
            output.push_str(&format!("<h2>{}</h2>\n", escape_html(&entry.title)));
            output.push_str(&format!("<p>{}</p>\n", escape_html(&entry.content)));
        }

        output.push_str("</body>\n</html>");

        Ok(output)
    }

    /// Generate AsciiDoc documentation
    fn generate_asciidoc(&self) -> Result<String, String> {
        let mut output = String::new();

        // Add metadata as document attributes
        if !self.metadata.is_empty() {
            for (key, value) in &self.metadata {
                output.push_str(&format!(": {}: {}\n", key, value));
            }
            output.push_str("\n");
        }

        // Sort entries by order
        let mut sorted_entries = self.entries.clone();
        sorted_entries.sort_by_key(|e| e.order);

        // Generate content
        for entry in &sorted_entries {
            output.push_str(&format!("== {}\n\n", entry.title));
            output.push_str(&entry.content);
            output.push_str("\n\n");
        }

        Ok(output)
    }

    /// Get all entries
    pub fn get_entries(&self) -> &[DocEntry] {
        &self.entries
    }

    /// Get metadata
    pub fn get_metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.clear();
        self.metadata.clear();
    }
}

impl Default for DocGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// API documentation builder
pub struct ApiDocBuilder {
    generator: DocGenerator,
}

impl ApiDocBuilder {
    pub fn new() -> Self {
        let mut generator = DocGenerator::new();
        generator.add_metadata("title".to_string(), "SigmaOS API Documentation".to_string());
        generator.add_metadata("version".to_string(), "1.0.0".to_string());

        Self { generator }
    }

    /// Add API overview
    pub fn add_overview(&mut self, content: String) {
        let entry = DocEntry::new("Overview".to_string(), content, SectionType::Overview, 1);
        self.generator.add_entry(entry);
    }

    /// Add API reference
    pub fn add_api_reference(&mut self, content: String) {
        let entry = DocEntry::new("API Reference".to_string(), content, SectionType::API, 2);
        self.generator.add_entry(entry);
    }

    /// Add examples
    pub fn add_examples(&mut self, content: String) {
        let entry = DocEntry::new("Examples".to_string(), content, SectionType::Examples, 3);
        self.generator.add_entry(entry);
    }

    /// Generate documentation
    pub fn generate(&self, format: DocFormat) -> Result<String, String> {
        self.generator.generate(format)
    }
}

impl Default for ApiDocBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_entry_creation() {
        let entry = DocEntry::new(
            "Test".to_string(),
            "Content".to_string(),
            SectionType::Overview,
            1,
        );
        assert_eq!(entry.title, "Test");
        assert_eq!(entry.section_type, SectionType::Overview);
        assert_eq!(entry.order, 1);
    }

    #[test]
    fn test_doc_generator_metadata() {
        let mut generator = DocGenerator::new();
        generator.add_metadata("key".to_string(), "value".to_string());
        assert_eq!(generator.get_metadata().len(), 1);
    }

    #[test]
    fn test_doc_generator_add_entry() {
        let mut generator = DocGenerator::new();
        let entry = DocEntry::new(
            "Test".to_string(),
            "Content".to_string(),
            SectionType::Overview,
            1,
        );
        generator.add_entry(entry);
        assert_eq!(generator.get_entries().len(), 1);
    }

    #[test]
    fn test_markdown_generation() {
        let mut generator = DocGenerator::new();
        generator.add_metadata("title".to_string(), "Test".to_string());

        let entry = DocEntry::new(
            "Test Section".to_string(),
            "Test content".to_string(),
            SectionType::Overview,
            1,
        );
        generator.add_entry(entry);

        let result = generator.generate_markdown();
        assert!(result.is_ok());
        let markdown = result.unwrap();
        assert!(markdown.contains("Test Section"));
        assert!(markdown.contains("Test content"));
    }

    #[test]
    fn test_plain_text_generation() {
        let mut generator = DocGenerator::new();
        generator.add_metadata("title".to_string(), "Test Doc".to_string());

        let entry = DocEntry::new(
            "Architecture".to_string(),
            "Sovereign Kernel Subsystem".to_string(),
            SectionType::Architecture,
            1,
        );
        generator.add_entry(entry);

        let result = generator.generate(DocFormat::PlainText);
        assert!(result.is_ok());
        let text = result.unwrap();
        assert!(text.contains("=== METADATA ==="));
        assert!(text.contains("[ SECTION: ARCHITECTURE ]"));
        assert!(text.contains("Sovereign Kernel Subsystem"));
    }

    #[test]
    fn test_html_generation() {
        let mut generator = DocGenerator::new();

        let entry = DocEntry::new(
            "Test Section".to_string(),
            "Test content".to_string(),
            SectionType::Overview,
            1,
        );
        generator.add_entry(entry);

        let result = generator.generate_html();
        assert!(result.is_ok());
        let html = result.unwrap();
        assert!(html.contains("<html>"));
        assert!(html.contains("Test Section"));
    }

    #[test]
    fn test_html_escaping_xss_prevention() {
        let mut generator = DocGenerator::new();

        let entry = DocEntry::new(
            "<script>alert('XSS')</script>".to_string(),
            "<img src=x onerror=alert(1)> & 'quote'".to_string(),
            SectionType::Overview,
            1,
        );
        generator.add_entry(entry);

        let html = generator.generate_html().unwrap();
        // Assert raw dangerous HTML tags are escaped and neutralized
        assert!(!html.contains("<script>"));
        assert!(html.contains("&lt;script&gt;alert(&#39;XSS&#39;)&lt;/script&gt;"));
        assert!(!html.contains("<img"));
        assert!(html.contains("&lt;img src=x onerror=alert(1)&gt; &amp; &#39;quote&#39;"));
    }

    #[test]
    fn test_api_doc_builder() {
        let mut builder = ApiDocBuilder::new();
        builder.add_overview("Overview content".to_string());
        builder.add_api_reference("API content".to_string());
        builder.add_examples("Example content".to_string());

        let result = builder.generate(DocFormat::Markdown);
        assert!(result.is_ok());
        let markdown = result.unwrap();
        assert!(markdown.contains("Overview"));
        assert!(markdown.contains("API Reference"));
        assert!(markdown.contains("Examples"));
    }

    #[test]
    fn test_doc_generator_clear() {
        let mut generator = DocGenerator::new();
        generator.add_metadata("key".to_string(), "value".to_string());

        let entry = DocEntry::new(
            "Test".to_string(),
            "Content".to_string(),
            SectionType::Overview,
            1,
        );
        generator.add_entry(entry);

        assert_eq!(generator.get_entries().len(), 1);
        assert_eq!(generator.get_metadata().len(), 1);

        generator.clear();

        assert_eq!(generator.get_entries().len(), 0);
        assert_eq!(generator.get_metadata().len(), 0);
    }

    #[test]
    fn test_pdf_generation() {
        let mut generator = DocGenerator::new();
        generator.add_entry(DocEntry::new(
            "Architecture Guide".to_string(),
            "Guide detail content".to_string(),
            SectionType::Architecture,
            1,
        ));

        let result = generator.generate(DocFormat::Pdf);
        assert!(result.is_ok());
        let pdf = result.unwrap();
        assert!(pdf.starts_with("%PDF-1.4"));
        assert!(pdf.contains("Architecture Guide"));
        assert!(pdf.ends_with("%%EOF"));
    }
}
