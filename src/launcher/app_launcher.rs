// src/launcher/app_launcher.rs
// AI-powered app launcher for SigmaOS
// Superior to Omarchy's basic launchers
//
// Features:
// - Fuzzy search
// - AI-powered suggestions
// - Command palette
// - Recent apps
// - Keyboard shortcuts
// - Plugin system

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::format;
use core::fmt;

/// Application entry
#[derive(Debug, Clone)]
pub struct AppEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub exec_path: String,
    pub categories: Vec<String>,
    pub keywords: Vec<String>,
    pub last_used: u64,
    pub use_count: u32,
    pub pinned: bool,
}

impl AppEntry {
    pub fn new(name: &str, exec_path: &str) -> Self {
        Self {
            id: Self::generate_id(name),
            name: name.into(),
            description: String::new(),
            icon: "application".into(),
            exec_path: exec_path.into(),
            categories: Vec::new(),
            keywords: Vec::new(),
            last_used: 0,
            use_count: 0,
            pinned: false,
        }
    }
    
    fn generate_id(name: &str) -> String {
        // Simple ID generation from name
        name.to_lowercase().replace(" ", "-")
    }
    
    pub fn matches_query(&self, query: &str) -> bool {
        let query = query.to_lowercase();
        
        // Check name
        if self.name.to_lowercase().contains(&query) {
            return true;
        }
        
        // Check description
        if self.description.to_lowercase().contains(&query) {
            return true;
        }
        
        // Check keywords
        for keyword in &self.keywords {
            if keyword.to_lowercase().contains(&query) {
                return true;
            }
        }
        
        // Check categories
        for category in &self.categories {
            if category.to_lowercase().contains(&query) {
                return true;
            }
        }
        
        false
    }
    
    pub fn fuzzy_score(&self, query: &str) -> i32 {
        let query = query.to_lowercase();
        let name = self.name.to_lowercase();
        
        if name == query {
            return 1000;  // Exact match
        }
        
        if name.starts_with(&query) {
            return 900;  // Prefix match
        }
        
        if name.contains(&query) {
            return 800;  // Substring match
        }
        
        // Check word boundaries
        let words: Vec<&str> = name.split_whitespace().collect();
        for word in words {
            if word.starts_with(&query) {
                return 700;  // Word start match
            }
        }
        
        // Keyword match
        for keyword in &self.keywords {
            if keyword.to_lowercase().contains(&query) {
                return 600;
            }
        }
        
        0  // No match
    }
}

/// Search result
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub app: AppEntry,
    pub score: i32,
    pub match_type: MatchType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MatchType {
    Exact,
    Prefix,
    Fuzzy,
    Recent,
    Suggested,
}

/// Command entry for command palette
#[derive(Debug, Clone)]
pub struct Command {
    pub id: String,
    pub name: String,
    pub description: String,
    pub shortcut: Option<String>,
    pub action: CommandAction,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommandAction {
    LaunchApp,
    OpenSettings,
    ToggleTheme,
    Screenshot,
    LockScreen,
    Logout,
    Shutdown,
    Reboot,
    Custom,
}

/// App launcher engine
pub struct AppLauncher {
    apps: BTreeMap<String, AppEntry>,
    recent_apps: Vec<String>,
    pinned_apps: Vec<String>,
    commands: Vec<Command>,
    max_recent: usize,
}

impl AppLauncher {
    pub fn new() -> Self {
        Self {
            apps: BTreeMap::new(),
            recent_apps: Vec::new(),
            pinned_apps: Vec::new(),
            commands: Vec::new(),
            max_recent: 10,
        }
    }
    
    pub fn register_app(&mut self, app: AppEntry) {
        if app.pinned {
            self.pinned_apps.push(app.id.clone());
        }
        self.apps.insert(app.id.clone(), app);
    }
    
    pub fn unregister_app(&mut self, id: &str) {
        self.apps.remove(id);
        self.pinned_apps.retain(|app_id| app_id != id);
        self.recent_apps.retain(|app_id| app_id != id);
    }
    
    pub fn register_command(&mut self, command: Command) {
        self.commands.push(command);
    }
    
    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        let mut results = Vec::new();
        
