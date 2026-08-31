//! Display Manager (GDM/LightDM Inspiration)
//! Login screen, session management, and display server spawning

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;




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

// ==========================================================
// Linux Mint MDM & SDDM/LightDM Inspired Display Manager Theme Engine
// ==========================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MdmThemeStyle {
    Html5Canvas,   // Linux Mint MDM HTML5/CSS3 WebKit animated greeter themes
    GtkWebkit,    // LightDM GTK/Webkit HTML greeter themes
    SddmQml,      // KDE SDDM QML animated greeter themes
    LightdmGtk,   // LightDM GTK3 greeter themes
    MinimalConsole, // Arch/Gentoo TTY console greeter
}

#[derive(Debug, Clone)]
pub struct MdmThemeConfig {
    pub theme_name: String,
    pub style: MdmThemeStyle,
    pub background_wallpaper: String,
    pub enable_background_blur: bool,
    pub blur_radius_px: u32,
    pub show_clock_widget: bool,
    pub custom_css: String,
    pub accent_color_hex: String,
    pub custom_logo_path: String,
}

impl MdmThemeConfig {
    pub fn new(name: &str, style: MdmThemeStyle) -> Self {
        MdmThemeConfig {
            theme_name: name.to_string(),
            style,
            background_wallpaper: "/usr/share/backgrounds/sigmaos-default.png".to_string(),
            enable_background_blur: true,
            blur_radius_px: 20,
            show_clock_widget: true,
            custom_css: String::from("body { font-family: 'Liberation Sans', sans-serif; }"),
            accent_color_hex: String::from("#3584E4"),
            custom_logo_path: "/usr/share/pixmaps/sigmaos-logo.svg".to_string(),
        }
    }
}

pub struct MdmGreeterThemeEngine {
    pub active_config: MdmThemeConfig,
    pub user_avatars: Vec<(u32, String)>, // user_id -> avatar_image_path
    pub selected_language: String,
    pub high_contrast_mode: bool,
    pub autologin_countdown_secs: Option<u32>,
}

impl MdmGreeterThemeEngine {
    pub fn new(config: MdmThemeConfig) -> Self {
        MdmGreeterThemeEngine {
            active_config: config,
            user_avatars: Vec::new(),
            selected_language: "en_US.UTF-8".to_string(),
            high_contrast_mode: false,
            autologin_countdown_secs: None,
        }
    }

    pub fn set_user_avatar(&mut self, user_id: u32, avatar_path: &str) {
        if let Some(pos) = self.user_avatars.iter().position(|(u, _)| *u == user_id) {
            self.user_avatars[pos].1 = avatar_path.to_string();
        } else {
            self.user_avatars.push((user_id, avatar_path.to_string()));
        }
    }

    pub fn get_user_avatar(&self, user_id: u32) -> String {
        self.user_avatars
            .iter()
            .find(|(u, _)| *u == user_id)
            .map(|(_, path)| path.clone())
            .unwrap_or_else(|| "/usr/share/pixmaps/faces/default.png".to_string())
    }

    pub fn enable_high_contrast(&mut self, enable: bool) {
        self.high_contrast_mode = enable;
        if enable {
            self.active_config.accent_color_hex = String::from("#FFFF00"); // High contrast yellow
            self.active_config.enable_background_blur = false;
        }
    }

    pub fn render_html5_greeter_markup(&self, users: &[User], sessions: &[Session]) -> String {
        let mut html = format!(
            "<!DOCTYPE html><html><head><title>MDM Greeter - {}</title>",
            self.active_config.theme_name
        );
        html.push_str(&format!(
            "<style>{} .wallpaper {{ background-image: url('{}'); filter: blur({}px); }}</style></head><body>",
            self.active_config.custom_css,
            self.active_config.background_wallpaper,
            if self.active_config.enable_background_blur { self.active_config.blur_radius_px } else { 0 }
        ));

        html.push_str("<div id='login-card'>");
        html.push_str(&format!("<img id='logo' src='{}' />", self.active_config.custom_logo_path));

        if self.active_config.show_clock_widget {
            html.push_str("<div id='clock-widget'>00:00</div>");
        }

        html.push_str("<select id='user-selector'>");
        for user in users {
            let avatar = self.get_user_avatar(user.id);
            html.push_str(&format!(
                "<option value='{}' data-avatar='{}'>{}</option>",
                user.id, avatar, user.name
            ));
        }
        html.push_str("</select>");

        html.push_str("<select id='session-selector'>");
        for session in sessions {
            html.push_str(&format!(
                "<option value='{}'>{} ({:?})</option>",
                session.name, session.name, session.session_type
            ));
        }
        html.push_str("</select>");

        html.push_str("<input type='password' id='password-input' placeholder='Enter Password...' />");
        html.push_str("<button id='login-btn'>Login</button>");
        html.push_str("</div></body></html>");

        html
    }
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

    #[test]
    fn test_mdm_greeter_theme_engine() {
        let config = MdmThemeConfig::new("Mint-Maya", MdmThemeStyle::Html5Canvas);
        let mut greeter = MdmGreeterThemeEngine::new(config);

        greeter.set_user_avatar(1000, "/home/alice/.face");
        assert_eq!(greeter.get_user_avatar(1000), "/home/alice/.face");
        assert_eq!(greeter.get_user_avatar(1001), "/usr/share/pixmaps/faces/default.png");

        let users = vec![User::new(1000, "alice", "/home/alice")];
        let sessions = vec![Session::new("SigmaOS", SessionType::Wayland, "/usr/bin/sigmaos-wm")];

        let html = greeter.render_html5_greeter_markup(&users, &sessions);
        assert!(html.contains("Mint-Maya"));
        assert!(html.contains("alice"));
        assert!(html.contains("SigmaOS"));

        greeter.enable_high_contrast(true);
        assert!(greeter.high_contrast_mode);
        assert_eq!(greeter.active_config.accent_color_hex, "#FFFF00");
    }
}