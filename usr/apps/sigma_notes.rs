// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/apps/sigma_notes.rs — Sigma-Notes Note Application
//
// Implements a note-taking application with note creation,
// editing, deletion, and search functionality.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Note Structure ───────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub created: String,
    pub modified: String,
}

// ─── Notes Application State ─────────────────────────────────────────────────

pub struct NotesApp {
    pub notes: HashMap<String, Note>,
    pub selected_note: Option<String>,
    pub search_query: String,
}

impl NotesApp {
    pub fn new() -> Self {
        NotesApp {
            notes: HashMap::new(),
            selected_note: None,
            search_query: String::new(),
        }
    }

    /// Create new note
    pub fn create_note(&mut self, title: &str, content: &str) -> String {
        let note_id = format!("note_{}", self.notes.len());
        
        let note = Note {
            id: note_id.clone(),
            title: title.to_string(),
            content: content.to_string(),
            tags: Vec::new(),
            created: "now".to_string(),
            modified: "now".to_string(),
        };
        
        self.notes.insert(note_id.clone(), note);
        note_id
    }

    /// Update note
    pub fn update_note(&mut self, note_id: &str, title: Option<&str>, content: Option<&str>) -> Result<(), String> {
        if let Some(note) = self.notes.get_mut(note_id) {
            if let Some(new_title) = title {
                note.title = new_title.to_string();
            }
            if let Some(new_content) = content {
                note.content = new_content.to_string();
            }
            note.modified = "now".to_string();
            Ok(())
        } else {
            Err("Note not found".to_string())
        }
    }

    /// Delete note
    pub fn delete_note(&mut self, note_id: &str) -> Result<(), String> {
        if self.notes.remove(note_id).is_some() {
            if self.selected_note.as_ref() == Some(&note_id.to_string()) {
                self.selected_note = None;
            }
            Ok(())
        } else {
            Err("Note not found".to_string())
        }
    }

    /// Select note
    pub fn select_note(&mut self, note_id: &str) {
        if self.notes.contains_key(note_id) {
            self.selected_note = Some(note_id.to_string());
        }
    }

    /// Get selected note
    pub fn get_selected_note(&self) -> Option<&Note> {
        self.selected_note.as_ref()
            .and_then(|id| self.notes.get(id))
    }

    /// Search notes
    pub fn search_notes(&mut self, query: &str) -> Vec<&Note> {
        self.search_query = query.to_lowercase();
        
        if self.search_query.is_empty() {
            self.notes.values().collect()
        } else {
            self.notes.values()
                .filter(|note| {
                    note.title.to_lowercase().contains(&self.search_query) ||
                    note.content.to_lowercase().contains(&self.search_query) ||
                    note.tags.iter().any(|tag| tag.to_lowercase().contains(&self.search_query))
                })
                .collect()
        }
    }

    /// Add tag to note
    pub fn add_tag(&mut self, note_id: &str, tag: &str) -> Result<(), String> {
        if let Some(note) = self.notes.get_mut(note_id) {
            if !note.tags.contains(&tag.to_string()) {
                note.tags.push(tag.to_string());
            }
            Ok(())
        } else {
            Err("Note not found".to_string())
        }
    }

    /// Remove tag from note
    pub fn remove_tag(&mut self, note_id: &str, tag: &str) -> Result<(), String> {
        if let Some(note) = self.notes.get_mut(note_id) {
            note.tags.retain(|t| t != tag);
            Ok(())
        } else {
            Err("Note not found".to_string())
        }
    }

    /// Get all notes
    pub fn get_all_notes(&self) -> Vec<&Note> {
        self.notes.values().collect()
    }

    /// Get notes by tag
    pub fn get_notes_by_tag(&self, tag: &str) -> Vec<&Note> {
        self.notes.values()
            .filter(|note| note.tags.contains(&tag.to_string()))
            .collect()
    }

