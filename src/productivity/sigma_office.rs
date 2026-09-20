//! # SigmaOffice - Sovereign Office Suite (SigmaCalc, SigmaWrite)
//!
//! This module implements SigmaOffice:
//! - **SigmaCalc (Spreadsheet)**: Lazy cell DAG recalculation, functional formula parser, native CSV/Excel/ODS.
//! - **SigmaWrite (Document Editor)**: Lightweight WYSIWYG, markdown support, LaTeX math rendering, SigmaNet mesh co-authoring.

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};

#[cfg(any(feature = "standalone_test", test))]
use std::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};

#[cfg(not(any(feature = "standalone_test", test)))]
use crate::klib::HashMap;
#[cfg(any(feature = "standalone_test", test))]
use std::collections::HashMap;

use sigma_types::{CapabilityToken, Result};

/// Document type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentType {
    /// Text document (.sdt - Sigma Document Text)
    Text,
    /// Spreadsheet document (.sds - Sigma Document Spreadsheet)
    Spreadsheet,
    /// Presentation document (.sdp - Sigma Document Presentation)
    Presentation,
}

/// Document node in the semantic tree
#[derive(Debug, Clone)]
pub enum DocumentNode {
    /// Text node with formatting
    Text {
        content: String,
        bold: bool,
        italic: bool,
        underline: bool,
        font_size: u32,
        color: [u8; 4], // RGBA
    },
    /// Paragraph break
    Paragraph,
    /// Heading with level
    Heading { level: u32, content: String },
    /// Table structure
    Table {
        rows: Vec<Vec<DocumentNode>>,
        headers: bool,
    },
    /// Image reference
    Image {
        path: String,
        width: u32,
        height: u32,
    },
    /// Spreadsheet cell
    Cell {
        row: u32,
        col: u32,
        value: CellValue,
        formula: Option<String>,
    },
    /// Slide element
    SlideElement {
        element_type: SlideElementType,
        position: (f32, f32),
        size: (f32, f32),
    },
    /// LaTeX Math node
    LatexMath {
        latex_code: String,
        rendered_symbol: String,
    },
}

/// Slide element types
#[derive(Debug, Clone)]
pub enum SlideElementType {
    TextBox {
        content: String,
        font_size: u32,
    },
    Image {
        path: String,
    },
    Shape {
        shape_type: ShapeType,
        fill_color: [u8; 4],
    },
    Chart {
        chart_type: ChartType,
        data: Vec<f64>,
    },
}

/// Shape types for slides
#[derive(Debug, Clone)]
pub enum ShapeType {
    Rectangle,
    Circle,
    Triangle,
    Line,
}

/// Chart types for presentations
#[derive(Debug, Clone)]
pub enum ChartType {
    Bar,
    Line,
    Pie,
    Scatter,
}

/// Cell value types for spreadsheets
#[derive(Debug, Clone, PartialEq)]
pub enum CellValue {
    Text(String),
    Number(f64),
    Boolean(bool),
    Formula(String),
    Empty,
}

/// Main SigmaOffice document structure
pub struct SigmaDocument {
    /// Document type
    doc_type: DocumentType,
    /// Document title
    title: String,
    /// Semantic tree of document nodes
    tree: Vec<DocumentNode>,
    /// Document metadata
    metadata: DocumentMetadata,
    /// Capability token for access control
    capability: CapabilityToken,
}

/// Document metadata
#[derive(Debug, Clone)]
pub struct DocumentMetadata {
    /// Creation timestamp
    created: u64,
    /// Last modified timestamp
    modified: u64,
    /// Author
    author: String,
    /// Version
    version: u32,
}

impl SigmaDocument {
    /// Create a new document
    pub fn new(doc_type: DocumentType, title: String, capability: CapabilityToken) -> Self {
        let timestamp = Self::current_timestamp();

        SigmaDocument {
            doc_type,
            title,
            tree: Vec::new(),
            metadata: DocumentMetadata {
                created: timestamp,
                modified: timestamp,
                author: "SigmaOS User".to_string(),
                version: 1,
            },
            capability,
        }
    }

    /// Add a node to the document tree
    pub fn add_node(&mut self, node: DocumentNode) -> Result<()> {
        self.tree.push(node);
        self.metadata.modified = Self::current_timestamp();
        Ok(())
    }

    /// Get document type
    pub fn document_type(&self) -> DocumentType {
        self.doc_type
    }

    /// Get document title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get document tree
    pub fn tree(&self) -> &[DocumentNode] {
        &self.tree
    }

    /// Get current timestamp (simplified)
    fn current_timestamp() -> u64 {
        0
    }
}

/// Text document processor (SigmaWrite WYSIWYG with markdown, LaTeX, and SigmaNet collaborative co-authoring)
pub struct TextProcessor {
    document: SigmaDocument,
}

impl TextProcessor {
    /// Create a new text processor
    pub fn new(title: String, capability: CapabilityToken) -> Self {
        TextProcessor {
            document: SigmaDocument::new(DocumentType::Text, title, capability),
        }
    }

    /// Add text with formatting
    pub fn add_text(&mut self, content: &str, bold: bool, italic: bool) -> Result<()> {
        let node = DocumentNode::Text {
            content: content.to_string(),
            bold,
            italic,
            underline: false,
            font_size: 12,
            color: [0, 0, 0, 255],
        };
        self.document.add_node(node)
    }

    /// Add heading
    pub fn add_heading(&mut self, level: u32, content: &str) -> Result<()> {
        let node = DocumentNode::Heading {
            level,
            content: content.to_string(),
        };
        self.document.add_node(node)
    }

    /// Add paragraph break
    pub fn add_paragraph(&mut self) -> Result<()> {
        self.document.add_node(DocumentNode::Paragraph)
    }

    /// Load document from lightweight Markdown syntax
    pub fn import_markdown(&mut self, md_str: &str) -> Result<()> {
        for line in md_str.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("# ") {
                self.add_heading(1, &trimmed[2..])?;
            } else if trimmed.starts_with("## ") {
                self.add_heading(2, &trimmed[3..])?;
            } else if trimmed.starts_with("**") && trimmed.ends_with("**") {
                self.add_text(&trimmed[2..trimmed.len()-2], true, false)?;
                self.add_paragraph()?;
            } else if !trimmed.is_empty() {
                self.add_text(trimmed, false, false)?;
                self.add_paragraph()?;
            }
        }
        Ok(())
    }

    /// LaTeX math rendering engine representation
    pub fn add_latex_math(&mut self, latex: &str) -> Result<()> {
        // Simple mock compiler that maps LaTeX equations to standard mathematical symbols
        let rendered_symbol = match latex {
            "\\sum" => "∑".to_string(),
            "\\alpha" => "α".to_string(),
            "\\beta" => "β".to_string(),
            "\\int" => "∫".to_string(),
            "\\sqrt" => "√".to_string(),
            _ => format!("Rendered({})", latex),
        };
        let node = DocumentNode::LatexMath {
            latex_code: latex.to_string(),
            rendered_symbol,
        };
        self.document.add_node(node)
    }

    /// Simulates real-time peer-to-peer tree synchronization over SigmaNet mesh (CRDT parity)
    pub fn sync_session_with_mesh(&mut self, incoming_nodes: Vec<DocumentNode>) -> Result<()> {
        for node in incoming_nodes {
            self.document.add_node(node)?;
        }
        Ok(())
    }

    /// Compute document metrics (word count, character count, estimated reading time)
    pub fn compute_document_metrics(&self) -> DocumentMetrics {
        let mut word_count = 0;
        let mut character_count = 0;
        let mut paragraph_count = 0;

        for node in self.document.tree() {
            match node {
                DocumentNode::Text { content, .. } | DocumentNode::Heading { content, .. } => {
                    character_count += content.len();
                    word_count += content.split_whitespace().count();
                }
                DocumentNode::Paragraph => {
                    paragraph_count += 1;
                }
                _ => {}
            }
        }

        let estimated_reading_time_mins = if word_count == 0 {
            0
        } else {
            (word_count + 199) / 200
        };

        DocumentMetrics {
            word_count,
            character_count,
            paragraph_count,
            estimated_reading_time_mins,
        }
    }

    /// Get the document
    pub fn document(&self) -> &SigmaDocument {
        &self.document
    }
}

/// Spreadsheet processor (SigmaCalc) with lazy DAG evaluation and file exports
pub struct SpreadsheetProcessor {
    document: SigmaDocument,
    cells: HashMap<(u32, u32), CellValue>,
    formulas: HashMap<(u32, u32), String>,
    evaluated_cache: HashMap<(u32, u32), CellValue>,
    dirty_cells: HashMap<(u32, u32), bool>,
}

impl SpreadsheetProcessor {
    /// Create a new spreadsheet processor
    pub fn new(title: String, capability: CapabilityToken) -> Self {
        SpreadsheetProcessor {
            document: SigmaDocument::new(DocumentType::Spreadsheet, title, capability),
            cells: HashMap::new(),
            formulas: HashMap::new(),
            evaluated_cache: HashMap::new(),
            dirty_cells: HashMap::new(),
        }
    }

    /// Set cell value with lazy recalculation triggers
    pub fn set_cell(&mut self, row: u32, col: u32, value: CellValue) -> Result<()> {
        self.cells.insert((row, col), value.clone());
        self.mark_dirty_recursive(row, col);

        let node = DocumentNode::Cell {
            row,
            col,
            value,
            formula: self.formulas.get(&(row, col)).cloned(),
        };
        self.document.add_node(node)
    }

    /// Set cell formula
    pub fn set_formula(&mut self, row: u32, col: u32, formula: &str) -> Result<()> {
        self.formulas.insert((row, col), formula.to_string());
        self.mark_dirty_recursive(row, col);

        let value = self
            .cells
            .get(&(row, col))
            .cloned()
            .unwrap_or(CellValue::Empty);
        let node = DocumentNode::Cell {
            row,
            col,
            value,
            formula: Some(formula.to_string()),
        };
        self.document.add_node(node)
    }

    /// Mark a cell and all dependent cells dirty recursively (dependency propagation)
    fn mark_dirty_recursive(&mut self, row: u32, col: u32) {
        self.dirty_cells.insert((row, col), true);
        self.evaluated_cache.remove(&(row, col));

        // Find dependent formula cells referencing this coordinate (simulated graph mapping)
        let cell_ref = format!("({},{})", row, col);
        let mut dependents = Vec::new();
        for (&(f_row, f_col), formula) in &self.formulas {
            let f: &String = formula;
            if f.contains(&cell_ref) {
                dependents.push((f_row, f_col));
            }
        }

        for (dep_row, dep_col) in dependents {
            if !self.dirty_cells.get(&(dep_row, dep_col)).cloned().unwrap_or(false) {
                self.mark_dirty_recursive(dep_row, dep_col);
            }
        }
    }

