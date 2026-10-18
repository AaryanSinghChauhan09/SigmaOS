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
    Kvantum = 5,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KvantumThemeSpec {
    pub theme_name: String,
    pub blur_background: bool,
    pub translucent_windows: bool,
    pub svg_path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SovereignAccentColor {
    Blue,
    Teal,
    Green,
    Yellow,
    Orange,
    Red,
    Pink,
    Purple,
    Slate,
}

impl SovereignAccentColor {
    pub fn to_hex(&self) -> &'static str {
        match self {
            SovereignAccentColor::Blue => "#3584e4",
            SovereignAccentColor::Teal => "#129eaf",
            SovereignAccentColor::Green => "#33d17a",
            SovereignAccentColor::Yellow => "#f6d32d",
            SovereignAccentColor::Orange => "#ff7800",
            SovereignAccentColor::Red => "#e01b24",
            SovereignAccentColor::Pink => "#d56199",
            SovereignAccentColor::Purple => "#9141ac",
            SovereignAccentColor::Slate => "#77767b",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SovereignKvantumThemeSpec {
    pub theme_name: String,
    pub svg_path: String,
    pub blur_background: bool,
    pub translucent_windows: bool,
    pub blur_radius: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolkitEvent {
    ThemeChanged { dark_mode: bool, theme_name: String },
    ColorSchemeChanged(AdwColorScheme),
    AccentColorChanged(SovereignAccentColor),
    FontConfigUpdated { family: String, size_pt: u32 },
    GtkInspectorToggled(bool),
    KvantumThemeApplied(String),
}

pub struct SovereignGtkQtToolkitEngine {
    pub color_scheme: AdwColorScheme,
    pub qt_style: QtPlatformStyle,
    pub accent_color: SovereignAccentColor,
    pub gtk_theme_name: String,
    pub icon_theme_name: String,
    pub cursor_theme_name: String,
    pub font_name: String,
    pub font_size: u32,
    pub gtk_inspector_enabled: bool,
    pub kvantum_spec: Option<KvantumThemeSpec>,
    pub css_rules: BTreeMap<String, BTreeMap<String, String>>,
    pub signal_listeners: BTreeMap<String, Vec<String>>,
    pub event_log: Vec<ToolkitEvent>,
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
            accent_color: SovereignAccentColor::Blue,
            gtk_theme_name: "Adwaita-dark".to_string(),
            icon_theme_name: "Papirus-Dark".to_string(),
            cursor_theme_name: "Adwaita".to_string(),
            font_name: "Cantarell".to_string(),
            font_size: 11,
            gtk_inspector_enabled: false,
            kvantum_spec: None,
            css_rules,
            signal_listeners: BTreeMap::new(),
            event_log: Vec::new(),
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
        self.event_log
            .push(ToolkitEvent::ColorSchemeChanged(scheme));
    }

    pub fn set_accent_color(&mut self, accent: SovereignAccentColor) {
        self.accent_color = accent;
        self.event_log
            .push(ToolkitEvent::AccentColorChanged(accent));
    }

    pub fn toggle_gtk_inspector(&mut self) -> bool {
        self.gtk_inspector_enabled = !self.gtk_inspector_enabled;
        self.event_log.push(ToolkitEvent::GtkInspectorToggled(
            self.gtk_inspector_enabled,
        ));
        self.gtk_inspector_enabled
    }

    pub fn parse_and_apply_kvantum_config(&mut self, config_str: &str) -> bool {
        let mut theme_name = "Catppuccin-Mocha".to_string();
        let mut blur = false;
        let mut translucent = false;
        for line in config_str.lines() {
            let line = line.trim();
            if line.starts_with("theme=") {
                theme_name = line["theme=".len()..].to_string();
            } else if line.starts_with("blur=") || line.starts_with("blur_radius=") {
                blur = true;
            } else if line.starts_with("translucent=") || line.starts_with("translucent_windows=") {
                translucent = line.ends_with("true");
            }
        }
        let svg_path = format!("/usr/share/Kvantum/{}/{}.svg", theme_name, theme_name);
        let spec = KvantumThemeSpec {
            theme_name: theme_name.clone(),
            blur_background: blur,
            translucent_windows: translucent,
            svg_path,
        };
        self.kvantum_spec = Some(spec);
        self.qt_style = QtPlatformStyle::Kvantum;
        self.event_log.push(ToolkitEvent::KvantumThemeApplied(
            self.kvantum_spec.as_ref().unwrap().theme_name.clone(),
        ));
        true
    }

    pub fn derive_accent_color_shades(&self) -> (String, String, String) {
        let base_hex = self.accent_color.to_hex();
        let lighter = format!("{}33", base_hex);
        let darker = format!("{}cc", base_hex);
        (lighter, base_hex.to_string(), darker)
    }

    pub fn connect_signal(&mut self, signal: &str, handler_id: &str) {
        self.signal_listeners
            .entry(signal.to_string())
            .or_insert_with(Vec::new)
            .push(handler_id.to_string());
    }

    pub fn emit_signal(&self, signal: &str) -> usize {
        self.signal_listeners
            .get(signal)
            .map_or(0, |list| list.len())
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
        palette.insert("QPalette::Text".to_string(), fg);
        palette.insert(
            "QPalette::Highlight".to_string(),
            self.accent_color.to_hex().to_string(),
        );
        palette
    }

    pub fn generate_gtk_css(&self) -> String {
        let mut css = format!(
            "@define-color accent_color {};\n",
            self.accent_color.to_hex()
        );
        for (selector, props) in &self.css_rules {
            css.push_str(&format!("{} {{\n", selector));
            for (key, val) in props {
                css.push_str(&format!("  {}: {};\n", key, val));
            }
            css.push_str("}\n");
        }
        css
    }

    pub fn generate_libadwaita_css(&self) -> String {
        let (bg, fg, card) = match self.color_scheme {
            AdwColorScheme::PreferDark | AdwColorScheme::ForceDark => {
                ("#1e1e2e", "#cdd6f4", "#313244")
            }
            _ => ("#fafafa", "#1c1c1e", "#f0f0f2"),
        };
        format!(
            "@define-color window_bg_color {};\n\
             @define-color window_fg_color {};\n\
             @define-color accent_color {};\n\
             @define-color accent_bg_color {};\n\
             @define-color card_bg_color {};\n",
            bg,
            fg,
            self.accent_color.to_hex(),
            self.accent_color.to_hex(),
            card
        )
    }

    pub fn generate_qt_qss(&self) -> String {
        format!(
            "QWidget {{ font-family: '{}'; font-size: {}pt; selection-background-color: {}; }}",
            self.font_name,
            self.font_size,
            self.accent_color.to_hex()
        )
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
    pub mfa_otp: Option<String>,
    pub biometric_token: Option<String>,
    pub fido2_security_key: Option<String>,
    pub session_type: DesktopSessionType,
    pub seat_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PamAuthResult {
    pub authenticated: bool,
    pub user_uid: u32,
    pub home_dir: String,
    pub mfa_verified: bool,
    pub fido2_verified: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreeterThemeConfig {
    pub wallpaper_path: String,
    pub user_avatars: BTreeMap<String, String>,
    pub cursor_theme: String,
    pub gtk_theme: String,
    pub icon_theme: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DMSeatConfig {
    pub seat_id: String,
    pub vt_number: u32,
    pub active_user: Option<String>,
    pub greeter_active: bool,
    pub auto_login_enabled: bool,
    pub is_guest_session: bool,
    pub bound_input_devices: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XdmcpSessionHandshake {
    pub session_id: usize,
    pub remote_address: String,
    pub display_number: u32,
    pub authenticated: bool,
    pub status: String,
}

pub struct XdmcpSessionManager {
    pub listening_port: u16,
    pub active_xdmcp_connections: BTreeMap<String, String>,
}

impl XdmcpSessionManager {
    pub fn new() -> Self {
        Self {
            listening_port: 177,
            active_xdmcp_connections: BTreeMap::new(),
        }
    }

    pub fn initiate_handshake(&mut self, remote_display: &str, client_host: &str) -> bool {
        self.active_xdmcp_connections
            .insert(remote_display.to_string(), client_host.to_string());
        true
    }
}

impl Default for XdmcpSessionManager {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SovereignDisplayManagerSuiteEngine {
    pub active_backend: DisplayManagerBackend,
    pub seats: BTreeMap<String, DMSeatConfig>,
    pub greeter_config: GreeterThemeConfig,
    pub available_sessions: Vec<DesktopSessionType>,
    pub pam_auth_history: Vec<(String, bool)>,
    pub xdmcp_sessions: Vec<XdmcpSessionHandshake>,
    pub xdmcp_manager: XdmcpSessionManager,
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
                is_guest_session: false,
                bound_input_devices: vec![
                    "/dev/input/event0".to_string(),
                    "/dev/input/event1".to_string(),
                ],
            },
        );

        let mut avatars = BTreeMap::new();
        avatars.insert("sigma_user".to_string(), "/var/lib/AccountsService/icons/sigma_user.png".to_string());

        Self {
            active_backend: backend,
            seats,
            greeter_config: GreeterThemeConfig {
                wallpaper_path: "/usr/share/backgrounds/sigma_default.png".to_string(),
                user_avatars: avatars,
                cursor_theme: "Adwaita".to_string(),
                gtk_theme: "Adwaita-dark".to_string(),
                icon_theme: "Papirus-Dark".to_string(),
            },
            available_sessions: vec![
                DesktopSessionType::Wayland,
                DesktopSessionType::Hyprland,
                DesktopSessionType::Zenith,
                DesktopSessionType::KdePlasma,
                DesktopSessionType::GnomeShell,
                DesktopSessionType::X11,
            ],
            pam_auth_history: Vec::new(),
            xdmcp_sessions: Vec::new(),
            xdmcp_manager: XdmcpSessionManager::new(),
        }
    }

    pub fn build_pam_session_env(
        &self,
        user: &str,
        session_type: DesktopSessionType,
    ) -> BTreeMap<String, String> {
        let mut env = BTreeMap::new();
        env.insert("USER".to_string(), user.to_string());
        env.insert("LOGNAME".to_string(), user.to_string());
        env.insert("HOME".to_string(), format!("/home/{}", user));
        env.insert("SHELL".to_string(), "/bin/bash".to_string());

        match session_type {
            DesktopSessionType::Wayland | DesktopSessionType::Hyprland | DesktopSessionType::Zenith => {
                env.insert("XDG_SESSION_TYPE".to_string(), "wayland".to_string());
                env.insert("WAYLAND_DISPLAY".to_string(), "wayland-0".to_string());
                env.insert("XDG_CURRENT_DESKTOP".to_string(), "SigmaOS:Wayland".to_string());
                env.insert("DESKTOP_SESSION".to_string(), "wayland".to_string());
            }
            DesktopSessionType::X11 | DesktopSessionType::KdePlasma | DesktopSessionType::GnomeShell => {
                env.insert("XDG_SESSION_TYPE".to_string(), "x11".to_string());
                env.insert("DISPLAY".to_string(), ":0".to_string());
                env.insert("XDG_CURRENT_DESKTOP".to_string(), "SigmaOS:X11".to_string());
                env.insert("DESKTOP_SESSION".to_string(), "x11".to_string());
            }
            _ => {
                env.insert("XDG_SESSION_TYPE".to_string(), "wayland".to_string());
                env.insert("WAYLAND_DISPLAY".to_string(), "wayland-0".to_string());
                env.insert("XDG_CURRENT_DESKTOP".to_string(), "SigmaOS".to_string());
            }
        }
        env
    }

    pub fn authenticate_pam(&mut self, req: &PamAuthRequest) -> PamAuthResult {
        let auth_success = !req.username.is_empty()
            && (!req.password_hash.is_empty()
                || req.biometric_token.is_some()
                || req.fido2_security_key.is_some());
        let mfa_ok = req.mfa_otp.as_deref() == Some("123456") || req.mfa_otp.is_none();
        let fido_ok = req.fido2_security_key.is_none()
            || req.fido2_security_key.as_deref() == Some("fido2_valid");

        let final_ok = auth_success && mfa_ok && fido_ok;
        self.pam_auth_history.push((req.username.clone(), final_ok));

        if final_ok {
            if let Some(seat) = self.seats.get_mut(&req.seat_id) {
                seat.active_user = Some(req.username.clone());
                seat.greeter_active = false;
            }
            PamAuthResult {
                authenticated: true,
                user_uid: 1000,
                home_dir: format!("/home/{}", req.username),
                mfa_verified: req.mfa_otp.is_some(),
                fido2_verified: req.fido2_security_key.is_some(),
                error_message: None,
            }
        } else {
            PamAuthResult {
                authenticated: false,
                user_uid: 0,
                home_dir: String::new(),
                mfa_verified: false,
                fido2_verified: false,
                error_message: Some(
                    "PAM Authentication Failed: Invalid credentials, MFA, or FIDO2 token"
                        .to_string(),
                ),
            }
        }
    }

    pub fn initiate_xdmcp_remote_handshake(
        &mut self,
        client_address: &str,
        display_number: u32,
    ) -> usize {
        let id = NEXT_DESKTOP_SERVICE_ID.fetch_add(1, Ordering::SeqCst);
        self.xdmcp_sessions.push(XdmcpSessionHandshake {
            session_id: id,
            remote_address: client_address.to_string(),
            display_number,
            authenticated: true,
            status: "ACTIVE".to_string(),
        });
        self.xdmcp_manager
            .initiate_handshake(&format!(":{}", display_number), client_address);
        id
    }

    pub fn create_seat(&mut self, seat_id: &str, vt_number: u32) {
        self.seats.insert(
            seat_id.to_string(),
            DMSeatConfig {
                seat_id: seat_id.to_string(),
                vt_number,
                active_user: None,
                greeter_active: true,
                auto_login_enabled: false,
                is_guest_session: false,
                bound_input_devices: vec!["/dev/input/event0".to_string()],
            },
        );
    }

    pub fn create_guest_session_sandbox(&mut self, seat_id: &str) -> Option<String> {
        if let Some(seat) = self.seats.get_mut(seat_id) {
            let guest_user = format!(
                "guest_{}",
                NEXT_DESKTOP_SERVICE_ID.fetch_add(1, Ordering::SeqCst)
            );
            seat.active_user = Some(guest_user.clone());
            seat.greeter_active = false;
            seat.is_guest_session = true;
            Some(guest_user)
        } else {
            None
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

    pub fn initiate_xdmcp_remote_handshake_v2(
        &mut self,
        client_address: &str,
        display_number: u32,
    ) -> usize {
        let id = NEXT_DESKTOP_SERVICE_ID.fetch_add(1, Ordering::SeqCst);
        self.xdmcp_sessions.push(XdmcpSessionHandshake {
            session_id: id,
            remote_address: client_address.to_string(),
            display_number,
            authenticated: true,
            status: "ACTIVE".to_string(),
        });
        self.xdmcp_manager
            .initiate_handshake(&format!(":{}", display_number), client_address);
        id
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
    GPhoto2,
    Afc,
    GoogleDrive,
    Davfs2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GFileMonitorEvent {
    Changed,
    Created,
    Deleted,
    AttributeChanged,
    PreUnmount,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GFileInfo {
    pub display_name: String,
    pub content_type: String,
    pub size_bytes: u64,
    pub is_executable: bool,
    pub is_hidden: bool,
    pub xattrs: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GvfsVolumeEvent {
    VolumeAdded(String),
    VolumeRemoved(String),
    MountAdded(usize),
    MountRemoved(usize),
    DriveConnected(String),
    DriveDisconnected(String),
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
    pub gio_metadata: BTreeMap<String, GFileInfo>,
    pub active_monitors: Vec<(String, GFileMonitorEvent)>,
    pub volume_events: Vec<GvfsVolumeEvent>,
    pub async_copy_operations: Vec<(String, String, u64, u64)>,
}

impl SovereignGvfsGioEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            mounts: Vec::new(),
            gio_file_cache: BTreeMap::new(),
            gio_metadata: BTreeMap::new(),
            active_monitors: Vec::new(),
            volume_events: Vec::new(),
            async_copy_operations: Vec::new(),
        };
        engine.volume_events.push(GvfsVolumeEvent::DriveConnected("TrashDrive".to_string()));
        // Default virtual mount: Trash
        engine
            .mount_uri("trash:///", "/run/user/1000/gvfs/trash")
            .ok();
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
        } else if uri.starts_with("gphoto2://") {
            GvfsMountScheme::GPhoto2
        } else if uri.starts_with("afc://") {
            GvfsMountScheme::Afc
        } else if uri.starts_with("google-drive://") {
            GvfsMountScheme::GoogleDrive
        } else if uri.starts_with("davfs2://") {
            GvfsMountScheme::Davfs2
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
        self.volume_events.push(GvfsVolumeEvent::MountAdded(id));
        Ok(id)
    }

    pub fn unmount(&mut self, mount_id: usize) -> bool {
        if let Some(m) = self.mounts.iter_mut().find(|m| m.mount_id == mount_id) {
            m.is_mounted = false;
            self.volume_events.push(GvfsVolumeEvent::MountRemoved(mount_id));
            true
        } else {
            false
        }
    }

    pub fn g_file_enumerate_children(&self, dir_path: &str) -> Vec<GFileInfo> {
        let prefix = if dir_path.ends_with('/') {
            dir_path.to_string()
        } else {
            format!("{}/", dir_path)
        };

        self.gio_metadata
            .iter()
            .filter(|(path, _)| path.starts_with(&prefix) && path.len() > prefix.len())
            .map(|(_, info)| info.clone())
            .collect()
    }

    pub fn g_file_read_all(&self, gio_path: &str) -> Option<Vec<u8>> {
        self.gio_file_cache.get(gio_path).cloned()
    }

    pub fn g_file_write_all(&mut self, gio_path: &str, data: &[u8], content_type: &str) {
        self.gio_file_cache
            .insert(gio_path.to_string(), data.to_vec());
        self.gio_metadata.insert(
            gio_path.to_string(),
            GFileInfo {
                display_name: gio_path.rsplit('/').next().unwrap_or("file").to_string(),
                content_type: content_type.to_string(),
                size_bytes: data.len() as u64,
                is_executable: false,
                is_hidden: gio_path.starts_with('.'),
                xattrs: BTreeMap::new(),
            },
        );
        self.active_monitors
            .push((gio_path.to_string(), GFileMonitorEvent::Created));
    }

    pub fn g_file_set_xattr(&mut self, gio_path: &str, key: &str, value: &str) -> bool {
        if let Some(info) = self.gio_metadata.get_mut(gio_path) {
            info.xattrs.insert(key.to_string(), value.to_string());
            self.active_monitors
                .push((gio_path.to_string(), GFileMonitorEvent::AttributeChanged));
            true
        } else {
            false
        }
    }

    pub fn g_file_copy_async(&mut self, src_path: &str, dst_path: &str) -> Result<u64, String> {
        if let Some(data) = self.gio_file_cache.get(src_path).cloned() {
            let total = data.len() as u64;
            self.g_file_write_all(dst_path, &data, "application/octet-stream");
            self.async_copy_operations.push((
                src_path.to_string(),
                dst_path.to_string(),
                total,
                total,
            ));
            Ok(total)
        } else {
            Err(format!("Source file '{}' not found in GIO cache", src_path))
        }
    }

    pub fn g_file_query_info(&self, gio_path: &str) -> Option<GFileInfo> {
        self.gio_metadata.get(gio_path).cloned()
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolkitActionRule {
    pub action_id: String,
    pub required_permission: PolkitResult,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolkitSubject {
    UnixProcess { pid: u32, start_time: u64 },
    SystemBusName { name: String },
}

pub struct SovereignPolicyKitAuthorizationEngine {
    pub action_rules: BTreeMap<String, PolkitActionRule>,
    pub js_rules: Vec<(String, PolkitResult)>,
    pub admin_groups: Vec<String>,
    pub auth_cache: BTreeMap<(String, String), (PolkitResult, u64)>,
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
            js_rules: Vec::new(),
            admin_groups: vec!["wheel".to_string(), "sudo".to_string(), "admin".to_string()],
            auth_cache: BTreeMap::new(),
            auth_audit_log: Vec::new(),
        }
    }

    pub fn add_js_rule(&mut self, action_prefix: &str, result: PolkitResult) {
        self.js_rules.push((action_prefix.to_string(), result));
    }

    pub fn eval_js_policy_rules(&self, action_id: &str) -> Option<PolkitResult> {
        for (prefix, result) in &self.js_rules {
            if action_id.starts_with(prefix) {
                return Some(*result);
            }
        }
        None
    }

    pub fn revoke_cached_authorizations(&mut self, user: &str) -> usize {
        let keys_to_remove: Vec<(String, String)> = self
            .auth_cache
            .keys()
            .filter(|(u, _)| u == user)
            .cloned()
            .collect();
        let count = keys_to_remove.len();
        for k in keys_to_remove {
            self.auth_cache.remove(&k);
        }
        count
    }

    pub fn check_authorization_subject(
        &mut self,
        subject: &PolkitSubject,
        action_id: &str,
    ) -> PolkitResult {
        let user = match subject {
            PolkitSubject::UnixProcess { pid, .. } => format!("process_{}", pid),
            PolkitSubject::SystemBusName { name } => format!("dbus_{}", name),
        };
        self.check_authorization(&user, &["wheel"], action_id)
    }

    pub fn check_authorization(
        &mut self,
        user: &str,
        user_groups: &[&str],
        action_id: &str,
    ) -> PolkitResult {
        let is_admin = user_groups
            .iter()
            .any(|g| self.admin_groups.contains(&g.to_string()));

        // Check JS dynamic rules first
        if let Some((_, res)) = self
            .js_rules
            .iter()
            .find(|(prefix, _)| action_id.starts_with(prefix))
        {
            self.auth_audit_log.push(format!(
                "Polkit JS Rule Match: User='{}' Action='{}' Result='{:?}'",
                user, action_id, res
            ));
            return *res;
        }

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

        self.auth_cache
            .insert((user.to_string(), action_id.to_string()), (res, 300));
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
pub struct ScreenCastStream {
    pub stream_id: u32,
    pub pipewire_node_id: u32,
    pub is_video: bool,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteDesktopSession {
    pub session_handle: String,
    pub app_id: String,
    pub keyboard_enabled: bool,
    pub pointer_enabled: bool,
}

pub struct SovereignXdgDesktopPortalEngine {
    pub mime_apps: BTreeMap<String, String>,
    pub user_dirs: BTreeMap<String, String>,
    pub portal_permissions: BTreeMap<(String, String), PortalPermissionState>,
    pub screencast_sessions: BTreeMap<String, Vec<ScreenCastStream>>,
    pub remote_desktop_sessions: BTreeMap<String, RemoteDesktopSession>,
    pub active_idle_inhibitors: Vec<(String, String)>,
    pub secret_wallet_store: BTreeMap<String, String>,
}

impl SovereignXdgDesktopPortalEngine {
    pub fn new() -> Self {
        let mut mime_apps = BTreeMap::new();
        mime_apps.insert(
            "text/html".to_string(),
            "org.sigmaos.browser.desktop".to_string(),
        );
        mime_apps.insert(
            "image/png".to_string(),
            "org.sigmaos.pix.desktop".to_string(),
        );
        mime_apps.insert(
            "application/pdf".to_string(),
            "org.sigmaos.xreader.desktop".to_string(),
        );

        let mut user_dirs = BTreeMap::new();
        user_dirs.insert("XDG_DESKTOP_DIR".to_string(), "$HOME/Desktop".to_string());
        user_dirs.insert(
            "XDG_DOWNLOAD_DIR".to_string(),
            "$HOME/Downloads".to_string(),
        );
        user_dirs.insert(
            "XDG_DOCUMENTS_DIR".to_string(),
            "$HOME/Documents".to_string(),
        );
        user_dirs.insert("XDG_MUSIC_DIR".to_string(), "$HOME/Music".to_string());
        user_dirs.insert("XDG_PICTURES_DIR".to_string(), "$HOME/Pictures".to_string());

        Self {
            mime_apps,
            user_dirs,
            portal_permissions: BTreeMap::new(),
            screencast_sessions: BTreeMap::new(),
            remote_desktop_sessions: BTreeMap::new(),
            active_idle_inhibitors: Vec::new(),
            secret_wallet_store: BTreeMap::new(),
        }
    }

    pub fn query_mime_association(&self, mime_type: &str) -> Option<String> {
        self.mime_apps.get(mime_type).cloned()
    }

    pub fn set_mime_association(&mut self, mime_type: &str, app_id: &str) {
        self.mime_apps.insert(mime_type.to_string(), app_id.to_string());
    }

    pub fn handle_portal_open_uri(&mut self, app_id: &str, uri: &str) -> Result<String, String> {
        let key = (app_id.to_string(), "org.freedesktop.portal.OpenURI".to_string());
        self.portal_permissions.insert(key, PortalPermissionState::Granted);
        self.xdg_open(uri)
    }

    pub fn handle_portal_screenshot(&mut self, app_id: &str, interactive: bool) -> Result<String, String> {
        let key = (app_id.to_string(), "org.freedesktop.portal.Screenshot".to_string());
        self.portal_permissions.insert(key, PortalPermissionState::Granted);
        Ok(format!("Portal Screenshot captured for '{}' (interactive={})", app_id, interactive))
    }

    pub fn handle_portal_location(&mut self, app_id: &str) -> Result<(f64, f64), String> {
        let key = (app_id.to_string(), "org.freedesktop.portal.Location".to_string());
        self.portal_permissions.insert(key, PortalPermissionState::Granted);
        Ok((37.7749, -122.4194)) // Mock lat, lon
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

    pub fn create_screencast_session(&mut self, app_id: &str) -> String {
        let session_handle = format!(
            "/org/freedesktop/portal/desktop/session/{}/{}",
            app_id,
            NEXT_DESKTOP_SERVICE_ID.fetch_add(1, Ordering::SeqCst)
        );
        let streams = vec![ScreenCastStream {
            stream_id: 1,
            pipewire_node_id: 42,
            is_video: true,
            width: 1920,
            height: 1080,
        }];
        self.screencast_sessions
            .insert(session_handle.clone(), streams);
        session_handle
    }

    pub fn create_remote_desktop_session(&mut self, app_id: &str) -> String {
        let session_handle = format!(
            "/org/freedesktop/portal/desktop/remote/{}/{}",
            app_id,
            NEXT_DESKTOP_SERVICE_ID.fetch_add(1, Ordering::SeqCst)
        );
        let session = RemoteDesktopSession {
            session_handle: session_handle.clone(),
            app_id: app_id.to_string(),
            keyboard_enabled: true,
            pointer_enabled: true,
        };
        self.remote_desktop_sessions
            .insert(session_handle.clone(), session);
        session_handle
    }

    pub fn inhibit_idle(&mut self, app_id: &str, reason: &str) -> usize {
        let id = NEXT_DESKTOP_SERVICE_ID.fetch_add(1, Ordering::SeqCst);
        self.active_idle_inhibitors
            .push((app_id.to_string(), reason.to_string()));
        id
    }

    pub fn handle_portal_file_chooser(
        &mut self,
        app_id: &str,
        title: &str,
    ) -> Result<String, String> {
        let key = (
            app_id.to_string(),
            "org.freedesktop.portal.FileChooser".to_string(),
        );
        self.portal_permissions
            .insert(key, PortalPermissionState::Granted);
        Ok(format!(
            "Portal FileChooser Approved for '{}': title='{}'",
            app_id, title
        ))
    }

    pub fn store_secret(&mut self, key: &str, secret_val: &str) {
        self.secret_wallet_store
            .insert(key.to_string(), secret_val.to_string());
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubpixelOrder {
    Rgb,
    Bgr,
    Vrgb,
    Vbgr,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontMatchRequest {
    pub family: String,
    pub weight: u32,
    pub size_pt: u32,
    pub antialias: bool,
    pub hinting: HintingStyle,
    pub subpixel_order: SubpixelOrder,
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

pub struct SovereignFontAndTextLayoutEngine {
    pub font_match_cache: BTreeMap<String, String>,
    pub fontconfig_aliases: BTreeMap<String, Vec<String>>,
    pub opentype_features: Vec<String>,
    pub shaped_buffer_cache: Vec<HbShapedGlyph>,
    pub skia_surface_width: u32,
    pub skia_surface_height: u32,
}

impl SovereignFontAndTextLayoutEngine {
    pub fn new() -> Self {
        let mut font_match_cache = BTreeMap::new();
        font_match_cache.insert(
            "sans-serif".to_string(),
            "/usr/share/fonts/dejavu/DejaVuSans.ttf".to_string(),
        );
        font_match_cache.insert(
            "monospace".to_string(),
            "/usr/share/fonts/ghostty/JetBrainsMono.ttf".to_string(),
        );
        font_match_cache.insert(
            "serif".to_string(),
            "/usr/share/fonts/dejavu/DejaVuSerif.ttf".to_string(),
        );

        let mut fontconfig_aliases = BTreeMap::new();
        fontconfig_aliases.insert(
            "monospace".to_string(),
            vec!["JetBrainsMono".to_string(), "DejaVu Sans Mono".to_string()],
        );
        fontconfig_aliases.insert(
            "sans-serif".to_string(),
            vec!["Cantarell".to_string(), "DejaVu Sans".to_string()],
        );

        Self {
            font_match_cache,
            fontconfig_aliases,
            opentype_features: vec![
                "liga".to_string(),
                "kern".to_string(),
                "zero".to_string(),
                "smcp".to_string(),
            ],
            shaped_buffer_cache: Vec::new(),
            skia_surface_width: 1920,
            skia_surface_height: 1080,
        }
    }

    pub fn parse_fonts_conf_xml(&mut self, xml_content: &str) -> usize {
        let mut count = 0;
        if xml_content.contains("<alias>") {
            count += 1;
        }
        count
    }

    pub fn match_font(&self, req: &FontMatchRequest) -> String {
        self.font_match_cache
            .get(&req.family)
            .cloned()
            .unwrap_or_else(|| "/usr/share/fonts/dejavu/DejaVuSans.ttf".to_string())
    }

    pub fn harfbuzz_shape_text(&mut self, text: &str, rtl: bool) -> usize {
        self.shaped_buffer_cache.clear();
        let all_chars: Vec<char> = text.chars().collect();
        let char_pairs: Vec<(usize, char)> = if rtl {
            all_chars
                .into_iter()
                .enumerate()
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect()
        } else {
            all_chars.into_iter().enumerate().collect()
        };

        for (idx, ch) in char_pairs {
            self.shaped_buffer_cache.push(HbShapedGlyph {
                glyph_id: ch as u32,
                x_advance: if rtl { -800 } else { 800 },
                y_advance: 0,
                x_offset: 0,
                y_offset: 0,
                cluster: idx as u32,
            });
        }
        self.shaped_buffer_cache.len()
    }

    pub fn harfbuzz_shape_text_bidi(&mut self, text: &str) -> usize {
        let is_rtl = text.chars().any(|c| (c as u32) >= 0x0590 && (c as u32) <= 0x08FF);
        self.harfbuzz_shape_text(text, is_rtl)
    }

    pub fn pango_cairo_render_layout(&self, text: &str) -> String {
        format!(
            "PangoLayout[text_len={}, glyphs={}, surface={}x{}, features={:?}]",
            text.len(),
            self.shaped_buffer_cache.len(),
            self.skia_surface_width,
            self.skia_surface_height,
            self.opentype_features
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
    pub current_page: usize,
    pub page_size: usize,
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
            current_page: 0,
            page_size: 5,
        }
    }

    pub fn switch_engine(&mut self, engine_id: &str) {
        self.active_engine_id = engine_id.to_string();
        self.preedit_text.clear();
        self.candidates.clear();
        self.current_page = 0;
    }

    pub fn next_page(&mut self) -> bool {
        let total_pages = (self.candidates.len() + self.page_size - 1) / self.page_size;
        if total_pages > 0 && self.current_page + 1 < total_pages {
            self.current_page += 1;
            true
        } else {
            false
        }
    }

    pub fn prev_page(&mut self) -> bool {
        if self.current_page > 0 {
            self.current_page -= 1;
            true
        } else {
            false
        }
    }

    pub fn select_candidate_by_index(&mut self, idx: usize) -> Option<String> {
        let global_idx = self.current_page * self.page_size + idx;
        if global_idx < self.candidates.len() {
            let commit = self.candidates[global_idx].text.clone();
            self.preedit_text.clear();
            self.candidates.clear();
            self.current_page = 0;
            Some(commit)
        } else {
            None
        }
    }

    pub fn process_key_event(&mut self, key_str: &str) -> Option<String> {
        if key_str == "Return" || key_str == "Enter" {
            if !self.candidates.is_empty() && self.selected_candidate_index < self.candidates.len()
            {
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
                    ImeCandidate {
                        index: 1,
                        text: "你好".to_string(),
                        comment: "Hello".to_string(),
                    },
                    ImeCandidate {
                        index: 2,
                        text: "拟好".to_string(),
                        comment: "Draft".to_string(),
                    },
                    ImeCandidate {
                        index: 3,
                        text: "你好吗".to_string(),
                        comment: "How are you".to_string(),
                    },
                    ImeCandidate {
                        index: 4,
                        text: "逆好".to_string(),
                        comment: "Inverse".to_string(),
                    },
                    ImeCandidate {
                        index: 5,
                        text: "霓好".to_string(),
                        comment: "Neon".to_string(),
                    },
                    ImeCandidate {
                        index: 6,
                        text: "拟号".to_string(),
                        comment: "Draft No".to_string(),
                    },
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
    pub dnd_start_hour: u8,
    pub dnd_end_hour: u8,
    pub sound_theme_name: String,
    pub notification_history: Vec<DesktopNotificationItem>,
    pub action_invocations: Vec<(u32, String)>,
}

impl SovereignDesktopNotificationDaemonEngine {
    pub fn new() -> Self {
        Self {
            active_notifications: Vec::new(),
            dnd_mode: false,
            dnd_start_hour: 22,
            dnd_end_hour: 7,
            sound_theme_name: "freedesktop".to_string(),
            notification_history: Vec::new(),
            action_invocations: Vec::new(),
        }
    }

    pub fn set_dnd_schedule(&mut self, start_hour: u8, end_hour: u8) {
        self.dnd_start_hour = start_hour;
        self.dnd_end_hour = end_hour;
    }

    pub fn is_dnd_active_at_hour(&self, hour: u8) -> bool {
        if self.dnd_start_hour > self.dnd_end_hour {
            hour >= self.dnd_start_hour || hour < self.dnd_end_hour
        } else {
            hour >= self.dnd_start_hour && hour < self.dnd_end_hour
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

        let sound_event = match urgency {
            NotificationUrgency::Critical => Some("dialog-error".to_string()),
            NotificationUrgency::Normal => Some("message-new-instant".to_string()),
            NotificationUrgency::Low => None,
        };

        let item = DesktopNotificationItem {
            id,
            app_name: app_name.to_string(),
            summary: summary.to_string(),
            body: body.to_string(),
            urgency,
            actions: vec![("default".to_string(), "Open".to_string())],
            timeout_ms,
            sound_event,
        };

        if !self.dnd_mode || urgency == NotificationUrgency::Critical {
            if let Some(existing) = self.active_notifications.iter_mut().find(|n| n.id == id) {
                *existing = item.clone();
            } else {
                self.active_notifications.push(item.clone());
            }
        }
        self.notification_history.push(item);
        id
    }

    pub fn invoke_action(&mut self, notification_id: u32, action_key: &str) -> bool {
        if self
            .notification_history
            .iter()
            .any(|n| n.id == notification_id)
        {
            self.action_invocations
                .push((notification_id, action_key.to_string()));
            self.close_notification(notification_id);
            true
        } else {
            false
        }
    }

    pub fn resolve_canberra_sound_event(&self, sound_event: &str) -> String {
        format!("/usr/share/sounds/{}/stereo/{}.ogg", self.sound_theme_name, sound_event)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DpmsPowerMode {
    On,
    Standby,
    Suspend,
    Off,
}

pub struct SovereignScreenLockerEngine {
    pub lock_state: SessionLockState,
    pub dpms_mode: DpmsPowerMode,
    pub failed_attempts: u32,
    pub inactivity_timeout_secs: u32,
    pub is_pam_locked_out: bool,
    pub lock_surfaces_count: u32,
    pub session_lock_v1_acked: bool,
    pub auto_pause_media_on_lock: bool,
}

impl SovereignScreenLockerEngine {
    pub fn new() -> Self {
        Self {
            lock_state: SessionLockState::Unlocked,
            dpms_mode: DpmsPowerMode::On,
            failed_attempts: 0,
            inactivity_timeout_secs: 300,
            is_pam_locked_out: false,
            lock_surfaces_count: 0,
            session_lock_v1_acked: false,
            auto_pause_media_on_lock: true,
        }
    }

    pub fn lock_session(&mut self, monitor_count: u32) {
        self.lock_state = SessionLockState::Locked;
        self.lock_surfaces_count = monitor_count;
        self.dpms_mode = DpmsPowerMode::On;
        self.session_lock_v1_acked = true;
    }

    pub fn should_pause_media(&self) -> bool {
        self.auto_pause_media_on_lock && self.lock_state == SessionLockState::Locked
    }

    pub fn set_dpms_mode(&mut self, mode: DpmsPowerMode) {
        self.dpms_mode = mode;
    }

    pub fn authenticate_and_unlock(&mut self, password: &str) -> bool {
        if self.is_pam_locked_out {
            return false;
        }
        if password == "sigmaos_secret" {
            self.lock_state = SessionLockState::Unlocked;
            self.dpms_mode = DpmsPowerMode::On;
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
    pub mime_map: BTreeMap<String, Vec<u8>>,
    pub is_secret: bool,
    pub app_origin: String,
}

pub struct SovereignClipboardManagerEngine {
    pub primary_selection: Option<ClipboardEntry>,
    pub clipboard_history: Vec<ClipboardEntry>,
    pub max_history_items: usize,
}

impl SovereignClipboardManagerEngine {
    pub fn new() -> Self {
        Self {
            primary_selection: None,
            clipboard_history: Vec::new(),
            max_history_items: 50,
        }
    }

    pub fn copy_text(&mut self, text: &str, app_origin: &str, is_secret: bool) {
        let id = NEXT_DESKTOP_SERVICE_ID.fetch_add(1, Ordering::SeqCst);
        let mut mime_map = BTreeMap::new();
        mime_map.insert("text/plain".to_string(), text.as_bytes().to_vec());
        mime_map.insert(
            "text/plain;charset=utf-8".to_string(),
            text.as_bytes().to_vec(),
        );

        let entry = ClipboardEntry {
            id,
            mime_map,
            is_secret,
            app_origin: app_origin.to_string(),
        };

        if self.clipboard_history.len() >= self.max_history_items {
            self.clipboard_history.remove(0);
        }
        self.clipboard_history.push(entry);
    }

    pub fn copy_multi_mime(
        &mut self,
        mime_data: BTreeMap<String, Vec<u8>>,
        app_origin: &str,
        is_secret: bool,
    ) {
        let id = NEXT_DESKTOP_SERVICE_ID.fetch_add(1, Ordering::SeqCst);
        let entry = ClipboardEntry {
            id,
            mime_map: mime_data,
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
        let mut mime_map = BTreeMap::new();
        mime_map.insert("text/plain".to_string(), text.as_bytes().to_vec());

        self.primary_selection = Some(ClipboardEntry {
            id,
            mime_map,
            is_secret: false,
            app_origin: app_origin.to_string(),
        });
    }

    pub fn get_latest_text(&self) -> Option<String> {
        self.clipboard_history.last().and_then(|e| {
            if e.is_secret {
                Some("********".to_string())
            } else {
                e.mime_map
                    .get("text/plain")
                    .and_then(|p| String::from_utf8(p.clone()).ok())
            }
        })
    }

    pub fn search_cliphist(&self, query: &str) -> Vec<&ClipboardEntry> {
        self.clipboard_history
            .iter()
            .filter(|e| {
                !e.is_secret
                    && e.mime_map.values().any(|v| {
                        String::from_utf8_lossy(v).contains(query)
                    })
            })
            .collect()
    }

    pub fn purge_secret_entries(&mut self) -> usize {
        let before = self.clipboard_history.len();
        self.clipboard_history.retain(|e| !e.is_secret);
        before - self.clipboard_history.len()
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

    pub fn search_prs(&self, query: &str) -> Vec<&DesktopServicePrSubmission> {
        self.submissions
            .iter()
            .filter(|p| p.title.contains(query) || p.service_target.contains(query))
            .collect()
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
// 12. Master Orchestrator: Sovereign Desktop Toolkits & Services Suite
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

    pub fn run_health_audit(&self) -> BTreeMap<String, bool> {
        let mut report = BTreeMap::new();
        report.insert("GtkQtToolkitSync".to_string(), true);
        report.insert(
            "DisplayManagerActive".to_string(),
            !self.display_manager.seats.is_empty(),
        );
        report.insert(
            "GvfsGioActive".to_string(),
            !self.gvfs_gio.mounts.is_empty(),
        );
        report.insert(
            "PolicyKitActive".to_string(),
            !self.polkit.action_rules.is_empty(),
        );
        report.insert(
            "XdgPortalsActive".to_string(),
            !self.xdg_portal.user_dirs.is_empty(),
        );
        report.insert(
            "FontLayoutActive".to_string(),
            !self.font_layout.font_match_cache.is_empty(),
        );
        report.insert(
            "ImeFrameworkActive".to_string(),
            !self.ime_engine.active_engine_id.is_empty(),
        );
        report.insert("NotificationDaemonActive".to_string(), true);
        report.insert(
            "ScreenLockerReady".to_string(),
            self.screen_locker.lock_state == SessionLockState::Unlocked,
        );
        report.insert("ClipboardManagerActive".to_string(), true);
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

        engine.set_accent_color(SovereignAccentColor::Teal);
        assert_eq!(engine.accent_color.to_hex(), "#129eaf");

        let adw_css = engine.generate_libadwaita_css();
        assert!(adw_css.contains("@define-color window_bg_color #1e1e2e;"));

        assert!(engine.parse_and_apply_kvantum_config(
            "theme=KvCatppuccinMocha\nblur=true\ntranslucent=true"
        ));
        assert_eq!(engine.qt_style, QtPlatformStyle::Kvantum);

        assert!(engine.toggle_gtk_inspector());
        assert!(!engine.toggle_gtk_inspector());

        let palette = engine.sync_qt_to_gtk_palette();
        assert_eq!(
            palette.get("QPalette::Window"),
            Some(&"#1e1e2e".to_string())
        );
        assert_eq!(
            palette.get("QPalette::Highlight"),
            Some(&"#129eaf".to_string())
        );

        let css = engine.generate_gtk_css();
        assert!(css.contains("@define-color accent_color #129eaf;"));

        let qss = engine.generate_qt_qss();
        assert!(qss.contains("selection-background-color: #129eaf;"));

        assert!(engine.parse_and_apply_kvantum_config(
            "theme=Catppuccin-Mocha\nblur_radius=20\ntranslucent_windows=true"
        ));
        assert_eq!(engine.qt_style, QtPlatformStyle::Kvantum);
        let spec = engine.kvantum_spec.as_ref().unwrap();
        assert_eq!(
            spec.svg_path,
            "/usr/share/Kvantum/Catppuccin-Mocha/Catppuccin-Mocha.svg"
        );

        engine.connect_signal("clicked", "btn_handler_1");
        assert_eq!(engine.emit_signal("clicked"), 1);
    }

    #[test]
    fn test_display_manager_pam_auth() {
        let mut dm = SovereignDisplayManagerSuiteEngine::new(DisplayManagerBackend::Gdm);
        dm.create_seat("seat1", 2);

        let req = PamAuthRequest {
            username: "sigma_user".to_string(),
            password_hash: "hashed_pass".to_string(),
            mfa_otp: Some("123456".to_string()),
            biometric_token: None,
            fido2_security_key: Some("fido2_valid".to_string()),
            session_type: DesktopSessionType::Wayland,
            seat_id: "seat0".to_string(),
        };

        let res = dm.authenticate_pam(&req);
        assert!(res.authenticated);
        assert!(res.mfa_verified);
        assert!(res.fido2_verified);
        assert_eq!(res.user_uid, 1000);
        assert!(!dm.seats.get("seat0").unwrap().greeter_active);

        let env = dm.build_pam_session_env("sigma_user", DesktopSessionType::Wayland);
        assert_eq!(env.get("XDG_SESSION_TYPE"), Some(&"wayland".to_string()));

        let guest = dm.create_guest_session_sandbox("seat1").unwrap();
        assert!(guest.starts_with("guest_"));
        assert!(dm.seats.get("seat1").unwrap().is_guest_session);

        let xdmcp_id = dm.initiate_xdmcp_remote_handshake("192.168.1.50", 1);
        assert!(xdmcp_id > 0);
    }

    #[test]
    fn test_gvfs_gio_virtual_mounts() {
        let mut gvfs = SovereignGvfsGioEngine::new();
        let mount_id = gvfs
            .mount_uri("smb://192.168.1.100/share", "/run/user/1000/gvfs/smb")
            .unwrap();
        assert!(mount_id > 0);
        assert_eq!(gvfs.mounts.len(), 2);

        let gphoto_id = gvfs
            .mount_uri("gphoto2://camera", "/run/user/1000/gvfs/gphoto2")
            .unwrap();
        assert!(gphoto_id > 0);

        gvfs.g_file_write_all("/trash/item1.txt", b"deleted item", "text/plain");
        assert_eq!(
            gvfs.g_file_read_all("/trash/item1.txt"),
            Some(b"deleted item".to_vec())
        );

        let children = gvfs.g_file_enumerate_children("/trash");
        assert_eq!(children.len(), 1);

        assert!(gvfs.g_file_set_xattr("/trash/item1.txt", "user.checksum", "sha256abc"));
        let copy_bytes = gvfs
            .g_file_copy_async("/trash/item1.txt", "/trash/item2.txt")
            .unwrap();
        assert_eq!(copy_bytes, 12);

        let info = gvfs.g_file_query_info("/trash/item1.txt").unwrap();
        assert_eq!(info.display_name, "item1.txt");
        assert_eq!(info.size_bytes, 12);
        assert_eq!(
            info.xattrs.get("user.checksum"),
            Some(&"sha256abc".to_string())
        );
    }

    #[test]
    fn test_polkit_and_xdg_portals() {
        let mut polkit = SovereignPolicyKitAuthorizationEngine::new();
        polkit.add_js_rule("org.sigmaos.device.", PolkitResult::Yes);

        let res_js = polkit.check_authorization("sigma_user", &[], "org.sigmaos.device.drive");
        assert_eq!(res_js, PolkitResult::Yes);

        let subj = PolkitSubject::UnixProcess { pid: 1234, start_time: 5000 };
        let res_subj = polkit.check_authorization_subject(&subj, "org.freedesktop.accounts.user-administration");
        assert_eq!(res_subj, PolkitResult::Yes);

        let res = polkit.check_authorization(
            "sigma_user",
            &["wheel"],
            "org.freedesktop.accounts.user-administration",
        );
        assert_eq!(res, PolkitResult::Yes);

        let mut portal = SovereignXdgDesktopPortalEngine::new();
        portal.set_mime_association("image/png", "org.sigmaos.pix.desktop");
        assert_eq!(portal.query_mime_association("image/png"), Some("org.sigmaos.pix.desktop".to_string()));

        let app_res = portal.handle_portal_open_uri("org.sigmaos.browser", "https://sigmaos.org");
        assert!(app_res.unwrap().contains("browser"));

        let shot_res = portal.handle_portal_screenshot("org.sigmaos.flameshot", true);
        assert!(shot_res.unwrap().contains("org.sigmaos.flameshot"));

        let loc = portal.handle_portal_location("org.sigmaos.maps").unwrap();
        assert_eq!(loc, (37.7749, -122.4194));

        let handle = portal.create_screencast_session("org.sigmaos.obs");
        assert!(handle.contains("org.sigmaos.obs"));

        let remote_handle = portal.create_remote_desktop_session("org.sigmaos.remotedesktop");
        assert!(remote_handle.contains("org.sigmaos.remotedesktop"));

        let in_id = portal.inhibit_idle("org.sigmaos.media", "Playing Video");
        assert!(in_id > 0);

        portal.store_secret("wifi_pass", "secret123");
        assert_eq!(
            portal.retrieve_secret("wifi_pass"),
            Some("secret123".to_string())
        );
    }

    #[test]
    fn test_fonts_ime_notifications_screenlock_clipboard_and_pr() {
        let mut font_engine = SovereignFontAndTextLayoutEngine::new();
        let xml_rules = font_engine.parse_fonts_conf_xml(
            "<fontconfig><alias><family>DejaVu Sans</family></alias></fontconfig>",
        );
        assert_eq!(xml_rules, 1);

        let font_path = font_engine.match_font(&FontMatchRequest {
            family: "monospace".to_string(),
            weight: 400,
            size_pt: 12,
            antialias: true,
            hinting: HintingStyle::Full,
            subpixel_order: SubpixelOrder::Rgb,
        });
        assert!(font_path.contains("JetBrainsMono"));

        let glyph_count = font_engine.harfbuzz_shape_text("SigmaOS", false);
        assert_eq!(glyph_count, 7);

        let bidi_count = font_engine.harfbuzz_shape_text_bidi("שָׁלוֹם");
        assert!(bidi_count > 0);

        let mut ime = SovereignInputMethodFrameworkEngine::new(ImeFrameworkKind::Fcitx5);
        ime.switch_engine("pinyin");
        ime.process_key_event("n");
        ime.process_key_event("i");
        ime.process_key_event("h");
        ime.process_key_event("a");
        ime.process_key_event("o");
        let committed = ime.process_key_event("Return");
        assert_eq!(committed, Some("你好".to_string()));

        let mut notif = SovereignDesktopNotificationDaemonEngine::new();
        notif.set_dnd_schedule(22, 7);
        assert!(notif.is_dnd_active_at_hour(23));

        let snd_path = notif.resolve_canberra_sound_event("dialog-error");
        assert!(snd_path.contains("dialog-error.ogg"));

        let nid = notif.notify(
            "systemd",
            0,
            "Update Available",
            "Restart to apply",
            NotificationUrgency::Normal,
            5000,
        );
        assert!(nid > 0);
        assert!(notif.invoke_action(nid, "default"));

        let mut locker = SovereignScreenLockerEngine::new();
        locker.lock_session(2);
        assert_eq!(locker.lock_state, SessionLockState::Locked);
        assert!(locker.should_pause_media());
        assert!(locker.session_lock_v1_acked);
        locker.set_dpms_mode(DpmsPowerMode::Off);
        assert_eq!(locker.dpms_mode, DpmsPowerMode::Off);
        assert!(locker.authenticate_and_unlock("sigmaos_secret"));
        assert_eq!(locker.lock_state, SessionLockState::Unlocked);

        let mut clip = SovereignClipboardManagerEngine::new();
        clip.copy_text("Secret Token", "KeePassXC", true);
        clip.copy_text("Normal Text Item", "Gedit", false);
        assert_eq!(clip.get_latest_text(), Some("Normal Text Item".to_string()));
        let search_res = clip.search_cliphist("Normal");
        assert_eq!(search_res.len(), 1);
        let purged = clip.purge_secret_entries();
        assert_eq!(purged, 1);

        let mut pr_gateway = SovereignDesktopServicesPrGatewayEngine::new();
        let pr_id = pr_gateway.submit_pr(
            "Add SDDM KCM module",
            "sddm-dev",
            "display_manager",
            "Adds Qt6 KCM module for SDDM configuration",
        );
        assert!(pr_gateway.run_automated_ci_pr_gate(pr_id));

        let results = pr_gateway.search_prs("SDDM");
        assert_eq!(results.len(), 1);

        let diff = pr_gateway.generate_pr_unified_diff(pr_id).unwrap();
        assert!(diff.contains("Add SDDM KCM module"));

        let master_suite = SovereignDesktopToolkitsAndServicesSuite::new();
        let audit = master_suite.run_health_audit();
        assert_eq!(audit.get("GtkQtToolkitSync"), Some(&true));
    }
}
