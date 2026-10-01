#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
//! Omarchy (Modern Arch + Hyprland + Omakub Inspiration) Subsystem for SigmaOS
//!
//! Inspired by Omarchy 1.1.0:
//! - Declarative Hyprland Wayland Compositor Configuration & Dwindle Tiling
//! - Dynamic Theme Switcher with Tokyo-Night, Catppuccin, Gruvbox, Nord, Everforest, Kanagawa
//! - Web2App PWA Launcher & Sandbox Generation
//! - Interactive Keybinding Fuzzy-Finder (Wofi / Rofi Parity)
//! - GPU & NVIDIA Early-KMS Hardware Acceleration Configuration
//! - Fast Terminal & Development Environment Provisioner

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;
use std::format;
use std::vec;

/// Supported Omarchy Curated Themes
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum OmarchyTheme {
    TokyoNight,
    Catppuccin,
    Gruvbox,
    Nord,
    Everforest,
    Kanagawa,
}

impl OmarchyTheme {
    pub fn name(&self) -> &'static str {
        match self {
            Self::TokyoNight => "tokyo-night",
            Self::Catppuccin => "catppuccin",
            Self::Gruvbox => "gruvbox",
            Self::Nord => "nord",
            Self::Everforest => "everforest",
            Self::Kanagawa => "kanagawa",
        }
    }

    pub fn accent_color(&self) -> &'static str {
        match self {
            Self::TokyoNight => "#7aa2f7",
            Self::Catppuccin => "#cba6f7",
            Self::Gruvbox => "#fe8019",
            Self::Nord => "#88c0d0",
            Self::Everforest => "#a7c080",
            Self::Kanagawa => "#7e9cd8",
        }
    }

    pub fn bg_color(&self) -> &'static str {
        match self {
            Self::TokyoNight => "#1a1b26",
            Self::Catppuccin => "#1e1e2e",
            Self::Gruvbox => "#282828",
            Self::Nord => "#2e3440",
            Self::Everforest => "#2d353b",
            Self::Kanagawa => "#1f1f28",
        }
    }

    pub fn fg_color(&self) -> &'static str {
        match self {
            Self::TokyoNight => "#c0caf5",
            Self::Catppuccin => "#cdd6f4",
            Self::Gruvbox => "#ebdbb2",
            Self::Nord => "#d8dee9",
            Self::Everforest => "#d3c6aa",
            Self::Kanagawa => "#dcd7ba",
        }
    }
}

/// Keybinding Action
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeybindingDefinition {
    pub modifiers: Vec<String>,
    pub key: String,
    pub command: String,
    pub description: String,
}

/// Web2App Desktop Launcher Definition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebAppSpec {
    pub name: String,
    pub url: String,
    pub icon_url: String,
    pub ozone_wayland: bool,
    pub custom_class: String,
}

/// GPU Acceleration Configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GpuDriverConfig {
    pub is_nvidia: bool,
    pub driver_package: String,
    pub kernel_headers: String,
    pub early_kms_enabled: bool,
    pub egl_wayland: bool,
    pub vaapi_enabled: bool,
}

/// The Core Omarchy Modern Desktop Engine
#[derive(Debug, Clone)]
pub struct OmarchyModernDesktopEngine {
    pub current_theme: OmarchyTheme,
    pub themes_catalog: Vec<OmarchyTheme>,
    pub keybindings: Vec<KeybindingDefinition>,
    pub webapps: BTreeMap<String, WebAppSpec>,
    pub gpu_config: Option<GpuDriverConfig>,
    pub dark_mode: bool,
}