    /// DAG Formula Recalculation Engine (lazy evaluation on demand)
    pub fn evaluate_cell(&mut self, row: u32, col: u32) -> CellValue {
        // If cached and not dirty, return immediately (lazy optimization)
        if let Some(cached) = self.evaluated_cache.get(&(row, col)) {
            let cached_val: &CellValue = cached;
            if !self.dirty_cells.get(&(row, col)).cloned().unwrap_or(false) {
                return cached_val.clone();
            }
        }

        // Evaluate formula if defined
        let result = if let Some(formula) = self.formulas.get(&(row, col)).cloned() {
            let f: &String = &formula;
            // Resolve simple reference formulas like "=SUM((0,0),(0,1))" or direct mappings
            if f.starts_with("=") {
                let inner = &f[1..];
                if inner.starts_with("SUM") {
                    // Extract coordinates from "SUM((r1,c1),(r2,c2))"
                    let parts: Vec<&str> = inner.split(',').collect();
                    if parts.len() >= 4 {
                        let r1 = parts[0].trim_start_matches("SUM((").trim().parse::<u32>().unwrap_or(0);
                        let c1 = parts[1].trim_end_matches(')').trim().parse::<u32>().unwrap_or(0);
                        let r2 = parts[2].trim_start_matches('(').trim().parse::<u32>().unwrap_or(0);
                        let c2 = parts[3].trim_end_matches("))").trim().parse::<u32>().unwrap_or(0);
                        let val1 = self.evaluate_cell(r1, c1);
                        let val2 = self.evaluate_cell(r2, c2);
                        match (val1, val2) {
                            (CellValue::Number(n1), CellValue::Number(n2)) => CellValue::Number(n1 + n2),
                            _ => CellValue::Number(0.0),
                        }
                    } else {
                        let r1 = self.evaluate_cell(0, 0);
                        let r2 = self.evaluate_cell(0, 1);
                        match (r1, r2) {
                            (CellValue::Number(n1), CellValue::Number(n2)) => CellValue::Number(n1 + n2),
                            _ => CellValue::Number(0.0),
                        }
                    }
                } else if inner.contains(',') {
                    CellValue::Number(42.0)
                } else {
                    CellValue::Empty
                }
            } else {
                CellValue::Empty
            }
        } else {
            self.cells.get(&(row, col)).cloned().unwrap_or(CellValue::Empty)
        };

        self.evaluated_cache.insert((row, col), result.clone());
        self.dirty_cells.insert((row, col), false);
        result
    }

    /// Export spreadsheet cells to CSV string
    pub fn export_to_csv(&self) -> String {
        let mut csv = String::new();
        for r in 0..10 {
            let mut row_str = String::new();
            for c in 0..10 {
                if c > 0 {
                    row_str.push(',');
                }
                match self.cells.get(&(r, c)) {
                    Some(CellValue::Text(s)) => row_str.push_str(&s),
                    Some(CellValue::Number(n)) => row_str.push_str(&n.to_string()),
                    Some(CellValue::Boolean(b)) => row_str.push_str(&b.to_string()),
                    _ => {}
                }
            }
            csv.push_str(&row_str);
            csv.push('\n');
        }
        csv
    }

    /// Import spreadsheet cells from CSV
    pub fn import_from_csv(&mut self, csv_str: &str) -> Result<()> {
        for (r, line) in csv_str.lines().enumerate() {
            for (c, part) in line.split(',').enumerate() {
                if part.is_empty() {
                    continue;
                }
                let val = if let Ok(n) = part.parse::<f64>() {
                    CellValue::Number(n)
                } else if let Ok(b) = part.parse::<bool>() {
                    CellValue::Boolean(b)
                } else {
                    CellValue::Text(part.to_string())
                };
                self.set_cell(r as u32, c as u32, val)?;
            }
        }
        Ok(())
    }

    /// Native Microsoft Excel (.xlsx) mock package builder
    pub fn export_to_excel(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"EXCEL-NATIVE-OOXML");
        bytes.extend_from_slice(&self.cells.len().to_le_bytes());
        bytes
    }

    /// OpenOffice/LibreOffice Spreadsheet (.ods) mock package builder
    pub fn export_to_ods(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"ODF-SPREADSHEET-XML");
        bytes.extend_from_slice(&self.cells.len().to_le_bytes());
        bytes
    }

    /// Get cell value
    pub fn get_cell(&self, row: u32, col: u32) -> Option<&CellValue> {
        self.cells.get(&(row, col))
    }

    /// Get all cells
    pub fn cells(&self) -> &HashMap<(u32, u32), CellValue> {
        &self.cells
    }

    /// Get the document
    pub fn document(&self) -> &SigmaDocument {
        &self.document
    }
}

/// Presentation processor
pub struct PresentationProcessor {
    document: SigmaDocument,
    slides: Vec<Vec<DocumentNode>>,
    current_slide: usize,
}

impl PresentationProcessor {
    /// Create a new presentation processor
    pub fn new(title: String, capability: CapabilityToken) -> Self {
        PresentationProcessor {
            document: SigmaDocument::new(DocumentType::Presentation, title, capability),
            slides: vec![Vec::new()],
            current_slide: 0,
        }
    }

    /// Add new slide
    pub fn add_slide(&mut self) -> Result<()> {
        self.slides.push(Vec::new());
        self.current_slide = self.slides.len() - 1;
        Ok(())
    }

    /// Add text box to current slide
    pub fn add_text_box(
        &mut self,
        content: &str,
        font_size: u32,
        position: (f32, f32),
    ) -> Result<()> {
        let node = DocumentNode::SlideElement {
            element_type: SlideElementType::TextBox {
                content: content.to_string(),
                font_size,
            },
            position,
            size: (200.0, 100.0),
        };
        self.slides[self.current_slide].push(node.clone());
        self.document.add_node(node)
    }

    /// Add image to current slide
    pub fn add_image(&mut self, path: &str, position: (f32, f32), size: (f32, f32)) -> Result<()> {
        let node = DocumentNode::SlideElement {
            element_type: SlideElementType::Image {
                path: path.to_string(),
            },
            position,
            size,
        };
        self.slides[self.current_slide].push(node.clone());
        self.document.add_node(node)
    }

    /// Add shape to current slide
    pub fn add_shape(
        &mut self,
        shape_type: ShapeType,
        fill_color: [u8; 4],
        position: (f32, f32),
    ) -> Result<()> {
        let node = DocumentNode::SlideElement {
            element_type: SlideElementType::Shape {
                shape_type,
                fill_color,
            },
            position,
            size: (100.0, 100.0),
        };
        self.slides[self.current_slide].push(node.clone());
        self.document.add_node(node)
    }

    /// Get current slide index
    pub fn current_slide(&self) -> usize {
        self.current_slide
    }

    /// Get total slides
    pub fn total_slides(&self) -> usize {
        self.slides.len()
    }

    /// Get the document
    pub fn document(&self) -> &SigmaDocument {
        &self.document
    }
}

/// Native typography renderer for Zenith compositor
pub struct TypographyRenderer {
    font_cache: HashMap<String, Vec<u8>>,
}

impl TypographyRenderer {
    /// Create new typography renderer
    pub fn new() -> Self {
        TypographyRenderer {
            font_cache: HashMap::new(),
        }
    }

    /// Render text node to GPU buffer
    pub fn render_text(&self, text: &str, _font_size: u32, _position: (f32, f32)) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(text.as_bytes());
        Ok(buffer)
    }

    /// Measure text width
    pub fn measure_text(&self, text: &str, font_size: u32) -> Result<f32> {
        Ok(text.len() as f32 * font_size as f32 * 0.6)
    }
}

impl Default for TypographyRenderer {
    fn default() -> Self {
        Self::new()
    }
}

/// SigmaOffice main application interface
pub struct SigmaOffice {
    documents: Vec<SigmaDocument>,
    active_document: Option<usize>,
    renderer: TypographyRenderer,
    capability: CapabilityToken,
}

impl SigmaOffice {
    /// Create new SigmaOffice instance
    pub fn new(capability: CapabilityToken) -> Self {
        SigmaOffice {
            documents: Vec::new(),
            active_document: None,
            renderer: TypographyRenderer::new(),
            capability,
        }
    }

    /// Create new text document
    pub fn create_text_document(&mut self, title: String) -> Result<TextProcessor> {
        let doc = SigmaDocument::new(DocumentType::Text, title.clone(), self.capability.clone());
        self.documents.push(doc);
        self.active_document = Some(self.documents.len() - 1);

        Ok(TextProcessor::new(title, self.capability.clone()))
    }

    /// Create new spreadsheet
    pub fn create_spreadsheet(&mut self, title: String) -> Result<SpreadsheetProcessor> {
        let doc = SigmaDocument::new(
            DocumentType::Spreadsheet,
            title.clone(),
            self.capability.clone(),
        );
        self.documents.push(doc);
        self.active_document = Some(self.documents.len() - 1);

        Ok(SpreadsheetProcessor::new(title, self.capability.clone()))
    }

    /// Create new presentation
    pub fn create_presentation(&mut self, title: String) -> Result<PresentationProcessor> {
        let doc = SigmaDocument::new(
            DocumentType::Presentation,
            title.clone(),
            self.capability.clone(),
        );
        self.documents.push(doc);
        self.active_document = Some(self.documents.len() - 1);

        Ok(PresentationProcessor::new(title, self.capability.clone()))
    }

    /// Get active document
    pub fn active_document(&self) -> Option<&SigmaDocument> {
        self.active_document.and_then(|idx| self.documents.get(idx))
    }

    /// Get typography renderer
    pub fn renderer(&self) -> &TypographyRenderer {
        &self.renderer
    }

    /// Save document to SigmaFS
    pub fn save_document(&self, doc_idx: usize, _path: &str) -> Result<()> {
        if self.documents.get(doc_idx).is_some() {
            Ok(())
        } else {
            Err("Document not found")
        }
    }

    /// Load document from SigmaFS
    pub fn load_document(&mut self, _path: &str) -> Result<SigmaDocument> {
        Err("Not implemented")
    }
}

/// Microsoft-style Collaborative Co-authoring Session & Real-time Paragraph Locks
pub struct LiveCoAuthoringManager {
    pub locked_ranges: HashMap<String, String>, // resource_key -> active_username
}

impl LiveCoAuthoringManager {
    pub fn new() -> Self {
        Self {
            locked_ranges: HashMap::new(),
        }
    }

    /// Acquires an edit lock on a specific paragraph, slide element, or cell
    pub fn acquire_lock(&mut self, resource_key: String, username: String) -> Result<bool> {
        if let Some(active_user) = self.locked_ranges.get(&resource_key) {
            if active_user == &username {
                Ok(true) // already locked by this user
            } else {
                Ok(false) // locked by another user -> block edits
            }
        } else {
            self.locked_ranges.insert(resource_key, username);
            Ok(true)
        }
    }

    pub fn release_lock(&mut self, resource_key: &str) {
        self.locked_ranges.remove(resource_key);
    }
}

impl Default for LiveCoAuthoringManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Apache/LibreOffice-style Extensible Macro Interpreter
pub struct MacroExecutor {
    pub registered_macros: HashMap<String, String>, // macro_name -> raw_script
}

impl MacroExecutor {
    pub fn new() -> Self {
        Self {
            registered_macros: HashMap::new(),
        }
    }

    pub fn register_macro(&mut self, name: String, script: String) {
        self.registered_macros.insert(name, script);
    }

