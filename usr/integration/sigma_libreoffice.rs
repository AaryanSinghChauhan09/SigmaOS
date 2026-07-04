// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/integration/sigma_libreoffice.rs — Sigma LibreOffice Integration
//
// Implements integration with LibreOffice suite for document editing,
// spreadsheet operations, and presentation management.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── LibreOffice Types ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LibreOfficeApp {
    Writer,
    Calc,
    Impress,
    Draw,
    Math,
    Base,
}

#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,
    pub name: String,
    pub app: LibreOfficeApp,
    pub path: String,
    pub format: String,
    pub is_open: bool,
    pub last_modified: String,
}

#[derive(Debug, Clone)]
pub struct SpreadsheetData {
    pub document_id: String,
    pub sheets: Vec<Sheet>,
    pub active_sheet: String,
}

#[derive(Debug, Clone)]
pub struct Sheet {
    pub name: String,
    pub rows: u32,
    pub columns: u32,
    pub data: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct PresentationSlide {
    pub number: u32,
    pub title: String,
    pub content: String,
    pub notes: String,
}

// ─── LibreOffice Integration Manager ─────────────────────────────────────────

pub struct LibreOfficeManager {
    pub documents: HashMap<String, Document>,
    pub spreadsheet_data: HashMap<String, SpreadsheetData>,
    pub presentations: HashMap<String, Vec<PresentationSlide>>,
    pub current_app: LibreOfficeApp,
}

impl LibreOfficeManager {
    pub fn new() -> Self {
        let mut manager = LibreOfficeManager {
            documents: HashMap::new(),
            spreadsheet_data: HashMap::new(),
            presentations: HashMap::new(),
            current_app: LibreOfficeApp::Writer,
        };
        
        manager.init_sample_documents();
        manager
    }

    /// Initialize sample documents
    fn init_sample_documents(&mut self) {
        // Writer document
        self.documents.insert("doc_001".to_string(), Document {
            id: "doc_001".to_string(),
            name: "Report.odt".to_string(),
            app: LibreOfficeApp::Writer,
            path: "/home/user/Documents/Report.odt".to_string(),
            format: "ODT".to_string(),
            is_open: false,
            last_modified: "2024-01-15".to_string(),
        });

        // Calc spreadsheet
        self.documents.insert("doc_002".to_string(), Document {
            id: "doc_002".to_string(),
            name: "Budget.ods".to_string(),
            app: LibreOfficeApp::Calc,
            path: "/home/user/Documents/Budget.ods".to_string(),
            format: "ODS".to_string(),
            is_open: false,
            last_modified: "2024-01-14".to_string(),
        });

        // Impress presentation
        self.documents.insert("doc_003".to_string(), Document {
            id: "doc_003".to_string(),
            name: "Presentation.odp".to_string(),
            app: LibreOfficeApp::Impress,
            path: "/home/user/Documents/Presentation.odp".to_string(),
            format: "ODP".to_string(),
            is_open: false,
            last_modified: "2024-01-13".to_string(),
        });
    }

    /// Set current LibreOffice app
    pub fn set_app(&mut self, app: LibreOfficeApp) {
        self.current_app = app;
    }

    /// Get app name
    pub fn get_app_name(&self, app: LibreOfficeApp) -> &str {
        match app {
            LibreOfficeApp::Writer => "Writer",
            LibreOfficeApp::Calc => "Calc",
            LibreOfficeApp::Impress => "Impress",
            LibreOfficeApp::Draw => "Draw",
            LibreOfficeApp::Math => "Math",
            LibreOfficeApp::Base => "Base",
        }
    }

    /// Create new document
    pub fn create_document(&mut self, name: String, app: LibreOfficeApp, format: String) -> Document {
        let document = Document {
            id: format!("doc_{}", self.documents.len()),
            name: name.clone(),
            app,
            path: format!("/home/user/Documents/{}", name),
            format,
            is_open: true,
            last_modified: "now".to_string(),
        };
        
        self.documents.insert(document.id.clone(), document.clone());
        document
    }

    /// Open document
    pub fn open_document(&mut self, id: &str) -> Result<(), String> {
        if let Some(doc) = self.documents.get_mut(id) {
            doc.is_open = true;
            doc.last_modified = "now".to_string();
            Ok(())
        } else {
            Err("Document not found".to_string())
        }
    }

