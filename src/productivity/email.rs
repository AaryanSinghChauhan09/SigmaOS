// SigmaOS Email Client
// OOP-based email client with IMAP/SMTP support and organization

use std::collections::HashMap;

/// Email address
#[derive(Debug, Clone)]
pub struct EmailAddress {
    pub name: String,
    pub address: String,
}

/// Email attachment
#[derive(Debug, Clone)]
pub struct EmailAttachment {
    pub filename: String,
    pub content_type: String,
    pub size_bytes: u64,
    pub content: Vec<u8>,
}

/// Email message
#[derive(Debug, Clone)]
pub struct EmailMessage {
    pub id: String,
    pub from: EmailAddress,
    pub to: Vec<EmailAddress>,
    pub cc: Vec<EmailAddress>,
    pub bcc: Vec<EmailAddress>,
    pub subject: String,
    pub body: String,
    pub body_html: Option<String>,
    pub attachments: Vec<EmailAttachment>,
    pub sent_at: u64,
    pub received_at: u64,
    pub read: bool,
    pub starred: bool,
    pub folder: EmailFolder,
    pub labels: Vec<String>,
}

/// Email folder
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmailFolder {
    Inbox,
    Sent,
    Drafts,
    Trash,
    Spam,
    Archive,
    Custom(String),
}

/// Email priority
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmailPriority {
    Low,
    Normal,
    High,
    Urgent,
}

/// Email filter
#[derive(Debug, Clone)]
pub struct EmailFilter {
    pub folder: Option<EmailFolder>,
    pub from: Option<String>,
    pub subject_contains: Option<String>,
    pub unread_only: bool,
    pub starred_only: bool,
    pub has_attachments: bool,
    pub date_from: Option<u64>,
    pub date_to: Option<u64>,
    pub label: Option<String>,
}

/// Email account config
#[derive(Debug, Clone)]
pub struct EmailAccountConfig {
    pub email: String,
    pub imap_server: String,
    pub imap_port: u16,
    pub smtp_server: String,
    pub smtp_port: u16,
    pub use_ssl: bool,
    pub use_tls: bool,
}

/// OOP trait for email backends
pub trait EmailBackend {
    /// Send email
    fn send_email(&mut self, message: EmailMessage) -> Result<(), EmailError>;
    /// Fetch emails
    fn fetch_emails(
        &mut self,
        folder: EmailFolder,
        limit: usize,
    ) -> Result<Vec<EmailMessage>, EmailError>;
    /// Mark as read
    fn mark_as_read(&mut self, email_id: &str) -> Result<(), EmailError>;
    /// Delete email
    fn delete_email(&mut self, email_id: &str) -> Result<(), EmailError>;
    /// Move to folder
    fn move_to_folder(&mut self, email_id: &str, folder: EmailFolder) -> Result<(), EmailError>;
    /// Get backend name
    fn name(&self) -> &str;
}

/// IMAP/SMTP backend
pub struct ImapSmtpBackend {
    config: EmailAccountConfig,
    emails: HashMap<String, EmailMessage>,
}

impl ImapSmtpBackend {
    pub fn new(config: EmailAccountConfig) -> Self {
        Self {
            config,
            emails: HashMap::new(),
        }
    }
}

impl EmailBackend for ImapSmtpBackend {
    fn send_email(&mut self, message: EmailMessage) -> Result<(), EmailError> {
        // Simulated sending
        let mut sent_message = message.clone();
        sent_message.folder = EmailFolder::Sent;
        self.emails.insert(sent_message.id.clone(), sent_message);
        Ok(())
    }

    fn fetch_emails(
        &mut self,
        folder: EmailFolder,
        limit: usize,
    ) -> Result<Vec<EmailMessage>, EmailError> {
        // Simulated fetching
        let mut results = Vec::new();

        for (_, email) in &self.emails {
            if email.folder == folder {
                results.push(email.clone());
                if results.len() >= limit {
                    break;
                }
            }
        }

        Ok(results)
    }

    fn mark_as_read(&mut self, email_id: &str) -> Result<(), EmailError> {
        if let Some(email) = self.emails.get_mut(email_id) {
            email.read = true;
            Ok(())
        } else {
            Err(EmailError::EmailNotFound(email_id.to_string()))
        }
    }

    fn delete_email(&mut self, email_id: &str) -> Result<(), EmailError> {
        if let Some(email) = self.emails.get_mut(email_id) {
            email.folder = EmailFolder::Trash;
            Ok(())
        } else {
            Err(EmailError::EmailNotFound(email_id.to_string()))
        }
    }

