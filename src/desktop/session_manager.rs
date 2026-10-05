// SigmaOS Session Manager
// Inspired by Linux Mint's session management and Omarchy's session utilities

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Session type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopSessionType {
    X11,
    Wayland,
    TTY,
    Remote,
}

impl DesktopSessionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopSessionType::X11 => "X11",
            DesktopSessionType::Wayland => "Wayland",
            DesktopSessionType::TTY => "TTY",
            DesktopSessionType::Remote => "Remote",
        }
    }
}

/// Session status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopSessionStatus {
    Active,
    Inactive,
    Failed,
    Terminated,
}

impl DesktopSessionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopSessionStatus::Active => "Active",
            DesktopSessionStatus::Inactive => "Inactive",
            DesktopSessionStatus::Failed => "Failed",
            DesktopSessionStatus::Terminated => "Terminated",
        }
    }
}

/// User session
#[derive(Debug, Clone)]
pub struct DesktopUserSession {
    pub id: String,
    pub user: String,
    pub session_type: DesktopSessionType,
    pub status: DesktopSessionStatus,
    pub display: String,
    pub tty: Option<String>,
    pub remote_host: Option<String>,
    pub start_time: u64,
    pub processes: Vec<String>,
}

impl DesktopUserSession {
    pub fn new(
        id: String,
        user: String,
        session_type: DesktopSessionType,
        display: String,
    ) -> Self {
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        DesktopUserSession {
            id,
            user,
            session_type,
            status: DesktopSessionStatus::Active,
            display,
            tty: None,
            remote_host: None,
            start_time,
            processes: Vec::new(),
        }
    }

    pub fn set_status(&mut self, status: DesktopSessionStatus) {
        self.status = status;
    }

    pub fn set_tty(&mut self, tty: String) {
        self.tty = Some(tty);
    }

    pub fn set_remote_host(&mut self, host: String) {
        self.remote_host = Some(host);
    }

    pub fn add_process(&mut self, pid: String) {
        self.processes.push(pid);
    }

    pub fn remove_process(&mut self, pid: &str) {
        self.processes.retain(|p| p != pid);
    }
}

/// Session Manager
pub struct DesktopSessionManager {
    sessions: HashMap<String, DesktopUserSession>,
    active_session: Option<String>,
    next_session_id: u32,
}

impl DesktopSessionManager {
    pub fn new() -> Self {
        DesktopSessionManager {
            sessions: HashMap::new(),
            active_session: None,
            next_session_id: 1,
        }
    }

    pub fn create_session(
        &mut self,
        user: String,
        session_type: DesktopSessionType,
        display: String,
    ) -> String {
        let id = format!("session_{}", self.next_session_id);
        let session = DesktopUserSession::new(id.clone(), user, session_type, display);
        self.sessions.insert(id.clone(), session);
        self.next_session_id += 1;

        // If this is the first session, make it active
        if self.active_session.is_none() {
            self.active_session = Some(id.clone());
        }

        id
    }

    pub fn remove_session(&mut self, id: &str) -> bool {
        if let Some(session) = self.sessions.get(id) {
            if session.status == DesktopSessionStatus::Active {
                return false; // Cannot remove active session
            }
        }

        let removed = self.sessions.remove(id).is_some();

        // If we removed the active session, clear it
        if self.active_session.as_ref() == Some(&id.to_string()) {
            self.active_session = None;
        }

        removed
    }

    pub fn get_session(&self, id: &str) -> Option<&DesktopUserSession> {
        self.sessions.get(id)
    }

    pub fn get_sessions(&self) -> Vec<&DesktopUserSession> {
        self.sessions.values().collect()
    }

    pub fn get_sessions_by_user(&self, user: &str) -> Vec<&DesktopUserSession> {
        self.sessions
            .values()
            .filter(|s| s.user == user)
            .collect()
    }

    pub fn get_sessions_by_type(&self, session_type: DesktopSessionType) -> Vec<&DesktopUserSession> {
        self.sessions
            .values()
            .filter(|s| s.session_type == session_type)
            .collect()
    }

    pub fn get_active_session(&self) -> Option<&DesktopUserSession> {
        if let Some(active_id) = &self.active_session {
            self.sessions.get(active_id)
        } else {
            None
        }
    }

    pub fn set_active_session(&mut self, id: &str) -> bool {
        if !self.sessions.contains_key(id) {
            return false;
        }

        // Deactivate current active session
        if let Some(current_id) = &self.active_session {
            if let Some(session) = self.sessions.get_mut(current_id) {
                session.set_status(DesktopSessionStatus::Inactive);
            }
        }

        // Activate new session
        if let Some(session) = self.sessions.get_mut(id) {
            session.set_status(DesktopSessionStatus::Active);
            self.active_session = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn terminate_session(&mut self, id: &str) -> bool {
        if let Some(session) = self.sessions.get_mut(id) {
            session.set_status(DesktopSessionStatus::Terminated);

            // If this was the active session, clear it
            if self.active_session.as_ref() == Some(&id.to_string()) {
                self.active_session = None;
            }

            true
        } else {
            false
        }
    }

    pub fn add_process_to_session(&mut self, session_id: &str, pid: String) -> bool {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.add_process(pid);
            true
        } else {
            false
        }
    }

    pub fn remove_process_from_session(&mut self, session_id: &str, pid: &str) -> bool {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.remove_process(pid);
            true
        } else {
            false
        }
    }

    pub fn get_session_count(&self) -> usize {
        self.sessions.len()
    }