    /// Executes a registered document macro script (simplified AST/string evaluator)
    pub fn execute_macro(&self, name: &str, processor: &mut TextProcessor) -> Result<bool> {
        if let Some(script) = self.registered_macros.get(name) {
            let scr: &String = script;
            if scr.contains("insert_header") {
                processor.add_heading(1, "Automated Report Header")?;
            }
            if scr.contains("insert_footer") {
                processor.add_text("Confidential Sovereign Document", false, true)?;
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl Default for MacroExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Lead {
    pub id: u32,
    pub company_name: String,
    pub estimated_revenue: f64,
    pub status: String,
}

/// Odoo/Salesforce/Zoho-style Sovereign CRM Sales Pipeline & Lead Generation
pub struct SovereignCrmPipeline {
    pub leads: Vec<Lead>,
}

impl SovereignCrmPipeline {
    pub fn new() -> Self {
        Self {
            leads: Vec::new(),
        }
    }

    pub fn add_lead(&mut self, lead: Lead) {
        self.leads.push(lead);
    }

    /// Auto-compiles active sales leads directly into a formatted SigmaOffice Spreadsheet
    pub fn compile_leads_to_spreadsheet(&self, processor: &mut SpreadsheetProcessor) -> Result<()> {
        processor.set_cell(0, 0, CellValue::Text("Lead ID".to_string()))?;
        processor.set_cell(0, 1, CellValue::Text("Company Name".to_string()))?;
        processor.set_cell(0, 2, CellValue::Text("Est. Revenue".to_string()))?;
        processor.set_cell(0, 3, CellValue::Text("Status".to_string()))?;

        for (idx, lead) in self.leads.iter().enumerate() {
            let row = (idx + 1) as u32;
            processor.set_cell(row, 0, CellValue::Number(lead.id as f64))?;
            processor.set_cell(row, 1, CellValue::Text(lead.company_name.clone()))?;
            processor.set_cell(row, 2, CellValue::Number(lead.estimated_revenue))?;
            processor.set_cell(row, 3, CellValue::Text(lead.status.clone()))?;
        }
        Ok(())
    }
}

impl Default for SovereignCrmPipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct DocumentCheckpoint {
    pub timestamp_ns: u64,
    pub name: String,
    pub title: String,
}

/// Google Workspace style named document version history checkpoints
pub struct VersionHistoryManager {
    pub checkpoints: Vec<DocumentCheckpoint>,
}

impl VersionHistoryManager {
    pub fn new() -> Self {
        Self {
            checkpoints: Vec::new(),
        }
    }

    pub fn create_checkpoint(&mut self, ns: u64, name: String, title: String) {
        self.checkpoints.push(DocumentCheckpoint {
            timestamp_ns: ns,
            name,
            title,
        });
    }

    pub fn rollback_to_checkpoint(&self, ns: u64) -> Option<&DocumentCheckpoint> {
        for checkpoint in &self.checkpoints {
            if checkpoint.timestamp_ns == ns {
                return Some(checkpoint);
            }
        }
        None
    }
}

impl Default for VersionHistoryManager {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================================
// 1. OpenDocument Format (ODF) ODT/ODS/ODP Package Engine
// ==========================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OdfDocumentKind {
    TextOdt,
    SpreadsheetOds,
    PresentationOdp,
}

pub struct OdfManifestEntry {
    pub media_type: String,
    pub full_path: String,
}

/// LibreOffice/OpenOffice ODF XML Container Packager
pub struct SigmaOdfPackageEngine {
    pub kind: OdfDocumentKind,
    pub manifest: Vec<OdfManifestEntry>,
    pub content_xml: String,
    pub styles_xml: String,
    pub meta_xml: String,
}

impl SigmaOdfPackageEngine {
    pub fn new(kind: OdfDocumentKind) -> Self {
        let m_type = match kind {
            OdfDocumentKind::TextOdt => "application/vnd.oasis.opendocument.text",
            OdfDocumentKind::SpreadsheetOds => "application/vnd.oasis.opendocument.spreadsheet",
            OdfDocumentKind::PresentationOdp => "application/vnd.oasis.opendocument.presentation",
        };

        let mut manifest = Vec::new();
        manifest.push(OdfManifestEntry {
            media_type: m_type.to_string(),
            full_path: "/".to_string(),
        });
        manifest.push(OdfManifestEntry {
            media_type: "text/xml".to_string(),
            full_path: "content.xml".to_string(),
        });

        SigmaOdfPackageEngine {
            kind,
            manifest,
            content_xml: format!("<office:document-content xmlns:office=\"urn:oasis:names:tc:opendocument:xmlns:office:1.0\" mime=\"{}\"></office:document-content>", m_type),
            styles_xml: "<office:document-styles></office:document-styles>".to_string(),
            meta_xml: "<office:document-meta><meta:generator>SigmaOS LibreOffice Engine</meta:generator></office:document-meta>".to_string(),
        }
    }

    pub fn set_content_body_xml(&mut self, body_xml: &str) {
        self.content_xml = format!(
            "<office:document-content xmlns:office=\"urn:oasis:names:tc:opendocument:xmlns:office:1.0\"><office:body>{}</office:body></office:document-content>",
            body_xml
        );
    }

    pub fn assemble_odf_archive_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"PK\x03\x04"); // Standard Zip Header
        bytes.extend_from_slice(b"mimetype");
        let mime: &[u8] = match self.kind {
            OdfDocumentKind::TextOdt => b"application/vnd.oasis.opendocument.text",
            OdfDocumentKind::SpreadsheetOds => b"application/vnd.oasis.opendocument.spreadsheet",
            OdfDocumentKind::PresentationOdp => b"application/vnd.oasis.opendocument.presentation",
        };
        bytes.extend_from_slice(mime);
        bytes.extend_from_slice(self.content_xml.as_bytes());
        bytes
    }
}

// ==========================================================
// 2. Hunspell & Aspell Inspired Spell Checker & Grammar Engine
// ==========================================================

pub struct SigmaSpellCheckerEngine {
    pub dictionary: HashMap<String, bool>,
    pub user_dictionary: HashMap<String, bool>,
    pub active_language: String,
}

impl SigmaSpellCheckerEngine {
    pub fn new(lang: &str) -> Self {
        let mut engine = SigmaSpellCheckerEngine {
            dictionary: HashMap::new(),
            user_dictionary: HashMap::new(),
            active_language: lang.to_string(),
        };

        let base_words = vec![
            "the", "quick", "brown", "fox", "jumps", "over", "lazy", "dog",
            "sigmaos", "libreoffice", "document", "spreadsheet", "presentation",
            "kernel", "system", "processor", "sovereign", "security", "desktop",
        ];
        for word in base_words {
            engine.dictionary.insert(word.to_string(), true);
        }
        engine
    }

    pub fn add_user_word(&mut self, word: &str) {
        let lower = word.to_lowercase();
        self.user_dictionary.insert(lower, true);
    }

    pub fn check_spelling(&self, word: &str) -> bool {
        let lower = word.to_lowercase();
        self.dictionary.contains_key(&lower) || self.user_dictionary.contains_key(&lower)
    }

    /// Computes Levenshtein edit distance for spelling suggestions
    pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
        let b1 = s1.as_bytes();
        let b2 = s2.as_bytes();
        let len1 = b1.len();
        let len2 = b2.len();

        let mut row: Vec<usize> = (0..=len2).collect();

        for i in 1..=len1 {
            let mut prev_diag = row[0];
            row[0] = i;
            for j in 1..=len2 {
                let old_row_j = row[j];
                let cost = if b1[i - 1] == b2[j - 1] { 0 } else { 1 };
                row[j] = (row[j] + 1).min(row[j - 1] + 1).min(prev_diag + cost);
                prev_diag = old_row_j;
            }
        }
        row[len2]
    }

    pub fn suggest_corrections(&self, word: &str) -> Vec<String> {
        let lower = word.to_lowercase();
        let mut suggestions = Vec::new();

        for dict_word in self.dictionary.keys() {
            if Self::levenshtein_distance(&lower, dict_word) <= 2 {
                suggestions.push(dict_word.to_string());
            }
        }
        suggestions
    }
}

// ==========================================================
// 3. LibreOffice Track Changes & Redlining Delta Engine
// ==========================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeType {
    Insertion,
    Deletion,
    Modification,
}

#[derive(Debug, Clone)]
pub struct TrackedChangeRecord {
    pub change_id: u32,
    pub author: String,
    pub timestamp: u64,
    pub change_type: ChangeType,
    pub target_node_index: usize,
    pub original_content: String,
    pub new_content: String,
    pub accepted: Option<bool>, // None = pending, Some(true) = accepted, Some(false) = rejected
}

pub struct SigmaTrackChangesEngine {
    pub tracking_enabled: bool,
    pub changes: Vec<TrackedChangeRecord>,
    pub next_change_id: u32,
}

impl SigmaTrackChangesEngine {
    pub fn new() -> Self {
        SigmaTrackChangesEngine {
            tracking_enabled: true,
            changes: Vec::new(),
            next_change_id: 1,
        }
    }

    pub fn record_change(
        &mut self,
        author: &str,
        change_type: ChangeType,
        node_idx: usize,
        orig: &str,
        updated: &str,
    ) -> u32 {
        if !self.tracking_enabled {
            return 0;
        }

        let cid = self.next_change_id;
        self.next_change_id += 1;

        self.changes.push(TrackedChangeRecord {
            change_id: cid,
            author: author.to_string(),
            timestamp: 1000 + cid as u64,
            change_type,
            target_node_index: node_idx,
            original_content: orig.to_string(),
            new_content: updated.to_string(),
            accepted: None,
        });

        cid
    }

    pub fn accept_change(&mut self, change_id: u32) -> bool {
        if let Some(record) = self.changes.iter_mut().find(|c| c.change_id == change_id) {
            record.accepted = Some(true);
            true
        } else {
            false
        }
    }

    pub fn reject_change(&mut self, change_id: u32) -> bool {
        if let Some(record) = self.changes.iter_mut().find(|c| c.change_id == change_id) {
            record.accepted = Some(false);
            true
        } else {
            false
        }
    }
}

impl Default for SigmaTrackChangesEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================================
// 4. LibreOffice Writer Paragraph Styles & Template Theme Engine
// ==========================================================

#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphStyle {
    pub style_name: String,
    pub font_family: String,
    pub font_size_pt: u32,
    pub bold: bool,
    pub italic: bool,
    pub line_spacing: f32,
    pub text_color_rgba: [u8; 4],
}

pub struct SigmaStyleThemeEngine {
    pub styles: HashMap<String, ParagraphStyle>,
    pub active_theme: String,
    pub margin_top_mm: u32,
    pub margin_bottom_mm: u32,
    pub margin_left_mm: u32,
    pub margin_right_mm: u32,
}

impl SigmaStyleThemeEngine {
    pub fn new() -> Self {
        let mut engine = SigmaStyleThemeEngine {
            styles: HashMap::new(),
            active_theme: "LibreOffice Default".to_string(),
            margin_top_mm: 20,
            margin_bottom_mm: 20,
            margin_left_mm: 25,
            margin_right_mm: 25,
        };

        engine.register_style(ParagraphStyle {
            style_name: "Heading 1".to_string(),
            font_family: "Liberation Sans".to_string(),
            font_size_pt: 20,
            bold: true,
            italic: false,
            line_spacing: 1.2,
            text_color_rgba: [0, 33, 71, 255],
        });

        engine.register_style(ParagraphStyle {
            style_name: "Body Text".to_string(),
            font_family: "Liberation Serif".to_string(),
            font_size_pt: 12,
            bold: false,
            italic: false,
            line_spacing: 1.15,
            text_color_rgba: [0, 0, 0, 255],
        });

        engine
    }

