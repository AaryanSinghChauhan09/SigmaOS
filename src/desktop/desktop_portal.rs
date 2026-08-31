// SPDX-License-Identifier: MIT
// SigmaOS Desktop Portal Engine (XDG Desktop Portal Parity & Linux/BSD Sandbox Access Framework)
// Inspired by Freedesktop xdg-desktop-portal, Flatpak bubblewrap portals, elementaryOS Contractor, PipeWire screen capture, and FreeBSD Capsicum capability gating

#[cfg(not(target_os = "none"))]
use std::vec::Vec;

#[cfg(target_os = "none")]
extern crate alloc;

#[cfg(target_os = "none")]
use alloc::vec::Vec;

// ============================================================================
// 1. File Dialog Portal (XDG OpenFile / SaveFile & Capability Token Granting)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileDialogMode {
    OpenFile,
    OpenMultipleFiles,
    SaveFile,
    SelectFolder,
}

#[derive(Debug, Clone)]
pub struct FileFilter {
    pub name: &'static str,
    pub pattern: &'static str,
    pub mime_type: &'static str,
}

#[derive(Debug, Clone)]
pub struct FileDialogRequest {
    pub app_id: &'static str,
    pub title: &'static str,
    pub mode: FileDialogMode,
    pub modal: bool,
    pub filters: Vec<FileFilter>,
}

#[derive(Debug, Clone)]
pub struct FileDialogResponse {
    pub success: bool,
    pub selected_paths: Vec<&'static str>,
    pub capability_token: u64,
}

#[derive(Debug)]
pub struct FileDialogPortal {
    requests_processed: u64,
}

impl FileDialogPortal {
    pub fn new() -> Self {
        Self { requests_processed: 0 }
    }

    pub fn show_dialog(&mut self, _request: FileDialogRequest, user_approved_paths: Vec<&'static str>) -> FileDialogResponse {
        self.requests_processed += 1;
        if user_approved_paths.is_empty() {
            return FileDialogResponse {
                success: false,
                selected_paths: Vec::new(),
                capability_token: 0,
            };
        }

        // Generate synthetic capability token granting access to selected files
        let capability_token = 0xCAFEE00000000000u64 | (self.requests_processed & 0xFFFFFFF);
        FileDialogResponse {
            success: true,
            selected_paths: user_approved_paths,
            capability_token,
        }
    }
}

impl Default for FileDialogPortal {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 2. ScreenCast & Screenshot Portal (Wayland / PipeWire Region Capture)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureSourceType {
    Monitor,
    Window,
    Region,
}

#[derive(Debug, Clone)]
pub struct ScreenCastSession {
    pub session_id: u32,
    pub app_id: &'static str,
    pub source_type: CaptureSourceType,
    pub include_cursor: bool,
    pub pipewire_node_id: u32,
    pub is_active: bool,
}

#[derive(Debug)]
pub struct ScreenCastScreenshotPortal {
    next_session_id: u32,
    active_sessions: Vec<ScreenCastSession>,
}

impl ScreenCastScreenshotPortal {
    pub fn new() -> Self {
        Self {
            next_session_id: 1000,
            active_sessions: Vec::new(),
        }
    }

    pub fn create_screencast_session(
        &mut self,
        app_id: &'static str,
        source_type: CaptureSourceType,
        include_cursor: bool,
    ) -> Result<ScreenCastSession, &'static str> {
        if app_id.is_empty() {
            return Err("Invalid App ID");
        }

        let session_id = self.next_session_id;
        self.next_session_id += 1;
        let pipewire_node_id = 42 + session_id;

        let session = ScreenCastSession {
            session_id,
            app_id,
            source_type,
            include_cursor,
            pipewire_node_id,
            is_active: true,
        };

        self.active_sessions.push(session.clone());
        Ok(session)
    }

