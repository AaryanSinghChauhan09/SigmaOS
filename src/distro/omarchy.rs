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
    RosePine,
    Dracula,
    Solarized,
    Oxide,
    Cyberpunk,
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
            Self::RosePine => "rose-pine",
            Self::Dracula => "dracula",
            Self::Solarized => "solarized",
            Self::Oxide => "oxide",
            Self::Cyberpunk => "cyberpunk",
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
            Self::RosePine => "#ebbcba",
            Self::Dracula => "#bd93f9",
            Self::Solarized => "#268bd2",
            Self::Oxide => "#00adb5",
            Self::Cyberpunk => "#ff007f",
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
            Self::RosePine => "#191724",
            Self::Dracula => "#282a36",
            Self::Solarized => "#002b36",
            Self::Oxide => "#222831",
            Self::Cyberpunk => "#0d0221",
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
            Self::RosePine => "#e0def4",
            Self::Dracula => "#f8f8f2",
            Self::Solarized => "#839496",
            Self::Oxide => "#eeeeee",
            Self::Cyberpunk => "#00f5d4",
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OmarchyNerdFont {
    FiraCode,
    JetBrainsMono,
    Hack,
    Meslo,
}

impl OmarchyNerdFont {
    pub fn font_family(&self) -> &'static str {
        match self {
            Self::FiraCode => "FiraCode Nerd Font",
            Self::JetBrainsMono => "JetBrainsMono Nerd Font",
            Self::Hack => "Hack Nerd Font",
            Self::Meslo => "MesloLGS Nerd Font",
        }
    }
}

#[derive(Debug, Clone)]
pub struct OmarchyTerminalFontConfig {
    pub active_font: OmarchyNerdFont,
    pub font_size_pt: f32,
}

impl OmarchyTerminalFontConfig {
    pub fn new() -> Self {
        Self {
            active_font: OmarchyNerdFont::JetBrainsMono,
            font_size_pt: 11.0,
        }
    }

    pub fn set_font(&mut self, font: OmarchyNerdFont, size_pt: f32) {
        self.active_font = font;
        self.font_size_pt = size_pt;
    }
}

impl Default for OmarchyTerminalFontConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default)]
pub struct OmarchyNeovimPresetEngine {
    pub lsp_servers: Vec<String>,
}

impl OmarchyNeovimPresetEngine {
    pub fn new() -> Self {
        Self {
            lsp_servers: Vec::new(),
        }
    }

    pub fn register_lsp_server(&mut self, server: &str) -> bool {
        if self.lsp_servers.iter().any(|s| s == server) {
            false
        } else {
            self.lsp_servers.push(server.to_string());
            true
        }
    }
}

#[derive(Debug, Clone)]
pub struct OmarchyAudioPipewireConfig {
    pub quantum_buffer_size: u32,
}

impl OmarchyAudioPipewireConfig {
    pub fn new() -> Self {
        Self {
            quantum_buffer_size: 256,
        }
    }

    pub fn set_low_latency(&mut self, quantum: u32) -> bool {
        if quantum == 0 {
            false
        } else {
            self.quantum_buffer_size = quantum;
            true
        }
    }
}