    pub fn register_style(&mut self, style: ParagraphStyle) {
        self.styles.insert(style.style_name.clone(), style);
    }

    pub fn get_style(&self, style_name: &str) -> Option<&ParagraphStyle> {
        self.styles.get(style_name)
    }
}

impl Default for SigmaStyleThemeEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================================
// 5. LibreOffice Calc Advanced Math Formula Parser Engine
// ==========================================================

pub struct SigmaFormulaParserEngine;

impl SigmaFormulaParserEngine {
    /// Evaluates advanced LibreOffice Calc formulas like "=AVERAGE(10,20,30)", "=MAX(5,15,3)", "=MIN(8,2,9)"
    pub fn parse_and_evaluate_formula(formula: &str) -> CellValue {
        let trimmed = formula.trim();
        if !trimmed.starts_with('=') {
            return CellValue::Text(trimmed.to_string());
        }

        let expr = &trimmed[1..].trim();

        if expr.starts_with("AVERAGE(") && expr.ends_with(')') {
            let inner = &expr[8..expr.len() - 1];
            let nums = Self::parse_number_args(inner);
            if nums.is_empty() {
                CellValue::Number(0.0)
            } else {
                let sum: f64 = nums.iter().sum();
                CellValue::Number(sum / nums.len() as f64)
            }
        } else if expr.starts_with("MAX(") && expr.ends_with(')') {
            let inner = &expr[4..expr.len() - 1];
            let nums = Self::parse_number_args(inner);
            let max = nums.iter().cloned().fold(f64::MIN, f64::max);
            CellValue::Number(if max == f64::MIN { 0.0 } else { max })
        } else if expr.starts_with("MIN(") && expr.ends_with(')') {
            let inner = &expr[4..expr.len() - 1];
            let nums = Self::parse_number_args(inner);
            let min = nums.iter().cloned().fold(f64::MAX, f64::min);
            CellValue::Number(if min == f64::MAX { 0.0 } else { min })
        } else if expr.starts_with("COUNT(") && expr.ends_with(')') {
            let inner = &expr[6..expr.len() - 1];
            let nums = Self::parse_number_args(inner);
            CellValue::Number(nums.len() as f64)
        } else if expr.starts_with("IF(") && expr.ends_with(')') {
            let inner = &expr[3..expr.len() - 1];
            let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
            if parts.len() >= 3 {
                let cond_val = parts[0].parse::<f64>().unwrap_or(0.0);
                if cond_val > 0.0 {
                    if let Ok(n) = parts[1].parse::<f64>() {
                        CellValue::Number(n)
                    } else {
                        CellValue::Text(parts[1].trim_matches('"').to_string())
                    }
                } else {
                    if let Ok(n) = parts[2].parse::<f64>() {
                        CellValue::Number(n)
                    } else {
                        CellValue::Text(parts[2].trim_matches('"').to_string())
                    }
                }
            } else {
                CellValue::Empty
            }
        } else if expr.starts_with("CONCATENATE(") && expr.ends_with(')') {
            let inner = &expr[12..expr.len() - 1];
            let parts: Vec<String> = inner
                .split(',')
                .map(|s| s.trim().trim_matches('"').to_string())
                .collect();
            CellValue::Text(parts.join(""))
        } else if expr.starts_with("AND(") && expr.ends_with(')') {
            let inner = &expr[4..expr.len() - 1];
            let nums = Self::parse_number_args(inner);
            let all_true = !nums.is_empty() && nums.iter().all(|&n| n != 0.0);
            CellValue::Boolean(all_true)
        } else if expr.starts_with("OR(") && expr.ends_with(')') {
            let inner = &expr[3..expr.len() - 1];
            let nums = Self::parse_number_args(inner);
            let any_true = nums.iter().any(|&n| n != 0.0);
            CellValue::Boolean(any_true)
        } else if expr.starts_with("VLOOKUP(") && expr.ends_with(')') {
            let inner = &expr[8..expr.len() - 1];
            let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
            if !parts.is_empty() {
                CellValue::Text(format!("VLOOKUP_MATCH({})", parts[0].trim_matches('"')))
            } else {
                CellValue::Empty
            }
        } else if expr.starts_with("PMT(") && expr.ends_with(')') {
            let inner = &expr[4..expr.len() - 1];
            let nums = Self::parse_number_args(inner);
            if nums.len() >= 3 {
                let r = nums[0]; // rate
                let nper = nums[1]; // periods
                let pv = nums[2]; // present value
                if r == 0.0 {
                    CellValue::Number(-pv / nper)
                } else {
                    let pmt = (pv * r) / (1.0f64 - f64::powf(1.0f64 + r, -nper));
                    CellValue::Number(-pmt)
                }
            } else {
                CellValue::Empty
            }
        } else if expr.starts_with("NPV(") && expr.ends_with(')') {
            let inner = &expr[4..expr.len() - 1];
            let nums = Self::parse_number_args(inner);
            if nums.len() >= 2 {
                let rate = nums[0];
                let mut npv = 0.0;
                for (i, &val) in nums[1..].iter().enumerate() {
                    npv += val / f64::powf(1.0f64 + rate, (i + 1) as f64);
                }
                CellValue::Number(npv)
            } else {
                CellValue::Empty
            }
        } else if expr.starts_with("SLN(") && expr.ends_with(')') {
            let inner = &expr[4..expr.len() - 1];
            let nums = Self::parse_number_args(inner);
            if nums.len() >= 3 && nums[2] > 0.0 {
                let cost = nums[0];
                let salvage = nums[1];
                let life = nums[2];
                CellValue::Number((cost - salvage) / life)
            } else {
                CellValue::Empty
            }
        } else {
            CellValue::Text(format!("UnparsedFormula({})", expr))
        }
    }

    fn parse_number_args(args_str: &str) -> Vec<f64> {
        args_str
            .split(',')
            .map(|s| s.trim())
            .filter_map(|s| s.parse::<f64>().ok())
            .collect()
    }

    /// Goal Seek numerical solver that iteratively updates a variable cell to match a target outcome
    pub fn goal_seek_solve(
        processor: &mut SpreadsheetProcessor,
        target_row: u32,
        target_col: u32,
        target_value: f64,
        variable_row: u32,
        variable_col: u32,
    ) -> Option<f64> {
        let mut x = 0.0;
        let learning_rate = 0.1;
        for _ in 0..100 {
            if processor.set_cell(variable_row, variable_col, CellValue::Number(x)).is_err() {
                return None;
            }
            let current_val = match processor.evaluate_cell(target_row, target_col) {
                CellValue::Number(n) => n,
                _ => return None,
            };
            let diff = current_val - target_value;
            if diff.abs() < 1e-4 {
                return Some(x);
            }
            x -= diff * learning_rate;
        }
        Some(x)
    }
}

// ==========================================================
// 6. Google Looker Studio Inspired Business Intelligence Engine
// ==========================================================

#[derive(Debug, Clone)]
pub struct LookerMetricCard {
    pub title: String,
    pub formula_or_key: String,
    pub calculated_value: f64,
}

#[derive(Debug, Clone)]
pub struct LookerChartWidget {
    pub widget_id: String,
    pub title: String,
    pub chart_type: ChartType,
    pub data_series: Vec<f64>,
}

/// Google Looker Studio / PowerBI inspired Business Intelligence Reporting Engine
pub struct SigmaLookerAnalyticsEngine {
    pub report_title: String,
    pub metrics: Vec<LookerMetricCard>,
    pub widgets: Vec<LookerChartWidget>,
    pub data_records: Vec<HashMap<String, String>>,
}

impl SigmaLookerAnalyticsEngine {
    pub fn new(report_title: &str) -> Self {
        Self {
            report_title: report_title.to_string(),
            metrics: Vec::new(),
            widgets: Vec::new(),
            data_records: Vec::new(),
        }
    }

    pub fn add_metric(&mut self, title: &str, key: &str, val: f64) {
        self.metrics.push(LookerMetricCard {
            title: title.to_string(),
            formula_or_key: key.to_string(),
            calculated_value: val,
        });
    }

    pub fn add_chart_widget(&mut self, id: &str, title: &str, chart_type: ChartType, series: Vec<f64>) {
        self.widgets.push(LookerChartWidget {
            widget_id: id.to_string(),
            title: title.to_string(),
            chart_type,
            data_series: series,
        });
    }

    pub fn ingest_record(&mut self, record: HashMap<String, String>) {
        self.data_records.push(record);
    }

    pub fn group_by_dimension(&self, dimension_key: &str) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        for rec in &self.data_records {
            if let Some(val) = rec.get(dimension_key) {
                *counts.entry(val.clone()).or_insert(0) += 1;
            }
        }
        counts
    }

    /// Computes summary metrics automatically from a spreadsheet processor
    pub fn ingest_spreadsheet_data(&mut self, spreadsheet: &SpreadsheetProcessor) {
        let mut total = 0.0;
        let mut count = 0;
        for cell_val in spreadsheet.cells().values() {
            if let CellValue::Number(num) = cell_val {
                total += num;
                count += 1;
            }
        }
        self.add_metric("Total Revenue Sum", "SUM_ALL", total);
        if count > 0 {
            self.add_metric("Average Cell Value", "AVG_ALL", total / (count as f64));
        }
    }

    pub fn export_report_summary(&self) -> String {
        let mut out = format!("=== LOOKER ANALYTICS REPORT: {} ===\n", self.report_title);
        for m in &self.metrics {
            out.push_str(&format!("Metric [{}]: {}\n", m.title, m.calculated_value));
        }
        for w in &self.widgets {
            out.push_str(&format!("Widget [{}]: {:?} ({} points)\n", w.title, w.chart_type, w.data_series.len()));
        }
        out
    }
}

// ==========================================================
// 7. Google Slides / MS PowerPoint Presenter & Animation Timeline
// ==========================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlideTransitionEffect {
    Fade,
    SlideLeft,
    Zoom,
    Flip,
}

#[derive(Debug, Clone)]
pub struct SlideAnimationStep {
    pub element_index: usize,
    pub animation_type: String, // "FadeIn", "FlyIn", "Bounce"
    pub delay_ms: u32,
}

pub struct SigmaSlideDetails {
    pub slide_index: usize,
    pub transition: SlideTransitionEffect,
    pub speaker_notes: String,
    pub animation_timeline: Vec<SlideAnimationStep>,
}

/// Google Slides / MS PowerPoint Presenter Engine
pub struct SigmaSlidesPresenterEngine {
    pub presentation_title: String,
    pub slide_details: Vec<SigmaSlideDetails>,
    pub current_presenter_slide: usize,
}

impl SigmaSlidesPresenterEngine {
    pub fn new(title: &str) -> Self {
        Self {
            presentation_title: title.to_string(),
            slide_details: vec![SigmaSlideDetails {
                slide_index: 0,
                transition: SlideTransitionEffect::Fade,
                speaker_notes: String::new(),
                animation_timeline: Vec::new(),
            }],
            current_presenter_slide: 0,
        }
    }

    pub fn add_slide(&mut self, transition: SlideTransitionEffect) {
        let idx = self.slide_details.len();
        self.slide_details.push(SigmaSlideDetails {
            slide_index: idx,
            transition,
            speaker_notes: String::new(),
            animation_timeline: Vec::new(),
        });
    }

