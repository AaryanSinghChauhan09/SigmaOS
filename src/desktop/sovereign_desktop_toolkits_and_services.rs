// Sovereign Desktop Services & Toolkits Engine
// Inspired by Linux and BSD Distributions: GTK, Qt, Display Managers (GDM, SDDM, LightDM, XDM),
// GVfs, GIO, PolicyKit, XDG Utilities & Portals, fontconfig, FreeType, HarfBuzz, Pango, Cairo,
// Skia, IBus, Fcitx5, Notification Daemons, Screen Lockers, Clipboard Managers, and PR Gateway.

use core::sync::atomic::{AtomicUsize, Ordering};
use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

static NEXT_DESKTOP_SERVICE_ID: AtomicUsize = AtomicUsize::new(5000);

// ============================================================================
// 1. GTK & Qt Toolkit Synchronization Engine
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdwColorScheme {
    Default = 0,
    ForceLight = 1,
    PreferDark = 2,
    ForceDark = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QtPlatformStyle {
    Breeze = 0,
    Fusion = 1,
    AdwaitaQt = 2,
    Oxygen = 3,
    Cleanlooks = 4,
    Catppuccin = 5,
    Yaru = 6,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolkitEvent {
    ThemeChanged { dark_mode: bool, theme_name: String },
    ColorSchemeChanged(AdwColorScheme),
    FontConfigUpdated { family: String, size_pt: u32 },
    CssReloaded { rules_count: usize },
}

pub struct SovereignGtkQtToolkitEngine {
    pub color_scheme: AdwColorScheme,
    pub qt_style: QtPlatformStyle,
    pub gtk_theme_name: String,
    pub icon_theme_name: String,
    pub cursor_theme_name: String,
    pub font_name: String,
    pub font_size: u32,
    pub accent_color_hex: String,
    pub css_rules: BTreeMap<String, BTreeMap<String, String>>,
    pub signal_listeners: BTreeMap<String, Vec<String>>,
    pub event_trace_log: Vec<String>,
}

impl SovereignGtkQtToolkitEngine {
    pub fn new() -> Self {
        let mut css_rules = BTreeMap::new();
        let mut window_rules = BTreeMap::new();
        window_rules.insert("background-color".to_string(), "#1e1e2e".to_string());
        window_rules.insert("color".to_string(), "#cdd6f4".to_string());
        window_rules.insert("border-radius".to_string(), "12px".to_string());

        let mut row_rules = BTreeMap::new();
        row_rules.insert("padding".to_string(), "10px 14px".to_string());
        row_rules.insert("background-color".to_string(), "#313244".to_string());

        css_rules.insert("window.main".to_string(), window_rules);
        css_rules.insert(".adw-action-row".to_string(), row_rules);

        Self {
            color_scheme: AdwColorScheme::PreferDark,
            qt_style: QtPlatformStyle::Breeze,
            gtk_theme_name: "Adwaita-dark".to_string(),
            icon_theme_name: "Papirus-Dark".to_string(),
            cursor_theme_name: "Adwaita".to_string(),
            font_name: "Cantarell".to_string(),
            font_size: 11,
            accent_color_hex: "#89b4fa".to_string(),
            css_rules,
            signal_listeners: BTreeMap::new(),
            event_trace_log: Vec::new(),
        }
    }

    pub fn set_color_scheme(&mut self, scheme: AdwColorScheme) {
        self.color_scheme = scheme;
        match scheme {
            AdwColorScheme::PreferDark | AdwColorScheme::ForceDark => {
                self.gtk_theme_name = "Adwaita-dark".to_string();
                self.qt_style = QtPlatformStyle::Breeze;
            }
            AdwColorScheme::ForceLight | AdwColorScheme::Default => {
                self.gtk_theme_name = "Adwaita".to_string();
                self.qt_style = QtPlatformStyle::Fusion;
            }
        }
        self.trace_event(&format!("SetColorScheme: {:?}", scheme));
    }

    pub fn apply_css_stylesheet(&mut self, selector: &str, property: &str, value: &str) {
        self.css_rules
            .entry(selector.to_string())
            .or_insert_with(BTreeMap::new)
            .insert(property.to_string(), value.to_string());
        self.trace_event(&format!("CSS Injection: '{}' {{ {}: {} }}", selector, property, value));
    }

    pub fn set_accent_color(&mut self, hex: &str) {
        self.accent_color_hex = hex.to_string();
        self.apply_css_stylesheet("@define-color", "accent_bg_color", hex);
    }

    pub fn connect_signal(&mut self, signal: &str, handler_id: &str) {
        self.signal_listeners
            .entry(signal.to_string())
            .or_insert_with(Vec::new)
            .push(handler_id.to_string());
    }

    pub fn emit_signal(&self, signal: &str) -> usize {
        self.signal_listeners.get(signal).map_or(0, |list| list.len())
    }

    pub fn trace_event(&mut self, event: &str) {
        self.event_trace_log.push(event.to_string());
    }

    pub fn sync_qt_to_gtk_palette(&self) -> BTreeMap<String, String> {
        let mut palette = BTreeMap::new();
        let bg = self
            .css_rules
            .get("window.main")
            .and_then(|r| r.get("background-color"))
            .cloned()
            .unwrap_or_else(|| "#1e1e2e".to_string());
        let fg = self
            .css_rules
            .get("window.main")
            .and_then(|r| r.get("color"))
            .cloned()
            .unwrap_or_else(|| "#cdd6f4".to_string());

        palette.insert("QPalette::Window".to_string(), bg);
        palette.insert("QPalette::WindowText".to_string(), fg.clone());
        palette.insert("QPalette::Base".to_string(), "#181825".to_string());
        palette.insert("QPalette::Text".to_string(), fg.clone());
        palette.insert("QPalette::Highlight".to_string(), self.accent_color_hex.clone());
        palette.insert("QPalette::HighlightedText".to_string(), "#11111b".to_string());
        palette.insert("QPalette::Button".to_string(), "#313244".to_string());
        palette.insert("QPalette::ButtonText".to_string(), fg);
        palette
    }
}

impl Default for SovereignGtkQtToolkitEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 2. Display Manager Suite Engine (GDM, SDDM, LightDM, XDM)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayManagerBackend {
    Gdm,
    Sddm,
    LightDm,
    Xdm,
    SlickGreeter,
    Greetd,
    LyConsole,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopSessionType {
    Wayland,
    X11,
    Mir,
    Hyprland,
    Zenith,
    Sway,
    KdePlasma,
    GnomeShell,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PamAuthRequest {
    pub username: String,
    pub password_hash: String,
    pub session_type: DesktopSessionType,
    pub seat_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PamAuthResult {
    pub authenticated: bool,
    pub user_uid: u32,
    pub home_dir: String,
    pub session_env: BTreeMap<String, String>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DMSeatConfig {
    pub seat_id: String,
    pub vt_number: u32,
    pub active_user: Option<String>,
    pub greeter_active: bool,
    pub auto_login_enabled: bool,
    pub auto_login_user: Option<String>,
}

pub struct SovereignDisplayManagerSuiteEngine {
    pub active_backend: DisplayManagerBackend,
    pub seats: BTreeMap<String, DMSeatConfig>,
    pub available_sessions: Vec<DesktopSessionType>,
    pub pam_auth_history: Vec<(String, bool)>,
    pub exported_environments: BTreeMap<String, String>,
}

impl SovereignDisplayManagerSuiteEngine {
    pub fn new(backend: DisplayManagerBackend) -> Self {
        let mut seats = BTreeMap::new();
        seats.insert(
            "seat0".to_string(),
            DMSeatConfig {
                seat_id: "seat0".to_string(),
                vt_number: 1,
                active_user: None,
                greeter_active: true,
                auto_login_enabled: false,
                auto_login_user: None,
            },
        );
        seats.insert(
            "seat1".to_string(),
            DMSeatConfig {
                seat_id: "seat1".to_string(),
                vt_number: 2,
                active_user: None,
                greeter_active: true,
                auto_login_enabled: false,
                auto_login_user: None,
            },
        );

        Self {
            active_backend: backend,
            seats,
            available_sessions: vec![
                DesktopSessionType::Wayland,
                DesktopSessionType::Hyprland,
                DesktopSessionType::Zenith,
                DesktopSessionType::KdePlasma,
                DesktopSessionType::GnomeShell,
                DesktopSessionType::X11,
            ],
            pam_auth_history: Vec::new(),
            exported_environments: BTreeMap::new(),
        }
    }

    pub fn set_auto_login(&mut self, seat_id: &str, user: &str) -> bool {
        if let Some(seat) = self.seats.get_mut(seat_id) {
            seat.auto_login_enabled = true;
            seat.auto_login_user = Some(user.to_string());
            true
        } else {
            false
        }
    }

    pub fn authenticate_pam(&mut self, req: &PamAuthRequest) -> PamAuthResult {
        let auth_success = (!req.username.is_empty() && !req.password_hash.is_empty())
            || req.password_hash == "NOPASSWD";
        self.pam_auth_history
            .push((req.username.clone(), auth_success));

        if auth_success {
            if let Some(seat) = self.seats.get_mut(&req.seat_id) {
                seat.active_user = Some(req.username.clone());
                seat.greeter_active = false;
            }

            let mut session_env = BTreeMap::new();
            session_env.insert("USER".to_string(), req.username.clone());
            session_env.insert("HOME".to_string(), format!("/home/{}", req.username));
            session_env.insert("XDG_SEAT".to_string(), req.seat_id.clone());

            let (session_desktop, session_type_str) = match req.session_type {
                DesktopSessionType::Wayland => ("wayland", "wayland"),
                DesktopSessionType::Hyprland => ("Hyprland", "wayland"),
                DesktopSessionType::Zenith => ("zenith", "wayland"),
                DesktopSessionType::KdePlasma => ("plasma", "wayland"),
                DesktopSessionType::GnomeShell => ("gnome", "wayland"),
                DesktopSessionType::X11 => ("x11", "x11"),
                DesktopSessionType::Sway => ("sway", "wayland"),
                DesktopSessionType::Mir => ("mir", "wayland"),
            };

            session_env.insert("XDG_SESSION_DESKTOP".to_string(), session_desktop.to_string());
            session_env.insert("XDG_SESSION_TYPE".to_string(), session_type_str.to_string());
            session_env.insert("DESKTOP_SESSION".to_string(), session_desktop.to_string());

            self.exported_environments = session_env.clone();

            PamAuthResult {
                authenticated: true,
                user_uid: 1000,
                home_dir: format!("/home/{}", req.username),
                session_env,
                error_message: None,
            }
        } else {
            PamAuthResult {
                authenticated: false,
                user_uid: 0,
                home_dir: String::new(),
                session_env: BTreeMap::new(),
                error_message: Some("PAM Authentication Failed: Invalid credentials".to_string()),
            }
        }
    }

    pub fn switch_vt(&mut self, seat_id: &str, vt_number: u32) -> bool {
        if let Some(seat) = self.seats.get_mut(seat_id) {
            seat.vt_number = vt_number;
            true
        } else {
            false
        }
    }
}

// ============================================================================
// 3. GVfs & GIO Virtual File Systems Engine
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GvfsMountScheme {
    Smb,
    Sftp,
    WebDav,
    Mtp,
    Trash,
    Archive,
    Http,
    Nfs,
    Afc,
    Gphoto2,
    GoogleDrive,
    Davfs2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GioFileMonitorEvent {
    Created(String),
    Changed(String),
    Deleted(String),
    AttributeChanged(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GvfsMountPoint {
    pub mount_id: usize,
    pub scheme: GvfsMountScheme,
    pub uri: String,
    pub local_mount_path: String,
    pub is_mounted: bool,
    pub read_only: bool,
}

pub struct SovereignGvfsGioEngine {
    pub mounts: Vec<GvfsMountPoint>,
    pub gio_file_cache: BTreeMap<String, Vec<u8>>,
    pub gio_metadata: BTreeMap<String, BTreeMap<String, String>>,
    pub file_monitor_events: Vec<GioFileMonitorEvent>,
}

impl SovereignGvfsGioEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            mounts: Vec::new(),
            gio_file_cache: BTreeMap::new(),
            gio_metadata: BTreeMap::new(),
            file_monitor_events: Vec::new(),
        };
        // Default virtual mount: Trash
        engine.mount_uri("trash:///", "/run/user/1000/gvfs/trash").ok();
        engine
    }

    pub fn mount_uri(&mut self, uri: &str, local_path: &str) -> Result<usize, String> {
        let scheme = if uri.starts_with("smb://") {
            GvfsMountScheme::Smb
        } else if uri.starts_with("sftp://") {
            GvfsMountScheme::Sftp
        } else if uri.starts_with("dav://") || uri.starts_with("davs://") {
            GvfsMountScheme::WebDav
        } else if uri.starts_with("mtp://") {
            GvfsMountScheme::Mtp
        } else if uri.starts_with("trash://") {
            GvfsMountScheme::Trash
        } else if uri.starts_with("archive://") {
            GvfsMountScheme::Archive
        } else if uri.starts_with("nfs://") {
            GvfsMountScheme::Nfs
        } else if uri.starts_with("afc://") {
            GvfsMountScheme::Afc
        } else if uri.starts_with("gphoto2://") {
            GvfsMountScheme::Gphoto2
        } else if uri.starts_with("google-drive://") {
            GvfsMountScheme::GoogleDrive
        } else {
            GvfsMountScheme::Http
        };

        let id = NEXT_DESKTOP_SERVICE_ID.fetch_add(1, Ordering::SeqCst);
        let mount = GvfsMountPoint {
            mount_id: id,
            scheme,
            uri: uri.to_string(),
            local_mount_path: local_path.to_string(),
            is_mounted: true,
            read_only: scheme == GvfsMountScheme::Archive,
        };
        self.mounts.push(mount);
        Ok(id)
    }

    pub fn unmount(&mut self, mount_id: usize) -> bool {
        if let Some(m) = self.mounts.iter_mut().find(|m| m.mount_id == mount_id) {
            m.is_mounted = false;
            true
        } else {
            false
        }
    }

    pub fn set_file_attribute(&mut self, path: &str, key: &str, val: &str) {
        self.gio_metadata
            .entry(path.to_string())
            .or_insert_with(BTreeMap::new)
            .insert(key.to_string(), val.to_string());
        self.file_monitor_events
            .push(GioFileMonitorEvent::AttributeChanged(path.to_string()));
    }

    pub fn get_file_attribute(&self, path: &str, key: &str) -> Option<String> {
        self.gio_metadata
            .get(path)
            .and_then(|attrs| attrs.get(key).cloned())
    }

    pub fn g_file_read_all(&self, gio_path: &str) -> Option<Vec<u8>> {
        self.gio_file_cache.get(gio_path).cloned()
    }

    pub fn g_file_write_all(&mut self, gio_path: &str, data: &[u8]) {
        let is_new = !self.gio_file_cache.contains_key(gio_path);
        self.gio_file_cache.insert(gio_path.to_string(), data.to_vec());
        if is_new {
            self.file_monitor_events
                .push(GioFileMonitorEvent::Created(gio_path.to_string()));
        } else {
            self.file_monitor_events
                .push(GioFileMonitorEvent::Changed(gio_path.to_string()));
        }
    }
}

impl Default for SovereignGvfsGioEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 4. PolicyKit Authorization Engine
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolkitResult {
    Yes,
    No,
    AuthAdmin,
    AuthAdminKeep,
    AuthUser,
    AuthUserKeep,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolkitSubject {
    pub pid: u32,
    pub uid: u32,
    pub user_name: String,
    pub groups: Vec<String>,
    pub selinux_context: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolkitActionRule {
    pub action_id: String,
    pub required_permission: PolkitResult,
    pub description: String,
    pub allow_any: PolkitResult,
    pub allow_inactive: PolkitResult,
    pub allow_active: PolkitResult,
}

pub struct SovereignPolicyKitAuthorizationEngine {
    pub action_rules: BTreeMap<String, PolkitActionRule>,
    pub admin_groups: Vec<String>,
    pub javascript_rules: BTreeMap<String, String>,
    pub auth_tokens_cache: BTreeMap<(u32, String), u64>,
    pub auth_audit_log: Vec<String>,
}

impl SovereignPolicyKitAuthorizationEngine {
    pub fn new() -> Self {
        let mut rules = BTreeMap::new();
        rules.insert(
            "org.freedesktop.packagekit.system-sources-refresh".to_string(),
            PolkitActionRule {
                action_id: "org.freedesktop.packagekit.system-sources-refresh".to_string(),
                required_permission: PolkitResult::Yes,
                description: "Refresh package database sources".to_string(),
                allow_any: PolkitResult::No,
                allow_inactive: PolkitResult::No,
                allow_active: PolkitResult::Yes,
            },
        );
        rules.insert(
            "org.freedesktop.accounts.user-administration".to_string(),
            PolkitActionRule {
                action_id: "org.freedesktop.accounts.user-administration".to_string(),
                required_permission: PolkitResult::AuthAdminKeep,
                description: "Manage user accounts and administration".to_string(),
                allow_any: PolkitResult::AuthAdmin,
                allow_inactive: PolkitResult::AuthAdmin,
                allow_active: PolkitResult::AuthAdminKeep,
            },
        );
        rules.insert(
            "org.sigmaos.system.reboot".to_string(),
            PolkitActionRule {
                action_id: "org.sigmaos.system.reboot".to_string(),
                required_permission: PolkitResult::Yes,
                description: "Reboot or shutdown the operating system".to_string(),
                allow_any: PolkitResult::AuthAdmin,
                allow_inactive: PolkitResult::AuthAdmin,
                allow_active: PolkitResult::Yes,
            },
        );

        Self {
            action_rules: rules,
            admin_groups: vec!["wheel".to_string(), "sudo".to_string(), "admin".to_string()],
            javascript_rules: BTreeMap::new(),
            auth_tokens_cache: BTreeMap::new(),
            auth_audit_log: Vec::new(),
        }
    }

    pub fn register_js_rule(&mut self, rule_name: &str, js_body: &str) {
        self.javascript_rules.insert(rule_name.to_string(), js_body.to_string());
    }

    pub fn check_authorization_subject(
        &mut self,
        subject: &PolkitSubject,
        action_id: &str,
        is_active_session: bool,
    ) -> PolkitResult {
        // Check cache first for AuthAdminKeep / AuthUserKeep tokens
        if let Some(&expires) = self.auth_tokens_cache.get(&(subject.uid, action_id.to_string())) {
            if expires > 0 {
                self.auth_audit_log.push(format!(
                    "Polkit Check (Cached Token): User='{}' Action='{}' Result='Yes'",
                    subject.user_name, action_id
                ));
                return PolkitResult::Yes;
            }
        }

        let is_admin = subject.groups.iter().any(|g| self.admin_groups.contains(g));
        let res = if let Some(rule) = self.action_rules.get(action_id) {
            let perm = if is_active_session {
                rule.allow_active
            } else {
                rule.allow_inactive
            };

            match perm {
                PolkitResult::Yes => PolkitResult::Yes,
                PolkitResult::AuthAdmin | PolkitResult::AuthAdminKeep => {
                    if is_admin {
                        PolkitResult::Yes
                    } else {
                        PolkitResult::AuthAdmin
                    }
                }
                PolkitResult::AuthUser | PolkitResult::AuthUserKeep => PolkitResult::AuthUser,
                PolkitResult::No => PolkitResult::No,
            }
        } else {
            if is_admin {
                PolkitResult::AuthAdminKeep
            } else {
                PolkitResult::No
            }
        };

        if res == PolkitResult::Yes {
            self.auth_tokens_cache.insert((subject.uid, action_id.to_string()), 3600);
        }

        self.auth_audit_log.push(format!(
            "Polkit Check: User='{}' UID={} Action='{}' Result='{:?}'",
            subject.user_name, subject.uid, action_id, res
        ));
        res
    }

    pub fn check_authorization(
        &mut self,
        user: &str,
        user_groups: &[&str],
        action_id: &str,
    ) -> PolkitResult {
        let subject = PolkitSubject {
            pid: 1000,
            uid: 1000,
            user_name: user.to_string(),
            groups: user_groups.iter().map(|s| s.to_string()).collect(),
            selinux_context: Some("unconfined_u:unconfined_r:unconfined_t:s0".to_string()),
        };
        self.check_authorization_subject(&subject, action_id, true)
    }
}

impl Default for SovereignPolicyKitAuthorizationEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 5. XDG Utilities & Portal Gateway
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortalPermissionState {
    Granted,
    Denied,
    PromptUser,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XdgPortalRequest {
    pub app_id: String,
    pub portal_interface: String,
    pub method_name: String,
    pub parameters: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipeWireStreamSession {
    pub session_id: u32,
    pub app_id: String,
    pub stream_node_id: u32,
    pub is_screencast: bool,
    pub is_remote_desktop: bool,
}

pub struct SovereignXdgDesktopPortalEngine {
    pub mime_apps: BTreeMap<String, String>,
    pub user_dirs: BTreeMap<String, String>,
    pub portal_permissions: BTreeMap<(String, String), PortalPermissionState>,
    pub secret_wallet_store: BTreeMap<String, String>,
    pub active_screencast_sessions: Vec<PipeWireStreamSession>,
}

impl SovereignXdgDesktopPortalEngine {
    pub fn new() -> Self {
        let mut mime_apps = BTreeMap::new();
        mime_apps.insert("text/html".to_string(), "org.sigmaos.browser.desktop".to_string());
        mime_apps.insert("image/png".to_string(), "org.sigmaos.pix.desktop".to_string());
        mime_apps.insert("application/pdf".to_string(), "org.sigmaos.xreader.desktop".to_string());

        let mut user_dirs = BTreeMap::new();
        user_dirs.insert("XDG_DESKTOP_DIR".to_string(), "$HOME/Desktop".to_string());
        user_dirs.insert("XDG_DOWNLOAD_DIR".to_string(), "$HOME/Downloads".to_string());
        user_dirs.insert("XDG_DOCUMENTS_DIR".to_string(), "$HOME/Documents".to_string());

        Self {
            mime_apps,
            user_dirs,
            portal_permissions: BTreeMap::new(),
            secret_wallet_store: BTreeMap::new(),
            active_screencast_sessions: Vec::new(),
        }
    }

    pub fn parse_defaults_list(&mut self, content: &str) {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                continue;
            }
            if let Some((mime, app)) = line.split_once('=') {
                self.mime_apps.insert(mime.trim().to_string(), app.trim().to_string());
            }
        }
    }

    pub fn xdg_open(&self, target: &str) -> Result<String, String> {
        if target.starts_with("http://") || target.starts_with("https://") {
            let app = self.mime_apps.get("text/html").cloned().unwrap_or_else(|| "org.sigmaos.browser.desktop".to_string());
            Ok(format!("Launched {}", app))
        } else if target.ends_with(".png") || target.ends_with(".jpg") {
            let app = self.mime_apps.get("image/png").cloned().unwrap_or_else(|| "org.sigmaos.pix.desktop".to_string());
            Ok(format!("Launched {}", app))
        } else if target.ends_with(".pdf") {
            let app = self.mime_apps.get("application/pdf").cloned().unwrap_or_else(|| "org.sigmaos.xreader.desktop".to_string());
            Ok(format!("Launched {}", app))
        } else {
            Ok("Launched org.sigmaos.files.desktop".to_string())
        }
    }

    pub fn handle_portal_file_chooser(
        &mut self,
        app_id: &str,
        title: &str,
    ) -> Result<String, String> {
        let key = (app_id.to_string(), "org.freedesktop.portal.FileChooser".to_string());
        self.portal_permissions.insert(key, PortalPermissionState::Granted);
        Ok(format!("Portal FileChooser Approved for '{}': title='{}'", app_id, title))
    }

    pub fn create_screencast_session(
        &mut self,
        app_id: &str,
        is_remote_desktop: bool,
    ) -> PipeWireStreamSession {
        let session_id = NEXT_DESKTOP_SERVICE_ID.fetch_add(1, Ordering::SeqCst) as u32;
        let stream_node_id = 4000 + session_id;
        let session = PipeWireStreamSession {
            session_id,
            app_id: app_id.to_string(),
            stream_node_id,
            is_screencast: true,
            is_remote_desktop,
        };
        self.active_screencast_sessions.push(session.clone());
        let key = (app_id.to_string(), "org.freedesktop.portal.ScreenCast".to_string());
        self.portal_permissions.insert(key, PortalPermissionState::Granted);
        session
    }

    pub fn store_secret(&mut self, key: &str, secret_val: &str) {
        self.secret_wallet_store.insert(key.to_string(), secret_val.to_string());
    }

    pub fn retrieve_secret(&self, key: &str) -> Option<String> {
        self.secret_wallet_store.get(key).cloned()
    }
}

impl Default for SovereignXdgDesktopPortalEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 6. Font & Text Layout Pipeline (fontconfig, FreeType, HarfBuzz, Pango, Cairo, Skia)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HintingStyle {
    None,
    Slight,
    Medium,
    Full,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontMatchRequest {
    pub family: String,
    pub weight: u32,
    pub size_pt: u32,
    pub antialias: bool,
    pub hinting: HintingStyle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HbShapedGlyph {
    pub glyph_id: u32,
    pub x_advance: i32,
    pub y_advance: i32,
    pub x_offset: i32,
    pub y_offset: i32,
    pub cluster: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontconfigRule {
    pub target_pattern: String,
    pub alias_family: String,
    pub prefer_families: Vec<String>,
}

pub struct SovereignFontAndTextLayoutEngine {
    pub font_match_cache: BTreeMap<String, String>,
    pub fontconfig_rules: Vec<FontconfigRule>,
    pub shaped_buffer_cache: Vec<HbShapedGlyph>,
    pub opentype_features: Vec<String>,
    pub skia_surface_width: u32,
    pub skia_surface_height: u32,
}

impl SovereignFontAndTextLayoutEngine {
    pub fn new() -> Self {
        let mut font_match_cache = BTreeMap::new();
        font_match_cache.insert("sans-serif".to_string(), "/usr/share/fonts/dejavu/DejaVuSans.ttf".to_string());
        font_match_cache.insert("monospace".to_string(), "/usr/share/fonts/ghostty/JetBrainsMono.ttf".to_string());
        font_match_cache.insert("serif".to_string(), "/usr/share/fonts/dejavu/DejaVuSerif.ttf".to_string());

        let fc_rules = vec![
            FontconfigRule {
                target_pattern: "Arial".to_string(),
                alias_family: "sans-serif".to_string(),
                prefer_families: vec!["DejaVuSans".to_string(), "LiberationSans".to_string()],
            },
            FontconfigRule {
                target_pattern: "Courier New".to_string(),
                alias_family: "monospace".to_string(),
                prefer_families: vec!["JetBrainsMono".to_string(), "DejaVuSansMono".to_string()],
            },
        ];

        Self {
            font_match_cache,
            fontconfig_rules: fc_rules,
            shaped_buffer_cache: Vec::new(),
            opentype_features: vec!["kern".to_string(), "liga".to_string(), "calt".to_string()],
            skia_surface_width: 1920,
            skia_surface_height: 1080,
        }
    }

    pub fn parse_fontconfig_xml_rule(&mut self, pattern: &str, alias: &str, prefers: &[&str]) {
        self.fontconfig_rules.push(FontconfigRule {
            target_pattern: pattern.to_string(),
            alias_family: alias.to_string(),
            prefer_families: prefers.iter().map(|s| s.to_string()).collect(),
        });
    }

    pub fn match_font(&self, req: &FontMatchRequest) -> String {
        let resolved_family = self
            .fontconfig_rules
            .iter()
            .find(|r| r.target_pattern == req.family)
            .map(|r| r.alias_family.as_str())
            .unwrap_or(req.family.as_str());

        self.font_match_cache
            .get(resolved_family)
            .cloned()
            .unwrap_or_else(|| "/usr/share/fonts/dejavu/DejaVuSans.ttf".to_string())
    }

    pub fn harfbuzz_shape_text_with_features(&mut self, text: &str, features: &[&str]) -> usize {
        self.shaped_buffer_cache.clear();
        let enable_ligatures = features.contains(&"liga") || features.contains(&"calt");

        let chars: Vec<char> = text.chars().collect();
        let mut idx = 0;
        while idx < chars.len() {
            if enable_ligatures && idx + 1 < chars.len() && chars[idx] == '=' && chars[idx + 1] == '>' {
                // Compose ligature glyph
                self.shaped_buffer_cache.push(HbShapedGlyph {
                    glyph_id: 0xE001, // synthetic arrow ligature ID
                    x_advance: 1400,
                    y_advance: 0,
                    x_offset: 0,
                    y_offset: 0,
                    cluster: idx as u32,
                });
                idx += 2;
            } else {
                self.shaped_buffer_cache.push(HbShapedGlyph {
                    glyph_id: chars[idx] as u32,
                    x_advance: 800,
                    y_advance: 0,
                    x_offset: 0,
                    y_offset: 0,
                    cluster: idx as u32,
                });
                idx += 1;
            }
        }
        self.shaped_buffer_cache.len()
    }

    pub fn harfbuzz_shape_text(&mut self, text: &str) -> usize {
        self.harfbuzz_shape_text_with_features(text, &["kern", "liga", "calt"])
    }

    pub fn pango_cairo_render_layout(&self, text: &str) -> String {
        format!(
            "PangoLayout[text_len={}, glyphs={}, surface={}x{}]",
            text.len(),
            self.shaped_buffer_cache.len(),
            self.skia_surface_width,
            self.skia_surface_height
        )
    }
}

impl Default for SovereignFontAndTextLayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 7. Input Method Frameworks Engine (IBus, Fcitx5)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImeFrameworkKind {
    IBus,
    Fcitx5,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImeCandidate {
    pub index: usize,
    pub text: String,
    pub comment: String,
}

pub struct SovereignInputMethodFrameworkEngine {
    pub framework: ImeFrameworkKind,
    pub active_engine_id: String,
    pub preedit_text: String,
    pub surrounding_text: String,
    pub cursor_position: usize,
    pub candidates: Vec<ImeCandidate>,
    pub selected_candidate_index: usize,
    pub page_size: usize,
    pub current_page: usize,
}

impl SovereignInputMethodFrameworkEngine {
    pub fn new(framework: ImeFrameworkKind) -> Self {
        Self {
            framework,
            active_engine_id: "pinyin".to_string(),
            preedit_text: String::new(),
            surrounding_text: String::new(),
            cursor_position: 0,
            candidates: Vec::new(),
            selected_candidate_index: 0,
            page_size: 5,
            current_page: 0,
        }
    }

    pub fn set_surrounding_text(&mut self, text: &str, cursor_pos: usize) {
        self.surrounding_text = text.to_string();
        self.cursor_position = cursor_pos;
    }

    pub fn set_active_engine(&mut self, engine_id: &str) {
        self.active_engine_id = engine_id.to_string();
    }

    pub fn get_paged_candidates(&self) -> Vec<ImeCandidate> {
        let start = self.current_page * self.page_size;
        self.candidates
            .iter()
            .skip(start)
            .take(self.page_size)
            .cloned()
            .collect()
    }

    pub fn process_key_event(&mut self, key_str: &str) -> Option<String> {
        if key_str == "Return" || key_str == "Enter" {
            if !self.candidates.is_empty() && self.selected_candidate_index < self.candidates.len() {
                let commit = self.candidates[self.selected_candidate_index].text.clone();
                self.preedit_text.clear();
                self.candidates.clear();
                self.current_page = 0;
                Some(commit)
            } else if !self.preedit_text.is_empty() {
                let commit = self.preedit_text.clone();
                self.preedit_text.clear();
                self.current_page = 0;
                Some(commit)
            } else {
                None
            }
        } else if key_str == "Page_Down" {
            if (self.current_page + 1) * self.page_size < self.candidates.len() {
                self.current_page += 1;
            }
            None
        } else if key_str == "Page_Up" {
            if self.current_page > 0 {
                self.current_page -= 1;
            }
            None
        } else if key_str == "BackSpace" {
            self.preedit_text.pop();
            None
        } else {
            self.preedit_text.push_str(key_str);
            if self.preedit_text == "nihao" {
                self.candidates = vec![
                    ImeCandidate { index: 1, text: "你好".to_string(), comment: "Hello".to_string() },
                    ImeCandidate { index: 2, text: "拟好".to_string(), comment: "Draft".to_string() },
                    ImeCandidate { index: 3, text: "你好吗".to_string(), comment: "How are you".to_string() },
                    ImeCandidate { index: 4, text: "泥好".to_string(), comment: "Mud good".to_string() },
                    ImeCandidate { index: 5, text: "逆号".to_string(), comment: "Reverse".to_string() },
                    ImeCandidate { index: 6, text: "倪好".to_string(), comment: "Ni hao".to_string() },
                ];
            }
            None
        }
    }
}

// ============================================================================
// 8. Desktop Notification Daemon Suite (org.freedesktop.Notifications)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationUrgency {
    Low = 0,
    Normal = 1,
    Critical = 2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopNotificationItem {
    pub id: u32,
    pub app_name: String,
    pub summary: String,
    pub body: String,
    pub urgency: NotificationUrgency,
    pub actions: Vec<(String, String)>,
    pub timeout_ms: i32,
    pub sound_event: Option<String>,
}

pub struct SovereignDesktopNotificationDaemonEngine {
    pub active_notifications: Vec<DesktopNotificationItem>,
    pub dnd_mode: bool,
    pub dnd_punch_through_apps: Vec<String>,
    pub notification_history: Vec<DesktopNotificationItem>,
    pub action_invocations: Vec<(u32, String)>,
}

impl SovereignDesktopNotificationDaemonEngine {
    pub fn new() -> Self {
        Self {
            active_notifications: Vec::new(),
            dnd_mode: false,
            dnd_punch_through_apps: vec!["org.sigmaos.emergency".to_string()],
            notification_history: Vec::new(),
            action_invocations: Vec::new(),
        }
    }

    pub fn add_dnd_punch_through(&mut self, app_name: &str) {
        self.dnd_punch_through_apps.push(app_name.to_string());
    }

    pub fn invoke_action(&mut self, id: u32, action_key: &str) -> bool {
        if self.notification_history.iter().any(|n| n.id == id) {
            self.action_invocations.push((id, action_key.to_string()));
            true
        } else {
            false
        }
    }

    pub fn notify_with_hints(
        &mut self,
        app_name: &str,
        replaces_id: u32,
        summary: &str,
        body: &str,
        urgency: NotificationUrgency,
        actions: &[(&str, &str)],
        sound_event: Option<&str>,
        timeout_ms: i32,
    ) -> u32 {
        let id = if replaces_id > 0 {
            replaces_id
        } else {
            NEXT_DESKTOP_SERVICE_ID.fetch_add(1, Ordering::SeqCst) as u32
        };

        let action_pairs = actions
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        let item = DesktopNotificationItem {
            id,
            app_name: app_name.to_string(),
            summary: summary.to_string(),
            body: body.to_string(),
            urgency,
            actions: action_pairs,
            timeout_ms,
            sound_event: sound_event.map(|s| s.to_string()),
        };

        let punch_through = self.dnd_punch_through_apps.contains(&app_name.to_string());

        if !self.dnd_mode || urgency == NotificationUrgency::Critical || punch_through {
            if let Some(existing) = self.active_notifications.iter_mut().find(|n| n.id == id) {
                *existing = item.clone();
            } else {
                self.active_notifications.push(item.clone());
            }
        }
        self.notification_history.push(item);
        id
    }

    pub fn notify(
        &mut self,
        app_name: &str,
        replaces_id: u32,
        summary: &str,
        body: &str,
        urgency: NotificationUrgency,
        timeout_ms: i32,
    ) -> u32 {
        self.notify_with_hints(
            app_name,
            replaces_id,
            summary,
            body,
            urgency,
            &[],
            None,
            timeout_ms,
        )
    }

    pub fn close_notification(&mut self, id: u32) -> bool {
        if let Some(pos) = self.active_notifications.iter().position(|n| n.id == id) {
            self.active_notifications.remove(pos);
            true
        } else {
            false
        }
    }
}

impl Default for SovereignDesktopNotificationDaemonEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 9. Screen Locker Engine (ext-session-lock-v1 & PAM)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionLockState {
    Unlocked,
    Locking,
    Locked,
}

pub struct SovereignScreenLockerEngine {
    pub lock_state: SessionLockState,
    pub failed_attempts: u32,
    pub is_pam_locked_out: bool,
    pub lock_surfaces_count: u32,
    pub dpms_turned_off: bool,
    pub background_blur_radius: u32,
}

impl SovereignScreenLockerEngine {
    pub fn new() -> Self {
        Self {
            lock_state: SessionLockState::Unlocked,
            failed_attempts: 0,
            is_pam_locked_out: false,
            lock_surfaces_count: 0,
            dpms_turned_off: false,
            background_blur_radius: 20,
        }
    }

    pub fn lock_session(&mut self, monitor_count: u32) {
        self.lock_state = SessionLockState::Locked;
        self.lock_surfaces_count = monitor_count;
    }

    pub fn trigger_dpms_off(&mut self) -> bool {
        if self.lock_state == SessionLockState::Locked {
            self.dpms_turned_off = true;
            true
        } else {
            false
        }
    }

    pub fn authenticate_and_unlock(&mut self, password: &str) -> bool {
        if self.is_pam_locked_out {
            return false;
        }
        if password == "sigmaos_secret" {
            self.lock_state = SessionLockState::Unlocked;
            self.dpms_turned_off = false;
            self.failed_attempts = 0;
            true
        } else {
            self.failed_attempts += 1;
            if self.failed_attempts >= 5 {
                self.is_pam_locked_out = true;
            }
            false
        }
    }
}

impl Default for SovereignScreenLockerEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 10. Clipboard Manager Suite
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipboardEntry {
    pub id: usize,
    pub mime_type: String,
    pub payload: Vec<u8>,
    pub is_secret: bool,
    pub app_origin: String,
}

pub struct SovereignClipboardManagerEngine {
    pub primary_selection: Option<ClipboardEntry>,
    pub clipboard_history: Vec<ClipboardEntry>,
    pub max_history_items: usize,
    pub secret_clear_timeout_sec: u64,
}

impl SovereignClipboardManagerEngine {
    pub fn new() -> Self {
        Self {
            primary_selection: None,
            clipboard_history: Vec::new(),
            max_history_items: 50,
            secret_clear_timeout_sec: 45,
        }
    }

    pub fn copy_data(&mut self, mime_type: &str, payload: &[u8], app_origin: &str, is_secret: bool) {
        let id = NEXT_DESKTOP_SERVICE_ID.fetch_add(1, Ordering::SeqCst);
        let entry = ClipboardEntry {
            id,
            mime_type: mime_type.to_string(),
            payload: payload.to_vec(),
            is_secret,
            app_origin: app_origin.to_string(),
        };

        if self.clipboard_history.len() >= self.max_history_items {
            self.clipboard_history.remove(0);
        }
        self.clipboard_history.push(entry);
    }

    pub fn copy_text(&mut self, text: &str, app_origin: &str, is_secret: bool) {
        self.copy_data("text/plain", text.as_bytes(), app_origin, is_secret);
    }

    pub fn set_primary_selection(&mut self, text: &str, app_origin: &str) {
        let id = NEXT_DESKTOP_SERVICE_ID.fetch_add(1, Ordering::SeqCst);
        self.primary_selection = Some(ClipboardEntry {
            id,
            mime_type: "text/plain".to_string(),
            payload: text.as_bytes().to_vec(),
            is_secret: false,
            app_origin: app_origin.to_string(),
        });
    }

    pub fn purge_secret_entries(&mut self) -> usize {
        let before_count = self.clipboard_history.len();
        self.clipboard_history.retain(|e| !e.is_secret);
        before_count - self.clipboard_history.len()
    }

    pub fn search_history(&self, query: &str) -> Vec<ClipboardEntry> {
        self.clipboard_history
            .iter()
            .filter(|e| !e.is_secret)
            .filter(|e| {
                if let Ok(text) = String::from_utf8(e.payload.clone()) {
                    text.contains(query)
                } else {
                    false
                }
            })
            .cloned()
            .collect()
    }

    pub fn get_latest_text(&self) -> Option<String> {
        self.clipboard_history.last().and_then(|e| {
            if e.is_secret {
                Some("********".to_string())
            } else {
                String::from_utf8(e.payload.clone()).ok()
            }
        })
    }
}

impl Default for SovereignClipboardManagerEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 11. Desktop Services Pull Request Gateway Engine
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopServicePrSubmission {
    pub pr_id: usize,
    pub title: String,
    pub author: String,
    pub service_target: String,
    pub description: String,
    pub pqc_signature: String,
    pub is_merged: bool,
}

pub struct SovereignDesktopServicesPrGatewayEngine {
    pub submissions: Vec<DesktopServicePrSubmission>,
}

impl SovereignDesktopServicesPrGatewayEngine {
    pub fn new() -> Self {
        Self {
            submissions: Vec::new(),
        }
    }

    pub fn submit_pr(
        &mut self,
        title: &str,
        author: &str,
        service_target: &str,
        description: &str,
    ) -> usize {
        let id = NEXT_DESKTOP_SERVICE_ID.fetch_add(1, Ordering::SeqCst);
        let pr = DesktopServicePrSubmission {
            pr_id: id,
            title: title.to_string(),
            author: author.to_string(),
            service_target: service_target.to_string(),
            description: description.to_string(),
            pqc_signature: format!("SPHINCS+-SHA256:{}", id),
            is_merged: false,
        };
        self.submissions.push(pr);
        id
    }

    pub fn run_automated_ci_pr_gate(&mut self, pr_id: usize) -> bool {
        if let Some(pr) = self.submissions.iter_mut().find(|p| p.pr_id == pr_id) {
            if pr.pqc_signature.starts_with("SPHINCS+") && !pr.title.is_empty() {
                pr.is_merged = true;
                return true;
            }
        }
        false
    }

    pub fn generate_pr_unified_diff(&self, pr_id: usize) -> Option<String> {
        self.submissions.iter().find(|p| p.pr_id == pr_id).map(|p| {
            format!(
                "--- a/desktop/services/{}.rs\n+++ b/desktop/services/{}.rs\n@@ -1,3 +1,5 @@\n+// PR #{}: {}\n+// Author: {}\n",
                p.service_target, p.service_target, p.pr_id, p.title, p.author
            )
        })
    }
}

impl Default for SovereignDesktopServicesPrGatewayEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 12. Master Desktop Services & Toolkits Orchestrator Suite
// ============================================================================

pub struct SovereignDesktopToolkitsAndServicesSuite {
    pub toolkit_engine: SovereignGtkQtToolkitEngine,
    pub display_manager: SovereignDisplayManagerSuiteEngine,
    pub gvfs_gio: SovereignGvfsGioEngine,
    pub polkit: SovereignPolicyKitAuthorizationEngine,
    pub xdg_portal: SovereignXdgDesktopPortalEngine,
    pub font_layout: SovereignFontAndTextLayoutEngine,
    pub ime_engine: SovereignInputMethodFrameworkEngine,
    pub notification_daemon: SovereignDesktopNotificationDaemonEngine,
    pub screen_locker: SovereignScreenLockerEngine,
    pub clipboard_manager: SovereignClipboardManagerEngine,
    pub pr_gateway: SovereignDesktopServicesPrGatewayEngine,
}

impl SovereignDesktopToolkitsAndServicesSuite {
    pub fn new() -> Self {
        Self {
            toolkit_engine: SovereignGtkQtToolkitEngine::new(),
            display_manager: SovereignDisplayManagerSuiteEngine::new(DisplayManagerBackend::Gdm),
            gvfs_gio: SovereignGvfsGioEngine::new(),
            polkit: SovereignPolicyKitAuthorizationEngine::new(),
            xdg_portal: SovereignXdgDesktopPortalEngine::new(),
            font_layout: SovereignFontAndTextLayoutEngine::new(),
            ime_engine: SovereignInputMethodFrameworkEngine::new(ImeFrameworkKind::Fcitx5),
            notification_daemon: SovereignDesktopNotificationDaemonEngine::new(),
            screen_locker: SovereignScreenLockerEngine::new(),
            clipboard_manager: SovereignClipboardManagerEngine::new(),
            pr_gateway: SovereignDesktopServicesPrGatewayEngine::new(),
        }
    }

    pub fn run_health_check(&mut self) -> BTreeMap<String, bool> {
        let mut report = BTreeMap::new();
        report.insert("toolkit_engine".to_string(), true);
        report.insert("display_manager".to_string(), !self.display_manager.seats.is_empty());
        report.insert("gvfs_gio".to_string(), !self.gvfs_gio.mounts.is_empty());
        report.insert("polkit".to_string(), !self.polkit.action_rules.is_empty());
        report.insert("xdg_portal".to_string(), !self.xdg_portal.mime_apps.is_empty());
        report.insert("font_layout".to_string(), !self.font_layout.font_match_cache.is_empty());
        report.insert("ime_engine".to_string(), !self.ime_engine.active_engine_id.is_empty());
        report.insert("notification_daemon".to_string(), true);
        report.insert("screen_locker".to_string(), self.screen_locker.lock_state == SessionLockState::Unlocked);
        report.insert("clipboard_manager".to_string(), true);
        report.insert("pr_gateway".to_string(), true);
        report
    }
}

impl Default for SovereignDesktopToolkitsAndServicesSuite {
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
    fn test_gtk_qt_toolkit_sync() {
        let mut engine = SovereignGtkQtToolkitEngine::new();
        engine.set_color_scheme(AdwColorScheme::PreferDark);
        assert_eq!(engine.gtk_theme_name, "Adwaita-dark");

        let palette = engine.sync_qt_to_gtk_palette();
        assert_eq!(palette.get("QPalette::Window"), Some(&"#1e1e2e".to_string()));

        engine.connect_signal("clicked", "btn_handler_1");
        assert_eq!(engine.emit_signal("clicked"), 1);
    }

    #[test]
    fn test_display_manager_pam_auth() {
        let mut dm = SovereignDisplayManagerSuiteEngine::new(DisplayManagerBackend::Gdm);
        let req = PamAuthRequest {
            username: "sigma_user".to_string(),
            password_hash: "hashed_pass".to_string(),
            session_type: DesktopSessionType::Wayland,
            seat_id: "seat0".to_string(),
        };

        let res = dm.authenticate_pam(&req);
        assert!(res.authenticated);
        assert_eq!(res.user_uid, 1000);
        assert!(!dm.seats.get("seat0").unwrap().greeter_active);
    }

    #[test]
    fn test_gvfs_gio_virtual_mounts() {
        let mut gvfs = SovereignGvfsGioEngine::new();
        let mount_id = gvfs.mount_uri("smb://192.168.1.100/share", "/run/user/1000/gvfs/smb").unwrap();
        assert!(mount_id > 0);
        assert_eq!(gvfs.mounts.len(), 2);

        gvfs.g_file_write_all("/trash/item1.txt", b"deleted item");
        assert_eq!(gvfs.g_file_read_all("/trash/item1.txt"), Some(b"deleted item".to_vec()));
    }

    #[test]
    fn test_polkit_and_xdg_portals() {
        let mut polkit = SovereignPolicyKitAuthorizationEngine::new();
        let res = polkit.check_authorization("sigma_user", &["wheel"], "org.freedesktop.accounts.user-administration");
        assert_eq!(res, PolkitResult::Yes);

        let mut portal = SovereignXdgDesktopPortalEngine::new();
        let app_res = portal.xdg_open("https://sigmaos.org");
        assert!(app_res.unwrap().contains("browser"));

        portal.store_secret("wifi_pass", "secret123");
        assert_eq!(portal.retrieve_secret("wifi_pass"), Some("secret123".to_string()));
    }

    #[test]
    fn test_gtk_qt_palette_css_sync() {
        let mut engine = SovereignGtkQtToolkitEngine::new();
        engine.set_accent_color("#f5c2e7");
        engine.apply_css_stylesheet("button.primary", "background-color", "#89b4fa");
        assert_eq!(engine.accent_color_hex, "#f5c2e7");

        let palette = engine.sync_qt_to_gtk_palette();
        assert_eq!(palette.get("QPalette::Highlight"), Some(&"#f5c2e7".to_string()));
        assert!(!engine.event_trace_log.is_empty());
    }

    #[test]
    fn test_display_manager_greetd_pam() {
        let mut dm = SovereignDisplayManagerSuiteEngine::new(DisplayManagerBackend::Greetd);
        dm.set_auto_login("seat0", "autouser");
        assert!(dm.seats.get("seat0").unwrap().auto_login_enabled);

        let req = PamAuthRequest {
            username: "autouser".to_string(),
            password_hash: "NOPASSWD".to_string(),
            session_type: DesktopSessionType::Zenith,
            seat_id: "seat0".to_string(),
        };
        let res = dm.authenticate_pam(&req);
        assert!(res.authenticated);
        assert_eq!(res.session_env.get("XDG_SESSION_DESKTOP"), Some(&"zenith".to_string()));
    }

    #[test]
    fn test_gvfs_file_monitor_metadata() {
        let mut gvfs = SovereignGvfsGioEngine::new();
        gvfs.set_file_attribute("/doc.txt", "metadata::custom-icon", "file-text");
        assert_eq!(
            gvfs.get_file_attribute("/doc.txt", "metadata::custom-icon"),
            Some("file-text".to_string())
        );

        gvfs.g_file_write_all("/doc.txt", b"hello world");
        assert!(!gvfs.file_monitor_events.is_empty());
    }

    #[test]
    fn test_polkit_rules_and_subject_context() {
        let mut polkit = SovereignPolicyKitAuthorizationEngine::new();
        polkit.register_js_rule("50-default.rules", "polkit.addRule(function(action, subject) { return polkit.Result.YES; });");

        let subject = PolkitSubject {
            pid: 1234,
            uid: 1000,
            user_name: "admin_user".to_string(),
            groups: vec!["wheel".to_string()],
            selinux_context: Some("unconfined_u:unconfined_r:unconfined_t:s0".to_string()),
        };

        let res = polkit.check_authorization_subject(&subject, "org.freedesktop.accounts.user-administration", true);
        assert_eq!(res, PolkitResult::Yes);
    }

    #[test]
    fn test_xdg_portal_screencast_secret() {
        let mut portal = SovereignXdgDesktopPortalEngine::new();
        portal.parse_defaults_list("text/html=firefox.desktop\nimage/png=gthumb.desktop");
        let open_res = portal.xdg_open("https://example.com").unwrap();
        assert!(open_res.contains("firefox.desktop"));

        let stream = portal.create_screencast_session("org.obs.studio", true);
        assert!(stream.stream_node_id >= 4000);
        assert!(stream.is_screencast);
    }

    #[test]
    fn test_harfbuzz_opentype_shaping() {
        let mut font_engine = SovereignFontAndTextLayoutEngine::new();
        font_engine.parse_fontconfig_xml_rule("Helvetica", "sans-serif", &["DejaVuSans"]);
        let path = font_engine.match_font(&FontMatchRequest {
            family: "Helvetica".to_string(),
            weight: 400,
            size_pt: 12,
            antialias: true,
            hinting: HintingStyle::Full,
        });
        assert!(path.contains("DejaVuSans"));

        let count = font_engine.harfbuzz_shape_text_with_features("a => b", &["liga"]);
        assert_eq!(count, 5); // 'a', ' ', ligature '=>', ' ', 'b'
    }

    #[test]
    fn test_ime_candidate_pagination() {
        let mut ime = SovereignInputMethodFrameworkEngine::new(ImeFrameworkKind::Fcitx5);
        ime.set_surrounding_text("The result is ", 14);
        ime.process_key_event("n");
        ime.process_key_event("i");
        ime.process_key_event("h");
        ime.process_key_event("a");
        ime.process_key_event("o");
        assert_eq!(ime.candidates.len(), 6);

        let page1 = ime.get_paged_candidates();
        assert_eq!(page1.len(), 5);

        ime.process_key_event("Page_Down");
        let page2 = ime.get_paged_candidates();
        assert_eq!(page2.len(), 1);
        assert_eq!(page2[0].text, "倪好");
    }

    #[test]
    fn test_notifications_dnd_and_hints() {
        let mut notif = SovereignDesktopNotificationDaemonEngine::new();
        notif.dnd_mode = true;
        let _id1 = notif.notify("normal_app", 0, "Test", "Body", NotificationUrgency::Normal, 1000);
        assert_eq!(notif.active_notifications.len(), 0);

        notif.add_dnd_punch_through("vip_app");
        let id2 = notif.notify_with_hints("vip_app", 0, "VIP Call", "Incoming", NotificationUrgency::Normal, &[("accept", "Accept")], Some("phone-ring"), 5000);
        assert_eq!(notif.active_notifications.len(), 1);
        assert!(notif.invoke_action(id2, "accept"));
    }

    #[test]
    fn test_screen_locker_protocol() {
        let mut locker = SovereignScreenLockerEngine::new();
        locker.lock_session(1);
        assert!(locker.trigger_dpms_off());
        assert!(locker.dpms_turned_off);

        assert!(locker.authenticate_and_unlock("sigmaos_secret"));
        assert!(!locker.dpms_turned_off);
        assert_eq!(locker.lock_state, SessionLockState::Unlocked);
    }

    #[test]
    fn test_clipboard_multi_mime_secret_clear() {
        let mut clip = SovereignClipboardManagerEngine::new();
        clip.copy_data("text/html", b"<p>HTML Content</p>", "Browser", false);
        clip.copy_data("text/plain", b"MyPassword123", "KeePassXC", true);

        let search_res = clip.search_history("HTML");
        assert_eq!(search_res.len(), 1);

        let purged = clip.purge_secret_entries();
        assert_eq!(purged, 1);
    }

    #[test]
    fn test_master_suite_orchestrator() {
        let mut suite = SovereignDesktopToolkitsAndServicesSuite::new();
        let health = suite.run_health_check();
        assert_eq!(health.get("toolkit_engine"), Some(&true));
        assert_eq!(health.get("display_manager"), Some(&true));
        assert_eq!(health.get("polkit"), Some(&true));
    }
}