impl Default for OmarchyAudioPipewireConfig {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// OMARCHY SPEAKER TUNING ENGINE (PipeWire Filter-Chain DSP)
// =========================================================================

/// Record declaring speaker tuning measurements, provenance, and match rules
#[derive(Debug, Clone)]
pub struct SpeakerTuningRecord {
    pub vendor_model: String,
    pub match_sku: Vec<String>,
    pub match_dmi: Vec<String>,
    pub match_command: Option<String>,
    pub sink_pattern: String,
    pub magnitude_rms_db: f32,
    pub bass_group_delay_swing_ms: f32,
    pub limiter_headroom_db: f32,
    pub dynamic_range_delta_lu: f32,
    pub has_limiter_stage: bool,
    pub has_low_end_excursion_cut: bool,
}

impl SpeakerTuningRecord {
    pub fn new(vendor_model: &str, sink_pattern: &str) -> Self {
        Self {
            vendor_model: String::from(vendor_model),
            match_sku: Vec::new(),
            match_dmi: Vec::new(),
            match_command: None,
            sink_pattern: String::from(sink_pattern),
            magnitude_rms_db: 0.8,
            bass_group_delay_swing_ms: 12.5,
            limiter_headroom_db: 3.0,
            dynamic_range_delta_lu: 1.2,
            has_limiter_stage: true,
            has_low_end_excursion_cut: true,
        }
    }
}

/// Dynamic runtime state of the speaker tuning subsystem
#[derive(Debug, Clone)]
pub struct SpeakerTuningState {
    pub installed: bool,
    pub active: bool,
    pub active_sink_name: String,
    pub physical_target_sink: String,
    pub easyeffects_running: bool,
    pub matched_tuning: Option<SpeakerTuningRecord>,
}

pub struct OmarchySpeakerTuningEngine {
    pub available_tunings: Vec<SpeakerTuningRecord>,
    pub state: SpeakerTuningState,
}

impl OmarchySpeakerTuningEngine {
    pub fn new() -> Self {
        let default_tuning = SpeakerTuningRecord {
            vendor_model: String::from("dell-xps-14-9440"),
            match_sku: vec![String::from("0DB9"), String::from("0DBA")],
            match_dmi: vec![String::from("XPS 14 9440"), String::from("Dell Laptops")],
            match_command: None,
            sink_pattern: String::from("alsa_output.pci-0000_00_1f.3.analog-stereo"),
            magnitude_rms_db: 0.65,
            bass_group_delay_swing_ms: 8.4,
            limiter_headroom_db: 3.5,
            dynamic_range_delta_lu: 1.1,
            has_limiter_stage: true,
            has_low_end_excursion_cut: true,
        };

        Self {
            available_tunings: vec![default_tuning],
            state: SpeakerTuningState {
                installed: false,
                active: false,
                active_sink_name: String::from("omarchy_speaker_tuning"),
                physical_target_sink: String::from("alsa_output.pci-0000_00_1f.3.analog-stereo"),
                easyeffects_running: false,
                matched_tuning: None,
            },
        }
    }

    pub fn register_tuning(&mut self, tuning: SpeakerTuningRecord) {
        self.available_tunings.push(tuning);
    }

    /// Evaluates hardware match in order: match_command -> match_sku -> match_dmi
    pub fn match_hardware(&self, sku: &str, dmi: &str, command_predicate_result: bool) -> Option<SpeakerTuningRecord> {
        for tuning in &self.available_tunings {
            if let Some(ref cmd) = tuning.match_command {
                if !cmd.is_empty() && command_predicate_result {
                    return Some(tuning.clone());
                }
            } else if !tuning.match_sku.is_empty() {
                if tuning.match_sku.iter().any(|s| s.eq_ignore_ascii_case(sku)) {
                    return Some(tuning.clone());
                }
            } else if !tuning.match_dmi.is_empty() {
                if tuning.match_dmi.iter().any(|d| dmi.contains(d)) {
                    return Some(tuning.clone());
                }
            }
        }
        None
    }

    /// `omarchy audio tuning on` - installs and activates matching tuning
    pub fn tuning_on(&mut self, sku: &str, dmi: &str, command_predicate_result: bool, force: bool) -> Result<String, &'static str> {
        if self.state.easyeffects_running {
            return Err("EasyEffects is currently running and cannot coexist with a speaker tuning");
        }

        if self.state.installed && self.state.active && !force {
            return Ok(String::from("Tuning is already installed and active (no-op). Pass force=true to re-apply."));
        }

        let matched = match self.match_hardware(sku, dmi, command_predicate_result) {
            Some(t) => t,
            None => return Err("No matching speaker tuning found for this hardware SKU/DMI"),
        };

