//! Linux Mint mintwelcome-inspired Welcome Screen
//! 
//! This module implements a welcome screen inspired by Linux Mint's mintwelcome,
//! which shows important information about the release and guides new users.

#![allow(dead_code)]

extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Welcome screen section
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WelcomeSection {
    /// Introduction
    Introduction,
    /// Release notes
    ReleaseNotes,
    /// System information
    SystemInfo,
    /// Getting started guide
    GettingStarted,
    /// Software recommendations
    SoftwareRecommendations,
    /// Community resources
    CommunityResources,
    /// Troubleshooting
    Troubleshooting,
    /// Custom section
    Custom(String),
}

/// Welcome screen content item
#[derive(Debug, Clone)]
pub struct WelcomeContent {
    /// Section identifier
    pub section: WelcomeSection,
    /// Title
    pub title: String,
    /// Content body
    pub content: String,
    /// URL for more information (optional)
    pub url: Option<String>,
    /// Icon name (optional)
    pub icon: Option<String>,
}

/// System information for welcome screen
#[derive(Debug, Clone)]
pub struct WelcomeSystemInfo {
    /// OS name
    pub os_name: String,
    /// OS version
    pub os_version: String,
    /// OS edition
    pub edition: String,
    /// Desktop environment
    pub desktop_environment: String,
    /// Kernel version
    pub kernel_version: String,
    /// Architecture
    pub architecture: String,
    /// Installation date
    pub installation_date: String,
}

/// Welcome screen - shows information to new users
#[derive(Debug)]
pub struct MintWelcomeScreen {
    /// System information
    pub system_info: WelcomeSystemInfo,
    /// Welcome content sections
    pub content: Vec<WelcomeContent>,
    /// Whether to show welcome on first boot
    pub show_on_first_boot: bool,
    /// Whether welcome has been shown
    pub has_been_shown: bool,
}

impl MintWelcomeScreen {
    /// Create a new Welcome Screen
    pub fn new(system_info: WelcomeSystemInfo) -> Self {
        Self {
            system_info,
            content: Vec::new(),
            show_on_first_boot: true,
            has_been_shown: false,
        }
    }

    /// Add a content section
    pub fn add_content(&mut self, content: WelcomeContent) {
        self.content.push(content);
    }

    /// Get content by section
    pub fn get_content_by_section(&self, section: WelcomeSection) -> Vec<&WelcomeContent> {
        self.content.iter().filter(|c| c.section == section).collect()
    }

    /// Mark welcome as shown
    pub fn mark_as_shown(&mut self) {
        self.has_been_shown = true;
    }

    /// Reset welcome to show again
    pub fn reset(&mut self) {
        self.has_been_shown = false;
    }

    /// Set whether to show on first boot
    pub fn set_show_on_first_boot(&mut self, show: bool) {
        self.show_on_first_boot = show;
    }

    /// Check if welcome should be shown
    pub fn should_show(&self) -> bool {
        self.show_on_first_boot && !self.has_been_shown
    }

