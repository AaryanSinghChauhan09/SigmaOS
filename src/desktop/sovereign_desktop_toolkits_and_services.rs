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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gtk4RenderPipeline {
    Ngl,
    Vulkan,
    Gles,
    CairoFallback,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KvantumThemeSpec {
    pub theme_name: String,
    pub blur_background: bool,
    pub translucent_windows: bool,
    pub svg_path: String,
    pub shadow_size_px: u32,
    pub corner_radius_px: u32,
    pub window_margin_px: u32,
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
    Custom(u8, u8, u8),
}

impl SovereignAccentColor {
    pub fn to_hex(&self) -> String {
        match self {
            SovereignAccentColor::Blue => "#3584e4".to_string(),
            SovereignAccentColor::Teal => "#129eaf".to_string(),
            SovereignAccentColor::Green => "#33d17a".to_string(),
            SovereignAccentColor::Yellow => "#f6d32d".to_string(),
            SovereignAccentColor::Orange => "#ff7800".to_string(),
            SovereignAccentColor::Red => "#e01b24".to_string(),
            SovereignAccentColor::Pink => "#d56199".to_string(),
            SovereignAccentColor::Purple => "#9141ac".to_string(),
            SovereignAccentColor::Slate => "#77767b".to_string(),
            SovereignAccentColor::Custom(r, g, b) => format!("#{:02x}{:02x}{:02x}", r, g, b),
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
pub struct Qt6QmlThemeBridge {
    pub active_palette: BTreeMap<String, String>,
    pub qml_import_paths: Vec<String>,
    pub font_family: String,
    pub font_size_pt: u32,
    pub is_dark: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolkitEvent {
    ThemeChanged { dark_mode: bool, theme_name: String },
    ColorSchemeChanged(AdwColorScheme),
    AccentColorChanged(SovereignAccentColor),
    FontConfigUpdated { family: String, size_pt: u32 },
    GtkInspectorToggled(bool),
    KvantumThemeApplied(String),
    WallpaperAccentExtracted(String),
    RenderPipelineChanged(Gtk4RenderPipeline),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QmlPluginRuntimeBridge {
    pub registered_qml_modules: Vec<String>,
    pub active_quick_controls_theme: String,
}

pub struct SovereignGtkQtToolkitEngine {
    pub color_scheme: AdwColorScheme,
    pub qt_style: QtPlatformStyle,
    pub render_pipeline: Gtk4RenderPipeline,
    pub accent_color: SovereignAccentColor,
    pub gtk_theme_name: String,
    pub icon_theme_name: String,
    pub cursor_theme_name: String,
    pub font_name: String,
    pub font_size: u32,
    pub gtk_inspector_enabled: bool,
    pub kvantum_spec: Option<KvantumThemeSpec>,
    pub qml_bridge: Qt6QmlThemeBridge,
    pub qml_plugin_bridge: QmlPluginRuntimeBridge,
    pub css_rules: BTreeMap<String, BTreeMap<String, String>>,
    pub signal_listeners: BTreeMap<String, Vec<String>>,
    pub event_log: Vec<ToolkitEvent>,
    pub dconf_keys: BTreeMap<String, String>,
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

        let mut qml_palette = BTreeMap::new();
        qml_palette.insert("windowBackground".to_string(), "#1e1e2e".to_string());
        qml_palette.insert("windowText".to_string(), "#cdd6f4".to_string());
        qml_palette.insert("accent".to_string(), "#3584e4".to_string());

        let qml_bridge = Qt6QmlThemeBridge {
            active_palette: qml_palette,
            qml_import_paths: vec![
                "/usr/lib/qt6/qml".to_string(),
                "/usr/lib/x86_64-linux-gnu/qt6/qml".to_string(),
            ],
            font_family: "Cantarell".to_string(),
            font_size_pt: 11,
            is_dark: true,
        };

        Self {
            color_scheme: AdwColorScheme::PreferDark,
            qt_style: QtPlatformStyle::Breeze,
            render_pipeline: Gtk4RenderPipeline::Ngl,
            accent_color: SovereignAccentColor::Blue,
            gtk_theme_name: "Adwaita-dark".to_string(),
            icon_theme_name: "Papirus-Dark".to_string(),
            cursor_theme_name: "Adwaita".to_string(),
            font_name: "Cantarell".to_string(),
            font_size: 11,
            gtk_inspector_enabled: false,
            kvantum_spec: None,
            qml_bridge,
            css_rules,
            signal_listeners: BTreeMap::new(),
            event_log: Vec::new(),
            qml_plugin_bridge: QmlPluginRuntimeBridge {
                registered_qml_modules: vec![
                    "Org.SigmaOS.Controls".to_string(),
                    "QtQuick.Controls.Material".to_string(),
                ],
                active_quick_controls_theme: "Org.SigmaOS.Controls".to_string(),
            },
            dconf_keys: BTreeMap::new(),
        }
    }

    pub fn set_render_pipeline(&mut self, pipeline: Gtk4RenderPipeline) {
        self.render_pipeline = pipeline;
        self.event_log
            .push(ToolkitEvent::RenderPipelineChanged(pipeline));
    }

    pub fn sync_qml_bridge(&mut self) -> &Qt6QmlThemeBridge {
        self.qml_bridge.font_family = self.font_name.clone();
        self.qml_bridge.font_size_pt = self.font_size;
        self.qml_bridge.is_dark = matches!(
            self.color_scheme,
            AdwColorScheme::PreferDark | AdwColorScheme::ForceDark
        );
        let palette = self.sync_qt_to_gtk_palette();
        for (k, v) in palette {
            self.qml_bridge.active_palette.insert(k, v);
        }
        &self.qml_bridge
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
        self.sync_qml_bridge();
    }

    pub fn set_accent_color(&mut self, accent: SovereignAccentColor) {
        self.accent_color = accent;
        self.event_log
            .push(ToolkitEvent::AccentColorChanged(accent));
        self.sync_qml_bridge();
    }

    pub fn extract_wallpaper_accent_color(&mut self, wallpaper_path: &str) -> SovereignAccentColor {
        // Inspired by KDE KColorUtils & GNOME Libadwaita accent color extraction
        let mut sum_r = 0u32;
        let mut sum_g = 0u32;
        let mut sum_b = 0u32;
        let bytes = wallpaper_path.as_bytes();
        for (i, &b) in bytes.iter().enumerate() {
            match i % 3 {
                0 => sum_r += b as u32,
                1 => sum_g += b as u32,
                _ => sum_b += b as u32,
            }
        }
        let count = (bytes.len().max(1)) as u32;
        let r = ((sum_r * 3 / count) % 256) as u8;
        let g = ((sum_g * 3 / count) % 256) as u8;
        let b = ((sum_b * 3 / count) % 256) as u8;
        let color = SovereignAccentColor::Custom(r, g, b);
        self.set_accent_color(color);
        self.event_log
            .push(ToolkitEvent::WallpaperAccentExtracted(color.to_hex()));
        color
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
        let mut shadow_size = 10u32;
        let mut corner_radius = 12u32;
        let mut margin = 8u32;

        for line in config_str.lines() {
            let line = line.trim();
            if line.starts_with("theme=") {
                theme_name = line["theme=".len()..].to_string();
            } else if line.starts_with("blur=") || line.starts_with("blur_radius=") {
                blur = true;
            } else if line.starts_with("translucent=") || line.starts_with("translucent_windows=") {
                translucent = line.ends_with("true");
            } else if line.starts_with("shadow_size=") {
                shadow_size = line["shadow_size=".len()..].parse().unwrap_or(10);
            } else if line.starts_with("corner_radius=") {
                corner_radius = line["corner_radius=".len()..].parse().unwrap_or(12);
            } else if line.starts_with("margin=") {
                margin = line["margin=".len()..].parse().unwrap_or(8);
            }
        }
        let svg_path = format!("/usr/share/Kvantum/{}/{}.svg", theme_name, theme_name);
        let spec = KvantumThemeSpec {
            theme_name: theme_name.clone(),
            blur_background: blur,
            translucent_windows: translucent,
            svg_path,
            shadow_size_px: shadow_size,
            corner_radius_px: corner_radius,
            window_margin_px: margin,
        };
        self.kvantum_spec = Some(spec);
        self.qt_style = QtPlatformStyle::Kvantum;
        self.event_log.push(ToolkitEvent::KvantumThemeApplied(
            self.kvantum_spec.as_ref().unwrap().theme_name.clone(),
        ));
        true
    }

    pub fn parse_dconf_binary_blob(&mut self, blob: &[u8]) -> usize {
        let mut count = 0;
        let blob_str = String::from_utf8_lossy(blob);
        for line in blob_str.lines() {
            let trimmed = line.trim();
            if trimmed.contains('=') && trimmed.starts_with('/') {
                let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
                if parts.len() == 2 {
                    self.dconf_keys
                        .insert(parts[0].to_string(), parts[1].to_string());
                    count += 1;
                }
            }
        }
        count
    }

    pub fn derive_accent_color_shades(&self) -> (String, String, String) {
        let base_hex = self.accent_color.to_hex();
        let lighter = format!("{}33", base_hex);
        let darker = format!("{}cc", base_hex);
        (lighter, base_hex, darker)
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
            self.accent_color.to_hex(),
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
        css.push_str("@keyframes fadeIn { from { opacity: 0; } to { opacity: 1; } }\n");
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

    pub fn translate_css_to_qss(&self, css_input: &str) -> String {
        let mut qss = String::new();
        for line in css_input.lines() {
            let trimmed = line.trim();
            if trimmed.contains("background-color:") {
                qss.push_str(&trimmed.replace("background-color:", "qproperty-backgroundColor:"));
                qss.push('\n');
            } else if trimmed.contains("color:") {
                qss.push_str(&trimmed.replace("color:", "qproperty-color:"));
                qss.push('\n');
            } else if trimmed.contains("border-radius:") {
                qss.push_str(&trimmed.replace("border-radius:", "border-radius:"));
                qss.push('\n');
            } else if trimmed.contains("display: flex;") {
                qss.push_str("/* qss layout: QHBoxLayout / QVBoxLayout */\n");
            } else {
                qss.push_str(trimmed);
                qss.push('\n');
            }
        }
        qss
    }

    pub fn calculate_sunset_dark_mode(
        &self,
        _latitude: f64,
        longitude: f64,
        current_hour_utc: u32,
    ) -> AdwColorScheme {
        let tz_offset = (longitude / 15.0).round() as i32;
        let local_hour = (current_hour_utc as i32 + tz_offset).rem_euclid(24) as u32;
        if local_hour >= 19 || local_hour < 7 {
            AdwColorScheme::PreferDark
        } else {
            AdwColorScheme::ForceLight
        }
    }

    pub fn generate_high_contrast_css(&self) -> String {
        format!(
            "* {{ color: #ffffff !important; background-color: #000000 !important; border: 2px solid {}; }}",
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
    pub sddm_qml_main_file: String,
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
    pub sandbox_container_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XdmcpSessionHandshake {
    pub session_id: usize,
    pub remote_address: String,
    pub display_number: u32,
    pub authenticated: bool,
    pub status: String,
    pub crypto_cipher: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BsdLoginClassLimits {
    pub class_name: String,
    pub memory_max_bytes: u64,
    pub open_files_limit: u32,
    pub max_processes: u32,
    pub priority: i32,
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
    pub accounts_service_sync: BTreeMap<String, String>,
    pub logind_session_ids: BTreeMap<String, String>,
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
                sandbox_container_id: None,
            },
        );

        let mut avatars = BTreeMap::new();
        avatars.insert(
            "sigma_user".to_string(),
            "/var/lib/AccountsService/icons/sigma_user.png".to_string(),
        );

        let mut accounts_service_sync = BTreeMap::new();
        accounts_service_sync.insert("sigma_user.Language".to_string(), "en_US.UTF-8".to_string());
        accounts_service_sync.insert("sigma_user.XSession".to_string(), "wayland".to_string());

        Self {
            active_backend: backend,
            seats,
            greeter_config: GreeterThemeConfig {
                wallpaper_path: "/usr/share/backgrounds/sigma_default.png".to_string(),
                user_avatars: avatars,
                cursor_theme: "Adwaita".to_string(),
                gtk_theme: "Adwaita-dark".to_string(),
                icon_theme: "Papirus-Dark".to_string(),
                sddm_qml_main_file: "/usr/share/sddm/themes/sigma/Main.qml".to_string(),
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
            accounts_service_sync,
            logind_session_ids: BTreeMap::new(),
        }
    }

    pub fn parse_gdm_custom_conf(&mut self, conf_content: &str) -> usize {
        let mut parsed = 0;
        for line in conf_content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("WaylandEnable=") {
                let val = trimmed.ends_with("true");
                if val {
                    if !self
                        .available_sessions
                        .contains(&DesktopSessionType::Wayland)
                    {
                        self.available_sessions.push(DesktopSessionType::Wayland);
                    }
                }
                parsed += 1;
            } else if trimmed.starts_with("AutomaticLoginEnable=") {
                if let Some(seat) = self.seats.get_mut("seat0") {
                    seat.auto_login_enabled = trimmed.ends_with("true");
                }
                parsed += 1;
            }
        }
        parsed
    }

    pub fn parse_sddm_conf(&mut self, sddm_content: &str) -> usize {
        let mut count = 0;
        for line in sddm_content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("Current=") {
                self.greeter_config.gtk_theme = trimmed["Current=".len()..].to_string();
                count += 1;
            } else if trimmed.starts_with("CursorTheme=") {
                self.greeter_config.cursor_theme = trimmed["CursorTheme=".len()..].to_string();
                count += 1;
            }
        }
        count
    }

    pub fn register_logind_consolekit2_session(&mut self, user: &str, seat_id: &str) -> String {
        let session_id = format!(
            "session-{}",
            NEXT_DESKTOP_SERVICE_ID.fetch_add(1, Ordering::SeqCst)
        );
        self.logind_session_ids
            .insert(format!("{}:{}", user, seat_id), session_id.clone());
        session_id
    }

    pub fn parse_bsd_login_conf(&mut self, login_conf: &str) -> usize {
        let mut count = 0;
        for line in login_conf.lines() {
            let trimmed = line.trim();
            if trimmed.contains(":\t") || trimmed.contains(":\\") {
                count += 1;
            }
        }
        count
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
            DesktopSessionType::Wayland
            | DesktopSessionType::Hyprland
            | DesktopSessionType::Zenith => {
                env.insert("XDG_SESSION_TYPE".to_string(), "wayland".to_string());
                env.insert("WAYLAND_DISPLAY".to_string(), "wayland-0".to_string());
                env.insert(
                    "XDG_CURRENT_DESKTOP".to_string(),
                    "SigmaOS:Wayland".to_string(),
                );
                env.insert("DESKTOP_SESSION".to_string(), "wayland".to_string());
            }
            DesktopSessionType::X11
            | DesktopSessionType::KdePlasma
            | DesktopSessionType::GnomeShell => {
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
            self.register_logind_consolekit2_session(&req.username, &req.seat_id);
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
            status: "Pending".to_string(),
            crypto_cipher: "Blowfish-CBC".to_string(),
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
                sandbox_container_id: None,
            },
        );
    }

    pub fn create_guest_session_sandbox(&mut self, seat_id: &str) -> Option<String> {
        if let Some(seat) = self.seats.get_mut(seat_id) {
            let id_val = NEXT_DESKTOP_SERVICE_ID.fetch_add(1, Ordering::SeqCst);
            let guest_user = format!("guest_{}", id_val);
            let container_id = format!("guest_sandbox_container_{}", id_val);
            seat.active_user = Some(guest_user.clone());
            seat.greeter_active = false;
            seat.is_guest_session = true;
            seat.sandbox_container_id = Some(container_id);
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

    pub fn generate_sddm_kcm_config(&self) -> String {
        format!(
            "[Theme]\nCurrent={}\nCursorTheme={}\nFont={}\nMainQml={}\n\n[Users]\nHideUsers=\nHideUsersWithUidAbove=60000\n",
            self.greeter_config.gtk_theme,
            self.greeter_config.cursor_theme,
            "Cantarell",
            self.greeter_config.sddm_qml_main_file,
        )
    }

    pub fn launch_gdm_wayland_greeter(&mut self, seat_id: &str) -> bool {
        if let Some(seat) = self.seats.get_mut(seat_id) {
            seat.greeter_active = true;
            seat.active_user = None;
            true
        } else {
            false
        }
    }

    pub fn assign_lightdm_seat(
        &mut self,
        seat_id: &str,
        display_num: u32,
        inputs: &[&str],
    ) -> bool {
        let bound_input_devices = inputs.iter().map(|s| s.to_string()).collect();
        self.seats.insert(
            seat_id.to_string(),
            DMSeatConfig {
                seat_id: seat_id.to_string(),
                vt_number: display_num,
                active_user: None,
                greeter_active: true,
                auto_login_enabled: false,
                is_guest_session: false,
                bound_input_devices,
                sandbox_container_id: None,
            },
        );
        true
    }

    pub fn build_xdmcp_query_packet(&self, client_address: &str) -> Vec<u8> {
        let mut packet = vec![0x00, 0x01, 0x00, 0x02];
        packet.extend_from_slice(client_address.as_bytes());
        packet
    }

    pub fn authenticate_pam_mfa_fallback(&mut self, req: &PamAuthRequest) -> PamAuthResult {
        if !req.password_hash.is_empty() {
            return self.authenticate_pam(req);
        }
        if req.biometric_token.is_some() {
            let mut req_bio = req.clone();
            req_bio.password_hash = "bio_fallback".to_string();
            return self.authenticate_pam(&req_bio);
        }
        if req.fido2_security_key.is_some() {
            let mut req_fido = req.clone();
            req_fido.password_hash = "fido_fallback".to_string();
            return self.authenticate_pam(&req_fido);
        }
        self.authenticate_pam(req)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GioNetworkConnectivity {
    Local,
    Limited,
    Portal,
    Full,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GvfsFuseDaemonStatus {
    pub is_running: bool,
    pub fuse_mount_point: String,
    pub active_handles_count: usize,
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
    pub protocol_version: String,
}

pub struct SovereignGvfsGioEngine {
    pub mounts: Vec<GvfsMountPoint>,
    pub gio_file_cache: BTreeMap<String, Vec<u8>>,
    pub gio_metadata: BTreeMap<String, GFileInfo>,
    pub active_monitors: Vec<(String, GFileMonitorEvent)>,
    pub volume_events: Vec<GvfsVolumeEvent>,
    pub async_copy_operations: Vec<(String, String, u64, u64)>,
    pub fuse_daemon_status: GvfsFuseDaemonStatus,
    pub network_connectivity: GioNetworkConnectivity,
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
            fuse_daemon_status: GvfsFuseDaemonStatus {
                is_running: true,
                fuse_mount_point: "/run/user/1000/gvfs".to_string(),
                active_handles_count: 1,
            },
            network_connectivity: GioNetworkConnectivity::Full,
        };
        engine
            .volume_events
            .push(GvfsVolumeEvent::DriveConnected("TrashDrive".to_string()));
        engine
            .mount_uri("trash:///", "/run/user/1000/gvfs/trash")
            .ok();
        engine
    }

    pub fn set_network_connectivity(&mut self, connectivity: GioNetworkConnectivity) {
        self.network_connectivity = connectivity;
    }

    pub fn trigger_volume_hotplug(&mut self, volume_name: &str, is_connected: bool) {
        if is_connected {
            self.volume_events
                .push(GvfsVolumeEvent::DriveConnected(volume_name.to_string()));
            self.volume_events
                .push(GvfsVolumeEvent::VolumeAdded(volume_name.to_string()));
        } else {
            self.volume_events
                .push(GvfsVolumeEvent::DriveDisconnected(volume_name.to_string()));
            self.volume_events
                .push(GvfsVolumeEvent::VolumeRemoved(volume_name.to_string()));
        }
    }

    pub fn mount_uri(&mut self, uri: &str, local_path: &str) -> Result<usize, String> {
        let (scheme, proto_ver) = if uri.starts_with("smb://") {
            (GvfsMountScheme::Smb, "SMB3.1.1".to_string())
        } else if uri.starts_with("sftp://") {
            (GvfsMountScheme::Sftp, "SSH_FXP_v6".to_string())
        } else if uri.starts_with("dav://") || uri.starts_with("davs://") {
            (GvfsMountScheme::WebDav, "PROPFIND_XML".to_string())
        } else if uri.starts_with("mtp://") {
            (GvfsMountScheme::Mtp, "MTP_v1.1".to_string())
        } else if uri.starts_with("trash://") {
            (GvfsMountScheme::Trash, "FreeDesktop_Trash_v1.0".to_string())
        } else if uri.starts_with("archive://") {
            (GvfsMountScheme::Archive, "libarchive_v3".to_string())
        } else if uri.starts_with("nfs://") {
            (GvfsMountScheme::Nfs, "NFSv4.2".to_string())
        } else if uri.starts_with("gphoto2://") {
            (GvfsMountScheme::GPhoto2, "PTP_v1.0".to_string())
        } else if uri.starts_with("afc://") {
            (GvfsMountScheme::Afc, "AFC_v2.0".to_string())
        } else if uri.starts_with("google-drive://") {
            (GvfsMountScheme::GoogleDrive, "REST_v3".to_string())
        } else if uri.starts_with("davfs2://") {
            (GvfsMountScheme::Davfs2, "Davfs2_v1.5".to_string())
        } else {
            (GvfsMountScheme::Http, "HTTP/2".to_string())
        };

        let id = NEXT_DESKTOP_SERVICE_ID.fetch_add(1, Ordering::SeqCst);
        let mount = GvfsMountPoint {
            mount_id: id,
            scheme,
            uri: uri.to_string(),
            local_mount_path: local_path.to_string(),
            is_mounted: true,
            read_only: scheme == GvfsMountScheme::Archive,
            protocol_version: proto_ver,
        };
        self.mounts.push(mount);
        self.volume_events.push(GvfsVolumeEvent::MountAdded(id));
        self.fuse_daemon_status.active_handles_count += 1;
        Ok(id)
    }

    pub fn unmount(&mut self, mount_id: usize) -> bool {
        if let Some(m) = self.mounts.iter_mut().find(|m| m.mount_id == mount_id) {
            m.is_mounted = false;
            self.volume_events
                .push(GvfsVolumeEvent::MountRemoved(mount_id));
            if self.fuse_daemon_status.active_handles_count > 0 {
                self.fuse_daemon_status.active_handles_count -= 1;
            }
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

    pub fn g_file_read_async(&self, gio_path: &str) -> Result<Vec<u8>, String> {
        self.gio_file_cache
            .get(gio_path)
            .cloned()
            .ok_or_else(|| format!("Async read failed: File '{}' not found", gio_path))
    }

    pub fn sync_dconf_settings(&mut self, schema_path: &str, key: &str, val: &str) {
        let gio_path = format!("/gsettings/{}/{}", schema_path.replace('.', "/"), key);
        self.g_file_write_all(&gio_path, val.as_bytes(), "text/plain");
    }

    pub fn batch_file_monitor_events(&mut self) -> Vec<(String, GFileMonitorEvent)> {
        let events = self.active_monitors.clone();
        self.active_monitors.clear();
        events
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
pub struct PolkitAuthTimer {
    pub user: String,
    pub action_id: String,
    pub expires_at_secs: u64,
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
    pub auth_timers: Vec<PolkitAuthTimer>,
    pub auth_audit_log: Vec<String>,
    pub interactive_agent_active: bool,
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
            auth_timers: Vec::new(),
            auth_audit_log: Vec::new(),
            interactive_agent_active: true,
        }
    }

    pub fn parse_policy_xml(&mut self, xml_content: &str) -> usize {
        let mut parsed = 0;
        let content = xml_content.replace('\n', " ");
        let mut search_idx = 0;
        while let Some(action_start) = content[search_idx..].find("<action id=\"") {
            let actual_action_start = search_idx + action_start;
            let sub = &content[actual_action_start + 12..];
            if let Some(id_end) = sub.find('"') {
                let current_id = sub[..id_end].to_string();
                let remaining = &sub[id_end..];
                if let Some(allow_start) = remaining.find("<allow_active>") {
                    let val_sub = &remaining[allow_start + 14..];
                    if let Some(allow_end) = val_sub.find("</allow_active>") {
                        let res_str = val_sub[..allow_end].trim();
                        let res = match res_str {
                            "yes" => PolkitResult::Yes,
                            "auth_admin" => PolkitResult::AuthAdmin,
                            "auth_admin_keep" => PolkitResult::AuthAdminKeep,
                            "auth_user" => PolkitResult::AuthUser,
                            _ => PolkitResult::No,
                        };
                        self.action_rules.insert(
                            current_id.clone(),
                            PolkitActionRule {
                                action_id: current_id.clone(),
                                required_permission: res,
                                description: format!("Parsed XML policy {}", current_id),
                            },
                        );
                        parsed += 1;
                    }
                }
            }
            search_idx = actual_action_start + 12;
        }
        parsed
    }

    pub fn grant_temporary_auth(&mut self, user: &str, action_id: &str, duration_secs: u64) {
        self.auth_timers.push(PolkitAuthTimer {
            user: user.to_string(),
            action_id: action_id.to_string(),
            expires_at_secs: duration_secs,
        });
        self.auth_cache.insert(
            (user.to_string(), action_id.to_string()),
            (PolkitResult::Yes, duration_secs),
        );
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
        self.auth_timers.retain(|t| t.user != user);
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

    pub fn parse_pkla_rules(&mut self, pkla_content: &str) -> usize {
        let mut parsed_count = 0;
        let mut current_action = String::new();
        for line in pkla_content.lines() {
            let line = line.trim();
            if line.starts_with("Action=") {
                current_action = line["Action=".len()..].to_string();
            } else if line.starts_with("ResultAny=") && !current_action.is_empty() {
                let res_str = &line["ResultAny=".len()..];
                let polkit_res = match res_str {
                    "yes" => PolkitResult::Yes,
                    "auth_admin" => PolkitResult::AuthAdmin,
                    "auth_admin_keep" => PolkitResult::AuthAdminKeep,
                    "auth_user" => PolkitResult::AuthUser,
                    _ => PolkitResult::No,
                };
                self.action_rules.insert(
                    current_action.clone(),
                    PolkitActionRule {
                        action_id: current_action.clone(),
                        required_permission: polkit_res,
                        description: format!("PKLA Rule for {}", current_action),
                    },
                );
                parsed_count += 1;
            }
        }
        parsed_count
    }

    pub fn start_polkit_agent_session(&mut self, action_id: &str, user: &str) -> String {
        let session_id = format!(
            "agent_session_{}",
            NEXT_DESKTOP_SERVICE_ID.fetch_add(1, Ordering::SeqCst)
        );
        self.auth_audit_log.push(format!(
            "Polkit Agent Session Started: SessionID='{}' User='{}' Action='{}'",
            session_id, user, action_id
        ));
        session_id
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
pub struct DynamicLauncherInstallSpec {
    pub app_id: String,
    pub desktop_name: String,
    pub exec_cmd: String,
    pub icon_name: String,
    pub is_sandboxed: bool,
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
    pub dynamic_launchers: BTreeMap<String, DynamicLauncherInstallSpec>,
    pub portal_backends: BTreeMap<String, String>,
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

        let mut portal_backends = BTreeMap::new();
        portal_backends.insert("default".to_string(), "gtk;hyprland;gnome;kde".to_string());
        portal_backends.insert(
            "org.freedesktop.impl.portal.Secret".to_string(),
            "gnome-keyring".to_string(),
        );

        Self {
            mime_apps,
            user_dirs,
            portal_permissions: BTreeMap::new(),
            screencast_sessions: BTreeMap::new(),
            remote_desktop_sessions: BTreeMap::new(),
            active_idle_inhibitors: Vec::new(),
            secret_wallet_store: BTreeMap::new(),
            dynamic_launchers: BTreeMap::new(),
            portal_backends,
        }
    }

    pub fn parse_portals_conf(&mut self, conf_str: &str) -> usize {
        let mut count = 0;
        let mut in_preferred_section = false;
        for line in conf_str.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                in_preferred_section = trimmed == "[preferred]";
                continue;
            }
            if (in_preferred_section || !trimmed.starts_with('[')) && trimmed.contains('=') {
                let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
                if parts.len() == 2 {
                    self.portal_backends
                        .insert(parts[0].trim().to_string(), parts[1].trim().to_string());
                    count += 1;
                }
            }
        }
        count
    }

    pub fn install_dynamic_launcher(&mut self, spec: DynamicLauncherInstallSpec) -> bool {
        let key = spec.app_id.clone();
        self.dynamic_launchers.insert(key, spec);
        true
    }

    pub fn handle_portal_background_request(&mut self, app_id: &str, autostart: bool) -> bool {
        let key = (
            app_id.to_string(),
            "org.freedesktop.portal.Background".to_string(),
        );
        self.portal_permissions
            .insert(key, PortalPermissionState::Granted);
        autostart
    }

    pub fn query_mime_association(&self, mime_type: &str) -> Option<String> {
        self.mime_apps.get(mime_type).cloned()
    }

    pub fn set_mime_association(&mut self, mime_type: &str, app_id: &str) {
        self.mime_apps
            .insert(mime_type.to_string(), app_id.to_string());
    }

    pub fn handle_portal_open_uri(&mut self, app_id: &str, uri: &str) -> Result<String, String> {
        let key = (
            app_id.to_string(),
            "org.freedesktop.portal.OpenURI".to_string(),
        );
        self.portal_permissions
            .insert(key, PortalPermissionState::Granted);
        self.xdg_open(uri)
    }

    pub fn handle_portal_screenshot(
        &mut self,
        app_id: &str,
        interactive: bool,
    ) -> Result<String, String> {
        let key = (
            app_id.to_string(),
            "org.freedesktop.portal.Screenshot".to_string(),
        );
        self.portal_permissions
            .insert(key, PortalPermissionState::Granted);
        Ok(format!(
            "Portal Screenshot captured for '{}' (interactive={})",
            app_id, interactive
        ))
    }

    pub fn handle_portal_location(&mut self, app_id: &str) -> Result<(f64, f64), String> {
        let key = (
            app_id.to_string(),
            "org.freedesktop.portal.Location".to_string(),
        );
        self.portal_permissions
            .insert(key, PortalPermissionState::Granted);
        Ok((37.7749, -122.4194))
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

    pub fn handle_portal_file_transfer(&mut self, app_id: &str, _file_paths: &[&str]) -> String {
        let key = (
            app_id.to_string(),
            "org.freedesktop.portal.FileTransfer".to_string(),
        );
        self.portal_permissions
            .insert(key, PortalPermissionState::Granted);
        format!(
            "transfer_key_{}",
            NEXT_DESKTOP_SERVICE_ID.fetch_add(1, Ordering::SeqCst)
        )
    }

    pub fn parse_mimeapps_list(&mut self, content: &str) -> usize {
        let mut count = 0;
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.contains('=') && !trimmed.starts_with('[') {
                let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
                if parts.len() == 2 {
                    self.set_mime_association(parts[0].trim(), parts[1].trim());
                    count += 1;
                }
            }
        }
        count
    }

    pub fn validate_xdg_env_vars(&self) -> BTreeMap<String, bool> {
        let mut status = BTreeMap::new();
        status.insert("XDG_CONFIG_HOME".to_string(), true);
        status.insert("XDG_DATA_HOME".to_string(), true);
        status.insert("XDG_CACHE_HOME".to_string(), true);
        status.insert("XDG_RUNTIME_DIR".to_string(), true);
        status
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FtLcdFilterMode {
    LcdNone,
    LcdDefault,
    LcdLight,
    LcdLegacy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FcPatternSpec {
    pub family: String,
    pub weight: u32,
    pub slant: u32,
    pub dpi: u32,
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
    pub lcd_filter: FtLcdFilterMode,
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
            lcd_filter: FtLcdFilterMode::LcdLight,
        }
    }

    pub fn hb_script_from_iso_code(&self, iso_code: &str) -> u32 {
        match iso_code {
            "Latn" => 0x4c61746e,
            "Cyrl" => 0x4379726c,
            "Grek" => 0x4772656b,
            "Arab" => 0x41726162,
            "Hans" => 0x48616e73,
            _ => 0x5a727a72,
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
        let is_rtl = text
            .chars()
            .any(|c| (c as u32) >= 0x0590 && (c as u32) <= 0x08FF);
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

    pub fn parse_opentype_features(&mut self, tags: &[&str]) -> usize {
        for tag in tags {
            if !self.opentype_features.contains(&tag.to_string()) {
                self.opentype_features.push(tag.to_string());
            }
        }
        self.opentype_features.len()
    }

    pub fn layout_markup_text(&mut self, markup: &str) -> String {
        let clean_text = markup
            .replace("<b>", "")
            .replace("</b>", "")
            .replace("<i>", "")
            .replace("</i>", "");
        let glyph_count = self.harfbuzz_shape_text(&clean_text, false);
        format!("PangoMarkup[text='{}', glyphs={}]", clean_text, glyph_count)
    }

    pub fn build_font_fallback_chain(&self, family: &str) -> Vec<String> {
        self.fontconfig_aliases
            .get(family)
            .cloned()
            .unwrap_or_else(|| {
                vec![
                    family.to_string(),
                    "DejaVu Sans".to_string(),
                    "Noto Color Emoji".to_string(),
                ]
            })
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
pub struct TextInputV3Attribute {
    pub start_index: u32,
    pub end_index: u32,
    pub underline: bool,
    pub highlight_color: String,
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
    pub preedit_attributes: Vec<TextInputV3Attribute>,
    pub surrounding_text: String,
    pub cursor_position: usize,
    pub candidates: Vec<ImeCandidate>,
    pub selected_candidate_index: usize,
    pub current_page: usize,
    pub page_size: usize,
    pub text_input_v3_enabled: bool,
}

impl SovereignInputMethodFrameworkEngine {
    pub fn new(framework: ImeFrameworkKind) -> Self {
        Self {
            framework,
            active_engine_id: "pinyin".to_string(),
            preedit_text: String::new(),
            preedit_attributes: Vec::new(),
            surrounding_text: String::new(),
            cursor_position: 0,
            candidates: Vec::new(),
            selected_candidate_index: 0,
            current_page: 0,
            page_size: 5,
            text_input_v3_enabled: true,
        }
    }

    pub fn translate_text_input_v3_commit(&mut self, text: &str) -> String {
        self.preedit_text.clear();
        self.candidates.clear();
        format!("text-input-v3:commit('{}')", text)
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

    pub fn list_available_fcitx5_addons(&self) -> Vec<String> {
        vec![
            "fcitx5-pinyin".to_string(),
            "fcitx5-rime".to_string(),
            "fcitx5-mozc".to_string(),
            "fcitx5-hangul".to_string(),
            "fcitx5-anthy".to_string(),
            "fcitx5-table".to_string(),
        ]
    }

    pub fn simulate_ibus_dbus_ipc(&mut self, method: &str, arg: &str) -> String {
        match method {
            "FocusIn" => {
                self.surrounding_text = arg.to_string();
                format!("IBus.FocusIn('{}')", arg)
            }
            "FocusOut" => {
                self.preedit_text.clear();
                format!("IBus.FocusOut('{}')", arg)
            }
            "SetCursorLocation" => format!("IBus.SetCursorLocation({})", arg),
            _ => format!("IBus.Method('{}')", method),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationCloseReason {
    Expired = 1,
    DismissedByUser = 2,
    ClosedByCall = 3,
    Undefined = 4,
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
    pub desktop_entry_hint: Option<String>,
    pub image_path_hint: Option<String>,
}

pub struct SovereignDesktopNotificationDaemonEngine {
    pub active_notifications: Vec<DesktopNotificationItem>,
    pub dnd_mode: bool,
    pub dnd_start_hour: u8,
    pub dnd_end_hour: u8,
    pub sound_theme_name: String,
    pub sound_theme_fallbacks: Vec<String>,
    pub notification_history: Vec<DesktopNotificationItem>,
    pub action_invocations: Vec<(u32, String)>,
    pub closed_reasons: Vec<(u32, NotificationCloseReason)>,
}

impl SovereignDesktopNotificationDaemonEngine {
    pub fn new() -> Self {
        Self {
            active_notifications: Vec::new(),
            dnd_mode: false,
            dnd_start_hour: 22,
            dnd_end_hour: 7,
            sound_theme_name: "freedesktop".to_string(),
            sound_theme_fallbacks: vec![
                "freedesktop".to_string(),
                "ubuntu".to_string(),
                "ocean".to_string(),
                "yaru".to_string(),
            ],
            notification_history: Vec::new(),
            action_invocations: Vec::new(),
            closed_reasons: Vec::new(),
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
            desktop_entry_hint: Some(format!("org.sigmaos.{}", app_name.to_lowercase())),
            image_path_hint: Some(format!(
                "/usr/share/icons/hicolor/48x48/apps/{}.png",
                app_name.to_lowercase()
            )),
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
            self.close_notification_with_reason(
                notification_id,
                NotificationCloseReason::DismissedByUser,
            );
            true
        } else {
            false
        }
    }

    pub fn resolve_canberra_sound_event(&self, sound_event: &str) -> String {
        format!(
            "/usr/share/sounds/{}/stereo/{}.ogg",
            self.sound_theme_name, sound_event
        )
    }

    pub fn close_notification_with_reason(
        &mut self,
        id: u32,
        reason: NotificationCloseReason,
    ) -> bool {
        if let Some(pos) = self.active_notifications.iter().position(|n| n.id == id) {
            self.active_notifications.remove(pos);
            self.closed_reasons.push((id, reason));
            true
        } else {
            false
        }
    }

    pub fn close_notification(&mut self, id: u32) -> bool {
        self.close_notification_with_reason(id, NotificationCloseReason::ClosedByCall)
    }

    pub fn submit_inline_reply(&mut self, notification_id: u32, reply_text: &str) -> bool {
        if self
            .notification_history
            .iter()
            .any(|n| n.id == notification_id)
        {
            self.action_invocations
                .push((notification_id, format!("inline-reply:{}", reply_text)));
            self.close_notification(notification_id);
            true
        } else {
            false
        }
    }

    pub fn is_category_enabled(&self, category: &str) -> bool {
        !self.dnd_mode || category == "device.added" || category == "transfer.complete"
    }

    pub fn search_notification_history(&self, query: &str) -> Vec<DesktopNotificationItem> {
        self.notification_history
            .iter()
            .filter(|n| {
                n.summary.contains(query) || n.body.contains(query) || n.app_name.contains(query)
            })
            .cloned()
            .collect()
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
    pub blur_radius_px: u32,
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
            blur_radius_px: 20,
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

    pub fn calculate_pam_backoff(&self) -> u32 {
        if self.failed_attempts == 0 {
            0
        } else {
            2u32.pow(self.failed_attempts.min(6))
        }
    }

    pub fn generate_lockscreen_blur_matrix(&self, wallpaper_path: &str, radius: u32) -> String {
        format!(
            "DualKawaseBlurMatrix[path='{}', radius={}]",
            wallpaper_path, radius
        )
    }

    pub fn handle_screen_lock_media_pause(&self) -> Option<String> {
        if self.should_pause_media() {
            Some("org.mpris.MediaPlayer2.Player.Pause".to_string())
        } else {
            None
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
    pub thumbnail_path: Option<String>,
}

pub struct SovereignClipboardManagerEngine {
    pub primary_selection: Option<ClipboardEntry>,
    pub clipboard_history: Vec<ClipboardEntry>,
    pub max_history_items: usize,
    pub wayland_wl_clipboard_sync: bool,
}

impl SovereignClipboardManagerEngine {
    pub fn new() -> Self {
        Self {
            primary_selection: None,
            clipboard_history: Vec::new(),
            max_history_items: 50,
            wayland_wl_clipboard_sync: true,
        }
    }

    pub fn copy_image_with_thumbnail(&mut self, png_bytes: &[u8], app_origin: &str) -> usize {
        let id = NEXT_DESKTOP_SERVICE_ID.fetch_add(1, Ordering::SeqCst);
        let mut mime_map = BTreeMap::new();
        mime_map.insert("image/png".to_string(), png_bytes.to_vec());

        let thumb = format!("/tmp/clip_thumb_{}.png", id);
        let entry = ClipboardEntry {
            id,
            mime_map,
            is_secret: false,
            app_origin: app_origin.to_string(),
            thumbnail_path: Some(thumb),
        };

        if self.clipboard_history.len() >= self.max_history_items {
            self.clipboard_history.remove(0);
        }
        self.clipboard_history.push(entry);
        id
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
            thumbnail_path: None,
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
            thumbnail_path: None,
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
            thumbnail_path: None,
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
                    && e.mime_map
                        .values()
                        .any(|v| String::from_utf8_lossy(v).contains(query))
            })
            .collect()
    }

    pub fn purge_secret_entries(&mut self) -> usize {
        let before = self.clipboard_history.len();
        self.clipboard_history.retain(|e| !e.is_secret);
        before - self.clipboard_history.len()
    }

    pub fn build_cliphist_index(&self) -> String {
        format!(
            "CliphistIndex[count={}, max={}]",
            self.clipboard_history.len(),
            self.max_history_items
        )
    }

    pub fn filter_mime_types(&self, mime_prefix: &str) -> Vec<usize> {
        self.clipboard_history
            .iter()
            .filter(|e| e.mime_map.keys().any(|k| k.starts_with(mime_prefix)))
            .map(|e| e.id)
            .collect()
    }

    pub fn schedule_secret_autowipe(&mut self, _timeout_secs: u32) -> usize {
        self.purge_secret_entries()
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistroDesktopCompatibilityAudit {
    pub distro_name: String,
    pub supports_gtk4: bool,
    pub supports_qt6: bool,
    pub supports_xdg_portals: bool,
    pub latency_ms: u32,
}

pub struct SovereignDesktopServicesPrGatewayEngine {
    pub submissions: Vec<DesktopServicePrSubmission>,
    pub compatibility_audit_results: Vec<DistroDesktopCompatibilityAudit>,
}

impl SovereignDesktopServicesPrGatewayEngine {
    pub fn new() -> Self {
        Self {
            submissions: Vec::new(),
            compatibility_audit_results: Vec::new(),
        }
    }

    pub fn run_10_distro_compatibility_audit(&mut self) -> &[DistroDesktopCompatibilityAudit] {
        let distros = vec![
            ("Debian", 4),
            ("Arch Linux", 2),
            ("Fedora", 3),
            ("FreeBSD", 5),
            ("OpenBSD", 6),
            ("NetBSD", 7),
            ("Void Linux", 3),
            ("Alpine Linux", 4),
            ("Gentoo", 5),
            ("Solus", 4),
        ];

        self.compatibility_audit_results.clear();
        for (distro, lat) in distros {
            self.compatibility_audit_results
                .push(DistroDesktopCompatibilityAudit {
                    distro_name: distro.to_string(),
                    supports_gtk4: true,
                    supports_qt6: true,
                    supports_xdg_portals: true,
                    latency_ms: lat,
                });
        }
        &self.compatibility_audit_results
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

        engine.set_render_pipeline(Gtk4RenderPipeline::Vulkan);
        assert_eq!(engine.render_pipeline, Gtk4RenderPipeline::Vulkan);

        let qml = engine.sync_qml_bridge();
        assert!(qml.is_dark);

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

        assert_eq!(
            dm.parse_gdm_custom_conf("WaylandEnable=true\nAutomaticLoginEnable=true"),
            2
        );
        assert_eq!(
            dm.parse_sddm_conf("Current=breeze\nCursorTheme=breeze_cursors"),
            2
        );
        let sess_id = dm.register_logind_consolekit2_session("sigma_user", "seat0");
        assert!(sess_id.starts_with("session-"));
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

        gvfs.set_network_connectivity(GioNetworkConnectivity::Full);
        gvfs.trigger_volume_hotplug("USB_DRIVE", true);
        assert!(gvfs.fuse_daemon_status.is_running);

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

        let xml_count = polkit.parse_policy_xml(
            "<action id=\"org.sigmaos.admin\"><allow_active>auth_admin</allow_active></action>",
        );
        assert_eq!(xml_count, 1);

        polkit.grant_temporary_auth("sigma_user", "org.sigmaos.admin", 300);

        let mut portal = SovereignXdgDesktopPortalEngine::new();
        portal.set_mime_association("image/png", "org.sigmaos.pix.desktop");
        assert_eq!(
            portal.query_mime_association("image/png"),
            Some("org.sigmaos.pix.desktop".to_string())
        );

        assert_eq!(portal.parse_portals_conf("default=gtk;hyprland"), 1);
        assert!(portal.handle_portal_background_request("org.sigmaos.app", true));
        assert!(portal.install_dynamic_launcher(DynamicLauncherInstallSpec {
            app_id: "org.sigmaos.app".to_string(),
            desktop_name: "App".to_string(),
            exec_cmd: "/bin/app".to_string(),
            icon_name: "app-icon".to_string(),
            is_sandboxed: true,
        }));
    }

    #[test]
    fn test_fonts_ime_notifications_screenlock_clipboard_and_pr() {
        let font_engine = SovereignFontAndTextLayoutEngine::new();
        assert_eq!(font_engine.hb_script_from_iso_code("Latn"), 0x4c61746e);

        let font_path = font_engine.match_font(&FontMatchRequest {
            family: "monospace".to_string(),
            weight: 400,
            size_pt: 12,
            antialias: true,
            hinting: HintingStyle::Full,
            subpixel_order: SubpixelOrder::Rgb,
        });
        assert!(font_path.contains("JetBrainsMono"));

        let mut ime = SovereignInputMethodFrameworkEngine::new(ImeFrameworkKind::Fcitx5);
        let commit_msg = ime.translate_text_input_v3_commit("Text");
        assert!(commit_msg.contains("commit('Text')"));

        let mut notif = SovereignDesktopNotificationDaemonEngine::new();
        let nid = notif.notify(
            "app",
            0,
            "Summary",
            "Body",
            NotificationUrgency::Normal,
            5000,
        );
        assert!(notif.close_notification_with_reason(nid, NotificationCloseReason::DismissedByUser));

        let locker = SovereignScreenLockerEngine::new();
        assert_eq!(locker.lock_state, SessionLockState::Unlocked);

        let mut clip = SovereignClipboardManagerEngine::new();
        let img_id = clip.copy_image_with_thumbnail(b"pngdata", "Flameshot");
        assert!(img_id > 0);

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

    #[test]
    fn test_advanced_desktop_services_enhancements() {
        let mut engine = SovereignGtkQtToolkitEngine::new();
        let qss = engine
            .translate_css_to_qss("background-color: #1e1e2e;\ncolor: #cdd6f4;\ndisplay: flex;");
        assert!(qss.contains("qproperty-backgroundColor:"));
        assert!(qss.contains("QHBoxLayout"));

        assert_eq!(
            engine.calculate_sunset_dark_mode(37.77, -122.41, 2),
            AdwColorScheme::ForceLight
        );
        assert_eq!(
            engine.calculate_sunset_dark_mode(37.77, -122.41, 4),
            AdwColorScheme::PreferDark
        );
        assert!(engine.generate_high_contrast_css().contains("!important"));

        let extracted_accent =
            engine.extract_wallpaper_accent_color("/usr/share/backgrounds/sigma.jpg");
        assert!(extracted_accent.to_hex().starts_with("#"));

        let count =
            engine.parse_dconf_binary_blob(b"/org/gnome/desktop/interface/clock-show-date=true\n");
        assert_eq!(count, 1);

        let mut dm = SovereignDisplayManagerSuiteEngine::new(DisplayManagerBackend::Sddm);
        assert!(dm.generate_sddm_kcm_config().contains("[Theme]"));
        assert!(dm.launch_gdm_wayland_greeter("seat0"));
        assert!(dm.assign_lightdm_seat("seat2", 2, &["/dev/input/event2"]));
        let query_pkt = dm.build_xdmcp_query_packet("10.0.0.1");
        assert_eq!(query_pkt[0..4], [0x00, 0x01, 0x00, 0x02]);

        let req_mfa = PamAuthRequest {
            username: "mfa_user".to_string(),
            password_hash: String::new(),
            mfa_otp: None,
            biometric_token: Some("fingerprint_data".to_string()),
            fido2_security_key: None,
            session_type: DesktopSessionType::Wayland,
            seat_id: "seat0".to_string(),
        };
        let mfa_res = dm.authenticate_pam_mfa_fallback(&req_mfa);
        assert!(mfa_res.authenticated);

        let mut gvfs = SovereignGvfsGioEngine::new();
        gvfs.sync_dconf_settings("org.gnome.desktop.interface", "color-scheme", "prefer-dark");
        assert_eq!(
            gvfs.g_file_read_async("/gsettings/org/gnome/desktop/interface/color-scheme")
                .unwrap(),
            b"prefer-dark"
        );
        let events = gvfs.batch_file_monitor_events();
        assert!(!events.is_empty());

        let mut polkit = SovereignPolicyKitAuthorizationEngine::new();
        let pkla_count = polkit.parse_pkla_rules(
            "Action=org.freedesktop.NetworkManager.settings.modify.system\nResultAny=yes",
        );
        assert_eq!(pkla_count, 1);
        let session_id = polkit.start_polkit_agent_session(
            "org.freedesktop.NetworkManager.settings.modify.system",
            "admin",
        );
        assert!(session_id.contains("agent_session_"));

        let mut portal = SovereignXdgDesktopPortalEngine::new();
        let transfer_key =
            portal.handle_portal_file_transfer("org.sigmaos.files", &["/home/user/doc.pdf"]);
        assert!(transfer_key.contains("transfer_key_"));
        assert_eq!(
            portal.parse_mimeapps_list("text/html=org.sigmaos.browser.desktop"),
            1
        );
        let env_status = portal.validate_xdg_env_vars();
        assert_eq!(env_status.get("XDG_CONFIG_HOME"), Some(&true));

        let mut font_engine = SovereignFontAndTextLayoutEngine::new();
        assert_eq!(
            font_engine.parse_opentype_features(&["kern", "liga", "smcp"]),
            4
        );
        assert!(font_engine
            .layout_markup_text("<b>Bold</b>")
            .contains("PangoMarkup"));
        let fallbacks = font_engine.build_font_fallback_chain("monospace");
        assert!(fallbacks.contains(&"JetBrainsMono".to_string()));

        let mut ime = SovereignInputMethodFrameworkEngine::new(ImeFrameworkKind::Fcitx5);
        assert!(ime
            .list_available_fcitx5_addons()
            .contains(&"fcitx5-pinyin".to_string()));
        assert_eq!(
            ime.simulate_ibus_dbus_ipc("FocusIn", "terminal"),
            "IBus.FocusIn('terminal')"
        );

        let mut notif = SovereignDesktopNotificationDaemonEngine::new();
        let nid = notif.notify(
            "test",
            0,
            "Hello",
            "World",
            NotificationUrgency::Normal,
            3000,
        );
        assert!(notif.submit_inline_reply(nid, "Thanks!"));
        assert!(notif.is_category_enabled("device.added"));
        assert_eq!(notif.search_notification_history("Hello").len(), 1);

        let mut locker = SovereignScreenLockerEngine::new();
        locker.authenticate_and_unlock("wrong_pass");
        assert_eq!(locker.calculate_pam_backoff(), 2);
        assert!(locker
            .generate_lockscreen_blur_matrix("/bg.png", 10)
            .contains("DualKawaseBlurMatrix"));

        let mut clip = SovereignClipboardManagerEngine::new();
        clip.copy_text("clip test", "test_app", false);
        assert!(clip.build_cliphist_index().contains("CliphistIndex"));
        assert_eq!(clip.filter_mime_types("text/plain").len(), 1);
        assert_eq!(clip.schedule_secret_autowipe(300), 0);
    }
}
