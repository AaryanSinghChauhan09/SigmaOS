//! SigmaSearch - System Search and Indexing System
//!
//! Provides comprehensive system search and indexing inspired by Linux search tools.
//! Supports file search, content search, and index management.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Search result
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub path: String,
    pub name: String,
    pub content_preview: String,
    pub relevance_score: f32,
}

impl SearchResult {
    pub fn new(path: String, name: String) -> Self {
        Self {
            path,
            name,
            content_preview: String::new(),
            relevance_score: 0.0,
        }
    }

    pub fn with_content(mut self, content_preview: String) -> Self {
        self.content_preview = content_preview;
        self
    }

    pub fn with_relevance(mut self, relevance_score: f32) -> Self {
        self.relevance_score = relevance_score;
        self
    }
}

/// Search index
#[derive(Debug, Clone)]
pub struct SearchIndex {
    pub id: String,
    pub name: String,
    pub path: String,
    pub indexed_files: u32,
    pub last_updated: u64,
}

impl SearchIndex {
    pub fn new(id: String, name: String, path: String) -> Self {
        Self {
            id,
            name,
            path,
            indexed_files: 0,
            last_updated: 0,
        }
    }

    pub fn with_indexed_files(mut self, indexed_files: u32) -> Self {
        self.indexed_files = indexed_files;
        self
    }

    pub fn set_indexed_files(&mut self, indexed_files: u32) {
        self.indexed_files = indexed_files;
    }

    pub fn with_last_updated(mut self, last_updated: u64) -> Self {
        self.last_updated = last_updated;
        self
    }

    pub fn set_last_updated(&mut self, last_updated: u64) {
        self.last_updated = last_updated;
    }
}

/// Search manager
pub struct SigmaSearch {
    indexes: BTreeMap<String, SearchIndex>,
    results: Vec<SearchResult>,
    next_index_id: u64,
}

impl SigmaSearch {
    pub fn new() -> Self {
        Self {
            indexes: BTreeMap::new(),
            results: Vec::new(),
            next_index_id: 1,
        }
    }

    pub fn add_index(&mut self, name: String, path: String) -> String {
        let id = format!("index-{}", self.next_index_id);
        self.next_index_id += 1;
        let index = SearchIndex::new(id.clone(), name, path);
        self.indexes.insert(id.clone(), index);
        id
    }

    pub fn get_index(&self, id: &str) -> Option<&SearchIndex> {
        self.indexes.get(id)
    }

    pub fn get_index_mut(&mut self, id: &str) -> Option<&mut SearchIndex> {
        self.indexes.get_mut(id)
    }

    pub fn list_indexes(&self) -> Vec<&SearchIndex> {
        self.indexes.values().collect()
    }

    pub fn remove_index(&mut self, id: &str) -> bool {
        self.indexes.remove(id).is_some()
    }

    pub fn add_result(&mut self, result: SearchResult) {
        self.results.push(result);
    }

    pub fn get_results(&self) -> Vec<&SearchResult> {
        self.results.iter().collect()
    }

    pub fn clear_results(&mut self) {
        self.results.clear();
    }

    pub fn index_count(&self) -> usize {
        self.indexes.len()
    }

    pub fn result_count(&self) -> usize {
        self.results.len()
    }

    pub fn total_indexed_files(&self) -> u32 {
        self.indexes.values().map(|i| i.indexed_files).sum()
    }
}

impl Default for SigmaSearch {
    fn default() -> Self {
        Self::new()
    }
}

static mut SIGMASEARCH_INSTANCE: Option<SigmaSearch> = None;

pub fn init_sigmasearch() {
    unsafe {
        SIGMASEARCH_INSTANCE = Some(SigmaSearch::new());
    }
}

pub fn get_sigmasearch() -> &'static SigmaSearch {
    unsafe {
        SIGMASEARCH_INSTANCE
            .as_ref()
            .expect("SigmaSearch not initialized")
    }
}

pub fn get_sigmasearch_mut() -> &'static mut SigmaSearch {
    unsafe {
        SIGMASEARCH_INSTANCE
            .as_mut()
            .expect("SigmaSearch not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_result_creation() {
        let result = SearchResult::new("/home/user/file.txt".to_string(), "file.txt".to_string());
        assert_eq!(result.path, "/home/user/file.txt");
        assert_eq!(result.name, "file.txt");
    }

    #[test]
    fn test_search_result_builder() {
        let result = SearchResult::new("/home/user/file.txt".to_string(), "file.txt".to_string())
            .with_content("Hello, World!".to_string())
            .with_relevance(0.95);
        assert_eq!(result.content_preview, "Hello, World!");
        assert_eq!(result.relevance_score, 0.95);
    }

    #[test]
    fn test_search_index_creation() {
        let index = SearchIndex::new(
            "index-1".to_string(),
            "Home".to_string(),
            "/home".to_string(),
        );
        assert_eq!(index.id, "index-1");
        assert_eq!(index.name, "Home");
        assert_eq!(index.path, "/home");
    }

    #[test]
    fn test_search_index_builder() {
        let index = SearchIndex::new(
            "index-1".to_string(),
            "Home".to_string(),
            "/home".to_string(),
        )
        .with_indexed_files(1000)
        .with_last_updated(1234567890);
        assert_eq!(index.indexed_files, 1000);
        assert_eq!(index.last_updated, 1234567890);
    }

    #[test]
    fn test_search_manager() {
        let manager = SigmaSearch::new();
        assert_eq!(manager.index_count(), 0);
        assert_eq!(manager.result_count(), 0);
    }

    #[test]
    fn test_add_index() {
        let mut manager = SigmaSearch::new();
        let id = manager.add_index("Home".to_string(), "/home".to_string());
        assert_eq!(id, "index-1");
        assert_eq!(manager.index_count(), 1);
    }

    #[test]
    fn test_add_result() {
        let mut manager = SigmaSearch::new();
        let result = SearchResult::new("/home/user/file.txt".to_string(), "file.txt".to_string());
        manager.add_result(result);
        assert_eq!(manager.result_count(), 1);
    }

    #[test]
    fn test_clear_results() {
        let mut manager = SigmaSearch::new();
        manager.add_result(SearchResult::new(
            "/home/user/file1.txt".to_string(),
            "file1.txt".to_string(),
        ));
        manager.add_result(SearchResult::new(
            "/home/user/file2.txt".to_string(),
            "file2.txt".to_string(),
        ));

        manager.clear_results();
        assert_eq!(manager.result_count(), 0);
    }

    #[test]
    fn test_total_indexed_files() {
        let mut manager = SigmaSearch::new();
        let id1 = manager.add_index("Home".to_string(), "/home".to_string());
        let id2 = manager.add_index("Documents".to_string(), "/home/user/Documents".to_string());

        manager.get_index_mut(&id1).unwrap().set_indexed_files(1000);
        manager.get_index_mut(&id2).unwrap().set_indexed_files(500);

        assert_eq!(manager.total_indexed_files(), 1500);
    }

    #[test]
    fn test_global_instance() {
        init_sigmasearch();
        let manager = get_sigmasearch();
        assert_eq!(manager.index_count(), 0);

        let manager_mut = get_sigmasearch_mut();
        manager_mut.add_index("Home".to_string(), "/home".to_string());
        assert_eq!(manager.index_count(), 1);
    }
}
