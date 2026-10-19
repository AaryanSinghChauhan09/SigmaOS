// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaSession - User Session Manager
//!
//! SigmaSession provides user session management for SigmaOS, inspired by systemd-logind from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Session type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionType {
    /// TTY session
    Tty,
    /// X11 session
    X11,
    /// Wayland session
    Wayland,
    /// Web session
    Web,
    /// Unknown
    Unknown,
;

impl fmt::Display for SessionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SessionType::Tty => write!(f, "tty"),
            SessionType::X11 => write!(f, "x11"),
            SessionType::Wayland => write!(f, "wayland"),
            SessionType::Web => write!(f, "web"),
            SessionType::Unknown => write!(f, "unknown"),
        }
    }
;

/// Session state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    /// Active
    Active,
    /// Online
    Online,
    /// Closing
    Closing,
    /// Closed
    Closed,
;

impl fmt::Display for SessionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SessionState::Active => write!(f, "active"),
            SessionState::Online => write!(f, "online"),
            SessionState::Closing => write!(f, "closing"),
            SessionState::Closed => write!(f, "closed"),
        }
    }
;

/// User session
#[derive(Debug, Clone)]
pub struct UserSession {
    /// Session ID
    pub session_id: String,
    /// User ID
    pub user_id: String,
    /// Username
    pub username: String,
    /// Session type
    pub session_type: SessionType,
    /// Session state
    pub state: SessionState,
    /// Display number
    pub display: Option<String>,
    /// TTY device
    pub tty: Option<String>,
    /// Remote host
    pub remote_host: Option<String>,
    /// Leader PID
    pub leader_pid: Option<u32>,
;

impl UserSession {
    /// Create a new user session
    pub fn new(
        session_id: String,
        user_id: String,
        username: String,
        session_type: SessionType,
    ) -> Self {
        Self {
            session_id,
            user_id,
            username,
            session_type,
            state: SessionState::Active,
            display: None,
            tty: None,
            remote_host: None,
            leader_pid: None,
        }
    }

    /// Set state
    pub fn with_state(mut self, state: SessionState) -> Self {
        self.state = state;
        self
    }

    /// Set display
    pub fn with_display(mut self, display: String) -> Self {
        self.display = Some(display);
        self
    }

    /// Set TTY
    pub fn with_tty(mut self, tty: String) -> Self {
        self.tty = Some(tty);
        self
    }

    /// Set remote host
    pub fn with_remote_host(mut self, remote_host: String) -> Self {
        self.remote_host = Some(remote_host);
        self
    }

    /// Set leader PID
    pub fn with_leader_pid(mut self, leader_pid: u32) -> Self {
        self.leader_pid = Some(leader_pid);
        self
    }
;

/// SigmaSession - User Session Manager
pub struct SigmaSession {
    /// User sessions (session_id -> session)
    sessions: BTreeMap<String, UserSession>,
    /// Running state
    running: bool,
    /// Session counter
    session_counter: u32,
;

impl SigmaSession {
    /// Create a new SigmaSession instance
    pub fn new() -> Self {
        Self {
            sessions: BTreeMap::new(),
            running: false,
            session_counter: 0,
        }
    }

    /// Start the session manager
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the session manager
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add a session
    pub fn add_session(&mut self, session: UserSession) -> Result<(), String> {
        if self.sessions.contains_key(&session.session_id) {
            return Err(format!("Session '{}' already exists", session.session_id));
        }

        self.sessions.insert(session.session_id.clone(), session);
        self.session_counter += 1;
        Ok(())
    }

    /// Remove a session
    pub fn remove_session(&mut self, session_id: &str) -> Result<(), String> {
        if self.sessions.remove(session_id).is_none() {
            return Err(format!("Session '{}' not found", session_id));
        }
        Ok(())
    }

    /// Get a session
    pub fn get_session(&self, session_id: &str) -> Option<&UserSession> {
        self.sessions.get(session_id)
    }

    /// Activate a session
    pub fn activate_session(&mut self, session_id: &str) -> Result<(), String> {
        match self.sessions.get_mut(session_id) {
            Some(session) => {
                session.state = SessionState::Active;
                Ok(())
            }
            None => Err(format!("Session '{}' not found", session_id)),
        }
    }

    /// Deactivate a session
    pub fn deactivate_session(&mut self, session_id: &str) -> Result<(), String> {
        match self.sessions.get_mut(session_id) {
            Some(session) => {
                session.state = SessionState::Online;
                Ok(())
            }
            None => Err(format!("Session '{}' not found", session_id)),
        }
    }

    /// Close a session
    pub fn close_session(&mut self, session_id: &str) -> Result<(), String> {
        match self.sessions.get_mut(session_id) {
            Some(session) => {
                session.state = SessionState::Closing;
                Ok(())
            }
            None => Err(format!("Session '{}' not found", session_id)),
        }
    }

    /// List all sessions
    pub fn list_sessions(&self) -> Vec<&UserSession> {
        self.sessions.values().collect()
    }

    /// List sessions by user
    pub fn list_by_user(&self, user_id: &str) -> Vec<&UserSession> {
        self.sessions
            .values()
            .filter(|s| s.user_id == user_id)
            .collect()
    }

    /// List active sessions
    pub fn list_active(&self) -> Vec<&UserSession> {
        self.sessions
            .values()
            .filter(|s| s.state == SessionState::Active)
            .collect()
    }