        // Graph validation: Must end in limiter and cut non-deliverable bass
        if !matched.has_limiter_stage {
            return Err("Invalid tuning graph: Must end in a limiter to prevent clipping");
        }

        if !matched.has_low_end_excursion_cut {
            return Err("Invalid tuning graph: Must cut non-deliverable low-end frequencies");
        }

        // Apply tuning: set active sink, link to physical sink, start PipeWire client service
        self.state.installed = true;
        self.state.active = true;
        self.state.physical_target_sink = matched.sink_pattern.clone();
        self.state.matched_tuning = Some(matched.clone());

        Ok(format!(
            "Successfully enabled speaker tuning for model '{}' targeting sink '{}'. PipeWire service 'omarchy-speaker-tuning.service' running.",
            matched.vendor_model, matched.sink_pattern
        ))
    }

    /// `omarchy audio tuning off` - removes tuning and restores raw physical speaker output
    pub fn tuning_off(&mut self) -> Result<String, &'static str> {
        if !self.state.installed && !self.state.active {
            return Ok(String::from("Speaker tuning is already disabled. Raw physical speakers active."));
        }

        self.state.installed = false;
        self.state.active = false;
        self.state.matched_tuning = None;

        Ok(String::from("Stopped omarchy-speaker-tuning.service and removed tuning graph. Default sink restored to raw physical speakers."))
    }

    /// `omarchy audio tuning status` - queries current tuning status and matching
    pub fn tuning_status(&self) -> String {
        let status_str = if self.state.active { "Active" } else { "Inactive" };
        let installed_str = if self.state.installed { "Installed" } else { "Not Installed" };

        let matched_info = match &self.state.matched_tuning {
            Some(t) => format!(
                "Model: {} | RMS Deviation: {:.2} dB | Headroom: {:.1} dB | Bass Delay Swing: {:.1} ms",
                t.vendor_model, t.magnitude_rms_db, t.limiter_headroom_db, t.bass_group_delay_swing_ms
            ),
            None => String::from("None"),
        };

        format!(
            "Speaker Tuning Subsystem: [{}] [{}]\nVirtual Sink: {}\nTarget Physical Sink: {}\nMatched Hardware: {}",
            status_str, installed_str, self.state.active_sink_name, self.state.physical_target_sink, matched_info
        )
    }
}

impl Default for OmarchySpeakerTuningEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(any(feature = "standalone_test", test)))]
pub use crate::distro::omarchy_inspiration::{
    AiAgentProvider, HerdrAgentTask, OmarchyHerdrAiAgentManager, OmarchyLuaConfigEngine,
    OmarchyPluginMarketplace, OmarchyQuickshellEngine, OmarchyReleaseChannel,
    OmarchyReleaseChannelSnapshotEngine, OmarchySystemThemeStudio, OmarchyThemePalette,
    QuickshellWidget, ShellComponentKind,
};

#[cfg(any(feature = "standalone_test", test))]
#[path = "omarchy_inspiration.rs"]
pub mod omarchy_inspiration;
#[cfg(any(feature = "standalone_test", test))]
#[path = "omarchy_app_ecosystem.rs"]
pub mod omarchy_app_ecosystem;
#[cfg(any(feature = "standalone_test", test))]
pub use omarchy_inspiration::{
    AiAgentProvider, HerdrAgentTask, OmarchyHerdrAiAgentManager, OmarchyLuaConfigEngine,
    OmarchyPluginMarketplace, OmarchyQuickshellEngine, OmarchyReleaseChannel,
    OmarchyReleaseChannelSnapshotEngine, OmarchySystemThemeStudio, OmarchyThemePalette,
    QuickshellWidget, ShellComponentKind,
};
#[cfg(any(feature = "standalone_test", test))]
pub use omarchy_app_ecosystem::*;

#[path = "."]
pub mod distro {
    pub use crate::omarchy_inspiration;
}