    pub fn get_active_session_count(&self) -> usize {
        self.sessions
            .values()
            .filter(|s| s.status == DesktopSessionStatus::Active)
            .count()
    }

    pub fn get_statistics(&self) -> DesktopSessionStatistics {
        DesktopSessionStatistics {
            total_sessions: self.sessions.len(),
            active_sessions: self.get_active_session_count(),
            x11_sessions: self.get_sessions_by_type(DesktopSessionType::X11).len(),
            wayland_sessions: self.get_sessions_by_type(DesktopSessionType::Wayland).len(),
            tty_sessions: self.get_sessions_by_type(DesktopSessionType::TTY).len(),
            remote_sessions: self.get_sessions_by_type(DesktopSessionType::Remote).len(),
        }
    }
}

impl Default for DesktopSessionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Session statistics
#[derive(Debug, Clone, Copy)]
pub struct DesktopSessionStatistics {
    pub total_sessions: usize,
    pub active_sessions: usize,
    pub x11_sessions: usize,
    pub wayland_sessions: usize,
    pub tty_sessions: usize,
    pub remote_sessions: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_manager_initialization() {
        let manager = DesktopSessionManager::new();
        assert_eq!(manager.get_session_count(), 0);
        assert!(manager.get_active_session().is_none());
    }

    #[test]
    fn test_create_session() {
        let mut manager = DesktopSessionManager::new();
        let id = manager.create_session(
            "user1".to_string(),
            DesktopSessionType::Wayland,
            ":0".to_string(),
        );
        assert!(manager.get_session(&id).is_some());
        assert_eq!(manager.get_session_count(), 1);
    }

    #[test]
    fn test_remove_session() {
        let mut manager = DesktopSessionManager::new();
        let id = manager.create_session(
            "user1".to_string(),
            DesktopSessionType::Wayland,
            ":0".to_string(),
        );
        manager.terminate_session(&id);
        assert!(manager.remove_session(&id));
        assert_eq!(manager.get_session_count(), 0);
    }

    #[test]
    fn test_set_active_session() {
        let mut manager = DesktopSessionManager::new();
        let id1 = manager.create_session(
            "user1".to_string(),
            DesktopSessionType::Wayland,
            ":0".to_string(),
        );
        let id2 = manager.create_session(
            "user2".to_string(),
            DesktopSessionType::X11,
            ":1".to_string(),
        );

        assert!(manager.set_active_session(&id2));
        assert_eq!(manager.get_active_session().unwrap().id, id2);
    }

    #[test]
    fn test_terminate_session() {
        let mut manager = DesktopSessionManager::new();
        let id = manager.create_session(
            "user1".to_string(),
            DesktopSessionType::Wayland,
            ":0".to_string(),
        );
        assert!(manager.terminate_session(&id));
        assert_eq!(
            manager.get_session(&id).unwrap().status,
            DesktopSessionStatus::Terminated
        );
    }

    #[test]
    fn test_add_process_to_session() {
        let mut manager = DesktopSessionManager::new();
        let id = manager.create_session(
            "user1".to_string(),
            DesktopSessionType::Wayland,
            ":0".to_string(),
        );
        assert!(manager.add_process_to_session(&id, "1234".to_string()));
        assert_eq!(manager.get_session(&id).unwrap().processes.len(), 1);
    }

    #[test]
    fn test_remove_process_from_session() {
        let mut manager = DesktopSessionManager::new();
        let id = manager.create_session(
            "user1".to_string(),
            DesktopSessionType::Wayland,
            ":0".to_string(),
        );
        manager.add_process_to_session(&id, "1234".to_string());
        assert!(manager.remove_process_from_session(&id, "1234"));
        assert_eq!(manager.get_session(&id).unwrap().processes.len(), 0);
    }

    #[test]
    fn test_get_sessions_by_user() {
        let mut manager = DesktopSessionManager::new();
        manager.create_session("user1".to_string(), DesktopSessionType::Wayland, ":0".to_string());
        manager.create_session("user1".to_string(), DesktopSessionType::X11, ":1".to_string());
        manager.create_session("user2".to_string(), DesktopSessionType::Wayland, ":2".to_string());

        let user1_sessions = manager.get_sessions_by_user("user1");
        assert_eq!(user1_sessions.len(), 2);
    }

    #[test]
    fn test_get_sessions_by_type() {
        let mut manager = DesktopSessionManager::new();
        manager.create_session("user1".to_string(), DesktopSessionType::Wayland, ":0".to_string());
        manager.create_session("user2".to_string(), DesktopSessionType::X11, ":1".to_string());
        manager.create_session("user3".to_string(), DesktopSessionType::Wayland, ":2".to_string());

        let wayland_sessions = manager.get_sessions_by_type(DesktopSessionType::Wayland);
        assert_eq!(wayland_sessions.len(), 2);
    }

    #[test]
    fn test_statistics() {
        let mut manager = DesktopSessionManager::new();
        manager.create_session("user1".to_string(), DesktopSessionType::Wayland, ":0".to_string());
        manager.create_session("user2".to_string(), DesktopSessionType::X11, ":1".to_string());
        manager.create_session("user3".to_string(), DesktopSessionType::TTY, "tty1".to_string());

        let stats = manager.get_statistics();
        assert_eq!(stats.total_sessions, 3);
        assert_eq!(stats.wayland_sessions, 1);
        assert_eq!(stats.x11_sessions, 1);
        assert_eq!(stats.tty_sessions, 1);
    }
}
