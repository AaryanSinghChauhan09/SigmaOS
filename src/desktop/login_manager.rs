// Desktop Display Manager
// Linux Mint & Omarchy inspiration for comprehensive display (login) manager

use std::collections::HashMap;

/// Login Session Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginSessionType {
    X11,
    Wayland,
    TTY,
}

impl LoginSessionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            LoginSessionType::X11 => "x11",
            LoginSessionType::Wayland => "wayland",
            LoginSessionType::TTY => "tty",
        }
    }
}

/// Login User Session
#[derive(Debug, Clone)]
pub struct LoginUserSession {
    pub id: String,
    pub username: String,
    pub display: String,
    pub session_type: LoginSessionType,
    pub seat: String,
    pub active: bool,
}

impl LoginUserSession {
    pub fn new(username: String, display: String, session_type: LoginSessionType) -> Self {
        Self {
            id: String::new(),
            username,
            display,
            session_type,
            seat: "seat0".to_string(),
            active: false,
        }
    }

    pub fn with_id(mut self, id: String) -> Self {
        self.id = id;
        self
    }

    pub fn with_seat(mut self, seat: String) -> Self {
        self.seat = seat;
        self
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

/// Login Desktop Environment
#[derive(Debug, Clone)]
pub struct LoginDesktopEnvironment {
    pub id: String,
    pub name: String,
    pub command: String,
    pub session_type: LoginSessionType,
    pub installed: bool,
}

impl LoginDesktopEnvironment {
    pub fn new(id: String, name: String, command: String, session_type: LoginSessionType) -> Self {
        Self {
            id,
            name,
            command,
            session_type,
            installed: false,
        }
    }

    pub fn set_installed(&mut self, installed: bool) {
        self.installed = installed;
    }
}

/// Desktop Login Manager
pub struct DesktopLoginManager {
    sessions: HashMap<String, LoginUserSession>,
    desktop_environments: HashMap<String, LoginDesktopEnvironment>,
    active_session: Option<String>,
    autologin_user: Option<String>,
    autologin_enabled: bool,
    guest_session_enabled: bool,
    counter: u32,
}

impl DesktopLoginManager {
    pub fn new() -> Self {
        let mut manager = Self {
            sessions: HashMap::new(),
            desktop_environments: HashMap::new(),
            active_session: None,
            autologin_user: None,
            autologin_enabled: false,
            guest_session_enabled: true,
            counter: 1000,
        };

        // Add default desktop environments
        manager.add_default_desktop_environments();

        manager
    }

    fn add_default_desktop_environments(&mut self) {
        let mut zenith = LoginDesktopEnvironment::new(
            "de_0".to_string(),
            "Zenith".to_string(),
            "/usr/bin/zenith-session".to_string(),
            LoginSessionType::Wayland,
        );
        zenith.set_installed(true);

        let gnome = LoginDesktopEnvironment::new(
            "de_1".to_string(),
            "GNOME".to_string(),
            "/usr/bin/gnome-session".to_string(),
            LoginSessionType::Wayland,
        );

        let kde = LoginDesktopEnvironment::new(
            "de_2".to_string(),
            "KDE Plasma".to_string(),
            "/usr/bin/startplasma-wayland".to_string(),
            LoginSessionType::Wayland,
        );

        let xfce = LoginDesktopEnvironment::new(
            "de_3".to_string(),
            "XFCE".to_string(),
            "/usr/bin/startxfce4".to_string(),
            LoginSessionType::X11,
        );

        let cinnamon = LoginDesktopEnvironment::new(
            "de_4".to_string(),
            "Cinnamon".to_string(),
            "/usr/bin/cinnamon-session".to_string(),
            LoginSessionType::X11,
        );

        let mate = LoginDesktopEnvironment::new(
            "de_5".to_string(),
            "MATE".to_string(),
            "/usr/bin/mate-session".to_string(),
            LoginSessionType::X11,
        );

        self.desktop_environments.insert(zenith.id.clone(), zenith);
        self.desktop_environments.insert(gnome.id.clone(), gnome);
        self.desktop_environments.insert(kde.id.clone(), kde);
        self.desktop_environments.insert(xfce.id.clone(), xfce);
        self.desktop_environments.insert(cinnamon.id.clone(), cinnamon);
        self.desktop_environments.insert(mate.id.clone(), mate);
    }

    pub fn add_session(&mut self, session: LoginUserSession) -> String {
        let id = format!("session_{}", self.counter);
        self.counter += 1;

        let session = LoginUserSession {
            id: id.clone(),
            ..session
        };

        self.sessions.insert(id.clone(), session);
        id
    }

    pub fn remove_session(&mut self, id: &str) -> bool {
        if Some(id.to_string()) == self.active_session {
            self.active_session = None;
        }
        self.sessions.remove(id).is_some()
    }

    pub fn get_session(&self, id: &str) -> Option<&LoginUserSession> {
        self.sessions.get(id)
    }

    pub fn get_sessions(&self) -> Vec<&LoginUserSession> {
        self.sessions.values().collect()
    }

    pub fn get_sessions_by_user(&self, username: &str) -> Vec<&LoginUserSession> {
        self.sessions
            .values()
            .filter(|s| s.username == username)
            .collect()
    }

    pub fn set_active_session(&mut self, id: &str) -> bool {
        if self.sessions.contains_key(id) {
            // Deactivate previous active session
            if let Some(prev_id) = &self.active_session {
                if let Some(session) = self.sessions.get_mut(prev_id) {
                    session.set_active(false);
                }
            }

            // Activate new session
            if let Some(session) = self.sessions.get_mut(id) {
                session.set_active(true);
            }

            self.active_session = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_active_session(&self) -> Option<&LoginUserSession> {
        self.active_session
            .as_ref()
            .and_then(|id| self.sessions.get(id))
    }

    pub fn add_desktop_environment(&mut self, de: LoginDesktopEnvironment) -> String {
        let id = format!("de_{}", self.counter);
        self.counter += 1;

        let de = LoginDesktopEnvironment {
            id: id.clone(),
            ..de
        };

        self.desktop_environments.insert(id.clone(), de);
        id
    }

    pub fn remove_desktop_environment(&mut self, id: &str) -> bool {
        self.desktop_environments.remove(id).is_some()
    }

    pub fn get_desktop_environment(&self, id: &str) -> Option<&LoginDesktopEnvironment> {
        self.desktop_environments.get(id)
    }

    pub fn get_desktop_environments(&self) -> Vec<&LoginDesktopEnvironment> {
        self.desktop_environments.values().collect()
    }

    pub fn get_installed_desktop_environments(&self) -> Vec<&LoginDesktopEnvironment> {
        self.desktop_environments
            .values()
            .filter(|de| de.installed)
            .collect()
    }

    pub fn get_desktop_environments_by_type(&self, session_type: LoginSessionType) -> Vec<&LoginDesktopEnvironment> {
        self.desktop_environments
            .values()
            .filter(|de| de.session_type == session_type)
            .collect()
    }

    pub fn set_desktop_environment_installed(&mut self, id: &str, installed: bool) -> bool {
        if let Some(de) = self.desktop_environments.get_mut(id) {
            de.set_installed(installed);
            true
        } else {
            false
        }
    }

    pub fn set_autologin_user(&mut self, username: Option<String>) {
        self.autologin_user = username;
    }

    pub fn get_autologin_user(&self) -> Option<&String> {
        self.autologin_user.as_ref()
    }

    pub fn set_autologin_enabled(&mut self, enabled: bool) {
        self.autologin_enabled = enabled;
    }

    pub fn is_autologin_enabled(&self) -> bool {
        self.autologin_enabled
    }

    pub fn set_guest_session_enabled(&mut self, enabled: bool) {
        self.guest_session_enabled = enabled;
    }

    pub fn is_guest_session_enabled(&self) -> bool {
        self.guest_session_enabled
    }

    pub fn start_session(&mut self, username: &str, display: &str, de_id: &str) -> Option<String> {
        if let Some(de) = self.desktop_environments.get(de_id) {
            if !de.installed {
                return None;
            }

            let session_type = de.session_type;

            let session = LoginUserSession::new(
                username.to_string(),
                display.to_string(),
                session_type,
            );

            let id = self.add_session(session);
            self.set_active_session(&id);
            Some(id)
        } else {
            None
        }
    }

    pub fn stop_session(&mut self, id: &str) -> bool {
        self.remove_session(id)
    }

    pub fn get_statistics(&self) -> LoginManagerStatistics {
        LoginManagerStatistics {
            total_sessions: self.sessions.len(),
            active_sessions: self.sessions.values().filter(|s| s.active).count(),
            total_desktop_environments: self.desktop_environments.len(),
            installed_desktop_environments: self.get_installed_desktop_environments().len(),
            autologin_enabled: self.autologin_enabled,
            guest_session_enabled: self.guest_session_enabled,
        }
    }
}

impl Default for DesktopLoginManager {
    fn default() -> Self {
        Self::new()
    }
}

/// LoginManagerStatistics
#[derive(Debug, Clone, Copy)]
pub struct LoginManagerStatistics {
    pub total_sessions: usize,
    pub active_sessions: usize,
    pub total_desktop_environments: usize,
    pub installed_desktop_environments: usize,
    pub autologin_enabled: bool,
    pub guest_session_enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_manager_state() {
        let manager = DesktopLoginManager::new();
        let stats = manager.get_statistics();

        assert!(stats.total_desktop_environments >= 6);
        assert!(stats.installed_desktop_environments >= 1);
        assert!(stats.guest_session_enabled);
        assert!(!stats.autologin_enabled);
    }

    #[test]
    fn test_add_session() {
        let mut manager = DesktopLoginManager::new();
        let initial_count = manager.get_sessions().len();

        let session = LoginUserSession::new(
            "user".to_string(),
            ":0".to_string(),
            LoginSessionType::Wayland,
        );

        let id = manager.add_session(session);
        assert!(manager.get_session(&id).is_some());
        assert_eq!(manager.get_sessions().len(), initial_count + 1);
    }

    #[test]
    fn test_remove_session() {
        let mut manager = DesktopLoginManager::new();

        let session = LoginUserSession::new(
            "user".to_string(),
            ":0".to_string(),
            LoginSessionType::Wayland,
        );

        let id = manager.add_session(session);
        assert!(manager.remove_session(&id));
        assert!(manager.get_session(&id).is_none());
    }

    #[test]
    fn test_set_active_session() {
        let mut manager = DesktopLoginManager::new();

        let session = LoginUserSession::new(
            "user".to_string(),
            ":0".to_string(),
            LoginSessionType::Wayland,
        );

        let id = manager.add_session(session);
        assert!(manager.set_active_session(&id));

        let active = manager.get_active_session().unwrap();
        assert_eq!(active.id, id);
        assert!(active.active);
    }

    #[test]
    fn test_start_session() {
        let mut manager = DesktopLoginManager::new();

        let id = manager.start_session("user", ":0", "de_0");
        assert!(id.is_some());

        let session_id = id.unwrap();
        let session = manager.get_session(&session_id).unwrap();
        assert_eq!(session.username, "user");
        assert_eq!(session.display, ":0");
    }

    #[test]
    fn test_start_session_uninstalled_de() {
        let mut manager = DesktopLoginManager::new();

        // Try to start session with uninstalled DE
        let id = manager.start_session("user", ":0", "de_1");
        assert!(id.is_none());
    }

    #[test]
    fn test_stop_session() {
        let mut manager = DesktopLoginManager::new();

        let id = manager.start_session("user", ":0", "de_0");
        assert!(id.is_some());

        let session_id = id.unwrap();
        assert!(manager.stop_session(&session_id));
        assert!(manager.get_session(&session_id).is_none());
    }

    #[test]
    fn test_add_desktop_environment() {
        let mut manager = DesktopLoginManager::new();
        let initial_count = manager.get_desktop_environments().len();

        let de = LoginDesktopEnvironment::new(
            "custom".to_string(),
            "Custom DE".to_string(),
            "/usr/bin/custom-session".to_string(),
            LoginSessionType::Wayland,
        );

        let id = manager.add_desktop_environment(de);
        assert!(manager.get_desktop_environment(&id).is_some());
        assert_eq!(manager.get_desktop_environments().len(), initial_count + 1);
    }

    #[test]
    fn test_set_desktop_environment_installed() {
        let mut manager = DesktopLoginManager::new();

        assert!(manager.set_desktop_environment_installed("de_1", true));
        let de = manager.get_desktop_environment("de_1").unwrap();
        assert!(de.installed);
    }

    #[test]
    fn test_get_desktop_environments_by_type() {
        let manager = DesktopLoginManager::new();

        let wayland_des = manager.get_desktop_environments_by_type(LoginSessionType::Wayland);
        let x11_des = manager.get_desktop_environments_by_type(LoginSessionType::X11);

        assert!(!wayland_des.is_empty());
        assert!(!x11_des.is_empty());

        for de in wayland_des {
            assert_eq!(de.session_type, LoginSessionType::Wayland);
        }

        for de in x11_des {
            assert_eq!(de.session_type, LoginSessionType::X11);
        }
    }

    #[test]
    fn test_autologin() {
        let mut manager = DesktopLoginManager::new();

        manager.set_autologin_user(Some("user".to_string()));
        manager.set_autologin_enabled(true);

        assert_eq!(manager.get_autologin_user(), Some(&"user".to_string()));
        assert!(manager.is_autologin_enabled());
    }

    #[test]
    fn test_guest_session() {
        let mut manager = DesktopLoginManager::new();

        manager.set_guest_session_enabled(false);
        assert!(!manager.is_guest_session_enabled());

        manager.set_guest_session_enabled(true);
        assert!(manager.is_guest_session_enabled());
    }

    #[test]
    fn test_get_sessions_by_user() {
        let mut manager = DesktopLoginManager::new();

        manager.add_session(LoginUserSession::new("user1".to_string(), ":0".to_string(), LoginSessionType::Wayland));
        manager.add_session(LoginUserSession::new("user1".to_string(), ":1".to_string(), LoginSessionType::Wayland));
        manager.add_session(LoginUserSession::new("user2".to_string(), ":2".to_string(), LoginSessionType::Wayland));

        let user1_sessions = manager.get_sessions_by_user("user1");
        assert_eq!(user1_sessions.len(), 2);

        let user2_sessions = manager.get_sessions_by_user("user2");
        assert_eq!(user2_sessions.len(), 1);
    }
}
