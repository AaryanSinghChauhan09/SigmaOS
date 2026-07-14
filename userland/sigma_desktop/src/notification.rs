use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Notification {
    pub title: String,
    pub body: String,
    pub urgency: u8,
}

/// Manages and displays system and application notifications.
pub struct NotificationCenter {
    pub is_open: bool,
    queue: VecDeque<Notification>,
}

impl Default for NotificationCenter {
    fn default() -> Self {
        Self::new()
    }
}

impl NotificationCenter {
    pub fn new() -> Self {
        Self {
            is_open: false,
            queue: VecDeque::new(),
        }
    }

    pub fn push(&mut self, notification: Notification) {
        self.queue.push_back(notification);
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn render(&self) {
        if self.is_open {
            // Render logic here: draw slide-out panel with notification list
        }
    }
}
