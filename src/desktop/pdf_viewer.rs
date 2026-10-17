// SPDX-License-Identifier: MIT
// SigmaOS - PDF Viewer
// Inspired by Evince and Okular
// Provides PDF viewing and navigation

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Zoom level
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZoomLevel {
    Fit,
    FitWidth,
    Custom(f32),
}

/// Page mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageMode {
    Single,
    Continuous,
    Facing,
}

/// PDF document
#[derive(Debug, Clone)]
pub struct PdfDocument {
    pub path: String,
    pub title: String,
    pub author: String,
    pub page_count: u32,
    pub current_page: u32,
}

/// PDF viewer
pub struct PdfViewer {
    pub documents: Vec<PdfDocument>,
    pub current_document: Option<usize>,
    pub zoom: ZoomLevel,
    pub page_mode: PageMode,
    pub show_bookmarks: bool,
    pub show_thumbnails: bool,
}

impl PdfViewer {
    pub fn new() -> Self {
        Self {
            documents: Vec::new(),
            current_document: None,
            zoom: ZoomLevel::Fit,
            page_mode: PageMode::Continuous,
            show_bookmarks: true,
            show_thumbnails: true,
        }
    }

    /// Open PDF
    pub fn open(&mut self, path: String) -> Result<(), &'static str> {
        let document = PdfDocument {
            path: path.clone(),
            title: path.split('/').last().unwrap_or("Unknown").to_string(),
            author: String::new(),
            page_count: 10, // Simplified
            current_page: 0,
        };
        self.documents.push(document);
        self.current_document = Some(self.documents.len() - 1);
        Ok(())
    }

    /// Close document
    pub fn close(&mut self) -> Result<(), &'static str> {
        if let Some(index) = self.current_document {
            self.documents.remove(index);
            self.current_document = None;
            Ok(())
        } else {
            Err("No document open")
        }
    }

    /// Next page
    pub fn next_page(&mut self) -> Result<(), &'static str> {
        if let Some(index) = self.current_document {
            let doc = &mut self.documents[index];
            if doc.current_page < doc.page_count - 1 {
                doc.current_page += 1;
            }
            Ok(())
        } else {
            Err("No document open")
        }
    }

    /// Previous page
    pub fn previous_page(&mut self) -> Result<(), &'static str> {
        if let Some(index) = self.current_document {
            let doc = &mut self.documents[index];
            if doc.current_page > 0 {
                doc.current_page -= 1;
            }
            Ok(())
        } else {
            Err("No document open")
        }
    }

    /// Go to page
    pub fn go_to_page(&mut self, page: u32) -> Result<(), &'static str> {
        if let Some(index) = self.current_document {
            let doc = &mut self.documents[index];
            if page < doc.page_count {
                doc.current_page = page;
                Ok(())
            } else {
                Err("Page out of range")
            }
        } else {
            Err("No document open")
        }
    }

    /// Set zoom
    pub fn set_zoom(&mut self, zoom: ZoomLevel) {
        self.zoom = zoom;
    }

    /// Set page mode
    pub fn set_page_mode(&mut self, mode: PageMode) {
        self.page_mode = mode;
    }

    /// Toggle bookmarks
    pub fn toggle_bookmarks(&mut self) {
        self.show_bookmarks = !self.show_bookmarks;
    }

    /// Toggle thumbnails
    pub fn toggle_thumbnails(&mut self) {
        self.show_thumbnails = !self.show_thumbnails;
    }

    /// Get current document
    pub fn get_current_document(&self) -> Option<&PdfDocument> {
        self.current_document.and_then(|i| self.documents.get(i))
    }

    /// Document count
    pub fn document_count(&self) -> usize {
        self.documents.len()
    }
}

/// Global PDF viewer
static mut PDF_VIEWER: Option<PdfViewer> = None;

/// Initialize PDF viewer
pub fn init_pdf_viewer() {
    unsafe {
        PDF_VIEWER = Some(PdfViewer::new());
    }
}

/// Get PDF viewer
pub fn get_pdf_viewer() -> &'static mut PdfViewer {
    unsafe { PDF_VIEWER.as_mut().expect("PDF viewer not initialized") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdf_viewer_creation() {
        let viewer = PdfViewer::new();
        assert_eq!(viewer.document_count(), 0);
    }

    #[test]
    fn test_open_pdf() {
        let mut viewer = PdfViewer::new();
        viewer.open("/path/to/document.pdf".to_string()).unwrap();
        assert_eq!(viewer.document_count(), 1);
    }

    #[test]
    fn test_next_page() {
        let mut viewer = PdfViewer::new();
        viewer.open("/path/to/document.pdf".to_string()).unwrap();
        viewer.next_page().unwrap();
        let doc = viewer.get_current_document().unwrap();
        assert_eq!(doc.current_page, 1);
    }

    #[test]
    fn test_previous_page() {
        let mut viewer = PdfViewer::new();
        viewer.open("/path/to/document.pdf".to_string()).unwrap();
        viewer.next_page().unwrap();
        viewer.previous_page().unwrap();
        let doc = viewer.get_current_document().unwrap();
        assert_eq!(doc.current_page, 0);
    }
}
