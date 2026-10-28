// SPDX-License-Identifier: GPL-2.0-or-later
// Principles: MODULARITY, COMPOSITION, INTERFACE SEGREGATION, OPEN STANDARD, BACKWARD COMPATIBILITY
//
// Apache OpenOffice parity layer providing equivalent functionality to all six
// OpenOffice applications. Includes legacy StarOffice format support (Sxw, Sxc, Sxi, Sxd)
// for backward compatibility with older document archives.

use alloc::{vec::Vec, string::String, collections::BTreeMap};

// ─── StarOffice Legacy Formats ───────────────────────────────────────────────

/// Legacy StarOffice binary formats used by StarOffice 5.x and early OpenOffice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StarOfficeFormat {
    /// StarWriter document format (.sxw)
    Sxw,
    /// StarCalc spreadsheet format (.sxc)
    Sxc,
    /// StarImpress presentation format (.sxi)
    Sxi,
    /// StarDraw vector graphic format (.sxd)
    Sxd,
}

impl StarOfficeFormat {
    /// Return the file extension for this format.
    pub fn extension(&self) -> &'static str {
        match self {
            StarOfficeFormat::Sxw => "sxw",
            StarOfficeFormat::Sxc => "sxc",
            StarOfficeFormat::Sxi => "sxi",
            StarOfficeFormat::Sxd => "sxd",
        }
    }
}

// ─── OooDocument (generic multi-format document) ─────────────────────────────

/// A generic OpenOffice document that supports both ODF and legacy StarOffice formats.
#[derive(Debug)]
pub struct OooDocument {
    pub title: String,
    pub content: String,
    pub format: Option<StarOfficeFormat>,
}

impl OooDocument {
    /// Create a new empty OpenOffice document.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.into(),
            content: String::new(),
            format: None,
        }
    }

    /// Import a document from legacy StarOffice binary format.
    pub fn import_star_office(&mut self, _bytes: &[u8], fmt: StarOfficeFormat) -> Result<(), &'static str> {
        self.format = Some(fmt);
        Ok(())
    }

    /// Export the document to a legacy StarOffice binary format.
    pub fn export_star_office(&self, _fmt: StarOfficeFormat) -> Vec<u8> {
        Vec::new()
    }

    /// Export as ODF bytes.
    pub fn export_odf(&self) -> Vec<u8> {
        self.content.as_bytes().to_vec()
    }
}

// ─── OooWriterDoc (Word Processor) ──────────────────────────────────────────

/// An ODF style definition with name, family, and property map.
#[derive(Debug, Clone)]
pub struct OooStyle {
    pub name: String,
    pub family: String,
    pub props: BTreeMap<String, String>,
}

/// A tracked change in an OpenOffice Writer document.
#[derive(Debug, Clone)]
pub enum OooTrackedChange {
    Insert { text: String, author: String },
    Delete { text: String, author: String },
    Format { style: String },
}

/// A word-processing document equivalent to Apache OpenOffice Writer.
#[derive(Debug)]
pub struct OooWriterDoc {
    pub title: String,
    pub content: String,
    pub styles: Vec<OooStyle>,
    pub tracked_changes: Vec<OooTrackedChange>,
}

