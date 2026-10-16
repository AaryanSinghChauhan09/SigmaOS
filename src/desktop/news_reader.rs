// SPDX-License-Identifier: MIT
// SigmaOS - News Reader
// Inspired by GNOME News and Liferea
// Provides RSS/Atom feed reading

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Feed type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeedType {
    RSS,
    Atom,
}

/// Feed item
#[derive(Debug, Clone)]
pub struct FeedItem {
    pub id: String,
    pub title: String,
    pub link: String,
    pub description: String,
    pub content: String,
    pub author: String,
    pub published: String,
    pub read: bool,
    pub starred: bool,
    pub feed_id: String,
}

/// Feed
#[derive(Debug, Clone)]
pub struct Feed {
    pub id: String,
    pub name: String,
    pub url: String,
    pub feed_type: FeedType,
    pub description: String,
    pub icon: Option<String>,
    pub update_interval: u32,
    pub last_updated: String,
    pub items: Vec<FeedItem>,
}

/// News reader
pub struct NewsReader {
    pub feeds: Vec<Feed>,
    pub current_feed: Option<usize>,
    pub selected_item: Option<String>,
    pub filter: String,
    pub show_read: bool,
}

impl NewsReader {
    pub fn new() -> Self {
        Self {
            feeds: Vec::new(),
            current_feed: None,
            selected_item: None,
            filter: String::new(),
            show_read: true,
        }
    }

    /// Add feed
    pub fn add_feed(&mut self, feed: Feed) {
        self.feeds.push(feed);
        if self.current_feed.is_none() {
            self.current_feed = Some(0);
        }
    }

    /// Remove feed
    pub fn remove_feed(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.feeds.len() {
            return Err("Feed index out of bounds");
        }
        self.feeds.remove(index);
        if self.current_feed == Some(index) {
            self.current_feed = self.feeds.len().checked_sub(1);
        }
        Ok(())
    }

    /// Set current feed
    pub fn set_current_feed(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.feeds.len() {
            return Err("Feed index out of bounds");
        }
        self.current_feed = Some(index);
        Ok(())
    }

    /// Select item
    pub fn select_item(&mut self, item_id: String) {
        self.selected_item = Some(item_id);
    }

    /// Mark as read
    pub fn mark_as_read(&mut self, item_id: String) -> Result<(), &'static str> {
        for feed in &mut self.feeds {
            if let Some(item) = feed.items.iter_mut().find(|i| i.id == item_id) {
                item.read = true;
                return Ok(());
            }
        }
        Err("Item not found")
    }

    /// Mark all as read
    pub fn mark_all_as_read(&mut self, feed_index: usize) -> Result<(), &'static str> {
        let feed = self.feeds.get_mut(feed_index).ok_or("Feed not found")?;
        for item in &mut feed.items {
            item.read = true;
        }
        Ok(())
    }

    /// Toggle star
    pub fn toggle_star(&mut self, item_id: String) -> Result<(), &'static str> {
        for feed in &mut self.feeds {
            if let Some(item) = feed.items.iter_mut().find(|i| i.id == item_id) {
                item.starred = !item.starred;
                return Ok(());
            }
        }
        Err("Item not found")
    }

    /// Set filter
    pub fn set_filter(&mut self, filter: String) {
        self.filter = filter;
    }

    /// Toggle show read
    pub fn toggle_show_read(&mut self) {
        self.show_read = !self.show_read;
    }

    /// Get unread count
    pub fn get_unread_count(&self) -> usize {
        self.feeds
            .iter()
            .flat_map(|f| f.items.iter().filter(|i| !i.read))
            .count()
    }

    /// Get starred items
    pub fn get_starred_items(&self) -> Vec<&FeedItem> {
        self.feeds
            .iter()
            .flat_map(|f| f.items.iter().filter(|i| i.starred))
            .collect()
    }

    /// Get feeds
    pub fn get_feeds(&self) -> &[Feed] {
        &self.feeds
    }

    /// Feed count
    pub fn feed_count(&self) -> usize {
        self.feeds.len()
    }
}

/// Global news reader
static mut NEWS_READER: Option<NewsReader> = None;

/// Initialize news reader
pub fn init_news_reader() {
    unsafe {
        NEWS_READER = Some(NewsReader::new());
    }
}

/// Get news reader
pub fn get_news_reader() -> &'static mut NewsReader {
    unsafe { NEWS_READER.as_mut().expect("News reader not initialized") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_news_reader_creation() {
        let nr = NewsReader::new();
        assert_eq!(nr.feed_count(), 0);
    }

    #[test]
    fn test_add_feed() {
        let mut nr = NewsReader::new();
        let feed = Feed {
            id: "1".to_string(),
            name: "Tech News".to_string(),
            url: "https://example.com/feed".to_string(),
            feed_type: FeedType::RSS,
            description: String::new(),
            icon: None,
            update_interval: 3600,
            last_updated: String::new(),
            items: Vec::new(),
        };
        nr.add_feed(feed);
        assert_eq!(nr.feed_count(), 1);
    }
}