impl OmarchyModernDesktopEngine {
    pub fn new() -> Self {
        let default_bindings = vec![
            KeybindingDefinition {
                modifiers: vec!["SUPER".to_string()],
                key: "Return".to_string(),
                command: "alacritty".to_string(),
                description: "Launch terminal".to_string(),
            },
            KeybindingDefinition {
                modifiers: vec!["SUPER".to_string()],
                key: "B".to_string(),
                command: "chromium --ozone-platform=wayland".to_string(),
                description: "Launch browser".to_string(),
            },
            KeybindingDefinition {
                modifiers: vec!["SUPER".to_string()],
                key: "Space".to_string(),
                command: "wofi --show drun".to_string(),
                description: "Application launcher".to_string(),
            },
            KeybindingDefinition {
                modifiers: vec!["SUPER".to_string(), "SHIFT".to_string(), "CTRL".to_string()],
                key: "Space".to_string(),
                command: "omarchy-theme-next".to_string(),
                description: "Cycle next desktop theme".to_string(),
            },
            KeybindingDefinition {
                modifiers: vec!["SUPER".to_string()],
                key: "K".to_string(),
                command: "omarchy-show-keybindings".to_string(),
                description: "Show interactive keybindings".to_string(),
            },
        ];

        let mut engine = Self {
            current_theme: OmarchyTheme::TokyoNight,
            themes_catalog: vec![
                OmarchyTheme::TokyoNight,
                OmarchyTheme::Catppuccin,
                OmarchyTheme::Gruvbox,
                OmarchyTheme::Nord,
                OmarchyTheme::Everforest,
                OmarchyTheme::Kanagawa,
            ],
            keybindings: default_bindings,
            webapps: BTreeMap::new(),
            gpu_config: None,
            dark_mode: true,
        };

        // Register default modern webapps inspired by Omarchy
        engine.register_webapp(
            "WhatsApp",
            "https://web.whatsapp.com/",
            "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/whatsapp.png",
        );
        engine.register_webapp(
            "ChatGPT",
            "https://chatgpt.com/",
            "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/chatgpt.png",
        );
        engine.register_webapp(
            "GitHub",
            "https://github.com/",
            "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/github-light.png",
        );
        engine.register_webapp(
            "YouTube",
            "https://youtube.com/",
            "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/youtube.png",
        );

        engine
    }

    /// Cycle to next curated theme
    pub fn cycle_next_theme(&mut self) -> OmarchyTheme {
        let current_idx = self
            .themes_catalog
            .iter()
            .position(|t| t == &self.current_theme)
            .unwrap_or(0);
        let next_idx = (current_idx + 1) % self.themes_catalog.len();
        self.current_theme = self.themes_catalog[next_idx].clone();
        self.current_theme.clone()
    }

    /// Generate declarative Wayland / Hyprland styling configuration
    pub fn generate_hyprland_theme_config(&self) -> String {
        format!(
            r#"# Omarchy Autogenerated Hyprland Look & Feel
            theme_name = {}
            variable = rgb({})
            variable = rgb({})
            
            general {{
                gaps_in = 6
                gaps_out = 12
                border_size = 2
                col.active_border = rgba({}ee) rgba({}ee) 45deg
                col.inactive_border = rgba({}aa)
                layout = dwindle
            }}
            
            decoration {{
                rounding = 10
                blur {{
                    enabled = true
                    size = 5
                    passes = 2
                }}
            }}
"#,
            self.current_theme.name(),
            self.current_theme.accent_color().trim_start_matches('#'),
            self.current_theme.bg_color().trim_start_matches('#'),
            self.current_theme.accent_color().trim_start_matches('#'),
            self.current_theme.fg_color().trim_start_matches('#'),
            self.current_theme.bg_color().trim_start_matches('#'),
        )
    }

    /// Generate Alacritty terminal configuration for the active theme
    pub fn generate_alacritty_theme_config(&self) -> String {
        format!(
            r#"[colors.primary]
background = "{}"
foreground = "{}"

[colors.cursor]
text = "{}"
cursor = "{}"
"#,
            self.current_theme.bg_color(),
            self.current_theme.fg_color(),
            self.current_theme.bg_color(),
            self.current_theme.accent_color()
        )
    }

    /// Register a Web2App PWA launcher
    pub fn register_webapp(&mut self, name: &str, url: &str, icon_url: &str) {
        let spec = WebAppSpec {
            name: name.to_string(),
            url: url.to_string(),
            icon_url: icon_url.to_string(),
            ozone_wayland: true,
            custom_class: name.to_string(),
        };
        self.webapps.insert(name.to_string(), spec);
    }