/// Omarchy Liveboot ISO & Automated Installer Engine
#[derive(Debug, Clone)]
pub struct OmarchyIsoInstallerEngine {
    pub iso_label: String,
    pub archiso_profile: String,
    pub btrfs_subvolumes: Vec<String>,
    pub auto_install_script: String,
}

impl OmarchyIsoInstallerEngine {
    pub fn new(iso_label: &str) -> Self {
        Self {
            iso_label: iso_label.to_string(),
            archiso_profile: "omarchy-hyprland-omakase".to_string(),
            btrfs_subvolumes: vec![
                "/@".to_string(),
                "/@home".to_string(),
                "/@snapshots".to_string(),
                "/@factory-clean".to_string(),
            ],
            auto_install_script: "/usr/bin/omarchy-install".to_string(),
        }
    }

    pub fn generate_archiso_bootstrap_manifest(&self) -> String {
        format!(
            "LABEL={}\nPROFILE={}\nSUBVOLUMES={:?}\nSCRIPT={}",
            self.iso_label, self.archiso_profile, self.btrfs_subvolumes, self.auto_install_script
        )
    }
}

impl Default for OmarchyIsoInstallerEngine {
    fn default() -> Self {
        Self::new("OMARCHY_2026_LIVE")
    }
}

/// Omarchy Web2App PWA Sandbox & Desktop Launcher Engine
#[derive(Debug, Clone)]
pub struct OmarchyAppLauncherEngine {
    pub registered_apps: Vec<String>,
}

impl OmarchyAppLauncherEngine {
    pub fn new() -> Self {
        Self {
            registered_apps: vec![
                "WhatsApp".to_string(),
                "ChatGPT".to_string(),
                "GitHub".to_string(),
                "YouTube".to_string(),
            ],
        }
    }

    pub fn register_pwa_app(&mut self, app_name: &str) -> bool {
        if self.registered_apps.contains(&app_name.to_string()) {
            false
        } else {
            self.registered_apps.push(app_name.to_string());
            true
        }
    }
}

impl Default for OmarchyAppLauncherEngine {
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
    fn test_omarchy_terminal_font_config() {
        let mut cfg = OmarchyTerminalFontConfig::new();
        cfg.set_font(OmarchyNerdFont::FiraCode, 12.0);
        assert_eq!(cfg.active_font.font_family(), "FiraCode Nerd Font");
        assert_eq!(cfg.font_size_pt, 12.0);
    }

    #[test]
    fn test_omarchy_neovim_preset_engine() {
        let mut nvim = OmarchyNeovimPresetEngine::new();
        assert!(nvim.register_lsp_server("zls"));
        assert!(!nvim.register_lsp_server("zls")); // Duplicate check
        assert!(nvim.lsp_servers.contains(&String::from("zls")));
    }

    #[test]
    fn test_omarchy_pipewire_audio_config() {
        let mut audio = OmarchyAudioPipewireConfig::new();
        assert!(audio.set_low_latency(64));
        assert_eq!(audio.quantum_buffer_size, 64);
        assert!(!audio.set_low_latency(0));
    }

