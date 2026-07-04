// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/apps/sigma_mail.rs — Sigma-Mail Email Client
//
// Implements an email client with inbox management, composition,
// and basic email handling capabilities.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Email Structures ─────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Email {
    pub id: String,
    pub from: String,
    pub to: String,
    pub subject: String,
    pub body: String,
    pub timestamp: String,
    pub read: bool,
    pub starred: bool,
}

#[derive(Debug, Clone)]
pub struct Folder {
    pub name: String,
    pub emails: Vec<String>, // Email IDs
}

// ─── Mail Client State ───────────────────────────────────────────────────────

pub struct MailClient {
    pub folders: HashMap<String, Folder>,
    pub current_folder: String,
    pub emails: HashMap<String, Email>,
    pub selected_email: Option<String>,
    pub initialized: bool,
}

impl MailClient {
    pub fn new() -> Self {
        let mut client = MailClient {
            folders: HashMap::new(),
            current_folder: "inbox".to_string(),
            emails: HashMap::new(),
            selected_email: None,
            initialized: false,
        };
        
        client.init();
        client
    }

    /// Initialize mail client with default folders
    pub fn init(&mut self) {
        // Create default folders
        self.folders.insert("inbox".to_string(), Folder {
            name: "Inbox".to_string(),
            emails: Vec::new(),
        });
        
        self.folders.insert("sent".to_string(), Folder {
            name: "Sent".to_string(),
            emails: Vec::new(),
        });
        
        self.folders.insert("drafts".to_string(), Folder {
            name: "Drafts".to_string(),
            emails: Vec::new(),
        });
        
        self.folders.insert("trash".to_string(), Folder {
            name: "Trash".to_string(),
            emails: Vec::new(),
        });
        
        self.folders.insert("starred".to_string(), Folder {
            name: "Starred".to_string(),
            emails: Vec::new(),
        });
        
        // Add welcome email
        let welcome_email = Email {
            id: "welcome".to_string(),
            from: "SigmaOS Team <noreply@sigmaos.dev>".to_string(),
            to: "user@sigmaos.dev".to_string(),
            subject: "Welcome to Sigma-Mail!".to_string(),
            body: "Thank you for using SigmaOS!\n\nThis is a welcome email to demonstrate the email client functionality.\n\nBest regards,\nThe SigmaOS Team".to_string(),
            timestamp: "now".to_string(),
            read: false,
            starred: false,
        };
        
        self.emails.insert(welcome_email.id.clone(), welcome_email);
        
        if let Some(inbox) = self.folders.get_mut("inbox") {
            inbox.emails.push("welcome".to_string());
        }
        
        self.initialized = true;
    }

    /// Switch to folder
    pub fn switch_folder(&mut self, folder: &str) -> Result<(), String> {
        if self.folders.contains_key(folder) {
            self.current_folder = folder.to_string();
            self.selected_email = None;
            Ok(())
        } else {
            Err(format!("Folder '{}' not found", folder))
        }
    }

