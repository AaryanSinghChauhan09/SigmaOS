// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/education/sigma_whiteboard.rs — Sigma Digital Whiteboard (OpenBoard)
//
// Implements OpenBoard-style digital whiteboard with drawing tools,
// multimedia support, annotation, and export capabilities.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Whiteboard Types ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct DrawingElement {
    pub id: String,
    pub element_type: String,  // pen, line, rectangle, circle, text, image
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub color: String,
    pub line_width: f64,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct Page {
    pub id: String,
    pub name: String,
    pub elements: Vec<DrawingElement>,
    pub background_color: String,
}

#[derive(Debug, Clone)]
pub struct Whiteboard {
    pub id: String,
    pub name: String,
    pub pages: Vec<Page>,
    pub current_page: usize,
    pub created: String,
}

#[derive(Debug, Clone)]
pub struct Tool {
    pub name: String,
    pub tool_type: String,
    pub color: String,
    pub size: f64,
}

// ─── Whiteboard Manager ────────────────────────────────────────────────────

pub struct WhiteboardManager {
    pub whiteboards: HashMap<String, Whiteboard>,
    pub current_whiteboard: Option<String>,
    pub current_tool: Tool,
    pub undo_stack: Vec<String>,
    pub redo_stack: Vec<String>,
}

impl WhiteboardManager {
    pub fn new() -> Self {
        let mut manager = WhiteboardManager {
            whiteboards: HashMap::new(),
            current_whiteboard: None,
            current_tool: Tool {
                name: "Pen".to_string(),
                tool_type: "pen".to_string(),
                color: "#000000".to_string(),
                size: 2.0,
            },
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        };
        
        manager.init_sample_whiteboard();
        manager
    }

    /// Initialize sample whiteboard
    fn init_sample_whiteboard(&mut self) {
        let page = Page {
            id: "page_001".to_string(),
            name: "Page 1".to_string(),
            elements: vec![
                DrawingElement {
                    id: "elem_001".to_string(),
                    element_type: "text".to_string(),
                    x: 100.0,
                    y: 100.0,
                    width: 0.0,
                    height: 0.0,
                    color: "#000000".to_string(),
                    line_width: 2.0,
                    content: "Welcome to Sigma Whiteboard".to_string(),
                },
            ],
            background_color: "#ffffff".to_string(),
        };
        
        let whiteboard = Whiteboard {
            id: "wb_001".to_string(),
            name: "Mathematics Lesson".to_string(),
            pages: vec![page],
            current_page: 0,
            created: "2024-01-15".to_string(),
        };
        
        self.whiteboards.insert(whiteboard.id.clone(), whiteboard);
        self.current_whiteboard = Some("wb_001".to_string());
    }

    /// Create new whiteboard
    pub fn create_whiteboard(&mut self, name: String) -> Whiteboard {
        let page = Page {
            id: format!("page_{}", 0),
            name: "Page 1".to_string(),
            elements: Vec::new(),
            background_color: "#ffffff".to_string(),
        };
        
        let whiteboard = Whiteboard {
            id: format!("wb_{}", self.whiteboards.len()),
            name,
            pages: vec![page],
            current_page: 0,
            created: "now".to_string(),
        };
        
        self.whiteboards.insert(whiteboard.id.clone(), whiteboard.clone());
        self.current_whiteboard = Some(whiteboard.id.clone());
        whiteboard
    }

    /// Add page to whiteboard
    pub fn add_page(&mut self, whiteboard_id: &str) -> Result<(), String> {
        if let Some(wb) = self.whiteboards.get_mut(whiteboard_id) {
            let page = Page {
                id: format!("page_{}", wb.pages.len()),
                name: format!("Page {}", wb.pages.len() + 1),
                elements: Vec::new(),
                background_color: "#ffffff".to_string(),
            };
            wb.pages.push(page);
            Ok(())
        } else {
            Err("Whiteboard not found".to_string())
        }
    }

    /// Add drawing element
    pub fn add_element(&mut self, whiteboard_id: &str, page_index: usize, element: DrawingElement) -> Result<(), String> {
        if let Some(wb) = self.whiteboards.get_mut(whiteboard_id) {
            if page_index < wb.pages.len() {
                wb.pages[page_index].elements.push(element);
                Ok(())
            } else {
                Err("Page index out of bounds".to_string())
            }
        } else {
            Err("Whiteboard not found".to_string())
        }
    }