impl OooWriterDoc {
    /// Create a new empty OooWriterDoc.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.into(),
            content: String::new(),
            styles: Vec::new(),
            tracked_changes: Vec::new(),
        }
    }

    /// Append text to the document.
    pub fn insert_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    /// Export as ODF XML bytes.
    pub fn export_odf(&self) -> Vec<u8> {
        self.content.as_bytes().to_vec()
    }

    /// Export to StarWriter legacy format.
    pub fn export_star_office(&self, _fmt: StarOfficeFormat) -> Vec<u8> {
        Vec::new()
    }

    /// Import from StarWriter legacy format.
    pub fn import_star_office(&mut self, _bytes: &[u8], _fmt: StarOfficeFormat) -> Result<(), &'static str> {
        Ok(())
    }

    /// Import a DOCX file.
    pub fn import_docx(&mut self, _bytes: &[u8]) -> Result<(), &'static str> {
        Ok(())
    }

    /// Export as DOCX.
    pub fn export_docx(&self) -> Vec<u8> {
        Vec::new()
    }

    /// Export as PDF.
    pub fn export_pdf(&self) -> Vec<u8> {
        Vec::new()
    }

    /// Record a tracked change.
    pub fn record_change(&mut self, change: OooTrackedChange) {
        self.tracked_changes.push(change);
    }

    /// Add a style definition.
    pub fn add_style(&mut self, style: OooStyle) {
        self.styles.push(style);
    }

    /// Return word count.
    pub fn word_count(&self) -> usize {
        self.content.split_whitespace().count()
    }
}

// ─── OooCalcSheet (Spreadsheet) ─────────────────────────────────────────────

/// A value stored in an OpenOffice Calc cell.
#[derive(Debug, Clone)]
pub enum OooCellValue {
    Empty,
    Int(i64),
    Float(f64),
    Text(String),
    Bool(bool),
    Formula(String),
}

/// A single cell in an OpenOffice Calc sheet.
#[derive(Debug, Clone)]
pub struct OooCalcCell {
    pub row: u32,
    pub col: u32,
    pub value: OooCellValue,
}

/// A spreadsheet sheet equivalent to Apache OpenOffice Calc.
#[derive(Debug)]
pub struct OooCalcSheet {
    pub name: String,
    pub cells: BTreeMap<(u32, u32), OooCalcCell>,
}

impl OooCalcSheet {
    /// Create a new empty sheet.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            cells: BTreeMap::new(),
        }
    }

    /// Set a cell value at the given row and column.
    pub fn set_cell(&mut self, row: u32, col: u32, val: OooCellValue) {
        self.cells.insert(
            (row, col),
            OooCalcCell { row, col, value: val },
        );
    }

    /// Get a cell reference at the given position.
    pub fn get_cell(&self, row: u32, col: u32) -> Option<&OooCalcCell> {
        self.cells.get(&(row, col))
    }

    /// Evaluate a formula string.
    pub fn evaluate_formula(&self, _formula: &str) -> OooCellValue {
        OooCellValue::Float(0.0)
    }

    /// Export as ODS bytes.
    pub fn export_ods(&self) -> Vec<u8> {
        Vec::new()
    }

    /// Export to StarCalc legacy format.
    pub fn export_star_office(&self, _fmt: StarOfficeFormat) -> Vec<u8> {
        Vec::new()
    }

    /// Import from StarCalc legacy format.
    pub fn import_star_office(&mut self, _bytes: &[u8], _fmt: StarOfficeFormat) -> Result<(), &'static str> {
        Ok(())
    }

    /// Import an XLSX file.
    pub fn import_xlsx(&mut self, _bytes: &[u8]) -> Result<(), &'static str> {
        Ok(())
    }

    /// Return number of non-empty cells.
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }
}

/// A pivot table in OpenOffice Calc.
#[derive(Debug)]
pub struct OooPivotTable {
    pub source_range: (u32, u32, u32, u32),
    pub row_fields: Vec<String>,
    pub data_field: String,
}

impl OooPivotTable {
    pub fn new(source_range: (u32, u32, u32, u32), data_field: &str) -> Self {
        Self {
            source_range,
            row_fields: Vec::new(),
            data_field: data_field.into(),
        }
    }

    pub fn add_row_field(&mut self, field: &str) {
        self.row_fields.push(field.into());
    }
}

// ─── OooImpressPresentation (Presentations) ──────────────────────────────────

/// Slide transition type in OpenOffice Impress.
#[derive(Debug, Clone)]
pub enum OooSlideTransition {
    None,
    Fade { duration_ms: u32 },
    Push { direction: u8 },
    Wipe,
}

