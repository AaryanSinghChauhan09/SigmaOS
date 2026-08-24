//! Display Manager (GDM/LightDM Inspiration)
//! Login screen, session management, and display server spawning

// #![no_std]

extern crate alloc;

use crate::klib::{Vec, String};

/// Session type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionType {
    Wayland,
    X11,
}

/// Session
#[derive(Debug, Clone)]
pub struct Session {
    pub name: String,
    pub session_type: SessionType,
    pub command: String,
}

impl Session {
    pub fn new(name: &str, session_type: SessionType, command: &str) -> Self {
        Self {
            name: name.to_string(),
            session_type,
            command: command.to_string(),
        }
    }
}

/// User
#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub home: String,
}

impl User {
    pub fn new(id: u32, name: &str, home: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            home: home.to_string(),
        }
    }
}

/// Display manager
pub struct DisplayManager {
    pub sessions: Vec<Session>,
    pub users: Vec<User>,
    pub autologin: Option<u32>,
    pub current_session: Option<String>,
}

impl DisplayManager {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
            users: Vec::new(),
            autologin: None,
            current_session: None,
        }
    }

    pub fn add_session(&mut self, session: Session) {
        self.sessions.push(session);
    }

    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    pub fn set_autologin(&mut self, user_id: u32) {
        self.autologin = Some(user_id);
    }

    pub fn start_session(&mut self, session_name: &str, user_id: u32) -> Result<(), DMError> {
        if let Some(session) = self.sessions.iter().find(|s| s.name == session_name) {
            // Start session
            self.current_session = Some(session_name.to_string());
            Ok(())
        } else {
            Err(DMError::SessionNotFound)
        }
    }

    pub fn stop_session(&mut self) -> Result<(), DMError> {
        self.current_session = None;
        Ok(())
    }

    pub fn get_available_sessions(&self) -> Vec<&Session> {
        self.sessions.iter().collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DMError {
    SessionNotFound,
    UserNotFound,
    StartFailed,
}

impl Default for DisplayManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session() {
        let session = Session::new("SigmaOS", SessionType::Wayland, "/usr/bin/sigmaos-wm");
        assert_eq!(session.name, "SigmaOS");
    }

    #[test]
    fn test_display_manager() {
        let mut dm = DisplayManager::new();
        let session = Session::new("SigmaOS", SessionType::Wayland, "/usr/bin/sigmaos-wm");
        dm.add_session(session);
        assert_eq!(dm.sessions.len(), 1);
    }
}