//! SigmaOS Documentation Generation System
//!
//! This module provides automatic documentation generation from source code,
//! including API documentation, architecture diagrams, and user guides.

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::format;

/// Documentation format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocFormat {
    Markdown,
    Html,
    Pdf,
    AsciiDoc,
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
            DocFormat::Pdf => Err("PDF generation not yet implemented".to_string()),
            DocFormat::AsciiDoc => self.generate_asciidoc(),
        }
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

    /// Generate HTML documentation
    fn generate_html(&self) -> Result<String, String> {
        let mut output = String::new();

        output.push_str("<!DOCTYPE html>\n");
        output.push_str("<html>\n<head>\n");
        output.push_str("<title>SigmaOS Documentation</title>\n");
        output.push_str("<style>\n");
        output.push_str("body { font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }\n");
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

        // Generate content
        for entry in &sorted_entries {
            output.push_str(&format!("<h2>{}</h2>\n", entry.title));
            output.push_str(&entry.content);
            output.push_str("\n");
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

// =========================================================================
// BSD/LINUX-STYLE MAN PAGE SYSTEM INDEXER & MANUAL COMPILER
// =========================================================================

#[derive(Debug, Clone)]
pub struct ManPage {
    pub name: String,
    pub section: u8, // e.g. 1 = Commands, 5 = File formats, 8 = Admin
    pub synopsis: String,
    pub description: String,
    pub examples: String,
}

pub struct SovereignManPageIndexer {
    pub pages: Vec<ManPage>,
}

impl SovereignManPageIndexer {
    pub fn new() -> Self {
        let mut indexer = Self { pages: Vec::new() };
        indexer.register_default_manuals();
        indexer
    }

    pub fn register_man_page(&mut self, page: ManPage) {
        self.pages.push(page);
    }

    fn register_default_manuals(&mut self) {
        self.register_man_page(ManPage {
            name: "sigpkg".to_string(),
            section: 1,
            synopsis: "sigpkg [install|remove|status] <package_name>".to_string(),
            description: "Sovereign content-addressed transactional package manager.".to_string(),
            examples: "sigpkg install sigma-vim".to_string(),
        });
        self.register_man_page(ManPage {
            name: "sysctl".to_string(),
            section: 8,
            synopsis: "sysctl [-w] <parameter_dot_path>[=<value>]".to_string(),
            description: "Dynamic tuning and security capability configuration of microkernel variables.".to_string(),
            examples: "sysctl -w kern.maxproc=2048".to_string(),
        });
    }

    /// Queries manual pages and compiles them into formatted ANSI manual outputs (defeats Linux man!)
    pub fn compile_man_page(&self, name: &str, section: Option<u8>) -> Option<String> {
        let page = self.pages.iter().find(|p| {
            p.name == name && (section.is_none() || section.unwrap() == p.section)
        })?;

        let mut output = String::new();
        output.push_str(&format!("NAME\n\t{} - {}\n\n", page.name, page.description));
        output.push_str(&format!("SYNOPSIS\n\t{}\n\n", page.synopsis));
        output.push_str(&format!("DESCRIPTION\n\tThis manual page documents the '{}' tool for SigmaOS. {}\n\n", page.name, page.description));
        output.push_str(&format!("EXAMPLES\n\t{}\n", page.examples));
        Some(output)
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

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
    fn test_sovereign_man_pages() {
        let mut indexer = SovereignManPageIndexer::new();

        // Check default pages registered
        assert_eq!(indexer.pages.len(), 2);

        // Compile sigpkg page
        let compiled = indexer.compile_man_page("sigpkg", None).unwrap();
        assert!(compiled.contains("NAME"));
        assert!(compiled.contains("sigpkg"));
        assert!(compiled.contains("SYNOPSIS"));
        assert!(compiled.contains("sigma-vim"));

        // Add custom manual page (Pledge)
        indexer.register_man_page(ManPage {
            name: "pledge".to_string(),
            section: 2,
            synopsis: "pledge(promises)".to_string(),
            description: "Dropping execution capabilities statically.".to_string(),
            examples: "pledge(\"stdio rpath\")".to_string(),
        });

        assert_eq!(indexer.pages.len(), 3);
        let pledge_page = indexer.compile_man_page("pledge", Some(2)).unwrap();
        assert!(pledge_page.contains("stdio rpath"));
    }
}