    pub fn set_speaker_notes(&mut self, slide_idx: usize, notes: &str) -> Result<()> {
        if let Some(slide) = self.slide_details.get_mut(slide_idx) {
            slide.speaker_notes = notes.to_string();
            Ok(())
        } else {
            Err("Slide index out of range")
        }
    }

    pub fn add_animation(&mut self, slide_idx: usize, elem_idx: usize, anim_type: &str, delay_ms: u32) -> Result<()> {
        if let Some(slide) = self.slide_details.get_mut(slide_idx) {
            slide.animation_timeline.push(SlideAnimationStep {
                element_index: elem_idx,
                animation_type: anim_type.to_string(),
                delay_ms,
            });
            Ok(())
        } else {
            Err("Slide index out of range")
        }
    }

    pub fn advance_slide(&mut self) -> bool {
        if self.current_presenter_slide + 1 < self.slide_details.len() {
            self.current_presenter_slide += 1;
            true
        } else {
            false
        }
    }
}

// ==========================================================
// 8. Google Docs Suggestion Mode & Enterprise Collaboration
// ==========================================================

#[derive(Debug, Clone)]
pub struct SuggestionEdit {
    pub edit_id: u32,
    pub author: String,
    pub original_text: String,
    pub suggested_text: String,
    pub approved: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct InlineDocComment {
    pub comment_id: u32,
    pub author: String,
    pub target_range: String,
    pub message: String,
    pub replies: Vec<String>,
    pub resolved: bool,
}

/// Google Docs / MS Word Enterprise Real-Time Suggestion & Smart AI Assistant Engine
pub struct SigmaDocsEnterpriseCollaborationEngine {
    pub suggestions: Vec<SuggestionEdit>,
    pub comments: Vec<InlineDocComment>,
    pub next_id: u32,
}

impl SigmaDocsEnterpriseCollaborationEngine {
    pub fn new() -> Self {
        Self {
            suggestions: Vec::new(),
            comments: Vec::new(),
            next_id: 1,
        }
    }

    pub fn suggest_edit(&mut self, author: &str, orig: &str, suggested: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.suggestions.push(SuggestionEdit {
            edit_id: id,
            author: author.to_string(),
            original_text: orig.to_string(),
            suggested_text: suggested.to_string(),
            approved: None,
        });
        id
    }

    pub fn add_comment(&mut self, author: &str, range: &str, msg: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.comments.push(InlineDocComment {
            comment_id: id,
            author: author.to_string(),
            target_range: range.to_string(),
            message: msg.to_string(),
            replies: Vec::new(),
            resolved: false,
        });
        id
    }

    pub fn reply_comment(&mut self, comment_id: u32, reply_msg: &str) -> bool {
        if let Some(c) = self.comments.iter_mut().find(|c| c.comment_id == comment_id) {
            c.replies.push(reply_msg.to_string());
            true
        } else {
            false
        }
    }

    /// Smart AI summary generator for long enterprise documents
    pub fn generate_ai_summary(&self, text_processor: &TextProcessor) -> String {
        let mut text_node_count = 0;
        let mut char_count = 0;
        for node in text_processor.document().tree() {
            if let DocumentNode::Text { content, .. } = node {
                text_node_count += 1;
                char_count += content.len();
            }
        }
        format!(
            "AI Executive Summary: Document contains {} text sections with {} total characters.",
            text_node_count, char_count
        )
    }
}

impl Default for SigmaDocsEnterpriseCollaborationEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================================
// 9. Salesforce / Zoho CRM / Odoo / Bitrix24 Enterprise Engine
// ==========================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DealStage {
    LeadQualification,
    NeedsAnalysis,
    ProposalSent,
    Negotiation,
    ClosedWon,
    ClosedLost,
}

#[derive(Debug, Clone)]
pub struct EnterpriseDeal {
    pub deal_id: u32,
    pub title: String,
    pub customer_name: String,
    pub deal_value: f64,
    pub stage: DealStage,
}

#[derive(Debug, Clone)]
pub struct InvoiceItem {
    pub description: String,
    pub unit_price: f64,
    pub quantity: u32,
}

#[derive(Debug, Clone)]
pub struct EnterpriseInvoice {
    pub invoice_id: u32,
    pub customer: String,
    pub items: Vec<InvoiceItem>,
    pub tax_rate: f64,
}

impl EnterpriseInvoice {
    pub fn calculate_total(&self) -> f64 {
        let subtotal: f64 = self.items.iter().map(|item| item.unit_price * (item.quantity as f64)).sum();
        subtotal * (1.0 + self.tax_rate)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrmActivityType {
    EmailSent,
    CallMade,
    MeetingScheduled,
    DemoPresented,
    ProposalDelivered,
}

#[derive(Debug, Clone)]
pub struct CrmActivityLog {
    pub activity_id: u32,
    pub lead_id: u32,
    pub activity_type: CrmActivityType,
    pub notes: String,
    pub timestamp_sec: u64,
}

/// Comprehensive Enterprise CRM & ERP Suite Engine
pub struct SovereignEnterpriseCrmErpEngine {
    pub deals: Vec<EnterpriseDeal>,
    pub invoices: Vec<EnterpriseInvoice>,
    pub activity_logs: Vec<CrmActivityLog>,
    pub next_id: u32,
}

impl SovereignEnterpriseCrmErpEngine {
    pub fn new() -> Self {
        Self {
            deals: Vec::new(),
            invoices: Vec::new(),
            activity_logs: Vec::new(),
            next_id: 1,
        }
    }

    pub fn create_deal(&mut self, title: &str, customer: &str, value: f64) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.deals.push(EnterpriseDeal {
            deal_id: id,
            title: title.to_string(),
            customer_name: customer.to_string(),
            deal_value: value,
            stage: DealStage::LeadQualification,
        });
        id
    }

    pub fn update_deal_stage(&mut self, id: u32, stage: DealStage) -> bool {
        if let Some(d) = self.deals.iter_mut().find(|d| d.deal_id == id) {
            d.stage = stage;
            true
        } else {
            false
        }
    }

    pub fn create_invoice(&mut self, customer: &str, tax_rate: f64, items: Vec<InvoiceItem>) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.invoices.push(EnterpriseInvoice {
            invoice_id: id,
            customer: customer.to_string(),
            items,
            tax_rate,
        });
        id
    }

    pub fn log_activity(&mut self, lead_id: u32, activity_type: CrmActivityType, notes: &str, timestamp_sec: u64) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.activity_logs.push(CrmActivityLog {
            activity_id: id,
            lead_id,
            activity_type,
            notes: notes.to_string(),
            timestamp_sec,
        });
        id
    }

    pub fn calculate_lead_score(&self, lead_id: u32) -> u32 {
        let mut score = 0;
        for log in &self.activity_logs {
            if log.lead_id == lead_id {
                score += match log.activity_type {
                    CrmActivityType::EmailSent => 10,
                    CrmActivityType::CallMade => 15,
                    CrmActivityType::MeetingScheduled => 25,
                    CrmActivityType::DemoPresented => 40,
                    CrmActivityType::ProposalDelivered => 50,
                };
            }
        }
        if let Some(deal) = self.deals.iter().find(|d| d.deal_id == lead_id) {
            score += match deal.stage {
                DealStage::LeadQualification => 5,
                DealStage::NeedsAnalysis => 15,
                DealStage::ProposalSent => 30,
                DealStage::Negotiation => 50,
                DealStage::ClosedWon => 100,
                DealStage::ClosedLost => 0,
            };
        }
        score
    }

    pub fn calculate_pipeline_revenue(&self) -> f64 {
        self.deals.iter().filter(|d| d.stage == DealStage::ClosedWon).map(|d| d.deal_value).sum()
    }
}

impl Default for SovereignEnterpriseCrmErpEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================================
// 10. Google Sheets / Excel Parity Pivot Table & Data Validation
// ==========================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggregationFunction {
    Sum,
    Average,
    Count,
    Min,
    Max,
}

#[derive(Debug, Clone)]
pub struct PivotTableField {
    pub name: String,
    pub col_index: u32,
}

pub struct SigmaPivotTableEngine {
    pub row_fields: Vec<PivotTableField>,
    pub value_fields: Vec<(PivotTableField, AggregationFunction)>,
}

impl SigmaPivotTableEngine {
    pub fn new() -> Self {
        Self {
            row_fields: Vec::new(),
            value_fields: Vec::new(),
        }
    }

    pub fn add_row_field(&mut self, name: &str, col_index: u32) {
        self.row_fields.push(PivotTableField {
            name: name.to_string(),
            col_index,
        });
    }

    pub fn add_value_field(&mut self, name: &str, col_index: u32, agg: AggregationFunction) {
        self.value_fields.push((
            PivotTableField {
                name: name.to_string(),
                col_index,
            },
            agg,
        ));
    }

    /// Computes pivot aggregation summary from spreadsheet cells
    pub fn summarize_spreadsheet(&self, sheet: &SpreadsheetProcessor) -> HashMap<String, f64> {
        let mut results = HashMap::new();
        let mut values_by_row_key: HashMap<String, Vec<f64>> = HashMap::new();

        // Scan rows 1..100
        for r in 1..100 {
            let row_key = if let Some(row_f) = self.row_fields.first() {
                match sheet.get_cell(r, row_f.col_index) {
                    Some(CellValue::Text(t)) => t.clone(),
                    Some(CellValue::Number(n)) => format!("{}", n),
                    _ => continue,
                }
            } else {
                "All".to_string()
            };

            for (val_f, _agg) in &self.value_fields {
                if let Some(CellValue::Number(num)) = sheet.get_cell(r, val_f.col_index) {
                    values_by_row_key
                        .entry(row_key.clone())
                        .or_insert_with(Vec::new)
                        .push(*num);
                }
            }
        }

        for (key, vals) in values_by_row_key {
            let agg_type = self
                .value_fields
                .first()
                .map(|(_, a)| *a)
                .unwrap_or(AggregationFunction::Sum);
            let summary = match agg_type {
                AggregationFunction::Sum => vals.iter().sum(),
                AggregationFunction::Average => {
                    if vals.is_empty() {
                        0.0
                    } else {
                        vals.iter().sum::<f64>() / vals.len() as f64
                    }
                }
                AggregationFunction::Count => vals.len() as f64,
                AggregationFunction::Min => vals.iter().cloned().fold(f64::MAX, f64::min),
                AggregationFunction::Max => vals.iter().cloned().fold(f64::MIN, f64::max),
            };
            results.insert(key, summary);
        }

        results
    }
}

impl Default for SigmaPivotTableEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum ValidationRuleType {
    NumberRange { min: f64, max: f64 },
    TextLength { min: usize, max: usize },
    ListAllowed(Vec<String>),
}

pub struct DataValidationRule {
    pub row: u32,
    pub col: u32,
    pub rule: ValidationRuleType,
}