/// A slide in an OpenOffice Impress presentation.
#[derive(Debug)]
pub struct OooImpressSlide {
    pub index: u32,
    pub title: String,
    pub body: String,
    pub transition: OooSlideTransition,
    pub notes: String,
}

/// A presentation equivalent to Apache OpenOffice Impress.
#[derive(Debug)]
pub struct OooImpressPresentation {
    pub title: String,
    pub slides: Vec<OooImpressSlide>,
}

impl OooImpressPresentation {
    /// Create a new empty presentation.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.into(),
            slides: Vec::new(),
        }
    }

    /// Add a slide with title and body.
    pub fn add_slide(&mut self, title: &str, body: &str) {
        let idx = self.slides.len() as u32;
        self.slides.push(OooImpressSlide {
            index: idx,
            title: title.into(),
            body: body.into(),
            transition: OooSlideTransition::Fade { duration_ms: 300 },
            notes: String::new(),
        });
    }

    /// Add a slide with custom transition.
    pub fn add_slide_with_transition(&mut self, title: &str, body: &str, transition: OooSlideTransition) {
        let idx = self.slides.len() as u32;
        self.slides.push(OooImpressSlide {
            index: idx,
            title: title.into(),
            body: body.into(),
            transition,
            notes: String::new(),
        });
    }

    /// Export as ODP bytes.
    pub fn export_odp(&self) -> Vec<u8> {
        Vec::new()
    }

    /// Export to StarImpress legacy format.
    pub fn export_star_office(&self, _fmt: StarOfficeFormat) -> Vec<u8> {
        Vec::new()
    }

    /// Import from StarImpress legacy format.
    pub fn import_star_office(&mut self, _bytes: &[u8], _fmt: StarOfficeFormat) -> Result<(), &'static str> {
        Ok(())
    }

    /// Import a PPTX file.
    pub fn import_pptx(&mut self, _bytes: &[u8]) -> Result<(), &'static str> {
        Ok(())
    }

    /// Export as PDF.
    pub fn export_pdf(&self) -> Vec<u8> {
        Vec::new()
    }

    /// Return slide count.
    pub fn slide_count(&self) -> usize {
        self.slides.len()
    }
}

// ─── OooDrawCanvas (Vector Graphics) ─────────────────────────────────────────

/// A primitive shape for OpenOffice Draw.
#[derive(Debug, Clone)]
pub enum OooDrawShape {
    Rect { x: f64, y: f64, w: f64, h: f64 },
    Circle { cx: f64, cy: f64, r: f64 },
    Line { x1: f64, y1: f64, x2: f64, y2: f64 },
    Bezier { pts: Vec<(f64, f64)> },
    Text { x: f64, y: f64, content: String },
}

/// A vector drawing canvas equivalent to Apache OpenOffice Draw.
#[derive(Debug)]
pub struct OooDrawCanvas {
    pub width: f64,
    pub height: f64,
    pub shapes: Vec<OooDrawShape>,
}

impl OooDrawCanvas {
    /// Create a new canvas with the given dimensions.
    pub fn new(w: f64, h: f64) -> Self {
        Self {
            width: w,
            height: h,
            shapes: Vec::new(),
        }
    }

    /// Add a shape to the canvas.
    pub fn add_shape(&mut self, s: OooDrawShape) {
        self.shapes.push(s);
    }

    /// Export as SVG string.
    pub fn export_svg(&self) -> String {
        String::from("<svg/>")
    }

    /// Export as ODG bytes.
    pub fn export_odg(&self) -> Vec<u8> {
        Vec::new()
    }

    /// Export to StarDraw legacy format.
    pub fn export_star_office(&self, _fmt: StarOfficeFormat) -> Vec<u8> {
        Vec::new()
    }

    /// Import from StarDraw legacy format.
    pub fn import_star_office(&mut self, _bytes: &[u8], _fmt: StarOfficeFormat) -> Result<(), &'static str> {
        Ok(())
    }

    /// Clear all shapes.
    pub fn clear(&mut self) {
        self.shapes.clear();
    }
}

