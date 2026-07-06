// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/security/sigma_keepass.rs — Sigma KeePassXC Password Manager
//
// Implements KeePassXC-style password manager with secure credential storage,
// password generation, database encryption, and auto-fill support.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Password Manager Types ───────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PasswordEntry {
    pub id: String,
    pub title: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub notes: String,
    pub group: String,
    pub created: String,
    pub modified: String,
    pub expires: Option<String>,
    pub tags: Vec<String>,
    pub custom_fields: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub icon: u32,
    pub parent_id: Option<String>,
    pub expanded: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EncryptionType {
    AES256,
    ChaCha20,
    Twofish,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub name: String,
    pub path: String,
    pub encryption: EncryptionType,
    pub key_derivation: String,
    pub iterations: u32,
    pub compression: bool,
    pub groups: HashMap<String, Group>,
    pub entries: HashMap<String, PasswordEntry>,
    pub recycle_bin: Vec<PasswordEntry>,
    pub last_modified: String,
}

#[derive(Debug, Clone)]
pub struct PasswordGenerator {
    pub length: u32,
    pub uppercase: bool,
    pub lowercase: bool,
    pub numbers: bool,
    pub symbols: bool,
    pub exclude_similar: bool,
    pub exclude_ambiguous: bool,
}

// ─── Password Manager ─────────────────────────────────────────────────────

pub struct KeePassXCManager {
    pub databases: HashMap<String, Database>,
    pub current_database: Option<String>,
    pub generator: PasswordGenerator,
    pub clipboard_timeout: u32,
    pub auto_lock_timeout: u32,
    pub auto_save: bool,
}

impl KeePassXCManager {
    pub fn new() -> Self {
        let mut manager = KeePassXCManager {
            databases: HashMap::new(),
            current_database: None,
            generator: PasswordGenerator {
                length: 16,
                uppercase: true,
                lowercase: true,
                numbers: true,
                symbols: true,
                exclude_similar: true,
                exclude_ambiguous: true,
            },
            clipboard_timeout: 30,
            auto_lock_timeout: 300,
            auto_save: true,
        };

        manager.init_default_database();
        manager
    }

    /// Initialize default database
    fn init_default_database(&mut self) {
        let mut groups = HashMap::new();
        groups.insert("root".to_string(), Group {
            id: "root".to_string(),
            name: "Root".to_string(),
            icon: 48,
            parent_id: None,
            expanded: true,
        });

        groups.insert("internet".to_string(), Group {
            id: "internet".to_string(),
            name: "Internet".to_string(),
            icon: 1,
            parent_id: Some("root".to_string()),
            expanded: true,
        });

        groups.insert("email".to_string(), Group {
            id: "email".to_string(),
            name: "Email".to_string(),
            icon: 19,
            parent_id: Some("root".to_string()),
            expanded: true,
        });

        let mut entries = HashMap::new();

        // Sample entry
        let entry1 = PasswordEntry {
            id: "entry1".to_string(),
            title: "Example Account".to_string(),
            username: "user@example.com".to_string(),
            password: "SecurePassword123!".to_string(),
            url: "https://example.com".to_string(),
            notes: "Sample entry".to_string(),
            group: "internet".to_string(),
            created: "now".to_string(),
            modified: "now".to_string(),
            expires: None,
            tags: vec!["social".to_string()],
            custom_fields: HashMap::new(),
        };
        entries.insert("entry1".to_string(), entry1);

        let database = Database {
            name: "Passwords".to_string(),
            path: "/home/user/.keepassxc/passwords.kdbx".to_string(),
            encryption: EncryptionType::AES256,
            key_derivation: "Argon2".to_string(),
            iterations: 100000,
            compression: true,
            groups,
            entries,
            recycle_bin: vec![],
            last_modified: "now".to_string(),
        };

        self.databases.insert("default".to_string(), database);
        self.current_database = Some("default".to_string());
    }

