// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaMail - System Mail Server
//!
//! SigmaMail provides system mail server functionality for SigmaOS, inspired by Postfix/Exim from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Mail message state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MailMessageState {
    /// Queued
    Queued,
    /// Sending
    Sending,
    /// Sent
    Sent,
    /// Failed
    Failed,
    /// Bounced
    Bounced,
}

impl fmt::Display for MailMessageState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MailMessageState::Queued => write!(f, "queued"),
            MailMessageState::Sending => write!(f, "sending"),
            MailMessageState::Sent => write!(f, "sent"),
            MailMessageState::Failed => write!(f, "failed"),
            MailMessageState::Bounced => write!(f, "bounced"),
        }
    }
}

/// Mail message
#[derive(Debug, Clone)]
pub struct MailMessage {
    /// Message ID
    pub message_id: String,
    /// Sender
    pub sender: String,
    /// Recipients
    pub recipients: Vec<String>,
    /// Subject
    pub subject: String,
    /// State
    pub state: MailMessageState,
    /// Size in bytes
    pub size: u64,
}

impl MailMessage {
    /// Create a new mail message
    pub fn new(
        message_id: String,
        sender: String,
        recipients: Vec<String>,
        subject: String,
    ) -> Self {
        Self {
            message_id,
            sender,
            recipients,
            subject,
            state: MailMessageState::Queued,
            size: 0,
        }
    }

    /// Set state
    pub fn with_state(mut self, state: MailMessageState) -> Self {
        self.state = state;
        self
    }

    /// Set size
    pub fn with_size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }
}

/// SigmaMail - System Mail Server
pub struct SigmaMail {
    /// Mail messages (message_id -> message)
    messages: BTreeMap<String, MailMessage>,
    /// Running state
    running: bool,
    /// Message counter
    message_counter: u32,
}

impl SigmaMail {
    /// Create a new SigmaMail instance
    pub fn new() -> Self {
        Self {
            messages: BTreeMap::new(),
            running: false,
            message_counter: 0,
        }
    }

    /// Start the mail server
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the mail server
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the server is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add a mail message
    pub fn add_message(&mut self, message: MailMessage) -> Result<(), String> {
        if self.messages.contains_key(&message.message_id) {
            return Err(format!("Message '{}' already exists", message.message_id));
        }

        self.messages.insert(message.message_id.clone(), message);
        self.message_counter += 1;
        Ok(())
    }

    /// Remove a mail message
    pub fn remove_message(&mut self, message_id: &str) -> Result<(), String> {
        if self.messages.remove(message_id).is_none() {
            return Err(format!("Message '{}' not found", message_id));
        }
        Ok(())
    }

    /// Send a message
    pub fn send_message(&mut self, message_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaMail not running".to_string());
        }

        match self.messages.get_mut(message_id) {
            Some(message) => {
                message.state = MailMessageState::Sending;
                Ok(())
            }
            None => Err(format!("Message '{}' not found", message_id)),
        }
    }

    /// Mark message as sent
    pub fn mark_sent(&mut self, message_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaMail not running".to_string());
        }

        match self.messages.get_mut(message_id) {
            Some(message) => {
                message.state = MailMessageState::Sent;
                Ok(())
            }
            None => Err(format!("Message '{}' not found", message_id)),
        }
    }

    /// Mark message as failed
    pub fn mark_failed(&mut self, message_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaMail not running".to_string());
        }

        match self.messages.get_mut(message_id) {
            Some(message) => {
                message.state = MailMessageState::Failed;
                Ok(())
            }
            None => Err(format!("Message '{}' not found", message_id)),
        }
    }

    /// Mark message as bounced
    pub fn mark_bounced(&mut self, message_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaMail not running".to_string());
        }

        match self.messages.get_mut(message_id) {
            Some(message) => {
                message.state = MailMessageState::Bounced;
                Ok(())
            }
            None => Err(format!("Message '{}' not found", message_id)),
        }
    }

    /// Get a mail message
    pub fn get_message(&self, message_id: &str) -> Option<&MailMessage> {
        self.messages.get(message_id)
    }

    /// List all messages
    pub fn list_messages(&self) -> Vec<&MailMessage> {
        self.messages.values().collect()
    }

    /// List messages by state
    pub fn list_by_state(&self, state: MailMessageState) -> Vec<&MailMessage> {
        self.messages
            .values()
            .filter(|m| m.state == state)
            .collect()
    }

    /// List messages by sender
    pub fn list_by_sender(&self, sender: &str) -> Vec<&MailMessage> {
        self.messages
            .values()
            .filter(|m| m.sender == sender)
            .collect()
    }

    /// Get message count
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    /// Get mail server statistics
    pub fn mail_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert(
            "total_messages".to_string(),
            self.message_count().to_string(),
        );
        stats.insert(
            "queued_messages".to_string(),
            self.list_by_state(MailMessageState::Queued)
                .len()
                .to_string(),
        );
        stats.insert(
            "sending_messages".to_string(),
            self.list_by_state(MailMessageState::Sending)
                .len()
                .to_string(),
        );
        stats.insert(
            "sent_messages".to_string(),
            self.list_by_state(MailMessageState::Sent).len().to_string(),
        );
        stats.insert(
            "failed_messages".to_string(),
            self.list_by_state(MailMessageState::Failed)
                .len()
                .to_string(),
        );
        stats.insert(
            "bounced_messages".to_string(),
            self.list_by_state(MailMessageState::Bounced)
                .len()
                .to_string(),
        );
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaMail {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaMail instance (placeholder for actual global state)
static mut SIGMAMAIL_INSTANCE: Option<SigmaMail> = None;

/// Initialize the global SigmaMail instance
pub fn init_sigmamail() {
    unsafe {
        SIGMAMAIL_INSTANCE = Some(SigmaMail::new());
    }
}

/// Get the global SigmaMail instance
pub fn get_sigmamail() -> Option<&'static mut SigmaMail> {
    unsafe { SIGMAMAIL_INSTANCE.as_mut() }
}