    /// Generate  entry content for Web2App launcher
    pub fn generate_desktop_entry(&self, app_name: &str) -> Option<String> {
        self.webapps.get(app_name).map(|app| {
            format!(
                r#"[Desktop Entry]
Version=1.0
Name={}
Comment=Omarchy Web2App for {}
Exec=chromium --new-window --ozone-platform=wayland --app="{}" --name="{}" --class="{}"
Terminal=false
Type=Application
Icon={}
StartupNotify=true
"#,
                app.name, app.name, app.url, app.custom_class, app.custom_class, app.icon_url
            )
        })
    }

    /// Format keybindings for interactive display/searching
    pub fn export_keybindings_guide(&self) -> Vec<String> {
        self.keybindings
            .iter()
            .map(|b| {
                let combo = if b.modifiers.is_empty() {
                    b.key.clone()
                } else {
                    format!("{} + {}", b.modifiers.join(" + "), b.key)
                };
                format!("{:<25} -> {:<30} # {}", combo, b.command, b.description)
            })
            .collect()
    }

    /// Detect and configure hardware acceleration
    pub fn configure_nvidia_early_kms(
        &mut self,
        gpu_name: &str,
        kernel_flavor: &str,
    ) -> GpuDriverConfig {
        let is_turing_or_newer = gpu_name.contains("RTX") || gpu_name.contains("GTX 16");
        let driver_pkg = if is_turing_or_newer {
            "nvidia-open-dkms".to_string()
        } else {
            "nvidia-dkms".to_string()
        };

        let headers = match kernel_flavor {
            "zen" => "linux-zen-headers".to_string(),
            "lts" => "linux-lts-headers".to_string(),
            "hardened" => "linux-hardened-headers".to_string(),
            _ => "linux-headers".to_string(),
        };

        let config = GpuDriverConfig {
            is_nvidia: true,
            driver_package: driver_pkg,
            kernel_headers: headers,
            early_kms_enabled: true,
            egl_wayland: true,
            vaapi_enabled: true,
        };

        self.gpu_config = Some(config.clone());
        config
    }
}

impl Default for OmarchyModernDesktopEngine {
    fn default() -> Self {
        Self::new()
    }
}


/// Sovereign Agent Definition (inspired by omacom/omarchy: ori-agent, hermes-agent, openclaw-agent, add-default-agent)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SovereignAgentKind {
    Ori,
    Hermes,
    OpenClaw,
    Claude,
    Codex,
    Grok,
    Agy,
    Copilot,
    Custom(String),
}

impl SovereignAgentKind {
    pub fn from_str(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "ori" => Self::Ori,
            "hermes" => Self::Hermes,
            "openclaw" => Self::OpenClaw,
            "claude" => Self::Claude,
            "codex" => Self::Codex,
            "grok" => Self::Grok,
            "agy" => Self::Agy,
            "copilot" => Self::Copilot,
            other => Self::Custom(other.to_string()),
        }
    }

    pub fn binary_name(&self) -> String {
        match self {
            Self::Ori => "ori".to_string(),
            Self::Hermes => "hermes".to_string(),
            Self::OpenClaw => "openclaw".to_string(),
            Self::Claude => "claude".to_string(),
            Self::Codex => "codex".to_string(),
            Self::Grok => "grok".to_string(),
            Self::Agy => "agy".to_string(),
            Self::Copilot => "copilot".to_string(),
            Self::Custom(s) => s.clone(),
        }
    }

    pub fn interactive_flag(&self) -> &'static str {
        match self {
            Self::Ori | Self::Agy | Self::OpenClaw => "--interactive",
            Self::Copilot => "--interactive",
            Self::Hermes => "--agent-prompt",
            _ => "--prompt-interactive",
        }
    }
}

/// Factory Reset & Snapshot Guardian (inspired by omacom/omarchy: factory-reset-requires-snapshot)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FactoryResetGuardian {
    pub factory_snapshot_label: String,
    pub snapshot_present: bool,
    pub btrfs_subvolume_root: String,
    pub btrfs_subvolume_factory: String,
}

