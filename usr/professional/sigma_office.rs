// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/professional/sigma_office.rs — Sigma Multilingual Office Suite
//
// Implements word processing and spreadsheet applications with Indian
// language support (Hindi, Gujarati, Tamil, Bengali, etc.)
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Office Types ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    English,
    Hindi,
    Gujarati,
    Tamil,
    Bengali,
    Marathi,
    Telugu,
    Kannada,
    Malayalam,
    Punjabi,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DocumentType {
    WordProcessor,
    Spreadsheet,
    Presentation,
}

#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub doc_type: DocumentType,
    pub language: Language,
    pub content: String,
    pub created_at: String,
    pub modified_at: String,
}

#[derive(Debug, Clone)]
pub struct SpreadsheetCell {
    pub row: u32,
    pub column: u32,
    pub value: String,
    pub formula: Option<String>,
    pub data_type: String,
}

#[derive(Debug, Clone)]
pub struct Spreadsheet {
    pub id: String,
    pub name: String,
    pub rows: u32,
    pub columns: u32,
    pub cells: HashMap<String, SpreadsheetCell>,
}

// ─── Multilingual Office Suite ───────────────────────────────────────────────

pub struct OfficeSuite {
    pub documents: HashMap<String, Document>,
    pub spreadsheets: HashMap<String, Spreadsheet>,
    pub translations: HashMap<String, HashMap<String, String>>,
    pub current_language: Language,
}

impl OfficeSuite {
    pub fn new() -> Self {
        let mut suite = OfficeSuite {
            documents: HashMap::new(),
            spreadsheets: HashMap::new(),
            translations: HashMap::new(),
            current_language: Language::English,
        };
        
        suite.init_translations();
        suite
    }

    /// Initialize translations for common UI elements
    fn init_translations(&mut self) {
        // English (default)
        let mut en = HashMap::new();
        en.insert("file".to_string(), "File".to_string());
        en.insert("edit".to_string(), "Edit".to_string());
        en.insert("view".to_string(), "View".to_string());
        en.insert("insert".to_string(), "Insert".to_string());
        en.insert("format".to_string(), "Format".to_string());
        en.insert("tools".to_string(), "Tools".to_string());
        en.insert("help".to_string(), "Help".to_string());
        en.insert("new".to_string(), "New".to_string());
        en.insert("open".to_string(), "Open".to_string());
        en.insert("save".to_string(), "Save".to_string());
        en.insert("print".to_string(), "Print".to_string());
        self.translations.insert("en".to_string(), en);

        // Hindi
        let mut hi = HashMap::new();
        hi.insert("file".to_string(), "फ़ाइल".to_string());
        hi.insert("edit".to_string(), "संपादन".to_string());
        hi.insert("view".to_string(), "दृश्य".to_string());
        hi.insert("insert".to_string(), "सम्मिलित करें".to_string());
        hi.insert("format".to_string(), "प्रारूप".to_string());
        hi.insert("tools".to_string(), "उपकरण".to_string());
        hi.insert("help".to_string(), "सहायता".to_string());
        hi.insert("new".to_string(), "नया".to_string());
        hi.insert("open".to_string(), "खोलें".to_string());
        hi.insert("save".to_string(), "सहेजें".to_string());
        hi.insert("print".to_string(), "प्रिंट".to_string());
        self.translations.insert("hi".to_string(), hi);

        // Gujarati
        let mut gu = HashMap::new();
        gu.insert("file".to_string(), "ફાઇલ".to_string());
        gu.insert("edit".to_string(), "સંપાદન".to_string());
        gu.insert("view".to_string(), "દૃશ્ય".to_string());
        gu.insert("insert".to_string(), "દાખલ કરો".to_string());
        gu.insert("format".to_string(), "ફોર્મેટ".to_string());
        gu.insert("tools".to_string(), "સાધનો".to_string());
        gu.insert("help".to_string(), "મદદ".to_string());
        gu.insert("new".to_string(), "નવું".to_string());
        gu.insert("open".to_string(), "ખોલો".to_string());
        gu.insert("save".to_string(), "સાચવો".to_string());
        gu.insert("print".to_string(), "પ્રિન્ટ".to_string());
        self.translations.insert("gu".to_string(), gu);

        // Tamil
        let mut ta = HashMap::new();
        ta.insert("file".to_string(), "கோப்பு".to_string());
        ta.insert("edit".to_string(), "திருத்து".to_string());
        ta.insert("view".to_string(), "காட்சி".to_string());
        ta.insert("insert".to_string(), "செருகு".to_string());
        ta.insert("format".to_string(), "வடிவம்".to_string());
        ta.insert("tools".to_string(), "கருவிகள்".to_string());
        ta.insert("help".to_string(), "உதவி".to_string());
        ta.insert("new".to_string(), "புதிய".to_string());
        ta.insert("open".to_string(), "திற".to_string());
        ta.insert("save".to_string(), "சேமி".to_string());
        ta.insert("print".to_string(), "அச்சு".to_string());
        self.translations.insert("ta".to_string(), ta);

        // Bengali
        let mut bn = HashMap::new();
        bn.insert("file".to_string(), "ফাইল".to_string());
        bn.insert("edit".to_string(), "সম্পাদনা".to_string());
        bn.insert("view".to_string(), "দৃশ্য".to_string());
        bn.insert("insert".to_string(), "সন্নিবেশ".to_string());
        bn.insert("format".to_string(), "ফরম্যাট".to_string());
        bn.insert("tools".to_string(), "টুলস".to_string());
        bn.insert("help".to_string(), "সাহায্য".to_string());
        bn.insert("new".to_string(), "নতুন".to_string());
        bn.insert("open".to_string(), "খোলুন".to_string());
        bn.insert("save".to_string(), "সংরক্ষণ".to_string());
        bn.insert("print".to_string(), "প্রিন্ট".to_string());
        self.translations.insert("bn".to_string(), bn);
    }