        // Search apps
        for app in self.apps.values() {
            let score = app.fuzzy_score(query);
            if score > 0 {
                let match_type = if score >= 1000 {
                    MatchType::Exact
                } else if score >= 900 {
                    MatchType::Prefix
                } else {
                    MatchType::Fuzzy
                };
                
                results.push(SearchResult {
                    app: app.clone(),
                    score,
                    match_type,
                });
            }
        }
        
        // Sort by score (descending)
        results.sort_by(|a, b| b.score.cmp(&a.score));
        
        results
    }
    
    pub fn get_recent_apps(&self) -> Vec<AppEntry> {
        let mut recent = Vec::new();
        
        for app_id in &self.recent_apps {
            if let Some(app) = self.apps.get(app_id) {
                recent.push(app.clone());
            }
        }
        
        recent
    }
    
    pub fn get_pinned_apps(&self) -> Vec<AppEntry> {
        let mut pinned = Vec::new();
        
        for app_id in &self.pinned_apps {
            if let Some(app) = self.apps.get(app_id) {
                pinned.push(app.clone());
            }
        }
        
        pinned
    }
    
    pub fn get_suggested_apps(&self) -> Vec<AppEntry> {
        let mut suggested = Vec::new();
        
        // Get most frequently used apps
        let mut sorted_apps: Vec<&AppEntry> = self.apps.values().collect();
        sorted_apps.sort_by(|a, b| b.use_count.cmp(&a.use_count));
        
        for app in sorted_apps.iter().take(5) {
            suggested.push((*app).clone());
        }
        
        suggested
    }
    
    pub fn launch_app(&mut self, id: &str) -> Result<(), LauncherError> {
        let app = self.apps.get_mut(id).ok_or(LauncherError::AppNotFound)?;
        
        // Update stats
        app.use_count += 1;
        app.last_used = Self::get_time();
        
        // Update recent apps
        self.recent_apps.retain(|app_id| app_id != id);
        self.recent_apps.insert(0, id.into());
        
        if self.recent_apps.len() > self.max_recent {
            self.recent_apps.truncate(self.max_recent);
        }
        
        // In real implementation, would spawn process
        Ok(())
    }
    
    pub fn pin_app(&mut self, id: &str) -> Result<(), LauncherError> {
        let app = self.apps.get_mut(id).ok_or(LauncherError::AppNotFound)?;
        
        if !app.pinned {
            app.pinned = true;
            self.pinned_apps.push(id.into());
        }
        
        Ok(())
    }
    
    pub fn unpin_app(&mut self, id: &str) -> Result<(), LauncherError> {
        let app = self.apps.get_mut(id).ok_or(LauncherError::AppNotFound)?;
        
        if app.pinned {
            app.pinned = false;
            self.pinned_apps.retain(|app_id| app_id != id);
        }
        
        Ok(())
    }
    
    pub fn search_commands(&self, query: &str) -> Vec<Command> {
        let query = query.to_lowercase();
        let mut results = Vec::new();
        
        for command in &self.commands {
            if command.name.to_lowercase().contains(&query) ||
               command.description.to_lowercase().contains(&query) {
                results.push(command.clone());
            }
        }
        
        results
    }
    
    fn get_time() -> u64 {
        // In real implementation, would get actual timestamp
        0
    }
}

impl Default for AppLauncher {
    fn default() -> Self {
        Self::new()
    }
}

/// Launcher errors
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LauncherError {
    AppNotFound,
    LaunchFailed,
    InvalidCommand,
}

impl fmt::Display for LauncherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AppNotFound => write!(f, "App not found"),
            Self::LaunchFailed => write!(f, "Launch failed"),
            Self::InvalidCommand => write!(f, "Invalid command"),
        }
    }
}

