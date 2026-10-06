// Xreader Document Viewer
// Linux Mint Xreader-inspired document viewer for PDF, PostScript, DJVU, DVI, XPS

use std::collections::HashMap;
use std::path::PathBuf;

/// Document format types supported by Xreader
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DocumentFormat {
    Pdf,
    PostScript,
    EncapsulatedPostScript,
    DjVu,
    Dvi,
    Xps,
    Tiff,
    Comic,
    Impress,
}

impl DocumentFormat {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "pdf" => Some(DocumentFormat::Pdf),
            "ps" => Some(DocumentFormat::PostScript),
            "eps" => Some(DocumentFormat::EncapsulatedPostScript),
            "djvu" | "djv" => Some(DocumentFormat::DjVu),
            "dvi" => Some(DocumentFormat::Dvi),
            "xps" => Some(DocumentFormat::Xps),
            "tif" | "tiff" => Some(DocumentFormat::Tiff),
            "cbr" | "cbz" | "cb7" | "cbt" => Some(DocumentFormat::Comic),
            "impress" => Some(DocumentFormat::Impress),
            _ => None,
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            DocumentFormat::Pdf => "pdf",
            DocumentFormat::PostScript => "ps",
            DocumentFormat::EncapsulatedPostScript => "eps",
            DocumentFormat::DjVu => "djvu",
            DocumentFormat::Dvi => "dvi",
            DocumentFormat::Xps => "xps",
            DocumentFormat::Tiff => "tiff",
            DocumentFormat::Comic => "cbr",
            DocumentFormat::Impress => "impress",
        }
    }
}

/// Page render quality
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderQuality {
    Low,
    Normal,
    High,
    VeryHigh,
}

/// View mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    SinglePage,
    Continuous,
    DualPage,
    DualContinuous,
}

/// Zoom level
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZoomLevel {
    FitPage,
    FitWidth,
    FitHeight,
    Custom(f64),
}

impl ZoomLevel {
    pub fn scale(&self) -> f64 {
        match self {
            ZoomLevel::FitPage => 1.0,
            ZoomLevel::FitWidth => 1.0,
            ZoomLevel::FitHeight => 1.0,
            ZoomLevel::Custom(scale) => *scale,
        }
    }
}

/// Page orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageOrientation {
    Portrait,
    Landscape,
    Auto,
}

/// Search match result
#[derive(Debug, Clone)]
pub struct SearchMatch {
    pub page: usize,
    pub start_index: usize,
    pub end_index: usize,
    pub text: String,
}

/// Bookmark
#[derive(Debug, Clone)]
pub struct Bookmark {
    pub page: usize,
    pub title: String,
    pub created_at: String,
}

/// Annotation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnotationType {
    Highlight,
    Underline,
    Note,
    Strikeout,
}

/// Annotation
#[derive(Debug, Clone)]
pub struct Annotation {
    pub id: String,
    pub page: usize,
    pub annotation_type: AnnotationType,
    pub start_index: usize,
    pub end_index: usize,
    pub content: String,
    pub color: String,
    pub created_at: String,
}

/// Document page
#[derive(Debug, Clone)]
pub struct DocumentPage {
    pub index: usize,
    pub label: String,
    pub width: f64,
    pub height: f64,
    pub rotation: i32,
}

/// Document metadata
#[derive(Debug, Clone)]
pub struct DocumentMetadata {
    pub title: String,
    pub author: String,
    pub subject: String,
    pub keywords: String,
    pub creator: String,
    pub producer: String,
    pub creation_date: String,
    pub modification_date: String,
    pub page_count: usize,
}

/// Opened document
#[derive(Debug, Clone)]
pub struct OpenedDocument {
    pub path: PathBuf,
    pub format: DocumentFormat,
    pub metadata: DocumentMetadata,
    pub pages: Vec<DocumentPage>,
    pub current_page: usize,
    pub zoom_level: ZoomLevel,
    pub view_mode: ViewMode,
    pub page_orientation: PageOrientation,
    pub render_quality: RenderQuality,
    pub inverted_colors: bool,
    pub fullscreen: bool,
    pub presentation_mode: bool,
    pub preview_mode: bool,
}

