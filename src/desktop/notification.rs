/// SigmaOS Notification Daemon (Phase 4)
/// Inspired by Linux Mint's notification system.

use std::string::String;
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NotificationUrgency { Low, Normal, Critical }

#[derive(Debug, Clone)]
pub struct Notification {
    pub id: u32,
    pub summary: String,
    pub body: String,
    pub urgency: NotificationUrgency,
    pub app_name: String,
    pub timestamp: u64,
    pub dismissed: bool,
}

pub struct NotificationDaemon {
    pub queue: Vec<Notification>,
    pub next_id: u32,
    pub do_not_disturb: bool,
}

impl NotificationDaemon {
    pub fn new() -> Self { Self { queue: Vec::new(), next_id: 1, do_not_disturb: false } }

    pub fn notify(&mut self, app: &str, summary: &str, body: &str, urgency: NotificationUrgency, ts: u64) -> u32 {
        let id = self.next_id; self.next_id += 1;
        self.queue.push(Notification { id, summary: summary.into(), body: body.into(), urgency, app_name: app.into(), timestamp: ts, dismissed: false });
        id
    }

    pub fn dismiss(&mut self, id: u32) {
        if let Some(n) = self.queue.iter_mut().find(|n| n.id == id) { n.dismissed = true; }
    }

    pub fn pending(&self) -> Vec<&Notification> {
        self.queue.iter().filter(|n| !n.dismissed && (!self.do_not_disturb || n.urgency == NotificationUrgency::Critical)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_lifecycle() {
        let mut d = NotificationDaemon::new();
        let id = d.notify("System", "Update available", "3 packages", NotificationUrgency::Normal, 1000);
        assert_eq!(d.pending().len(), 1);
        d.dismiss(id);
        assert_eq!(d.pending().len(), 0);
    }

    #[test]
    fn test_dnd_passes_critical() {
        let mut d = NotificationDaemon::new();
        d.do_not_disturb = true;
        d.notify("App", "Normal", "", NotificationUrgency::Normal, 1);
        d.notify("System", "CRITICAL", "", NotificationUrgency::Critical, 2);
        assert_eq!(d.pending().len(), 1);
        assert_eq!(d.pending()[0].urgency, NotificationUrgency::Critical);
    }
}