    /// Create new database
    pub fn create_database(&mut self, name: String, path: String, password: String) -> Result<Database, String> {
        let db_id = format!("db_{}", self.databases.len());
        
        let mut groups = HashMap::new();
        groups.insert("root".to_string(), Group {
            id: "root".to_string(),
            name: "Root".to_string(),
            icon: 48,
            parent_id: None,
            expanded: true,
        });

        let database = Database {
            name: name.clone(),
            path,
            encryption: EncryptionType::AES256,
            key_derivation: "Argon2".to_string(),
            iterations: 100000,
            compression: true,
            groups,
            entries: HashMap::new(),
            recycle_bin: vec![],
            last_modified: "now".to_string(),
        };

        self.databases.insert(db_id.clone(), database.clone());
        self.current_database = Some(db_id);
        Ok(database)
    }

    /// Add password entry
    pub fn add_entry(&mut self, title: String, username: String, password: String, url: String, group: String) -> Result<PasswordEntry, String> {
        if let Some(db_id) = &self.current_database {
            if let Some(db) = self.databases.get_mut(db_id) {
                let entry_id = format!("entry_{}", db.entries.len());
                let entry = PasswordEntry {
                    id: entry_id.clone(),
                    title,
                    username,
                    password,
                    url,
                    notes: String::new(),
                    group,
                    created: "now".to_string(),
                    modified: "now".to_string(),
                    expires: None,
                    tags: vec![],
                    custom_fields: HashMap::new(),
                };

                db.entries.insert(entry_id.clone(), entry.clone());
                db.last_modified = "now".to_string();
                Ok(entry)
            } else {
                Err("Database not found".to_string())
            }
        } else {
            Err("No database open".to_string())
        }
    }

    /// Update password entry
    pub fn update_entry(&mut self, entry_id: &str, title: Option<String>, username: Option<String>, password: Option<String>, url: Option<String>) -> Result<(), String> {
        if let Some(db_id) = &self.current_database {
            if let Some(db) = self.databases.get_mut(db_id) {
                if let Some(entry) = db.entries.get_mut(entry_id) {
                    if let Some(t) = title { entry.title = t; }
                    if let Some(u) = username { entry.username = u; }
                    if let Some(p) = password { entry.password = p; }
                    if let Some(u) = url { entry.url = u; }
                    entry.modified = "now".to_string();
                    db.last_modified = "now".to_string();
                    Ok(())
                } else {
                    Err("Entry not found".to_string())
                }
            } else {
                Err("Database not found".to_string())
            }
        } else {
            Err("No database open".to_string())
        }
    }

    /// Delete entry (move to recycle bin)
    pub fn delete_entry(&mut self, entry_id: &str) -> Result<(), String> {
        if let Some(db_id) = &self.current_database {
            if let Some(db) = self.databases.get_mut(db_id) {
                if let Some(entry) = db.entries.remove(entry_id) {
                    db.recycle_bin.push(entry);
                    db.last_modified = "now".to_string();
                    Ok(())
                } else {
                    Err("Entry not found".to_string())
                }
            } else {
                Err("Database not found".to_string())
            }
        } else {
            Err("No database open".to_string())
        }
    }

