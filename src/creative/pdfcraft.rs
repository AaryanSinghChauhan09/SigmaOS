// SPDX-License-Identifier: MIT
// SigmaOS - PDFcraft (PDF Tools Framework)
// Inspired by LibreOffice Draw, Okular, PDFtk
// Provides PDF editing and viewing capabilities

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// PDF element
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdfElement {
    Text,
    Image,
    Shape,
    Annotation,
    FormField,
}

/// PDF page
#[derive(Debug, Clone)]
pub struct PdfPage {
    pub id: u32,
    pub width: f32,
    pub height: f32,
    pub elements: Vec<PdfElement>,
}

/// PDF editor
pub struct PdfEditor {
    pub document: Option<String>,
    pub pages: BTreeMap<u32, PdfPage>,
    pub next_page_id: AtomicU32,
    pub active: AtomicBool,
}

impl PdfEditor {
    pub fn new() -> Self {
        Self {
            document: None,
            pages: BTreeMap::new(),
            next_page_id: AtomicU32::new(1),
            active: AtomicBool::new(false),
        }
    }
    
    /// Initialize PDF editor
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
    }
    
    /// Open PDF
    pub fn open_pdf(&mut self, path: String) -> Result<(), &'static str> {
        self.document = Some(path);
        Ok(())
    }
    
    /// Create new PDF
    pub fn create_pdf(&mut self, name: String) {
        self.document = Some(name);
    }
    
    /// Add page
    pub fn add_page(&mut self, width: f32, height: f32) -> u32 {
        let id = self.next_page_id.fetch_add(1, Ordering::Relaxed);
        let page = PdfPage {
            id,
            width,
            height,
            elements: Vec::new(),
        };
        self.pages.insert(id, page);
        id
    }
    
    /// Add element to page
    pub fn add_element(&mut self, page_id: u32, element: PdfElement) -> Result<(), &'static str> {
        let page = self.pages.get_mut(&page_id).ok_or("Page not found")?;
        page.elements.push(element);
        Ok(())
    }
    
    /// Remove page
    pub fn remove_page(&mut self, page_id: u32) -> Result<(), &'static str> {
        self.pages.remove(&page_id).ok_or("Page not found")?;
        Ok(())
    }
    
    /// Get page
    pub fn get_page(&self, id: u32) -> Option<&PdfPage> {
        self.pages.get(&id)
    }
    
    /// Get page count
    pub fn get_page_count(&self) -> usize {
        self.pages.len()
    }
    
    /// Is active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
}

impl Default for PdfEditor {
    fn default() -> Self {
        Self::new()
    }
}

/// Global pdfcraft
static mut PDFCRAFT: Option<PdfEditor> = None;

/// Initialize pdfcraft
pub fn init_pdfcraft() {
    unsafe {
        PDFCRAFT = Some(PdfEditor::new());
    }
}

/// Get global pdfcraft
pub fn get_pdfcraft() -> Option<&'static mut PdfEditor> {
    unsafe {
        PDFCRAFT.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pdfcraft_init() {
        let mut editor = PdfEditor::new();
        editor.init();
        assert!(editor.is_active());
    }
    
    #[test]
    fn test_create_pdf() {
        let mut editor = PdfEditor::new();
        editor.init();
        editor.create_pdf("Document.pdf".to_string());
        assert!(editor.document.is_some());
    }
    
    #[test]
    fn test_add_page() {
        let mut editor = PdfEditor::new();
        editor.init();
        editor.add_page(595.0, 842.0); // A4 size
        assert_eq!(editor.get_page_count(), 1);
    }
}