impl FactoryResetGuardian {
    pub fn new() -> Self {
        Self {
            factory_snapshot_label: "@factory-clean".to_string(),
            snapshot_present: true,
            btrfs_subvolume_root: "/@".to_string(),
            btrfs_subvolume_factory: "/@factory-clean".to_string(),
        }
    }

    pub fn can_perform_factory_reset(&self) -> bool {
        self.snapshot_present && !self.btrfs_subvolume_factory.is_empty()
    }

    pub fn plan_rollback_instructions(&self) -> Vec<String> {
        vec![
            format!("btrfs subvolume snapshot -r {} {}", self.btrfs_subvolume_root, "/@pre-reset-backup"),
            format!("btrfs subvolume delete {}", self.btrfs_subvolume_root),
            format!("btrfs subvolume snapshot {} {}", self.btrfs_subvolume_factory, self.btrfs_subvolume_root),
            "systemctl reboot".to_string(),
        ]
    }
}

/// Hardware Quirk & Device Adaptation Engine (inspired by omacom/omarchy: be211-wifi7-quattro, asus-rog, framework16, dell-xps-oled)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardwareQuirkAdapter {
    pub pci_id: String,
    pub device_name: String,
    pub workaround_modprobe: Option<String>,
    pub recommended_kernel_params: Vec<String>,
}

impl HardwareQuirkAdapter {
    /// Detect Intel BE200 / BE211 WiFi 7 chipsets and resolve kernel firmware assert bugs (Linux Bug 221675)
    pub fn probe_intel_be211_wifi7(pci_id: &str) -> Option<Self> {
        if pci_id.contains("8086:272b") || pci_id.contains("8086:2723") {
            Some(Self {
                pci_id: pci_id.to_string(),
                device_name: "Intel BE200/BE211 Wi-Fi 7 Controller".to_string(),
                workaround_modprobe: Some("options iwlwifi disable_eht=1".to_string()),
                recommended_kernel_params: vec!["iwlwifi.disable_eht=1".to_string()],
            })
        } else {
            None
        }
    }

    /// Framework 16 & ASUS ROG Keyboard RGB / Backlight Quirk
    pub fn probe_rgb_keyboard(device_name: &str) -> Option<Self> {
        if device_name.to_lowercase().contains("framework16") || device_name.to_lowercase().contains("asus-rog") {
            Some(Self {
                pci_id: "usb:input-rgb".to_string(),
                device_name: device_name.to_string(),
                workaround_modprobe: Some("options asus_wmi fnlock_default=1".to_string()),
                recommended_kernel_params: vec!["asus_wmi.fnlock_default=1".to_string()],
            })
        } else {
            None
        }
    }
}

/// Fail-Closed Passwordless Sudo Expiry Guard (inspired by omacom/omarchy: security/nopasswd-expiry-fail-closed)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordlessSudoExpiryGuard {
    pub expiry_seconds: u64,
    pub armed: bool,
    pub fail_closed: bool,
}

impl PasswordlessSudoExpiryGuard {
    pub fn new(duration_secs: u64) -> Self {
        Self {
            expiry_seconds: duration_secs,
            armed: true,
            fail_closed: true,
        }
    }

    pub fn verify_access(&self, elapsed_secs: u64) -> bool {
        if !self.armed && self.fail_closed {
            return false;
        }
        elapsed_secs < self.expiry_seconds
    }
}

/// Dynamic Wallpaper & Color Palette Engine (inspired by omarchy: pywal / matugen integration)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OmarchyWallpaperManager {
    pub current_wallpaper_path: String,
    pub extracted_palette: Vec<String>,
}

impl OmarchyWallpaperManager {
    pub fn new(wallpaper_path: &str) -> Self {
        Self {
            current_wallpaper_path: wallpaper_path.to_string(),
            extracted_palette: vec![
                "#1a1b26".to_string(),
                "#f7768e".to_string(),
                "#9ece6a".to_string(),
                "#e0af68".to_string(),
                "#7aa2f7".to_string(),
                "#bb9af7".to_string(),
                "#7dcfff".to_string(),
                "#a9b1d6".to_string(),
            ],
        }
    }

    pub fn set_wallpaper(&mut self, path: &str) {
        self.current_wallpaper_path = path.to_string();
    }

