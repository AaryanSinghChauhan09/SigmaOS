// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS Context Manager - Context and state management for AI

use serde::{Deserialize, Serialize};

/// Context Manager for AI conversation context
pub struct ContextManager {
    entries: Vec<ContextEntry>,
    max_length: usize,
}

impl ContextManager {
    /// Create a new Context Manager
    pub fn new(max_length: usize) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            entries: Vec::new(),
            max_length,
        })
    }

    /// Add an entry to the context
    pub fn add_entry(&mut self, role: &str, content: &str) {
        let entry = ContextEntry {
            role: role.to_string(),
            content: content.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };
        
        self.entries.push(entry);
        
        // Trim context if it exceeds max length
        if self.entries.len() > self.max_length {
            self.entries.remove(0);
        }
    }

    /// Get the current context
    pub fn get_context(&self) -> Vec<ContextEntry> {
        self.entries.clone()
    }

    /// Get context as a formatted string
    pub fn get_context_string(&self) -> String {
        self.entries
            .iter()
            .map(|entry| format!("{}: {}", entry.role, entry.content))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Clear the context
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Get the last N entries
    pub fn get_last_entries(&self, n: usize) -> Vec<ContextEntry> {
        let start = if self.entries.len() > n {
            self.entries.len() - n
        } else {
            0
        };
        self.entries[start..].to_vec()
    }

    /// Get context length
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if context is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Set max context length
    pub fn set_max_length(&mut self, max_length: usize) {
        self.max_length = max_length;
        
        // Trim if necessary
        while self.entries.len() > self.max_length {
            self.entries.remove(0);
        }
    }

    /// Get context summary
    pub fn get_summary(&self) -> ContextSummary {
        let user_entries = self.entries.iter().filter(|e| e.role == "user").count();
        let assistant_entries = self.entries.iter().filter(|e| e.role == "assistant").count();
        
        ContextSummary {
            total_entries: self.entries.len(),
            user_entries,
            assistant_entries,
            oldest_entry: self.entries.first().map(|e| e.timestamp.clone()),
            newest_entry: self.entries.last().map(|e| e.timestamp.clone()),
        }
    }
}

/// Context entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextEntry {
    pub role: String,
    pub content: String,
    pub timestamp: String,
}

/// Context summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSummary {
    pub total_entries: usize,
    pub user_entries: usize,
    pub assistant_entries: usize,
    pub oldest_entry: Option<String>,
    pub newest_entry: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_manager_creation() {
        let manager = ContextManager::new(10);
        assert!(manager.is_ok());
    }

    #[test]
    fn test_add_entry() {
        let mut manager = ContextManager::new(10).unwrap();
        manager.add_entry("user", "Hello");
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_context_limit() {
        let mut manager = ContextManager::new(5).unwrap();
        for i in 0..10 {
            manager.add_entry("user", &format!("Message {}", i));
        }
        assert_eq!(manager.len(), 5);
    }
}
