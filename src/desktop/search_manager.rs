//! Search Manager
//!
//! Search management inspired by Linux Mint's search and Omarchy's
//! search utilities, supporting file, application, and system-wide search.

use std::collections::HashMap;

/// Search result type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchResultType {
    File,
    Application,
    Setting,
    Command,
    Web,
}

impl SearchResultType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "file" => Some(SearchResultType::File),
            "application" => Some(SearchResultType::Application),
            "setting" => Some(SearchResultType::Setting),
            "command" => Some(SearchResultType::Command),
            "web" => Some(SearchResultType::Web),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SearchResultType::File => "File",
            SearchResultType::Application => "Application",
            SearchResultType::Setting => "Setting",
            SearchResultType::Command => "Command",
            SearchResultType::Web => "Web",
        }
    }
}

/// Search result
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub id: String,
    pub result_type: SearchResultType,
    pub title: String,
    pub description: String,
    pub path: String,
    pub icon: Option<String>,
    pub relevance: f32,
}

impl SearchResult {
    pub fn new(
        id: String,
        result_type: SearchResultType,
        title: String,
        description: String,
        path: String,
    ) -> Self {
        Self {
            id,
            result_type,
            title,
            description,
            path,
            icon: None,
            relevance: 1.0,
        }
    }

    pub fn set_icon(&mut self, icon: String) {
        self.icon = Some(icon);
    }

    pub fn set_relevance(&mut self, relevance: f32) {
        self.relevance = relevance;
    }
}

/// Search query
#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub query: String,
    pub search_types: Vec<SearchResultType>,
    pub max_results: usize,
}

impl SearchQuery {
    pub fn new(query: String) -> Self {
        Self {
            query,
            search_types: vec![
                SearchResultType::File,
                SearchResultType::Application,
                SearchResultType::Setting,
                SearchResultType::Command,
            ],
            max_results: 20,
        }
    }

    pub fn with_types(mut self, types: Vec<SearchResultType>) -> Self {
        self.search_types = types;
        self
    }

    pub fn with_max_results(mut self, max: usize) -> Self {
        self.max_results = max;
        self
    }
}

/// Search manager
#[derive(Debug)]
pub struct SearchManager {
    indexed_files: HashMap<String, SearchResult>,
    indexed_apps: HashMap<String, SearchResult>,
    indexed_settings: HashMap<String, SearchResult>,
    indexed_commands: HashMap<String, SearchResult>,
}

impl SearchManager {
    pub fn new() -> Self {
        let mut manager = Self {
            indexed_files: HashMap::new(),
            indexed_apps: HashMap::new(),
            indexed_settings: HashMap::new(),
            indexed_commands: HashMap::new(),
        };

        // Index default applications
        manager.index_default_applications();

        // Index default commands
        manager.index_default_commands();

        // Index default settings
        manager.index_default_settings();

        manager
    }

    /// Index a file
    pub fn index_file(&mut self, result: SearchResult) {
        self.indexed_files.insert(result.id.clone(), result);
    }

    /// Index an application
    pub fn index_application(&mut self, result: SearchResult) {
        self.indexed_apps.insert(result.id.clone(), result);
    }

    /// Index a setting
    pub fn index_setting(&mut self, result: SearchResult) {
        self.indexed_settings.insert(result.id.clone(), result);
    }

    /// Index a command
    pub fn index_command(&mut self, result: SearchResult) {
        self.indexed_commands.insert(result.id.clone(), result);
    }

    /// Search
    pub fn search(&self, query: SearchQuery) -> Vec<SearchResult> {
        let mut results = Vec::new();

        for search_type in &query.search_types {
            match search_type {
                SearchResultType::File => {
                    results.extend(self.search_files(&query.query));
                }
                SearchResultType::Application => {
                    results.extend(self.search_applications(&query.query));
                }
                SearchResultType::Setting => {
                    results.extend(self.search_settings(&query.query));
                }
                SearchResultType::Command => {
                    results.extend(self.search_commands(&query.query));
                }
                SearchResultType::Web => {
                    // Web search would be implemented externally
                }
            }
        }

        // Sort by relevance
        results.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap());

        // Limit results
        results.truncate(query.max_results);

