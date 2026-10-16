// SPDX-License-Identifier: MIT
// SigmaOS - Email Client
// Inspired by Thunderbird and Geary
// Provides email management

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Email account type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountType {
    IMAP,
    POP3,
}

/// Encryption type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptionType {
    None,
    SSL,
    TLS,
    StartTLS,
}

/// Email priority
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmailPriority {
    Low,
    Normal,
    High,
}

/// Email folder
#[derive(Debug, Clone)]
pub struct EmailFolder {
    pub name: String,
    pub unread_count: u32,
    pub total_count: u32,
    pub is_system: bool,
}

/// Email attachment
#[derive(Debug, Clone)]
pub struct EmailAttachment {
    pub filename: String,
    pub size: u64,
    pub content_type: String,
}

/// Email
#[derive(Debug, Clone)]
pub struct Email {
    pub id: String,
    pub subject: String,
    pub from: String,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub body: String,
    pub date: String,
    pub read: bool,
    pub starred: bool,
    pub priority: EmailPriority,
    pub attachments: Vec<EmailAttachment>,
    pub folder: String,
}

/// Email account
#[derive(Debug, Clone)]
pub struct EmailAccount {
    pub id: String,
    pub name: String,
    pub email_address: String,
    pub account_type: AccountType,
    pub imap_server: String,
    pub imap_port: u16,
    pub smtp_server: String,
    pub smtp_port: u16,
    pub username: String,
    pub encryption: EncryptionType,
    pub folders: Vec<EmailFolder>,
}

/// Email client
pub struct EmailClient {
    pub accounts: Vec<EmailAccount>,
    pub emails: Vec<Email>,
    pub current_account: Option<usize>,
    pub current_folder: Option<String>,
    pub selected_email: Option<String>,
    pub composing: bool,
    pub draft: Option<Email>,
}

impl EmailClient {
    pub fn new() -> Self {
        Self {
            accounts: Vec::new(),
            emails: Vec::new(),
            current_account: None,
            current_folder: None,
            selected_email: None,
            composing: false,
            draft: None,
        }
    }

    /// Add account
    pub fn add_account(&mut self, account: EmailAccount) {
        self.accounts.push(account);
        if self.current_account.is_none() {
            self.current_account = Some(0);
        }
    }

    /// Remove account
    pub fn remove_account(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.accounts.len() {
            return Err("Account index out of bounds");
        }
        self.accounts.remove(index);
        if self.current_account == Some(index) {
            self.current_account = self.accounts.len().checked_sub(1);
        }
        Ok(())
    }

    /// Set current account
    pub fn set_current_account(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.accounts.len() {
            return Err("Account index out of bounds");
        }
        self.current_account = Some(index);
        Ok(())
    }

    /// Set current folder
    pub fn set_current_folder(&mut self, folder: String) {
        self.current_folder = Some(folder);
    }

    /// Select email
    pub fn select_email(&mut self, email_id: String) {
        self.selected_email = Some(email_id);
    }

    /// Compose new email
    pub fn compose(&mut self) {
        self.composing = true;
        self.draft = Some(Email {
            id: String::new(),
            subject: String::new(),
            from: String::new(),
            to: Vec::new(),
            cc: Vec::new(),
            bcc: Vec::new(),
            body: String::new(),
            date: String::new(),
            read: true,
            starred: false,
            priority: EmailPriority::Normal,
            attachments: Vec::new(),
            folder: "Drafts".to_string(),
        });
    }

    /// Send email
    pub fn send(&mut self) -> Result<(), &'static str> {
        if !self.composing {
            return Err("Not composing");
        }
        let draft = self.draft.take().ok_or("No draft")?;
        self.composing = false;
        self.emails.push(draft);
        Ok(())
    }

    /// Mark as read
    pub fn mark_as_read(&mut self, email_id: String) -> Result<(), &'static str> {
        let email = self
            .emails
            .iter_mut()
            .find(|e| e.id == email_id)
            .ok_or("Email not found")?;
        email.read = true;
        Ok(())
    }

    /// Mark as unread
    pub fn mark_as_unread(&mut self, email_id: String) -> Result<(), &'static str> {
        let email = self
            .emails
            .iter_mut()
            .find(|e| e.id == email_id)
            .ok_or("Email not found")?;
        email.read = false;
        Ok(())
    }

    /// Toggle star
    pub fn toggle_star(&mut self, email_id: String) -> Result<(), &'static str> {
        let email = self
            .emails
            .iter_mut()
            .find(|e| e.id == email_id)
            .ok_or("Email not found")?;
        email.starred = !email.starred;
        Ok(())
    }

    /// Delete email
    pub fn delete_email(&mut self, email_id: String) -> Result<(), &'static str> {
        let index = self
            .emails
            .iter()
            .position(|e| e.id == email_id)
            .ok_or("Email not found")?;
        self.emails.remove(index);
        Ok(())
    }

    /// Get emails in folder
    pub fn get_emails_in_folder(&self, folder: &str) -> Vec<&Email> {
        self.emails.iter().filter(|e| e.folder == folder).collect()
    }

    /// Get accounts
    pub fn get_accounts(&self) -> &[EmailAccount] {
        &self.accounts
    }

    /// Account count
    pub fn account_count(&self) -> usize {
        self.accounts.len()
    }

    /// Email count
    pub fn email_count(&self) -> usize {
        self.emails.len()
    }
}

/// Global email client
static mut EMAIL_CLIENT: Option<EmailClient> = None;

/// Initialize email client
pub fn init_email_client() {
    unsafe {
        EMAIL_CLIENT = Some(EmailClient::new());
    }
}

/// Get email client
pub fn get_email_client() -> &'static mut EmailClient {
    unsafe { EMAIL_CLIENT.as_mut().expect("Email client not initialized") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_client_creation() {
        let ec = EmailClient::new();
        assert_eq!(ec.account_count(), 0);
    }

    #[test]
    fn test_add_account() {
        let mut ec = EmailClient::new();
        let account = EmailAccount {
            id: "1".to_string(),
            name: "Personal".to_string(),
            email_address: "user@example.com".to_string(),
            account_type: AccountType::IMAP,
            imap_server: "imap.example.com".to_string(),
            imap_port: 993,
            smtp_server: "smtp.example.com".to_string(),
            smtp_port: 587,
            username: "user".to_string(),
            encryption: EncryptionType::SSL,
            folders: Vec::new(),
        };
        ec.add_account(account);
        assert_eq!(ec.account_count(), 1);
    }

    #[test]
    fn test_compose() {
        let mut ec = EmailClient::new();
        ec.compose();
        assert!(ec.composing);
    }
}