    /// Set current language
    pub fn set_language(&mut self, language: Language) {
        self.current_language = language;
    }

    /// Get translation for key
    pub fn translate(&self, key: &str) -> String {
        let lang_code = match self.current_language {
            Language::English => "en",
            Language::Hindi => "hi",
            Language::Gujarati => "gu",
            Language::Tamil => "ta",
            Language::Bengali => "bn",
            Language::Marathi => "mr",
            Language::Telugu => "te",
            Language::Kannada => "kn",
            Language::Malayalam => "ml",
            Language::Punjabi => "pa",
        };
        
        if let Some(translations) = self.translations.get(lang_code) {
            translations.get(key).cloned().unwrap_or_else(|| key.to_string())
        } else {
            key.to_string()
        }
    }

    /// Create document
    pub fn create_document(&mut self, title: String, doc_type: DocumentType, content: String) -> Document {
        let document = Document {
            id: format!("doc_{}", self.documents.len()),
            title,
            doc_type,
            language: self.current_language,
            content,
            created_at: "now".to_string(),
            modified_at: "now".to_string(),
        };
        
        self.documents.insert(document.id.clone(), document.clone());
        document
    }

    /// Get document by ID
    pub fn get_document(&self, id: &str) -> Option<&Document> {
        self.documents.get(id)
    }

    /// Create spreadsheet
    pub fn create_spreadsheet(&mut self, name: String, rows: u32, columns: u32) -> Spreadsheet {
        let spreadsheet = Spreadsheet {
            id: format!("sheet_{}", self.spreadsheets.len()),
            name,
            rows,
            columns,
            cells: HashMap::new(),
        };
        
        self.spreadsheets.insert(spreadsheet.id.clone(), spreadsheet.clone());
        spreadsheet
    }

    /// Set cell value
    pub fn set_cell_value(&mut self, sheet_id: &str, row: u32, column: u32, value: String, formula: Option<String>) -> Result<(), String> {
        if let Some(spreadsheet) = self.spreadsheets.get_mut(sheet_id) {
            let cell_key = format!("{}_{}", row, column);
            let cell = SpreadsheetCell {
                row,
                column,
                value: value.clone(),
                formula,
                data_type: if value.parse::<f64>().is_ok() { "number".to_string() } else { "text".to_string() },
            };
            spreadsheet.cells.insert(cell_key, cell);
            Ok(())
        } else {
            Err("Spreadsheet not found".to_string())
        }
    }

    /// Get cell value
    pub fn get_cell_value(&self, sheet_id: &str, row: u32, column: u32) -> Option<&SpreadsheetCell> {
        if let Some(spreadsheet) = self.spreadsheets.get(sheet_id) {
            let cell_key = format!("{}_{}", row, column);
            spreadsheet.cells.get(&cell_key)
        } else {
            None
        }
    }

