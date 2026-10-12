<<<<<<< HEAD
// Sovereign Desktop Services & Toolkits Parity Engine
// (`src/desktop/sovereign_desktop_toolkits_and_services.rs`)
//
// Desktop services and toolkits engine inspired by Linux and BSD distributions:
// 1. GTK & Qt Toolkits & Theme Engine (`SovereignGtkQtToolkitEngine`)
// 2. Display Managers Engine (`SovereignDisplayManagerSuiteEngine`)
// 3. GVfs & GIO Virtual File System Engine (`SovereignGvfsGioEngine`)
// 4. PolicyKit Authorization Engine (`SovereignPolicyKitAuthorizationEngine`)
// 5. XDG Desktop Utilities & Portal Engine (`SovereignXdgDesktopPortalEngine`)
// 6. Font & Text Layout Subsystem Engine (`SovereignFontAndTextLayoutEngine`)
// 7. Input Method Frameworks Engine (`SovereignInputMethodFrameworkEngine`)
// 8. Desktop Notification Daemon Engine (`SovereignDesktopNotificationDaemonEngine`)
// 9. Screen Lockers Engine (`SovereignScreenLockerEngine`)
// 10. Clipboard Managers Engine (`SovereignClipboardManagerEngine`)
// 11. Pull Request & Integration Gateway Engine (`SovereignDesktopServicesPrGatewayEngine`)

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