    pub fn stop_screencast_session(&mut self, session_id: u32) -> bool {
        if let Some(pos) = self.active_sessions.iter().position(|s| s.session_id == session_id) {
            self.active_sessions.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn take_screenshot(&mut self, app_id: &'static str, interactive: bool) -> Result<u32, &'static str> {
        if app_id.is_empty() {
            return Err("Invalid App ID");
        }
        let buffer_handle = 0xBD000000 | if interactive { 0x01 } else { 0x02 };
        Ok(buffer_handle)
    }

    pub fn get_active_sessions_count(&self) -> usize {
        self.active_sessions.len()
    }
}

impl Default for ScreenCastScreenshotPortal {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 3. OpenURI Portal (Protocol Verification & Handlers)
// ============================================================================

#[derive(Debug, Clone)]
pub struct UriHandler {
    pub scheme: &'static str,
    pub default_app: &'static str,
}

#[derive(Debug)]
pub struct OpenUriPortal {
    handlers: Vec<UriHandler>,
}

impl OpenUriPortal {
    pub fn new() -> Self {
        let mut portal = Self { handlers: Vec::new() };
        portal.register_handler(UriHandler { scheme: "http", default_app: "SigmaWeb" });
        portal.register_handler(UriHandler { scheme: "https", default_app: "SigmaWeb" });
        portal.register_handler(UriHandler { scheme: "mailto", default_app: "PantheonMail" });
        portal.register_handler(UriHandler { scheme: "sigma", default_app: "SigmaControl" });
        portal
    }

    pub fn register_handler(&mut self, handler: UriHandler) {
        self.handlers.push(handler);
    }

    pub fn open_uri(&self, _app_id: &str, uri: &str) -> Result<&'static str, &'static str> {
        if uri.is_empty() {
            return Err("Empty URI");
        }

        let scheme = if uri.starts_with("http://") {
            "http"
        } else if uri.starts_with("https://") {
            "https"
        } else if uri.starts_with("mailto:") {
            "mailto"
        } else if uri.starts_with("sigma://") {
            "sigma"
        } else {
            return Err("Unsupported URI scheme");
        };

        if let Some(handler) = self.handlers.iter().find(|h| h.scheme == scheme) {
            Ok(handler.default_app)
        } else {
            Err("No registered handler for URI scheme")
        }
    }
}

impl Default for OpenUriPortal {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 4. Secret Keyring Portal (Freedesktop Secret Service & Sandboxed Storage)
// ============================================================================

#[derive(Debug, Clone)]
pub struct SecretItem {
    pub label: &'static str,
    pub key: &'static str,
    pub value: &'static str,
    pub app_owner: &'static str,
}

#[derive(Debug)]
pub struct SecretKeyringPortal {
    vault: Vec<SecretItem>,
}

impl SecretKeyringPortal {
    pub fn new() -> Self {
        Self { vault: Vec::new() }
    }

    pub fn store_secret(&mut self, app_id: &'static str, item: SecretItem) -> Result<(), &'static str> {
        if app_id != item.app_owner {
            return Err("Security Violation: Cannot store secret for another application");
        }
        // Remove existing item with same key for this app
        self.vault.retain(|i| !(i.app_owner == app_id && i.key == item.key));
        self.vault.push(item);
        Ok(())
    }

    pub fn get_secret(&self, app_id: &str, key: &str) -> Option<&'static str> {
        self.vault
            .iter()
            .find(|item| item.app_owner == app_id && item.key == key)
            .map(|item| item.value)
    }

    pub fn delete_secret(&mut self, app_id: &str, key: &str) -> bool {
        let initial_len = self.vault.len();
        self.vault.retain(|item| !(item.app_owner == app_id && item.key == key));
        self.vault.len() < initial_len
    }
}

impl Default for SecretKeyringPortal {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 5. Inhibit Portal (System Power / Idle / Sleep Inhibitor Manager)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InhibitFlag {
    Logout = 1,
    SwitchUser = 2,
    Suspend = 4,
    Idle = 8,
}

#[derive(Debug, Clone)]
pub struct Inhibitor {
    pub cookie: u32,
    pub app_id: &'static str,
    pub reason: &'static str,
    pub flags: u32,
}

#[derive(Debug)]
pub struct InhibitPortal {
    next_cookie: u32,
    inhibitors: Vec<Inhibitor>,
}

impl InhibitPortal {
    pub fn new() -> Self {
        Self {
            next_cookie: 5000,
            inhibitors: Vec::new(),
        }
    }

    pub fn inhibit(&mut self, app_id: &'static str, reason: &'static str, flags: u32) -> Result<u32, &'static str> {
        if app_id.is_empty() || reason.is_empty() {
            return Err("Invalid Inhibitor parameters");
        }

        let cookie = self.next_cookie;
        self.next_cookie += 1;

        self.inhibitors.push(Inhibitor {
            cookie,
            app_id,
            reason,
            flags,
        });

        Ok(cookie)
    }

    pub fn uninhibit(&mut self, cookie: u32) -> bool {
        if let Some(pos) = self.inhibitors.iter().position(|i| i.cookie == cookie) {
            self.inhibitors.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn is_flag_inhibited(&self, flag: InhibitFlag) -> bool {
        let flag_val = flag as u32;
        self.inhibitors.iter().any(|i| (i.flags & flag_val) != 0)
    }
}

impl Default for InhibitPortal {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 6. Permission Store Portal (Dynamic Capability Policy Store)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionCategory {
    Camera,
    Microphone,
    Location,
    FileSystem,
    Notifications,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionState {
    Granted,
    Denied,
    Prompt,
}

#[derive(Debug, Clone)]
pub struct AppPermissionRule {
    pub app_id: &'static str,
    pub category: PermissionCategory,
    pub state: PermissionState,
}

#[derive(Debug)]
pub struct PermissionStorePortal {
    rules: Vec<AppPermissionRule>,
}

impl PermissionStorePortal {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn set_permission(&mut self, app_id: &'static str, category: PermissionCategory, state: PermissionState) {
        self.rules.retain(|r| !(r.app_id == app_id && r.category == category));
        self.rules.push(AppPermissionRule { app_id, category, state });
    }

    pub fn check_permission(&self, app_id: &str, category: PermissionCategory) -> PermissionState {
        if let Some(rule) = self.rules.iter().find(|r| r.app_id == app_id && r.category == category) {
            rule.state
        } else {
            PermissionState::Prompt
        }
    }
}

impl Default for PermissionStorePortal {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 7. Contractor & AppChooser Portal (elementaryOS & Freedesktop Association)
// ============================================================================

#[derive(Debug, Clone)]
pub struct AppChooserChoice {
    pub app_name: &'static str,
    pub mime_type: &'static str,
    pub exec_cmd: &'static str,
}

#[derive(Debug)]
pub struct ContractorAppChooserPortal {
    choices: Vec<AppChooserChoice>,
}

impl ContractorAppChooserPortal {
    pub fn new() -> Self {
        let mut portal = Self { choices: Vec::new() };

        portal.register_choice(AppChooserChoice {
            app_name: "SigmaText",
            mime_type: "text/plain",
            exec_cmd: "sigmatext %f",
        });

        portal.register_choice(AppChooserChoice {
            app_name: "SigmaCut",
            mime_type: "video/mp4",
            exec_cmd: "sigmacut %f",
        });

        portal
    }

    pub fn register_choice(&mut self, choice: AppChooserChoice) {
        self.choices.push(choice);
    }

    pub fn get_apps_for_mime(&self, mime_type: &str) -> Vec<AppChooserChoice> {
        self.choices
            .iter()
            .filter(|c| c.mime_type == mime_type)
            .cloned()
            .collect()
    }

    pub fn launch_app_for_file(&self, app_name: &str, file_path: &str) -> Result<&'static str, &'static str> {
        if file_path.is_empty() {
            return Err("Target file path is empty");
        }
        if self.choices.iter().any(|c| c.app_name == app_name) {
            Ok("App Chooser contract launched successfully")
        } else {
            Err("Requested app not found in AppChooser portal")
        }
    }
}

impl Default for ContractorAppChooserPortal {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 8. Unified Desktop Portal Application Engine
// ============================================================================

#[derive(Debug)]
pub struct XdgDesktopPortalEngine {
    pub file_dialog: FileDialogPortal,
    pub screencast: ScreenCastScreenshotPortal,
    pub open_uri: OpenUriPortal,
    pub secret_keyring: SecretKeyringPortal,
    pub inhibit: InhibitPortal,
    pub permission_store: PermissionStorePortal,
    pub app_chooser: ContractorAppChooserPortal,
}

impl XdgDesktopPortalEngine {
    pub fn new() -> Self {
        Self {
            file_dialog: FileDialogPortal::new(),
            screencast: ScreenCastScreenshotPortal::new(),
            open_uri: OpenUriPortal::new(),
            secret_keyring: SecretKeyringPortal::new(),
            inhibit: InhibitPortal::new(),
            permission_store: PermissionStorePortal::new(),
            app_chooser: ContractorAppChooserPortal::new(),
        }
    }
}

impl Default for XdgDesktopPortalEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_dialog_portal() {
        let mut portal = FileDialogPortal::new();
        let req = FileDialogRequest {
            app_id: "org.sigmaos.editor",
            title: "Open Document",
            mode: FileDialogMode::OpenFile,
            modal: true,
            filters: Vec::new(),
        };

        let response = portal.show_dialog(req, vec!["/home/user/document.txt"]);
        assert!(response.success);
        assert_eq!(response.selected_paths, vec!["/home/user/document.txt"]);
        assert_ne!(response.capability_token, 0);
    }

    #[test]
    fn test_screencast_screenshot_portal() {
        let mut portal = ScreenCastScreenshotPortal::new();
        let session = portal
            .create_screencast_session("org.sigmaos.obs", CaptureSourceType::Monitor, true)
            .unwrap();
        assert_eq!(session.app_id, "org.sigmaos.obs");
        assert_eq!(portal.get_active_sessions_count(), 1);

        assert!(portal.stop_screencast_session(session.session_id));
        assert_eq!(portal.get_active_sessions_count(), 0);

        let screenshot_handle = portal.take_screenshot("org.sigmaos.editor", true).unwrap();
        assert_ne!(screenshot_handle, 0);
    }

    #[test]
    fn test_open_uri_portal() {
        let portal = OpenUriPortal::new();
        let app = portal.open_uri("org.sigmaos.browser", "https://sigmaos.dev").unwrap();
        assert_eq!(app, "SigmaWeb");

        let mail_app = portal.open_uri("org.sigmaos.browser", "mailto:dev@sigmaos.dev").unwrap();
        assert_eq!(mail_app, "PantheonMail");

        assert!(portal.open_uri("org.sigmaos.browser", "ftp://invalid").is_err());
    }

    #[test]
    fn test_secret_keyring_portal() {
        let mut portal = SecretKeyringPortal::new();
        let item = SecretItem {
            label: "Database Password",
            key: "db_pass",
            value: "SuperSecret123",
            app_owner: "org.sigmaos.db",
        };

        // Fail when app_id doesn't match owner
        assert!(portal.store_secret("org.sigmaos.hacker", item.clone()).is_err());

        // Store successfully
        assert!(portal.store_secret("org.sigmaos.db", item).is_ok());

        assert_eq!(portal.get_secret("org.sigmaos.db", "db_pass"), Some("SuperSecret123"));
        assert_eq!(portal.get_secret("org.sigmaos.other", "db_pass"), None);

        assert!(portal.delete_secret("org.sigmaos.db", "db_pass"));
        assert_eq!(portal.get_secret("org.sigmaos.db", "db_pass"), None);
    }

    #[test]
    fn test_inhibit_portal() {
        let mut portal = InhibitPortal::new();
        let cookie = portal
            .inhibit("org.sigmaos.media", "Playing Movie", InhibitFlag::Idle as u32 | InhibitFlag::Suspend as u32)
            .unwrap();

        assert!(portal.is_flag_inhibited(InhibitFlag::Idle));
        assert!(portal.is_flag_inhibited(InhibitFlag::Suspend));
        assert!(!portal.is_flag_inhibited(InhibitFlag::Logout));

        assert!(portal.uninhibit(cookie));
        assert!(!portal.is_flag_inhibited(InhibitFlag::Idle));
    }

    #[test]
    fn test_permission_store_portal() {
        let mut portal = PermissionStorePortal::new();
        assert_eq!(
            portal.check_permission("org.sigmaos.camera", PermissionCategory::Camera),
            PermissionState::Prompt
        );

        portal.set_permission("org.sigmaos.camera", PermissionCategory::Camera, PermissionState::Granted);
        assert_eq!(
            portal.check_permission("org.sigmaos.camera", PermissionCategory::Camera),
            PermissionState::Granted
        );
    }

    #[test]
    fn test_app_chooser_portal() {
        let portal = ContractorAppChooserPortal::new();
        let apps = portal.get_apps_for_mime("text/plain");
        assert_eq!(apps.len(), 1);
        assert_eq!(apps[0].app_name, "SigmaText");

        assert!(portal.launch_app_for_file("SigmaText", "/tmp/notes.txt").is_ok());
        assert!(portal.launch_app_for_file("UnknownApp", "/tmp/notes.txt").is_err());
    }

    #[test]
    fn test_xdg_desktop_portal_engine() {
        let engine = XdgDesktopPortalEngine::new();
        assert_eq!(engine.screencast.get_active_sessions_count(), 0);
    }
}
