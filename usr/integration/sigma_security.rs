// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/integration/sigma_security.rs — Sigma Security Tools Integration
//
// Implements integration with OpenSSL for cryptography and KeePassXC
// for password management within SigmaOS.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Security Tool Types ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SecurityTool {
    OpenSSL,
    KeePassXC,
    GnuPG,
    SSH,
}

#[derive(Debug, Clone)]
pub struct PasswordEntry {
    pub id: String,
    pub title: String,
    pub username: String,
    pub password: String,  // In real implementation, this would be encrypted
    pub url: String,
    pub notes: String,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct EncryptionKey {
    pub id: String,
    pub key_type: String,
    pub algorithm: String,
    pub key_size: u32,
    pub public_key: String,
    pub private_key: String,  // In real implementation, this would be encrypted
}

#[derive(Debug, Clone)]
pub struct Certificate {
    pub id: String,
    pub common_name: String,
    pub issuer: String,
    pub valid_from: String,
    pub valid_to: String,
    pub fingerprint: String,
}

// ─── Security Tools Manager ───────────────────────────────────────────────────

pub struct SecurityToolsManager {
    pub password_database: HashMap<String, PasswordEntry>,
    pub encryption_keys: HashMap<String, EncryptionKey>,
    pub certificates: HashMap<String, Certificate>,
    pub current_tool: SecurityTool,
}

impl SecurityToolsManager {
    pub fn new() -> Self {
        let mut manager = SecurityToolsManager {
            password_database: HashMap::new(),
            encryption_keys: HashMap::new(),
            certificates: HashMap::new(),
            current_tool: SecurityTool::OpenSSL,
        };
        
        manager.init_sample_data();
        manager
    }

    /// Initialize sample data
    fn init_sample_data(&mut self) {
        // Sample password entry
        self.password_database.insert("pwd_001".to_string(), PasswordEntry {
            id: "pwd_001".to_string(),
            title: "GitHub".to_string(),
            username: "user@example.com".to_string(),
            password: "********".to_string(),
            url: "https://github.com".to_string(),
            notes: "Personal account".to_string(),
            created_at: "2024-01-15".to_string(),
        });

        // Sample encryption key
        self.encryption_keys.insert("key_001".to_string(), EncryptionKey {
            id: "key_001".to_string(),
            key_type: "RSA".to_string(),
            algorithm: "RSA-4096".to_string(),
            key_size: 4096,
            public_key: "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...\n-----END PUBLIC KEY-----".to_string(),
            private_key: "-----BEGIN PRIVATE KEY-----\nMIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQ...\n-----END PRIVATE KEY-----".to_string(),
        });

        // Sample certificate
        self.certificates.insert("cert_001".to_string(), Certificate {
            id: "cert_001".to_string(),
            common_name: "sigmaos.local".to_string(),
            issuer: "SigmaOS CA".to_string(),
            valid_from: "2024-01-01".to_string(),
            valid_to: "2025-01-01".to_string(),
            fingerprint: "AA:BB:CC:DD:EE:FF:00:11:22:33:44:55:66:77:88:99".to_string(),
        });
    }

    /// Set current security tool
    pub fn set_tool(&mut self, tool: SecurityTool) {
        self.current_tool = tool;
    }

    /// Add password entry
    pub fn add_password(&mut self, entry: PasswordEntry) {
        self.password_database.insert(entry.id.clone(), entry);
    }

    /// Get password entry
    pub fn get_password(&self, id: &str) -> Option<&PasswordEntry> {
        self.password_database.get(id)
    }

    /// Search passwords
    pub fn search_passwords(&self, query: &str) -> Vec<&PasswordEntry> {
        self.password_database.values()
            .filter(|p| {
                p.title.to_lowercase().contains(&query.to_lowercase()) ||
                p.username.to_lowercase().contains(&query.to_lowercase()) ||
                p.url.to_lowercase().contains(&query.to_lowercase())
            })
            .collect()
    }

    /// Generate password (simulated)
    pub fn generate_password(&self, length: u32, include_symbols: bool) -> String {
        let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
        let symbols: Vec<char> = "!@#$%^&*()_+-=[]{}|;:,.<>?".chars().collect();
        
        let mut password = String::new();
        let charset: Vec<char> = if include_symbols {
            chars.iter().chain(symbols.iter()).copied().collect()
        } else {
            chars.clone()
        };
        
        for _ in 0..length {
            let idx = (password.len() as u32) % charset.len() as u32;
            password.push(charset[idx as usize]);
        }
        
        password
    }