    pub fn generate_hyprpaper_config(&self) -> String {
        format!(
            "preload = {}\nwallpaper = ,{}\nipc = on\n",
            self.current_wallpaper_path, self.current_wallpaper_path
        )
    }
}

/// Omarchy Font & Typography Installer
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OmarchyFontInstaller {
    pub installed_nerd_fonts: Vec<String>,
}

impl OmarchyFontInstaller {
    pub fn new() -> Self {
        Self {
            installed_nerd_fonts: vec![
                "JetBrainsMonoNerdFont".to_string(),
                "FiraCodeNerdFont".to_string(),
                "HackNerdFont".to_string(),
                "Inter".to_string(),
            ],
        }
    }

    pub fn install_font(&mut self, font_name: &str) {
        if !self.installed_nerd_fonts.contains(&font_name.to_string()) {
            self.installed_nerd_fonts.push(font_name.to_string());
        }
    }

    pub fn generate_fontconfig_xml(&self) -> String {
        format!(
            r#"<?xml version="1.0"?>
<!DOCTYPE fontconfig SYSTEM "fonts.dtd">
<fontconfig>
  <alias>
    <family>monospace</family>
    <prefer>
      <family>JetBrainsMono Nerd Font</family>
    </prefer>
  </alias>
</fontconfig>"#
        )
    }
}

impl Default for OmarchyFontInstaller {
    fn default() -> Self {
        Self::new()
    }
}

/// PipeWire Bluetooth Audio LDAC/AptX HD Auto-Switching Engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OmarchyBluetoothAudioAutoSwitch {
    pub auto_ldac_enabled: bool,
    pub preferred_codec: String,
}

impl OmarchyBluetoothAudioAutoSwitch {
    pub fn new() -> Self {
        Self {
            auto_ldac_enabled: true,
            preferred_codec: "ldac".to_string(),
        }
    }

    pub fn resolve_codec_for_device(&self, device_name: &str) -> String {
        if self.auto_ldac_enabled && (device_name.contains("Sony") || device_name.contains("WH-1000") || device_name.contains("LDAC")) {
            "ldac".to_string()
        } else if device_name.contains("AptX") {
            "aptx_hd".to_string()
        } else {
            "aac".to_string()
        }
    }
}

impl Default for OmarchyBluetoothAudioAutoSwitch {
    fn default() -> Self {
        Self::new()
    }
}

/// Unified Kernel Image (UKI) systemd-boot Generator
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OmarchySystemdBootGenerator {
    pub default_entry: String,
    pub timeout_seconds: u32,
}

impl OmarchySystemdBootGenerator {
    pub fn new() -> Self {
        Self {
            default_entry: "omarchy-linux-zen.conf".to_string(),
            timeout_seconds: 3,
        }
    }

    pub fn generate_boot_entry(&self, title: &str, kernel: &str, initrd: &str, params: &str) -> String {
        format!(
            "title {}\nlinux {}\ninitrd {}\noptions {}\n",
            title, kernel, initrd, params
        )
    }
}

impl Default for OmarchySystemdBootGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Curated Omarchy Package Bundle Installer
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OmarchyPackageInstaller {
    pub core_packages: Vec<String>,
}

impl OmarchyPackageInstaller {
    pub fn new() -> Self {
        Self {
            core_packages: vec![
                "hyprland".to_string(),
                "waybar".to_string(),
                "swaync".to_string(),
                "rofi-wayland".to_string(),
                "thunar".to_string(),
                "kitty".to_string(),
                "alacritty".to_string(),
                "pipewire".to_string(),
                "hyprpaper".to_string(),
            ],
        }
    }

    pub fn add_package(&mut self, pkg: &str) {
        if !self.core_packages.contains(&pkg.to_string()) {
            self.core_packages.push(pkg.to_string());
        }
    }
}