/// Convenience function to send a message
pub fn send_mail_message(message_id: &str) -> Result<(), String> {
    if let Some(mail) = get_sigmamail() {
        mail.send_message(message_id)
    } else {
        Err("SigmaMail not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mail_message_state_display() {
        assert_eq!(MailMessageState::Queued.to_string(), "queued");
        assert_eq!(MailMessageState::Sent.to_string(), "sent");
    }

    #[test]
    fn test_mail_message() {
        let message = MailMessage::new(
            "msg1".to_string(),
            "sender@example.com".to_string(),
            vec!["recipient@example.com".to_string()],
            "Test Subject".to_string(),
        )
        .with_state(MailMessageState::Sent)
        .with_size(1024);

        assert_eq!(message.message_id, "msg1");
        assert_eq!(message.state, MailMessageState::Sent);
        assert_eq!(message.size, 1024);
    }

    #[test]
    fn test_sigmamail() {
        let mut mail = SigmaMail::new();
        assert!(!mail.is_running());
        mail.start();
        assert!(mail.is_running());
        mail.stop();
        assert!(!mail.is_running());
    }

    #[test]
    fn test_sigmamail_add_message() {
        let mut mail = SigmaMail::new();

        let message = MailMessage::new(
            "msg1".to_string(),
            "sender@example.com".to_string(),
            vec!["recipient@example.com".to_string()],
            "Test Subject".to_string(),
        );

        assert!(mail.add_message(message).is_ok());
        assert_eq!(mail.message_count(), 1);
    }

    #[test]
    fn test_sigmamail_send_message() {
        let mut mail = SigmaMail::new();
        mail.start();

        let message = MailMessage::new(
            "msg1".to_string(),
            "sender@example.com".to_string(),
            vec!["recipient@example.com".to_string()],
            "Test Subject".to_string(),
        );

        mail.add_message(message).unwrap();
        assert!(mail.send_message("msg1").is_ok());

        let msg = mail.get_message("msg1").unwrap();
        assert_eq!(msg.state, MailMessageState::Sending);
    }

    #[test]
    fn test_sigmamail_mark_sent() {
        let mut mail = SigmaMail::new();
        mail.start();

        let message = MailMessage::new(
            "msg1".to_string(),
            "sender@example.com".to_string(),
            vec!["recipient@example.com".to_string()],
            "Test Subject".to_string(),
        )
        .with_state(MailMessageState::Sending);

        mail.add_message(message).unwrap();
        assert!(mail.mark_sent("msg1").is_ok());

        let msg = mail.get_message("msg1").unwrap();
        assert_eq!(msg.state, MailMessageState::Sent);
    }

    #[test]
    fn test_sigmamail_mark_failed() {
        let mut mail = SigmaMail::new();
        mail.start();

        let message = MailMessage::new(
            "msg1".to_string(),
            "sender@example.com".to_string(),
            vec!["recipient@example.com".to_string()],
            "Test Subject".to_string(),
        );

        mail.add_message(message).unwrap();
        assert!(mail.mark_failed("msg1").is_ok());

        let msg = mail.get_message("msg1").unwrap();
        assert_eq!(msg.state, MailMessageState::Failed);
    }

    #[test]
    fn test_sigmamail_list_by_state() {
        let mut mail = SigmaMail::new();

        let message = MailMessage::new(
            "msg1".to_string(),
            "sender@example.com".to_string(),
            vec!["recipient@example.com".to_string()],
            "Test Subject".to_string(),
        )
        .with_state(MailMessageState::Sent);

        mail.add_message(message).unwrap();

        let messages = mail.list_by_state(MailMessageState::Sent);
        assert_eq!(messages.len(), 1);
    }

    #[test]
    fn test_sigmamail_list_by_sender() {
        let mut mail = SigmaMail::new();

        let message = MailMessage::new(
            "msg1".to_string(),
            "sender@example.com".to_string(),
            vec!["recipient@example.com".to_string()],
            "Test Subject".to_string(),
        );

        mail.add_message(message).unwrap();

        let messages = mail.list_by_sender("sender@example.com");
        assert_eq!(messages.len(), 1);
    }

    #[test]
    fn test_sigmamail_stats() {
        let mut mail = SigmaMail::new();

        let message = MailMessage::new(
            "msg1".to_string(),
            "sender@example.com".to_string(),
            vec!["recipient@example.com".to_string()],
            "Test Subject".to_string(),
        )
        .with_state(MailMessageState::Sent);

        mail.add_message(message).unwrap();

        let stats = mail.mail_stats();
        assert_eq!(stats.get("total_messages"), Some(&"1".to_string()));
        assert_eq!(stats.get("sent_messages"), Some(&"1".to_string()));
    }

    #[test]
    fn test_send_mail_message_convenience() {
        init_sigmamail();

        let mut mail = get_sigmamail().unwrap();
        mail.start();

        let message = MailMessage::new(
            "msg1".to_string(),
            "sender@example.com".to_string(),
            vec!["recipient@example.com".to_string()],
            "Test Subject".to_string(),
        );

        mail.add_message(message).unwrap();

        let result = send_mail_message("msg1");
        assert!(result.is_ok());
    }
}