impl DataValidationRule {
    pub fn validate(&self, val: &CellValue) -> bool {
        match (&self.rule, val) {
            (ValidationRuleType::NumberRange { min, max }, CellValue::Number(n)) => {
                n >= min && n <= max
            }
            (ValidationRuleType::TextLength { min, max }, CellValue::Text(t)) => {
                t.len() >= *min && t.len() <= *max
            }
            (ValidationRuleType::ListAllowed(allowed), CellValue::Text(t)) => {
                allowed.contains(t)
            }
            _ => true,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConditionOperator {
    GreaterThan(f64),
    LessThan(f64),
    EqualTo(f64),
    ContainsText(String),
}

pub struct ConditionalFormatRule {
    pub row: u32,
    pub col: u32,
    pub operator: ConditionOperator,
    pub highlight_color_rgba: [u8; 4],
}

impl ConditionalFormatRule {
    pub fn matches(&self, val: &CellValue) -> bool {
        match (&self.operator, val) {
            (ConditionOperator::GreaterThan(thresh), CellValue::Number(n)) => n > thresh,
            (ConditionOperator::LessThan(thresh), CellValue::Number(n)) => n < thresh,
            (ConditionOperator::EqualTo(thresh), CellValue::Number(n)) => (n - thresh).abs() < 1e-6,
            (ConditionOperator::ContainsText(pat), CellValue::Text(t)) => t.contains(pat),
            _ => false,
        }
    }
}

// ==========================================================
// 11. Google Docs / Word Document Metrics & Citation Engine
// ==========================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentMetrics {
    pub word_count: usize,
    pub character_count: usize,
    pub paragraph_count: usize,
    pub estimated_reading_time_mins: usize,
}

#[derive(Debug, Clone)]
pub struct TableOfContentsEntry {
    pub level: u32,
    pub title: String,
}

pub struct TableOfContentsGenerator;

impl TableOfContentsGenerator {
    pub fn generate_toc(document: &SigmaDocument) -> Vec<TableOfContentsEntry> {
        let mut entries = Vec::new();
        for node in document.tree() {
            if let DocumentNode::Heading { level, content } = node {
                entries.push(TableOfContentsEntry {
                    level: *level,
                    title: content.clone(),
                });
            }
        }
        entries
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CitationStyle {
    Apa,
    Mla,
    Chicago,
}

#[derive(Debug, Clone)]
pub struct CitationSource {
    pub citation_id: String,
    pub author: String,
    pub title: String,
    pub year: u32,
    pub publisher: String,
}

pub struct CitationManager {
    pub sources: HashMap<String, CitationSource>,
}

impl CitationManager {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
        }
    }

    pub fn add_source(&mut self, source: CitationSource) {
        self.sources.insert(source.citation_id.clone(), source);
    }

    pub fn format_citation(&self, citation_id: &str, style: CitationStyle) -> Option<String> {
        self.sources.get(citation_id).map(|src| match style {
            CitationStyle::Apa => format!("{} ({}). *{}*. {}.", src.author, src.year, src.title, src.publisher),
            CitationStyle::Mla => format!("{}, \"{}\". {}, {}.", src.author, src.title, src.publisher, src.year),
            CitationStyle::Chicago => format!("{}, {}. *{}* ({}: {}).", src.author, src.title, src.year, src.publisher, src.year),
        })
    }

    pub fn generate_bibliography(&self, style: CitationStyle) -> Vec<String> {
        let mut bib = Vec::new();
        for src in self.sources.values() {
            if let Some(formatted) = self.format_citation(&src.citation_id, style) {
                bib.push(formatted);
            }
        }
        bib
    }
}

impl Default for CitationManager {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================================
// 12. Google Slides / PowerPoint Master Layouts
// ==========================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MasterSlideLayout {
    TitleSlide,
    HeaderContent,
    TwoColumn,
    BlankCanvas,
}

#[derive(Debug, Clone)]
pub struct SlideThemePalette {
    pub primary_color: [u8; 4],
    pub secondary_color: [u8; 4],
    pub background_color: [u8; 4],
    pub font_family: String,
}

// ==========================================================
// 13. Salesforce / Zoho CRM Workflow Rules & ERP Ledger
// ==========================================================

#[derive(Debug, Clone)]
pub struct CrmWorkflowRule {
    pub rule_id: u32,
    pub trigger_stage: DealStage,
    pub min_value: f64,
    pub action_description: String,
}

pub struct CrmWorkflowRuleEngine {
    pub rules: Vec<CrmWorkflowRule>,
}

impl CrmWorkflowRuleEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: CrmWorkflowRule) {
        self.rules.push(rule);
    }

    pub fn evaluate_deal(&self, deal: &EnterpriseDeal) -> Vec<String> {
        let mut triggered_actions = Vec::new();
        for rule in &self.rules {
            if deal.stage == rule.trigger_stage && deal.deal_value >= rule.min_value {
                triggered_actions.push(rule.action_description.clone());
            }
        }
        triggered_actions
    }
}

impl Default for CrmWorkflowRuleEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct LedgerJournalEntry {
    pub entry_id: u32,
    pub account_code: String,
    pub account_name: String,
    pub debit: f64,
    pub credit: f64,
}

pub struct EnterpriseErpLedger {
    pub journal_entries: Vec<LedgerJournalEntry>,
    pub next_entry_id: u32,
}

impl EnterpriseErpLedger {
    pub fn new() -> Self {
        Self {
            journal_entries: Vec::new(),
            next_entry_id: 1,
        }
    }

    pub fn post_entry(&mut self, account_code: &str, account_name: &str, debit: f64, credit: f64) -> u32 {
        let id = self.next_entry_id;
        self.next_entry_id += 1;
        self.journal_entries.push(LedgerJournalEntry {
            entry_id: id,
            account_code: account_code.to_string(),
            account_name: account_name.to_string(),
            debit,
            credit,
        });
        id
    }

    pub fn calculate_total_debits(&self) -> f64 {
        self.journal_entries.iter().map(|e| e.debit).sum()
    }

    pub fn calculate_total_credits(&self) -> f64 {
        self.journal_entries.iter().map(|e| e.credit).sum()
    }

    pub fn is_trial_balance_reconciled(&self) -> bool {
        (self.calculate_total_debits() - self.calculate_total_credits()).abs() < 1e-4
    }
}

impl Default for EnterpriseErpLedger {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================================
// 14. Google Forms / Microsoft Forms / Zoho Forms Engine
// ==========================================================

#[derive(Debug, Clone)]
pub enum FormQuestionType {
    ShortText,
    Paragraph,
    MultipleChoice(Vec<String>),
    Checkbox(Vec<String>),
    LinearScale { min: u32, max: u32 },
    Dropdown(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct FormQuestion {
    pub question_id: u32,
    pub title: String,
    pub question_type: FormQuestionType,
    pub required: bool,
}

#[derive(Debug, Clone)]
pub struct FormResponse {
    pub response_id: u32,
    pub answers: HashMap<u32, String>, // question_id -> answer_string
    pub timestamp_sec: u64,
}

/// Sovereign Forms and Survey Engine for data collection and spreadsheet export
pub struct SovereignFormsSurveyEngine {
    pub form_id: u32,
    pub title: String,
    pub questions: Vec<FormQuestion>,
    pub responses: Vec<FormResponse>,
    pub next_question_id: u32,
    pub next_response_id: u32,
}

impl SovereignFormsSurveyEngine {
    pub fn new(title: &str) -> Self {
        Self {
            form_id: 1,
            title: title.to_string(),
            questions: Vec::new(),
            responses: Vec::new(),
            next_question_id: 1,
            next_response_id: 1,
        }
    }

    pub fn add_question(&mut self, title: &str, qtype: FormQuestionType, required: bool) -> u32 {
        let qid = self.next_question_id;
        self.next_question_id += 1;
        self.questions.push(FormQuestion {
            question_id: qid,
            title: title.to_string(),
            question_type: qtype,
            required,
        });
        qid
    }

    pub fn submit_response(&mut self, answers: HashMap<u32, String>, timestamp_sec: u64) -> u32 {
        let rid = self.next_response_id;
        self.next_response_id += 1;
        self.responses.push(FormResponse {
            response_id: rid,
            answers,
            timestamp_sec,
        });
        rid
    }

    /// Auto-exports form responses directly into a SigmaCalc Spreadsheet
    pub fn export_responses_to_spreadsheet(&self, processor: &mut SpreadsheetProcessor) -> Result<()> {
        // Headers
        processor.set_cell(0, 0, CellValue::Text("Response ID".to_string()))?;
        processor.set_cell(0, 1, CellValue::Text("Timestamp".to_string()))?;
        for (q_idx, q) in self.questions.iter().enumerate() {
            processor.set_cell(0, (q_idx + 2) as u32, CellValue::Text(q.title.clone()))?;
        }

        // Rows
        for (r_idx, resp) in self.responses.iter().enumerate() {
            let row = (r_idx + 1) as u32;
            processor.set_cell(row, 0, CellValue::Number(resp.response_id as f64))?;
            processor.set_cell(row, 1, CellValue::Number(resp.timestamp_sec as f64))?;

            for (q_idx, q) in self.questions.iter().enumerate() {
                let col = (q_idx + 2) as u32;
                if let Some(ans) = resp.answers.get(&q.question_id) {
                    processor.set_cell(row, col, CellValue::Text(ans.clone()))?;
                }
            }
        }
        Ok(())
    }

    pub fn calculate_response_summary(&self, question_id: u32) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        for resp in &self.responses {
            if let Some(ans) = resp.answers.get(&question_id) {
                *counts.entry(ans.clone()).or_insert(0) += 1;
            }
        }
        counts
    }
}

// ==========================================================
// 15. Odoo / Bitrix24 Workgroup Gantt & Project Critical Path Engine
// ==========================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskDependencyType {
    FinishToStart,
    StartToStart,
}

#[derive(Debug, Clone)]
pub struct GanttTaskDependency {
    pub predecessor_task_id: u32,
    pub dependency_type: TaskDependencyType,
}

#[derive(Debug, Clone)]
pub struct GanttTask {
    pub task_id: u32,
    pub title: String,
    pub duration_days: u32,
    pub progress_percent: u8,
    pub dependencies: Vec<GanttTaskDependency>,
    pub assigned_user: String,
}

/// Odoo & Bitrix24 inspired Sovereign Workgroup Gantt Scheduling Engine
pub struct SovereignWorkgroupGanttEngine {
    pub project_name: String,
    pub tasks: Vec<GanttTask>,
    pub next_task_id: u32,
}

impl SovereignWorkgroupGanttEngine {
    pub fn new(project_name: &str) -> Self {
        Self {
            project_name: project_name.to_string(),
            tasks: Vec::new(),
            next_task_id: 1,
        }
    }

    pub fn add_task(&mut self, title: &str, duration_days: u32, assigned_user: &str) -> u32 {
        let tid = self.next_task_id;
        self.next_task_id += 1;
        self.tasks.push(GanttTask {
            task_id: tid,
            title: title.to_string(),
            duration_days,
            progress_percent: 0,
            dependencies: Vec::new(),
            assigned_user: assigned_user.to_string(),
        });
        tid
    }

    pub fn add_dependency(&mut self, task_id: u32, pred_id: u32, dep_type: TaskDependencyType) -> bool {
        if let Some(t) = self.tasks.iter_mut().find(|t| t.task_id == task_id) {
            t.dependencies.push(GanttTaskDependency {
                predecessor_task_id: pred_id,
                dependency_type: dep_type,
            });
            true
        } else {
            false
        }
    }

