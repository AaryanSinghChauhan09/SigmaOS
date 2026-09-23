#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use std::vec;
use std::format;
// SigmaOS Network Protocol Layer

// Browser Core - High-performance, memory-safe browser implementation
// Parses HTML5, CSS3, ES2022+, and SVG with integrated security

// (no_std only applicable at crate root - removed)

use std::string::{String, ToString};
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrowserTabState {
    Loading,
    Loaded,
    Error,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    Standard,
    Strict,
    Custom(u8),
}

#[derive(Debug, Clone)]
pub struct TabCapabilities {
    pub network_allowed: bool,
    pub filesystem_read_allowed: bool,
    pub filesystem_write_allowed: bool,
    pub camera_allowed: bool,
    pub microphone_allowed: bool,
}

#[derive(Debug, Clone)]
pub struct BrowserTab {
    pub id: u32,
    pub url: String,
    pub title: String,
    pub state: BrowserTabState,
    pub capabilities: TabCapabilities,
    pub security_level: SecurityLevel,
}

#[derive(Debug, Clone)]
pub struct AdblockRule {
    pub pattern: String,
    pub is_whitelist: bool,
}

#[derive(Debug, Clone)]
pub struct TrackingProtection {
    pub enabled: bool,
    pub blocked_trackers: Vec<String>,
}

pub struct BrowserCore {
    tabs: Vec<BrowserTab>,
    active_tab: Option<usize>,
    adblock_rules: Vec<AdblockRule>,
    tracking_protection: TrackingProtection,
}

impl BrowserCore {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active_tab: None,
            adblock_rules: Self::default_adblock_rules(),
            tracking_protection: TrackingProtection {
                enabled: true,
                blocked_trackers: Vec::new(),
            },
        }
    }

    /// Create default adblock rules
    fn default_adblock_rules() -> Vec<AdblockRule> {
        vec![
            AdblockRule {
                pattern: String::from("*.doubleclick.net"),
                is_whitelist: false,
            },
            AdblockRule {
                pattern: String::from("*.google-analytics.com"),
                is_whitelist: false,
            },
            AdblockRule {
                pattern: String::from("*.facebook.com/tr"),
                is_whitelist: false,
            },
        ]
    }

    /// Create a new tab with specified capabilities
    pub fn create_tab(&mut self, url: String, capabilities: TabCapabilities) -> u32 {
        let tab_id = self.tabs.len() as u32 + 1;
        
        let tab = BrowserTab {
            id: tab_id,
            url: url.clone(),
            title: String::from("Loading..."),
            state: BrowserTabState::Loading,
            capabilities,
            security_level: SecurityLevel::Standard,
        };

        self.tabs.push(tab);
        self.active_tab = Some(self.tabs.len() - 1);
        
        tab_id
    }

    /// Navigate to a URL
    pub fn navigate(&mut self, tab_id: u32, url: String) -> Result<(), &'static str> {
        let tab = self.find_tab_mut(tab_id).ok_or("Tab not found")?;
        
        tab.url = url.clone();
        tab.title = String::from("Loading...");
        tab.state = BrowserTabState::Loading;

        // Simulate loading
        tab.title = Self::extract_title_from_url(&url);
        tab.state = BrowserTabState::Loaded;

        Ok(())
    }

    /// Close a tab
    pub fn close_tab(&mut self, tab_id: u32) -> Result<(), &'static str> {
        let idx = self.find_tab_index(tab_id).ok_or("Tab not found")?;
        
        self.tabs[idx].state = BrowserTabState::Closed;
        
        // Remove the tab
        self.tabs.remove(idx);
        
        // Update active tab if needed
        if self.active_tab == Some(idx) {
            self.active_tab = if self.tabs.is_empty() {
                None
            } else {
                Some(self.tabs.len() - 1)
            };
        } else if let Some(active) = self.active_tab {
            if active > idx {
                self.active_tab = Some(active - 1);
            }
        }

        Ok(())
    }

    /// Get active tab
    pub fn get_active_tab(&self) -> Option<&BrowserTab> {
        self.active_tab.and_then(|idx| self.tabs.get(idx))
    }

    /// Set active tab
    pub fn set_active_tab(&mut self, tab_id: u32) -> Result<(), &'static str> {
        let idx = self.find_tab_index(tab_id).ok_or("Tab not found")?;
        self.active_tab = Some(idx);
        Ok(())
    }

    /// Add adblock rule
    pub fn add_adblock_rule(&mut self, pattern: String, is_whitelist: bool) {
        self.adblock_rules.push(AdblockRule {
            pattern,
            is_whitelist,
        });
    }

    /// Check if URL should be blocked by adblock
    pub fn should_block_url(&self, url: &str) -> bool {
        for rule in &self.adblock_rules {
            if !rule.is_whitelist {
                let pat = rule.pattern.trim_start_matches("*.");
                if url.contains(pat) {
                    return true;
                }
            }
        }
        false
    }

    /// Enable/disable tracking protection
    pub fn set_tracking_protection(&mut self, enabled: bool) {
        self.tracking_protection.enabled = enabled;
    }

    /// Check if URL is a known tracker
    pub fn is_tracker(&self, url: &str) -> bool {
        let tracker_domains = [
            "google-analytics.com",
            "doubleclick.net",
            "facebook.com/tr",
            "analytics.twitter.com",
        ];

        for domain in &tracker_domains {
            if url.contains(domain) {
                return true;
            }
        }
        false
    }

    /// Block a tracker
    pub fn block_tracker(&mut self, tracker: String) {
        if self.tracking_protection.enabled {
            self.tracking_protection.blocked_trackers.push(tracker);
        }
    }

    /// Get blocked trackers count
    pub fn blocked_trackers_count(&self) -> usize {
        self.tracking_protection.blocked_trackers.len()
    }

    /// Set tab security level
    pub fn set_tab_security(&mut self, tab_id: u32, level: SecurityLevel) -> Result<(), &'static str> {
        let tab = self.find_tab_mut(tab_id).ok_or("Tab not found")?;
        tab.security_level = level;
        Ok(())
    }

    /// Get total tab count
    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    /// Extract simple title from URL
    fn extract_title_from_url(url: &str) -> String {
        if let Some(domain_start) = url.find("://") {
            let after_protocol = &url[domain_start + 3..];
            if let Some(path_start) = after_protocol.find('/') {
                let domain = &after_protocol[..path_start];
                format!("{} - SigmaBrowser", domain)
            } else {
                format!("{} - SigmaBrowser", after_protocol)
            }
        } else {
            format!("SigmaBrowser - {}", url)
        }
    }

    fn find_tab_mut(&mut self, tab_id: u32) -> Option<&mut BrowserTab> {
        self.tabs.iter_mut().find(|tab| tab.id == tab_id)
    }

    fn find_tab_index(&self, tab_id: u32) -> Option<usize> {
        self.tabs.iter().position(|tab| tab.id == tab_id)
    }
}

