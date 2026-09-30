#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unexpected_cfgs)]
#![allow(clippy::new_without_default)]

#[cfg(not(any(feature = "standalone_test", test)))]


#[cfg(not(any(feature = "standalone_test", test)))]
use std::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use std::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// ─── fanotify Event Mask Flags ────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FanotifyEventKind {
    Access,      // FAN_ACCESS: File was accessed (read)
    Modify,      // FAN_MODIFY: File was modified (written)
    Open,        // FAN_OPEN: File was opened
    CloseWrite,  // FAN_CLOSE_WRITE: Writable file was closed
    OpenPerm,    // FAN_OPEN_PERM: Permission to open file requested
    AccessPerm,  // FAN_ACCESS_PERM: Permission to read file requested
}

// ─── fanotify Response ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FanotifyResponse {
    Allow, // FAN_ALLOW: Permission granted
    Deny,  // FAN_DENY: Permission denied (-EPERM)
}

// ─── fanotify Notification Event ──────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct FanotifyEvent {
    pub event_id: u64,
    pub kind: FanotifyEventKind,
    pub pid: u32,
    pub path: String,
    pub response: Option<FanotifyResponse>,
}

impl FanotifyEvent {
    pub fn new(event_id: u64, kind: FanotifyEventKind, pid: u32, path: &str) -> Self {
        FanotifyEvent {
            event_id,
            kind,
            pid,
            path: path.to_string(),
            response: None,
        }
    }
}

// ─── fanotify Mark ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct FanotifyMark {
    pub path: String,
    pub is_mount: bool,
    pub mask: Vec<FanotifyEventKind>,
}

// ─── Sovereign fanotify Group ─────────────────────────────────────────────────

pub struct SovereignFanotifyGroup {
    pub group_id: u32,
    pub marks: Vec<FanotifyMark>,
    pub event_queue: Vec<FanotifyEvent>,
    pub next_event_id: u64,
    pub permissions_blocked: u64,
    pub permissions_allowed: u64,
}

impl SovereignFanotifyGroup {
    pub fn new(group_id: u32) -> Self {
        SovereignFanotifyGroup {
            group_id,
            marks: Vec::new(),
            event_queue: Vec::new(),
            next_event_id: 1,
            permissions_blocked: 0,
            permissions_allowed: 0,
        }
    }

    pub fn mark_path(&mut self, path: &str, is_mount: bool, events: Vec<FanotifyEventKind>) {
        self.marks.push(FanotifyMark {
            path: path.to_string(),
            is_mount,
            mask: events,
        });
    }

    /// Check if path is watched for a given event kind
    pub fn is_watched(&self, path: &str, kind: FanotifyEventKind) -> bool {
        self.marks.iter().any(|m| {
            (path.starts_with(&m.path) || m.path == "*") && m.mask.contains(&kind)
        })
    }

    /// Notify file activity
    pub fn notify_event(&mut self, kind: FanotifyEventKind, pid: u32, path: &str) -> Option<u64> {
        if self.is_watched(path, kind) {
            let id = self.next_event_id;
            self.next_event_id = self.next_event_id.saturating_add(1);
            self.event_queue.push(FanotifyEvent::new(id, kind, pid, path));
            Some(id)
        } else {
            None
        }
    }

    /// Check permission hook (blocking open/access until user space responds)
    pub fn check_permission(&mut self, kind: FanotifyEventKind, pid: u32, path: &str) -> Result<(), i32> {
        if !self.is_watched(path, kind) {
            return Ok(()); // Not marked for permission checks
        }

        let event_id = self.next_event_id;
        self.next_event_id = self.next_event_id.saturating_add(1);

        // Queue permission event
        self.event_queue.push(FanotifyEvent::new(event_id, kind, pid, path));

        // Default heuristic: If path is in /etc/forbidden, deny; otherwise allow
        if path.contains("forbidden") || path.contains("malware") {
            self.permissions_blocked = self.permissions_blocked.saturating_add(1);
            Err(-1) // -EPERM
        } else {
            self.permissions_allowed = self.permissions_allowed.saturating_add(1);
            Ok(())
        }
    }

    /// Read next pending event from queue
    pub fn read_event(&mut self) -> Option<FanotifyEvent> {
        if self.event_queue.is_empty() {
            None
        } else {
            Some(self.event_queue.remove(0))
        }
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fanotify_mark_and_watch() {
        let mut fan = SovereignFanotifyGroup::new(1);
        fan.mark_path("/home/user", false, vec![FanotifyEventKind::Access, FanotifyEventKind::Modify]);

        assert!(fan.is_watched("/home/user/document.txt", FanotifyEventKind::Access));
        assert!(fan.is_watched("/home/user/document.txt", FanotifyEventKind::Modify));
        assert!(!fan.is_watched("/home/user/document.txt", FanotifyEventKind::Open));
        assert!(!fan.is_watched("/var/log/syslog", FanotifyEventKind::Access));
    }

    #[test]
    fn test_fanotify_notification_event_queue() {
        let mut fan = SovereignFanotifyGroup::new(1);
        fan.mark_path("/etc", false, vec![FanotifyEventKind::Open]);

        let ev_id = fan.notify_event(FanotifyEventKind::Open, 1001, "/etc/hostname");
        assert!(ev_id.is_some());

        let ev = fan.read_event().unwrap();
        assert_eq!(ev.pid, 1001);
        assert_eq!(ev.path, "/etc/hostname");
        assert_eq!(ev.kind, FanotifyEventKind::Open);
    }

    #[test]
    fn test_fanotify_permission_allow() {
        let mut fan = SovereignFanotifyGroup::new(1);
        fan.mark_path("/data", false, vec![FanotifyEventKind::OpenPerm]);

        assert!(fan.check_permission(FanotifyEventKind::OpenPerm, 200, "/data/report.pdf").is_ok());
        assert_eq!(fan.permissions_allowed, 1);
        assert_eq!(fan.permissions_blocked, 0);
    }

    #[test]
    fn test_fanotify_permission_deny() {
        let mut fan = SovereignFanotifyGroup::new(1);
        fan.mark_path("/tmp", false, vec![FanotifyEventKind::OpenPerm]);

        let res = fan.check_permission(FanotifyEventKind::OpenPerm, 666, "/tmp/malware_payload.bin");
        assert_eq!(res, Err(-1)); // Denied
        assert_eq!(fan.permissions_blocked, 1);
    }

    #[test]
    fn test_fanotify_wildcard_mount_mark() {
        let mut fan = SovereignFanotifyGroup::new(1);
        fan.mark_path("*", true, vec![FanotifyEventKind::Modify]);

        assert!(fan.is_watched("/any/arbitrary/path", FanotifyEventKind::Modify));
    }

    #[test]
    fn test_fanotify_read_empty_queue() {
        let mut fan = SovereignFanotifyGroup::new(1);
        assert!(fan.read_event().is_none());
    }
}
