// Walker Application Launcher
// Omarchy Walker-inspired universal launcher and selector

use std::path::PathBuf;

/// Walker mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalkerMode {
    Applications,
    Clipboard,
    Emoji,
    Theme,
    Background,
    Files,
    Calculator,
    WebSearch,
}

/// Search prefix for different modes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchPrefix {
    Files,
    Emoji,
    Calculator,
    Web,
    Commands,
}

impl SearchPrefix {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(SearchPrefix::Files),
            ':' => Some(SearchPrefix::Emoji),
            '=' => Some(SearchPrefix::Calculator),
            '?' => Some(SearchPrefix::Web),
            '>' => Some(SearchPrefix::Commands),
            _ => None,
        }
    }

    pub fn char(&self) -> char {
        match self {
            SearchPrefix::Files => '.',
            SearchPrefix::Emoji => ':',
            SearchPrefix::Calculator => '=',
            SearchPrefix::Web => '?',
            SearchPrefix::Commands => '>',
        }
    }
}

/// Desktop application entry
#[derive(Debug, Clone)]
pub struct AppEntry {
    pub name: String,
    pub exec: String,
    pub icon: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    pub terminal: bool,
    pub no_display: bool,
    pub pinned: bool,
}

/// Clipboard entry
#[derive(Debug, Clone)]
pub struct ClipboardEntry {
    pub content: String,
    pub timestamp: String,
    pub is_sensitive: bool,
}

/// Emoji entry
#[derive(Debug, Clone)]
pub struct EmojiEntry {
    pub emoji: String,
    pub name: String,
    pub keywords: Vec<String>,
    pub category: String,
}

/// File entry
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub is_directory: bool,
    pub size: u64,
    pub modified: String,
}

/// Search result
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub entry_type: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub score: f64,
    pub action: String,
}

/// Walker configuration
#[derive(Debug, Clone)]
pub struct WalkerConfig {
    pub fuzzy_search: bool,
    pub acronym_search: bool,
    pub case_sensitive: bool,
    pub max_results: usize,
    pub history_size: usize,
    pub auto_web_search: bool,
    pub clipboard_history_size: usize,
    pub file_search_paths: Vec<PathBuf>,
}

impl Default for WalkerConfig {
    fn default() -> Self {
        Self {
            fuzzy_search: true,
            acronym_search: true,
            case_sensitive: false,
            max_results: 10,
            history_size: 50,
            auto_web_search: true,
            clipboard_history_size: 20,
            file_search_paths: vec![PathBuf::from("/home")],
        }
    }
}

/// Walker launcher
#[derive(Debug, Clone)]
pub struct WalkerLauncher {
    pub applications: Vec<AppEntry>,
    pub clipboard_history: Vec<ClipboardEntry>,
    pub emojis: Vec<EmojiEntry>,
    pub config: WalkerConfig,
    pub search_history: Vec<String>,
    pub pinned_apps: Vec<String>,
    pub active_mode: WalkerMode,
}

impl Default for WalkerLauncher {
    fn default() -> Self {
        Self::new()
    }
}

impl WalkerLauncher {
    pub fn new() -> Self {
        WalkerLauncher {
            applications: Vec::new(),
            clipboard_history: Vec::new(),
            emojis: Vec::new(),
            config: WalkerConfig::default(),
            search_history: Vec::new(),
            pinned_apps: Vec::new(),
            active_mode: WalkerMode::Applications,
        }
    }

    /// Register application
    pub fn register_application(&mut self, app: AppEntry) {
        if !app.no_display {
            self.applications.push(app);
        }
    }

    /// Add to clipboard history
    pub fn add_clipboard_entry(&mut self, content: String, is_sensitive: bool) {
        let entry = ClipboardEntry {
            content,
            timestamp: "now".to_string(),
            is_sensitive,
        };

        if !is_sensitive {
            self.clipboard_history
                .retain(|e| e.content != entry.content);
            self.clipboard_history.insert(0, entry);
            if self.clipboard_history.len() > self.config.clipboard_history_size {
                self.clipboard_history.pop();
            }
        }
    }

    /// Register emoji
    pub fn register_emoji(&mut self, emoji: EmojiEntry) {
        self.emojis.push(emoji);
    }

    /// Set active mode
    pub fn set_mode(&mut self, mode: WalkerMode) {
        self.active_mode = mode;
    }

