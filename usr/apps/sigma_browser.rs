// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/apps/sigma_browser.rs — Sigma-Browser Web Browser (Stub)
//
// Implements a minimal web browser stub with basic navigation,
// bookmark management, and history tracking.
// Note: This is NOT a full browser engine implementation.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Browser State ───────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Bookmark {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub url: String,
    pub title: String,
    pub timestamp: String,
}

pub struct Browser {
    pub current_url: String,
    pub current_title: String,
    pub bookmarks: Vec<Bookmark>,
    pub history: Vec<HistoryEntry>,
    pub history_index: usize,
    pub initialized: bool,
}

impl Browser {
    pub fn new() -> Self {
        let mut browser = Browser {
            current_url: "sigma://home".to_string(),
            current_title: "SigmaOS Home".to_string(),
            bookmarks: Vec::new(),
            history: Vec::new(),
            history_index: 0,
            initialized: false,
        };
        
        browser.init();
        browser
    }

    /// Initialize browser with default bookmarks
    pub fn init(&mut self) {
        // Add default bookmarks
        self.bookmarks.push(Bookmark {
            title: "SigmaOS Home".to_string(),
            url: "sigma://home".to_string(),
        });
        
        self.bookmarks.push(Bookmark {
            title: "SigmaOS Documentation".to_string(),
            url: "https://sigmaos.dev/docs".to_string(),
        });
        
        self.bookmarks.push(Bookmark {
            title: "SigmaOS GitHub".to_string(),
            url: "https://github.com/AaryanSinghChauhan09/SigmaOS".to_string(),
        });
        
        self.initialized = true;
    }

    /// Navigate to URL
    pub fn navigate(&mut self, url: &str) {
        // Add current page to history if different
        if self.current_url != url {
            // Trim history after current index
            if self.history_index < self.history.len() {
                self.history.truncate(self.history_index + 1);
            }
            
            self.history.push(HistoryEntry {
                url: url.to_string(),
                title: self.current_title.clone(),
                timestamp: "now".to_string(),
            });
            
            self.history_index = self.history.len() - 1;
        }
        
        self.current_url = url.to_string();
        self.current_title = self.extract_title(url);
    }

    /// Extract title from URL (stub implementation)
    fn extract_title(&self, url: &str) -> String {
        if url.starts_with("sigma://") {
            match url {
                "sigma://home" => "SigmaOS Home".to_string(),
                "sigma://settings" => "Settings".to_string(),
                "sigma://bookmarks" => "Bookmarks".to_string(),
                "sigma://history" => "History".to_string(),
                _ => "SigmaOS Page".to_string(),
            }
        } else {
            // Extract domain as title
            let domain = url.replace("https://", "")
                           .replace("http://", "")
                           .split('/')
                           .next()
                           .unwrap_or("Unknown");
            domain.to_string()
        }
    }

    /// Navigate back in history
    pub fn back(&mut self) -> bool {
        if self.history_index > 0 {
            self.history_index -= 1;
            if let Some(entry) = self.history.get(self.history_index) {
                self.current_url = entry.url.clone();
                self.current_title = entry.title.clone();
                return true;
            }
        }
        false
    }

    /// Navigate forward in history
    pub fn forward(&mut self) -> bool {
        if self.history_index + 1 < self.history.len() {
            self.history_index += 1;
            if let Some(entry) = self.history.get(self.history_index) {
                self.current_url = entry.url.clone();
                self.current_title = entry.title.clone();
                return true;
            }
        }
        false
    }

    /// Reload current page
    pub fn reload(&mut self) {
        // In a real implementation, this would re-fetch the page
        // For stub, just keep current state
    }

    /// Add bookmark
    pub fn add_bookmark(&mut self) {
        let bookmark = Bookmark {
            title: self.current_title.clone(),
            url: self.current_url.clone(),
        };
        
        // Check if already bookmarked
        if !self.bookmarks.iter().any(|b| b.url == self.current_url) {
            self.bookmarks.push(bookmark);
        }
    }

    /// Remove bookmark
    pub fn remove_bookmark(&mut self, index: usize) {
        if index < self.bookmarks.len() {
            self.bookmarks.remove(index);
        }
    }

    /// Get bookmarks
    pub fn get_bookmarks(&self) -> &[Bookmark] {
        &self.bookmarks
    }

    /// Get history
    pub fn get_history(&self) -> &[HistoryEntry] {
        &self.history
    }

    /// Get current URL
    pub fn get_url(&self) -> &str {
        &self.current_url
    }

    /// Get current title
    pub fn get_title(&self) -> &str {
        &self.current_title
    }
}

// ─── CLI Interface ───────────────────────────────────────────────────────────

fn main() {
    let mut browser = Browser::new();
    
    println!("Sigma-Browser v0.1 - Web Browser (Stub)");
    println!("Current: {} - {}", browser.get_url(), browser.get_title());
    
    loop {
        println!("\nCommands: open <url>, back, forward, reload, bookmark, bookmarks, history, quit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "open" => {
                if let Some(arg) = parts.get(1) {
                    browser.navigate(arg);
                    println!("Navigated to: {} - {}", browser.get_url(), browser.get_title());
                }
            }
            "back" => {
                if browser.back() {
                    println!("Back: {} - {}", browser.get_url(), browser.get_title());
                } else {
                    println!("No history to go back");
                }
            }
            "forward" => {
                if browser.forward() {
                    println!("Forward: {} - {}", browser.get_url(), browser.get_title());
                } else {
                    println!("No history to go forward");
                }
            }
            "reload" => {
                browser.reload();
                println!("Reloaded: {} - {}", browser.get_url(), browser.get_title());
            }
            "bookmark" => {
                browser.add_bookmark();
                println!("Added bookmark: {}", browser.get_title());
            }
            "bookmarks" => {
                println!("--- Bookmarks ---");
                for (i, bm) in browser.get_bookmarks().iter().enumerate() {
                    println!("{}. {} - {}", i + 1, bm.title, bm.url);
                }
            }
            "history" => {
                println!("--- History ---");
                for (i, entry) in browser.get_history().iter().enumerate() {
                    let marker = if i == browser.history_index { " >" } else { "  " };
                    println!("{}{}. {} - {}", marker, i + 1, entry.title, entry.url);
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
