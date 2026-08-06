// SigmaOS Network Protocol Layer
#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SovereignBrowser - Native browser core for SigmaOS
// HTML5/CSS3 rendering, adblocking, and container isolation

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrowserError {
    InvalidUrl,
    RenderingFailed,
    SecurityViolation,
    NetworkError,
    ScriptError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityProfile {
    Standard,
    Strict,
    Incognito,
    Tor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabState {
    Loading,
    Loaded,
    Error,
}

/// Browser tab with container isolation
pub struct BrowserTab {
    pub id: u64,
    pub url: Option<String>,
    pub state: TabState,
    pub container_id: Option<u64>,
    pub security_profile: SecurityProfile,
    pub cookies: BTreeMap<String, String>,
    pub cache: Vec<u8>,
}

impl BrowserTab {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            url: None,
            state: TabState::Loading,
            container_id: None,
            security_profile: SecurityProfile::Standard,
            cookies: BTreeMap::new(),
            cache: Vec::new(),
        }
    }

    pub fn load_url(&mut self, url: String) -> Result<(), BrowserError> {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(BrowserError::InvalidUrl);
        }

        self.url = Some(url);
        self.state = TabState::Loading;
        Ok(())
    }

    pub fn set_container(&mut self, container_id: u64) {
        self.container_id = Some(container_id);
    }

    pub fn set_security_profile(&mut self, profile: SecurityProfile) {
        self.security_profile = profile;
    }

    pub fn set_cookie(&mut self, name: String, value: String) {
        self.cookies.insert(name, value);
    }

    pub fn get_cookie(&self, name: &str) -> Option<&String> {
        self.cookies.get(name)
    }

    pub fn clear_cookies(&mut self) {
        self.cookies.clear();
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    pub fn set_loaded(&mut self) {
        self.state = TabState::Loaded;
    }

    pub fn set_error(&mut self) {
        self.state = TabState::Error;
    }

    pub fn is_incognito(&self) -> bool {
        self.security_profile == SecurityProfile::Incognito
    }
}

/// AdBlock rule engine
pub struct AdBlockRule {
    pub pattern: String,
    pub is_whitelist: bool,
}

impl AdBlockRule {
    pub fn new(pattern: String, is_whitelist: bool) -> Self {
        Self { pattern, is_whitelist }
    }

    pub fn matches(&self, url: &str) -> bool {
        url.contains(&self.pattern)
    }
}

/// Brave Shield adblocking engine
pub struct BraveShield {
    rules: Vec<AdBlockRule>,
    enabled: bool,
}

impl BraveShield {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            enabled: true,
        }
    }

    pub fn add_rule(&mut self, rule: AdBlockRule) {
        self.rules.push(rule);
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn should_block(&self, url: &str) -> bool {
        if !self.enabled {
            return false;
        }

        for rule in &self.rules {
            if rule.matches(url) && !rule.is_whitelist {
                return true;
            }
        }
        false
    }

    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

impl Default for BraveShield {
    fn default() -> Self {
        Self::new()
    }
}

/// Container isolation for Firefox-style containers
pub struct TabContainer {
    pub id: u64,
    pub name: String,
    pub color: String,
    pub icon: String,
    pub tabs: Vec<u64>,
}

impl TabContainer {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            color: "#ffffff".to_string(),
            icon: "folder".to_string(),
            tabs: Vec::new(),
        }
    }

    pub fn add_tab(&mut self, tab_id: u64) {
        self.tabs.push(tab_id);
    }

    pub fn remove_tab(&mut self, tab_id: u64) {
        self.tabs.retain(|&id| id != tab_id);
    }

    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    pub fn set_color(&mut self, color: String) {
        self.color = color;
    }

    pub fn set_icon(&mut self, icon: String) {
        self.icon = icon;
    }
}

/// SovereignBrowser core
pub struct SovereignBrowser {
    tabs: BTreeMap<u64, BrowserTab>,
    containers: BTreeMap<u64, TabContainer>,
    shield: BraveShield,
    next_tab_id: u64,
    next_container_id: u64,
}