/// Xreader document viewer
#[derive(Debug, Clone)]
pub struct XreaderViewer {
    pub documents: HashMap<PathBuf, OpenedDocument>,
    pub current_document: Option<PathBuf>,
    pub recent_documents: Vec<PathBuf>,
    pub bookmarks: HashMap<PathBuf, Vec<Bookmark>>,
    pub annotations: HashMap<PathBuf, Vec<Annotation>>,
    pub search_history: Vec<String>,
    pub zoom_history: Vec<ZoomLevel>,
    pub max_recent: usize,
}

impl Default for XreaderViewer {
    fn default() -> Self {
        Self::new()
    }
}

impl XreaderViewer {
    pub fn new() -> Self {
        XreaderViewer {
            documents: HashMap::new(),
            current_document: None,
            recent_documents: Vec::new(),
            bookmarks: HashMap::new(),
            annotations: HashMap::new(),
            search_history: Vec::new(),
            zoom_history: Vec::new(),
            max_recent: 20,
        }
    }

    /// Open a document
    pub fn open_document(&mut self, path: PathBuf) -> Result<(), String> {
        let format = path
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(DocumentFormat::from_extension)
            .ok_or_else(|| "Unsupported document format".to_string())?;

        let metadata = DocumentMetadata {
            title: path.file_name().unwrap().to_string_lossy().to_string(),
            author: String::new(),
            subject: String::new(),
            keywords: String::new(),
            creator: String::new(),
            producer: String::new(),
            creation_date: String::new(),
            modification_date: String::new(),
            page_count: 1,
        };

        let pages = vec![DocumentPage {
            index: 0,
            label: "1".to_string(),
            width: 595.0,
            height: 842.0,
            rotation: 0,
        }];

        let document = OpenedDocument {
            path: path.clone(),
            format,
            metadata,
            pages,
            current_page: 0,
            zoom_level: ZoomLevel::FitPage,
            view_mode: ViewMode::SinglePage,
            page_orientation: PageOrientation::Auto,
            render_quality: RenderQuality::Normal,
            inverted_colors: false,
            fullscreen: false,
            presentation_mode: false,
            preview_mode: false,
        };

        self.documents.insert(path.clone(), document);
        self.current_document = Some(path.clone());
        self.add_to_recent(path);

        Ok(())
    }

    /// Close current document
    pub fn close_document(&mut self) -> Result<(), String> {
        if let Some(path) = self.current_document.take() {
            self.documents.remove(&path);
            Ok(())
        } else {
            Err("No document open".to_string())
        }
    }

    /// Get current document
    pub fn get_current_document(&self) -> Option<&OpenedDocument> {
        self.current_document
            .as_ref()
            .and_then(|path| self.documents.get(path))
    }

    /// Navigate to specific page
    pub fn go_to_page(&mut self, page: usize) -> Result<(), String> {
        if let Some(path) = &self.current_document {
            if let Some(doc) = self.documents.get_mut(path) {
                if page < doc.pages.len() {
                    doc.current_page = page;
                    return Ok(());
                }
            }
        }
        Err("Invalid page number".to_string())
    }

    /// Go to next page
    pub fn next_page(&mut self) -> Result<(), String> {
        if let Some(path) = &self.current_document {
            if let Some(doc) = self.documents.get_mut(path) {
                if doc.current_page + 1 < doc.pages.len() {
                    doc.current_page += 1;
                    return Ok(());
                }
            }
        }
        Err("Already on last page".to_string())
    }

    /// Go to previous page
    pub fn previous_page(&mut self) -> Result<(), String> {
        if let Some(path) = &self.current_document {
            if let Some(doc) = self.documents.get_mut(path) {
                if doc.current_page > 0 {
                    doc.current_page -= 1;
                    return Ok(());
                }
            }
        }
        Err("Already on first page".to_string())
    }

    /// Go to first page
    pub fn first_page(&mut self) -> Result<(), String> {
        if let Some(path) = &self.current_document {
            if let Some(doc) = self.documents.get_mut(path) {
                doc.current_page = 0;
                return Ok(());
            }
        }
        Err("No document open".to_string())
    }

    /// Go to last page
    pub fn last_page(&mut self) -> Result<(), String> {
        if let Some(path) = &self.current_document {
            if let Some(doc) = self.documents.get_mut(path) {
                doc.current_page = doc.pages.len() - 1;
                return Ok(());
            }
        }
        Err("No document open".to_string())
    }