    #[test]
    fn test_omarchy_speaker_tuning_engine() {
        let mut tuning_engine = OmarchySpeakerTuningEngine::new();

        // 1. Hardware Matching (SKU match)
        let matched = tuning_engine.match_hardware("0DB9", "Dell XPS", false);
        assert!(matched.is_some());
        assert_eq!(matched.unwrap().vendor_model, "dell-xps-14-9440");

        // 2. Hardware Matching Order Priority (match_command over SKU/DMI for a tuning)
        let mut custom_tuning = SpeakerTuningRecord::new("custom-laptop-1", "alsa_output.pci-custom");
        custom_tuning.match_command = Some("is_custom_hardware".to_string());
        custom_tuning.match_sku = vec!["0DB9".to_string()];
        // Insert custom tuning at front to test matching priority
        tuning_engine.available_tunings.insert(0, custom_tuning);

        let priority_matched = tuning_engine.match_hardware("0DB9", "Dell XPS", true);
        assert!(priority_matched.is_some());
        assert_eq!(priority_matched.unwrap().vendor_model, "custom-laptop-1");

        // 3. EasyEffects Collision Prevention
        tuning_engine.state.easyeffects_running = true;
        assert!(tuning_engine.tuning_on("0DB9", "Dell XPS", false, false).is_err());
        tuning_engine.state.easyeffects_running = false;

        // 4. Successful Tuning Activation
        let on_res = tuning_engine.tuning_on("0DB9", "Dell XPS", false, false);
        assert!(on_res.is_ok());
        assert!(tuning_engine.state.active);
        assert!(tuning_engine.state.installed);

        // 5. No-Op on duplicate activation unless forced
        let duplicate_res = tuning_engine.tuning_on("0DB9", "Dell XPS", false, false);
        assert!(duplicate_res.unwrap().contains("no-op"));

        // 6. Status Report
        let status = tuning_status_report(&tuning_engine);
        assert!(status.contains("Active"));
        assert!(status.contains("dell-xps-14-9440"));

        // Helper function for status assertion
        fn tuning_status_report(engine: &OmarchySpeakerTuningEngine) -> String {
            engine.tuning_status()
        }

        // 7. Successful Tuning Off
        let off_res = tuning_engine.tuning_off();
        assert!(off_res.is_ok());
        assert!(!tuning_engine.state.active);
        assert!(!tuning_engine.state.installed);
    }
    #[test]
    fn test_omarchy_expanded_themes_and_iso_installer() {
        let mut engine = OmarchyModernDesktopEngine::new();
        engine.current_theme = OmarchyTheme::RosePine;
        assert_eq!(engine.current_theme.name(), "rose-pine");
        assert_eq!(engine.current_theme.accent_color(), "#ebbcba");

        let iso = OmarchyIsoInstallerEngine::default();
        let manifest = iso.generate_archiso_bootstrap_manifest();
        assert!(manifest.contains("OMARCHY_2026_LIVE"));
        assert!(manifest.contains("/@factory-clean"));

        let mut pwa = OmarchyAppLauncherEngine::default();
        assert!(pwa.register_pwa_app("Linear"));
        assert!(!pwa.register_pwa_app("Linear"));
    }
}

// =========================================================================
// OMARCHY & OMAKUB MISSING ECOSYSTEM GAP CLOSURE ENGINES
// =========================================================================


/// Ghostty GPU-accelerated terminal configuration generator engine
pub struct OmarchyGhosttyTerminalConfigEngine {
    pub font_family: String,
    pub font_size: f32,
    pub theme: String,
    pub wayland_native: bool,
}

impl OmarchyGhosttyTerminalConfigEngine {
    pub fn new(theme: &str) -> Self {
        Self {
            font_family: "JetBrainsMono Nerd Font".to_string(),
            font_size: 11.5,
            theme: theme.to_string(),
            wayland_native: true,
        }
    }

    pub fn generate_ghostty_config(&self) -> String {
        format!(
            "font-family = \"{}\"\nfont-size = {}\ntheme = \"{}\"\nwindow-decoration = false\ngtk-single-instance = true\nwayland-backend = {}\n",
            self.font_family, self.font_size, self.theme, self.wayland_native
        )
    }
}

impl Default for OmarchyGhosttyTerminalConfigEngine {
    fn default() -> Self {
        Self::new("tokyonight")
    }
}

/// Fastfetch TUI system information tool config generator engine
pub struct OmarchyFastfetchSystemInfoEngine {
    pub logo: String,
    pub show_kernel: bool,
    pub show_gpu: bool,
    pub show_memory: bool,
}

impl OmarchyFastfetchSystemInfoEngine {
    pub fn new() -> Self {
        Self {
            logo: "arch".to_string(),
            show_kernel: true,
            show_gpu: true,
            show_memory: true,
        }
    }