    /// Close document
    pub fn close_document(&mut self, id: &str) -> Result<(), String> {
        if let Some(doc) = self.documents.get_mut(id) {
            doc.is_open = false;
            Ok(())
        } else {
            Err("Document not found".to_string())
        }
    }

    /// Get document by ID
    pub fn get_document(&self, id: &str) -> Option<&Document> {
        self.documents.get(id)
    }

    /// Get documents by app
    pub fn get_documents_by_app(&self, app: LibreOfficeApp) -> Vec<&Document> {
        self.documents.values()
            .filter(|d| d.app == app)
            .collect()
    }

    /// Get all documents
    pub fn get_all_documents(&self) -> Vec<&Document> {
        self.documents.values().collect()
    }

    /// Create spreadsheet data
    pub fn create_spreadsheet(&mut self, document_id: String, sheet_name: String, rows: u32, columns: u32) {
        let sheet = Sheet {
            name: sheet_name,
            rows,
            columns,
            data: HashMap::new(),
        };
        
        let spreadsheet_data = SpreadsheetData {
            document_id: document_id.clone(),
            sheets: vec![sheet],
            active_sheet: sheet_name,
        };
        
        self.spreadsheet_data.insert(document_id, spreadsheet_data);
    }

    /// Set cell value in spreadsheet
    pub fn set_cell_value(&mut self, document_id: &str, sheet_name: &str, row: u32, column: u32, value: String) -> Result<(), String> {
        if let Some(spreadsheet) = self.spreadsheet_data.get_mut(document_id) {
            let cell_key = format!("{}_{}_{}", sheet_name, row, column);
            spreadsheet.data.insert(cell_key, value);
            Ok(())
        } else {
            Err("Spreadsheet not found".to_string())
        }
    }

    /// Get cell value from spreadsheet
    pub fn get_cell_value(&self, document_id: &str, sheet_name: &str, row: u32, column: u32) -> Option<String> {
        if let Some(spreadsheet) = self.spreadsheet_data.get(document_id) {
            let cell_key = format!("{}_{}_{}", sheet_name, row, column);
            spreadsheet.data.get(&cell_key).cloned()
        } else {
            None
        }
    }

    /// Create presentation
    pub fn create_presentation(&mut self, document_id: String) {
        self.presentations.insert(document_id, Vec::new());
    }

    /// Add slide to presentation
    pub fn add_slide(&mut self, document_id: &str, slide: PresentationSlide) -> Result<(), String> {
        if let Some(slides) = self.presentations.get_mut(document_id) {
            slides.push(slide);
            Ok(())
        } else {
            Err("Presentation not found".to_string())
        }
    }

    /// Get presentation slides
    pub fn get_slides(&self, document_id: &str) -> Option<&[PresentationSlide]> {
        self.presentations.get(document_id).map(|v| v.as_slice())
    }

    /// Export document to PDF (simulated)
    pub fn export_to_pdf(&self, document_id: &str) -> Result<String, String> {
        if let Some(doc) = self.get_document(document_id) {
            let pdf_path = format!("{}.pdf", doc.path.trim_end_matches(&doc.format));
            Ok(format!("Document exported to: {}", pdf_path))
        } else {
            Err("Document not found".to_string())
        }
    }