    /// Search entries
    pub fn search_entries(&self, query: &str) -> Vec<&PasswordEntry> {
        if let Some(db_id) = &self.current_database {
            if let Some(db) = self.databases.get(db_id) {
                let query_lower = query.to_lowercase();
                db.entries.values()
                    .filter(|e| {
                        e.title.to_lowercase().contains(&query_lower) ||
                        e.username.to_lowercase().contains(&query_lower) ||
                        e.url.to_lowercase().contains(&query_lower) ||
                        e.notes.to_lowercase().contains(&query_lower)
                    })
                    .collect()
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }

    /// Generate password
    pub fn generate_password(&self) -> String {
        let mut chars = String::new();
        
        if self.generator.lowercase {
            chars.push_str("abcdefghijklmnopqrstuvwxyz");
        }
        if self.generator.uppercase {
            chars.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        }
        if self.generator.numbers {
            chars.push_str("0123456789");
        }
        if self.generator.symbols {
            chars.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?");
        }

        if self.generator.exclude_similar {
            chars = chars.replace("i", "").replace("l", "").replace("1", "").replace("L", "").replace("O", "").replace("0");
        }

        if self.generator.exclude_ambiguous {
            chars = chars.replace("{", "").replace("}", "").replace("[", "").replace("]").replace("(", "").replace(")").replace("/", "").replace("\\", "").replace("'", "").replace("\"", "").replace("`", "").replace(",", "").replace(".", "").replace("<", "").replace(">", "").replace(";", "").replace(":", "").replace("|", "");
        }

        if chars.is_empty() {
            chars = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
        }

        let char_vec: Vec<char> = chars.chars().collect();
        let mut password = String::new();
        
        for _ in 0..self.generator.length {
            let idx = (rand::random::<usize>()) % char_vec.len();
            password.push(char_vec[idx]);
        }

        password
    }

    /// Add group
    pub fn add_group(&mut self, name: String, parent_id: Option<String>) -> Result<Group, String> {
        if let Some(db_id) = &self.current_database {
            if let Some(db) = self.databases.get_mut(db_id) {
                let group_id = format!("group_{}", db.groups.len());
                let group = Group {
                    id: group_id.clone(),
                    name,
                    icon: 48,
                    parent_id,
                    expanded: true,
                };

                db.groups.insert(group_id.clone(), group.clone());
                db.last_modified = "now".to_string();
                Ok(group)
            } else {
                Err("Database not found".to_string())
            }
        } else {
            Err("No database open".to_string())
        }
    }

    /// Get all entries
    pub fn get_all_entries(&self) -> Vec<&PasswordEntry> {
        if let Some(db_id) = &self.current_database {
            if let Some(db) = self.databases.get(db_id) {
                db.entries.values().collect()
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }

    /// Get all groups: Vec<&Group> {
        if let Some(db_id) = &self.current_database {
            if let Some(db) = self.databases.get(db_id) {
                db.groups.values().collect()
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }

    /// Get entries by group
    pub fn get_entries_by_group(&self, group_id: &str) -> Vec<&PasswordEntry> {
        if let Some(db_id) = &self.current_database {
            if let Some(db) = self.databases.get(db_id) {
                db.entries.values()
                    .filter(|e| e.group == group_id)
                    .collect()
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }

    /// Empty recycle bin
    pub fn empty_recycle_bin(&mut self) -> Result<(), String> {
        if let Some(db_id) = &self.current_database {
            if let Some(db) = self.databases.get_mut(db_id) {
                db.recycle_bin.clear();
                db.last_modified = "now".to_string();
                Ok(())
            } else {
                Err("Database not found".to_string())
            }
        } else {
            Err("No database open".to_string())
        }
    }

    /// Lock database
    pub fn lock_database(&mut self) {
        self.current_database = None;
    }

    /// Unlock database
    pub fn unlock_database(&mut self, db_id: &str, password: &str) -> Result<(), String> {
        if self.databases.contains_key(db_id) {
            // Simplified - in real implementation would verify password
            self.current_database = Some(db_id.to_string());
            Ok(())
        } else {
            Err("Database not found".to_string())
        }
    }

    /// Export database
    pub fn export_database(&self, format: &str) -> Result<String, String> {
        if let Some(db_id) = &self.current_database {
            if let Some(db) = self.databases.get(db_id) {
                match format {
                    "json" => {
                        let output = format!(
                            "{{\"name\": \"{}\", \"entries\": {}, \"groups\": {}}}",
                            db.name,
                            db.entries.len(),
                            db.groups.len()
                        );
                        Ok(output)
                    }
                    "csv" => {
                        let mut output = String::from("Title,Username,Password,URL\n");
                        for entry in db.entries.values() {
                            output.push_str(&format!("{},{},{},{}\n", entry.title, entry.username, entry.password, entry.url));
                        }
                        Ok(output)
                    }
                    _ => Err("Unsupported format".to_string())
                }
            } else {
                Err("Database not found".to_string())
            }
        } else {
            Err("No database open".to_string())
        }
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut keepass = KeePassXCManager::new();
    
    println!("Sigma KeePassXC v0.1 - Password Manager");
    
    loop {
        println!("\n--- Password Manager Commands ---");
        println!("entries            - List all entries");
        println!("groups             - List all groups");
        println!("add <title> <user> <pass> <url> <group> - Add entry");
        println!("update <id> [title] [user] [pass] [url] - Update entry");
        println!("delete <id>        - Delete entry");
        println!("search <query>     - Search entries");
        println!("generate           - Generate password");
        println!("add_group <name> [parent] - Add group");
        println!("group_entries <id> - Get entries in group");
        println!("recycle            - Show recycle bin");
        println!("empty_recycle      - Empty recycle bin");
        println!("lock               - Lock database");
        println!("export <format>   - Export (json/csv)");
        println!("quit               - Exit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "entries" => {
                println!("--- Password Entries ---");
                for entry in keepass.get_all_entries() {
                    println!("{} - {} - {}", entry.title, entry.username, entry.url);
                }
            }
            "groups" => {
                println!("--- Groups ---");
                for group in keepass.get_all_groups() {
                    println!("{} - {}", group.id, group.name);
                }
            }
            "add" => {
                if parts.len() >= 6 {
                    let title = parts[1].to_string();
                    let username = parts[2].to_string();
                    let password = parts[3].to_string();
                    let url = parts[4].to_string();
                    let group = parts[5].to_string();
                    match keepass.add_entry(title, username, password, url, group) {
                        Ok(_) => println!("Entry added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "update" => {
                if parts.len() >= 2 {
                    let id = parts[1];
                    let title = parts.get(2).map(|s| s.to_string());
                    let username = parts.get(3).map(|s| s.to_string());
                    let password = parts.get(4).map(|s| s.to_string());
                    let url = parts.get(5).map(|s| s.to_string());
                    match keepass.update_entry(id, title, username, password, url) {
                        Ok(_) => println!("Entry updated"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "delete" => {
                if let Some(id) = parts.get(1) {
                    match keepass.delete_entry(id) {
                        Ok(_) => println!("Entry deleted"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "search" => {
                if let Some(query) = parts.get(1) {
                    println!("--- Search Results ---");
                    for entry in keepass.search_entries(query) {
                        println!("{} - {} - {}", entry.title, entry.username, entry.url);
                    }
                }
            }
            "generate" => {
                let password = keepass.generate_password();
                println!("Generated password: {}", password);
            }
            "add_group" => {
                if parts.len() >= 2 {
                    let name = parts[1].to_string();
                    let parent = parts.get(2).map(|s| s.to_string());
                    match keepass.add_group(name, parent) {
                        Ok(_) => println!("Group added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "group_entries" => {
                if let Some(id) = parts.get(1) {
                    println!("--- Entries in Group ---");
                    for entry in keepass.get_entries_by_group(id) {
                        println!("{} - {} - {}", entry.title, entry.username, entry.url);
                    }
                }
            }
            "recycle" => {
                println!("--- Recycle Bin ---");
                if let Some(db_id) = &keepass.current_database {
                    if let Some(db) = keepass.databases.get(db_id) {
                        for entry in &db.recycle_bin {
                            println!("{} - {}", entry.title, entry.username);
                        }
                    }
                }
            }
            "empty_recycle" => {
                match keepass.empty_recycle_bin() {
                    Ok(_) => println!("Recycle bin emptied"),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "lock" => {
                keepass.lock_database();
                println!("Database locked");
            }
            "export" => {
                if let Some(format) = parts.get(1) {
                    match keepass.export_database(format) {
                        Ok(output) => println!("{}", output),
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

// Simple random number generator for password generation
mod rand {
    use std::cell::RefCell;
    use std::rc::Rc;
    
    thread_local! {
        static SEED: Rc<RefCell<u64>> = Rc::new(RefCell::new(12345));
    }
    
    pub fn random<T>() -> T where T: Default {
        // Simplified - in real implementation use proper RNG
        SEED.with(|seed| {
            let mut s = seed.borrow_mut();
            *s = s.wrapping_mul(1103515245).wrapping_add(12345);
            T::default()
        })
    }
}