    /// Set zoom level
    pub fn set_zoom(&mut self, zoom: ZoomLevel) -> Result<(), String> {
        if let Some(path) = &self.current_document {
            if let Some(doc) = self.documents.get_mut(path) {
                self.zoom_history.push(doc.zoom_level);
                doc.zoom_level = zoom;
                return Ok(());
            }
        }
        Err("No document open".to_string())
    }

    /// Zoom in
    pub fn zoom_in(&mut self) -> Result<(), String> {
        if let Some(path) = &self.current_document {
            if let Some(doc) = self.documents.get_mut(path) {
                let current = doc.zoom_level.scale();
                let new_zoom = ZoomLevel::Custom((current * 1.2).min(5.0));
                self.zoom_history.push(doc.zoom_level);
                doc.zoom_level = new_zoom;
                return Ok(());
            }
        }
        Err("No document open".to_string())
    }

    /// Zoom out
    pub fn zoom_out(&mut self) -> Result<(), String> {
        if let Some(path) = &self.current_document {
            if let Some(doc) = self.documents.get_mut(path) {
                let current = doc.zoom_level.scale();
                let new_zoom = ZoomLevel::Custom((current / 1.2).max(0.1));
                self.zoom_history.push(doc.zoom_level);
                doc.zoom_level = new_zoom;
                return Ok(());
            }
        }
        Err("No document open".to_string())
    }

    /// Reset zoom
    pub fn reset_zoom(&mut self) -> Result<(), String> {
        self.set_zoom(ZoomLevel::FitPage)
    }

    /// Set view mode
    pub fn set_view_mode(&mut self, mode: ViewMode) -> Result<(), String> {
        if let Some(path) = &self.current_document {
            if let Some(doc) = self.documents.get_mut(path) {
                doc.view_mode = mode;
                return Ok(());
            }
        }
        Err("No document open".to_string())
    }

    /// Set page orientation
    pub fn set_page_orientation(&mut self, orientation: PageOrientation) -> Result<(), String> {
        if let Some(path) = &self.current_document {
            if let Some(doc) = self.documents.get_mut(path) {
                doc.page_orientation = orientation;
                return Ok(());
            }
        }
        Err("No document open".to_string())
    }

    /// Set render quality
    pub fn set_render_quality(&mut self, quality: RenderQuality) -> Result<(), String> {
        if let Some(path) = &self.current_document {
            if let Some(doc) = self.documents.get_mut(path) {
                doc.render_quality = quality;
                return Ok(());
            }
        }
        Err("No document open".to_string())
    }

    /// Toggle inverted colors
    pub fn toggle_inverted_colors(&mut self) -> Result<(), String> {
        if let Some(path) = &self.current_document {
            if let Some(doc) = self.documents.get_mut(path) {
                doc.inverted_colors = !doc.inverted_colors;
                return Ok(());
            }
        }
        Err("No document open".to_string())
    }

    /// Toggle fullscreen
    pub fn toggle_fullscreen(&mut self) -> Result<(), String> {
        if let Some(path) = &self.current_document {
            if let Some(doc) = self.documents.get_mut(path) {
                doc.fullscreen = !doc.fullscreen;
                return Ok(());
            }
        }
        Err("No document open".to_string())
    }

    /// Toggle presentation mode
    pub fn toggle_presentation_mode(&mut self) -> Result<(), String> {
        if let Some(path) = &self.current_document {
            if let Some(doc) = self.documents.get_mut(path) {
                doc.presentation_mode = !doc.presentation_mode;
                if doc.presentation_mode {
                    doc.fullscreen = true;
                }
                return Ok(());
            }
        }
        Err("No document open".to_string())
    }

    /// Toggle preview mode
    pub fn toggle_preview_mode(&mut self) -> Result<(), String> {
        if let Some(path) = &self.current_document {
            if let Some(doc) = self.documents.get_mut(path) {
                doc.preview_mode = !doc.preview_mode;
                return Ok(());
            }
        }
        Err("No document open".to_string())
    }