    pub fn generate_fastfetch_json(&self) -> String {
        format!(
            "{{\n  \"$schema\": \"https://github.com/fastfetch-cli/fastfetch/raw/dev/doc/json_schema.json\",\n  \"logo\": {{\n    \"type\": \"builtin\",\n    \"source\": \"{}\"\n  }},\n  \"modules\": [\n    \"title\",\n    \"separator\",\n    \"os\",\n    \"host\",\n    \"kernel\",\n    \"uptime\",\n    \"packages\",\n    \"shell\",\n    \"display\",\n    \"wm\",\n    \"terminal\",\n    \"cpu\",\n    \"gpu\",\n    \"memory\",\n    \"break\",\n    \"colors\"\n  ]\n}}\n",
            self.logo
        )
    }
}

impl Default for OmarchyFastfetchSystemInfoEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Dynamic Hyprland dwindle tiling layout engine for automatic workspace layout calculations
#[derive(Debug, Clone, PartialEq)]
pub struct OmarchyHyprlandDwindleTilingEngine {
    pub split_ratio: f32,
    pub force_split: u32,
    pub preserve_split: bool,
}

impl OmarchyHyprlandDwindleTilingEngine {
    pub fn new() -> Self {
        Self {
            split_ratio: 1.0,
            force_split: 2,
            preserve_split: true,
        }
    }

    pub fn generate_dwindle_conf(&self) -> String {
        format!(
            "dwindle {{\n  pseudotile = true\n  preserve_split = {}\n  force_split = {}\n  default_split_ratio = {}\n}}\n",
            self.preserve_split, self.force_split, self.split_ratio
        )
    }
}

impl Default for OmarchyHyprlandDwindleTilingEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod omarchy_gap_closure_tests {
    use super::*;

    #[test]
    fn test_omarchy_hyprland_compositor_config_engine() {
        let hypr = OmarchyHyprlandCompositorConfigEngine::new();
        let conf = hypr.render_hyprland_conf();
        assert!(conf.contains("border_size = 2"));
        assert!(conf.contains("windowrulev2 = float, class:^pavucontrol$"));
    }

    #[test]
    fn test_omarchy_mise_and_lazygit_engines() {
        let mise = OmarchyMiseVersionManagerEngine::new();
        assert_eq!(mise.get_tool_version("node").unwrap(), "20.11.0");

        let lazygit = OmarchyLazyGitConfigurationEngine::new();
        let lazy_yml = lazygit.generate_config_yaml();
        assert!(lazy_yml.contains("sideBySideDiff: true"));
    }

    #[test]
    fn test_omarchy_ayu_and_starship_engines() {
        let ayu_dark = OmarchyAyuThemeEngine::ayu_dark();
        assert_eq!(ayu_dark.bg_color, "#0f1419");

        let starship_toml = OmarchyStarshipPromptConfigEngine::generate_starship_toml();
        assert!(starship_toml.contains("truncation_length = 3"));
    }

    #[test]
    fn test_omarchy_ghostty_fastfetch_dwindle_engines() {
        let ghostty = OmarchyGhosttyTerminalConfigEngine::new("catppuccin");
        let ghostty_cfg = ghostty.generate_ghostty_config();
        assert!(ghostty_cfg.contains("theme = \"catppuccin\""));
        assert!(ghostty_cfg.contains("wayland-backend = true"));

        let fastfetch = OmarchyFastfetchSystemInfoEngine::default();
        let ff_json = fastfetch.generate_fastfetch_json();
        assert!(ff_json.contains("\"source\": \"arch\""));
        assert!(ff_json.contains("\"modules\": ["));

        let dwindle = OmarchyHyprlandDwindleTilingEngine::default();
        let dwindle_conf = dwindle.generate_dwindle_conf();
        assert!(dwindle_conf.contains("preserve_split = true"));
        assert!(dwindle_conf.contains("force_split = 2"));
    }
}