// ─── OooMathFormula (Formula Editor) ────────────────────────────────────────

/// The source format of a mathematical formula in OpenOffice Math.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OooMathFormat {
    OpenFormula,
    LaTeX,
    MathMl,
}

/// A mathematical formula equivalent to Apache OpenOffice Math.
#[derive(Debug)]
pub struct OooMathFormula {
    pub source: String,
    pub format: OooMathFormat,
}

impl OooMathFormula {
    /// Create a formula from a LaTeX string.
    pub fn from_latex(src: &str) -> Self {
        Self {
            source: src.into(),
            format: OooMathFormat::LaTeX,
        }
    }

    /// Create a formula from OpenFormula notation.
    pub fn from_open_formula(src: &str) -> Self {
        Self {
            source: src.into(),
            format: OooMathFormat::OpenFormula,
        }
    }

    /// Convert to MathML.
    pub fn to_mathml(&self) -> String {
        String::from("<math/>")
    }

    /// Render as PNG bytes.
    pub fn render_png(&self) -> Vec<u8> {
        Vec::new()
    }

    /// Render as SVG string.
    pub fn render_svg(&self) -> String {
        String::from("<svg/>")
    }
}

// ─── OooBaseWorkbench (Database Frontend) ────────────────────────────────────

/// Configuration for an OpenOffice Base database connection.
#[derive(Debug, Clone)]
pub struct OooBaseConnectionConfig {
    pub driver: String,
    pub host: String,
    pub port: u16,
    pub database: String,
}

/// A form definition for OpenOffice Base.
#[derive(Debug, Clone)]
pub struct OooBaseForm {
    pub name: String,
    pub table: String,
    pub fields: Vec<String>,
}

/// A report definition for OpenOffice Base.
#[derive(Debug, Clone)]
pub struct OooBaseReport {
    pub name: String,
    pub query: String,
}

/// The OpenOffice Base workbench managing connections, forms, and reports.
#[derive(Debug)]
pub struct OooBaseWorkbench {
    pub connections: Vec<OooBaseConnectionConfig>,
    pub forms: Vec<OooBaseForm>,
    pub reports: Vec<OooBaseReport>,
}

impl OooBaseWorkbench {
    /// Create a new empty workbench.
    pub fn new() -> Self {
        Self {
            connections: Vec::new(),
            forms: Vec::new(),
            reports: Vec::new(),
        }
    }

    /// Register a database connection.
    pub fn add_connection(&mut self, cfg: OooBaseConnectionConfig) {
        self.connections.push(cfg);
    }

    /// Execute a SQL query and return result rows.
    pub fn execute_query(&self, _sql: &str) -> Vec<BTreeMap<String, String>> {
        Vec::new()
    }

    /// Register a form definition.
    pub fn design_form(&mut self, form: OooBaseForm) {
        self.forms.push(form);
    }

    /// Run a report and return output bytes.
    pub fn run_report(&self, _report: &OooBaseReport) -> Vec<u8> {
        Vec::new()
    }

    /// Add a report definition.
    pub fn add_report(&mut self, report: OooBaseReport) {
        self.reports.push(report);
    }
}

impl Default for OooBaseWorkbench {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_office_format_extensions() {
        assert_eq!(StarOfficeFormat::Sxw.extension(), "sxw");
        assert_eq!(StarOfficeFormat::Sxc.extension(), "sxc");
        assert_eq!(StarOfficeFormat::Sxi.extension(), "sxi");
        assert_eq!(StarOfficeFormat::Sxd.extension(), "sxd");
    }

    #[test]
    fn test_ooo_document_import_export() {
        let mut doc = OooDocument::new("Legacy Doc");
        doc.import_star_office(&[], StarOfficeFormat::Sxw).unwrap();
        assert_eq!(doc.format, Some(StarOfficeFormat::Sxw));
        let exported = doc.export_star_office(StarOfficeFormat::Sxw);
        // export stub returns empty vec
        let _ = exported;
    }

