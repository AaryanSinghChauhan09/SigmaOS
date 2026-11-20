//! SigmaOS Notification Center Plugin
//!
//! Panel applet plugin that aggregates and displays a notification history
//! drawer, mirrors the GNOME notification center / mako history model.
//! Upstream: GNOME Shell notification list, mako history mode

use std::collections::VecDeque;

/// Notification entry stored in the center.
#[derive(Debug, Clone)]
pub struct NotificationEntry {
    pub id:      u32,
    pub app:     String,
    pub summary: String,
    pub body:    String,
    pub read:    bool,
}

impl NotificationEntry {
    pub fn new(id: u32, app: &str, summary: &str, body: &str) -> Self {
        Self { id, app: app.to_owned(), summary: summary.to_owned(),
               body: body.to_owned(), read: false }
    }
}

/// The notification center plugin state.
pub struct NotificationCenter {
    entries:  VecDeque<NotificationEntry>,
    max_size: usize,
    open:     bool,
}

impl NotificationCenter {
    pub fn new(max_size: usize) -> Self {
        Self { entries: VecDeque::new(), max_size, open: false }
    }

    /// Add a notification to the history. Returns false if capacity exceeded.
    pub fn add(&mut self, entry: NotificationEntry) -> bool {
        if self.entries.len() >= self.max_size {
            self.entries.pop_back();  // evict oldest
        }
        self.entries.push_front(entry);
        true
    }

    /// Mark all notifications as read.
    pub fn mark_all_read(&mut self) {
        for e in &mut self.entries { e.read = true; }
    }

    /// Mark a single notification as read by ID.
    pub fn mark_read(&mut self, id: u32) -> bool {
        self.entries.iter_mut()
            .find(|e| e.id == id)
            .map(|e| { e.read = true; true })
            .unwrap_or(false)
    }

    /// Remove a notification by ID.
    pub fn dismiss(&mut self, id: u32) -> bool {
        let before = self.entries.len();
        self.entries.retain(|e| e.id != id);
        self.entries.len() < before
    }

    /// Clear all notifications.
    pub fn clear(&mut self) { self.entries.clear(); }

    pub fn unread_count(&self) -> usize {
        self.entries.iter().filter(|e| !e.read).count()
    }

    pub fn total(&self)  -> usize { self.entries.len() }
    pub fn is_open(&self) -> bool { self.open }

    pub fn toggle(&mut self) {
        self.open = !self.open;
        if self.open { self.mark_all_read(); }
    }

    /// Badge label for the panel applet.
    pub fn badge(&self) -> String {
        let u = self.unread_count();
        if u == 0 { "🔔".to_owned() } else { format!("🔔 {}", u) }
    }

    /// Return entries in display order (newest first).
    pub fn entries(&self) -> Vec<&NotificationEntry> {
        self.entries.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(id: u32) -> NotificationEntry {
        NotificationEntry::new(id, "test-app", "Subject", "Body text")
    }

    #[test]
    fn add_and_count() {
        let mut nc = NotificationCenter::new(10);
        nc.add(entry(1));
        nc.add(entry(2));
        assert_eq!(nc.total(), 2);
        assert_eq!(nc.unread_count(), 2);
    }

    #[test]
    fn capacity_evicts_oldest() {
        let mut nc = NotificationCenter::new(3);
        for i in 1..=4 { nc.add(entry(i)); }
        assert_eq!(nc.total(), 3);
        // Oldest (id=1) should be gone, newest at front
        assert_eq!(nc.entries()[0].id, 4);
    }

    #[test]
    fn mark_read_and_badge() {
        let mut nc = NotificationCenter::new(10);
        nc.add(entry(1)); nc.add(entry(2));
        assert_eq!(nc.badge(), "🔔 2");
        nc.mark_read(1);
        assert_eq!(nc.unread_count(), 1);
        nc.mark_all_read();
        assert_eq!(nc.badge(), "🔔");
    }

    #[test]
    fn dismiss() {
        let mut nc = NotificationCenter::new(10);
        nc.add(entry(7));
        assert!(nc.dismiss(7));
        assert_eq!(nc.total(), 0);
        assert!(!nc.dismiss(7));
    }

    #[test]
    fn toggle_marks_read() {
        let mut nc = NotificationCenter::new(10);
        nc.add(entry(1));
        assert_eq!(nc.unread_count(), 1);
        nc.toggle();
        assert!(nc.is_open());
        assert_eq!(nc.unread_count(), 0);
    }
}