    /// Generate welcome HTML
    pub fn generate_html(&self) -> String {
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n");
        html.push_str("<head>\n");
        html.push_str("<title>Welcome to ");
        html.push_str(&self.system_info.os_name);
        html.push_str("</title>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        
        // Header
        html.push_str("<h1>Welcome to ");
        html.push_str(&self.system_info.os_name);
        html.push_str(" ");
        html.push_str(&self.system_info.os_version);
        html.push_str("</h1>\n");
        
        // System info
        html.push_str("<h2>System Information</h2>\n");
        html.push_str("<ul>\n");
        html.push_str(&format!("<li>Edition: {}</li>\n", self.system_info.edition));
        html.push_str(&format!("<li>Desktop: {}</li>\n", self.system_info.desktop_environment));
        html.push_str(&format!("<li>Kernel: {}</li>\n", self.system_info.kernel_version));
        html.push_str(&format!("<li>Architecture: {}</li>\n", self.system_info.architecture));
        html.push_str("</ul>\n");
        
        // Content sections
        for section in &self.content {
            html.push_str("<h2>");
            html.push_str(&section.title);
            html.push_str("</h2>\n");
            html.push_str("<p>");
            html.push_str(&section.content);
            html.push_str("</p>\n");
            
            if let Some(url) = &section.url {
                html.push_str("<p><a href=\"");
                html.push_str(url);
                html.push_str("\">Learn more</a></p>\n");
            }
        }
        
        html.push_str("</body>\n");
        html.push_str("</html>\n");
        
        html
    }

    /// Generate welcome markdown
    pub fn generate_markdown(&self) -> String {
        let mut md = String::new();
        
        md.push_str("# Welcome to ");
        md.push_str(&self.system_info.os_name);
        md.push_str(" ");
        md.push_str(&self.system_info.os_version);
        md.push_str("\n\n");
        
        // System info
        md.push_str("## System Information\n\n");
        md.push_str(&format!("- **Edition:** {}\n", self.system_info.edition));
        md.push_str(&format!("- **Desktop:** {}\n", self.system_info.desktop_environment));
        md.push_str(&format!("- **Kernel:** {}\n", self.system_info.kernel_version));
        md.push_str(&format!("- **Architecture:** {}\n", self.system_info.architecture));
        md.push_str("\n");
        
        // Content sections
        for section in &self.content {
            md.push_str("## ");
            md.push_str(&section.title);
            md.push_str("\n\n");
            md.push_str(&section.content);
            md.push_str("\n\n");
            
            if let Some(url) = &section.url {
                md.push_str("[Learn more](");
                md.push_str(url);
                md.push_str(")\n\n");
            }
        }
        
        md
    }
}

impl Default for MintWelcomeScreen {
    fn default() -> Self {
        Self::new(WelcomeSystemInfo {
            os_name: "SigmaOS".to_string(),
            os_version: "1.0".to_string(),
            edition: "Standard".to_string(),
            desktop_environment: "SigmaOS Desktop".to_string(),
            kernel_version: "0.1.0".to_string(),
            architecture: "x86_64".to_string(),
            installation_date: "2024-01-01".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welcome_screen_creation() {
        let system_info = WelcomeSystemInfo {
            os_name: "SigmaOS".to_string(),
            os_version: "1.0".to_string(),
            edition: "Standard".to_string(),
            desktop_environment: "SigmaOS Desktop".to_string(),
            kernel_version: "0.1.0".to_string(),
            architecture: "x86_64".to_string(),
            installation_date: "2024-01-01".to_string(),
        };
        
        let welcome = MintWelcomeScreen::new(system_info);
        assert_eq!(welcome.content.len(), 0);
        assert!(welcome.show_on_first_boot);
    }

    #[test]
    fn test_add_content() {
        let mut welcome = MintWelcomeScreen::default();
        
        let content = WelcomeContent {
            section: WelcomeSection::Introduction,
            title: "Welcome".to_string(),
            content: "Welcome to SigmaOS".to_string(),
            url: None,
            icon: None,
        };
        
        welcome.add_content(content);
        assert_eq!(welcome.content.len(), 1);
    }

    #[test]
    fn test_should_show() {
        let welcome = MintWelcomeScreen::default();
        assert!(welcome.should_show());
        
        let mut welcome = MintWelcomeScreen::default();
        welcome.mark_as_shown();
        assert!(!welcome.should_show());
    }

    #[test]
    fn test_generate_markdown() {
        let mut welcome = MintWelcomeScreen::default();
        
        let content = WelcomeContent {
            section: WelcomeSection::Introduction,
            title: "Welcome".to_string(),
            content: "Welcome to SigmaOS".to_string(),
            url: None,
            icon: None,
        };
        
        welcome.add_content(content);
        let md = welcome.generate_markdown();
        assert!(md.contains("Welcome to SigmaOS"));
        assert!(md.contains("## Welcome"));
    }

    #[test]
    fn test_generate_html() {
        let mut welcome = MintWelcomeScreen::default();
        
        let content = WelcomeContent {
            section: WelcomeSection::Introduction,
            title: "Welcome".to_string(),
            content: "Welcome to SigmaOS".to_string(),
            url: None,
            icon: None,
        };
        
        welcome.add_content(content);
        let html = welcome.generate_html();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Welcome to SigmaOS"));
    }
}