impl Default for OmarchyPackageInstaller {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// OMARCHY TOGGLES, IDLE, NIGHT LIGHT, AND SCREENSAVER SUBSYSTEM
// =========================================================================

/// Supported Omarchy Toggle Items
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OmarchyToggleState {
    Nightlight,
    NotificationSilencing,
    Idle,
    CrashCapture,
    Screensaver,
    Bar,
    Touchpad,
    Touchscreen,
    Suspend,
    HybridGpu,
}

impl OmarchyToggleState {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Nightlight => "nightlight",
            Self::NotificationSilencing => "notification silencing",
            Self::Idle => "idle",
            Self::CrashCapture => "crash-capture",
            Self::Screensaver => "screensaver",
            Self::Bar => "bar",
            Self::Touchpad => "touchpad",
            Self::Touchscreen => "touchscreen",
            Self::Suspend => "suspend",
            Self::HybridGpu => "hybrid gpu",
        }
    }

    pub fn flag_filename(&self) -> &'static str {
        match self {
            Self::Nightlight => "nightlight-off",
            Self::NotificationSilencing => "dnd-off",
            Self::Idle => "idle-off",
            Self::CrashCapture => "crash-capture-off",
            Self::Screensaver => "screensaver-off",
            Self::Bar => "bar-off",
            Self::Touchpad => "touchpad-off",
            Self::Touchscreen => "touchscreen-off",
            Self::Suspend => "suspend-off",
            Self::HybridGpu => "hybrid-gpu-off",
        }
    }

    pub fn hotkey(&self) -> Option<&'static str> {
        match self {
            Self::Nightlight => Some("Super + Ctrl + N"),
            Self::NotificationSilencing => Some("Super + Ctrl + ,"),
            Self::Idle => Some("Super + Ctrl + I"),
            Self::Bar => Some("Super + Shift + Space"),
            Self::Touchpad => Some("XF86TouchpadToggle"),
            _ => None,
        }
    }
}

/// Omarchy Toggle Manager controlling state flags under `~/.local/state/omarchy/toggles/`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OmarchyToggleManager {
    pub disabled_flags: Vec<String>,
}

impl OmarchyToggleManager {
    pub fn new() -> Self {
        Self {
            disabled_flags: Vec::new(),
        }
    }

    pub fn is_toggle_disabled(&self, toggle: OmarchyToggleState) -> bool {
        self.disabled_flags.contains(&toggle.flag_filename().to_string())
    }

    pub fn is_toggle_enabled(&self, toggle: OmarchyToggleState) -> bool {
        !self.is_toggle_disabled(toggle)
    }

    pub fn toggle(&mut self, toggle: OmarchyToggleState) -> bool {
        let flag = toggle.flag_filename().to_string();
        if self.disabled_flags.contains(&flag) {
            self.disabled_flags.retain(|f| f != &flag);
            true // Enabled
        } else {
            self.disabled_flags.push(flag);
            false // Disabled
        }
    }

    pub fn omarchy_toggle_enabled_cmd(&self, flag_name: &str) -> bool {
        self.disabled_flags.iter().any(|f| f == flag_name)
    }

    pub fn generate_status_json(&self) -> String {
        format!(
            "{{\"idle_disabled\": {}, \"screensaver_disabled\": {}, \"nightlight_disabled\": {}, \"dnd_active\": {}}}",
            self.is_toggle_disabled(OmarchyToggleState::Idle),
            self.is_toggle_disabled(OmarchyToggleState::Screensaver),
            self.is_toggle_disabled(OmarchyToggleState::Nightlight),
            self.is_toggle_disabled(OmarchyToggleState::NotificationSilencing),
        )
    }
}

impl Default for OmarchyToggleManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Omarchy Shell Idle & Lock Configuration (`~/.config/omarchy/shell.json`)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OmarchyIdleLockManager {
    pub screensaver_delay_secs: u32,
    pub lock_delay_secs: u32,
    pub stay_awake: bool,
    pub active_terminal_emulator: String,
}

impl OmarchyIdleLockManager {
    pub fn new() -> Self {
        Self {
            screensaver_delay_secs: 150,
            lock_delay_secs: 300,
            stay_awake: false,
            active_terminal_emulator: "Kitty".to_string(),
        }
    }

    pub fn set_stay_awake(&mut self, stay_awake: bool) {
        self.stay_awake = stay_awake;
    }