    /// Get all tags
    pub fn get_all_tags(&self) -> Vec<String> {
        use std::collections::HashSet;
        
        let mut tags: HashSet<String> = HashSet::new();
        for note in self.notes.values() {
            for tag in &note.tags {
                tags.insert(tag.clone());
            }
        }
        
        let mut tag_list: Vec<String> = tags.into_iter().collect();
        tag_list.sort();
        tag_list
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut notes = NotesApp::new();
    
    println!("Sigma-Notes v0.1 - Note Application");
    
    loop {
        println!("\n--- Notes ---");
        for note in notes.get_all_notes() {
            let marker = if notes.selected_note.as_ref() == Some(&note.id) { " >" } else { "  " };
            println!("{}{} - {} ({} tags)", marker, note.title, note.modified, note.tags.len());
        }
        
        println!("\nCommands: new, select <id>, read, edit, delete, search <query>, tags, tag <id> <tag>, untag <id> <tag>, quit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "new" => {
                println!("Create new note");
                print!("Title: ");
                std::io::stdout().flush().unwrap();
                let mut title = String::new();
                std::io::stdin().read_line(&mut title).unwrap();
                
                print!("Content (end with . on a line): ");
                std::io::stdout().flush().unwrap();
                let mut content = String::new();
                loop {
                    let mut line = String::new();
                    std::io::stdin().read_line(&mut line).unwrap();
                    if line.trim() == "." {
                        break;
                    }
                    content.push_str(&line);
                }
                
                let note_id = notes.create_note(title.trim(), &content);
                notes.select_note(&note_id);
                println!("Note created: {}", note_id);
            }
            "select" => {
                if let Some(arg) = parts.get(1) {
                    notes.select_note(arg);
                    println!("Selected: {}", arg);
                }
            }
            "read" => {
                if let Some(note) = notes.get_selected_note() {
                    println!("\n--- Note ---");
                    println!("Title: {}", note.title);
                    println!("Created: {}", note.created);
                    println!("Modified: {}", note.modified);
                    println!("Tags: {}", note.tags.join(", "));
                    println!("\n{}", note.content);
                } else {
                    println!("No note selected");
                }
            }
            "edit" => {
                if let Some(note) = notes.get_selected_note() {
                    println!("Edit note: {}", note.title);
                    print!("New title (leave empty to keep): ");
                    std::io::stdout().flush().unwrap();
                    let mut title = String::new();
                    std::io::stdin().read_line(&mut title).unwrap();
                    
                    print!("New content (end with . on a line): ");
                    std::io::stdout().flush().unwrap();
                    let mut content = String::new();
                    loop {
                        let mut line = String::new();
                        std::io::stdin().read_line(&mut line).unwrap();
                        if line.trim() == "." {
                            break;
                        }
                        content.push_str(&line);
                    }
                    
                    let title_arg = if title.trim().is_empty() { None } else { Some(title.trim()) };
                    let content_arg = if content.trim().is_empty() { None } else { Some(&content) };
                    
                    match notes.update_note(&note.id, title_arg, content_arg) {
                        Ok(_) => println!("Note updated"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                } else {
                    println!("No note selected");
                }
            }
            "delete" => {
                if let Some(note_id) = &notes.selected_note {
                    match notes.delete_note(note_id) {
                        Ok(_) => println!("Note deleted"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                } else {
                    println!("No note selected");
                }
            }
            "search" => {
                if let Some(arg) = parts.get(1) {
                    let query = parts[1..].join(" ");
                    println!("--- Search Results ---");
                    for note in notes.search_notes(&query) {
                        println!("{} - {}", note.title, note.id);
                    }
                }
            }
            "tags" => {
                println!("--- All Tags ---");
                for tag in notes.get_all_tags() {
                    println!("{}", tag);
                }
            }
            "tag" => {
                if parts.len() >= 3 {
                    let note_id = parts[1];
                    let tag = parts[2];
                    match notes.add_tag(note_id, tag) {
                        Ok(_) => println!("Tag added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "untag" => {
                if parts.len() >= 3 {
                    let note_id = parts[1];
                    let tag = parts[2];
                    match notes.remove_tag(note_id, tag) {
                        Ok(_) => println!("Tag removed"),
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