        results
    }

    /// Search files
    fn search_files(&self, query: &str) -> Vec<SearchResult> {
        // Bolt Optimization: Zero-allocation case-insensitive substring matching.
        // Avoids allocating heap String instances via `to_lowercase()` for title, description, and path on every searched file entry.
        self.indexed_files
            .values()
            .filter(|r| {
                contains_ignore_case(&r.title, query)
                    || contains_ignore_case(&r.description, query)
                    || contains_ignore_case(&r.path, query)
            })
            .cloned()
            .collect()
    }

    /// Search applications
    fn search_applications(&self, query: &str) -> Vec<SearchResult> {
        // Bolt Optimization: Zero-allocation case-insensitive substring matching.
        // Avoids allocating heap String instances via `to_lowercase()` for title and description on every searched app entry.
        self.indexed_apps
            .values()
            .filter(|r| {
                contains_ignore_case(&r.title, query) || contains_ignore_case(&r.description, query)
            })
            .cloned()
            .collect()
    }

    /// Search settings
    fn search_settings(&self, query: &str) -> Vec<SearchResult> {
        // Bolt Optimization: Zero-allocation case-insensitive substring matching.
        // Avoids allocating heap String instances via `to_lowercase()` for title and description on every searched setting entry.
        self.indexed_settings
            .values()
            .filter(|r| {
                contains_ignore_case(&r.title, query) || contains_ignore_case(&r.description, query)
            })
            .cloned()
            .collect()
    }

    /// Search commands
    fn search_commands(&self, query: &str) -> Vec<SearchResult> {
        // Bolt Optimization: Zero-allocation case-insensitive substring matching.
        // Avoids allocating heap String instances via `to_lowercase()` for title and description on every searched command entry.
        self.indexed_commands
            .values()
            .filter(|r| {
                contains_ignore_case(&r.title, query) || contains_ignore_case(&r.description, query)
            })
            .cloned()
            .collect()
    }

    /// Index default applications
    fn index_default_applications(&mut self) {
        let apps = vec![
            (
                "terminal",
                "Terminal",
                "Command line terminal",
                "/usr/bin/terminal",
            ),
            (
                "file-manager",
                "File Manager",
                "Browse files",
                "/usr/bin/file-manager",
            ),
            (
                "web-browser",
                "Web Browser",
                "Browse the web",
                "/usr/bin/web-browser",
            ),
            (
                "settings",
                "Settings",
                "System settings",
                "/usr/bin/settings",
            ),
            (
                "calculator",
                "Calculator",
                "Perform calculations",
                "/usr/bin/calculator",
            ),
        ];

        for (id, title, description, path) in apps {
            let result = SearchResult::new(
                id.to_string(),
                SearchResultType::Application,
                title.to_string(),
                description.to_string(),
                path.to_string(),
            );
            self.index_application(result);
        }
    }

    /// Index default commands
    fn index_default_commands(&mut self) {
        let commands = vec![
            ("ls", "List directory contents", "ls [directory]"),
            ("cd", "Change directory", "cd <directory>"),
            ("cp", "Copy files", "cp <source> <destination>"),
            ("mv", "Move files", "mv <source> <destination>"),
            ("rm", "Remove files", "rm <file>"),
            ("grep", "Search text", "grep <pattern> <file>"),
        ];

        for (id, title, description) in commands {
            let result = SearchResult::new(
                id.to_string(),
                SearchResultType::Command,
                title.to_string(),
                description.to_string(),
                format!("/usr/bin/{}", id),
            );
            self.index_command(result);
        }
    }

    /// Index default settings
    fn index_default_settings(&mut self) {
        let settings = vec![
            ("theme", "Theme", "Desktop theme", "settings://theme"),
            (
                "display",
                "Display",
                "Display settings",
                "settings://display",
            ),
            ("sound", "Sound", "Sound settings", "settings://sound"),
            (
                "network",
                "Network",
                "Network settings",
                "settings://network",
            ),
            ("power", "Power", "Power settings", "settings://power"),
        ];

        for (id, title, description, path) in settings {
            let result = SearchResult::new(
                id.to_string(),
                SearchResultType::Setting,
                title.to_string(),
                description.to_string(),
                path.to_string(),
            );
            self.index_setting(result);
        }
    }

    /// Get statistics
    pub fn get_statistics(&self) -> SearchStatistics {
        SearchStatistics {
            indexed_files: self.indexed_files.len(),
            indexed_apps: self.indexed_apps.len(),
            indexed_settings: self.indexed_settings.len(),
            indexed_commands: self.indexed_commands.len(),
        }
    }
}

impl Default for SearchManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Search statistics
#[derive(Debug, Clone)]
pub struct SearchStatistics {
    pub indexed_files: usize,
    pub indexed_apps: usize,
    pub indexed_settings: usize,
    pub indexed_commands: usize,
}

/// Zero-allocation case-insensitive substring search helper.
/// Checks whether `haystack` contains `needle`, ignoring ASCII case, without allocating heap Strings.
fn contains_ignore_case(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    if haystack.len() < needle.len() {
        return false;
    }
    if haystack.is_ascii() && needle.is_ascii() {
        let needle_bytes = needle.as_bytes();
        haystack
            .as_bytes()
            .windows(needle_bytes.len())
            .any(|window| {
                window
                    .iter()
                    .zip(needle_bytes.iter())
                    .all(|(&b1, &b2)| b1.to_ascii_lowercase() == b2.to_ascii_lowercase())
            })
    } else {
        haystack.to_lowercase().contains(&needle.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_manager_creation() {
        let manager = SearchManager::new();
        let stats = manager.get_statistics();
        assert!(stats.indexed_apps > 0);
        assert!(stats.indexed_commands > 0);
    }

    #[test]
    fn test_search_applications() {
        let manager = SearchManager::new();
        let query = SearchQuery::new("terminal".to_string());
        let results = manager.search(query);
        assert!(results.len() > 0);
    }

    #[test]
    fn test_search_commands() {
        let manager = SearchManager::new();
        let query = SearchQuery::new("ls".to_string());
        let results = manager.search(query);
        assert!(results.len() > 0);
    }

    #[test]
    fn test_search_settings() {
        let manager = SearchManager::new();
        let query = SearchQuery::new("theme".to_string());
        let results = manager.search(query);
        assert!(results.len() > 0);
    }

    #[test]
    fn test_search_with_types() {
        let manager = SearchManager::new();
        let query = SearchQuery::new("terminal".to_string())
            .with_types(vec![SearchResultType::Application]);
        let results = manager.search(query);
        assert!(results.len() > 0);
    }

    #[test]
    fn test_search_max_results() {
        let manager = SearchManager::new();
        let query = SearchQuery::new("t".to_string()).with_max_results(5);
        let results = manager.search(query);
        assert!(results.len() <= 5);
    }

    #[test]
    fn test_index_file() {
        let mut manager = SearchManager::new();
        let result = SearchResult::new(
            "file1".to_string(),
            SearchResultType::File,
            "Document.txt".to_string(),
            "Text document".to_string(),
            "/home/user/Document.txt".to_string(),
        );
        manager.index_file(result);
        assert_eq!(manager.get_statistics().indexed_files, 1);
    }
}
