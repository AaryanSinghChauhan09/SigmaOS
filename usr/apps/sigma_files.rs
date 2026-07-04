// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/apps/sigma_files.rs — Sigma-Files File Manager
//
// Implements a file manager with directory navigation, file operations,
// and basic file management capabilities.
//
// Language: Rust (std for userland applications)

use std::fs;
use std::path::{Path, PathBuf};
use std::io;

// ─── File Entry ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_directory: bool,
    pub size: u64,
    pub modified: String,
}

// ─── File Manager State ───────────────────────────────────────────────────────

pub struct FileManager {
    pub current_path: PathBuf,
    pub entries: Vec<FileEntry>,
    pub selected_index: usize,
    pub show_hidden: bool,
}

impl FileManager {
    pub fn new() -> Self {
        let mut fm = FileManager {
            current_path: PathBuf::from("/home/user"),
            entries: Vec::new(),
            selected_index: 0,
            show_hidden: false,
        };
        fm.refresh_entries();
        fm
    }

    /// Refresh directory entries
    pub fn refresh_entries(&mut self) {
        self.entries.clear();
        
        if let Ok(entries) = fs::read_dir(&self.current_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let name = entry.file_name().to_string_lossy().to_string();
                    
                    // Skip hidden files if not showing them
                    if !self.show_hidden && name.starts_with('.') {
                        continue;
                    }
                    
                    let metadata = entry.metadata();
                    let is_directory = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);
                    let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
                    
                    let modified = metadata
                        .and_then(|m| m.modified())
                        .map(|t| format!("{:?}", t))
                        .unwrap_or_else(|_| "Unknown".to_string());
                    
                    self.entries.push(FileEntry {
                        name,
                        path,
                        is_directory,
                        size,
                        modified,
                    });
                }
            }
        }
        
        // Sort: directories first, then files
        self.entries.sort_by(|a, b| {
            if a.is_directory && !b.is_directory {
                std::cmp::Ordering::Less
            } else if !a.is_directory && b.is_directory {
                std::cmp::Ordering::Greater
            } else {
                a.name.cmp(&b.name)
            }
        });
    }

    /// Navigate to directory
    pub fn navigate(&mut self, path: &Path) -> Result<(), String> {
        if path.is_dir() {
            self.current_path = path.to_path_buf();
            self.refresh_entries();
            self.selected_index = 0;
            Ok(())
        } else {
            Err("Not a directory".to_string())
        }
    }

    /// Navigate up one directory
    pub fn navigate_up(&mut self) {
        if let Some(parent) = self.current_path.parent() {
            self.navigate(parent).ok();
        }
    }

    /// Navigate to home directory
    pub fn navigate_home(&mut self) {
        self.navigate(Path::new("/home/user")).ok();
    }

    /// Get selected entry
    pub fn get_selected(&self) -> Option<&FileEntry> {
        if self.selected_index < self.entries.len() {
            Some(&self.entries[self.selected_index])
        } else {
            None
        }
    }

    /// Select next entry
    pub fn select_next(&mut self) {
        if !self.entries.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.entries.len();
        }
    }

    /// Select previous entry
    pub fn select_previous(&mut self) {
        if !self.entries.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.entries.len() - 1
            } else {
                self.selected_index - 1
            };
        }
    }

    /// Create directory
    pub fn create_directory(&mut self, name: &str) -> Result<(), String> {
        let path = self.current_path.join(name);
        fs::create_dir(&path).map_err(|e| format!("Failed to create directory: {}", e))?;
        self.refresh_entries();
        Ok(())
    }

    /// Create file
    pub fn create_file(&mut self, name: &str) -> Result<(), String> {
        let path = self.current_path.join(name);
        fs::File::create(&path).map_err(|e| format!("Failed to create file: {}", e))?;
        self.refresh_entries();
        Ok(())
    }

    /// Delete selected entry
    pub fn delete_selected(&mut self) -> Result<(), String> {
        if let Some(entry) = self.get_selected() {
            if entry.is_directory {
                fs::remove_dir_all(&entry.path).map_err(|e| format!("Failed to delete directory: {}", e))?;
            } else {
                fs::remove_file(&entry.path).map_err(|e| format!("Failed to delete file: {}", e))?;
            }
            self.refresh_entries();
            if self.selected_index >= self.entries.len() && !self.entries.is_empty() {
                self.selected_index = self.entries.len() - 1;
            }
            Ok(())
        } else {
            Err("No entry selected".to_string())
        }
    }

    /// Rename selected entry
    pub fn rename_selected(&mut self, new_name: &str) -> Result<(), String> {
        if let Some(entry) = self.get_selected() {
            let new_path = self.current_path.join(new_name);
            fs::rename(&entry.path, &new_path).map_err(|e| format!("Failed to rename: {}", e))?;
            self.refresh_entries();
            Ok(())
        } else {
            Err("No entry selected".to_string())
        }
    }

    /// Copy selected entry
    pub fn copy_selected(&mut self, dest: &Path) -> Result<(), String> {
        if let Some(entry) = self.get_selected() {
            let dest_path = dest.join(&entry.name);
            if entry.is_directory {
                copy_dir(&entry.path, &dest_path).map_err(|e| format!("Failed to copy directory: {}", e))?;
            } else {
                fs::copy(&entry.path, &dest_path).map_err(|e| format!("Failed to copy file: {}", e))?;
            }
            Ok(())
        } else {
            Err("No entry selected".to_string())
        }
    }

    /// Move selected entry
    pub fn move_selected(&mut self, dest: &Path) -> Result<(), String> {
        if let Some(entry) = self.get_selected() {
            let dest_path = dest.join(&entry.name);
            fs::rename(&entry.path, &dest_path).map_err(|e| format!("Failed to move: {}", e))?;
            self.refresh_entries();
            Ok(())
        } else {
            Err("No entry selected".to_string())
        }
    }

    /// Toggle hidden files visibility
    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        self.refresh_entries();
    }

    /// Get current path as string
    pub fn get_current_path(&self) -> String {
        self.current_path.display().to_string()
    }

    /// Get entries for display
    pub fn get_entries(&self) -> &[FileEntry] {
        &self.entries
    }
}