    /// Delete element
    pub fn delete_element(&mut self, whiteboard_id: &str, page_index: usize, element_id: &str) -> Result<(), String> {
        if let Some(wb) = self.whiteboards.get_mut(whiteboard_id) {
            if page_index < wb.pages.len() {
                if let Some(pos) = wb.pages[page_index].elements.iter().position(|e| e.id == element_id) {
                    wb.pages[page_index].elements.remove(pos);
                    Ok(())
                } else {
                    Err("Element not found".to_string())
                }
            } else {
                Err("Page index out of bounds".to_string())
            }
        } else {
            Err("Whiteboard not found".to_string())
        }
    }

    /// Clear page
    pub fn clear_page(&mut self, whiteboard_id: &str, page_index: usize) -> Result<(), String> {
        if let Some(wb) = self.whiteboards.get_mut(whiteboard_id) {
            if page_index < wb.pages.len() {
                wb.pages[page_index].elements.clear();
                Ok(())
            } else {
                Err("Page index out of bounds".to_string())
            }
        } else {
            Err("Whiteboard not found".to_string())
        }
    }

    /// Switch page
    pub fn switch_page(&mut self, whiteboard_id: &str, page_index: usize) -> Result<(), String> {
        if let Some(wb) = self.whiteboards.get_mut(whiteboard_id) {
            if page_index < wb.pages.len() {
                wb.current_page = page_index;
                Ok(())
            } else {
                Err("Page index out of bounds".to_string())
            }
        } else {
            Err("Whiteboard not found".to_string())
        }
    }

    /// Set tool
    pub fn set_tool(&mut self, tool: Tool) {
        self.current_tool = tool;
    }

    /// Set background color
    pub fn set_background_color(&mut self, whiteboard_id: &str, page_index: usize, color: String) -> Result<(), String> {
        if let Some(wb) = self.whiteboards.get_mut(whiteboard_id) {
            if page_index < wb.pages.len() {
                wb.pages[page_index].background_color = color;
                Ok(())
            } else {
                Err("Page index out of bounds".to_string())
            }
        } else {
            Err("Whiteboard not found".to_string())
        }
    }

    /// Get whiteboard by ID
    pub fn get_whiteboard(&self, id: &str) -> Option<&Whiteboard> {
        self.whiteboards.get(id)
    }

    /// Get all whiteboards
    pub fn get_all_whiteboards(&self) -> Vec<&Whiteboard> {
        self.whiteboards.values().collect()
    }

    /// Switch whiteboard
    pub fn switch_whiteboard(&mut self, id: &str) -> Result<(), String> {
        if self.whiteboards.contains_key(id) {
            self.current_whiteboard = Some(id.to_string());
            Ok(())
        } else {
            Err("Whiteboard not found".to_string())
        }
    }