    fn move_to_folder(&mut self, email_id: &str, folder: EmailFolder) -> Result<(), EmailError> {
        if let Some(email) = self.emails.get_mut(email_id) {
            email.folder = folder;
            Ok(())
        } else {
            Err(EmailError::EmailNotFound(email_id.to_string()))
        }
    }

    fn name(&self) -> &str {
        "IMAP/SMTP"
    }
}

/// OOP-based Email Client
pub struct EmailClient {
    backend: Box<dyn EmailBackend>,
    accounts: Vec<EmailAccountConfig>,
    current_account: Option<usize>,
    drafts: Vec<EmailMessage>,
    labels: Vec<String>,
}

impl EmailClient {
    pub fn new(backend: Box<dyn EmailBackend>) -> Self {
        Self {
            backend,
            accounts: Vec::new(),
            current_account: None,
            drafts: Vec::new(),
            labels: vec![
                "Important".to_string(),
                "Work".to_string(),
                "Personal".to_string(),
            ],
        }
    }

    /// Add account
    pub fn add_account(&mut self, config: EmailAccountConfig) {
        self.accounts.push(config);
        if self.current_account.is_none() {
            self.current_account = Some(0);
        }
    }

    /// Set current account
    pub fn set_current_account(&mut self, index: usize) -> Result<(), EmailError> {
        if index >= self.accounts.len() {
            return Err(EmailError::AccountNotFound(index));
        }
        self.current_account = Some(index);
        Ok(())
    }

    /// Send email
    pub fn send_email(&mut self, message: EmailMessage) -> Result<(), EmailError> {
        self.backend.send_email(message)
    }

    /// Compose email
    pub fn compose_email(
        &mut self,
        to: Vec<EmailAddress>,
        subject: String,
        body: String,
    ) -> EmailMessage {
        EmailMessage {
            id: format!(
                "email_{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ),
            from: EmailAddress {
                name: "User".to_string(),
                address: self
                    .accounts
                    .get(self.current_account.unwrap_or(0))
                    .map(|a| a.email.clone())
                    .unwrap_or_default(),
            },
            to,
            cc: Vec::new(),
            bcc: Vec::new(),
            subject,
            body,
            body_html: None,
            attachments: Vec::new(),
            sent_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            received_at: 0,
            read: true,
            starred: false,
            folder: EmailFolder::Drafts,
            labels: Vec::new(),
        }
    }

    /// Save draft
    pub fn save_draft(&mut self, message: EmailMessage) {
        self.drafts.push(message);
    }

    /// Get drafts
    pub fn get_drafts(&self) -> &[EmailMessage] {
        &self.drafts
    }

    /// Fetch emails from folder
    pub fn fetch_emails(
        &mut self,
        folder: EmailFolder,
        limit: usize,
    ) -> Result<Vec<EmailMessage>, EmailError> {
        self.backend.fetch_emails(folder, limit)
    }

    /// Mark as read
    pub fn mark_as_read(&mut self, email_id: &str) -> Result<(), EmailError> {
        self.backend.mark_as_read(email_id)
    }

    /// Delete email
    pub fn delete_email(&mut self, email_id: &str) -> Result<(), EmailError> {
        self.backend.delete_email(email_id)
    }

    /// Move to folder
    pub fn move_to_folder(
        &mut self,
        email_id: &str,
        folder: EmailFolder,
    ) -> Result<(), EmailError> {
        self.backend.move_to_folder(email_id, folder)
    }

    /// Star email
    pub fn star_email(&mut self, _email_id: &str, _starred: bool) -> Result<(), EmailError> {
        // In real implementation, would update via backend
        Ok(())
    }

    /// Add label
    pub fn add_label(&mut self, _email_id: &str, _label: String) -> Result<(), EmailError> {
        // In real implementation, would update via backend
        Ok(())
    }

    /// Remove label
    pub fn remove_label(&mut self, _email_id: &str, _label: &str) -> Result<(), EmailError> {
        // In real implementation, would update via backend
        Ok(())
    }

    /// Search emails
    pub fn search_emails(&mut self, filter: EmailFilter) -> Result<Vec<EmailMessage>, EmailError> {
        let mut emails = Vec::new();

        // Fetch from folder if specified
        if let Some(folder) = filter.folder {
            emails = self.backend.fetch_emails(folder, 1000)?;
        } else {
            // Fetch from all folders
            for folder in [EmailFolder::Inbox, EmailFolder::Sent, EmailFolder::Archive] {
                emails.extend(self.backend.fetch_emails(folder, 1000)?);
            }
        }

        // Apply filters
        let filtered: Vec<EmailMessage> = emails
            .into_iter()
            .filter(|email| {
                if let Some(ref from) = filter.from {
                    if !email.from.address.contains(from) {
                        return false;
                    }
                }
                if let Some(ref subject) = filter.subject_contains {
                    if !email
                        .subject
                        .to_lowercase()
                        .contains(&subject.to_lowercase())
                    {
                        return false;
                    }
                }
                if filter.unread_only && email.read {
                    return false;
                }
                if filter.starred_only && !email.starred {
                    return false;
                }
                if filter.has_attachments && email.attachments.is_empty() {
                    return false;
                }
                if let Some(from) = filter.date_from {
                    if email.received_at < from {
                        return false;
                    }
                }
                if let Some(to) = filter.date_to {
                    if email.received_at > to {
                        return false;
                    }
                }
                if let Some(ref label) = filter.label {
                    if !email.labels.contains(label) {
                        return false;
                    }
                }
                true
            })
            .collect();

        Ok(filtered)
    }

    /// Get labels
    pub fn labels(&self) -> &[String] {
        &self.labels
    }

    /// Add custom label
    pub fn add_label_definition(&mut self, label: String) {
        if !self.labels.contains(&label) {
            self.labels.push(label);
        }
    }

    /// Get accounts
    pub fn accounts(&self) -> &[EmailAccountConfig] {
        &self.accounts
    }

    /// Get current account
    pub fn current_account(&self) -> Option<&EmailAccountConfig> {
        self.current_account.and_then(|i| self.accounts.get(i))
    }

    /// Get backend name
    pub fn backend_name(&self) -> &str {
        self.backend.name()
    }
}

impl Default for EmailClient {
    fn default() -> Self {
        let config = EmailAccountConfig {
            email: "user@example.com".to_string(),
            imap_server: "imap.example.com".to_string(),
            imap_port: 993,
            smtp_server: "smtp.example.com".to_string(),
            smtp_port: 587,
            use_ssl: true,
            use_tls: true,
        };

        let mut client = Self::new(Box::new(ImapSmtpBackend::new(config.clone())));
        client.add_account(config);
        client
    }
}

/// Email errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmailError {
    AccountNotFound(usize),
    EmailNotFound(String),
    SendFailed(String),
    FetchFailed(String),
    AuthenticationFailed,
    ConnectionFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_address() {
        let addr = EmailAddress {
            name: "John Doe".to_string(),
            address: "john@example.com".to_string(),
        };
        assert_eq!(addr.address, "john@example.com");
    }