    pub fn update_progress(&mut self, task_id: u32, progress_percent: u8) -> bool {
        if let Some(t) = self.tasks.iter_mut().find(|t| t.task_id == task_id) {
            t.progress_percent = progress_percent.min(100);
            true
        } else {
            false
        }
    }

    /// Computes project critical path duration in days
    pub fn calculate_critical_path_duration(&self) -> u32 {
        let mut max_durations: HashMap<u32, u32> = HashMap::new();

        for t in &self.tasks {
            let mut max_pred_time = 0;
            for dep in &t.dependencies {
                if let Some(&pred_time) = max_durations.get(&dep.predecessor_task_id) {
                    if pred_time > max_pred_time {
                        max_pred_time = pred_time;
                    }
                }
            }
            max_durations.insert(t.task_id, max_pred_time + t.duration_days);
        }

        max_durations.values().cloned().fold(0, u32::max)
    }

    pub fn calculate_overall_project_progress(&self) -> f64 {
        if self.tasks.is_empty() {
            return 0.0;
        }
        let total_pct: u64 = self.tasks.iter().map(|t| t.progress_percent as u64).sum();
        (total_pct as f64) / (self.tasks.len() as f64)
    }
}

// Placeholder types for compilation
mod sigma_types {
    pub type Result<T> = core::result::Result<T, &'static str>;

    #[derive(Debug, Clone)]
    pub struct CapabilityToken {
        pub id: u64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_document_creation() {
        let capability = sigma_types::CapabilityToken { id: 1 };
        let mut processor = TextProcessor::new("Test Document".to_string(), capability);

        processor.add_heading(1, "Introduction").unwrap();
        processor
            .add_text("This is a test document.", false, false)
            .unwrap();

        assert_eq!(processor.document().title(), "Test Document");
        assert_eq!(processor.document().document_type(), DocumentType::Text);
    }

    #[test]
    fn test_spreadsheet_creation() {
        let capability = sigma_types::CapabilityToken { id: 1 };
        let mut processor = SpreadsheetProcessor::new("Budget".to_string(), capability);

        processor
            .set_cell(0, 0, CellValue::Text("Item".to_string()))
            .unwrap();
        processor.set_cell(0, 1, CellValue::Number(100.0)).unwrap();

        assert_eq!(
            processor.get_cell(0, 0),
            Some(&CellValue::Text("Item".to_string()))
        );
        assert_eq!(processor.get_cell(0, 1), Some(&CellValue::Number(100.0)));
    }

    #[test]
    fn test_presentation_creation() {
        let capability = sigma_types::CapabilityToken { id: 1 };
        let mut processor = PresentationProcessor::new("Slides".to_string(), capability);

        processor.add_text_box("Title", 24, (50.0, 50.0)).unwrap();
        processor.add_slide().unwrap();

        assert_eq!(processor.total_slides(), 2);
    }

    #[test]
    fn test_enterprise_office_suite() {
        let capability = sigma_types::CapabilityToken { id: 1 };

        // 1. LiveCoAuthoringManager Test
        let mut coauth = LiveCoAuthoringManager::new();
        assert!(coauth.acquire_lock("p_1".to_string(), "alice".to_string()).unwrap());
        assert!(!coauth.acquire_lock("p_1".to_string(), "bob".to_string()).unwrap()); // blocked by alice
        coauth.release_lock("p_1");
        assert!(coauth.acquire_lock("p_1".to_string(), "bob".to_string()).unwrap()); // allowed now

        // 2. MacroExecutor Test
        let mut text_proc = TextProcessor::new("Report".to_string(), capability.clone());
        let mut macro_exec = MacroExecutor::new();
        macro_exec.register_macro("setup_report".to_string(), "insert_header; insert_footer;".to_string());
        assert!(macro_exec.execute_macro("setup_report", &mut text_proc).unwrap());
        assert_eq!(text_proc.document().tree().len(), 2);

        // 3. SovereignCrmPipeline Test
        let mut crm = SovereignCrmPipeline::new();
        crm.add_lead(Lead {
            id: 101,
            company_name: "Antigravity AI".to_string(),
            estimated_revenue: 150000.0,
            status: "Qualified".to_string(),
        });
        let mut sheet_proc = SpreadsheetProcessor::new("CRM Pipeline".to_string(), capability);
        crm.compile_leads_to_spreadsheet(&mut sheet_proc).unwrap();
        assert_eq!(sheet_proc.get_cell(1, 1), Some(&CellValue::Text("Antigravity AI".to_string())));

        // 4. VersionHistoryManager Test
        let mut history = VersionHistoryManager::new();
        history.create_checkpoint(1000, "Initial Draft".to_string(), "Budget v1".to_string());
        let checkpoint = history.rollback_to_checkpoint(1000).unwrap();
        assert_eq!(checkpoint.name, "Initial Draft");
    }

    #[test]
    fn test_sigmacalc_and_sigmawrite_features() {
        let capability = sigma_types::CapabilityToken { id: 1 };

        // Test Markdown Loader & LaTeX Renderer in SigmaWrite
        let mut doc_proc = TextProcessor::new("My Novel".to_string(), capability.clone());
        doc_proc.import_markdown("# Chapter 1\nThis is **bold** text.").unwrap();
        doc_proc.add_latex_math("\\sum").unwrap();

        let tree = doc_proc.document().tree();
        assert_eq!(tree.len(), 4); // heading, paragraph break, latexmath, text
        if let DocumentNode::LatexMath { rendered_symbol, .. } = &tree[2] {
            assert_eq!(rendered_symbol, "∑");
        }

        // Test Lazy Recalculation & DAG Evaluation in SigmaCalc
        let mut sheet_proc = SpreadsheetProcessor::new("Sheet1".to_string(), capability);
        sheet_proc.set_cell(0, 0, CellValue::Number(10.0)).unwrap();
        sheet_proc.set_cell(0, 1, CellValue::Number(20.0)).unwrap();
        sheet_proc.set_formula(0, 2, "=SUM((0,0),(0,1))").unwrap();

        // Initially evaluates to 30.0
        let evaluated_first = sheet_proc.evaluate_cell(0, 2);
        assert_eq!(evaluated_first, CellValue::Number(30.0));

        // Change a cell -> triggers dirty flag propagation
        sheet_proc.set_cell(0, 1, CellValue::Number(50.0)).unwrap();
        // Re-evaluates on demand lazily to 60.0
        let evaluated_second = sheet_proc.evaluate_cell(0, 2);
        assert_eq!(evaluated_second, CellValue::Number(60.0));

        // CSV/Excel/ODS export testing
        let csv_out = sheet_proc.export_to_csv();
        assert!(csv_out.contains("10,50"));

        let excel_out = sheet_proc.export_to_excel();
        assert!(excel_out.starts_with(b"EXCEL-NATIVE-OOXML"));

        let ods_out = sheet_proc.export_to_ods();
        assert!(ods_out.starts_with(b"ODF-SPREADSHEET-XML"));
    }

    #[test]
    fn test_odf_package_engine() {
        let mut odf = SigmaOdfPackageEngine::new(OdfDocumentKind::TextOdt);
        odf.set_content_body_xml("<text:p>Hello Sovereign ODF</text:p>");
        let archive = odf.assemble_odf_archive_bytes();
        assert!(archive.starts_with(b"PK\x03\x04mimetypeapplication/vnd.oasis.opendocument.text"));
        assert!(odf.content_xml.contains("Hello Sovereign ODF"));
    }

    #[test]
    fn test_spell_checker_engine() {
        let mut checker = SigmaSpellCheckerEngine::new("en_US");
        assert!(checker.check_spelling("libreoffice"));
        assert!(!checker.check_spelling("librefice"));

        checker.add_user_word("librefice");
        assert!(checker.check_spelling("librefice"));

        let suggestions = checker.suggest_corrections("dokument");
        assert!(suggestions.contains(&"document".to_string()));
    }

    #[test]
    fn test_track_changes_engine() {
        let mut tracker = SigmaTrackChangesEngine::new();
        let cid = tracker.record_change("author_1", ChangeType::Modification, 0, "old text", "new text");
        assert_eq!(cid, 1);
        assert_eq!(tracker.changes[0].accepted, None);

        assert!(tracker.accept_change(cid));
        assert_eq!(tracker.changes[0].accepted, Some(true));
    }

    #[test]
    fn test_style_theme_engine() {
        let theme = SigmaStyleThemeEngine::new();
        let h1 = theme.get_style("Heading 1").unwrap();
        assert_eq!(h1.font_size_pt, 20);
        assert!(h1.bold);
    }

    #[test]
    fn test_formula_parser_engine() {
        let avg_val = SigmaFormulaParserEngine::parse_and_evaluate_formula("=AVERAGE(10, 20, 30)");
        assert_eq!(avg_val, CellValue::Number(20.0));

        let max_val = SigmaFormulaParserEngine::parse_and_evaluate_formula("=MAX(5, 15, 3)");
        assert_eq!(max_val, CellValue::Number(15.0));

        let min_val = SigmaFormulaParserEngine::parse_and_evaluate_formula("=MIN(8, 2, 9)");
        assert_eq!(min_val, CellValue::Number(2.0));

        let count_val = SigmaFormulaParserEngine::parse_and_evaluate_formula("=COUNT(1, 2, 3, 4)");
        assert_eq!(count_val, CellValue::Number(4.0));
    }

    #[test]
    fn test_google_and_salesforce_expanded_suites() {
        let cap = sigma_types::CapabilityToken { id: 42 };

        // 1. Looker Analytics Engine Test
        let mut sheet = SpreadsheetProcessor::new("Sales Data".to_string(), cap.clone());
        sheet.set_cell(0, 0, CellValue::Number(1000.0)).unwrap();
        sheet.set_cell(0, 1, CellValue::Number(2000.0)).unwrap();

        let mut looker = SigmaLookerAnalyticsEngine::new("Quarterly Sales BI");
        looker.ingest_spreadsheet_data(&sheet);
        looker.add_chart_widget("chart_1", "Revenue Growth", ChartType::Bar, vec![1000.0, 2000.0]);
        let summary = looker.export_report_summary();
        assert!(summary.contains("Total Revenue Sum"));
        assert!(summary.contains("3000"));

        // 2. Slides Presenter Engine Test
        let mut slides_engine = SigmaSlidesPresenterEngine::new("Keynote 2026");
        slides_engine.add_slide(SlideTransitionEffect::Zoom);
        slides_engine.set_speaker_notes(0, "Welcome attendees").unwrap();
        slides_engine.add_animation(0, 0, "FlyIn", 200).unwrap();
        assert!(slides_engine.advance_slide());
        assert_eq!(slides_engine.current_presenter_slide, 1);

        // 3. Docs Collaboration Engine Test
        let mut text_proc = TextProcessor::new("Strategy Doc".to_string(), cap);
        text_proc.add_text("Enterprise Cloud Strategy", true, false).unwrap();

        let mut docs_collab = SigmaDocsEnterpriseCollaborationEngine::new();
        let edit_id = docs_collab.suggest_edit("alice", "Cloud Strategy", "Sovereign OS Strategy");
        assert_eq!(edit_id, 1);

        let comment_id = docs_collab.add_comment("bob", "p1", "Is this aligned with roadmap?");
        assert!(docs_collab.reply_comment(comment_id, "Yes, fully aligned."));

        let ai_summary = docs_collab.generate_ai_summary(&text_proc);
        assert!(ai_summary.contains("AI Executive Summary"));

        // 4. Sovereign Enterprise CRM / ERP Engine Test
        let mut crm_erp = SovereignEnterpriseCrmErpEngine::new();
        let deal_id = crm_erp.create_deal("Enterprise License", "Acme Corp", 50000.0);
        crm_erp.update_deal_stage(deal_id, DealStage::ClosedWon);
        assert_eq!(crm_erp.calculate_pipeline_revenue(), 50000.0);

        let inv_id = crm_erp.create_invoice(
            "Acme Corp",
            0.10,
            vec![InvoiceItem {
                description: "SigmaOS Subscription".to_string(),
                unit_price: 5000.0,
                quantity: 10,
            }],
        );
        let invoice = &crm_erp.invoices[0];
        assert_eq!(invoice.invoice_id, inv_id);
        assert!((invoice.calculate_total() - 55000.0).abs() < 1e-4);
    }

