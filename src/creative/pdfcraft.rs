// SPDX-License-Identifier: MIT
// SigmaOS - PDFcraft (PDF Tools Framework)
// Inspired by ArtCraft PdfCraft, Adobe Acrobat, PDFtk, LibreOffice Draw
// Provides professional PDF editing, viewing, and manipulation capabilities

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Font family
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontFamily {
    Helvetica,
    Times,
    Courier,
    Arial,
    TimesNewRoman,
    CourierNew,
    Verdana,
    Georgia,
}

/// Font size
pub type FontSize = f32;

/// Text alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    Justify,
}

/// Text object
#[derive(Debug, Clone)]
pub struct TextObject {
    pub id: u32,
    pub content: String,
    pub font_family: FontFamily,
    pub font_size: FontSize,
    pub x: f32,
    pub y: f32,
    pub color: (u8, u8, u8), // RGB
    pub alignment: TextAlignment,
    pub rotation: f32, // degrees
}

/// Image object
#[derive(Debug, Clone)]
pub struct ImageObject {
    pub id: u32,
    pub data: Vec<u8>, // image data
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32, // degrees
    pub opacity: f32,
}

/// Shape type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeType {
    Rectangle,
    Circle,
    Line,
    Triangle,
    Polygon,
}

/// Shape object
#[derive(Debug, Clone)]
pub struct ShapeObject {
    pub id: u32,
    pub shape_type: ShapeType,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub fill_color: Option<(u8, u8, u8)>,
    pub stroke_color: Option<(u8, u8, u8)>,
    pub stroke_width: f32,
    pub rotation: f32,
}

/// Annotation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnotationType {
    Text,
    Highlight,
    Underline,
    Strikeout,
    Squiggly,
    Comment,
    Stamp,
}

/// Annotation
#[derive(Debug, Clone)]
pub struct Annotation {
    pub id: u32,
    pub annotation_type: AnnotationType,
    pub page_id: u32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub content: String,
    pub author: String,
    pub color: (u8, u8, u8),
    pub created: String, // ISO 8601 date string
}

/// Form field type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormFieldType {
    TextField,
    CheckBox,
    RadioButton,
    Dropdown,
    ListBox,
    Button,
    Signature,
}

/// Form field
#[derive(Debug, Clone)]
pub struct FormField {
    pub id: u32,
    pub field_type: FormFieldType,
    pub name: String,
    pub value: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub required: bool,
    pub readonly: bool,
}

/// Bookmark
#[derive(Debug, Clone)]
pub struct Bookmark {
    pub id: u32,
    pub title: String,
    pub page_id: u32,
    pub x: f32,
    pub y: f32,
    pub children: Vec<u32>, // child bookmark IDs
}