    /// Get emails in current folder
    pub fn get_folder_emails(&self) -> Vec<&Email> {
        if let Some(folder) = self.folders.get(&self.current_folder) {
            folder.emails.iter()
                .filter_map(|id| self.emails.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Select email
    pub fn select_email(&mut self, email_id: &str) {
        self.selected_email = Some(email_id.to_string());
        
        // Mark as read
        if let Some(email) = self.emails.get_mut(email_id) {
            email.read = true;
        }
    }

    /// Get selected email
    pub fn get_selected_email(&self) -> Option<&Email> {
        self.selected_email.as_ref()
            .and_then(|id| self.emails.get(id))
    }

    /// Compose new email
    pub fn compose(&mut self, to: &str, subject: &str, body: &str) -> String {
        let email_id = format!("email_{}", self.emails.len());
        
        let email = Email {
            id: email_id.clone(),
            from: "user@sigmaos.dev".to_string(),
            to: to.to_string(),
            subject: subject.to_string(),
            body: body.to_string(),
            timestamp: "now".to_string(),
            read: true,
            starred: false,
        };
        
        self.emails.insert(email_id.clone(), email);
        
        // Add to sent folder
        if let Some(sent) = self.folders.get_mut("sent") {
            sent.emails.push(email_id.clone());
        }
        
        email_id
    }

    /// Reply to email
    pub fn reply(&mut self, email_id: &str, body: &str) -> Result<String, String> {
        if let Some(original) = self.emails.get(email_id) {
            let reply_subject = if original.subject.starts_with("Re: ") {
                original.subject.clone()
            } else {
                format!("Re: {}", original.subject)
            };
            
            let reply_body = format!(
                "\n\n--- Original Message ---\nFrom: {}\nTo: {}\nSubject: {}\n\n{}",
                original.from, original.to, original.subject, original.body
            );
            
            let reply_email_id = self.compose(&original.from, &reply_subject, &(body.to_string() + &reply_body));
            Ok(reply_email_id)
        } else {
            Err("Email not found".to_string())
        }
    }

    /// Delete email
    pub fn delete_email(&mut self, email_id: &str) -> Result<(), String> {
        // Remove from current folder
        if let Some(folder) = self.folders.get_mut(&self.current_folder) {
            if let Some(pos) = folder.emails.iter().position(|id| id == email_id) {
                folder.emails.remove(pos);
            }
        }
        
        // Add to trash
        if let Some(trash) = self.folders.get_mut("trash") {
            if !trash.emails.contains(&email_id.to_string()) {
                trash.emails.push(email_id.to_string());
            }
        }
        
        Ok(())
    }

    /// Toggle star on email
    pub fn toggle_star(&mut self, email_id: &str) {
        if let Some(email) = self.emails.get_mut(email_id) {
            email.starred = !email.starred;
            
            // Update starred folder
            if let Some(starred) = self.folders.get_mut("starred") {
                if email.starred {
                    if !starred.emails.contains(&email_id.to_string()) {
                        starred.emails.push(email_id.to_string());
                    }
                } else {
                    starred.emails.retain(|id| id != email_id);
                }
            }
        }
    }

    /// Get folder list
    pub fn get_folders(&self) -> Vec<&Folder> {
        self.folders.values().collect()
    }

    /// Get current folder name
    pub fn get_current_folder(&self) -> &str {
        &self.current_folder
    }
}

// ─── CLI Interface ───────────────────────────────────────────────────────────

fn main() {
    let mut mail = MailClient::new();
    
    println!("Sigma-Mail v0.1 - Email Client");
    println!("Current folder: {}", mail.get_current_folder());
    
    loop {
        println!("\n--- Folder: {} ---", mail.get_current_folder());
        for email in mail.get_folder_emails() {
            let marker = if email.starred { "*" } else { " " };
            let read_marker = if email.read { " " } else { "N" };
            println!("{}[{}] {} - {} ({})", marker, read_marker, email.from, email.subject, email.timestamp);
        }
        
        println!("\nCommands: folders, folder <name>, read <id>, compose, reply <id>, delete <id>, star <id>, quit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "folders" => {
                println!("--- Folders ---");
                for folder in mail.get_folders() {
                    let marker = if folder.name == mail.get_current_folder() { " >" } else { "  " };
                    println!("{}{} ({})", marker, folder.name, folder.emails.len());
                }
            }
            "folder" => {
                if let Some(arg) = parts.get(1) {
                    match mail.switch_folder(arg) {
                        Ok(_) => println!("Switched to: {}", arg),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "read" => {
                if let Some(arg) = parts.get(1) {
                    mail.select_email(arg);
                    if let Some(email) = mail.get_selected_email() {
                        println!("\n--- Email ---");
                        println!("From: {}", email.from);
                        println!("To: {}", email.to);
                        println!("Subject: {}", email.subject);
                        println!("Date: {}", email.timestamp);
                        println!("\n{}", email.body);
                    }
                }
            }
            "compose" => {
                println!("Compose new email");
                print!("To: ");
                std::io::stdout().flush().unwrap();
                let mut to = String::new();
                std::io::stdin().read_line(&mut to).unwrap();
                
                print!("Subject: ");
                std::io::stdout().flush().unwrap();
                let mut subject = String::new();
                std::io::stdin().read_line(&mut subject).unwrap();
                
                print!("Body (end with . on a line): ");
                std::io::stdout().flush().unwrap();
                let mut body = String::new();
                loop {
                    let mut line = String::new();
                    std::io::stdin().read_line(&mut line).unwrap();
                    if line.trim() == "." {
                        break;
                    }
                    body.push_str(&line);
                }
                
                let email_id = mail.compose(to.trim(), subject.trim(), &body);
                println!("Email sent: {}", email_id);
            }
            "reply" => {
                if let Some(arg) = parts.get(1) {
                    print!("Reply: ");
                    std::io::stdout().flush().unwrap();
                    let mut body = String::new();
                    std::io::stdin().read_line(&mut body).unwrap();
                    
                    match mail.reply(arg, body.trim()) {
                        Ok(_) => println!("Reply sent"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "delete" => {
                if let Some(arg) = parts.get(1) {
                    match mail.delete_email(arg) {
                        Ok(_) => println!("Email deleted"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "star" => {
                if let Some(arg) = parts.get(1) {
                    mail.toggle_star(arg);
                    println!("Star toggled");
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