    #[test]
    fn test_email_message() {
        let message = EmailMessage {
            id: "test".to_string(),
            from: EmailAddress {
                name: "Sender".to_string(),
                address: "sender@example.com".to_string(),
            },
            to: vec![EmailAddress {
                name: "Recipient".to_string(),
                address: "recipient@example.com".to_string(),
            }],
            cc: Vec::new(),
            bcc: Vec::new(),
            subject: "Test".to_string(),
            body: "Test body".to_string(),
            body_html: None,
            attachments: Vec::new(),
            sent_at: 1234567890,
            received_at: 1234567890,
            read: false,
            starred: false,
            folder: EmailFolder::Inbox,
            labels: Vec::new(),
        };
        assert_eq!(message.subject, "Test");
    }

    #[test]
    fn test_imap_smtp_backend() {
        let config = EmailAccountConfig {
            email: "user@example.com".to_string(),
            imap_server: "imap.example.com".to_string(),
            imap_port: 993,
            smtp_server: "smtp.example.com".to_string(),
            smtp_port: 587,
            use_ssl: true,
            use_tls: true,
        };
        let backend = ImapSmtpBackend::new(config);
        assert_eq!(backend.name(), "IMAP/SMTP");
    }

    #[test]
    fn test_email_client() {
        let client = EmailClient::default();
        assert_eq!(client.accounts().len(), 1);
    }

    #[test]
    fn test_compose_email() {
        let mut client = EmailClient::default();
        let to = vec![EmailAddress {
            name: "Recipient".to_string(),
            address: "recipient@example.com".to_string(),
        }];
        let message = client.compose_email(to, "Test Subject".to_string(), "Test Body".to_string());
        assert_eq!(message.subject, "Test Subject");
    }
}
