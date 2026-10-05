//! Display Manager
//!
//! Display and login session management inspired by Linux Mint's MDM and
//! Omarchy's login system, supporting user authentication, session selection,
//! and display server management.

use std::collections::HashMap;

/// Session type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionType {
    X11,
    Wayland,
    Tty,
}

impl SessionType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "x11" | "xorg" => Some(SessionType::X11),
            "wayland" => Some(SessionType::Wayland),
            "tty" => Some(SessionType::Tty),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SessionType::X11 => "X11",
            SessionType::Wayland => "Wayland",
            SessionType::Tty => "TTY",
        }
    }
}

/// Desktop session
#[derive(Debug, Clone)]
pub struct DesktopSession {
    pub name: String,
    pub command: String,
    pub session_type: SessionType,
    pub is_default: bool,
}

impl DesktopSession {
    pub fn new(name: String, command: String, session_type: SessionType) -> Self {
        Self {
            name,
            command,
            session_type,
            is_default: false,
        }
    }

    pub fn set_default(&mut self, default: bool) {
        self.is_default = default;
    }
}

/// User session
#[derive(Debug, Clone)]
pub struct UserSession {
    pub username: String,
    pub session: String,
    pub display: String,
    pub start_time: u64,
    pub is_active: bool,
}

impl UserSession {
    pub fn new(username: String, session: String, display: String) -> Self {
        let start_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            username,
            session,
            display,
            start_time,
            is_active: true,
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }
}

/// Display manager configuration
#[derive(Debug, Clone)]
pub struct DisplayConfig {
    pub auto_login_enabled: bool,
    pub auto_login_user: Option<String>,
    pub default_session: String,
    pub allow_guest_login: bool,
    pub remember_last_session: bool,
    pub hide_users: bool,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            auto_login_enabled: false,
            auto_login_user: None,
            default_session: "SigmaOS".to_string(),
            allow_guest_login: false,
            remember_last_session: true,
            hide_users: false,
        }
    }
}

/// Display manager
#[derive(Debug)]
pub struct DisplayManager {
    sessions: HashMap<String, DesktopSession>,
    active_sessions: Vec<UserSession>,
    config: DisplayConfig,
    current_display: u32,
}

impl DisplayManager {
    pub fn new() -> Self {
        let mut manager = Self {
            sessions: HashMap::new(),
            active_sessions: Vec::new(),
            config: DisplayConfig::default(),
            current_display: 0,
        };

        manager.add_default_sessions();
        manager
    }

    /// Add default desktop sessions
    fn add_default_sessions(&mut self) {
        let sigma_session = DesktopSession::new(
            "SigmaOS".to_string(),
            "/usr/bin/sigma-session".to_string(),
            SessionType::Wayland,
        );
        let mut sigma_session = sigma_session;
        sigma_session.set_default(true);
        self.sessions.insert("SigmaOS".to_string(), sigma_session);

        let x11_session = DesktopSession::new(
            "SigmaOS (X11)".to_string(),
            "/usr/bin/sigma-session-x11".to_string(),
            SessionType::X11,
        );
        self.sessions.insert("SigmaOS (X11)".to_string(), x11_session);

        let tty_session = DesktopSession::new(
            "TTY".to_string(),
            "/bin/login".to_string(),
            SessionType::Tty,
        );
        self.sessions.insert("TTY".to_string(), tty_session);
    }

    /// Add a desktop session
    pub fn add_session(&mut self, session: DesktopSession) {
        self.sessions.insert(session.name.clone(), session);
    }

    /// Get a session
    pub fn get_session(&self, name: &str) -> Option<&DesktopSession> {
        self.sessions.get(name)
    }

    /// List all sessions
    pub fn list_sessions(&self) -> Vec<&DesktopSession> {
        self.sessions.values().collect()
    }

    /// Get default session
    pub fn get_default_session(&self) -> Option<&DesktopSession> {
        self.sessions.values()
            .find(|s| s.is_default)
            .or_else(|| self.sessions.get(&self.config.default_session))
    }

    /// Set default session
    pub fn set_default_session(&mut self, name: &str) {
        // Remove default flag from all sessions
        for session in self.sessions.values_mut() {
            session.set_default(false);
        }

        // Set default flag on specified session
        if let Some(session) = self.sessions.get_mut(name) {
            session.set_default(true);
            self.config.default_session = name.to_string();
        }
    }

    /// Get configuration
    pub fn get_config(&self) -> &DisplayConfig {
        &self.config
    }

    /// Set configuration
    pub fn set_config(&mut self, config: DisplayConfig) {
        self.config = config;
    }