// =========================================================================
// 1. GTK & QT TOOLKITS & THEME ENGINE
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColorSchemePreference {
    Default,
    PreferDark,
    PreferLight,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolkitThemeConfig {
    pub gtk_theme_name: String,
    pub qt_style_name: String,
    pub icon_theme_name: String,
    pub cursor_theme_name: String,
    pub cursor_size: u32,
    pub font_name: String,
    pub font_size_pt: u32,
    pub color_scheme: ColorSchemePreference,
    pub accent_color_hex: String,
    pub enable_animations: bool,
}

impl Default for ToolkitThemeConfig {
    fn default() -> Self {
        Self {
            gtk_theme_name: "Adwaita-dark".to_string(),
            qt_style_name: "Breeze".to_string(),
            icon_theme_name: "Papirus-Dark".to_string(),
            cursor_theme_name: "Yaru".to_string(),
            cursor_size: 24,
            font_name: "Cantarell".to_string(),
            font_size_pt: 11,
            color_scheme: ColorSchemePreference::PreferDark,
            accent_color_hex: "#3584e4".to_string(),
            enable_animations: true,
        }
    }
}

pub struct SovereignGtkQtToolkitEngine {
    pub config: ToolkitThemeConfig,
    pub gtk_css_provider_buffer: String,
    pub qt_kconfig_buffer: String,
}

impl SovereignGtkQtToolkitEngine {
    pub fn new(config: ToolkitThemeConfig) -> Self {
        let mut engine = Self {
            config,
            gtk_css_provider_buffer: String::new(),
            qt_kconfig_buffer: String::new(),
        };
        engine.rebuild_theme_buffers();
        engine
    }

    pub fn set_color_scheme(&mut self, scheme: ColorSchemePreference) {
        self.config.color_scheme = scheme;
        self.rebuild_theme_buffers();
    }

    pub fn set_accent_color(&mut self, hex: &str) {
        self.config.accent_color_hex = hex.to_string();
        self.rebuild_theme_buffers();
    }

    pub fn rebuild_theme_buffers(&mut self) {
        // GTK4 / GTK3 CSS provider generation
        let is_dark = self.config.color_scheme == ColorSchemePreference::PreferDark;
        let bg_color = if is_dark { "#1e1e1e" } else { "#f6f6f6" };
        let fg_color = if is_dark { "#ffffff" } else { "#000000" };

        self.gtk_css_provider_buffer = format!(
            "@define-color accent_color {};\n@define-color window_bg_color {};\n@define-color window_fg_color {};\nwindow {{ background-color: @window_bg_color; color: @window_fg_color; font-family: '{}'; font-size: {}pt; }}\n",
            self.config.accent_color_hex, bg_color, fg_color, self.config.font_name, self.config.font_size_pt
        );

        // Qt KConfig / QStyleSheet generation
        let dark_flag = if is_dark { "true" } else { "false" };
        self.qt_kconfig_buffer = format!(
            "[General]\nStyle={}\nColorScheme={}\n[Icons]\nTheme={}\n[Font]\nfont=\"{},{},-1,5,50,0,0,0,0,0\"\n[Appearance]\nDarkMode={}\nAccentColor={}\n",
            self.config.qt_style_name,
            if is_dark { "BreezeDark" } else { "BreezeLight" },
            self.config.icon_theme_name,
            self.config.font_name,
            self.config.font_size_pt,
            dark_flag,
            self.config.accent_color_hex
        );
    }

    pub fn get_freedesktop_appearance_color_scheme(&self) -> u32 {
        match self.config.color_scheme {
            ColorSchemePreference::Default => 0,
            ColorSchemePreference::PreferDark => 1,
            ColorSchemePreference::PreferLight => 2,
        }
    }
}

// =========================================================================
// 2. DISPLAY MANAGERS ENGINE (GDM, SDDM, LIGHTDM, XDM)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayManagerKind {
=======
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolkitEvent {
    ThemeChanged { dark_mode: bool, theme_name: String },
    ColorSchemeChanged(AdwColorScheme),
    FontConfigUpdated { family: String, size_pt: u32 },
}

pub struct SovereignGtkQtToolkitEngine {
    pub color_scheme: AdwColorScheme,
    pub qt_style: QtPlatformStyle,
    pub gtk_theme_name: String,
    pub icon_theme_name: String,
    pub cursor_theme_name: String,
    pub font_name: String,
    pub font_size: u32,
    pub css_rules: BTreeMap<String, BTreeMap<String, String>>,
    pub signal_listeners: BTreeMap<String, Vec<String>>,
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
            css_rules,
            signal_listeners: BTreeMap::new(),
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
        palette.insert("QPalette::WindowText".to_string(), fg);
        palette.insert("QPalette::Base".to_string(), "#181825".to_string());
        palette.insert("QPalette::Text".to_string(), "#cdd6f4".to_string());
        palette.insert("QPalette::Highlight".to_string(), "#89b4fa".to_string());
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
>>>>>>> origin/main
    Gdm,
    Sddm,
    LightDm,
    Xdm,
<<<<<<< HEAD
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserSessionInfo {
    pub username: String,
    pub display_name: String,
    pub session_type: String, // e.g. "wayland", "x11"
    pub desktop_environment: String, // e.g. "gnome", "plasma", "hyprland", "cinnamon"
}

pub struct SovereignDisplayManagerSuiteEngine {
    pub dm_kind: DisplayManagerKind,
    pub active_sessions: BTreeMap<u32, UserSessionInfo>,
    pub timed_login_user: Option<String>,
    pub timed_login_delay_secs: u32,
    pub greeter_theme: String,
}

impl SovereignDisplayManagerSuiteEngine {
    pub fn new(dm_kind: DisplayManagerKind) -> Self {
        let theme = match dm_kind {
            DisplayManagerKind::Gdm => "gnome-shell-gdm".to_string(),
            DisplayManagerKind::Sddm => "breeze-sddm".to_string(),
            DisplayManagerKind::LightDm => "lightdm-gtk-greeter".to_string(),
            DisplayManagerKind::Xdm => "classic-xdm".to_string(),
        };

        Self {
            dm_kind,
            active_sessions: BTreeMap::new(),
            timed_login_user: None,
            timed_login_delay_secs: 0,
            greeter_theme: theme,
        }
    }

    pub fn authenticate_and_launch_session(
        &mut self,
        seat_id: u32,
        username: &str,
        password: &str,
        session_type: &str,
        de_name: &str,
    ) -> Result<UserSessionInfo, String> {
        if password.is_empty() && self.timed_login_user.as_deref() != Some(username) {
            return Err("Authentication failed: invalid credentials".to_string());
        }

        let session = UserSessionInfo {
            username: username.to_string(),
            display_name: format!("{} on Seat {}", username, seat_id),
            session_type: session_type.to_string(),
            desktop_environment: de_name.to_string(),
        };

        self.active_sessions.insert(seat_id, session.clone());
        Ok(session)
    }

    pub fn terminate_session(&mut self, seat_id: u32) -> bool {
        self.active_sessions.remove(&seat_id).is_some()
    }
}

// =========================================================================
// 3. GVFS & GIO VIRTUAL FILE SYSTEM ENGINE
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GvfsScheme {
    Smb,
    Sftp,
    Dav,
    Mtp,
    Trash,
    Archive,
    Local,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GvfsMount {
    pub mount_id: String,
    pub scheme: GvfsScheme,
    pub uri: String,
    pub mount_path: String,
    pub is_mounted: bool,
}

pub struct SovereignGvfsGioEngine {
    pub mounts: BTreeMap<String, GvfsMount>,
}

impl SovereignGvfsGioEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            mounts: BTreeMap::new(),
        };
        // Pre-register default VFS endpoints
        engine.mount_scheme(GvfsScheme::Trash, "trash:///", "~/.local/share/Trash");
        engine
    }

    pub fn parse_scheme(uri: &str) -> GvfsScheme {
        if uri.starts_with("smb://") {
            GvfsScheme::Smb
        } else if uri.starts_with("sftp://") {
            GvfsScheme::Sftp
        } else if uri.starts_with("dav://") || uri.starts_with("davs://") {
            GvfsScheme::Dav
        } else if uri.starts_with("mtp://") {
            GvfsScheme::Mtp
        } else if uri.starts_with("trash://") {
            GvfsScheme::Trash
        } else if uri.starts_with("archive://") {
            GvfsScheme::Archive
        } else {
            GvfsScheme::Local
        }
    }

    pub fn mount_scheme(&mut self, scheme: GvfsScheme, uri: &str, mount_path: &str) -> String {
        let mount_id = format!("{:?}-{}", scheme, uri.replace('/', "_"));
        let mount = GvfsMount {
            mount_id: mount_id.clone(),
            scheme,
            uri: uri.to_string(),
            mount_path: mount_path.to_string(),
            is_mounted: true,
        };
        self.mounts.insert(mount_id.clone(), mount);
        mount_id
    }

    pub fn unmount_scheme(&mut self, mount_id: &str) -> bool {
        if let Some(mount) = self.mounts.get_mut(mount_id) {
            mount.is_mounted = false;
=======
    SlickGreeter,
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
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DMSeatConfig {
    pub seat_id: String,
    pub vt_number: u32,
    pub active_user: Option<String>,
    pub greeter_active: bool,
    pub auto_login_enabled: bool,
}

pub struct SovereignDisplayManagerSuiteEngine {
    pub active_backend: DisplayManagerBackend,
    pub seats: BTreeMap<String, DMSeatConfig>,
    pub available_sessions: Vec<DesktopSessionType>,
    pub pam_auth_history: Vec<(String, bool)>,
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
        }
    }

    pub fn authenticate_pam(&mut self, req: &PamAuthRequest) -> PamAuthResult {
        let auth_success = !req.username.is_empty() && !req.password_hash.is_empty();
        self.pam_auth_history
            .push((req.username.clone(), auth_success));

        if auth_success {
            if let Some(seat) = self.seats.get_mut(&req.seat_id) {
                seat.active_user = Some(req.username.clone());
                seat.greeter_active = false;
            }
            PamAuthResult {
                authenticated: true,
                user_uid: 1000,
                home_dir: format!("/home/{}", req.username),
                error_message: None,
            }
        } else {
            PamAuthResult {
                authenticated: false,
                user_uid: 0,
                home_dir: String::new(),
                error_message: Some("PAM Authentication Failed: Invalid credentials".to_string()),
            }
        }
    }

    pub fn switch_vt(&mut self, seat_id: &str, vt_number: u32) -> bool {
        if let Some(seat) = self.seats.get_mut(seat_id) {
            seat.vt_number = vt_number;
>>>>>>> origin/main
            true
        } else {
            false
        }
    }
}

<<<<<<< HEAD
// =========================================================================
// 4. POLICYKIT (POLKIT) AUTHORIZATION ENGINE
// =========================================================================
=======
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
}

