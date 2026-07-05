// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/apps/sigma_thunderbird.rs — Sigma Thunderbird Email Client
//
// Implements Thunderbird-style email client with account management,
// message handling, folders, contacts, and calendar integration.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Email Types ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageType {
    Inbox,
    Sent,
    Drafts,
    Trash,
    Spam,
    Archive,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessagePriority {
    High,
    Normal,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageStatus {
    Unread,
    Read,
    Replied,
    Forwarded,
    Flagged,
}

#[derive(Debug, Clone)]
pub struct EmailAddress {
    pub name: String,
    pub address: String,
}

#[derive(Debug, Clone)]
pub struct EmailMessage {
    pub id: String,
    pub subject: String,
    pub from: EmailAddress,
    pub to: Vec<EmailAddress>,
    pub cc: Vec<EmailAddress>,
    pub bcc: Vec<EmailAddress>,
    pub body: String,
    pub html_body: Option<String>,
    pub attachments: Vec<Attachment>,
    pub sent_date: String,
    pub received_date: String,
    pub message_type: MessageType,
    pub priority: MessagePriority,
    pub status: MessageStatus,
    pub folder: String,
}

#[derive(Debug, Clone)]
pub struct Attachment {
    pub filename: String,
    pub size: u64,
    pub content_type: String,
    pub content_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EmailAccount {
    pub id: String,
    pub name: String,
    pub email: String,
    pub imap_server: String,
    pub imap_port: u16,
    pub smtp_server: String,
    pub smtp_port: u16,
    pub use_ssl: bool,
    pub use_tls: bool,
    pub password_encrypted: String,
    pub signature: Option<String>,
    pub auto_check: bool,
    pub check_interval: u32,
}

#[derive(Debug, Clone)]
pub struct Contact {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub display_name: String,
    pub email_addresses: Vec<String>,
    pub phone_numbers: Vec<String>,
    pub company: Option<String>,
    pub notes: String,
}

#[derive(Debug, Clone)]
pub struct Folder {
    pub id: String,
    pub name: String,
    pub message_type: MessageType,
    pub message_count: u32,
    pub unread_count: u32,
    pub custom: bool,
}

// ─── Email Manager ───────────────────────────────────────────────────────

pub struct ThunderbirdManager {
    pub accounts: HashMap<String, EmailAccount>,
    pub messages: HashMap<String, EmailMessage>,
    pub folders: HashMap<String, Folder>,
    pub contacts: HashMap<String, Contact>,
    pub current_account: Option<String>,
    pub current_folder: String,
}

impl ThunderbirdManager {
    pub fn new() -> Self {
        let mut manager = ThunderbirdManager {
            accounts: HashMap::new(),
            messages: HashMap::new(),
            folders: HashMap::new(),
            contacts: HashMap::new(),
            current_account: None,
            current_folder: "inbox".to_string(),
        };

        manager.init_default_folders();
        manager
    }

    /// Initialize default folders
    fn init_default_folders(&mut self) {
        self.folders.insert("inbox".to_string(), Folder {
            id: "inbox".to_string(),
            name: "Inbox".to_string(),
            message_type: MessageType::Inbox,
            message_count: 0,
            unread_count: 0,
            custom: false,
        });

        self.folders.insert("sent".to_string(), Folder {
            id: "sent".to_string(),
            name: "Sent".to_string(),
            message_type: MessageType::Sent,
            message_count: 0,
            unread_count: 0,
            custom: false,
        });

        self.folders.insert("drafts".to_string(), Folder {
            id: "drafts".to_string(),
            name: "Drafts".to_string(),
            message_type: MessageType::Drafts,
            message_count: 0,
            unread_count: 0,
            custom: false,
        });

        self.folders.insert("trash".to_string(), Folder {
            id: "trash".to_string(),
            name: "Trash".to_string(),
            message_type: MessageType::Trash,
            message_count: 0,
            unread_count: 0,
            custom: false,
        });

        self.folders.insert("spam".to_string(), Folder {
            id: "spam".to_string(),
            name: "Spam".to_string(),
            message_type: MessageType::Spam,
            message_count: 0,
            unread_count: 0,
            custom: false,
        });

        self.folders.insert("archive".to_string(), Folder {
            id: "archive".to_string(),
            name: "Archive".to_string(),
            message_type: MessageType::Archive,
            message_count: 0,
            unread_count: 0,
            custom: false,
        });
    }

    /// Add an email account
    pub fn add_account(&mut self, account: EmailAccount) -> Result<EmailAccount, String> {
        if self.accounts.contains_key(&account.id) {
            return Err("Account already exists".to_string());
        }

        self.accounts.insert(account.id.clone(), account.clone());
        self.current_account = Some(account.id.clone());
        Ok(account)
    }

    /// Remove an account
    pub fn remove_account(&mut self, account_id: &str) -> Result<(), String> {
        if self.accounts.remove(account_id).is_some() {
            if self.current_account.as_ref() == Some(&account_id.to_string()) {
                self.current_account = None;
            }
            Ok(())
        } else {
            Err("Account not found".to_string())
        }
    }

    /// Set current account
    pub fn set_current_account(&mut self, account_id: &str) -> Result<(), String> {
        if self.accounts.contains_key(account_id) {
            self.current_account = Some(account_id.to_string());
            Ok(())
        } else {
            Err("Account not found".to_string())
        }
    }

    /// Compose a new message
    pub fn compose_message(&mut self, subject: String, to: Vec<String>, body: String) -> Result<EmailMessage, String> {
        let message_id = format!("msg_{}", self.messages.len());
        
        let to_addresses: Vec<EmailAddress> = to.iter().map(|addr| EmailAddress {
            name: addr.clone(),
            address: addr.clone(),
        }).collect();

        let message = EmailMessage {
            id: message_id.clone(),
            subject,
            from: EmailAddress {
                name: "Me".to_string(),
                address: self.current_account.as_ref()
                    .and_then(|id| self.accounts.get(id))
                    .map(|a| a.email.clone())
                    .unwrap_or("me@example.com".to_string()),
            },
            to: to_addresses,
            cc: vec![],
            bcc: vec![],
            body,
            html_body: None,
            attachments: vec![],
            sent_date: "now".to_string(),
            received_date: "now".to_string(),
            message_type: MessageType::Drafts,
            priority: MessagePriority::Normal,
            status: MessageStatus::Unread,
            folder: "drafts".to_string(),
        };

        self.messages.insert(message_id.clone(), message.clone());
        self.update_folder_counts("drafts");
        Ok(message)
    }

    /// Send a message
    pub fn send_message(&mut self, message_id: &str) -> Result<(), String> {
        if let Some(message) = self.messages.get_mut(message_id) {
            message.message_type = MessageType::Sent;
            message.folder = "sent".to_string();
            message.status = MessageStatus::Read;
            self.update_folder_counts("drafts");
            self.update_folder_counts("sent");
            Ok(())
        } else {
            Err("Message not found".to_string())
        }
    }

    /// Receive a message
    pub fn receive_message(&mut self, message: EmailMessage) -> Result<(), String> {
        self.messages.insert(message.id.clone(), message.clone());
        self.update_folder_counts(&message.folder);
        Ok(())
    }

    /// Move message to folder
    pub fn move_message(&mut self, message_id: &str, folder: &str) -> Result<(), String> {
        if let Some(message) = self.messages.get_mut(message_id) {
            let old_folder = message.folder.clone();
            message.folder = folder.to_string();
            self.update_folder_counts(&old_folder);
            self.update_folder_counts(folder);
            Ok(())
        } else {
            Err("Message not found".to_string())
        }
    }

    /// Delete message
    pub fn delete_message(&mut self, message_id: &str) -> Result<(), String> {
        if let Some(message) = self.messages.get_mut(message_id) {
            let old_folder = message.folder.clone();
            message.folder = "trash".to_string();
            self.update_folder_counts(&old_folder);
            self.update_folder_counts("trash");
            Ok(())
        } else {
            Err("Message not found".to_string())
        }
    }

    /// Mark message as read
    pub fn mark_read(&mut self, message_id: &str) -> Result<(), String> {
        if let Some(message) = self.messages.get_mut(message_id) {
            message.status = MessageStatus::Read;
            self.update_folder_counts(&message.folder);
            Ok(())
        } else {
            Err("Message not found".to_string())
        }
    }

    /// Mark message as unread
    pub fn mark_unread(&mut self, message_id: &str) -> Result<(), String> {
        if let Some(message) = self.messages.get_mut(message_id) {
            message.status = MessageStatus::Unread;
            self.update_folder_counts(&message.folder);
            Ok(())
        } else {
            Err("Message not found".to_string())
        }
    }

    /// Flag message
    pub fn flag_message(&mut self, message_id: &str) -> Result<(), String> {
        if let Some(message) = self.messages.get_mut(message_id) {
            message.status = MessageStatus::Flagged;
            Ok(())
        } else {
            Err("Message not found".to_string())
        }
    }

    /// Add attachment to message
    pub fn add_attachment(&mut self, message_id: &str, attachment: Attachment) -> Result<(), String> {
        if let Some(message) = self.messages.get_mut(message_id) {
            message.attachments.push(attachment);
            Ok(())
        } else {
            Err("Message not found".to_string())
        }
    }

    /// Create custom folder
    pub fn create_folder(&mut self, name: String) -> Result<Folder, String> {
        let folder_id = format!("folder_{}", self.folders.len());
        
        if self.folders.contains_key(&name) {
            return Err("Folder already exists".to_string());
        }

        let folder = Folder {
            id: folder_id.clone(),
            name: name.clone(),
            message_type: MessageType::Custom,
            message_count: 0,
            unread_count: 0,
            custom: true,
        };

        self.folders.insert(name.clone(), folder.clone());
        Ok(folder)
    }

    /// Add contact
    pub fn add_contact(&mut self, contact: Contact) -> Result<Contact, String> {
        self.contacts.insert(contact.id.clone(), contact.clone());
        Ok(contact)
    }

    /// Remove contact
    pub fn remove_contact(&mut self, contact_id: &str) -> Result<(), String> {
        if self.contacts.remove(contact_id).is_some() {
            Ok(())
        } else {
            Err("Contact not found".to_string())
        }
    }

    /// Search contacts
    pub fn search_contacts(&self, query: &str) -> Vec<&Contact> {
        let query_lower = query.to_lowercase();
        self.contacts.values()
            .filter(|c| {
                c.display_name.to_lowercase().contains(&query_lower) ||
                c.email_addresses.iter().any(|e| e.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// Search messages
    pub fn search_messages(&self, query: &str) -> Vec<&EmailMessage> {
        let query_lower = query.to_lowercase();
        self.messages.values()
            .filter(|m| {
                m.subject.to_lowercase().contains(&query_lower) ||
                m.body.to_lowercase().contains(&query_lower) ||
                m.from.address.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Get messages in folder
    pub fn get_folder_messages(&self, folder: &str) -> Vec<&EmailMessage> {
        self.messages.values()
            .filter(|m| m.folder == folder)
            .collect()
    }

    /// Update folder counts
    fn update_folder_counts(&mut self, folder: &str) {
        if let Some(f) = self.folders.get_mut(folder) {
            f.message_count = self.messages.values().filter(|m| m.folder == folder).count() as u32;
            f.unread_count = self.messages.values()
                .filter(|m| m.folder == folder && m.status == MessageStatus::Unread)
                .count() as u32;
        }
    }

    /// Set current folder
    pub fn set_current_folder(&mut self, folder: &str) {
        self.current_folder = folder.to_string();
    }

    /// Get unread count
    pub fn get_unread_count(&self) -> u32 {
        self.folders.values().map(|f| f.unread_count).sum()
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut thunderbird = ThunderbirdManager::new();
    
    println!("Sigma Thunderbird v0.1 - Email Client");
    
    loop {
        println!("\n--- Thunderbird Commands ---");
        println!("accounts           - List accounts");
        println!("add_account <id> <email> <imap> <smtp> - Add account");
        println!("remove_account <id> - Remove account");
        println!("set_account <id>   - Set current account");
        println!("folders            - List folders");
        println!("create_folder <name> - Create folder");
        println!("set_folder <name>  - Set current folder");
        println!("messages           - List messages in current folder");
        println!("compose <subject> <to> - Compose message");
        println!("send <msg_id>      - Send message");
        println!("read <msg_id>     - Read message");
        println!("reply <msg_id>    - Reply to message");
        println!("forward <msg_id>  - Forward message");
        println!("move <msg_id> <folder> - Move message");
        println!("delete <msg_id>   - Delete message");
        println!("mark_read <msg_id> - Mark as read");
        println!("mark_unread <msg_id> - Mark as unread");
        println!("flag <msg_id>      - Flag message");
        println!("attach <msg_id> <file> <size> - Add attachment");
        println!("contacts          - List contacts");
        println!("add_contact <id> <name> <email> - Add contact");
        println!("remove_contact <id> - Remove contact");
        println!("search_contacts <query> - Search contacts");
        println!("search_messages <query> - Search messages");
        println!("unread            - Show unread count");
        println!("quit              - Exit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "accounts" => {
                println!("--- Accounts ---");
                for account in thunderbird.accounts.values() {
                    println!("{} - {} ({})", account.id, account.name, account.email);
                }
            }
            "add_account" => {
                if parts.len() >= 5 {
                    let id = parts[1].to_string();
                    let email = parts[2].to_string();
                    let imap = parts[3].to_string();
                    let smtp = parts[4].to_string();
                    let account = EmailAccount {
                        id: id.clone(),
                        name: email.clone(),
                        email,
                        imap_server: imap,
                        imap_port: 993,
                        smtp_server: smtp,
                        smtp_port: 587,
                        use_ssl: true,
                        use_tls: true,
                        password_encrypted: "encrypted".to_string(),
                        signature: None,
                        auto_check: true,
                        check_interval: 300,
                    };
                    match thunderbird.add_account(account) {
                        Ok(_) => println!("Account added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "remove_account" => {
                if let Some(id) = parts.get(1) {
                    match thunderbird.remove_account(id) {
                        Ok(_) => println!("Account removed"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "set_account" => {
                if let Some(id) = parts.get(1) {
                    match thunderbird.set_current_account(id) {
                        Ok(_) => println!("Account set"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "folders" => {
                println!("--- Folders ---");
                for folder in thunderbird.folders.values() {
                    println!("{} - {} ({} messages, {} unread)", 
                        folder.id, folder.name, folder.message_count, folder.unread_count);
                }
            }
            "create_folder" => {
                if let Some(name) = parts.get(1) {
                    match thunderbird.create_folder(name.to_string()) {
                        Ok(_) => println!("Folder created"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "set_folder" => {
                if let Some(name) = parts.get(1) {
                    thunderbird.set_current_folder(name);
                    println!("Folder set to {}", name);
                }
            }
            "messages" => {
                println!("--- Messages in {} ---", thunderbird.current_folder);
                for message in thunderbird.get_folder_messages(&thunderbird.current_folder) {
                    println!("{} - {} - From: {} - {:?}", 
                        message.id, message.subject, message.from.address, message.status);
                }
            }
            "compose" => {
                if parts.len() >= 3 {
                    let subject = parts[1].to_string();
                    let to = parts[2].to_string();
                    match thunderbird.compose_message(subject, vec![to], "Message body".to_string()) {
                        Ok(msg) => println!("Message composed: {}", msg.id),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "send" => {
                if let Some(id) = parts.get(1) {
                    match thunderbird.send_message(id) {
                        Ok(_) => println!("Message sent"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "read" => {
                if let Some(id) = parts.get(1) {
                    if let Some(message) = thunderbird.messages.get(id) {
                        println!("--- Message ---");
                        println!("From: {}", message.from.address);
                        println!("To: {:?}", message.to.iter().map(|e| &e.address).collect::<Vec<_>>());
                        println!("Subject: {}", message.subject);
                        println!("Body: {}", message.body);
                        thunderbird.mark_read(id).ok();
                    }
                }
            }
            "reply" => {
                if let Some(id) = parts.get(1) {
                    if let Some(message) = thunderbird.messages.get(id) {
                        match thunderbird.compose_message(
                            format!("Re: {}", message.subject),
                            vec![message.from.address.clone()],
                            format!("\n\nOn {} wrote:\n{}", message.sent_date, message.body)
                        ) {
                            Ok(msg) => println!("Reply composed: {}", msg.id),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "forward" => {
                if let Some(id) = parts.get(1) {
                    if let Some(message) = thunderbird.messages.get(id) {
                        match thunderbird.compose_message(
                            format!("Fwd: {}", message.subject),
                            vec![],
                            format!("\n\n---------- Forwarded message ----------\nFrom: {}\nSubject: {}\n\n{}", 
                                message.from.address, message.subject, message.body)
                        ) {
                            Ok(msg) => println!("Forward composed: {}", msg.id),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "move" => {
                if parts.len() >= 3 {
                    let id = parts[1];
                    let folder = parts[2];
                    match thunderbird.move_message(id, folder) {
                        Ok(_) => println!("Message moved"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "delete" => {
                if let Some(id) = parts.get(1) {
                    match thunderbird.delete_message(id) {
                        Ok(_) => println!("Message deleted"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "mark_read" => {
                if let Some(id) = parts.get(1) {
                    match thunderbird.mark_read(id) {
                        Ok(_) => println!("Marked as read"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "mark_unread" => {
                if let Some(id) = parts.get(1) {
                    match thunderbird.mark_unread(id) {
                        Ok(_) => println!("Marked as unread"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "flag" => {
                if let Some(id) = parts.get(1) {
                    match thunderbird.flag_message(id) {
                        Ok(_) => println!("Message flagged"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "attach" => {
                if parts.len() >= 4 {
                    let id = parts[1];
                    let filename = parts[2].to_string();
                    let size = parts[3].parse::<u64>().unwrap_or(0);
                    let attachment = Attachment {
                        filename,
                        size,
                        content_type: "application/octet-stream".to_string(),
                        content_id: None,
                    };
                    match thunderbird.add_attachment(id, attachment) {
                        Ok(_) => println!("Attachment added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "contacts" => {
                println!("--- Contacts ---");
                for contact in thunderbird.contacts.values() {
                    println!("{} - {} - {:?}", contact.display_name, contact.id, contact.email_addresses);
                }
            }
            "add_contact" => {
                if parts.len() >= 4 {
                    let id = parts[1].to_string();
                    let name = parts[2].to_string();
                    let email = parts[3].to_string();
                    let contact = Contact {
                        id: id.clone(),
                        first_name: name.clone(),
                        last_name: String::new(),
                        display_name: name,
                        email_addresses: vec![email],
                        phone_numbers: vec![],
                        company: None,
                        notes: String::new(),
                    };
                    match thunderbird.add_contact(contact) {
                        Ok(_) => println!("Contact added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "remove_contact" => {
                if let Some(id) = parts.get(1) {
                    match thunderbird.remove_contact(id) {
                        Ok(_) => println!("Contact removed"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "search_contacts" => {
                if parts.len() >= 2 {
                    let query = parts[1..].join(" ");
                    println!("--- Search Results ---");
                    for contact in thunderbird.search_contacts(&query) {
                        println!("{} - {:?}", contact.display_name, contact.email_addresses);
                    }
                }
            }
            "search_messages" => {
                if parts.len() >= 2 {
                    let query = parts[1..].join(" ");
                    println!("--- Search Results ---");
                    for message in thunderbird.search_messages(&query) {
                        println!("{} - {} - {}", message.id, message.subject, message.from.address);
                    }
                }
            }
            "unread" => {
                println!("Unread messages: {}", thunderbird.get_unread_count());
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
