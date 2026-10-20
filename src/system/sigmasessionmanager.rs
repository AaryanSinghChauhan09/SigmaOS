// SPDX-License-Identifier: MIT
// SigmaOS - Session Manager
// Session management inspired by Omarchy uwsm session and Linux session systems

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Session type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionType {
    Wayland,
    X11,
    TTY,
}

/// Session
#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub session_type: SessionType,
    pub display: String,
    pub active: bool,
}

/// Session manager
pub struct SigmaSessionManager {
    sessions: Vec<Session>,
}

impl SigmaSessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
        }
    }

    pub fn add_session(&mut self, session: Session) {
        self.sessions.push(session);
    }

    pub fn get_sessions(&self) -> &[Session] {
        &self.sessions
    }

    pub fn get_active_session(&self) -> Option<&Session> {
        self.sessions.iter().find(|s| s.active)
    }

    pub fn set_active_session(&mut self, id: String) -> Result<(), String> {
        for session in &mut self.sessions {
            session.active = session.id == id;
        }
        Ok(())
    }
}

impl Default for SigmaSessionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_manager_creation() {
        let manager = SigmaSessionManager::new();
        assert_eq!(manager.get_sessions().len(), 0);
    }

    #[test]
    fn test_add_session() {
        let mut manager = SigmaSessionManager::new();
        let session = Session {
            id: "wayland-0".to_string(),
            session_type: SessionType::Wayland,
            display: "wayland-0".to_string(),
            active: true,
        };
        manager.add_session(session);
        assert_eq!(manager.get_sessions().len(), 1);
    }

    #[test]
    fn test_set_active_session() {
        let mut manager = SigmaSessionManager::new();
        let session1 = Session {
            id: "wayland-0".to_string(),
            session_type: SessionType::Wayland,
            display: "wayland-0".to_string(),
            active: true,
        };
        let session2 = Session {
            id: "wayland-1".to_string(),
            session_type: SessionType::Wayland,
            display: "wayland-1".to_string(),
            active: false,
        };
        manager.add_session(session1);
        manager.add_session(session2);
        manager.set_active_session("wayland-1".to_string()).unwrap();
        assert!(manager.get_active_session().unwrap().id == "wayland-1");
    }
}
