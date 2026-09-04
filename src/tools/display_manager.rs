use std::format;
use std::string::{String, ToString};
use std::vec::Vec;
// Display Manager (GDM/LightDM Inspiration)
// Login screen, session management, and display server spawning

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

/// MDM / SDDM Greeter Theme Style Engine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GreeterEngineStyle {
    Html5WebKit, // MDM HTML5 animated greeter
    QmlSddm,     // SDDM QML greeter
    GtkLightDm,  // LightDM GTK greeter
}

/// Linux Mint MDM (Mint Display Manager) Greeter Theme Specification
#[derive(Debug, Clone)]
pub struct MdmGreeterTheme {
    pub name: String,
    pub engine_style: GreeterEngineStyle,
    pub background_image_path: String,
    pub logo_path: String,
    pub show_user_list: bool,
    pub allow_guest: bool,
}

impl MdmGreeterTheme {
    pub fn new(name: &str, engine_style: GreeterEngineStyle) -> Self {
        Self {
            name: name.to_string(),
            engine_style,
            background_image_path: "/usr/share/mdm/themes/default/background.jpg".to_string(),
            logo_path: "/usr/share/mdm/themes/default/logo.png".to_string(),
            show_user_list: true,
            allow_guest: false,
        }
    }
}

/// User Face Avatar & Session Memory Store
#[derive(Debug, Clone)]
pub struct UserSessionMemory {
    pub user_id: u32,
    pub face_icon_path: String,
    pub last_selected_session: String,
}

/// On-Screen Accessibility Keyboard & High-Contrast Greeter Overlay
pub struct GreeterAccessibilityOverlay {
    pub onscreen_keyboard_enabled: bool,
    pub high_contrast_enabled: bool,
    pub screen_reader_enabled: bool,
}

impl GreeterAccessibilityOverlay {
    pub fn new() -> Self {
        Self {
            onscreen_keyboard_enabled: false,
            high_contrast_enabled: false,
            screen_reader_enabled: false,
        }
    }

    pub fn toggle_onscreen_keyboard(&mut self) {
        self.onscreen_keyboard_enabled = !self.onscreen_keyboard_enabled;
    }
}

/// Display manager
pub struct DisplayManager {
    pub sessions: Vec<Session>,
    pub users: Vec<User>,
    pub autologin: Option<u32>,
    pub current_session: Option<String>,
    pub active_greeter_theme: Option<MdmGreeterTheme>,
    pub accessibility_overlay: GreeterAccessibilityOverlay,
}

impl DisplayManager {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
            users: Vec::new(),
            autologin: None,
            current_session: None,
            active_greeter_theme: Some(MdmGreeterTheme::new(
                "Mint-HTML5-Default",
                GreeterEngineStyle::Html5WebKit,
            )),
            accessibility_overlay: GreeterAccessibilityOverlay::new(),
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

impl Default for DisplayManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DMError {
    SessionNotFound,
    UserNotFound,
    StartFailed,
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

    #[test]
    fn test_mdm_greeter_theme() {
        let theme = MdmGreeterTheme::new("Mint-Metallic", GreeterEngineStyle::Html5WebKit);
        assert_eq!(theme.name, "Mint-Metallic");
        assert_eq!(theme.engine_style, GreeterEngineStyle::Html5WebKit);
    }

    #[test]
    fn test_user_session_memory() {
        let memory = UserSessionMemory {
            user_id: 1000,
            face_icon_path: "/var/lib/AccountsService/icons/user.png".to_string(),
            last_selected_session: "Wayland".to_string(),
        };
        assert_eq!(memory.user_id, 1000);
        assert_eq!(memory.last_selected_session, "Wayland");
    }

    #[test]
    fn test_greeter_accessibility_overlay() {
        let mut overlay = GreeterAccessibilityOverlay::new();
        assert!(!overlay.onscreen_keyboard_enabled);
        overlay.toggle_onscreen_keyboard();
        assert!(overlay.onscreen_keyboard_enabled);
    }
}