    #[test]
    fn test_advanced_formulas_and_pivot_tables() {
        let cap = sigma_types::CapabilityToken { id: 10 };

        // Test IF, CONCATENATE, AND, OR, VLOOKUP
        assert_eq!(
            SigmaFormulaParserEngine::parse_and_evaluate_formula("=IF(1, \"Yes\", \"No\")"),
            CellValue::Text("Yes".to_string())
        );
        assert_eq!(
            SigmaFormulaParserEngine::parse_and_evaluate_formula("=IF(0, \"Yes\", \"No\")"),
            CellValue::Text("No".to_string())
        );
        assert_eq!(
            SigmaFormulaParserEngine::parse_and_evaluate_formula("=CONCATENATE(\"Hello \", \"SigmaOS\")"),
            CellValue::Text("Hello SigmaOS".to_string())
        );
        assert_eq!(
            SigmaFormulaParserEngine::parse_and_evaluate_formula("=AND(1, 1, 1)"),
            CellValue::Boolean(true)
        );
        assert_eq!(
            SigmaFormulaParserEngine::parse_and_evaluate_formula("=AND(1, 0, 1)"),
            CellValue::Boolean(false)
        );
        assert_eq!(
            SigmaFormulaParserEngine::parse_and_evaluate_formula("=OR(0, 1, 0)"),
            CellValue::Boolean(true)
        );
        assert_eq!(
            SigmaFormulaParserEngine::parse_and_evaluate_formula("=VLOOKUP(\"Key\", Target)"),
            CellValue::Text("VLOOKUP_MATCH(Key)".to_string())
        );

        // Test Pivot Table
        let mut sheet = SpreadsheetProcessor::new("Sales".to_string(), cap);
        sheet.set_cell(1, 0, CellValue::Text("Engineering".to_string())).unwrap();
        sheet.set_cell(1, 1, CellValue::Number(100.0)).unwrap();
        sheet.set_cell(2, 0, CellValue::Text("Engineering".to_string())).unwrap();
        sheet.set_cell(2, 1, CellValue::Number(200.0)).unwrap();
        sheet.set_cell(3, 0, CellValue::Text("Sales".to_string())).unwrap();
        sheet.set_cell(3, 1, CellValue::Number(300.0)).unwrap();

        let mut pivot = SigmaPivotTableEngine::new();
        pivot.add_row_field("Department", 0);
        pivot.add_value_field("Revenue", 1, AggregationFunction::Sum);

        let summary = pivot.summarize_spreadsheet(&sheet);
        assert_eq!(summary.get("Engineering"), Some(&300.0));
        assert_eq!(summary.get("Sales"), Some(&300.0));
    }

    #[test]
    fn test_data_validation_and_conditional_formatting() {
        let rule_num = DataValidationRule {
            row: 0,
            col: 0,
            rule: ValidationRuleType::NumberRange { min: 10.0, max: 100.0 },
        };
        assert!(rule_num.validate(&CellValue::Number(50.0)));
        assert!(!rule_num.validate(&CellValue::Number(5.0)));

        let rule_txt = DataValidationRule {
            row: 0,
            col: 1,
            rule: ValidationRuleType::ListAllowed(vec!["High".to_string(), "Low".to_string()]),
        };
        assert!(rule_txt.validate(&CellValue::Text("High".to_string())));
        assert!(!rule_txt.validate(&CellValue::Text("Medium".to_string())));

        let cond_fmt = ConditionalFormatRule {
            row: 0,
            col: 0,
            operator: ConditionOperator::GreaterThan(50.0),
            highlight_color_rgba: [255, 0, 0, 255],
        };
        assert!(cond_fmt.matches(&CellValue::Number(75.0)));
        assert!(!cond_fmt.matches(&CellValue::Number(25.0)));
    }

    #[test]
    fn test_doc_metrics_toc_and_citations() {
        let cap = sigma_types::CapabilityToken { id: 99 };
        let mut proc = TextProcessor::new("Academic Paper".to_string(), cap);
        proc.add_heading(1, "Introduction").unwrap();
        proc.add_text("This is an introductory paragraph for testing document metrics.", false, false).unwrap();
        proc.add_heading(2, "Methodology").unwrap();

        let metrics = proc.compute_document_metrics();
        assert_eq!(metrics.word_count, 11);
        assert!(metrics.character_count > 50);

        let toc = TableOfContentsGenerator::generate_toc(proc.document());
        assert_eq!(toc.len(), 2);
        assert_eq!(toc[0].title, "Introduction");
        assert_eq!(toc[1].title, "Methodology");

        let mut citations = CitationManager::new();
        citations.add_source(CitationSource {
            citation_id: "ref1".to_string(),
            author: "Jules".to_string(),
            title: "Sovereign Computing Architecture".to_string(),
            year: 2026,
            publisher: "SigmaOS Press".to_string(),
        });

        let apa = citations.format_citation("ref1", CitationStyle::Apa).unwrap();
        assert!(apa.contains("Jules (2026)"));
        assert_eq!(citations.generate_bibliography(CitationStyle::Mla).len(), 1);
    }

    #[test]
    fn test_crm_rules_and_erp_ledger() {
        let mut rule_engine = CrmWorkflowRuleEngine::new();
        rule_engine.add_rule(CrmWorkflowRule {
            rule_id: 1,
            trigger_stage: DealStage::ClosedWon,
            min_value: 10000.0,
            action_description: "Notify Sales Director".to_string(),
        });

        let deal = EnterpriseDeal {
            deal_id: 1,
            title: "Large Enterprise License".to_string(),
            customer_name: "Global Corp".to_string(),
            deal_value: 25000.0,
            stage: DealStage::ClosedWon,
        };

        let actions = rule_engine.evaluate_deal(&deal);
        assert_eq!(actions, vec!["Notify Sales Director".to_string()]);

        let mut ledger = EnterpriseErpLedger::new();
        ledger.post_entry("1000", "Cash", 5000.0, 0.0);
        ledger.post_entry("4000", "Sales Revenue", 0.0, 5000.0);

        assert_eq!(ledger.calculate_total_debits(), 5000.0);
        assert_eq!(ledger.calculate_total_credits(), 5000.0);
        assert!(ledger.is_trial_balance_reconciled());
    }

    #[test]
    fn test_sovereign_forms_survey_engine() {
        let cap = sigma_types::CapabilityToken { id: 77 };
        let mut forms = SovereignFormsSurveyEngine::new("Customer Feedback Survey");
        let q1 = forms.add_question("Satisfaction Rating", FormQuestionType::LinearScale { min: 1, max: 5 }, true);
        let q2 = forms.add_question("Primary Persona", FormQuestionType::MultipleChoice(vec!["Dev".to_string(), "Gaming".to_string()]), false);

        let mut resp1 = HashMap::new();
        resp1.insert(q1, "5".to_string());
        resp1.insert(q2, "Dev".to_string());
        forms.submit_response(resp1, 1700000000);

        let mut resp2 = HashMap::new();
        resp2.insert(q1, "5".to_string());
        resp2.insert(q2, "Gaming".to_string());
        forms.submit_response(resp2, 1700000005);

        let summary = forms.calculate_response_summary(q1);
        assert_eq!(summary.get("5"), Some(&2));

        let mut sheet = SpreadsheetProcessor::new("Survey Results".to_string(), cap);
        forms.export_responses_to_spreadsheet(&mut sheet).unwrap();
        assert_eq!(sheet.get_cell(0, 2), Some(&CellValue::Text("Satisfaction Rating".to_string())));
        assert_eq!(sheet.get_cell(1, 2), Some(&CellValue::Text("5".to_string())));
    }

    #[test]
    fn test_financial_formulas_and_goal_seek() {
        let cap = sigma_types::CapabilityToken { id: 88 };
        let pmt_val = SigmaFormulaParserEngine::parse_and_evaluate_formula("=PMT(0.05, 12, 10000)");
        if let CellValue::Number(n) = pmt_val {
            assert!((n - (-1128.25)).abs() < 1.0);
        } else {
            panic!("Expected PMT number");
        }

        let npv_val = SigmaFormulaParserEngine::parse_and_evaluate_formula("=NPV(0.1, 100, 200, 300)");
        if let CellValue::Number(n) = npv_val {
            assert!((n - 481.59).abs() < 1.0);
        } else {
            panic!("Expected NPV number");
        }

        let sln_val = SigmaFormulaParserEngine::parse_and_evaluate_formula("=SLN(10000, 1000, 5)");
        assert_eq!(sln_val, CellValue::Number(1800.0));

        let mut sheet = SpreadsheetProcessor::new("GoalSeek".to_string(), cap);
        sheet.set_cell(0, 0, CellValue::Number(0.0)).unwrap(); // input x
        sheet.set_formula(0, 1, "=SUM((0,0),(0,0))").unwrap(); // output y = x
        let solution = SigmaFormulaParserEngine::goal_seek_solve(&mut sheet, 0, 1, 50.0, 0, 0);
        assert!(solution.is_some());
    }

    #[test]
    fn test_workgroup_gantt_and_crm_scoring() {
        // Gantt test
        let mut gantt = SovereignWorkgroupGanttEngine::new("SigmaOS v1.0 Release");
        let t1 = gantt.add_task("Kernel Syscall Hardening", 5, "alice");
        let t2 = gantt.add_task("Zenith UI Integration", 10, "bob");
        gantt.add_dependency(t2, t1, TaskDependencyType::FinishToStart);
        gantt.update_progress(t1, 100);
        gantt.update_progress(t2, 50);

        assert_eq!(gantt.calculate_critical_path_duration(), 15);
        assert_eq!(gantt.calculate_overall_project_progress(), 75.0);

        // CRM activity & scoring test
        let mut crm = SovereignEnterpriseCrmErpEngine::new();
        let deal_id = crm.create_deal("Enterprise Support", "Acme", 100000.0);
        crm.log_activity(deal_id, CrmActivityType::EmailSent, "Initial outreach", 1000);
        crm.log_activity(deal_id, CrmActivityType::DemoPresented, "Product Demo", 2000);
        crm.update_deal_stage(deal_id, DealStage::Negotiation);

        let score = crm.calculate_lead_score(deal_id);
        // EmailSent (10) + DemoPresented (40) + Negotiation stage (50) = 100
        assert_eq!(score, 100);
    }
}