    /// List sessions by type
    pub fn list_by_type(&self, session_type: SessionType) -> Vec<&UserSession> {
        self.sessions
            .values()
            .filter(|s| s.session_type == session_type)
            .collect()
    }

    /// Get session count
    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }

    /// Get session statistics
    pub fn session_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert(
            "total_sessions".to_string(),
            self.session_count().to_string(),
        );
        stats.insert(
            "active_sessions".to_string(),
            self.list_active().len().to_string(),
        );
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
;

impl Default for SigmaSession {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaSession instance (placeholder for actual global state)
static mut SIGMASESSION_INSTANCE: Option<SigmaSession> = None;

/// Initialize the global SigmaSession instance
pub fn init_sigmasession() {
    unsafe {
        SIGMASESSION_INSTANCE = Some(SigmaSession::new());
    }
;

/// Get the global SigmaSession instance
pub fn get_sigmasession() -> Option<&'static mut SigmaSession> {
    unsafe { SIGMASESSION_INSTANCE.as_mut() }
;

/// Convenience function to add a session
pub fn add_user_session(session: UserSession) -> Result<(), String> {
    if let Some(session_mgr) = get_sigmasession() {
        session_mgr.add_session(session)
    } else {
        Err("SigmaSession not initialized".to_string())
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_type_display() {
        assert_eq!(SessionType::Tty.to_string(), "tty");
        assert_eq!(SessionType::X11.to_string(), "x11");
    }

    #[test]
    fn test_session_state_display() {
        assert_eq!(SessionState::Active.to_string(), "active");
        assert_eq!(SessionState::Closed.to_string(), "closed");
    }

    #[test]
    fn test_user_session() {
        let session = UserSession::new(
            "session1".to_string(),
            "1000".to_string(),
            "user".to_string(),
            SessionType::X11,
        )
        .with_state(SessionState::Active)
        .with_display(":0".to_string())
        .with_leader_pid(1234);

        assert_eq!(session.session_id, "session1");
        assert_eq!(session.state, SessionState::Active);
        assert_eq!(session.display, Some(":0".to_string()));
    }

    #[test]
    fn test_sigmasession() {
        let mut session_mgr = SigmaSession::new();
        assert!(!session_mgr.is_running());
        session_mgr.start();
        assert!(session_mgr.is_running());
        session_mgr.stop();
        assert!(!session_mgr.is_running());
    }

    #[test]
    fn test_sigmasession_add_session() {
        let mut session_mgr = SigmaSession::new();

        let session = UserSession::new(
            "session1".to_string(),
            "1000".to_string(),
            "user".to_string(),
            SessionType::X11,
        );

        assert!(session_mgr.add_session(session).is_ok());
        assert_eq!(session_mgr.session_count(), 1);
    }

    #[test]
    fn test_sigmasession_activate_session() {
        let mut session_mgr = SigmaSession::new();

        let session = UserSession::new(
            "session1".to_string(),
            "1000".to_string(),
            "user".to_string(),
            SessionType::X11,
        )
        .with_state(SessionState::Online);

        session_mgr.add_session(session).unwrap();
        assert!(session_mgr.activate_session("session1").is_ok());

        let session = session_mgr.get_session("session1").unwrap();
        assert_eq!(session.state, SessionState::Active);
    }

    #[test]
    fn test_sigmasession_close_session() {
        let mut session_mgr = SigmaSession::new();

        let session = UserSession::new(
            "session1".to_string(),
            "1000".to_string(),
            "user".to_string(),
            SessionType::X11,
        );

        session_mgr.add_session(session).unwrap();
        assert!(session_mgr.close_session("session1").is_ok());

        let session = session_mgr.get_session("session1").unwrap();
        assert_eq!(session.state, SessionState::Closing);
    }

    #[test]
    fn test_sigmasession_list_by_user() {
        let mut session_mgr = SigmaSession::new();

        let session = UserSession::new(
            "session1".to_string(),
            "1000".to_string(),
            "user".to_string(),
            SessionType::X11,
        );

        session_mgr.add_session(session).unwrap();

        let sessions = session_mgr.list_by_user("1000");
        assert_eq!(sessions.len(), 1);
    }

    #[test]
    fn test_sigmasession_list_by_type() {
        let mut session_mgr = SigmaSession::new();

        let session = UserSession::new(
            "session1".to_string(),
            "1000".to_string(),
            "user".to_string(),
            SessionType::X11,
        );

        session_mgr.add_session(session).unwrap();

        let sessions = session_mgr.list_by_type(SessionType::X11);
        assert_eq!(sessions.len(), 1);
    }

    #[test]
    fn test_sigmasession_stats() {
        let mut session_mgr = SigmaSession::new();

        let session = UserSession::new(
            "session1".to_string(),
            "1000".to_string(),
            "user".to_string(),
            SessionType::X11,
        )
        .with_state(SessionState::Active);

        session_mgr.add_session(session).unwrap();

        let stats = session_mgr.session_stats();
        assert_eq!(stats.get("total_sessions"), Some(&"1".to_string()));
        assert_eq!(stats.get("active_sessions"), Some(&"1".to_string()));
    }

    #[test]
    fn test_add_user_session_convenience() {
        init_sigmasession();

        let session = UserSession::new(
            "session1".to_string(),
            "1000".to_string(),
            "user".to_string(),
            SessionType::X11,
        );

        let result = add_user_session(session);
        assert!(result.is_ok());
    }
;
