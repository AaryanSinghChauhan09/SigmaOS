use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum SessionType {
    Wayland,
    X11,
    TTY,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DesktopEnvironment {
    Zenith,
    Cinnamon,
    Gnome,
    Kde,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoginSession {
    pub session_type: SessionType,
    pub desktop: DesktopEnvironment,
}

#[derive(Debug, Clone)]
pub struct GreeterTheme {
    pub wallpaper: String,
    pub logo: String,
    pub panel_color: String,
    pub font: String,
}

#[derive(Debug, Clone)]
pub struct AccessibilityOptions {
    pub high_contrast: bool,
    pub screen_reader: bool,
    pub on_screen_keyboard: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AuthenticationResult {
    Success { session_token: String },
    Failure(String),
}

pub struct UserProfile {
    pub username: String,
    pub avatar_path: Option<String>,
    pub is_guest: bool,
    pub auto_login: bool,
}

pub struct DisplayManager {
    users: HashMap<String, UserProfile>,
    theme: GreeterTheme,
    accessibility: AccessibilityOptions,
    session_history: HashMap<String, LoginSession>,
    multi_monitor_layout: Vec<MonitorConfig>,
}

pub struct MonitorConfig {
    pub id: u32,
    pub resolution: (u32, u32),
    pub is_primary: bool,
}

impl DisplayManager {
    pub fn new(theme: GreeterTheme) -> Self {
        Self {
            users: HashMap::new(),
            theme,
            accessibility: AccessibilityOptions {
                high_contrast: false,
                screen_reader: false,
                on_screen_keyboard: false,
            },
            session_history: HashMap::new(),
            multi_monitor_layout: Vec::new(),
        }
    }

    pub fn add_user(&mut self, profile: UserProfile) {
        self.users.insert(profile.username.clone(), profile);
    }

    pub fn toggle_accessibility(&mut self, high_contrast: Option<bool>, screen_reader: Option<bool>, on_screen_keyboard: Option<bool>) {
        if let Some(hc) = high_contrast { self.accessibility.high_contrast = hc; }
        if let Some(sr) = screen_reader { self.accessibility.screen_reader = sr; }
        if let Some(osk) = on_screen_keyboard { self.accessibility.on_screen_keyboard = osk; }
    }

    pub fn set_monitors(&mut self, monitors: Vec<MonitorConfig>) {
        self.multi_monitor_layout = monitors;
    }

    pub fn authenticate(&mut self, username: &str, password_hash: &str) -> AuthenticationResult {
        if let Some(user) = self.users.get(username) {
            if user.is_guest || password_hash == "valid_hash" {
                AuthenticationResult::Success { session_token: format!("token_{}", username) }
            } else {
                AuthenticationResult::Failure("Invalid credentials".to_string())
            }
        } else {
            AuthenticationResult::Failure("User not found".to_string())
        }
    }

    pub fn start_session(&mut self, username: &str, session: LoginSession) -> Result<String, String> {
        if !self.users.contains_key(username) {
            return Err("Unknown user".to_string());
        }
        self.session_history.insert(username.to_string(), session.clone());
        Ok(format!("Started {:?} session for {}", session.desktop, username))
    }
    
    pub fn get_last_session(&self, username: &str) -> Option<&LoginSession> {
        self.session_history.get(username)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_theme() -> GreeterTheme {
        GreeterTheme {
            wallpaper: "/usr/share/backgrounds/default.jpg".to_string(),
            logo: "/usr/share/icons/logo.png".to_string(),
            panel_color: "#000000".to_string(),
            font: "Ubuntu 11".to_string(),
        }
    }

    #[test]
    fn test_add_user() {
        let mut dm = DisplayManager::new(default_theme());
        dm.add_user(UserProfile {
            username: "alice".to_string(),
            avatar_path: Some("/home/alice/.face".to_string()),
            is_guest: false,
            auto_login: false,
        });
        assert!(dm.users.contains_key("alice"));
    }

    #[test]
    fn test_guest_auth() {
        let mut dm = DisplayManager::new(default_theme());
        dm.add_user(UserProfile {
            username: "guest".to_string(),
            avatar_path: None,
            is_guest: true,
            auto_login: false,
        });
        let auth = dm.authenticate("guest", "");
        assert!(matches!(auth, AuthenticationResult::Success { .. }));
    }

    #[test]
    fn test_invalid_auth() {
        let mut dm = DisplayManager::new(default_theme());
        dm.add_user(UserProfile {
            username: "alice".to_string(),
            avatar_path: None,
            is_guest: false,
            auto_login: false,
        });
        let auth = dm.authenticate("alice", "wrong_hash");
        assert!(matches!(auth, AuthenticationResult::Failure(_)));
    }

    #[test]
    fn test_session_history() {
        let mut dm = DisplayManager::new(default_theme());
        dm.add_user(UserProfile {
            username: "alice".to_string(),
            avatar_path: None,
            is_guest: false,
            auto_login: false,
        });
        let session = LoginSession {
            session_type: SessionType::Wayland,
            desktop: DesktopEnvironment::Zenith,
        };
        dm.start_session("alice", session.clone()).unwrap();
        assert_eq!(dm.get_last_session("alice"), Some(&session));
    }

    #[test]
    fn test_accessibility_toggle() {
        let mut dm = DisplayManager::new(default_theme());
        dm.toggle_accessibility(Some(true), None, Some(true));
        assert!(dm.accessibility.high_contrast);
        assert!(!dm.accessibility.screen_reader);
        assert!(dm.accessibility.on_screen_keyboard);
    }

    #[test]
    fn test_monitor_layout() {
        let mut dm = DisplayManager::new(default_theme());
        dm.set_monitors(vec![
            MonitorConfig { id: 1, resolution: (1920, 1080), is_primary: true },
            MonitorConfig { id: 2, resolution: (1920, 1080), is_primary: false },
        ]);
        assert_eq!(dm.multi_monitor_layout.len(), 2);
        assert!(dm.multi_monitor_layout[0].is_primary);
    }
}
