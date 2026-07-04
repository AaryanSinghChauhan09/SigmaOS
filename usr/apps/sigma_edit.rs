// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/apps/sigma_edit.rs — Sigma-Edit Text Editor
//
// Implements a minimal text editor with basic editing capabilities,
// file operations, and syntax highlighting support.
//
// Language: Rust (std for userland applications)

use std::fs;
use std::io::{self, Write};

// ─── Editor State ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Editor {
    pub filename: Option<String>,
    pub content: String,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub modified: bool,
    pub line_numbers: bool,
    pub syntax_highlighting: bool,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            filename: None,
            content: String::new(),
            cursor_line: 0,
            cursor_col: 0,
            modified: false,
            line_numbers: true,
            syntax_highlighting: true,
        }
    }

    /// Open a file
    pub fn open_file(&mut self, filename: &str) -> Result<(), String> {
        match fs::read_to_string(filename) {
            Ok(content) => {
                self.content = content;
                self.filename = Some(filename.to_string());
                self.cursor_line = 0;
                self.cursor_col = 0;
                self.modified = false;
                Ok(())
            }
            Err(e) => Err(format!("Failed to open file: {}", e)),
        }
    }

    /// Save current content to file
    pub fn save_file(&mut self) -> Result<(), String> {
        if let Some(filename) = &self.filename {
            match fs::write(filename, &self.content) {
                Ok(_) => {
                    self.modified = false;
                    Ok(())
                }
                Err(e) => Err(format!("Failed to save file: {}", e)),
            }
        } else {
            Err("No filename specified".to_string())
        }
    }

    /// Save content to a new file
    pub fn save_file_as(&mut self, filename: &str) -> Result<(), String> {
        match fs::write(filename, &self.content) {
            Ok(_) => {
                self.filename = Some(filename.to_string());
                self.modified = false;
                Ok(())
            }
            Err(e) => Err(format!("Failed to save file: {}", e)),
        }
    }

    /// Insert character at cursor position
    pub fn insert_char(&mut self, c: char) {
        let lines: Vec<&str> = self.content.lines().collect();
        
        if self.cursor_line < lines.len() {
            let line = lines[self.cursor_line];
            let mut new_line = String::from(line);
            new_line.insert(self.cursor_col.min(new_line.len()), c);
            
            let mut new_content = String::new();
            for (i, l) in lines.iter().enumerate() {
                if i == self.cursor_line {
                    new_content.push_str(&new_line);
                } else {
                    new_content.push_str(l);
                }
                if i < lines.len() - 1 {
                    new_content.push('\n');
                }
            }
            self.content = new_content;
        } else {
            // Append new line
            if !self.content.is_empty() {
                self.content.push('\n');
            }
            self.content.push(c);
        }
        
        self.cursor_col += 1;
        self.modified = true;
    }

    /// Delete character at cursor position
    pub fn delete_char(&mut self) {
        let lines: Vec<&str> = self.content.lines().collect();
        
        if self.cursor_line < lines.len() && self.cursor_col > 0 {
            let line = lines[self.cursor_line];
            let mut new_line = String::from(line);
            let delete_pos = self.cursor_col - 1;
            
            if delete_pos < new_line.len() {
                new_line.remove(delete_pos);
            }
            
            let mut new_content = String::new();
            for (i, l) in lines.iter().enumerate() {
                if i == self.cursor_line {
                    new_content.push_str(&new_line);
                } else {
                    new_content.push_str(l);
                }
                if i < lines.len() - 1 {
                    new_content.push('\n');
                }
            }
            self.content = new_content;
            self.cursor_col = self.cursor_col.saturating_sub(1);
            self.modified = true;
        }
    }

    /// Move cursor up
    pub fn cursor_up(&mut self) {
        if self.cursor_line > 0 {
            self.cursor_line -= 1;
            let lines: Vec<&str> = self.content.lines().collect();
            if self.cursor_line < lines.len() {
                self.cursor_col = self.cursor_col.min(lines[self.cursor_line].len());
            }
        }
    }

    /// Move cursor down
    pub fn cursor_down(&mut self) {
        let lines: Vec<&str> = self.content.lines().collect();
        if self.cursor_line + 1 < lines.len() {
            self.cursor_line += 1;
            self.cursor_col = self.cursor_col.min(lines[self.cursor_line].len());
        }
    }

    /// Move cursor left
    pub fn cursor_left(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        }
    }

    /// Move cursor right
    pub fn cursor_right(&mut self) {
        let lines: Vec<&str> = self.content.lines().collect();
        if self.cursor_line < lines.len() {
            self.cursor_col = (self.cursor_col + 1).min(lines[self.cursor_line].len());
        }
    }

    /// Get current line
    pub fn get_current_line(&self) -> String {
        let lines: Vec<&str> = self.content.lines().collect();
        if self.cursor_line < lines.len() {
            lines[self.cursor_line].to_string()
        } else {
            String::new()
        }
    }

    /// Get line count
    pub fn line_count(&self) -> usize {
        self.content.lines().count()
    }

    /// Toggle line numbers
    pub fn toggle_line_numbers(&mut self) {
        self.line_numbers = !self.line_numbers;
    }

    /// Toggle syntax highlighting
    pub fn toggle_syntax_highlighting(&mut self) {
        self.syntax_highlighting = !self.syntax_highlighting;
    }

    /// Check if file has unsaved changes
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Get filename
    pub fn get_filename(&self) -> Option<&String> {
        self.filename.as_ref()
    }

    /// Render editor content for display
    pub fn render(&self) -> Vec<String> {
        let lines: Vec<&str> = self.content.lines().collect();
        let mut rendered = Vec::new();
        
        for (i, line) in lines.iter().enumerate() {
            let mut display_line = String::new();
            
            if self.line_numbers {
                display_line.push_str(&format!("{:4} │ ", i + 1));
            }
            
            display_line.push_str(line);
            rendered.push(display_line);
        }
        
        rendered
    }
}

// ─── CLI Interface ───────────────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let mut editor = Editor::new();
    
    if args.len() > 1 {
        match editor.open_file(&args[1]) {
            Ok(_) => println!("Opened: {}", args[1]),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    
    // Simple interactive mode
    println!("Sigma-Edit v0.1 - Type 'help' for commands");
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        match input {
            "quit" | "exit" => {
                if editor.is_modified() {
                    println!("Unsaved changes! Use 'save' first or 'force' to quit.");
                } else {
                    break;
                }
            }
            "force" => break,
            "save" => {
                match editor.save_file() {
                    Ok(_) => println!("File saved."),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            cmd if cmd.starts_with("open ") => {
                let filename = &cmd[5..];
                match editor.open_file(filename) {
                    Ok(_) => println!("Opened: {}", filename),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "help" => {
                println!("Commands:");
                println!("  open <file>   - Open a file");
                println!("  save          - Save current file");
                println!("  quit          - Quit (with unsaved check)");
                println!("  force         - Force quit without saving");
                println!("  help          - Show this help");
            }
            _ => {
                // Treat as text input
                for c in input.chars() {
                    editor.insert_char(c);
                }
                editor.insert_char('\n');
            }
        }
    }
}