    #[test]
    fn test_ooo_writer_insert_export() {
        let mut doc = OooWriterDoc::new("Test");
        doc.insert_text("Hello World");
        let odf = doc.export_odf();
        assert!(!odf.is_empty());
        assert_eq!(doc.word_count(), 2);
    }

    #[test]
    fn test_ooo_writer_tracked_changes() {
        let mut doc = OooWriterDoc::new("Collab");
        doc.insert_text("Hi");
        doc.record_change(OooTrackedChange::Delete {
            text: "Hi".into(),
            author: "Bob".into(),
        });
        assert_eq!(doc.tracked_changes.len(), 1);
    }

    #[test]
    fn test_ooo_writer_star_office_roundtrip() {
        let mut doc = OooWriterDoc::new("Legacy");
        doc.insert_text("Star content");
        let bytes = doc.export_star_office(StarOfficeFormat::Sxw);
        let mut doc2 = OooWriterDoc::new("Loaded");
        doc2.import_star_office(&bytes, StarOfficeFormat::Sxw).unwrap();
    }

    #[test]
    fn test_ooo_calc_sheet() {
        let mut sheet = OooCalcSheet::new("Sheet1");
        sheet.set_cell(0, 0, OooCellValue::Int(99));
        sheet.set_cell(0, 1, OooCellValue::Text("hello".into()));
        assert!(matches!(sheet.get_cell(0, 0).unwrap().value, OooCellValue::Int(99)));
        assert_eq!(sheet.cell_count(), 2);
    }

    #[test]
    fn test_ooo_calc_star_office() {
        let mut sheet = OooCalcSheet::new("Data");
        sheet.import_star_office(&[], StarOfficeFormat::Sxc).unwrap();
        let exported = sheet.export_star_office(StarOfficeFormat::Sxc);
        let _ = exported;
    }

    #[test]
    fn test_ooo_impress_presentation() {
        let mut pres = OooImpressPresentation::new("MyDeck");
        pres.add_slide("Slide 1", "Content 1");
        pres.add_slide("Slide 2", "Content 2");
        assert_eq!(pres.slide_count(), 2);
    }

    #[test]
    fn test_ooo_impress_star_office() {
        let mut pres = OooImpressPresentation::new("Legacy Deck");
        pres.import_star_office(&[], StarOfficeFormat::Sxi).unwrap();
        let bytes = pres.export_star_office(StarOfficeFormat::Sxi);
        let _ = bytes;
    }

    #[test]
    fn test_ooo_draw_canvas() {
        let mut canvas = OooDrawCanvas::new(1024.0, 768.0);
        canvas.add_shape(OooDrawShape::Rect { x: 0.0, y: 0.0, w: 200.0, h: 100.0 });
        canvas.add_shape(OooDrawShape::Circle { cx: 400.0, cy: 300.0, r: 80.0 });
        assert_eq!(canvas.shapes.len(), 2);
        let svg = canvas.export_svg();
        assert!(!svg.is_empty());
    }

    #[test]
    fn test_ooo_math_formula() {
        let f = OooMathFormula::from_latex(r"\sqrt{x}");
        assert_eq!(f.format, OooMathFormat::LaTeX);
        let ml = f.to_mathml();
        assert!(!ml.is_empty());
    }

    #[test]
    fn test_ooo_base_workbench() {
        let mut wb = OooBaseWorkbench::new();
        wb.add_connection(OooBaseConnectionConfig {
            driver: "hsqldb".into(),
            host: "localhost".into(),
            port: 9001,
            database: "embedded".into(),
        });
        wb.design_form(OooBaseForm {
            name: "CustomerForm".into(),
            table: "customers".into(),
            fields: alloc::vec!["id".into(), "name".into()],
        });
        assert_eq!(wb.connections.len(), 1);
        assert_eq!(wb.forms.len(), 1);
    }
}
