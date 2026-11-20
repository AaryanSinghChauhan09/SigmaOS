//! SigmaOS Session Manager — desktop session lifecycle management.
//!
//! Manages login sessions, logout, suspend, shutdown, and session save/restore.
//! Implements the `desktop-session.md` platform contract.

#![allow(dead_code)]

use std::fmt;

// ---------------------------------------------------------------------------
// Session state machine
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    LoggedOut,
    Starting,
    Running,
    Locked,
    Suspending,
    Suspended,
    ShuttingDown,
}

impl fmt::Display for SessionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum SessionError {
    AlreadyRunning,
    NotRunning,
    AuthFailed,
    SaveFailed(String),
}

// ---------------------------------------------------------------------------
// Session
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub user: String,
    pub desktop: String,   // "hyprland", "sigma-desktop", etc.
    pub seat: String,
    pub display: String,
}

pub struct SessionManager {
    pub state: SessionState,
    pub session: Option<SessionInfo>,
    lock_count: u32,
}

impl SessionManager {
    pub fn new() -> Self {
        Self { state: SessionState::LoggedOut, session: None, lock_count: 0 }
    }

    pub fn login(&mut self, user: impl Into<String>, desktop: impl Into<String>) -> Result<(), SessionError> {
        if self.state == SessionState::Running {
            return Err(SessionError::AlreadyRunning);
        }
        self.session = Some(SessionInfo {
            user: user.into(),
            desktop: desktop.into(),
            seat: "seat0".into(),
            display: ":0".into(),
        });
        self.state = SessionState::Running;
        Ok(())
    }

    pub fn lock(&mut self) {
        if self.state == SessionState::Running {
            self.state = SessionState::Locked;
            self.lock_count += 1;
        }
    }

    pub fn unlock(&mut self, _credential: &str) -> Result<(), SessionError> {
        if self.state != SessionState::Locked {
            return Err(SessionError::NotRunning);
        }
        // In a real impl: verify credential via SigmaOS-auth
        self.state = SessionState::Running;
        Ok(())
    }

    pub fn suspend(&mut self) {
        self.state = SessionState::Suspending;
        // Real impl: notify services via pre-suspend hook, then suspend
        self.state = SessionState::Suspended;
    }

    pub fn resume(&mut self) {
        if self.state == SessionState::Suspended {
            self.state = SessionState::Running;
        }
    }

    pub fn logout(&mut self) -> Result<SessionInfo, SessionError> {
        let info = self.session.take().ok_or(SessionError::NotRunning)?;
        self.state = SessionState::LoggedOut;
        Ok(info)
    }

    pub fn shutdown(&mut self) {
        self.state = SessionState::ShuttingDown;
    }

    pub fn lock_count(&self) -> u32 { self.lock_count }
}

impl Default for SessionManager {
    fn default() -> Self { Self::new() }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn login_logout_cycle() {
        let mut sm = SessionManager::new();
        assert_eq!(sm.state, SessionState::LoggedOut);
        sm.login("alice", "hyprland").unwrap();
        assert_eq!(sm.state, SessionState::Running);
        let info = sm.logout().unwrap();
        assert_eq!(info.user, "alice");
        assert_eq!(sm.state, SessionState::LoggedOut);
    }

    #[test]
    fn double_login_fails() {
        let mut sm = SessionManager::new();
        sm.login("alice", "hyprland").unwrap();
        assert!(sm.login("bob", "hyprland").is_err());
    }

    #[test]
    fn lock_unlock_cycle() {
        let mut sm = SessionManager::new();
        sm.login("alice", "hyprland").unwrap();
        sm.lock();
        assert_eq!(sm.state, SessionState::Locked);
        sm.unlock("correct-pin").unwrap();
        assert_eq!(sm.state, SessionState::Running);
        assert_eq!(sm.lock_count(), 1);
    }

    #[test]
    fn suspend_resume_cycle() {
        let mut sm = SessionManager::new();
        sm.login("alice", "hyprland").unwrap();
        sm.suspend();
        assert_eq!(sm.state, SessionState::Suspended);
        sm.resume();
        assert_eq!(sm.state, SessionState::Running);
    }
}
