//! SigmaOS Notification System
//!
//! Desktop notification service with urgency levels, action callbacks,
//! and an in-memory notification queue.
//! Upstream inspiration: libnotify (https://gitlab.gnome.org/GNOME/libnotify),
//! mako notification daemon (https://github.com/emersion/mako)

use std::collections::VecDeque;

/// Urgency level of a desktop notification (mirrors libnotify spec).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Urgency {
    Low,
    Normal,
    Critical,
}

impl std::fmt::Display for Urgency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Urgency::Low      => write!(f, "low"),
            Urgency::Normal   => write!(f, "normal"),
            Urgency::Critical => write!(f, "critical"),
        }
    }
}

/// A desktop notification with optional action labels.
#[derive(Debug, Clone)]
pub struct Notification {
    pub id:      u32,
    pub app:     String,
    pub summary: String,
    pub body:    String,
    pub urgency: Urgency,
    pub timeout_ms: i32,   // -1 = never, 0 = default
    pub actions: Vec<String>,
    pub dismissed: bool,
}

impl Notification {
    pub fn new(app: &str, summary: &str, body: &str) -> Self {
        static COUNTER: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(1);
        Self {
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            app:      app.to_owned(),
            summary:  summary.to_owned(),
            body:     body.to_owned(),
            urgency:  Urgency::Normal,
            timeout_ms: 0,
            actions:  Vec::new(),
            dismissed: false,
        }
    }

    pub fn urgency(mut self, u: Urgency) -> Self { self.urgency = u; self }
    pub fn timeout(mut self, ms: i32)    -> Self { self.timeout_ms = ms; self }
    pub fn action(mut self, label: &str) -> Self { self.actions.push(label.to_owned()); self }
}

/// In-memory notification queue (D-Bus daemon would wrap this).
pub struct NotificationServer {
    queue: VecDeque<Notification>,
    history: Vec<Notification>,
}

impl NotificationServer {
    pub fn new() -> Self {
        Self { queue: VecDeque::new(), history: Vec::new() }
    }

    /// Send a notification; returns the assigned notification ID.
    pub fn send(&mut self, n: Notification) -> u32 {
        let id = n.id;
        self.queue.push_back(n);
        id
    }

    /// Dismiss a notification by ID.
    pub fn dismiss(&mut self, id: u32) -> bool {
        if let Some(pos) = self.queue.iter().position(|n| n.id == id) {
            let mut n = self.queue.remove(pos).unwrap();
            n.dismissed = true;
            self.history.push(n);
            true
        } else {
            false
        }
    }

    /// Pop the oldest notification for display.
    pub fn pop(&mut self) -> Option<Notification> {
        self.queue.pop_front()
    }

    pub fn pending(&self) -> usize  { self.queue.len() }
    pub fn history(&self)  -> &[Notification] { &self.history }
}

impl Default for NotificationServer { fn default() -> Self { Self::new() } }

/// Convenience function — create and send a one-liner notification.
pub fn notify(app: &str, summary: &str, body: &str) {
    let mut srv = NotificationServer::new();
    let id = srv.send(Notification::new(app, summary, body));
    println!("[notify] id={} app={} {}: {}", id, app, summary, body);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_and_pop() {
        let mut srv = NotificationServer::new();
        let n = Notification::new("test-app", "Hello", "World");
        let id = srv.send(n);
        assert_eq!(srv.pending(), 1);
        let popped = srv.pop().unwrap();
        assert_eq!(popped.id, id);
        assert_eq!(popped.summary, "Hello");
        assert_eq!(srv.pending(), 0);
    }

    #[test]
    fn dismiss_removes_from_queue() {
        let mut srv = NotificationServer::new();
        let id = srv.send(Notification::new("app", "S", "B"));
        assert!(srv.dismiss(id));
        assert_eq!(srv.pending(), 0);
        assert_eq!(srv.history().len(), 1);
        assert!(srv.history()[0].dismissed);
    }

    #[test]
    fn urgency_display() {
        assert_eq!(Urgency::Critical.to_string(), "critical");
        assert_eq!(Urgency::Low.to_string(),      "low");
    }

    #[test]
    fn builder_chain() {
        let n = Notification::new("app", "summary", "body")
            .urgency(Urgency::Critical)
            .timeout(5000)
            .action("OK")
            .action("Dismiss");
        assert_eq!(n.urgency,    Urgency::Critical);
        assert_eq!(n.timeout_ms, 5000);
        assert_eq!(n.actions.len(), 2);
    }
}