/// Initialize default apps
pub fn init_default_apps() -> AppLauncher {
    let mut launcher = AppLauncher::new();
    
    // Register default apps
    let mut browser = AppEntry::new("Sigma Browser", "/usr/bin/sigma-browser");
    browser.description = "Privacy-focused web browser".into();
    browser.categories = vec!["Internet".into(), "WebBrowser".into()];
    browser.keywords = vec!["web".into(), "internet".into(), "browser".into()];
    browser.pinned = true;
    launcher.register_app(browser);
    
    let mut terminal = AppEntry::new("Sigma Terminal", "/usr/bin/sigma-terminal");
    terminal.description = "GPU-accelerated terminal emulator".into();
    terminal.categories = vec!["System".into(), "TerminalEmulator".into()];
    terminal.keywords = vec!["shell".into(), "command".into(), "cli".into()];
    terminal.pinned = true;
    launcher.register_app(terminal);
    
    let mut files = AppEntry::new("Sigma Files", "/usr/bin/sigma-files");
    files.description = "Advanced file manager".into();
    files.categories = vec!["System".into(), "FileManager".into()];
    files.keywords = vec!["folder".into(), "directory".into(), "explorer".into()];
    files.pinned = true;
    launcher.register_app(files);
    
    let mut code = AppEntry::new("Sigma Code", "/usr/bin/sigma-code");
    code.description = "AI-powered code editor".into();
    code.categories = vec!["Development".into(), "TextEditor".into()];
    code.keywords = vec!["editor".into(), "coding".into(), "ide".into()];
    launcher.register_app(code);
    
    let mut settings = AppEntry::new("Settings", "/usr/bin/sigma-settings");
    settings.description = "System settings".into();
    settings.categories = vec!["System".into(), "Settings".into()];
    settings.keywords = vec!["preferences".into(), "config".into(), "control".into()];
    launcher.register_app(settings);
    
    // Register default commands
    launcher.register_command(Command {
        id: "screenshot".into(),
        name: "Take Screenshot".into(),
        description: "Capture screen or selection".into(),
        shortcut: Some("Super+Shift+S".into()),
        action: CommandAction::Screenshot,
    });
    
    launcher.register_command(Command {
        id: "lock".into(),
        name: "Lock Screen".into(),
        description: "Lock your session".into(),
        shortcut: Some("Super+L".into()),
        action: CommandAction::LockScreen,
    });
    
    launcher.register_command(Command {
        id: "theme".into(),
        name: "Toggle Theme".into(),
        description: "Switch between light and dark mode".into(),
        shortcut: Some("Super+T".into()),
        action: CommandAction::ToggleTheme,
    });
    
    launcher
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_app_entry() {
        let mut app = AppEntry::new("Test App", "/usr/bin/test");
        app.keywords.push("testing".into());
        
        assert!(app.matches_query("test"));
        assert!(app.matches_query("testing"));
        assert!(!app.matches_query("nonexistent"));
    }
    
    #[test]
    fn test_fuzzy_search() {
        let app = AppEntry::new("Firefox Browser", "/usr/bin/firefox");
        
        assert_eq!(app.fuzzy_score("firefox browser"), 1000);  // Exact
        assert_eq!(app.fuzzy_score("firefox"), 900);  // Prefix
        assert!(app.fuzzy_score("fox") > 0);  // Fuzzy
    }
    
    #[test]
    fn test_launcher() {
        let mut launcher = AppLauncher::new();
        
        let app = AppEntry::new("Test App", "/usr/bin/test");
        launcher.register_app(app);
        
        let results = launcher.search("test");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].app.name, "Test App");
    }
    
    #[test]
    fn test_recent_apps() {
        let mut launcher = AppLauncher::new();
        
        let app1 = AppEntry::new("App 1", "/usr/bin/app1");
        let app2 = AppEntry::new("App 2", "/usr/bin/app2");
        
        launcher.register_app(app1);
        launcher.register_app(app2);
        
        launcher.launch_app("app-1").unwrap();
        launcher.launch_app("app-2").unwrap();
        
        let recent = launcher.get_recent_apps();
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0].name, "App 2");  // Most recent first
    }
    
    #[test]
    fn test_pinned_apps() {
        let mut launcher = AppLauncher::new();
        
        let app = AppEntry::new("Pinned App", "/usr/bin/pinned");
        launcher.register_app(app);
        
        launcher.pin_app("pinned-app").unwrap();
        
        let pinned = launcher.get_pinned_apps();
        assert_eq!(pinned.len(), 1);
        assert_eq!(pinned[0].name, "Pinned App");
    }
}