    /// Generate encryption key (simulated)
    pub fn generate_key(&mut self, key_type: String, algorithm: String, key_size: u32) -> EncryptionKey {
        let key = EncryptionKey {
            id: format!("key_{}", self.encryption_keys.len()),
            key_type: key_type.clone(),
            algorithm: algorithm.clone(),
            key_size,
            public_key: format!("-----BEGIN PUBLIC KEY-----\nGenerated {}-bit key\n-----END PUBLIC KEY-----", key_size),
            private_key: format!("-----BEGIN PRIVATE KEY-----\nGenerated {}-bit key\n-----END PRIVATE KEY-----", key_size),
        };
        
        self.encryption_keys.insert(key.id.clone(), key.clone());
        key
    }

    /// Encrypt data (simulated)
    pub fn encrypt_data(&self, data: &str, key_id: &str) -> Result<String, String> {
        if let Some(_key) = self.encryption_keys.get(key_id) {
            // In real implementation, this would use actual encryption
            Ok(format!("ENCRYPTED:{}", base64_encode(data)))
        } else {
            Err("Key not found".to_string())
        }
    }

    /// Decrypt data (simulated)
    pub fn decrypt_data(&self, encrypted: &str, key_id: &str) -> Result<String, String> {
        if let Some(_key) = self.encryption_keys.get(key_id) {
            // In real implementation, this would use actual decryption
            if encrypted.starts_with("ENCRYPTED:") {
                Ok(base64_decode(&encrypted[10..]))
            } else {
                Err("Invalid encrypted data format".to_string())
            }
        } else {
            Err("Key not found".to_string())
        }
    }

    /// Generate certificate (simulated)
    pub fn generate_certificate(&mut self, common_name: String, issuer: String, validity_days: u32) -> Certificate {
        let cert = Certificate {
            id: format!("cert_{}", self.certificates.len()),
            common_name,
            issuer,
            valid_from: "now".to_string(),
            valid_to: format!("{} days from now", validity_days),
            fingerprint: format!("{:02X}", rand_hash()),
        };
        
        self.certificates.insert(cert.id.clone(), cert.clone());
        cert
    }

    /// Get tool name
    pub fn get_tool_name(&self, tool: SecurityTool) -> &str {
        match tool {
            SecurityTool::OpenSSL => "OpenSSL",
            SecurityTool::KeePassXC => "KeePassXC",
            SecurityTool::GnuPG => "GnuPG",
            SecurityTool::SSH => "SSH",
        }
    }

    /// Get all passwords
    pub fn get_all_passwords(&self) -> Vec<&PasswordEntry> {
        self.password_database.values().collect()
    }

    /// Get all keys
    pub fn get_all_keys(&self) -> Vec<&EncryptionKey> {
        self.encryption_keys.values().collect()
    }

    /// Get all certificates
    pub fn get_all_certificates(&self) -> Vec<&Certificate> {
        self.certificates.values().collect()
    }
}

// Simple base64 encoding for demo
fn base64_encode(input: &str) -> String {
    input.chars()
        .map(|c| format!("{:02x}", c as u8))
        .collect()
}

// Simple base64 decoding for demo
fn base64_decode(input: &str) -> String {
    input.as_bytes()
        .chunks(2)
        .filter_map(|chunk| {
            if chunk.len() == 2 {
                u8::from_str_radix(std::str::from_utf8(chunk).ok()?, 16).ok()
            } else {
                None
            }
        })
        .map(|b| b as char)
        .collect()
}

// Simple random hash for demo
fn rand_hash() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    duration.as_nanos() as u64
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut manager = SecurityToolsManager::new();
    
    println!("Sigma Security Tools Integration v0.1 - OpenSSL/KeePassXC/GnuPG");
    