    /// Evaluate formula with support for basic arithmetic and functions
    pub fn evaluate_formula(&self, formula: &str) -> Result<f64, String> {
        if !formula.starts_with("=") {
            return Err("Formula must start with =".to_string());
        }
        
        let expr = &formula[1..].trim();
        
        // Try direct number
        if let Ok(result) = expr.parse::<f64>() {
            return Ok(result);
        }
        
        // Handle SUM function: SUM(A1:A5)
        if expr.starts_with("SUM(") && expr.ends_with(")") {
            let range = &expr[4..expr.len()-1];
            let parts: Vec<&str> = range.split(':').collect();
            if parts.len() == 2 {
                // Simplified: just return 0 for now as we need cell references
                return Ok(0.0);
            }
        }
        
        // Handle basic arithmetic: +, -, *, /
        let tokens: Vec<&str> = expr.split_whitespace().collect();
        if tokens.len() == 3 {
            let a = tokens[0].parse::<f64>().map_err(|_| "Invalid number".to_string())?;
            let b = tokens[2].parse::<f64>().map_err(|_| "Invalid number".to_string())?;
            
            match tokens[1] {
                "+" => Ok(a + b),
                "-" => Ok(a - b),
                "*" => Ok(a * b),
                "/" => {
                    if b == 0.0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(a / b)
                    }
                }
                "^" => Ok(a.powf(b)),
                _ => Err(format!("Unknown operator: {}", tokens[1]))
            }
        } else {
            Err("Formula format: = <number> <op> <number> or =SUM(range)".to_string())
        }
    }

    /// Get all documents
    pub fn get_all_documents(&self) -> Vec<&Document> {
        self.documents.values().collect()
    }

    /// Get all spreadsheets
    pub fn get_all_spreadsheets(&self) -> Vec<&Spreadsheet> {
        self.spreadsheets.values().collect()
    }

    /// Calculate sum of a column range
    pub fn sum_column(&self, sheet_id: &str, col: u32, start_row: u32, end_row: u32) -> Result<f64, String> {
        if let Some(spreadsheet) = self.spreadsheets.get(sheet_id) {
            let mut sum = 0.0;
            for row in start_row..=end_row {
                let cell_key = format!("{}_{}", row, col);
                if let Some(cell) = spreadsheet.cells.get(&cell_key) {
                    if let Ok(value) = cell.value.parse::<f64>() {
                        sum += value;
                    }
                }
            }
            Ok(sum)
        } else {
            Err("Spreadsheet not found".to_string())
        }
    }

    /// Calculate average of a column range
    pub fn avg_column(&self, sheet_id: &str, col: u32, start_row: u32, end_row: u32) -> Result<f64, String> {
        let sum = self.sum_column(sheet_id, col, start_row, end_row)?;
        let count = (end_row - start_row + 1) as f64;
        if count > 0.0 {
            Ok(sum / count)
        } else {
            Err("Invalid range".to_string())
        }
    }

    /// Export document to file
    pub fn export_document(&self, doc_id: &str, filename: &str) -> Result<(), String> {
        if let Some(doc) = self.documents.get(doc_id) {
            std::fs::write(filename, &doc.content)
                .map_err(|e| format!("Failed to export: {}", e))
        } else {
            Err("Document not found".to_string())
        }
    }

    /// Import document from file
    pub fn import_document(&mut self, filename: &str, title: String) -> Result<Document, String> {
        let content = std::fs::read_to_string(filename)
            .map_err(|e| format!("Failed to import: {}", e))?;
        
        let doc = self.create_document(title, DocumentType::WordProcessor, content);
        Ok(doc)
    }

    /// Get language name
    pub fn get_language_name(&self, language: Language) -> &str {
        match language {
            Language::English => "English",
            Language::Hindi => "हिन्दी (Hindi)",
            Language::Gujarati => "ગુજરાતી (Gujarati)",
            Language::Tamil => "தமிழ் (Tamil)",
            Language::Bengali => "বাংলা (Bengali)",
            Language::Marathi => "मराठी (Marathi)",
            Language::Telugu => "తెలుగు (Telugu)",
            Language::Kannada => "ಕನ್ನಡ (Kannada)",
            Language::Malayalam => "മലയാളം (Malayalam)",
            Language::Punjabi => "ਪੰਜਾਬੀ (Punjabi)",
        }
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut suite = OfficeSuite::new();
    
    println!("Sigma Multilingual Office Suite v0.1 - Indian Language Support");
    
    loop {
        println!("\n--- Current Language: {} ---", suite.get_language_name(suite.current_language));
        println!("--- Menu (Translated) ---");
        println!("{}: {}", suite.translate("file"), suite.translate("file"));
        println!("{}: {}", suite.translate("edit"), suite.translate("edit"));
        println!("{}: {}", suite.translate("view"), suite.translate("view"));
        println!("{}: {}", suite.translate("insert"), suite.translate("insert"));
        
        println!("\nCommands: lang <code>, new_doc <title> <type>, docs, doc <id>, new_sheet <name> <rows> <cols>, set_cell <sheet> <row> <col> <value>, get_cell <sheet> <row> <col>, sheets, quit");
        println!("Languages: en, hi, gu, ta, bn, mr, te, kn, ml, pa");
        println!("Document Types: word, spreadsheet, presentation");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "lang" => {
                if let Some(arg) = parts.get(1) {
                    let language = match *arg {
                        "en" => Language::English,
                        "hi" => Language::Hindi,
                        "gu" => Language::Gujarati,
                        "ta" => Language::Tamil,
                        "bn" => Language::Bengali,
                        "mr" => Language::Marathi,
                        "te" => Language::Telugu,
                        "kn" => Language::Kannada,
                        "ml" => Language::Malayalam,
                        "pa" => Language::Punjabi,
                        _ => {
                            println!("Unknown language code");
                            continue;
                        }
                    };
                    suite.set_language(language);
                    println!("Language changed to {}", suite.get_language_name(language));
                }
            }
            "new_doc" => {
                if parts.len() >= 3 {
                    let title = parts[1].to_string();
                    let doc_type = match parts[2] {
                        "word" => DocumentType::WordProcessor,
                        "spreadsheet" => DocumentType::Spreadsheet,
                        "presentation" => DocumentType::Presentation,
                        _ => {
                            println!("Unknown document type");
                            continue;
                        }
                    };
                    let doc = suite.create_document(title, doc_type, String::new());
                    println!("Document created: {}", doc.id);
                }
            }
            "docs" => {
                println!("--- All Documents ---");
                for doc in suite.get_all_documents() {
                    println!("{} - {} ({})", doc.id, doc.title, suite.get_language_name(doc.language));
                }
            }
            "doc" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(doc) = suite.get_document(arg) {
                        println!("--- Document ---");
                        println!("Title: {}", doc.title);
                        println!("Type: {:?}", doc.doc_type);
                        println!("Language: {}", suite.get_language_name(doc.language));
                        println!("Content: {}", doc.content);
                    }
                }
            }
            "new_sheet" => {
                if parts.len() >= 4 {
                    let name = parts[1].to_string();
                    if let (Ok(rows), Ok(cols)) = (parts[2].parse::<u32>(), parts[3].parse::<u32>()) {
                        let sheet = suite.create_spreadsheet(name, rows, cols);
                        println!("Spreadsheet created: {}", sheet.id);
                    }
                }
            }
            "set_cell" => {
                if parts.len() >= 5 {
                    let sheet_id = parts[1];
                    if let (Ok(row), Ok(col)) = (parts[2].parse::<u32>(), parts[3].parse::<u32>()) {
                        let value = parts[4..].join(" ");
                        let formula = if value.starts_with("=") { Some(value.clone()) } else { None };
                        match suite.set_cell_value(sheet_id, row, col, value, formula) {
                            Ok(_) => println!("Cell set"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "get_cell" => {
                if parts.len() >= 4 {
                    let sheet_id = parts[1];
                    if let (Ok(row), Ok(col)) = (parts[2].parse::<u32>(), parts[3].parse::<u32>()) {
                        if let Some(cell) = suite.get_cell_value(sheet_id, row, col) {
                            println!("Cell ({}, {}): {}", cell.row, cell.column, cell.value);
                        }
                    }
                }
            }
            "sheets" => {
                println!("--- All Spreadsheets ---");
                for sheet in suite.get_all_spreadsheets() {
                    println!("{} - {} ({}x{})", sheet.id, sheet.name, sheet.rows, sheet.columns);
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
