#![no_std]

/// AI-Based Search Assistant for SigmaOS
/// Based on 100-Improvement-Ideas.md #54: AI-based search assistant
/// Implements intelligent search with context awareness and ranking

use core::sync::atomic::{AtomicU64, Ordering};

/// Search result type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchResultType {
    File = 0,
    Application = 1,
    Setting = 2,
    Web = 3,
    Command = 4,
}

/// Search result
#[repr(C)]
pub struct SearchResult {
    pub id: u64,
    pub result_type: SearchResultType,
    pub title: [u8; 128],
    pub description: [u8; 256],
    pub path: [u8; 256],
    pub relevance_score: f32,
}

impl SearchResult {
    pub fn new(id: u64, result_type: SearchResultType, title: &str, path: &str) -> Self {
        let mut title_array = [0u8; 128];
        let title_bytes = title.as_bytes();
        let title_len = title_bytes.len().min(127);
        
        unsafe {
            core::ptr::copy_nonoverlapping(title_bytes.as_ptr(), title_array.as_mut_ptr(), title_len);
        }
        
        let mut path_array = [0u8; 256];
        let path_bytes = path.as_bytes();
        let path_len = path_bytes.len().min(255);
        
        unsafe {
            core::ptr::copy_nonoverlapping(path_bytes.as_ptr(), path_array.as_mut_ptr(), path_len);
        }
        
        SearchResult {
            id,
            result_type,
            title: title_array,
            description: [0u8; 256],
            path: path_array,
            relevance_score: 0.0,
        }
    }
    
    pub fn set_description(&mut self, description: &str) {
        let desc_bytes = description.as_bytes();
        let desc_len = desc_bytes.len().min(255);
        
        unsafe {
            core::ptr::copy_nonoverlapping(desc_bytes.as_ptr(), self.description.as_mut_ptr(), desc_len);
        }
    }
}

/// Search query
#[repr(C)]
pub struct SearchQuery {
    pub id: u64,
    pub query_text: [u8; 256],
    pub timestamp: u64,
    pub context: [u8; 128],
}

impl SearchQuery {
    pub fn new(id: u64, query_text: &str, context: &str) -> Self {
        let mut query_array = [0u8; 256];
        let query_bytes = query_text.as_bytes();
        let query_len = query_bytes.len().min(255);
        
        unsafe {
            core::ptr::copy_nonoverlapping(query_bytes.as_ptr(), query_array.as_mut_ptr(), query_len);
        }
        
        let mut context_array = [0u8; 128];
        let context_bytes = context.as_bytes();
        let context_len = context_bytes.len().min(127);
        
        unsafe {
            core::ptr::copy_nonoverlapping(context_bytes.as_ptr(), context_array.as_mut_ptr(), context_len);
        }
        
        SearchQuery {
            id,
            query_text: query_array,
            timestamp: get_current_time(),
            context: context_array,
        }
    }
}

/// AI search assistant
pub struct AISearchAssistant {
    pub search_results: Vec<Option<SearchResult>>,
    pub search_history: Vec<Option<SearchQuery>>,
    pub next_result_id: AtomicU64,
    pub next_query_id: AtomicU64,
    pub ai_ranking_enabled: bool,
}

impl AISearchAssistant {
    pub fn new() -> Self {
        AISearchAssistant {
            search_results: Vec::new(),
            search_history: Vec::new(),
            next_result_id: AtomicU64::new(1),
            next_query_id: AtomicU64::new(1),
            ai_ranking_enabled: true,
        }
    }
    
    /// Search
    pub fn search(&mut self, query: &str, context: &str) -> Vec<&SearchResult> {
        let query_id = self.next_query_id.fetch_add(1, Ordering::SeqCst);
        let search_query = SearchQuery::new(query_id, query, context);
        self.search_history.push(Some(search_query));
        
        let mut results = Vec::new();
        
        // Simple search implementation
        // In real implementation, use full-text search and AI ranking
        for result_option in &self.search_results {
            if let Some(ref result) = *result_option {
                let title_str = unsafe {
                    let len = result.title.iter().position(|&b| b == 0).unwrap_or(128);
                    core::str::from_utf8_unchecked(&result.title[..len])
                };
                
                if title_str.contains(query) || title_str.to_lowercase().contains(&query.to_lowercase()) {
                    results.push(result);
                }
            }
        }
        
        // Apply AI ranking if enabled
        if self.ai_ranking_enabled {
            self.rank_results(query, &mut results);
        }
        
        results
    }
    
    /// Add search result
    pub fn add_result(&mut self, result_type: SearchResultType, title: &str, path: &str) -> u64 {
        let id = self.next_result_id.fetch_add(1, Ordering::SeqCst);
        let result = SearchResult::new(id, result_type, title, path);
        self.search_results.push(Some(result));
        id
    }
    
    /// Rank results by relevance
    fn rank_results(&self, query: &str, results: &mut Vec<&SearchResult>) {
        // Simple relevance scoring
        for result in results.iter_mut() {
            let title_str = unsafe {
                let len = result.title.iter().position(|&b| b == 0).unwrap_or(128);
                core::str::from_utf8_unchecked(&result.title[..len])
            };
            
            // Exact match gets higher score
            if title_str == query {
                result.relevance_score = 1.0;
            } else if title_str.contains(query) {
                result.relevance_score = 0.8;
            } else {
                result.relevance_score = 0.5;
            }
        }
        
        // Sort by relevance (descending)
        for i in 0..results.len() {
            for j in i+1..results.len() {
                if results[j].relevance_score > results[i].relevance_score {
                    let temp = results[i];
                    results[i] = results[j];
                    results[j] = temp;
                }
            }
        }
    }
    
    /// Get search suggestions
    pub fn get_suggestions(&self, partial_query: &str) -> Vec<&str> {
        let mut suggestions = Vec::new();
        
        for query_option in &self.search_history {
            if let Some(ref query) = *query_option {
                let query_str = unsafe {
                    let len = query.query_text.iter().position(|&b| b == 0).unwrap_or(256);
                    core::str::from_utf8_unchecked(&query.query_text[..len])
                };
                
                if query_str.starts_with(partial_query) {
                    suggestions.push(query_str);
                }
            }
        }
        
        suggestions
    }
    
    /// Clear search history
    pub fn clear_history(&mut self) {
        self.search_history = Vec::new();
    }
    
    /// Enable/disable AI ranking
    pub fn set_ai_ranking_enabled(&mut self, enabled: bool) {
        self.ai_ranking_enabled = enabled;
    }
}

/// Simple Vec implementation for no_std
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * core::mem::size_of::<T>()) as *mut T;

        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }

            if self.capacity > 0 {
                free(self.data as *mut u8);
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

/// Get current time (nanoseconds)
fn get_current_time() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    COUNTER.fetch_add(1_000_000, Ordering::SeqCst)
}