impl SovereignBrowser {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            tabs: BTreeMap::new(),
            containers: BTreeMap::new(),
            shield: BraveShield::new(),
            next_tab_id: 1,
            next_container_id: 1,
        }
    }

    /// Create a new tab
    pub fn create_tab(&mut self) -> u64 {
        let tab_id = self.next_tab_id;
        self.next_tab_id += 1;

        let tab = BrowserTab::new(tab_id);
        self.tabs.insert(tab_id, tab);

        tab_id
    }

    /// Create a new container
    pub fn create_container(&mut self, name: String) -> u64 {
        let container_id = self.next_container_id;
        self.next_container_id += 1;

        let container = TabContainer::new(container_id, name);
        self.containers.insert(container_id, container);

        container_id
    }

    /// Load URL in a tab
    pub fn load_url(&mut self, tab_id: u64, url: String) -> Result<(), BrowserError> {
        let tab = self.tabs.get_mut(&tab_id)
            .ok_or(BrowserError::ScriptError)?;

        // Check adblock
        if self.shield.should_block(&url) {
            return Err(BrowserError::SecurityViolation);
        }

        tab.load_url(url)
    }

    /// Get tab by ID
    pub fn get_tab(&self, tab_id: u64) -> Option<&BrowserTab> {
        self.tabs.get(&tab_id)
    }

    /// Get container by ID
    pub fn get_container(&self, container_id: u64) -> Option<&TabContainer> {
        self.containers.get(&container_id)
    }

    /// Assign tab to container
    pub fn assign_tab_to_container(&mut self, tab_id: u64, container_id: u64) -> Result<(), BrowserError> {
        let tab = self.tabs.get_mut(&tab_id)
            .ok_or(BrowserError::ScriptError)?;

        let container = self.containers.get_mut(&container_id)
            .ok_or(BrowserError::ScriptError)?;

        tab.set_container(container_id);
        container.add_tab(tab_id);

        Ok(())
    }

    /// Set security profile for tab
    pub fn set_tab_security_profile(&mut self, tab_id: u64, profile: SecurityProfile) -> Result<(), BrowserError> {
        let tab = self.tabs.get_mut(&tab_id)
            .ok_or(BrowserError::ScriptError)?;

        tab.set_security_profile(profile);

        // Clear cookies and cache for incognito
        if profile == SecurityProfile::Incognito {
            tab.clear_cookies();
            tab.clear_cache();
        }

        Ok(())
    }

    /// Add adblock rule
    pub fn add_adblock_rule(&mut self, pattern: String, is_whitelist: bool) {
        self.shield.add_rule(AdBlockRule::new(pattern, is_whitelist));
    }

    /// Enable/disable adblock
    pub fn set_adblock_enabled(&mut self, enabled: bool) {
        if enabled {
            self.shield.enable();
        } else {
            self.shield.disable();
        }
    }

    /// Close tab
    pub fn close_tab(&mut self, tab_id: u64) -> Result<(), BrowserError> {
        let tab = self.tabs.remove(&tab_id)
            .ok_or(BrowserError::ScriptError)?;

        // Remove from container
        if let Some(container_id) = tab.container_id {
            if let Some(container) = self.containers.get_mut(&container_id) {
                container.remove_tab(tab_id);
            }
        }

        Ok(())
    }

    /// Get tab count
    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    /// Get container count
    pub fn container_count(&self) -> usize {
        self.containers.len()
    }

    /// List all tabs
    pub fn list_tabs(&self) -> Vec<&BrowserTab> {
        self.tabs.values().collect()
    }

    /// List all containers
    pub fn list_containers(&self) -> Vec<&TabContainer> {
        self.containers.values().collect()
    }
}

impl Default for SovereignBrowser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_tab_creation() {
        let mut browser = SovereignBrowser::new();
        let tab_id = browser.create_tab();

        assert_eq!(browser.tab_count(), 1);
        assert!(browser.get_tab(tab_id).is_some());
    }

    #[test]
    fn test_url_loading() {
        let mut browser = SovereignBrowser::new();
        let tab_id = browser.create_tab();

        let result = browser.load_url(tab_id, "https://example.com".to_string());
        assert!(result.is_ok());

        let tab = browser.get_tab(tab_id).unwrap();
        assert_eq!(tab.url, Some("https://example.com".to_string()));
    }

    #[test]
    fn test_invalid_url() {
        let mut browser = SovereignBrowser::new();
        let tab_id = browser.create_tab();

        let result = browser.load_url(tab_id, "not-a-url".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_adblock() {
        let mut browser = SovereignBrowser::new();
        browser.add_adblock_rule("ads.com".to_string(), false);

        let tab_id = browser.create_tab();
        let result = browser.load_url(tab_id, "https://ads.com/banner".to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_adblock_whitelist() {
        let mut browser = SovereignBrowser::new();
        browser.add_adblock_rule("ads.com".to_string(), false);
        browser.add_adblock_rule("ads.com/allowed".to_string(), true);

        let tab_id = browser.create_tab();
        let result = browser.load_url(tab_id, "https://ads.com/allowed".to_string());

        assert!(result.is_ok());
    }

    #[test]
    fn test_container_creation() {
        let mut browser = SovereignBrowser::new();
        let container_id = browser.create_container("Personal".to_string());

        assert_eq!(browser.container_count(), 1);
        assert!(browser.get_container(container_id).is_some());
    }

    #[test]
    fn test_tab_container_assignment() {
        let mut browser = SovereignBrowser::new();
        let tab_id = browser.create_tab();
        let container_id = browser.create_container("Work".to_string());

        browser.assign_tab_to_container(tab_id, container_id).unwrap();

        let tab = browser.get_tab(tab_id).unwrap();
        assert_eq!(tab.container_id, Some(container_id));

        let container = browser.get_container(container_id).unwrap();
        assert_eq!(container.tab_count(), 1);
    }

    #[test]
    fn test_security_profile() {
        let mut browser = SovereignBrowser::new();
        let tab_id = browser.create_tab();

        browser.set_tab_security_profile(tab_id, SecurityProfile::Incognito).unwrap();

        let tab = browser.get_tab(tab_id).unwrap();
        assert!(tab.is_incognito());
    }

    #[test]
    fn test_cookies() {
        let mut browser = SovereignBrowser::new();
        let tab_id = browser.create_tab();

        let tab = browser.tabs.get_mut(&tab_id).unwrap();
        tab.set_cookie("session".to_string(), "abc123".to_string());

        assert_eq!(tab.get_cookie("session"), Some(&"abc123".to_string()));
    }

    #[test]
    fn test_close_tab() {
        let mut browser = SovereignBrowser::new();
        let tab_id = browser.create_tab();
        let container_id = browser.create_container("Test".to_string());

        browser.assign_tab_to_container(tab_id, container_id).unwrap();
        browser.close_tab(tab_id).unwrap();

        assert_eq!(browser.tab_count(), 0);

        let container = browser.get_container(container_id).unwrap();
        assert_eq!(container.tab_count(), 0);
    }

    #[test]
    fn test_adblock_toggle() {
        let mut browser = SovereignBrowser::new();
        browser.add_adblock_rule("ads.com".to_string(), false);
        browser.set_adblock_enabled(false);

        let tab_id = browser.create_tab();
        let result = browser.load_url(tab_id, "https://ads.com/banner".to_string());

        assert!(result.is_ok()); // Should not block when disabled
    }
}