    /// Convert document format (simulated)
    pub fn convert_format(&mut self, document_id: &str, new_format: String) -> Result<(), String> {
        if let Some(doc) = self.documents.get_mut(document_id) {
            let old_path = doc.path.clone();
            doc.path = old_path.trim_end_matches(&doc.format).to_string() + &new_format;
            doc.format = new_format;
            doc.last_modified = "now".to_string();
            Ok(())
        } else {
            Err("Document not found".to_string())
        }
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut manager = LibreOfficeManager::new();
    
    println!("Sigma LibreOffice Integration v0.1 - Writer/Calc/Impress/Draw");
    
    loop {
        println!("\n--- Current App: {} ---", manager.get_app_name(manager.current_app));
        
        println!("\nCommands: app <type>, new <name> <format>, open <id>, close <id>, docs, doc <id>, sheet <id> <name> <rows> <cols>, set_cell <id> <sheet> <row> <col> <value>, get_cell <id> <sheet> <row> <col>, create_pres <id>, add_slide <id> <num> <title> <content>, slides <id>, export_pdf <id>, convert <id> <format>, quit");
        println!("Apps: writer, calc, impress, draw, math, base");
        println!("Formats: odt, ods, odp, pdf, docx, xlsx, pptx");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "app" => {
                if let Some(arg) = parts.get(1) {
                    let app = match *arg {
                        "writer" => LibreOfficeApp::Writer,
                        "calc" => LibreOfficeApp::Calc,
                        "impress" => LibreOfficeApp::Impress,
                        "draw" => LibreOfficeApp::Draw,
                        "math" => LibreOfficeApp::Math,
                        "base" => LibreOfficeApp::Base,
                        _ => {
                            println!("Unknown app");
                            continue;
                        }
                    };
                    manager.set_app(app);
                    println!("App changed to {}", manager.get_app_name(app));
                }
            }
            "new" => {
                if parts.len() >= 3 {
                    let name = parts[1].to_string();
                    let format = parts[2].to_string();
                    let doc = manager.create_document(name, manager.current_app, format);
                    println!("Document created: {}", doc.id);
                }
            }
            "open" => {
                if let Some(arg) = parts.get(1) {
                    match manager.open_document(arg) {
                        Ok(_) => println!("Document opened"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "close" => {
                if let Some(arg) = parts.get(1) {
                    match manager.close_document(arg) {
                        Ok(_) => println!("Document closed"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "docs" => {
                println!("--- All Documents ---");
                for doc in manager.get_all_documents() {
                    let status = if doc.is_open { "[OPEN]" } else { "" };
                    println!("{} - {} ({}) {} - {}", doc.id, doc.name, manager.get_app_name(doc.app), status, doc.format);
                }
            }
            "doc" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(doc) = manager.get_document(arg) {
                        println!("--- Document ---");
                        println!("Name: {}", doc.name);
                        println!("App: {}", manager.get_app_name(doc.app));
                        println!("Format: {}", doc.format);
                        println!("Path: {}", doc.path);
                        println!("Status: {}", if doc.is_open { "Open" } else { "Closed" });
                        println!("Last Modified: {}", doc.last_modified);
                    }
                }
            }
            "sheet" => {
                if parts.len() >= 5 {
                    let doc_id = parts[1].to_string();
                    let sheet_name = parts[2].to_string();
                    if let (Ok(rows), Ok(cols)) = (parts[3].parse::<u32>(), parts[4].parse::<u32>()) {
                        manager.create_spreadsheet(doc_id, sheet_name, rows, cols);
                        println!("Spreadsheet created");
                    }
                }
            }
            "set_cell" => {
                if parts.len() >= 6 {
                    let doc_id = parts[1];
                    let sheet_name = parts[2];
                    if let (Ok(row), Ok(col)) = (parts[3].parse::<u32>(), parts[4].parse::<u32>()) {
                        let value = parts[5..].join(" ");
                        match manager.set_cell_value(doc_id, sheet_name, row, col, value) {
                            Ok(_) => println!("Cell value set"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "get_cell" => {
                if parts.len() >= 5 {
                    let doc_id = parts[1];
                    let sheet_name = parts[2];
                    if let (Ok(row), Ok(col)) = (parts[3].parse::<u32>(), parts[4].parse::<u32>()) {
                        if let Some(value) = manager.get_cell_value(doc_id, sheet_name, row, col) {
                            println!("Cell value: {}", value);
                        }
                    }
                }
            }
            "create_pres" => {
                if let Some(arg) = parts.get(1) {
                    manager.create_presentation(arg.to_string());
                    println!("Presentation created");
                }
            }
            "add_slide" => {
                if parts.len() >= 5 {
                    let doc_id = parts[1];
                    if let Ok(number) = parts[2].parse::<u32>() {
                        let title = parts[3].to_string();
                        let content = parts[4..].join(" ");
                        let slide = PresentationSlide {
                            number,
                            title,
                            content,
                            notes: String::new(),
                        };
                        match manager.add_slide(doc_id, slide) {
                            Ok(_) => println!("Slide added"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "slides" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(slides) = manager.get_slides(arg) {
                        println!("--- Presentation Slides ---");
                        for slide in slides {
                            println!("Slide {}: {}", slide.number, slide.title);
                            println!("  Content: {}", slide.content);
                        }
                    }
                }
            }
            "export_pdf" => {
                if let Some(arg) = parts.get(1) {
                    match manager.export_to_pdf(arg) {
                        Ok(msg) => println!("{}", msg),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "convert" => {
                if parts.len() >= 3 {
                    let doc_id = parts[1];
                    let new_format = parts[2].to_string();
                    match manager.convert_format(doc_id, new_format) {
                        Ok(_) => println!("Document converted"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
