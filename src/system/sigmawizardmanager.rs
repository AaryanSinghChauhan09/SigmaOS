// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Aaryan Singh Chauhan

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Wizard page type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WizardPageType {
    Welcome,
    Input,
    Selection,
    Confirmation,
    Progress,
    Completion,
}

/// Wizard page
#[derive(Debug, Clone)]
pub struct WizardPage {
    pub id: String,
    pub title: String,
    pub content: String,
    pub page_type: WizardPageType,
    pub options: Vec<String>,
}

/// Wizard manager for managing multi-step wizards
/// Inspired by Omarchy setup wizards and Linux installation wizards
#[derive(Debug, Clone)]
pub struct SigmaWizardManager {
    pages: Vec<WizardPage>,
    current_page: usize,
}

impl SigmaWizardManager {
    pub fn new() -> Self {
        Self {
            pages: Vec::new(),
            current_page: 0,
        }
    }

    /// Add a wizard page
    pub fn add_page(&mut self, page: WizardPage) {
        self.pages.push(page);
    }

    /// Get current page
    pub fn get_current_page(&self) -> Option<&WizardPage> {
        self.pages.get(self.current_page)
    }

    /// Get all pages
    pub fn get_pages(&self) -> &[WizardPage] {
        &self.pages
    }

    /// Next page
    pub fn next_page(&mut self) -> Result<String, String> {
        if self.current_page < self.pages.len() - 1 {
            self.current_page += 1;
            Ok(format!("Moved to page: {}", self.current_page))
        } else {
            Err("Already on last page".to_string())
        }
    }

    /// Previous page
    pub fn previous_page(&mut self) -> Result<String, String> {
        if self.current_page > 0 {
            self.current_page -= 1;
            Ok(format!("Moved to page: {}", self.current_page))
        } else {
            Err("Already on first page".to_string())
        }
    }

    /// Go to specific page
    pub fn go_to_page(&mut self, index: usize) -> Result<String, String> {
        if index < self.pages.len() {
            self.current_page = index;
            Ok(format!("Moved to page: {}", index))
        } else {
            Err("Invalid page index".to_string())
        }
    }

    /// Get current page index
    pub fn get_current_index(&self) -> usize {
        self.current_page
    }

    /// Get total pages
    pub fn get_total_pages(&self) -> usize {
        self.pages.len()
    }

    /// Check if on first page
    pub fn is_first_page(&self) -> bool {
        self.current_page == 0
    }

    /// Check if on last page
    pub fn is_last_page(&self) -> bool {
        self.current_page == self.pages.len() - 1
    }

    /// Reset wizard
    pub fn reset(&mut self) {
        self.current_page = 0;
    }
}

impl Default for SigmaWizardManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_page() {
        let mut manager = SigmaWizardManager::new();
        let page = WizardPage {
            id: "welcome".to_string(),
            title: "Welcome".to_string(),
            content: "Welcome to SigmaOS".to_string(),
            page_type: WizardPageType::Welcome,
            options: Vec::new(),
        };
        manager.add_page(page);
        assert_eq!(manager.get_total_pages(), 1);
    }

    #[test]
    fn test_next_page() {
        let mut manager = SigmaWizardManager::new();
        let page1 = WizardPage {
            id: "welcome".to_string(),
            title: "Welcome".to_string(),
            content: "Welcome".to_string(),
            page_type: WizardPageType::Welcome,
            options: Vec::new(),
        };
        let page2 = WizardPage {
            id: "input".to_string(),
            title: "Input".to_string(),
            content: "Enter data".to_string(),
            page_type: WizardPageType::Input,
            options: Vec::new(),
        };
        manager.add_page(page1);
        manager.add_page(page2);
        manager.next_page().unwrap();
        assert_eq!(manager.get_current_index(), 1);
    }

    #[test]
    fn test_previous_page() {
        let mut manager = SigmaWizardManager::new();
        let page1 = WizardPage {
            id: "welcome".to_string(),
            title: "Welcome".to_string(),
            content: "Welcome".to_string(),
            page_type: WizardPageType::Welcome,
            options: Vec::new(),
        };
        let page2 = WizardPage {
            id: "input".to_string(),
            title: "Input".to_string(),
            content: "Enter data".to_string(),
            page_type: WizardPageType::Input,
            options: Vec::new(),
        };
        manager.add_page(page1);
        manager.add_page(page2);
        manager.current_page = 1;
        manager.previous_page().unwrap();
        assert_eq!(manager.get_current_index(), 0);
    }

    #[test]
    fn test_is_first_page() {
        let mut manager = SigmaWizardManager::new();
        let page = WizardPage {
            id: "welcome".to_string(),
            title: "Welcome".to_string(),
            content: "Welcome".to_string(),
            page_type: WizardPageType::Welcome,
            options: Vec::new(),
        };
        manager.add_page(page);
        assert!(manager.is_first_page());
    }

    #[test]
    fn test_is_last_page() {
        let mut manager = SigmaWizardManager::new();
        let page = WizardPage {
            id: "welcome".to_string(),
            title: "Welcome".to_string(),
            content: "Welcome".to_string(),
            page_type: WizardPageType::Welcome,
            options: Vec::new(),
        };
        manager.add_page(page);
        assert!(manager.is_last_page());
    }

    #[test]
    fn test_reset() {
        let mut manager = SigmaWizardManager::new();
        let page1 = WizardPage {
            id: "welcome".to_string(),
            title: "Welcome".to_string(),
            content: "Welcome".to_string(),
            page_type: WizardPageType::Welcome,
            options: Vec::new(),
        };
        let page2 = WizardPage {
            id: "input".to_string(),
            title: "Input".to_string(),
            content: "Enter data".to_string(),
            page_type: WizardPageType::Input,
            options: Vec::new(),
        };
        manager.add_page(page1);
        manager.add_page(page2);
        manager.current_page = 1;
        manager.reset();
        assert_eq!(manager.get_current_index(), 0);
    }
}