    /// Search text in document
    pub fn search_text(&mut self, query: String) -> Result<Vec<SearchMatch>, String> {
        if let Some(path) = &self.current_document {
            if self.documents.contains_key(path) {
                self.search_history.push(query.clone());
                if self.search_history.len() > 50 {
                    self.search_history.remove(0);
                }

                let matches = vec![SearchMatch {
                    page: 0,
                    start_index: 0,
                    end_index: query.len(),
                    text: query,
                }];

                Ok(matches)
            } else {
                Err("Document not found".to_string())
            }
        } else {
            Err("No document open".to_string())
        }
    }

    /// Add bookmark
    pub fn add_bookmark(&mut self, page: usize, title: String) -> Result<(), String> {
        if let Some(path) = &self.current_document {
            let bookmarks = self.bookmarks.entry(path.clone()).or_insert_with(Vec::new);
            bookmarks.push(Bookmark {
                page,
                title,
                created_at: "now".to_string(),
            });
            Ok(())
        } else {
            Err("No document open".to_string())
        }
    }

    /// Remove bookmark
    pub fn remove_bookmark(&mut self, page: usize) -> Result<(), String> {
        if let Some(path) = &self.current_document {
            if let Some(bookmarks) = self.bookmarks.get_mut(path) {
                bookmarks.retain(|b| b.page != page);
                return Ok(());
            }
        }
        Err("No document open".to_string())
    }

    /// Get bookmarks
    pub fn get_bookmarks(&self) -> Vec<&Bookmark> {
        self.current_document
            .as_ref()
            .and_then(|path| self.bookmarks.get(path))
            .map(|b| b.iter().collect())
            .unwrap_or_default()
    }

    /// Add annotation
    pub fn add_annotation(
        &mut self,
        page: usize,
        annotation_type: AnnotationType,
        start_index: usize,
        end_index: usize,
        content: String,
        color: String,
    ) -> Result<(), String> {
        if let Some(path) = &self.current_document {
            let annotations = self.annotations.entry(path.clone()).or_insert_with(Vec::new);
            annotations.push(Annotation {
                id: format!("{}_{}", page, annotations.len()),
                page,
                annotation_type,
                start_index,
                end_index,
                content,
                color,
                created_at: "now".to_string(),
            });
            Ok(())
        } else {
            Err("No document open".to_string())
        }
    }

    /// Remove annotation
    pub fn remove_annotation(&mut self, id: String) -> Result<(), String> {
        if let Some(path) = &self.current_document {
            if let Some(annotations) = self.annotations.get_mut(path) {
                annotations.retain(|a| a.id != id);
                return Ok(());
            }
        }
        Err("No document open".to_string())
    }

    /// Get annotations
    pub fn get_annotations(&self) -> Vec<&Annotation> {
        self.current_document
            .as_ref()
            .and_then(|path| self.annotations.get(path))
            .map(|a| a.iter().collect())
            .unwrap_or_default()
    }

    /// Get recent documents
    pub fn get_recent_documents(&self) -> &[PathBuf] {
        &self.recent_documents
    }

    /// Get search history
    pub fn get_search_history(&self) -> &[String] {
        &self.search_history
    }

    /// Clear search history
    pub fn clear_search_history(&mut self) {
        self.search_history.clear();
    }

    /// Add to recent documents
    fn add_to_recent(&mut self, path: PathBuf) {
        self.recent_documents.retain(|p| p != &path);
        self.recent_documents.insert(0, path);
        if self.recent_documents.len() > self.max_recent {
            self.recent_documents.pop();
        }
    }

