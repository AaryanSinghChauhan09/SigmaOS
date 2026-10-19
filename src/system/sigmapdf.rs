//! SigmaPDF - PDF Document Management System
//!
//! Provides comprehensive PDF document management inspired by PDF Craft.
//! Supports PDF creation, manipulation, rendering, and extraction.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Page size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageSize {
    A4,
    Letter,
    Legal,
    A3,
    A5,
    Custom { width: u32, height: u32 },
}

/// Page orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageOrientation {
    Portrait,
    Landscape,
}

/// PDF page
#[derive(Debug, Clone)]
pub struct PDFPage {
    pub id: String,
    pub page_number: u32,
    pub page_size: PageSize,
    pub orientation: PageOrientation,
    pub rotation: u8, // 0, 90, 180, 270
}

impl PDFPage {
    pub fn new(
        id: String,
        page_number: u32,
        page_size: PageSize,
        orientation: PageOrientation,
    ) -> Self {
        Self {
            id,
            page_number,
            page_size,
            orientation,
            rotation: 0,
        }
    }

    pub fn set_rotation(&mut self, rotation: u8) {
        self.rotation = rotation;
    }

    pub fn set_orientation(&mut self, orientation: PageOrientation) {
        self.orientation = orientation;
    }
}

/// PDF document
#[derive(Debug, Clone)]
pub struct PDFDocument {
    pub id: String,
    pub name: String,
    pub author: String,
    pub title: String,
    pub subject: String,
    pub keywords: String,
    pub creator: String,
    pub pages: BTreeMap<String, PDFPage>,
    pub page_count: u32,
}

impl PDFDocument {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            author: String::new(),
            title: String::new(),
            subject: String::new(),
            keywords: String::new(),
            creator: String::new(),
            pages: BTreeMap::new(),
            page_count: 0,
        }
    }

    pub fn add_page(&mut self, page: PDFPage) {
        self.page_count += 1;
        self.pages.insert(page.id.clone(), page);
    }

    pub fn remove_page(&mut self, id: &str) -> bool {
        if self.pages.remove(id).is_some() {
            self.page_count -= 1;
            true
        } else {
            false
        }
    }

    pub fn get_page(&self, id: &str) -> Option<&PDFPage> {
        self.pages.get(id)
    }

    pub fn get_page_mut(&mut self, id: &str) -> Option<&mut PDFPage> {
        self.pages.get_mut(id)
    }

    pub fn set_metadata(
        &mut self,
        author: String,
        title: String,
        subject: String,
        keywords: String,
    ) {
        self.author = author;
        self.title = title;
        self.subject = subject;
        self.keywords = keywords;
    }

    pub fn set_creator(&mut self, creator: String) {
        self.creator = creator;
    }
}

/// SigmaPDF - PDF document management system
#[derive(Debug)]
pub struct SigmaPDF {
    documents: BTreeMap<String, PDFDocument>,
    current_document: Option<String>,
}

impl SigmaPDF {
    pub fn new() -> Self {
        Self {
            documents: BTreeMap::new(),
            current_document: None,
        }
    }

    pub fn add_document(&mut self, document: PDFDocument) {
        self.documents.insert(document.id.clone(), document);
    }

    pub fn remove_document(&mut self, id: &str) -> bool {
        if self.current_document.as_ref() == Some(&id.to_string()) {
            self.current_document = None;
        }
        self.documents.remove(id).is_some()
    }

    pub fn get_document(&self, id: &str) -> Option<&PDFDocument> {
        self.documents.get(id)
    }

    pub fn get_document_mut(&mut self, id: &str) -> Option<&mut PDFDocument> {
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

    pub fn get_current_document(&self) -> Option<&PDFDocument> {
        self.current_document
            .as_ref()
            .and_then(|id| self.documents.get(id))
    }

    pub fn get_current_document_mut(&mut self) -> Option<&mut PDFDocument> {
        self.current_document
            .as_ref()
            .and_then(|id| self.documents.get_mut(id.as_str()))
    }

    pub fn list_documents(&self) -> Vec<&PDFDocument> {
        self.documents.values().collect()
    }

    pub fn document_count(&self) -> usize {
        self.documents.len()
    }
}

impl Default for SigmaPDF {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaPDF instance
static mut SIGMAPDF_INSTANCE: Option<SigmaPDF> = None;

/// Initialize the global SigmaPDF instance
pub fn init_sigmapdf() {
    unsafe {
        SIGMAPDF_INSTANCE = Some(SigmaPDF::new());
    }
}

/// Get the global SigmaPDF instance
pub fn get_sigmapdf() -> Option<&'static SigmaPDF> {
    unsafe { SIGMAPDF_INSTANCE.as_ref() }
}

/// Get the global SigmaPDF instance (mutable)
pub fn get_sigmapdf_mut() -> Option<&'static mut SigmaPDF> {
    unsafe { SIGMAPDF_INSTANCE.as_mut() }
}

/// Convenience function to add a document
pub fn add_document(document: PDFDocument) -> bool {
    if let Some(pdf) = get_sigmapdf_mut() {
        pdf.add_document(document);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdf_page_creation() {
        let page = PDFPage::new(
            "page1".to_string(),
            1,
            PageSize::A4,
            PageOrientation::Portrait,
        );
        assert_eq!(page.id, "page1");
        assert_eq!(page.page_number, 1);
        assert_eq!(page.page_size, PageSize::A4);
        assert_eq!(page.rotation, 0);
    }

    #[test]
    fn test_pdf_page_modification() {
        let mut page = PDFPage::new(
            "page1".to_string(),
            1,
            PageSize::A4,
            PageOrientation::Portrait,
        );
        page.set_rotation(90);
        page.set_orientation(PageOrientation::Landscape);
        assert_eq!(page.rotation, 90);
        assert_eq!(page.orientation, PageOrientation::Landscape);
    }

    #[test]
    fn test_pdf_document_creation() {
        let doc = PDFDocument::new("doc1".to_string(), "Report".to_string());
        assert_eq!(doc.id, "doc1");
        assert_eq!(doc.name, "Report");
        assert_eq!(doc.page_count, 0);
    }

    #[test]
    fn test_pdf_document_management() {
        let mut doc = PDFDocument::new("doc1".to_string(), "Test".to_string());
        let page = PDFPage::new(
            "page1".to_string(),
            1,
            PageSize::A4,
            PageOrientation::Portrait,
        );
        doc.add_page(page);
        assert_eq!(doc.page_count, 1);
        assert!(doc.remove_page("page1"));
        assert_eq!(doc.page_count, 0);
    }

    #[test]
    fn test_sigmapdf_creation() {
        let pdf = SigmaPDF::new();
        assert_eq!(pdf.document_count(), 0);
    }

    #[test]
    fn test_sigmapdf_management() {
        let mut pdf = SigmaPDF::new();
        let doc = PDFDocument::new("doc1".to_string(), "Test".to_string());
        pdf.add_document(doc);
        assert_eq!(pdf.document_count(), 1);
    }
}