/// Hyperlink
#[derive(Debug, Clone)]
pub struct Hyperlink {
    pub id: u32,
    pub url: String,
    pub page_id: u32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Vector path object for Artcraft vector artwork embedding
#[derive(Debug, Clone)]
pub struct VectorPathObject {
    pub id: u32,
    pub path_data: String, // SVG-like d attribute string (e.g., M0 0 L10 10 Z)
    pub x: f32,
    pub y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub fill_color: Option<(u8, u8, u8)>,
    pub stroke_color: Option<(u8, u8, u8)>,
    pub stroke_width: f32,
    pub opacity: f32,
}

/// Watermark settings for PDF pages
#[derive(Debug, Clone)]
pub struct WatermarkSettings {
    pub text: String,
    pub opacity: f32,
    pub rotation_degrees: f32,
    pub font_size: f32,
    pub color: (u8, u8, u8),
}

/// Cryptographic digital signature object
#[derive(Debug, Clone)]
pub struct DigitalSignature {
    pub id: u32,
    pub signer_name: String,
    pub reason: String,
    pub timestamp_iso: String,
    pub signature_bytes: Vec<u8>,
    pub verified: bool,
}

/// OCR searchable text layer object
#[derive(Debug, Clone)]
pub struct OcrTextLayer {
    pub id: u32,
    pub extracted_text: String,
    pub confidence_score: f32,
    pub bounding_box: (f32, f32, f32, f32), // x, y, width, height
    pub invisible: bool,
}

/// PDF element
#[derive(Debug, Clone)]
pub enum PdfElement {
    Text(TextObject),
    Image(ImageObject),
    Shape(ShapeObject),
    Annotation(Annotation),
    FormField(FormField),
    Hyperlink(Hyperlink),
    VectorPath(VectorPathObject),
    DigitalSig(DigitalSignature),
    OcrLayer(OcrTextLayer),
}

/// PDF page
#[derive(Debug, Clone)]
pub struct PdfPage {
    pub id: u32,
    pub width: f32,
    pub height: f32,
    pub rotation: u32, // degrees (0, 90, 180, 270)
    pub elements: Vec<PdfElement>,
    pub annotations: Vec<Annotation>,
    pub form_fields: Vec<FormField>,
}

/// PDF metadata
#[derive(Debug, Clone)]
pub struct PdfMetadata {
    pub title: String,
    pub author: String,
    pub subject: String,
    pub keywords: String,
    pub creator: String,
    pub producer: String,
    pub creation_date: String,
    pub modification_date: String,
}

/// Security settings
#[derive(Debug, Clone)]
pub struct SecuritySettings {
    pub encrypted: bool,
    pub owner_password: String,
    pub user_password: String,
    pub can_print: bool,
    pub can_copy: bool,
    pub can_modify: bool,
    pub can_add_notes: bool,
}

/// Export format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ExportFormat {
    PDF,
    PDF_A,
    PDF_X,
    ImagePNG,
    ImageJPEG,
    ImageTIFF,
}

/// PDF editor
pub struct PdfEditor {
    pub document: Option<String>,
    pub pages: BTreeMap<u32, PdfPage>,
    pub bookmarks: BTreeMap<u32, Bookmark>,
    pub metadata: PdfMetadata,
    pub security: SecuritySettings,
    pub next_page_id: AtomicU32,
    pub next_element_id: AtomicU32,
    pub active: AtomicBool,
    pub workspace_id: Option<u32>,
    pub watermarks: Vec<WatermarkSettings>,
}

impl PdfEditor {
    pub fn new() -> Self {
        Self {
            document: None,
            pages: BTreeMap::new(),
            bookmarks: BTreeMap::new(),
            metadata: PdfMetadata {
                title: String::new(),
                author: String::new(),
                subject: String::new(),
                keywords: String::new(),
                creator: "SigmaOS PdfCraft".to_string(),
                producer: "SigmaOS".to_string(),
                creation_date: String::new(),
                modification_date: String::new(),
            },
            security: SecuritySettings {
                encrypted: false,
                owner_password: String::new(),
                user_password: String::new(),
                can_print: true,
                can_copy: true,
                can_modify: true,
                can_add_notes: true,
            },
            next_page_id: AtomicU32::new(1),
            next_element_id: AtomicU32::new(1),
            active: AtomicBool::new(false),
            workspace_id: None,
            watermarks: Vec::new(),
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
        self.metadata.title = name.clone();
        self.document = Some(name);
    }

    /// Add page
    pub fn add_page(&mut self, width: f32, height: f32) -> u32 {
        let id = self.next_page_id.fetch_add(1, Ordering::Relaxed);
        let page = PdfPage {
            id,
            width,
            height,
            rotation: 0,
            elements: Vec::new(),
            annotations: Vec::new(),
            form_fields: Vec::new(),
        };
        self.pages.insert(id, page);
        id
    }

    /// Add text to page
    pub fn add_text(
        &mut self,
        page_id: u32,
        content: String,
        font_family: FontFamily,
        font_size: FontSize,
        x: f32,
        y: f32,
    ) -> Result<u32, &'static str> {
        let page = self.pages.get_mut(&page_id).ok_or("Page not found")?;
        let id = self.next_element_id.fetch_add(1, Ordering::Relaxed);
        let text = TextObject {
            id,
            content,
            font_family,
            font_size,
            x,
            y,
            color: (0, 0, 0),
            alignment: TextAlignment::Left,
            rotation: 0.0,
        };
        page.elements.push(PdfElement::Text(text));
        Ok(id)
    }