    /// Get statistics
    pub fn get_statistics(&self) -> (usize, usize, usize, usize) {
        (
            self.documents.len(),
            self.bookmarks.values().map(|b| b.len()).sum(),
            self.annotations.values().map(|a| a.len()).sum(),
            self.search_history.len(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_xreader_creation() {
        let viewer = XreaderViewer::new();
        assert_eq!(viewer.documents.len(), 0);
        assert_eq!(viewer.recent_documents.len(), 0);
    }

    #[test]
    fn test_open_document() {
        let mut viewer = XreaderViewer::new();
        let path = PathBuf::from("/test/document.pdf");
        viewer.open_document(path.clone()).unwrap();
        assert_eq!(viewer.documents.len(), 1);
        assert_eq!(viewer.current_document, Some(path));
    }

    #[test]
    fn test_navigation() {
        let mut viewer = XreaderViewer::new();
        let path = PathBuf::from("/test/document.pdf");
        viewer.open_document(path).unwrap();
        viewer.next_page().unwrap_err(); // Already on last page
        viewer.previous_page().unwrap_err(); // Already on first page
    }

    #[test]
    fn test_zoom() {
        let mut viewer = XreaderViewer::new();
        let path = PathBuf::from("/test/document.pdf");
        viewer.open_document(path).unwrap();
        viewer.zoom_in().unwrap();
        viewer.zoom_out().unwrap();
        viewer.reset_zoom().unwrap();
    }

    #[test]
    fn test_view_mode() {
        let mut viewer = XreaderViewer::new();
        let path = PathBuf::from("/test/document.pdf");
        viewer.open_document(path).unwrap();
        viewer.set_view_mode(ViewMode::Continuous).unwrap();
        viewer.set_view_mode(ViewMode::DualPage).unwrap();
    }

    #[test]
    fn test_inverted_colors() {
        let mut viewer = XreaderViewer::new();
        let path = PathBuf::from("/test/document.pdf");
        viewer.open_document(path).unwrap();
        viewer.toggle_inverted_colors().unwrap();
        let doc = viewer.get_current_document().unwrap();
        assert!(doc.inverted_colors);
    }

    #[test]
    fn test_fullscreen() {
        let mut viewer = XreaderViewer::new();
        let path = PathBuf::from("/test/document.pdf");
        viewer.open_document(path).unwrap();
        viewer.toggle_fullscreen().unwrap();
        let doc = viewer.get_current_document().unwrap();
        assert!(doc.fullscreen);
    }

    #[test]
    fn test_presentation_mode() {
        let mut viewer = XreaderViewer::new();
        let path = PathBuf::from("/test/document.pdf");
        viewer.open_document(path).unwrap();
        viewer.toggle_presentation_mode().unwrap();
        let doc = viewer.get_current_document().unwrap();
        assert!(doc.presentation_mode);
        assert!(doc.fullscreen);
    }

    #[test]
    fn test_bookmarks() {
        let mut viewer = XreaderViewer::new();
        let path = PathBuf::from("/test/document.pdf");
        viewer.open_document(path).unwrap();
        viewer.add_bookmark(0, "Chapter 1".to_string()).unwrap();
        let bookmarks = viewer.get_bookmarks();
        assert_eq!(bookmarks.len(), 1);
        viewer.remove_bookmark(0).unwrap();
        let bookmarks = viewer.get_bookmarks();
        assert_eq!(bookmarks.len(), 0);
    }

    #[test]
    fn test_annotations() {
        let mut viewer = XreaderViewer::new();
        let path = PathBuf::from("/test/document.pdf");
        viewer.open_document(path).unwrap();
        viewer
            .add_annotation(
                0,
                AnnotationType::Highlight,
                0,
                10,
                "Important".to_string(),
                "yellow".to_string(),
            )
            .unwrap();
        let annotations = viewer.get_annotations();
        assert_eq!(annotations.len(), 1);
    }

    #[test]
    fn test_search() {
        let mut viewer = XreaderViewer::new();
        let path = PathBuf::from("/test/document.pdf");
        viewer.open_document(path).unwrap();
        let matches = viewer.search_text("test".to_string()).unwrap();
        assert_eq!(matches.len(), 1);
        assert_eq!(viewer.search_history.len(), 1);
    }

    #[test]
    fn test_statistics() {
        let mut viewer = XreaderViewer::new();
        let path = PathBuf::from("/test/document.pdf");
        viewer.open_document(path).unwrap();
        viewer.add_bookmark(0, "Test".to_string()).unwrap();
        viewer
            .add_annotation(
                0,
                AnnotationType::Highlight,
                0,
                10,
                "Test".to_string(),
                "yellow".to_string(),
            )
            .unwrap();
        viewer.search_text("test".to_string()).unwrap();
        let (docs, bookmarks, annotations, searches) = viewer.get_statistics();
        assert_eq!(docs, 1);
        assert_eq!(bookmarks, 1);
        assert_eq!(annotations, 1);
        assert_eq!(searches, 1);
    }
}