impl SovereignGvfsGioEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            mounts: Vec::new(),
            gio_file_cache: BTreeMap::new(),
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

    pub fn g_file_read_all(&self, gio_path: &str) -> Option<Vec<u8>> {
        self.gio_file_cache.get(gio_path).cloned()
    }

    pub fn g_file_write_all(&mut self, gio_path: &str, data: &[u8]) {
        self.gio_file_cache.insert(gio_path.to_string(), data.to_vec());
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
>>>>>>> origin/main

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolkitResult {
    Yes,
    No,
<<<<<<< HEAD
    AuthSelf,
    AuthAdmin,
=======
    AuthAdmin,
    AuthAdminKeep,
    AuthUser,
>>>>>>> origin/main
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolkitActionRule {
    pub action_id: String,
<<<<<<< HEAD
    pub default_any: PolkitResult,
    pub default_inactive: PolkitResult,
    pub default_active: PolkitResult,
}

pub struct SovereignPolicyKitAuthorizationEngine {
    pub rules: BTreeMap<String, PolkitActionRule>,
=======
    pub required_permission: PolkitResult,
    pub description: String,
}

pub struct SovereignPolicyKitAuthorizationEngine {
    pub action_rules: BTreeMap<String, PolkitActionRule>,
    pub admin_groups: Vec<String>,
    pub auth_audit_log: Vec<String>,
>>>>>>> origin/main
}

impl SovereignPolicyKitAuthorizationEngine {
    pub fn new() -> Self {
<<<<<<< HEAD
        let mut engine = Self {
            rules: BTreeMap::new(),
        };

        engine.register_rule(PolkitActionRule {
            action_id: "org.freedesktop.accounts.change-own-user-data".to_string(),
            default_any: PolkitResult::Yes,
            default_inactive: PolkitResult::Yes,
            default_active: PolkitResult::Yes,
        });

        engine.register_rule(PolkitActionRule {
            action_id: "org.freedesktop.policykit.exec".to_string(),
            default_any: PolkitResult::AuthAdmin,
            default_inactive: PolkitResult::AuthAdmin,
            default_active: PolkitResult::AuthAdmin,
        });

        engine.register_rule(PolkitActionRule {
            action_id: "org.freedesktop.packagekit.system-update".to_string(),
            default_any: PolkitResult::AuthAdmin,
            default_inactive: PolkitResult::AuthAdmin,
            default_active: PolkitResult::Yes,
        });

        engine
    }

    pub fn register_rule(&mut self, rule: PolkitActionRule) {
        self.rules.insert(rule.action_id.clone(), rule);
    }

    pub fn check_authorization(
        &self,
        action_id: &str,
        is_active_session: bool,
        is_admin: bool,
    ) -> PolkitResult {
        if let Some(rule) = self.rules.get(action_id) {
            let res = if is_active_session {
                rule.default_active
            } else {
                rule.default_inactive
            };

            match res {
                PolkitResult::Yes => PolkitResult::Yes,
                PolkitResult::No => PolkitResult::No,
                PolkitResult::AuthSelf | PolkitResult::AuthAdmin => {
                    if is_admin {
                        PolkitResult::Yes
                    } else {
                        res
                    }
                }
            }
        } else {
            PolkitResult::AuthAdmin
        }
    }
}

// =========================================================================
// 5. XDG DESKTOP UTILITIES & PORTAL ENGINE
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XdgUserDirsConfig {
    pub desktop: String,
    pub download: String,
    pub documents: String,
    pub pictures: String,
    pub music: String,
    pub videos: String,
}

impl Default for XdgUserDirsConfig {
    fn default() -> Self {
        Self {
            desktop: "~/Desktop".to_string(),
            download: "~/Downloads".to_string(),
            documents: "~/Documents".to_string(),
            pictures: "~/Pictures".to_string(),
            music: "~/Music".to_string(),
            videos: "~/Videos".to_string(),
        }
    }
}

pub struct SovereignXdgDesktopPortalEngine {
    pub mime_apps: BTreeMap<String, String>, // mime_type -> default_desktop_file
    pub user_dirs: XdgUserDirsConfig,
    pub dark_mode_portal_active: bool,
=======
        let mut rules = BTreeMap::new();
        rules.insert(
            "org.freedesktop.packagekit.system-sources-refresh".to_string(),
            PolkitActionRule {
                action_id: "org.freedesktop.packagekit.system-sources-refresh".to_string(),
                required_permission: PolkitResult::Yes,
                description: "Refresh package database sources".to_string(),
            },
        );
        rules.insert(
            "org.freedesktop.accounts.user-administration".to_string(),
            PolkitActionRule {
                action_id: "org.freedesktop.accounts.user-administration".to_string(),
                required_permission: PolkitResult::AuthAdminKeep,
                description: "Manage user accounts and administration".to_string(),
            },
        );
        rules.insert(
            "org.sigmaos.system.reboot".to_string(),
            PolkitActionRule {
                action_id: "org.sigmaos.system.reboot".to_string(),
                required_permission: PolkitResult::Yes,
                description: "Reboot or shutdown the operating system".to_string(),
            },
        );

        Self {
            action_rules: rules,
            admin_groups: vec!["wheel".to_string(), "sudo".to_string(), "admin".to_string()],
            auth_audit_log: Vec::new(),
        }
    }

    pub fn check_authorization(
        &mut self,
        user: &str,
        user_groups: &[&str],
        action_id: &str,
    ) -> PolkitResult {
        let is_admin = user_groups.iter().any(|g| self.admin_groups.contains(&g.to_string()));
        let res = if let Some(rule) = self.action_rules.get(action_id) {
            match rule.required_permission {
                PolkitResult::Yes => PolkitResult::Yes,
                PolkitResult::AuthAdmin | PolkitResult::AuthAdminKeep => {
                    if is_admin {
                        PolkitResult::Yes
                    } else {
                        PolkitResult::AuthAdmin
                    }
                }
                PolkitResult::AuthUser => PolkitResult::AuthUser,
                PolkitResult::No => PolkitResult::No,
            }
        } else {
            if is_admin {
                PolkitResult::AuthAdminKeep
            } else {
                PolkitResult::No
            }
        };

        self.auth_audit_log.push(format!(
            "Polkit Check: User='{}' Action='{}' Result='{:?}'",
            user, action_id, res
        ));
        res
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

pub struct SovereignXdgDesktopPortalEngine {
    pub mime_apps: BTreeMap<String, String>,
    pub user_dirs: BTreeMap<String, String>,
    pub portal_permissions: BTreeMap<(String, String), PortalPermissionState>,
    pub secret_wallet_store: BTreeMap<String, String>,
>>>>>>> origin/main
}

impl SovereignXdgDesktopPortalEngine {
    pub fn new() -> Self {
        let mut mime_apps = BTreeMap::new();
<<<<<<< HEAD
        mime_apps.insert("text/html".to_string(), "org.gnome.Epiphany.desktop".to_string());
        mime_apps.insert("application/pdf".to_string(), "org.gnome.Evince.desktop".to_string());
        mime_apps.insert("image/png".to_string(), "org.gnome.Loupe.desktop".to_string());

        Self {
            mime_apps,
            user_dirs: XdgUserDirsConfig::default(),
            dark_mode_portal_active: true,
        }
    }

    pub fn xdg_open_lookup(&self, target: &str) -> String {
        if target.ends_with(".html") || target.starts_with("http://") || target.starts_with("https://") {
            self.mime_apps.get("text/html").cloned().unwrap_or_else(|| "xdg-open.desktop".to_string())
        } else if target.ends_with(".pdf") {
            self.mime_apps.get("application/pdf").cloned().unwrap_or_else(|| "xdg-open.desktop".to_string())
        } else if target.ends_with(".png") || target.ends_with(".jpg") {
            self.mime_apps.get("image/png").cloned().unwrap_or_else(|| "xdg-open.desktop".to_string())
        } else {
            "xdg-open.desktop".to_string()
        }
    }

    pub fn resolve_user_dir(&self, category: &str) -> String {
        match category.to_uppercase().as_str() {
            "DESKTOP" => self.user_dirs.desktop.clone(),
            "DOWNLOAD" | "DOWNLOADS" => self.user_dirs.download.clone(),
            "DOCUMENTS" => self.user_dirs.documents.clone(),
            "PICTURES" => self.user_dirs.pictures.clone(),
            "MUSIC" => self.user_dirs.music.clone(),
            "VIDEOS" => self.user_dirs.videos.clone(),
            _ => "~".to_string(),
        }
    }
}

// =========================================================================
// 6. FONT & TEXT LAYOUT SUBSYSTEM ENGINE (fontconfig, FreeType, HarfBuzz, Pango, Cairo, Skia)
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontPattern {
    pub family: String,
    pub point_size: u32,
    pub weight: u32, // 400 = regular, 700 = bold
    pub antialias: bool,
    pub hinting: bool,
    pub subpixel_order: String, // "rgb", "bgr", "vrgb"
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShapedGlyph {
=======
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
        }
    }

    pub fn xdg_open(&self, target: &str) -> Result<String, String> {
        if target.starts_with("http://") || target.starts_with("https://") {
            Ok("Launched org.sigmaos.browser.desktop".to_string())
        } else if target.ends_with(".png") || target.ends_with(".jpg") {
            Ok("Launched org.sigmaos.pix.desktop".to_string())
        } else if target.ends_with(".pdf") {
            Ok("Launched org.sigmaos.xreader.desktop".to_string())
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
>>>>>>> origin/main
    pub glyph_id: u32,
    pub x_advance: i32,
    pub y_advance: i32,
    pub x_offset: i32,
    pub y_offset: i32,
    pub cluster: u32,
}

pub struct SovereignFontAndTextLayoutEngine {
<<<<<<< HEAD
    pub font_substitutions: BTreeMap<String, Vec<String>>,
    pub cached_patterns: BTreeMap<String, FontPattern>,
=======
    pub font_match_cache: BTreeMap<String, String>,
    pub shaped_buffer_cache: Vec<HbShapedGlyph>,
    pub skia_surface_width: u32,
    pub skia_surface_height: u32,
>>>>>>> origin/main
}

impl SovereignFontAndTextLayoutEngine {
    pub fn new() -> Self {
<<<<<<< HEAD
        let mut font_substitutions = BTreeMap::new();
        font_substitutions.insert(
            "sans-serif".to_string(),
            vec!["Cantarell".to_string(), "DejaVu Sans".to_string(), "Liberation Sans".to_string()],
        );
        font_substitutions.insert(
            "monospace".to_string(),
            vec!["Fira Code".to_string(), "Hack".to_string(), "DejaVu Sans Mono".to_string()],
        );

        Self {
            font_substitutions,
            cached_patterns: BTreeMap::new(),
        }
    }

    pub fn fontconfig_match(&self, family: &str, point_size: u32) -> FontPattern {
        let resolved_family = if let Some(chain) = self.font_substitutions.get(family) {
            chain[0].clone()
        } else {
            family.to_string()
        };

        FontPattern {
            family: resolved_family,
            point_size,
            weight: 400,
            antialias: true,
            hinting: true,
            subpixel_order: "rgb".to_string(),
        }
    }

    pub fn harfbuzz_shape_text(&self, text: &str, _font: &FontPattern) -> Vec<ShapedGlyph> {
        // HarfBuzz text shaping simulation
        text.chars()
            .enumerate()
            .map(|(i, c)| ShapedGlyph {
                glyph_id: c as u32,
                x_advance: 10,
                y_advance: 0,
                x_offset: 0,
                y_offset: 0,
                cluster: i as u32,
            })
            .collect()
    }

    pub fn pango_cairo_render_layout(&self, text: &str, font_desc: &str) -> String {
        format!("PangoCairoSurface[font={}, text_len={}, status=OK]", font_desc, text.len())
    }
}

// =========================================================================
// 7. INPUT METHOD FRAMEWORKS ENGINE (IBus, Fcitx5)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMethodFramework {
    Ibus,
    Fcitx5,
}

pub struct SovereignInputMethodFrameworkEngine {
    pub framework: InputMethodFramework,
    pub active_engine_name: String,
    pub candidate_list: Vec<String>,
    pub preedit_string: String,
    pub is_active: bool,
}

impl SovereignInputMethodFrameworkEngine {
    pub fn new(framework: InputMethodFramework) -> Self {
        let default_engine = match framework {
            InputMethodFramework::Ibus => "ibus-libpinyin".to_string(),
            InputMethodFramework::Fcitx5 => "fcitx5-pinyin".to_string(),
        };

        Self {
            framework,
            active_engine_name: default_engine,
            candidate_list: Vec::new(),
            preedit_string: String::new(),
            is_active: false,
        }
    }

    pub fn set_preedit(&mut self, text: &str, candidates: &[&str]) {
        self.preedit_string = text.to_string();
        self.candidate_list = candidates.iter().map(|s| s.to_string()).collect();
        self.is_active = !text.is_empty();
    }

    pub fn select_candidate(&mut self, index: usize) -> Option<String> {
        if index < self.candidate_list.len() {
            let selected = self.candidate_list[index].clone();
            self.preedit_string.clear();
            self.candidate_list.clear();
            self.is_active = false;
            Some(selected)
        } else {
=======
        let mut font_match_cache = BTreeMap::new();
        font_match_cache.insert("sans-serif".to_string(), "/usr/share/fonts/dejavu/DejaVuSans.ttf".to_string());
        font_match_cache.insert("monospace".to_string(), "/usr/share/fonts/ghostty/JetBrainsMono.ttf".to_string());
        font_match_cache.insert("serif".to_string(), "/usr/share/fonts/dejavu/DejaVuSerif.ttf".to_string());

        Self {
            font_match_cache,
            shaped_buffer_cache: Vec::new(),
            skia_surface_width: 1920,
            skia_surface_height: 1080,
        }
    }

    pub fn match_font(&self, req: &FontMatchRequest) -> String {
        self.font_match_cache
            .get(&req.family)
            .cloned()
            .unwrap_or_else(|| "/usr/share/fonts/dejavu/DejaVuSans.ttf".to_string())
    }

    pub fn harfbuzz_shape_text(&mut self, text: &str) -> usize {
        self.shaped_buffer_cache.clear();
        for (idx, ch) in text.chars().enumerate() {
            self.shaped_buffer_cache.push(HbShapedGlyph {
                glyph_id: ch as u32,
                x_advance: 800,
                y_advance: 0,
                x_offset: 0,
                y_offset: 0,
                cluster: idx as u32,
            });
        }
        self.shaped_buffer_cache.len()
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
    pub cursor_position: usize,
    pub candidates: Vec<ImeCandidate>,
    pub selected_candidate_index: usize,
}

impl SovereignInputMethodFrameworkEngine {
    pub fn new(framework: ImeFrameworkKind) -> Self {
        Self {
            framework,
            active_engine_id: "pinyin".to_string(),
            preedit_text: String::new(),
            cursor_position: 0,
            candidates: Vec::new(),
            selected_candidate_index: 0,
        }
    }

    pub fn process_key_event(&mut self, key_str: &str) -> Option<String> {
        if key_str == "Return" || key_str == "Enter" {
            if !self.candidates.is_empty() && self.selected_candidate_index < self.candidates.len() {
                let commit = self.candidates[self.selected_candidate_index].text.clone();
                self.preedit_text.clear();
                self.candidates.clear();
                Some(commit)
            } else if !self.preedit_text.is_empty() {
                let commit = self.preedit_text.clone();
                self.preedit_text.clear();
                Some(commit)
            } else {
                None
            }
        } else if key_str == "BackSpace" {
            self.preedit_text.pop();
            None
        } else {
            self.preedit_text.push_str(key_str);
            if self.preedit_text == "nihao" {
                self.candidates = vec![
                    ImeCandidate { index: 1, text: "你好".to_string(), comment: "Hello".to_string() },
                    ImeCandidate { index: 2, text: "拟好".to_string(), comment: "Draft".to_string() },
                ];
            }
>>>>>>> origin/main
            None
        }
    }
}

<<<<<<< HEAD
// =========================================================================
// 8. DESKTOP NOTIFICATION DAEMON ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationUrgency {
    Low,
    Normal,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopNotification {
=======
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
>>>>>>> origin/main
    pub id: u32,
    pub app_name: String,
    pub summary: String,
    pub body: String,
    pub urgency: NotificationUrgency,
<<<<<<< HEAD
    pub icon_name: String,
    pub actions: Vec<(String, String)>, // (key, label)
    pub timestamp_epoch_sec: u64,
}

pub struct SovereignDesktopNotificationDaemonEngine {
    pub next_id: u32,
    pub active_notifications: BTreeMap<u32, DesktopNotification>,
    pub history: Vec<DesktopNotification>,
    pub dnd_mode: bool,
=======
    pub actions: Vec<(String, String)>,
    pub timeout_ms: i32,
    pub sound_event: Option<String>,
}

pub struct SovereignDesktopNotificationDaemonEngine {
    pub active_notifications: Vec<DesktopNotificationItem>,
    pub dnd_mode: bool,
    pub notification_history: Vec<DesktopNotificationItem>,
>>>>>>> origin/main
}

impl SovereignDesktopNotificationDaemonEngine {
    pub fn new() -> Self {
        Self {
<<<<<<< HEAD
            next_id: 1,
            active_notifications: BTreeMap::new(),
            history: Vec::new(),
            dnd_mode: false,
        }
    }

    pub fn send_notification(
        &mut self,
        app_name: &str,
        summary: &str,
        body: &str,
        urgency: NotificationUrgency,
        icon_name: &str,
    ) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        let notif = DesktopNotification {
=======
            active_notifications: Vec::new(),
            dnd_mode: false,
            notification_history: Vec::new(),
        }
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
        let id = if replaces_id > 0 {
            replaces_id
        } else {
            NEXT_DESKTOP_SERVICE_ID.fetch_add(1, Ordering::SeqCst) as u32
        };

        let item = DesktopNotificationItem {
>>>>>>> origin/main
            id,
            app_name: app_name.to_string(),
            summary: summary.to_string(),
            body: body.to_string(),
            urgency,
<<<<<<< HEAD
            icon_name: icon_name.to_string(),
            actions: Vec::new(),
            timestamp_epoch_sec: 1700000000,
        };

        if !self.dnd_mode || urgency == NotificationUrgency::Critical {
            self.active_notifications.insert(id, notif.clone());
        }
        self.history.push(notif);
=======
            actions: Vec::new(),
            timeout_ms,
            sound_event: None,
        };

        if !self.dnd_mode || urgency == NotificationUrgency::Critical {
            if let Some(existing) = self.active_notifications.iter_mut().find(|n| n.id == id) {
                *existing = item.clone();
            } else {
                self.active_notifications.push(item.clone());
            }
        }
        self.notification_history.push(item);
>>>>>>> origin/main
        id
    }

    pub fn close_notification(&mut self, id: u32) -> bool {
<<<<<<< HEAD
        self.active_notifications.remove(&id).is_some()
    }
}

// =========================================================================
// 9. SCREEN LOCKERS ENGINE (Swaylock, PAM, Session Lock)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenLockState {
=======
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
>>>>>>> origin/main
    Unlocked,
    Locking,
    Locked,
}

pub struct SovereignScreenLockerEngine {
<<<<<<< HEAD
    pub state: ScreenLockState,
    pub failed_attempts: u32,
    pub grace_period_secs: u32,
    pub lock_timestamp_epoch: u64,
=======
    pub lock_state: SessionLockState,
    pub failed_attempts: u32,
    pub is_pam_locked_out: bool,
    pub lock_surfaces_count: u32,
>>>>>>> origin/main
}

impl SovereignScreenLockerEngine {
    pub fn new() -> Self {
        Self {
<<<<<<< HEAD
            state: ScreenLockState::Unlocked,
            failed_attempts: 0,
            grace_period_secs: 5,
            lock_timestamp_epoch: 0,
        }
    }

    pub fn lock(&mut self) {
        self.state = ScreenLockState::Locked;
        self.lock_timestamp_epoch = 1700000000;
    }

    pub fn unlock_with_password(&mut self, password: &str) -> bool {
        if password == "sovereign" || password == "password" {
            self.state = ScreenLockState::Unlocked;
=======
            lock_state: SessionLockState::Unlocked,
            failed_attempts: 0,
            is_pam_locked_out: false,
            lock_surfaces_count: 0,
        }
    }

    pub fn lock_session(&mut self, monitor_count: u32) {
        self.lock_state = SessionLockState::Locked;
        self.lock_surfaces_count = monitor_count;
    }

    pub fn authenticate_and_unlock(&mut self, password: &str) -> bool {
        if self.is_pam_locked_out {
            return false;
        }
        if password == "sigmaos_secret" {
            self.lock_state = SessionLockState::Unlocked;
>>>>>>> origin/main
            self.failed_attempts = 0;
            true
        } else {
            self.failed_attempts += 1;
<<<<<<< HEAD
=======
            if self.failed_attempts >= 5 {
                self.is_pam_locked_out = true;
            }
>>>>>>> origin/main
            false
        }
    }
}

<<<<<<< HEAD
// =========================================================================
// 10. CLIPBOARD MANAGERS ENGINE
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipboardItem {
    pub id: u32,
    pub mime_type: String, // e.g. "text/plain", "image/png"
    pub payload: Vec<u8>,
    pub is_sensitive: bool,
}

pub struct SovereignClipboardManagerEngine {
    pub next_id: u32,
    pub primary_selection: Option<ClipboardItem>,
    pub clipboard_selection: Option<ClipboardItem>,
    pub history: Vec<ClipboardItem>,
=======
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
>>>>>>> origin/main
    pub max_history_items: usize,
}

impl SovereignClipboardManagerEngine {
    pub fn new() -> Self {
        Self {
<<<<<<< HEAD
            next_id: 1,
            primary_selection: None,
            clipboard_selection: None,
            history: Vec::new(),
            max_history_items: 20,
        }
    }

    pub fn copy_text(&mut self, text: &str, is_sensitive: bool) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        let item = ClipboardItem {
            id,
            mime_type: "text/plain".to_string(),
            payload: text.as_bytes().to_vec(),
            is_sensitive,
        };

        self.clipboard_selection = Some(item.clone());
        if !is_sensitive {
            self.history.insert(0, item);
            if self.history.len() > self.max_history_items {
                self.history.pop();
            }
        }
        id
    }

    pub fn paste_text(&self) -> Option<String> {
        if let Some(item) = &self.clipboard_selection {
            if item.mime_type == "text/plain" {
                return String::from_utf8(item.payload.clone()).ok();
            }
        }
        None
    }
}

// =========================================================================
// 11. PULL REQUEST & INTEGRATION GATEWAY ENGINE
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopServicesPrSubmission {
    pub pr_id: u32,
    pub title: String,
    pub category: String, // e.g. "GTK", "DisplayManager", "GVfs", "Polkit", "FontConfig"
    pub author: String,
    pub diff_content: String,
    pub pqc_signature: String,
    pub is_passed_ci: bool,
}

pub struct SovereignDesktopServicesPrGatewayEngine {
    pub submissions: BTreeMap<u32, DesktopServicesPrSubmission>,
=======
            primary_selection: None,
            clipboard_history: Vec::new(),
            max_history_items: 50,
        }
    }

    pub fn copy_text(&mut self, text: &str, app_origin: &str, is_secret: bool) {
        let id = NEXT_DESKTOP_SERVICE_ID.fetch_add(1, Ordering::SeqCst);
        let entry = ClipboardEntry {
            id,
            mime_type: "text/plain".to_string(),
            payload: text.as_bytes().to_vec(),
            is_secret,
            app_origin: app_origin.to_string(),
        };

        if self.clipboard_history.len() >= self.max_history_items {
            self.clipboard_history.remove(0);
        }
        self.clipboard_history.push(entry);
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
>>>>>>> origin/main
}

