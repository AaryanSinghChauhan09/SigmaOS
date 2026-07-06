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
    pub undo_stack: Vec<String>,
    pub redo_stack: Vec<String>,
    pub search_term: Option<String>,
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
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            search_term: None,
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
        self.save_state_for_undo();
        
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
        self.save_state_for_undo();
        
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
            
            let display_content = if let Some(ref search) = self.search_term {
                line.replace(search, &format!("[{}]", search))
            } else {
                line.to_string()
            };
            
            display_line.push_str(&display_content);
            rendered.push(display_line);
        }
        
        rendered
    }

    /// Save current state for undo
    fn save_state_for_undo(&mut self) {
        self.undo_stack.push(self.content.clone());
        if self.undo_stack.len() > 50 {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
    }

    /// Undo last action
    pub fn undo(&mut self) -> Result<(), String> {
        if let Some(prev_content) = self.undo_stack.pop() {
            self.redo_stack.push(self.content.clone());
            self.content = prev_content;
            self.modified = true;
            Ok(())
        } else {
            Err("Nothing to undo".to_string())
        }
    }

    /// Redo last undone action
    pub fn redo(&mut self) -> Result<(), String> {
        if let Some(next_content) = self.redo_stack.pop() {
            self.undo_stack.push(self.content.clone());
            self.content = next_content;
            self.modified = true;
            Ok(())
        } else {
            Err("Nothing to redo".to_string())
        }
    }

    /// Search for text
    pub fn search(&mut self, term: String) -> Vec<(usize, usize)> {
        self.search_term = Some(term.clone());
        let mut results = Vec::new();
        
        for (line_num, line) in self.content.lines().enumerate() {
            if let Some(col) = line.find(&term) {
                results.push((line_num, col));
            }
        }
        
        results
    }

    /// Clear search
    pub fn clear_search(&mut self) {
        self.search_term = None;
    }

    /// Replace text at cursor position
    pub fn replace(&mut self, old: &str, new: &str) -> Result<usize, String> {
        self.save_state_for_undo();
        let count = self.content.matches(old).count();
        self.content = self.content.replace(old, new);
        self.modified = true;
        Ok(count)
    }

    /// Go to line
    pub fn goto_line(&mut self, line_num: usize) -> Result<(), String> {
        let total_lines = self.line_count();
        if line_num == 0 || line_num > total_lines {
            return Err(format!("Line number out of range (1-{})", total_lines));
        }
        self.cursor_line = line_num - 1;
        self.cursor_col = 0;
        Ok(())
    }

    /// Get word count
    pub fn word_count(&self) -> usize {
        self.content.split_whitespace().count()
    }

    /// Get character count
    pub fn char_count(&self) -> usize {
        self.content.chars().count()
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
                println!("  open <file>      - Open a file");
                println!("  save            - Save current file");
                println!("  saveas <file>   - Save to new file");
                println!("  undo            - Undo last action");
                println!("  redo            - Redo last undone action");
                println!("  search <term>   - Search for text");
                println!("  clear           - Clear search");
                println!("  replace <old> <new> - Replace text");
                println!("  goto <line>     - Go to line number");
                println!("  stats           - Show file statistics");
                println!("  quit            - Quit (with unsaved check)");
                println!("  force           - Force quit without saving");
                println!("  help            - Show this help");
            }
            "undo" => {
                match editor.undo() {
                    Ok(_) => println!("Undone"),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "redo" => {
                match editor.redo() {
                    Ok(_) => println!("Redone"),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            cmd if cmd.starts_with("search ") => {
                let term = &cmd[7..];
                let results = editor.search(term.to_string());
                println!("Found {} occurrences", results.len());
                for (line, col) in results {
                    println!("  Line {}, Column {}", line + 1, col);
                }
            }
            "clear" => {
                editor.clear_search();
                println!("Search cleared");
            }
            cmd if cmd.starts_with("replace ") => {
                let parts: Vec<&str> = cmd[8..].splitn(2, ' ').collect();
                if parts.len() == 2 {
                    match editor.replace(parts[0], parts[1]) {
                        Ok(count) => println!("Replaced {} occurrences", count),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            cmd if cmd.starts_with("goto ") => {
                let line_str = &cmd[5..];
                if let Ok(line_num) = line_str.parse::<usize>() {
                    match editor.goto_line(line_num) {
                        Ok(_) => println!("Moved to line {}", line_num),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "stats" => {
                println!("--- File Statistics ---");
                println!("Lines: {}", editor.line_count());
                println!("Words: {}", editor.word_count());
                println!("Characters: {}", editor.char_count());
                println!("Modified: {}", editor.is_modified());
                if let Some(filename) = editor.get_filename() {
                    println!("File: {}", filename);
                }
            }
            cmd if cmd.starts_with("saveas ") => {
                let filename = &cmd[7..];
                match editor.save_file_as(filename) {
                    Ok(_) => println!("Saved as: {}", filename),
                    Err(e) => eprintln!("Error: {}", e),
                }
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