/// Recursively copy directory
fn copy_dir(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

// ─── CLI Interface ───────────────────────────────────────────────────────────

fn main() {
    let mut fm = FileManager::new();
    
    println!("Sigma-Files v0.1 - File Manager");
    println!("Current: {}", fm.get_current_path());
    
    loop {
        println!("\n--- Directory Contents ---");
        for entry in fm.get_entries() {
            let prefix = if entry.is_directory { "[DIR] " } else { "[FILE] " };
            println!("{} {} ({} bytes)", prefix, entry.name, entry.size);
        }
        
        println!("\nCommands: ls, cd <dir>, up, home, mkdir <name>, touch <name>, rm, mv <dest>, cp <dest>, hidden, quit");
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "ls" => {
                fm.refresh_entries();
            }
            "cd" => {
                if let Some(arg) = parts.get(1) {
                    let path = fm.current_path.join(arg);
                    match fm.navigate(&path) {
                        Ok(_) => println!("Changed to: {}", fm.get_current_path()),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "up" => {
                fm.navigate_up();
                println!("Current: {}", fm.get_current_path());
            }
            "home" => {
                fm.navigate_home();
                println!("Current: {}", fm.get_current_path());
            }
            "mkdir" => {
                if let Some(arg) = parts.get(1) {
                    match fm.create_directory(arg) {
                        Ok(_) => println!("Created directory: {}", arg),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "touch" => {
                if let Some(arg) = parts.get(1) {
                    match fm.create_file(arg) {
                        Ok(_) => println!("Created file: {}", arg),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "rm" => {
                match fm.delete_selected() {
                    Ok(_) => println!("Deleted selected entry"),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "mv" => {
                if let Some(arg) = parts.get(1) {
                    let dest = Path::new(arg);
                    match fm.move_selected(dest) {
                        Ok(_) => println!("Moved selected entry"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "cp" => {
                if let Some(arg) = parts.get(1) {
                    let dest = Path::new(arg);
                    match fm.copy_selected(dest) {
                        Ok(_) => println!("Copied selected entry"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "hidden" => {
                fm.toggle_hidden();
                println!("Hidden files: {}", if fm.show_hidden { "visible" } else { "hidden" });
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