    /// Search applications
    pub fn search_applications(&self, query: &str) -> Vec<SearchResult> {
        let mut results: Vec<SearchResult> = self
            .applications
            .iter()
            .filter(|app| {
                if self.config.case_sensitive {
                    if self.config.fuzzy_search {
                        app.name.contains(query)
                            || app.description.contains(query)
                            || app.keywords.iter().any(|k| k.contains(query))
                    } else if self.config.acronym_search {
                        app.name.contains(query) || self.acronym_match(&app.name, query)
                    } else {
                        app.name == query
                    }
                } else {
                    if self.config.fuzzy_search {
                        contains_ignore_case(&app.name, query)
                            || contains_ignore_case(&app.description, query)
                            || app.keywords.iter().any(|k| contains_ignore_case(k, query))
                    } else if self.config.acronym_search {
                        contains_ignore_case(&app.name, query) || self.acronym_match(&app.name, query)
                    } else {
                        eq_ignore_case(&app.name, query)
                    }
                }
            })
            .map(|app| SearchResult {
                entry_type: "application".to_string(),
                title: app.name.clone(),
                description: app.description.clone(),
                icon: app.icon.clone(),
                score: self.calculate_score(&app.name, &app.description, query),
                action: app.exec.clone(),
            })
            .collect();

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(self.config.max_results);
        results
    }

    /// Search clipboard
    pub fn search_clipboard(&self, query: &str) -> Vec<SearchResult> {
        let mut results: Vec<SearchResult> = self
            .clipboard_history
            .iter()
            .filter(|entry| {
                if self.config.case_sensitive {
                    entry.content.contains(query)
                } else {
                    contains_ignore_case(&entry.content, query)
                }
            })
            .map(|entry| SearchResult {
                entry_type: "clipboard".to_string(),
                title: entry.content.chars().take(50).collect(),
                description: entry.timestamp.clone(),
                icon: "clipboard".to_string(),
                score: 1.0,
                action: entry.content.clone(),
            })
            .collect();

        results.truncate(self.config.max_results);
        results
    }

    /// Search emojis
    pub fn search_emojis(&self, query: &str) -> Vec<SearchResult> {
        let mut results: Vec<SearchResult> = self
            .emojis
            .iter()
            .filter(|emoji| {
                if self.config.case_sensitive {
                    emoji.name.contains(query) || emoji.keywords.iter().any(|k| k.contains(query))
                } else {
                    contains_ignore_case(&emoji.name, query)
                        || emoji.keywords.iter().any(|k| contains_ignore_case(k, query))
                }
            })
            .map(|emoji| SearchResult {
                entry_type: "emoji".to_string(),
                title: emoji.emoji.clone(),
                description: emoji.name.clone(),
                icon: "emoji".to_string(),
                score: 1.0,
                action: emoji.emoji.clone(),
            })
            .collect();

        results.truncate(self.config.max_results);
        results
    }

    /// Universal search with prefix support
    pub fn search(&mut self, query: &str) -> Vec<SearchResult> {
        // Add to search history
        if !query.is_empty() {
            self.search_history.retain(|q| q != query);
            self.search_history.insert(0, query.to_string());
            if self.search_history.len() > self.config.history_size {
                self.search_history.pop();
            }
        }

        // Check for prefix
        let first_char = query.chars().next();
        if let Some(c) = first_char {
            if let Some(prefix) = SearchPrefix::from_char(c) {
                let inner_query = &query[c.len_utf8()..];
                return match prefix {
                    SearchPrefix::Files => self.search_files(inner_query),
                    SearchPrefix::Emoji => self.search_emojis(inner_query),
                    SearchPrefix::Calculator => self.calculate(inner_query),
                    SearchPrefix::Web => self.web_search(inner_query),
                    SearchPrefix::Commands => self.search_commands(inner_query),
                };
            }
        }

        // Default: search applications
        let app_results = self.search_applications(query);

        // Auto web search fallback
        if app_results.is_empty() && self.config.auto_web_search && !query.is_empty() {
            return self.web_search(query);
        }

        app_results
    }

