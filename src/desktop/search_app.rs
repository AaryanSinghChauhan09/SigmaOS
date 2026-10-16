// SPDX-License-Identifier: MIT
// SigmaOS - Search Application
// Inspired by GNOME Search, Everything, and Recoll
// Provides fast file and content search

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Search type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchType {
    FileName,
    FileContent,
    Both,
}

/// File type filter
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileTypeFilter {
    All,
    Documents,
    Images,
    Audio,
    Video,
    Archives,
    Code,
}

/// Sort order
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Name,
    Size,
    Date,
    Relevance,
}

/// Search result
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub path: String,
    pub name: String,
    pub file_type: FileTypeFilter,
    pub size: u64,
    pub modified: String,
    pub relevance: u32,
    pub preview: String,
}

/// Search index
#[derive(Debug, Clone)]
pub struct SearchIndex {
    pub indexed_paths: Vec<String>,
    pub file_count: usize,
    pub last_updated: String,
}

/// Search application
pub struct SearchApp {
    pub query: String,
    pub search_type: SearchType,
    pub file_type_filter: FileTypeFilter,
    pub sort_order: SortOrder,
    pub results: Vec<SearchResult>,
    pub index: SearchIndex,
    pub searching: bool,
    pub case_sensitive: bool,
    pub regex_enabled: bool,
}

impl SearchApp {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            search_type: SearchType::Both,
            file_type_filter: FileTypeFilter::All,
            sort_order: SortOrder::Relevance,
            results: Vec::new(),
            index: SearchIndex {
                indexed_paths: Vec::new(),
                file_count: 0,
                last_updated: String::new(),
            },
            searching: false,
            case_sensitive: false,
            regex_enabled: false,
        }
    }

    /// Set query
    pub fn set_query(&mut self, query: String) {
        self.query = query;
    }

    /// Set search type
    pub fn set_search_type(&mut self, search_type: SearchType) {
        self.search_type = search_type;
    }

    /// Set file type filter
    pub fn set_file_type_filter(&mut self, filter: FileTypeFilter) {
        self.file_type_filter = filter;
    }

    /// Set sort order
    pub fn set_sort_order(&mut self, order: SortOrder) {
        self.sort_order = order;
    }

    /// Toggle case sensitive
    pub fn toggle_case_sensitive(&mut self) {
        self.case_sensitive = !self.case_sensitive;
    }

    /// Toggle regex
    pub fn toggle_regex(&mut self) {
        self.regex_enabled = !self.regex_enabled;
    }

    /// Start search
    pub fn start_search(&mut self) -> Result<(), &'static str> {
        if self.query.is_empty() {
            return Err("Query is empty");
        }
        self.searching = true;
        self.results.clear();
        Ok(())
    }

    /// Stop search
    pub fn stop_search(&mut self) {
        self.searching = false;
    }

    /// Get results
    pub fn get_results(&self) -> &[SearchResult] {
        &self.results
    }

    /// Add result
    pub fn add_result(&mut self, result: SearchResult) {
        self.results.push(result);
    }

    /// Clear results
    pub fn clear_results(&mut self) {
        self.results.clear();
    }

    /// Build index
    pub fn build_index(&mut self, paths: Vec<String>) {
        self.index.indexed_paths = paths;
        self.index.file_count = self.index.indexed_paths.len();
        self.index.last_updated = "Now".to_string();
    }

    /// Update index
    pub fn update_index(&mut self) {
        self.index.last_updated = "Now".to_string();
    }

    /// Get index
    pub fn get_index(&self) -> &SearchIndex {
        &self.index
    }

    /// Is searching
    pub fn is_searching(&self) -> bool {
        self.searching
    }

    /// Result count
    pub fn result_count(&self) -> usize {
        self.results.len()
    }
}

/// Global search app
static mut SEARCH_APP: Option<SearchApp> = None;

/// Initialize search app
pub fn init_search_app() {
    unsafe {
        SEARCH_APP = Some(SearchApp::new());
    }
}

/// Get search app
pub fn get_search_app() -> &'static mut SearchApp {
    unsafe { SEARCH_APP.as_mut().expect("Search app not initialized") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_app_creation() {
        let sa = SearchApp::new();
        assert_eq!(sa.result_count(), 0);
    }

    #[test]
    fn test_set_query() {
        let mut sa = SearchApp::new();
        sa.set_query("test".to_string());
        assert_eq!(sa.query, "test");
    }

    #[test]
    fn test_build_index() {
        let mut sa = SearchApp::new();
        sa.build_index(vec!["/home".to_string(), "/usr".to_string()]);
        assert_eq!(sa.index.file_count, 2);
    }
}