    /// Export whiteboard (simulated)
    pub fn export(&self, whiteboard_id: &str, format: String) -> Result<String, String> {
        if self.whiteboards.contains_key(whiteboard_id) {
            Ok(format!("{}.{}", whiteboard_id, format))
        } else {
            Err("Whiteboard not found".to_string())
        }
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut manager = WhiteboardManager::new();
    
    println!("Sigma Digital Whiteboard v0.1 - OpenBoard Style");
    
    loop {
        println!("\n--- Whiteboard Status ---");
        if let Some(wb_id) = &manager.current_whiteboard {
            if let Some(wb) = manager.get_whiteboard(wb_id) {
                println!("Current Whiteboard: {}", wb.name);
                println!("Pages: {}", wb.pages.len());
                println!("Current Page: {}", wb.current_page + 1);
                if wb.current_page < wb.pages.len() {
                    println!("Elements: {}", wb.pages[wb.current_page].elements.len());
                }
            }
        }
        println!("Current Tool: {} ({})", manager.current_tool.name, manager.current_tool.color);
        
        println!("\nCommands: create <name>, add_page, add_element <type> <x> <y> <content>, delete_element <id>, clear_page, switch_page <index>, set_tool <name> <type> <color> <size>, set_bg <color>, switch <wb_id>, export <format>, whiteboards, whiteboard <id>, quit");
        println!("Element types: pen, line, rectangle, circle, text, image");
        println!("Export formats: pdf, png, svg");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "create" => {
                if let Some(arg) = parts.get(1) {
                    let wb = manager.create_whiteboard(arg.to_string());
                    println!("Whiteboard created: {}", wb.name);
                }
            }
            "add_page" => {
                if let Some(wb_id) = &manager.current_whiteboard {
                    match manager.add_page(wb_id) {
                        Ok(_) => println!("Page added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "add_element" => {
                if parts.len() >= 5 {
                    let element_type = parts[1].to_string();
                    if let (Ok(x), Ok(y)) = (parts[2].parse::<f64>(), parts[3].parse::<f64>()) {
                        let content = parts[4..].join(" ");
                        let element = DrawingElement {
                            id: format!("elem_{}", rand_id()),
                            element_type,
                            x,
                            y,
                            width: 0.0,
                            height: 0.0,
                            color: manager.current_tool.color.clone(),
                            line_width: manager.current_tool.size,
                            content,
                        };
                        if let Some(wb_id) = &manager.current_whiteboard {
                            if let Some(wb) = manager.get_whiteboard(wb_id) {
                                match manager.add_element(wb_id, wb.current_page, element) {
                                    Ok(_) => println!("Element added"),
                                    Err(e) => eprintln!("Error: {}", e),
                                }
                            }
                        }
                    }
                }
            }
            "delete_element" => {
                if parts.len() >= 2 {
                    if let Some(wb_id) = &manager.current_whiteboard {
                        if let Some(wb) = manager.get_whiteboard(wb_id) {
                            match manager.delete_element(wb_id, wb.current_page, parts[1]) {
                                Ok(_) => println!("Element deleted"),
                                Err(e) => eprintln!("Error: {}", e),
                            }
                        }
                    }
                }
            }
            "clear_page" => {
                if let Some(wb_id) = &manager.current_whiteboard {
                    if let Some(wb) = manager.get_whiteboard(wb_id) {
                        match manager.clear_page(wb_id, wb.current_page) {
                            Ok(_) => println!("Page cleared"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "switch_page" => {
                if let Some(arg) = parts.get(1) {
                    if let Ok(index) = arg.parse::<usize>() {
                        if let Some(wb_id) = &manager.current_whiteboard {
                            match manager.switch_page(wb_id, index - 1) {
                                Ok(_) => println!("Switched to page {}", index),
                                Err(e) => eprintln!("Error: {}", e),
                            }
                        }
                    }
                }
            }
            "set_tool" => {
                if parts.len() >= 5 {
                    let name = parts[1].to_string();
                    let tool_type = parts[2].to_string();
                    let color = parts[3].to_string();
                    if let Ok(size) = parts[4].parse::<f64>() {
                        manager.set_tool(Tool { name, tool_type, color, size });
                        println!("Tool updated");
                    }
                }
            }
            "set_bg" => {
                if parts.len() >= 2 {
                    let color = parts[1].to_string();
                    if let Some(wb_id) = &manager.current_whiteboard {
                        if let Some(wb) = manager.get_whiteboard(wb_id) {
                            match manager.set_background_color(wb_id, wb.current_page, color) {
                                Ok(_) => println!("Background color updated"),
                                Err(e) => eprintln!("Error: {}", e),
                            }
                        }
                    }
                }
            }
            "switch" => {
                if let Some(arg) = parts.get(1) {
                    match manager.switch_whiteboard(arg) {
                        Ok(_) => println!("Switched to whiteboard"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "export" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(wb_id) = &manager.current_whiteboard {
                        match manager.export(wb_id, arg.to_string()) {
                            Ok(path) => println!("Exported to: {}", path),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "whiteboards" => {
                println!("--- All Whiteboards ---");
                for wb in manager.get_all_whiteboards() {
                    println!("{} - {} ({} pages)", wb.id, wb.name, wb.pages.len());
                }
            }
            "whiteboard" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(wb) = manager.get_whiteboard(arg) {
                        println!("--- Whiteboard Details ---");
                        println!("Name: {}", wb.name);
                        println!("Created: {}", wb.created);
                        println!("Pages: {}", wb.pages.len());
                        for (i, page) in wb.pages.iter().enumerate() {
                            println!("  Page {}: {} ({} elements, bg: {})", i + 1, page.name, page.elements.len(), page.background_color);
                        }
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

fn rand_id() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    duration.as_nanos() as u32
}