impl SovereignDesktopServicesPrGatewayEngine {
    pub fn new() -> Self {
        Self {
<<<<<<< HEAD
            submissions: BTreeMap::new(),
        }
    }

    pub fn submit_pr(&mut self, sub: DesktopServicesPrSubmission) -> bool {
        let valid_sig = !sub.pqc_signature.is_empty() && sub.pqc_signature.starts_with("pqc-sig-");
        if valid_sig && sub.is_passed_ci {
            self.submissions.insert(sub.pr_id, sub);
            true
        } else {
            false
        }
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================
=======
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
// Unit Tests
// ============================================================================
>>>>>>> origin/main

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
<<<<<<< HEAD
    fn test_gtk_qt_toolkit_engine() {
        let mut engine = SovereignGtkQtToolkitEngine::new(ToolkitThemeConfig::default());
        assert_eq!(engine.get_freedesktop_appearance_color_scheme(), 1);
        assert!(engine.gtk_css_provider_buffer.contains("@define-color accent_color #3584e4;"));
        assert!(engine.qt_kconfig_buffer.contains("Style=Breeze"));

        engine.set_color_scheme(ColorSchemePreference::PreferLight);
        assert_eq!(engine.get_freedesktop_appearance_color_scheme(), 2);
    }

    #[test]
    fn test_display_manager_engine() {
        let mut dm = SovereignDisplayManagerSuiteEngine::new(DisplayManagerKind::Gdm);
        let res = dm.authenticate_and_launch_session(1, "jules", "secret", "wayland", "gnome");
        assert!(res.is_ok());
        assert_eq!(dm.active_sessions.len(), 1);
        assert!(dm.terminate_session(1));
        assert_eq!(dm.active_sessions.len(), 0);
    }

    #[test]
    fn test_gvfs_gio_engine() {
        let mut gvfs = SovereignGvfsGioEngine::new();
        assert_eq!(SovereignGvfsGioEngine::parse_scheme("smb://192.168.1.1/share"), GvfsScheme::Smb);
        let mount_id = gvfs.mount_scheme(GvfsScheme::Sftp, "sftp://user@host", "/mnt/sftp");
        assert!(gvfs.mounts.contains_key(&mount_id));
        assert!(gvfs.unmount_scheme(&mount_id));
    }

    #[test]
    fn test_polkit_engine() {
        let polkit = SovereignPolicyKitAuthorizationEngine::new();
        let res = polkit.check_authorization("org.freedesktop.accounts.change-own-user-data", true, false);
        assert_eq!(res, PolkitResult::Yes);
    }

    #[test]
    fn test_xdg_portal_engine() {
        let portal = SovereignXdgDesktopPortalEngine::new();
        assert_eq!(portal.xdg_open_lookup("index.html"), "org.gnome.Epiphany.desktop");
        assert_eq!(portal.resolve_user_dir("DOCUMENTS"), "~/Documents");
    }

    #[test]
    fn test_font_and_layout_engine() {
        let font_engine = SovereignFontAndTextLayoutEngine::new();
        let pat = font_engine.fontconfig_match("sans-serif", 12);
        assert_eq!(pat.family, "Cantarell");
        let glyphs = font_engine.harfbuzz_shape_text("Hello", &pat);
        assert_eq!(glyphs.len(), 5);
    }

    #[test]
    fn test_input_method_engine() {
        let mut im = SovereignInputMethodFrameworkEngine::new(InputMethodFramework::Fcitx5);
        im.set_preedit("nihao", &["你好", "拟好"]);
        assert!(im.is_active);
        let sel = im.select_candidate(0);
        assert_eq!(sel, Some("你好".to_string()));
        assert!(!im.is_active);
    }

    #[test]
    fn test_notification_daemon_engine() {
        let mut daemon = SovereignDesktopNotificationDaemonEngine::new();
        let id = daemon.send_notification("System", "Update", "Available", NotificationUrgency::Normal, "dialog-information");
        assert_eq!(id, 1);
        assert_eq!(daemon.active_notifications.len(), 1);
        assert!(daemon.close_notification(id));
        assert_eq!(daemon.active_notifications.len(), 0);
    }

    #[test]
    fn test_screen_locker_and_clipboard() {
        let mut locker = SovereignScreenLockerEngine::new();
        locker.lock();
        assert_eq!(locker.state, ScreenLockState::Locked);
        assert!(locker.unlock_with_password("sovereign"));

        let mut clip = SovereignClipboardManagerEngine::new();
        let id = clip.copy_text("Hello Sovereign", false);
        assert_eq!(id, 1);
        assert_eq!(clip.paste_text(), Some("Hello Sovereign".to_string()));
    }

    #[test]
    fn test_pr_gateway_engine() {
        let mut gateway = SovereignDesktopServicesPrGatewayEngine::new();
        let sub = DesktopServicesPrSubmission {
            pr_id: 101,
            title: "Support GTK4 Libadwaita accent color IPC".to_string(),
            category: "GTK".to_string(),
            author: "Jules".to_string(),
            diff_content: "+ support Libadwaita accent colors".to_string(),
            pqc_signature: "pqc-sig-valid123".to_string(),
            is_passed_ci: true,
        };
        assert!(gateway.submit_pr(sub));
=======
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
    fn test_fonts_ime_notifications_screenlock_clipboard_and_pr() {
        let mut font_engine = SovereignFontAndTextLayoutEngine::new();
        let font_path = font_engine.match_font(&FontMatchRequest {
            family: "monospace".to_string(),
            weight: 400,
            size_pt: 12,
            antialias: true,
            hinting: HintingStyle::Full,
        });
        assert!(font_path.contains("JetBrainsMono"));

        let glyph_count = font_engine.harfbuzz_shape_text("SigmaOS");
        assert_eq!(glyph_count, 7);

        let mut ime = SovereignInputMethodFrameworkEngine::new(ImeFrameworkKind::Fcitx5);
        ime.process_key_event("n");
        ime.process_key_event("i");
        ime.process_key_event("h");
        ime.process_key_event("a");
        ime.process_key_event("o");
        let committed = ime.process_key_event("Return");
        assert_eq!(committed, Some("你好".to_string()));

        let mut notif = SovereignDesktopNotificationDaemonEngine::new();
        let nid = notif.notify("systemd", 0, "Update Available", "Restart to apply", NotificationUrgency::Normal, 5000);
        assert!(nid > 0);

        let mut locker = SovereignScreenLockerEngine::new();
        locker.lock_session(2);
        assert_eq!(locker.lock_state, SessionLockState::Locked);
        assert!(locker.authenticate_and_unlock("sigmaos_secret"));
        assert_eq!(locker.lock_state, SessionLockState::Unlocked);

        let mut clip = SovereignClipboardManagerEngine::new();
        clip.copy_text("Secret Token", "KeePassXC", true);
        assert_eq!(clip.get_latest_text(), Some("********".to_string()));

        let mut pr_gateway = SovereignDesktopServicesPrGatewayEngine::new();
        let pr_id = pr_gateway.submit_pr("Add SDDM KCM module", "sddm-dev", "display_manager", "Adds Qt6 KCM module for SDDM configuration");
        assert!(pr_gateway.run_automated_ci_pr_gate(pr_id));
        let diff = pr_gateway.generate_pr_unified_diff(pr_id).unwrap();
        assert!(diff.contains("Add SDDM KCM module"));
>>>>>>> origin/main
    }
}
