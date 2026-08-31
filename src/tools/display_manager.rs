//! Display Manager (GDM/LightDM & Linux Mint MDM Inspiration)
//! Login screen, session management, display server spawning, and MDM theme engine

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::customization::theme::{
    MdmAccessibilitySettings, MdmPamAuthStage, MdmPowerAction, MdmThemeInfo, MdmUserAvatar,
    SovereignMdmThemeEngine,
};




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
    pub mdm_theme_engine: SovereignMdmThemeEngine,
}

impl DisplayManager {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
            users: Vec::new(),
            autologin: None,
            current_session: None,
            mdm_theme_engine: SovereignMdmThemeEngine::new(),
        }
    }

    pub fn set_mdm_theme(&mut self, theme_name: &str) -> Result<(), &'static str> {
        self.mdm_theme_engine.set_active_theme(theme_name)
    }

    pub fn get_active_mdm_theme(&self) -> Option<&MdmThemeInfo> {
        self.mdm_theme_engine.get_active_theme()
    }

    pub fn discover_user_avatar(
        &mut self,
        username: &str,
        real_name: &str,
        face_path: &str,
    ) -> &MdmUserAvatar {
        self.mdm_theme_engine
            .discover_user_avatar(username, real_name, face_path)
    }

    pub fn authenticate_user_pam(
        &mut self,
        username: &str,
        credential: &str,
        pam_type: &str,
    ) -> MdmPamAuthStage {
        self.mdm_theme_engine
            .authenticate_pam(username, credential, pam_type)
    }

    pub fn render_greeter_canvas_frame(&mut self, now_ms: u64) -> Vec<(f32, f32, f32)> {
        self.mdm_theme_engine.render_html5_canvas_frame(now_ms)
    }

    pub fn evaluate_monitor_layout(
        &self,
        monitors_count: u32,
        active_monitor: u32,
    ) -> (u32, u32, f32) {
        self.mdm_theme_engine
            .evaluate_monitor_layout(monitors_count, active_monitor)
    }

    pub fn dispatch_power_action(
        &self,
        action: MdmPowerAction,
    ) -> Result<&'static str, &'static str> {
        self.mdm_theme_engine.dispatch_power_action(action)
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

    #[test]
    fn test_display_manager_mdm_integration() {
        let mut dm = DisplayManager::new();

        // Check active MDM theme
        let theme = dm.get_active_mdm_theme().unwrap();
        assert_eq!(theme.name, "Mint-Webkit-Sovereign");

        // Change active theme
        assert!(dm.set_mdm_theme("Adwaita-MDM").is_ok());
        assert_eq!(dm.get_active_mdm_theme().unwrap().name, "Adwaita-MDM");

        // User avatar discovery
        let avatar = dm.discover_user_avatar("alice", "Alice Smith", "/home/alice/.face");
        assert_eq!(avatar.username, "alice");

        // PAM authentication
        let auth = dm.authenticate_user_pam("alice", "correct_pass", "password");
        assert_eq!(
            auth,
            MdmPamAuthStage::Authenticated {
                username: "alice".to_string()
            }
        );

        // Canvas frame rendering
        let frame = dm.render_greeter_canvas_frame(100);
        assert_eq!(frame.len(), 16);

        // Monitor alignment evaluation
        let (mon, scale_pct, scale_f) = dm.evaluate_monitor_layout(2, 0);
        assert_eq!(mon, 0);
        assert_eq!(scale_pct, 100);
        assert_eq!(scale_f, 1.0);

        // Power action dispatch
        let power_res = dm.dispatch_power_action(MdmPowerAction::Reboot);
        assert!(power_res.is_ok());
        assert!(power_res.unwrap().contains("reboot"));
    }
}