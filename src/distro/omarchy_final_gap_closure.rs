// SigmaOS Omarchy Final Gap Closure Subsystem
// Zero-dependency Rust implementation covering Chromium Ozone Wayland PWA launchers, Wofi/Rofi fuzzy finder keybindings, NVIDIA Early KMS module flags, and Fail-Closed Sudo Expiry guards.

use crate::klib::string::String;
use crate::klib::vec::Vec;

/// Chromium Ozone Wayland PWA Spec
#[derive(Debug, Clone)]
pub struct PwaSpec {
    pub app_name: String,
    pub target_url: String,
    pub icon_path: String,
    pub custom_class: String,
}

/// Chromium Ozone Wayland PWA Launcher Engine
#[derive(Debug, Clone)]
pub struct OmarchyOzonewaylandChromiumEngine {
    pub registered_pwas: Vec<PwaSpec>,
    pub ozone_platform_flag: String,
}

impl OmarchyOzonewaylandChromiumEngine {
    pub fn new() -> Self {
        let mut pwas = Vec::new();
        pwas.push(PwaSpec {
            app_name: String::from("ChatGPT"),
            target_url: String::from("https://chatgpt.com/"),
            icon_path: String::from("/usr/share/icons/chatgpt.png"),
            custom_class: String::from("ChatGPT"),
        });

        Self {
            registered_pwas: pwas,
            ozone_platform_flag: String::from("--ozone-platform=wayland"),
        }
    }

    pub fn generate_pwa_exec_command(&self, app_name: &str) -> Option<String> {
        self.registered_pwas.iter().find(|p| p.app_name == app_name).map(|p| {
            let mut cmd = String::from("chromium ");
            cmd.push_str(&self.ozone_platform_flag);
            cmd.push_str(" --app=\"");
            cmd.push_str(&p.target_url);
            cmd.push_str("\" --class=\"");
            cmd.push_str(&p.custom_class);
            cmd.push_str("\"");
            cmd
        })
    }
}

impl Default for OmarchyOzonewaylandChromiumEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Wofi / Rofi Interactive Keybinding Guide Spec
#[derive(Debug, Clone)]
pub struct KeybindingGuideEntry {
    pub combo: String,
    pub command: String,
    pub description: String,
}

/// Wofi / Rofi Interactive Fuzzy-Finder Keybindings Engine
#[derive(Debug, Clone)]
pub struct OmarchyWaylandWofiRofiEngine {
    pub entries: Vec<KeybindingGuideEntry>,
}

impl OmarchyWaylandWofiRofiEngine {
    pub fn new() -> Self {
        let mut entries = Vec::new();
        entries.push(KeybindingGuideEntry {
            combo: String::from("SUPER + Return"),
            command: String::from("kitty"),
            description: String::from("Launch terminal"),
        });
        entries.push(KeybindingGuideEntry {
            combo: String::from("SUPER + Space"),
            command: String::from("wofi --show drun"),
            description: String::from("App launcher"),
        });

        Self { entries }
    }

    pub fn format_wofi_dmenu_input(&self) -> String {
        let mut text = String::new();
        for entry in &self.entries {
            text.push_str(&entry.combo);
            text.push_str(" : ");
            text.push_str(&entry.description);
            text.push_str(" -> ");
            text.push_str(&entry.command);
            text.push_str("\n");
        }
        text
    }
}

impl Default for OmarchyWaylandWofiRofiEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// NVIDIA Early KMS Initramfs & EGL Wayland Engine
#[derive(Debug, Clone)]
pub struct OmarchyNvidiaEarlyKmsEngine {
    pub kms_modules: Vec<String>,
    pub egl_wayland_enabled: bool,
    pub dkms_package: String,
}

impl OmarchyNvidiaEarlyKmsEngine {
    pub fn new() -> Self {
        let mut mods = Vec::new();
        mods.push(String::from("nvidia"));
        mods.push(String::from("nvidia_modeset"));
        mods.push(String::from("nvidia_uvm"));
        mods.push(String::from("nvidia_drm"));

        Self {
            kms_modules: mods,
            egl_wayland_enabled: true,
            dkms_package: String::from("nvidia-open-dkms"),
        }
    }

    pub fn render_mkinitcpio_modules_conf(&self) -> String {
        let mut line = String::from("MODULES=(");
        for (i, m) in self.kms_modules.iter().enumerate() {
            line.push_str(m);
            if i + 1 < self.kms_modules.len() {
                line.push_str(" ");
            }
        }
        line.push_str(")");
        line
    }
}

impl Default for OmarchyNvidiaEarlyKmsEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Armed Passwordless Sudo Expiry Guard Engine
#[derive(Debug, Clone)]
pub struct OmarchySudoFailClosedGuardEngine {
    pub timeout_seconds: u64,
    pub is_armed: bool,
    pub fail_closed: bool,
}

impl OmarchySudoFailClosedGuardEngine {
    pub fn new(timeout_seconds: u64) -> Self {
        Self {
            timeout_seconds,
            is_armed: true,
            fail_closed: true,
        }
    }

    pub fn validate_sudo_privilege(&self, elapsed_seconds: u64) -> bool {
        if !self.is_armed && self.fail_closed {
            return false;
        }
        elapsed_seconds < self.timeout_seconds
    }
}

/// Master Omarchy Final Gap Closure Coordinator Suite
#[derive(Debug, Clone)]
pub struct SovereignOmarchyFinalGapClosureSuite {
    pub pwa_launcher: OmarchyOzonewaylandChromiumEngine,
    pub wofi_guide: OmarchyWaylandWofiRofiEngine,
    pub nvidia_kms: OmarchyNvidiaEarlyKmsEngine,
    pub sudo_guard: OmarchySudoFailClosedGuardEngine,
}

impl SovereignOmarchyFinalGapClosureSuite {
    pub fn new() -> Self {
        Self {
            pwa_launcher: OmarchyOzonewaylandChromiumEngine::new(),
            wofi_guide: OmarchyWaylandWofiRofiEngine::new(),
            nvidia_kms: OmarchyNvidiaEarlyKmsEngine::new(),
            sudo_guard: OmarchySudoFailClosedGuardEngine::new(300), // 5 minutes
        }
    }

    pub fn verify_suite(&self) -> bool {
        self.pwa_launcher.generate_pwa_exec_command("ChatGPT").is_some()
            && !self.wofi_guide.format_wofi_dmenu_input().is_empty()
            && !self.nvidia_kms.render_mkinitcpio_modules_conf().is_empty()
            && self.sudo_guard.validate_sudo_privilege(100)
    }
}

impl Default for SovereignOmarchyFinalGapClosureSuite {
    fn default() -> Self {
        Self::new()
    }
}