    loop {
        println!("\n--- Current Tool: {} ---", manager.get_tool_name(manager.current_tool));
        
        println!("\nCommands: tool <type>, add_pwd, pwd <id>, search_pwd <query>, gen_pwd <length> <symbols>, gen_key <type> <algo> <size>, encrypt <data> <key_id>, decrypt <data> <key_id>, gen_cert <cn> <issuer> <days>, passwords, keys, certs, quit");
        println!("Tools: openssl, keepassxc, gnupg, ssh");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "tool" => {
                if let Some(arg) = parts.get(1) {
                    let tool = match *arg {
                        "openssl" => SecurityTool::OpenSSL,
                        "keepassxc" => SecurityTool::KeePassXC,
                        "gnupg" => SecurityTool::GnuPG,
                        "ssh" => SecurityTool::SSH,
                        _ => {
                            println!("Unknown tool");
                            continue;
                        }
                    };
                    manager.set_tool(tool);
                    println!("Tool changed to {}", manager.get_tool_name(tool));
                }
            }
            "add_pwd" => {
                println!("Enter password details:");
                print!("Title: ");
                std::io::stdout().flush().unwrap();
                let mut title = String::new();
                std::io::stdin().read_line(&mut title).unwrap();
                
                print!("Username: ");
                std::io::stdout().flush().unwrap();
                let mut username = String::new();
                std::io::stdin().read_line(&mut username).unwrap();
                
                print!("Password: ");
                std::io::stdout().flush().unwrap();
                let mut password = String::new();
                std::io::stdin().read_line(&mut password).unwrap();
                
                print!("URL: ");
                std::io::stdout().flush().unwrap();
                let mut url = String::new();
                std::io::stdin().read_line(&mut url).unwrap();
                
                let entry = PasswordEntry {
                    id: format!("pwd_{}", manager.password_database.len()),
                    title: title.trim().to_string(),
                    username: username.trim().to_string(),
                    password: password.trim().to_string(),
                    url: url.trim().to_string(),
                    notes: String::new(),
                    created_at: "now".to_string(),
                };
                
                manager.add_password(entry);
                println!("Password entry added");
            }
            "pwd" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(entry) = manager.get_password(arg) {
                        println!("--- Password Entry ---");
                        println!("Title: {}", entry.title);
                        println!("Username: {}", entry.username);
                        println!("Password: {}", entry.password);
                        println!("URL: {}", entry.url);
                        println!("Notes: {}", entry.notes);
                    }
                }
            }
            "search_pwd" => {
                if parts.len() >= 2 {
                    let query = parts[1..].join(" ");
                    let results = manager.search_passwords(&query);
                    println!("--- Search Results ---");
                    for entry in results {
                        println!("{} - {} ({})", entry.id, entry.title, entry.username);
                    }
                }
            }
            "gen_pwd" => {
                if parts.len() >= 2 {
                    if let Ok(length) = parts[1].parse::<u32>() {
                        let include_symbols = parts.get(2).map_or(false, |s| *s == "true" || *s == "1");
                        let password = manager.generate_password(length, include_symbols);
                        println!("Generated password: {}", password);
                    }
                }
            }
            "gen_key" => {
                if parts.len() >= 4 {
                    let key_type = parts[1].to_string();
                    let algorithm = parts[2].to_string();
                    if let Ok(key_size) = parts[3].parse::<u32>() {
                        let key = manager.generate_key(key_type, algorithm, key_size);
                        println!("Key generated: {}", key.id);
                        println!("Algorithm: {}", key.algorithm);
                        println!("Key Size: {} bits", key.key_size);
                    }
                }
            }
            "encrypt" => {
                if parts.len() >= 3 {
                    let data = parts[1];
                    let key_id = parts[2];
                    match manager.encrypt_data(data, key_id) {
                        Ok(encrypted) => println!("Encrypted: {}", encrypted),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "decrypt" => {
                if parts.len() >= 3 {
                    let data = parts[1];
                    let key_id = parts[2];
                    match manager.decrypt_data(data, key_id) {
                        Ok(decrypted) => println!("Decrypted: {}", decrypted),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "gen_cert" => {
                if parts.len() >= 4 {
                    let common_name = parts[1].to_string();
                    let issuer = parts[2].to_string();
                    if let Ok(validity_days) = parts[3].parse::<u32>() {
                        let cert = manager.generate_certificate(common_name, issuer, validity_days);
                        println!("Certificate generated: {}", cert.id);
                        println!("Common Name: {}", cert.common_name);
                        println!("Issuer: {}", cert.issuer);
                        println!("Valid: {} to {}", cert.valid_from, cert.valid_to);
                    }
                }
            }
            "passwords" => {
                println!("--- All Password Entries ---");
                for entry in manager.get_all_passwords() {
                    println!("{} - {} ({})", entry.id, entry.title, entry.username);
                }
            }
            "keys" => {
                println!("--- All Encryption Keys ---");
                for key in manager.get_all_keys() {
                    println!("{} - {} ({} bits)", key.id, key.algorithm, key.key_size);
                }
            }
            "certs" => {
                println!("--- All Certificates ---");
                for cert in manager.get_all_certificates() {
                    println!("{} - {} (Issuer: {})", cert.id, cert.common_name, cert.issuer);
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