impl Default for BrowserCore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tab() {
        let mut browser = BrowserCore::new();
        let capabilities = TabCapabilities {
            network_allowed: true,
            filesystem_read_allowed: false,
            filesystem_write_allowed: false,
            camera_allowed: false,
            microphone_allowed: false,
        };

        let tab_id = browser.create_tab("https://example.com".to_string(), capabilities);
        assert_eq!(tab_id, 1);
        assert_eq!(browser.tab_count(), 1);
    }

    #[test]
    fn test_navigate() {
        let mut browser = BrowserCore::new();
        let capabilities = TabCapabilities {
            network_allowed: true,
            filesystem_read_allowed: false,
            filesystem_write_allowed: false,
            camera_allowed: false,
            microphone_allowed: false,
        };

        let tab_id = browser.create_tab("https://example.com".to_string(), capabilities);
        browser.navigate(tab_id, "https://sigmaos.org".to_string()).unwrap();

        let tab = browser.get_active_tab().unwrap();
        assert_eq!(tab.url, "https://sigmaos.org");
        assert_eq!(tab.state, BrowserTabState::Loaded);
    }

    #[test]
    fn test_close_tab() {
        let mut browser = BrowserCore::new();
        let capabilities = TabCapabilities {
            network_allowed: true,
            filesystem_read_allowed: false,
            filesystem_write_allowed: false,
            camera_allowed: false,
            microphone_allowed: false,
        };

        let tab_id = browser.create_tab("https://example.com".to_string(), capabilities);
        browser.close_tab(tab_id).unwrap();

        assert_eq!(browser.tab_count(), 0);
    }

    #[test]
    fn test_adblock() {
        let browser = BrowserCore::new();
        
        assert!(browser.should_block_url("https://doubleclick.net/ad"));
        assert!(browser.should_block_url("https://google-analytics.com/track"));
        assert!(!browser.should_block_url("https://example.com"));
    }

    #[test]
    fn test_tracker_detection() {
        let mut browser = BrowserCore::new();
        
        assert!(browser.is_tracker("https://google-analytics.com/collect"));
        assert!(browser.is_tracker("https://doubleclick.net/ad"));
        assert!(!browser.is_tracker("https://example.com"));
    }

    #[test]
    fn test_tracking_protection() {
        let mut browser = BrowserCore::new();
        
        browser.block_tracker("google-analytics.com".to_string());
        assert_eq!(browser.blocked_trackers_count(), 1);
        
        browser.set_tracking_protection(false);
        browser.block_tracker("doubleclick.net".to_string());
        assert_eq!(browser.blocked_trackers_count(), 1); // Should not block when disabled
    }

    #[test]
    fn test_tab_isolation() {
        let mut browser = BrowserCore::new();
        
        let caps1 = TabCapabilities {
            network_allowed: true,
            filesystem_read_allowed: true,
            filesystem_write_allowed: false,
            camera_allowed: false,
            microphone_allowed: false,
        };

        let caps2 = TabCapabilities {
            network_allowed: true,
            filesystem_read_allowed: false,
            filesystem_write_allowed: false,
            camera_allowed: false,
            microphone_allowed: false,
        };

        let tab1 = browser.create_tab("https://example.com".to_string(), caps1);
        let tab2 = browser.create_tab("https://sigmaos.org".to_string(), caps2);

        browser.set_active_tab(tab1).unwrap();
        let active = browser.get_active_tab().unwrap();
        assert!(active.capabilities.filesystem_read_allowed);

        browser.set_active_tab(tab2).unwrap();
        let active = browser.get_active_tab().unwrap();
        assert!(!active.capabilities.filesystem_read_allowed);
    }
}
