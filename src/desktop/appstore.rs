extern crate alloc;
use alloc::vec::Vec;
use alloc::string::{String, ToString};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppReview {
    pub author: String,
    pub rating: u32, // 1 to 5 stars
    pub comment: String,
    pub helpful_votes: u32,
}

impl AppReview {
    pub fn new(author: &str, rating: u32, comment: &str) -> Self {
        Self {
            author: author.to_string(),
            rating: rating.clamp(1, 5),
            comment: comment.to_string(),
            helpful_votes: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppStoreItem {
    pub name: String,
    pub version: String,
    pub developer: String,
    pub description: String,
    pub category: String,
    pub size_bytes: u64,
    pub installed: bool,
    pub reviews: Vec<AppReview>,
}

impl AppStoreItem {
    pub fn new(name: &str, version: &str, developer: &str, description: &str, category: &str, size: u64) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            developer: developer.to_string(),
            description: description.to_string(),
            category: category.to_string(),
            size_bytes: size,
            installed: false,
            reviews: Vec::new(),
        }
    }

    pub fn get_average_rating(&self) -> f32 {
        if self.reviews.is_empty() {
            return 0.0;
        }
        let total_rating: u32 = self.reviews.iter().map(|r| r.rating).sum();
        total_rating as f32 / self.reviews.len() as f32
    }
}

pub struct GuiAppStore {
    pub items: Vec<AppStoreItem>,
}

impl GuiAppStore {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn register_app(&mut self, item: AppStoreItem) -> Result<(), &'static str> {
        if self.items.iter().any(|i| i.name == item.name) {
            return Err("Application is already registered in the store catalog");
        }
        self.items.push(item);
        Ok(())
    }

    pub fn install_app(&mut self, name: &str) -> Result<(), &'static str> {
        if let Some(item) = self.items.iter_mut().find(|i| i.name == name) {
            if item.installed {
                return Err("Application is already installed");
            }
            item.installed = true;
            Ok(())
        } else {
            Err("Application not found in store catalog")
        }
    }

    pub fn uninstall_app(&mut self, name: &str) -> Result<(), &'static str> {
        if let Some(item) = self.items.iter_mut().find(|i| i.name == name) {
            if !item.installed {
                return Err("Application is not installed");
            }
            item.installed = false;
            Ok(())
        } else {
            Err("Application not found in store catalog")
        }
    }

    pub fn add_review(&mut self, app_name: &str, review: AppReview) -> Result<(), &'static str> {
        if let Some(item) = self.items.iter_mut().find(|i| i.name == app_name) {
            item.reviews.push(review);
            Ok(())
        } else {
            Err("Application not found in store catalog")
        }
    }

    pub fn search_apps(&self, query: &str) -> Vec<&AppStoreItem> {
        let mut results = Vec::new();
        let query_lower = query.to_string(); // simple mock search
        for item in &self.items {
            if item.name.contains(&query_lower) || item.description.contains(&query_lower) {
                results.push(item);
            }
        }
        results
    }

    pub fn filter_by_category(&self, category: &str) -> Vec<&AppStoreItem> {
        let mut results = Vec::new();
        for item in &self.items {
            if item.category == category {
                results.push(item);
            }
        }
        results
    }
}

impl Default for GuiAppStore {
    fn default() -> Self {
        Self::new()
    }
}