    pub fn should_trigger_screensaver(&self, idle_time_secs: u32) -> bool {
        !self.stay_awake && idle_time_secs >= self.screensaver_delay_secs && idle_time_secs < self.lock_delay_secs
    }

    pub fn should_trigger_lock(&self, idle_time_secs: u32) -> bool {
        !self.stay_awake && idle_time_secs >= self.lock_delay_secs
    }

    pub fn generate_shell_json(&self) -> String {
        format!(
            "{{\n  \"version\": 1,\n  \"idle\": {{\n    \"screensaver\": {},\n    \"lock\": {}\n  }}\n}}",
            self.screensaver_delay_secs, self.lock_delay_secs
        )
    }

    pub fn execute_lock_screen(&self) -> String {
        "omarchy-lockscreen --blank --reset-layout --lock-1password".to_string()
    }
}

impl Default for OmarchyIdleLockManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Omarchy Night Light Controller (hyprsunset integration)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OmarchyNightLightConfig {
    pub active_temperature_k: u32,
    pub profile_schedule_time: Option<String>,
}

impl OmarchyNightLightConfig {
    pub fn new() -> Self {
        Self {
            active_temperature_k: 6500,
            profile_schedule_time: None,
        }
    }

    pub fn toggle_night_light(&mut self) -> u32 {
        if self.active_temperature_k == 6500 {
            self.active_temperature_k = 4000;
        } else {
            self.active_temperature_k = 6500;
        }
        self.active_temperature_k
    }

    pub fn generate_hyprsunset_conf(&self) -> String {
        if let Some(ref time) = self.profile_schedule_time {
            format!(
                "profile {{\n    time = {}\n    temperature = {}\n}}",
                time, self.active_temperature_k
            )
        } else {
            format!(
                "profile {{\n    temperature = {}\n}}",
                self.active_temperature_k
            )
        }
    }
}

impl Default for OmarchyNightLightConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Omarchy Bar Indicators Widget (`omarchy.indicators`)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OmarchyBarIndicatorsConfig {
    pub always_show: bool,
    pub active_indicators: Vec<String>,
}

impl OmarchyBarIndicatorsConfig {
    pub fn new() -> Self {
        Self {
            always_show: false,
            active_indicators: Vec::new(),
        }
    }

    pub fn update_indicator(&mut self, name: &str, active: bool) {
        if active {
            if !self.active_indicators.contains(&name.to_string()) {
                self.active_indicators.push(name.to_string());
            }
        } else {
            self.active_indicators.retain(|i| i != name);
        }
    }

    pub fn is_visible(&self, name: &str) -> bool {
        self.always_show || self.active_indicators.contains(&name.to_string())
    }
}

impl Default for OmarchyBarIndicatorsConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_omarchy_theme_cycling() {
        let mut engine = OmarchyModernDesktopEngine::new();
        assert_eq!(engine.current_theme, OmarchyTheme::TokyoNight);
        let next = engine.cycle_next_theme();
        assert_eq!(next, OmarchyTheme::Catppuccin);
        assert_eq!(engine.current_theme, OmarchyTheme::Catppuccin);
    }

    #[test]
    fn test_omarchy_hyprland_config_gen() {
        let engine = OmarchyModernDesktopEngine::new();
        let config = engine.generate_hyprland_theme_config();
        assert!(config.contains("tokyo-night"));
        assert!(config.contains("layout = dwindle"));
    }

    #[test]
    fn test_omarchy_web2app_registration() {
        let mut engine = OmarchyModernDesktopEngine::new();
        engine.register_webapp("Slack", "https://app.slack.com/", "https://example.com/slack.png");
        let desktop = engine.generate_desktop_entry("Slack").unwrap();
        assert!(desktop.contains("Name=Slack"));
        assert!(desktop.contains("--ozone-platform=wayland"));
    }

    #[test]
    fn test_omarchy_nvidia_early_kms() {
        let mut engine = OmarchyModernDesktopEngine::new();
        let gpu = engine.configure_nvidia_early_kms("NVIDIA GeForce RTX 4080", "zen");
        assert_eq!(gpu.driver_package, "nvidia-open-dkms");
        assert_eq!(gpu.kernel_headers, "linux-zen-headers");
        assert!(gpu.early_kms_enabled);
    }

    #[test]
    fn test_omarchy_wallpaper_manager() {
        let mut wp = OmarchyWallpaperManager::new("/usr/share/backgrounds/tokyo.png");
        assert_eq!(wp.current_wallpaper_path, "/usr/share/backgrounds/tokyo.png");
        wp.set_wallpaper("/usr/share/backgrounds/nord.png");
        assert_eq!(wp.current_wallpaper_path, "/usr/share/backgrounds/nord.png");
        let conf = wp.generate_hyprpaper_config();
        assert!(conf.contains("preload = /usr/share/backgrounds/nord.png"));
    }

    #[test]
    fn test_omarchy_font_and_bluetooth_audio() {
        let mut font = OmarchyFontInstaller::new();
        font.install_font("HackNerdFont");
        assert!(font.installed_nerd_fonts.contains(&"HackNerdFont".to_string()));
        let xml = font.generate_fontconfig_xml();
        assert!(xml.contains("JetBrainsMono Nerd Font"));

        let bt = OmarchyBluetoothAudioAutoSwitch::new();
        assert_eq!(bt.resolve_codec_for_device("Sony WH-1000XM5"), "ldac");
        assert_eq!(bt.resolve_codec_for_device("Generic AptX Headset"), "aptx_hd");
    }

    #[test]
    fn test_omarchy_boot_and_packages() {
        let gen = OmarchySystemdBootGenerator::new();
        let entry = gen.generate_boot_entry("Omarchy Zen", "/vmlinuz-linux-zen", "/initramfs-linux-zen.img", "quiet splash");
        assert!(entry.contains("title Omarchy Zen"));
        assert!(entry.contains("linux /vmlinuz-linux-zen"));

        let mut pkg = OmarchyPackageInstaller::new();
        pkg.add_package("fastfetch");
        assert!(pkg.core_packages.contains(&"fastfetch".to_string()));
    }

    #[test]
    fn test_omarchy_toggles_and_idle() {
        let mut toggle_mgr = OmarchyToggleManager::new();
        assert!(toggle_mgr.is_toggle_enabled(OmarchyToggleState::Nightlight));

        // Toggle nightlight off
        assert!(!toggle_mgr.toggle(OmarchyToggleState::Nightlight)); // returns false when disabled
        assert!(toggle_mgr.is_toggle_disabled(OmarchyToggleState::Nightlight));
        assert!(toggle_mgr.omarchy_toggle_enabled_cmd("nightlight-off"));

        // Status JSON check
        let json = toggle_mgr.generate_status_json();
        assert!(json.contains("\"nightlight_disabled\": true"));

        // Toggle back on
        assert!(toggle_mgr.toggle(OmarchyToggleState::Nightlight));
        assert!(toggle_mgr.is_toggle_enabled(OmarchyToggleState::Nightlight));

        // Idle & Lock
        let mut idle = OmarchyIdleLockManager::new();
        assert!(!idle.should_trigger_screensaver(100));
        assert!(idle.should_trigger_screensaver(180));
        assert!(idle.should_trigger_lock(300));

        idle.set_stay_awake(true);
        assert!(!idle.should_trigger_screensaver(180));
        assert!(!idle.should_trigger_lock(300));

        let lock_cmd = idle.execute_lock_screen();
        assert!(lock_cmd.contains("omarchy-lockscreen"));
    }

    #[test]
    fn test_omarchy_nightlight_and_indicators() {
        let mut nl = OmarchyNightLightConfig::new();
        assert_eq!(nl.active_temperature_k, 6500);
        let warm = nl.toggle_night_light();
        assert_eq!(warm, 4000);
        let conf = nl.generate_hyprsunset_conf();
        assert!(conf.contains("temperature = 4000"));

        let mut bar = OmarchyBarIndicatorsConfig::new();
        assert!(!bar.is_visible("stay_awake"));
        bar.update_indicator("stay_awake", true);
        assert!(bar.is_visible("stay_awake"));

        bar.always_show = true;
        assert!(bar.is_visible("dnd"));
    }
}