    /// Search files
    pub fn search_files(&self, query: &str) -> Vec<SearchResult> {
        let mut results: Vec<SearchResult> = Vec::new();

        for path in &self.config.file_search_paths {
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    let file_path = entry.path();
                    let name = file_path.file_name().unwrap().to_string_lossy().to_string();

                    let is_match = if self.config.case_sensitive {
                        name.contains(query)
                    } else {
                        contains_ignore_case(&name, query)
                    };

                    if is_match {
                        let metadata = entry.metadata().ok();
                        let is_directory = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);

                        results.push(SearchResult {
                            entry_type: if is_directory { "directory" } else { "file" }.to_string(),
                            title: name.clone(),
                            description: file_path.display().to_string(),
                            icon: if is_directory { "folder" } else { "file" }.to_string(),
                            score: 1.0,
                            action: file_path.display().to_string(),
                        });
                    }
                }
            }
        }

        results.truncate(self.config.max_results);
        results
    }

    /// Calculate expression
    pub fn calculate(&self, expr: &str) -> Vec<SearchResult> {
        let result = if let Ok(value) = self.evaluate_expression(expr) {
            value.to_string()
        } else {
            "Error".to_string()
        };

        vec![SearchResult {
            entry_type: "calculator".to_string(),
            title: result.clone(),
            description: expr.to_string(),
            icon: "calculator".to_string(),
            score: 1.0,
            action: result,
        }]
    }

    /// Simple expression evaluator
    fn evaluate_expression(&self, expr: &str) -> Result<f64, String> {
        let tokens: Vec<&str> = expr.split_whitespace().collect();
        if tokens.len() != 3 {
            return Err("Invalid expression".to_string());
        }

        let a: f64 = tokens[0].parse().map_err(|_| "Invalid number")?;
        let b: f64 = tokens[2].parse().map_err(|_| "Invalid number")?;

        let result = match tokens[1] {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => {
                if b == 0.0 {
                    return Err("Division by zero".to_string());
                }
                a / b
            }
            _ => return Err("Invalid operator".to_string()),
        };

        Ok(result)
    }

    /// Web search
    pub fn web_search(&self, query: &str) -> Vec<SearchResult> {
        vec![SearchResult {
            entry_type: "web".to_string(),
            title: format!("Search: {}", query),
            description: "Google Search".to_string(),
            icon: "web".to_string(),
            score: 1.0,
            action: format!("https://www.google.com/search?q={}", query),
        }]
    }

    /// Search commands
    pub fn search_commands(&self, query: &str) -> Vec<SearchResult> {
        vec![SearchResult {
            entry_type: "command".to_string(),
            title: query.to_string(),
            description: "Execute command".to_string(),
            icon: "terminal".to_string(),
            score: 1.0,
            action: query.to_string(),
        }]
    }

    /// Pin application
    pub fn pin_app(&mut self, app_name: &str) {
        if !self.pinned_apps.contains(&app_name.to_string()) {
            self.pinned_apps.push(app_name.to_string());
        }
    }

    /// Unpin application
    pub fn unpin_app(&mut self, app_name: &str) {
        self.pinned_apps.retain(|name| name != app_name);
    }

    /// Get pinned apps
    pub fn get_pinned_apps(&self) -> &[String] {
        &self.pinned_apps
    }

    /// Calculate relevance score
    fn calculate_score(&self, name: &str, description: &str, query: &str) -> f64 {
        let mut score = 0.0;

        let (exact_match, starts_with, contains, desc_contains) = if self.config.case_sensitive {
            (
                name == query,
                name.starts_with(query),
                name.contains(query),
                description.contains(query),
            )
        } else {
            (
                eq_ignore_case(name, query),
                starts_with_ignore_case(name, query),
                contains_ignore_case(name, query),
                contains_ignore_case(description, query),
            )
        };

        if exact_match {
            score += 100.0;
        }
        if starts_with {
            score += 50.0;
        }
        if contains {
            score += 25.0;
        }
        if desc_contains {
            score += 10.0;
        }

        // Bolt optimization: check pinned apps without String allocation
        if self.pinned_apps.iter().any(|p| p == name) {
            score += 20.0;
        }

        score
    }

    /// Acronym matching
    fn acronym_match(&self, text: &str, query: &str) -> bool {
        let mut words = text.split_whitespace();
        for q_char in query.chars() {
            if let Some(word) = words.next() {
                if let Some(first_char) = word.chars().next() {
                    if !first_char.eq_ignore_ascii_case(&q_char)
                        && first_char.to_lowercase().ne(q_char.to_lowercase())
                    {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    /// Get search history
    pub fn get_search_history(&self) -> &[String] {
        &self.search_history
    }

    /// Clear search history
    pub fn clear_search_history(&mut self) {
        self.search_history.clear();
    }

    /// Get statistics
    pub fn get_statistics(&self) -> (usize, usize, usize, usize) {
        (
            self.applications.len(),
            self.clipboard_history.len(),
            self.emojis.len(),
            self.search_history.len(),
        )
    }
}

// Bolt performance optimization: Zero-allocation case-insensitive matching helpers
fn contains_ignore_case(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    if haystack.len() < needle.len() {
        return false;
    }
    if haystack.is_ascii() && needle.is_ascii() {
        haystack
            .as_bytes()
            .windows(needle.len())
            .any(|w| w.eq_ignore_ascii_case(needle.as_bytes()))
    } else {
        haystack.to_lowercase().contains(&needle.to_lowercase())
    }
}

fn starts_with_ignore_case(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    if haystack.len() < needle.len() {
        return false;
    }
    if haystack.is_ascii() && needle.is_ascii() {
        haystack.as_bytes()[..needle.len()].eq_ignore_ascii_case(needle.as_bytes())
    } else {
        haystack.to_lowercase().starts_with(&needle.to_lowercase())
    }
}

fn eq_ignore_case(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    if a.is_ascii() && b.is_ascii() {
        a.as_bytes().eq_ignore_ascii_case(b.as_bytes())
    } else {
        a.to_lowercase() == b.to_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walker_creation() {
        let walker = WalkerLauncher::new();
        assert_eq!(walker.applications.len(), 0);
        assert_eq!(walker.clipboard_history.len(), 0);
    }

    #[test]
    fn test_register_application() {
        let mut walker = WalkerLauncher::new();
        let app = AppEntry {
            name: "Test App".to_string(),
            exec: "test-app".to_string(),
            icon: "test".to_string(),
            description: "Test application".to_string(),
            keywords: vec!["test".to_string()],
            categories: vec!["Utility".to_string()],
            terminal: false,
            no_display: false,
            pinned: false,
        };
        walker.register_application(app);
        assert_eq!(walker.applications.len(), 1);
    }

    #[test]
    fn test_search_applications() {
        let mut walker = WalkerLauncher::new();
        let app = AppEntry {
            name: "Test App".to_string(),
            exec: "test-app".to_string(),
            icon: "test".to_string(),
            description: "Test application".to_string(),
            keywords: vec!["test".to_string()],
            categories: vec!["Utility".to_string()],
            terminal: false,
            no_display: false,
            pinned: false,
        };
        walker.register_application(app);
        let results = walker.search_applications("test");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_clipboard_history() {
        let mut walker = WalkerLauncher::new();
        walker.add_clipboard_entry("test content".to_string(), false);
        assert_eq!(walker.clipboard_history.len(), 1);
        walker.add_clipboard_entry("test content".to_string(), false);
        assert_eq!(walker.clipboard_history.len(), 1); // Deduplicated
    }

    #[test]
    fn test_sensitive_clipboard() {
        let mut walker = WalkerLauncher::new();
        walker.add_clipboard_entry("password".to_string(), true);
        assert_eq!(walker.clipboard_history.len(), 0); // Not added
    }

    #[test]
    fn test_search_emojis() {
        let mut walker = WalkerLauncher::new();
        let emoji = EmojiEntry {
            emoji: "🔥".to_string(),
            name: "fire".to_string(),
            keywords: vec!["hot".to_string(), "burn".to_string()],
            category: "Nature".to_string(),
        };
        walker.register_emoji(emoji);
        let results = walker.search_emojis("fire");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_prefix_search() {
        let mut walker = WalkerLauncher::new();
        let emoji = EmojiEntry {
            emoji: "🔥".to_string(),
            name: "fire".to_string(),
            keywords: vec!["hot".to_string()],
            category: "Nature".to_string(),
        };
        walker.register_emoji(emoji);
        let results = walker.search(":fire");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_calculator() {
        let walker = WalkerLauncher::new();
        let results = walker.calculate("5 + 3");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "8");
    }

    #[test]
    fn test_web_search() {
        let walker = WalkerLauncher::new();
        let results = walker.web_search("test");
        assert_eq!(results.len(), 1);
        assert!(results[0].action.contains("google.com"));
    }

    #[test]
    fn test_pin_app() {
        let mut walker = WalkerLauncher::new();
        walker.pin_app("Test App");
        assert_eq!(walker.pinned_apps.len(), 1);
        walker.unpin_app("Test App");
        assert_eq!(walker.pinned_apps.len(), 0);
    }

    #[test]
    fn test_search_history() {
        let mut walker = WalkerLauncher::new();
        walker.search("test");
        assert_eq!(walker.search_history.len(), 1);
        walker.clear_search_history();
        assert_eq!(walker.search_history.len(), 0);
    }

    #[test]
    fn test_statistics() {
        let mut walker = WalkerLauncher::new();
        let app = AppEntry {
            name: "Test App".to_string(),
            exec: "test-app".to_string(),
            icon: "test".to_string(),
            description: "Test".to_string(),
            keywords: vec![],
            categories: vec![],
            terminal: false,
            no_display: false,
            pinned: false,
        };
        walker.register_application(app);
        walker.add_clipboard_entry("test".to_string(), false);
        walker.search("test");
        let (apps, clips, emojis, history) = walker.get_statistics();
        assert_eq!(apps, 1);
        assert_eq!(clips, 1);
        assert_eq!(emojis, 0);
        assert_eq!(history, 1);
    }
}