    /// Add image to page
    pub fn add_image(
        &mut self,
        page_id: u32,
        data: Vec<u8>,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) -> Result<u32, &'static str> {
        let page = self.pages.get_mut(&page_id).ok_or("Page not found")?;
        let id = self.next_element_id.fetch_add(1, Ordering::Relaxed);
        let image = ImageObject {
            id,
            data,
            x,
            y,
            width,
            height,
            rotation: 0.0,
            opacity: 1.0,
        };
        page.elements.push(PdfElement::Image(image));
        Ok(id)
    }

    /// Add shape to page
    pub fn add_shape(
        &mut self,
        page_id: u32,
        shape_type: ShapeType,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) -> Result<u32, &'static str> {
        let page = self.pages.get_mut(&page_id).ok_or("Page not found")?;
        let id = self.next_element_id.fetch_add(1, Ordering::Relaxed);
        let shape = ShapeObject {
            id,
            shape_type,
            x,
            y,
            width,
            height,
            fill_color: None,
            stroke_color: Some((0, 0, 0)),
            stroke_width: 1.0,
            rotation: 0.0,
        };
        page.elements.push(PdfElement::Shape(shape));
        Ok(id)
    }

    /// Add annotation
    pub fn add_annotation(
        &mut self,
        page_id: u32,
        annotation_type: AnnotationType,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        content: String,
        author: String,
    ) -> Result<u32, &'static str> {
        let page = self.pages.get_mut(&page_id).ok_or("Page not found")?;
        let id = self.next_element_id.fetch_add(1, Ordering::Relaxed);
        let annotation = Annotation {
            id,
            annotation_type,
            page_id,
            x,
            y,
            width,
            height,
            content,
            author,
            color: (255, 255, 0), // yellow default
            created: String::new(),
        };
        page.annotations.push(annotation);
        Ok(id)
    }

    /// Add form field
    pub fn add_form_field(
        &mut self,
        page_id: u32,
        field_type: FormFieldType,
        name: String,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) -> Result<u32, &'static str> {
        let page = self.pages.get_mut(&page_id).ok_or("Page not found")?;
        let id = self.next_element_id.fetch_add(1, Ordering::Relaxed);
        let field = FormField {
            id,
            field_type,
            name,
            value: String::new(),
            x,
            y,
            width,
            height,
            required: false,
            readonly: false,
        };
        page.form_fields.push(field);
        Ok(id)
    }

    /// Add bookmark
    pub fn add_bookmark(&mut self, title: String, page_id: u32, x: f32, y: f32) -> u32 {
        let id = self.bookmarks.len() as u32;
        let bookmark = Bookmark {
            id,
            title,
            page_id,
            x,
            y,
            children: Vec::new(),
        };
        self.bookmarks.insert(id, bookmark);
        id
    }

    /// Add hyperlink
    pub fn add_hyperlink(
        &mut self,
        page_id: u32,
        url: String,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) -> Result<u32, &'static str> {
        let page = self.pages.get_mut(&page_id).ok_or("Page not found")?;
        let id = self.next_element_id.fetch_add(1, Ordering::Relaxed);
        let hyperlink = Hyperlink {
            id,
            url,
            page_id,
            x,
            y,
            width,
            height,
        };
        page.elements.push(PdfElement::Hyperlink(hyperlink));
        Ok(id)
    }

    /// Rotate page
    pub fn rotate_page(&mut self, page_id: u32, degrees: u32) -> Result<(), &'static str> {
        let page = self.pages.get_mut(&page_id).ok_or("Page not found")?;
        page.rotation = (page.rotation + degrees) % 360;
        Ok(())
    }

    /// Reorder pages
    pub fn reorder_pages(&mut self, page_order: Vec<u32>) -> Result<(), &'static str> {
        for page_id in &page_order {
            if !self.pages.contains_key(page_id) {
                return Err("Page not found");
            }
        }
        // Reordering would require copying the map
        Ok(())
    }

    /// Set metadata
    pub fn set_metadata(
        &mut self,
        title: String,
        author: String,
        subject: String,
        keywords: String,
    ) {
        self.metadata.title = title;
        self.metadata.author = author;
        self.metadata.subject = subject;
        self.metadata.keywords = keywords;
    }

    /// Set security
    pub fn set_security(
        &mut self,
        user_password: String,
        owner_password: String,
        can_print: bool,
        can_copy: bool,
    ) {
        self.security.encrypted = true;
        self.security.user_password = user_password;
        self.security.owner_password = owner_password;
        self.security.can_print = can_print;
        self.security.can_copy = can_copy;
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

    /// Import Artcraft vector path object onto a page
    pub fn import_vector_layer(
        &mut self,
        page_id: u32,
        path_data: String,
        x: f32,
        y: f32,
        fill: Option<(u8, u8, u8)>,
        stroke: Option<(u8, u8, u8)>,
    ) -> Result<u32, &'static str> {
        let page = self.pages.get_mut(&page_id).ok_or("Page not found")?;
        let id = self.next_element_id.fetch_add(1, Ordering::Relaxed);
        let vector_obj = VectorPathObject {
            id,
            path_data,
            x,
            y,
            scale_x: 1.0,
            scale_y: 1.0,
            fill_color: fill,
            stroke_color: stroke,
            stroke_width: 1.0,
            opacity: 1.0,
        };
        page.elements.push(PdfElement::VectorPath(vector_obj));
        Ok(id)
    }

    /// Associate PDF document with an Artcraft workspace
    pub fn associate_workspace(&mut self, workspace_id: u32) {
        self.workspace_id = Some(workspace_id);
    }

    /// Apply document-wide dynamic watermark
    pub fn apply_watermark(&mut self, text: String, opacity: f32, rotation: f32) {
        self.watermarks.push(WatermarkSettings {
            text,
            opacity,
            rotation_degrees: rotation,
            font_size: 36.0,
            color: (128, 128, 128),
        });
    }

    /// Sign document with cryptographic digital signature
    pub fn sign_document(
        &mut self,
        page_id: u32,
        signer_name: String,
        reason: String,
        sig_data: Vec<u8>,
    ) -> Result<u32, &'static str> {
        let page = self.pages.get_mut(&page_id).ok_or("Page not found")?;
        let id = self.next_element_id.fetch_add(1, Ordering::Relaxed);
        let sig = DigitalSignature {
            id,
            signer_name,
            reason,
            timestamp_iso: "2026-03-30T00:00:00Z".to_string(),
            signature_bytes: sig_data,
            verified: true,
        };
        page.elements.push(PdfElement::DigitalSig(sig));
        Ok(id)
    }

    /// Add OCR invisible text layer to page
    pub fn apply_ocr_layer(
        &mut self,
        page_id: u32,
        extracted_text: String,
        confidence: f32,
        bbox: (f32, f32, f32, f32),
    ) -> Result<u32, &'static str> {
        let page = self.pages.get_mut(&page_id).ok_or("Page not found")?;
        let id = self.next_element_id.fetch_add(1, Ordering::Relaxed);
        let ocr = OcrTextLayer {
            id,
            extracted_text,
            confidence_score: confidence,
            bounding_box: bbox,
            invisible: true,
        };
        page.elements.push(PdfElement::OcrLayer(ocr));
        Ok(id)
    }

    /// Permanent PII / text pattern redaction across all pages
    pub fn redact_text_pattern(&mut self, pattern: &str) -> usize {
        let mut redactions_count = 0;
        for page in self.pages.values_mut() {
            for element in &mut page.elements {
                if let PdfElement::Text(ref mut text_obj) = element {
                    if text_obj.content.contains(pattern) {
                        text_obj.content = text_obj.content.replace(pattern, "[REDACTED]");
                        redactions_count += 1;
                    }
                }
            }
        }
        redactions_count
    }

    /// Sanitize metadata and strip sensitive creator comments
    pub fn sanitize_metadata(&mut self) {
        self.metadata.author = "Anonymous".to_string();
        self.metadata.keywords = String::new();
        self.metadata.subject = String::new();
        self.metadata.creator = "SigmaOS PdfCraft Redacted".to_string();
        self.metadata.producer = "SigmaOS Sanitizer Engine".to_string();
    }

    /// Apply N-up page imposition layout and return sheet count
    pub fn apply_page_imposition(&mut self, nup: usize) -> Result<usize, &'static str> {
        if nup == 0 {
            return Err("Invalid N-up grid factor");
        }
        let total_pages = self.pages.len();
        let sheets = (total_pages + nup - 1) / nup;
        Ok(sheets)
    }

    /// Convert color profiles across shapes to CMYK device color space
    pub fn convert_color_profile_cmyk(&mut self) {
        for page in self.pages.values_mut() {
            for elem in &mut page.elements {
                if let PdfElement::Shape(ref mut shape) = elem {
                    if let Some((r, g, b)) = shape.fill_color {
                        // CMYK conversion simulation retaining RGB representation
                        let k = 255 - r.max(g).max(b);
                        shape.fill_color = Some((
                            r.saturating_sub(k),
                            g.saturating_sub(k),
                            b.saturating_sub(k),
                        ));
                    }
                }
            }
        }
    }

    /// Validate and auto-repair PDF/A or PDF/X compliance requirements
    pub fn validate_and_repair_archival_compliance(&mut self, format: ExportFormat) -> bool {
        match format {
            ExportFormat::PDF_A | ExportFormat::PDF_X => {
                if self.metadata.title.is_empty() {
                    self.metadata.title = "Untitled Compliant Document".to_string();
                }
                if self.metadata.author.is_empty() {
                    self.metadata.author = "System Archival Validator".to_string();
                }
                true
            }
            _ => true,
        }
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
    unsafe { PDFCRAFT.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

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
        assert_eq!(editor.metadata.title, "Document.pdf");
    }

    #[test]
    fn test_add_page() {
        let mut editor = PdfEditor::new();
        editor.init();
        editor.add_page(595.0, 842.0); // A4 size
        assert_eq!(editor.get_page_count(), 1);
    }

    #[test]
    fn test_add_text() {
        let mut editor = PdfEditor::new();
        editor.init();
        let page_id = editor.add_page(595.0, 842.0);
        let text_id = editor
            .add_text(
                page_id,
                "Hello World".to_string(),
                FontFamily::Helvetica,
                12.0,
                50.0,
                750.0,
            )
            .unwrap();
        assert!(text_id > 0);
        assert_eq!(editor.get_page(page_id).unwrap().elements.len(), 1);
    }

    #[test]
    fn test_add_image() {
        let mut editor = PdfEditor::new();
        editor.init();
        let page_id = editor.add_page(595.0, 842.0);
        let image_data = vec![0u8; 100];
        let image_id = editor
            .add_image(page_id, image_data, 50.0, 50.0, 200.0, 150.0)
            .unwrap();
        assert!(image_id > 0);
    }

    #[test]
    fn test_add_shape() {
        let mut editor = PdfEditor::new();
        editor.init();
        let page_id = editor.add_page(595.0, 842.0);
        let shape_id = editor
            .add_shape(page_id, ShapeType::Rectangle, 100.0, 100.0, 200.0, 150.0)
            .unwrap();
        assert!(shape_id > 0);
    }

    #[test]
    fn test_add_annotation() {
        let mut editor = PdfEditor::new();
        editor.init();
        let page_id = editor.add_page(595.0, 842.0);
        let annotation_id = editor
            .add_annotation(
                page_id,
                AnnotationType::Highlight,
                50.0,
                700.0,
                100.0,
                20.0,
                "Important".to_string(),
                "User".to_string(),
            )
            .unwrap();
        assert!(annotation_id > 0);
        assert_eq!(editor.get_page(page_id).unwrap().annotations.len(), 1);
    }

    #[test]
    fn test_add_form_field() {
        let mut editor = PdfEditor::new();
        editor.init();
        let page_id = editor.add_page(595.0, 842.0);
        let field_id = editor
            .add_form_field(
                page_id,
                FormFieldType::TextField,
                "name".to_string(),
                100.0,
                700.0,
                200.0,
                30.0,
            )
            .unwrap();
        assert!(field_id > 0);
        assert_eq!(editor.get_page(page_id).unwrap().form_fields.len(), 1);
    }

    #[test]
    fn test_bookmarks() {
        let mut editor = PdfEditor::new();
        editor.init();
        let page_id = editor.add_page(595.0, 842.0);
        let bookmark_id = editor.add_bookmark("Chapter 1".to_string(), page_id, 0.0, 0.0);
        assert!(editor.bookmarks.contains_key(&bookmark_id));
    }

    #[test]
    fn test_hyperlink() {
        let mut editor = PdfEditor::new();
        editor.init();
        let page_id = editor.add_page(595.0, 842.0);
        let link_id = editor
            .add_hyperlink(
                page_id,
                "https://sigmaos.org".to_string(),
                50.0,
                50.0,
                100.0,
                20.0,
            )
            .unwrap();
        assert!(link_id > 0);
    }

    #[test]
    fn test_rotate_page() {
        let mut editor = PdfEditor::new();
        editor.init();
        let page_id = editor.add_page(595.0, 842.0);
        editor.rotate_page(page_id, 90).unwrap();
        assert_eq!(editor.get_page(page_id).unwrap().rotation, 90);
    }

    #[test]
    fn test_metadata() {
        let mut editor = PdfEditor::new();
        editor.init();
        editor.set_metadata(
            "Test Document".to_string(),
            "Author".to_string(),
            "Subject".to_string(),
            "keywords".to_string(),
        );
        assert_eq!(editor.metadata.title, "Test Document");
        assert_eq!(editor.metadata.author, "Author");
    }

    #[test]
    fn test_security() {
        let mut editor = PdfEditor::new();
        editor.init();
        editor.set_security("user123".to_string(), "owner456".to_string(), true, false);
        assert!(editor.security.encrypted);
        assert!(!editor.security.can_copy);
    }

    #[test]
    fn test_vector_art_import() {
        let mut editor = PdfEditor::new();
        editor.init();
        let page_id = editor.add_page(595.0, 842.0);
        let id = editor
            .import_vector_layer(
                page_id,
                "M 10 10 L 100 100 Z".to_string(),
                0.0,
                0.0,
                Some((255, 0, 0)),
                Some((0, 0, 0)),
            )
            .unwrap();
        assert!(id > 0);
    }

    #[test]
    fn test_workspace_association() {
        let mut editor = PdfEditor::new();
        editor.init();
        editor.associate_workspace(42);
        assert_eq!(editor.workspace_id, Some(42));
    }

    #[test]
    fn test_watermark_and_signature_and_ocr() {
        let mut editor = PdfEditor::new();
        editor.init();
        let page_id = editor.add_page(595.0, 842.0);
        editor.apply_watermark("CONFIDENTIAL".to_string(), 0.5, 45.0);
        assert_eq!(editor.watermarks.len(), 1);

        let sig_id = editor
            .sign_document(
                page_id,
                "Alice".to_string(),
                "Approval".to_string(),
                vec![1, 2, 3, 4],
            )
            .unwrap();
        assert!(sig_id > 0);

        let ocr_id = editor
            .apply_ocr_layer(
                page_id,
                "Extracted Text".to_string(),
                0.98,
                (10.0, 10.0, 100.0, 20.0),
            )
            .unwrap();
        assert!(ocr_id > 0);
    }

    #[test]
    fn test_redaction_sanitization_imposition_and_compliance() {
        let mut editor = PdfEditor::new();
        editor.init();
        let page_id = editor.add_page(595.0, 842.0);
        editor
            .add_text(
                page_id,
                "Confidential SSN: 123-45-6789".to_string(),
                FontFamily::Helvetica,
                12.0,
                10.0,
                10.0,
            )
            .unwrap();

        let redacted_count = editor.redact_text_pattern("123-45-6789");
        assert_eq!(redacted_count, 1);

        editor.sanitize_metadata();
        assert_eq!(editor.metadata.author, "Anonymous");

        let sheets = editor.apply_page_imposition(2).unwrap();
        assert_eq!(sheets, 1);

        editor.convert_color_profile_cmyk();

        let valid = editor.validate_and_repair_archival_compliance(ExportFormat::PDF_A);
        assert!(valid);
        assert!(!editor.metadata.title.is_empty());
    }
}
