//! SigmaOS Display Manager — graphical login manager.
//!
//! Handles Wayland session startup, user authentication, greeter orchestration,
//! and session selection. Implements the `desktop-session.md` platform contract
//! for the login phase.

#![allow(dead_code)]

use std::fmt;

// ---------------------------------------------------------------------------
// Session type
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionType {
    pub id: String,
    pub name: String,
    pub exec: String,
    pub desktop_names: Vec<String>,
    pub kind: SessionKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionKind {
    Wayland,
    X11,
    Tty,
}

impl fmt::Display for SessionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self) }
}

// ---------------------------------------------------------------------------
// Greeter state
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GreeterState {
    Idle,
    WaitingForCredentials,
    Authenticating,
    Authenticated,
    LaunchingSession,
    Failed,
}

// ---------------------------------------------------------------------------
// Auth result
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum AuthResult {
    Success { user: String },
    Failure { reason: String, retry_allowed: bool },
}

// ---------------------------------------------------------------------------
// Display Manager
// ---------------------------------------------------------------------------

pub struct DisplayManager {
    pub greeter_state: GreeterState,
    pub available_sessions: Vec<SessionType>,
    pub selected_session: Option<usize>,
    pub auto_login_user: Option<String>,
    pub theme: String,
    auth_failures: u32,
    max_failures: u32,
}

impl DisplayManager {
    pub fn new() -> Self {
        let sessions = vec![
            SessionType {
                id: "sigma-wayland".into(),
                name: "SigmaOS (Wayland)".into(),
                exec: "/usr/bin/sigma-desktop --wayland".into(),
                desktop_names: vec!["SigmaOS".into()],
                kind: SessionKind::Wayland,
            },
            SessionType {
                id: "hyprland".into(),
                name: "Hyprland".into(),
                exec: "/usr/bin/Hyprland".into(),
                desktop_names: vec!["Hyprland".into()],
                kind: SessionKind::Wayland,
            },
            SessionType {
                id: "tty".into(),
                name: "TTY Login".into(),
                exec: "/bin/login".into(),
                desktop_names: vec![],
                kind: SessionKind::Tty,
            },
        ];
        Self {
            greeter_state: GreeterState::Idle,
            available_sessions: sessions,
            selected_session: Some(0),
            auto_login_user: None,
            theme: "sigma-dark".into(),
            auth_failures: 0,
            max_failures: 5,
        }
    }

    pub fn start_greeter(&mut self) {
        self.greeter_state = GreeterState::WaitingForCredentials;
    }

    /// Authenticate a user. In a real impl, delegates to PAM / SigmaOS-auth.
    /// Here: accepts any non-empty credential for scaffold testing.
    pub fn authenticate(&mut self, user: &str, credential: &str) -> AuthResult {
        self.greeter_state = GreeterState::Authenticating;
        if user.is_empty() || credential.is_empty() {
            self.auth_failures += 1;
            let retry = self.auth_failures < self.max_failures;
            self.greeter_state = GreeterState::Failed;
            return AuthResult::Failure {
                reason: "Empty credentials".into(),
                retry_allowed: retry,
            };
        }
        // Scaffold: any non-empty credential succeeds
        self.auth_failures = 0;
        self.greeter_state = GreeterState::Authenticated;
        AuthResult::Success { user: user.to_string() }
    }

    pub fn select_session(&mut self, index: usize) -> bool {
        if index < self.available_sessions.len() {
            self.selected_session = Some(index);
            true
        } else {
            false
        }
    }

    pub fn launch_session(&mut self, user: &str) -> Result<String, String> {
        if self.greeter_state != GreeterState::Authenticated {
            return Err("not authenticated".to_string());
        }
        let idx = self.selected_session.ok_or_else(|| "no session selected".to_string())?;
        let session = &self.available_sessions[idx];
        self.greeter_state = GreeterState::LaunchingSession;
        Ok(format!("launching '{}' for user '{}': {}", session.name, user, session.exec))
    }

    pub fn session_names(&self) -> Vec<&str> {
        self.available_sessions.iter().map(|s| s.name.as_str()).collect()
    }

    pub fn wayland_sessions(&self) -> Vec<&SessionType> {
        self.available_sessions.iter().filter(|s| s.kind == SessionKind::Wayland).collect()
    }

    pub fn auth_failure_count(&self) -> u32 { self.auth_failures }
}

impl Default for DisplayManager {
    fn default() -> Self { Self::new() }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeter_starts_waiting() {
        let mut dm = DisplayManager::new();
        dm.start_greeter();
        assert_eq!(dm.greeter_state, GreeterState::WaitingForCredentials);
    }

    #[test]
    fn auth_success() {
        let mut dm = DisplayManager::new();
        dm.start_greeter();
        match dm.authenticate("alice", "password") {
            AuthResult::Success { user } => assert_eq!(user, "alice"),
            _ => panic!("expected success"),
        }
        assert_eq!(dm.greeter_state, GreeterState::Authenticated);
    }

    #[test]
    fn auth_failure_empty_creds() {
        let mut dm = DisplayManager::new();
        dm.start_greeter();
        match dm.authenticate("", "") {
            AuthResult::Failure { retry_allowed, .. } => assert!(retry_allowed),
            _ => panic!("expected failure"),
        }
        assert_eq!(dm.auth_failure_count(), 1);
    }

    #[test]
    fn launch_requires_auth() {
        let mut dm = DisplayManager::new();
        assert!(dm.launch_session("alice").is_err());
    }

    #[test]
    fn launch_after_auth() {
        let mut dm = DisplayManager::new();
        dm.start_greeter();
        dm.authenticate("alice", "pass");
        let result = dm.launch_session("alice").unwrap();
        assert!(result.contains("alice"));
    }

    #[test]
    fn wayland_sessions_filtered() {
        let dm = DisplayManager::new();
        let wayland = dm.wayland_sessions();
        assert!(!wayland.is_empty());
        assert!(wayland.iter().all(|s| s.kind == SessionKind::Wayland));
    }
}