    /// Enable auto-login
    pub fn enable_auto_login(&mut self, username: String) {
        self.config.auto_login_enabled = true;
        self.config.auto_login_user = Some(username);
    }

    /// Disable auto-login
    pub fn disable_auto_login(&mut self) {
        self.config.auto_login_enabled = false;
        self.config.auto_login_user = None;
    }

    /// Start a user session
    pub fn start_session(&mut self, username: String, session_name: String) -> Result<String, String> {
        if !self.sessions.contains_key(&session_name) {
            return Err(format!("Session {} not found", session_name));
        }

        let display = format!(":{}", self.current_display);
        self.current_display += 1;

        let user_session = UserSession::new(username, session_name, display);
        let display = user_session.display.clone();
        self.active_sessions.push(user_session);

        Ok(display)
    }

    /// Get active sessions
    pub fn get_active_sessions(&self) -> Vec<&UserSession> {
        self.active_sessions.iter().collect()
    }

    /// Get session for user
    pub fn get_user_session(&self, username: &str) -> Option<&UserSession> {
        self.active_sessions.iter()
            .find(|s| s.username == username)
    }

    /// End a session
    pub fn end_session(&mut self, display: &str) -> Result<(), String> {
        let pos = self.active_sessions.iter()
            .position(|s| s.display == display)
            .ok_or_else(|| format!("Session {} not found", display))?;

        self.active_sessions.remove(pos);
        Ok(())
    }

    /// Switch session
    pub fn switch_session(&mut self, display: &str) -> Result<(), String> {
        let session = self.active_sessions.iter()
            .find(|s| s.display == display)
            .ok_or_else(|| format!("Session {} not found", display))?;

        // Deactivate all sessions
        for s in self.active_sessions.iter_mut() {
            s.set_active(false);
        }

        // Activate specified session
        if let Some(active) = self.active_sessions.iter_mut()
            .find(|s| s.display == display) {
            active.set_active(true);
        }

        Ok(())
    }

    /// Get next available display number
    pub fn get_next_display(&self) -> String {
        format!(":{}", self.current_display)
    }

    /// Get statistics
    pub fn get_statistics(&self) -> DisplayStatistics {
        DisplayStatistics {
            total_sessions: self.sessions.len(),
            active_sessions: self.active_sessions.len(),
            default_session: self.config.default_session.clone(),
            auto_login_enabled: self.config.auto_login_enabled,
            current_display: self.current_display,
        }
    }
}

impl Default for DisplayManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Display statistics
#[derive(Debug, Clone)]
pub struct DisplayStatistics {
    pub total_sessions: usize,
    pub active_sessions: usize,
    pub default_session: String,
    pub auto_login_enabled: bool,
    pub current_display: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_type_from_str() {
        assert_eq!(SessionType::from_str("x11"), Some(SessionType::X11));
        assert_eq!(SessionType::from_str("wayland"), Some(SessionType::Wayland));
    }

    #[test]
    fn test_desktop_session_creation() {
        let session = DesktopSession::new(
            "Test".to_string(),
            "/usr/bin/test".to_string(),
            SessionType::Wayland,
        );
        assert_eq!(session.name, "Test");
    }

    #[test]
    fn test_display_manager_creation() {
        let manager = DisplayManager::new();
        assert!(manager.list_sessions().len() >= 3);
    }

    #[test]
    fn test_add_session() {
        let mut manager = DisplayManager::new();
        let session = DesktopSession::new(
            "Custom".to_string(),
            "/usr/bin/custom".to_string(),
            SessionType::X11,
        );
        manager.add_session(session);
        assert!(manager.get_session("Custom").is_some());
    }

    #[test]
    fn test_set_default_session() {
        let mut manager = DisplayManager::new();
        manager.set_default_session("TTY");
        assert_eq!(manager.get_config().default_session, "TTY");
    }

    #[test]
    fn test_auto_login() {
        let mut manager = DisplayManager::new();
        manager.enable_auto_login("user".to_string());
        assert!(manager.get_config().auto_login_enabled);
        assert_eq!(manager.get_config().auto_login_user, Some("user".to_string()));
    }

    #[test]
    fn test_start_session() {
        let mut manager = DisplayManager::new();
        let display = manager.start_session("user".to_string(), "SigmaOS".to_string()).unwrap();
        assert_eq!(display, ":0");
    }

    #[test]
    fn test_end_session() {
        let mut manager = DisplayManager::new();
        let display = manager.start_session("user".to_string(), "SigmaOS".to_string()).unwrap();
        assert!(manager.end_session(&display).is_ok());
    }

    #[test]
    fn test_statistics() {
        let manager = DisplayManager::new();
        let stats = manager.get_statistics();
        assert!(stats.total_sessions >= 3);
    }
}
