// SigmaOS Omarchy Expanded Parity Engine
// Inspired by Omarchy Linux (Modern Arch + Hyprland + Omakub + Neovim + Starship + Ghostty + Zellij)

use std::vec::Vec;

/// Omakub automated software stack installer & development environment bootstrapper.
#[derive(Debug, Clone)]
pub struct OmarchyOmakubBootstrapEngine {
    pub installed_apps: Vec<String>,
    pub docker_enabled: bool,
    pub default_browser: String,
    pub default_terminal: String,
}

impl OmarchyOmakubBootstrapEngine {
    pub fn new() -> Self {
        let mut apps = Vec::new();
        apps.push(String::from("Ghostty"));
        apps.push(String::from("Neovim"));
        apps.push(String::from("Zellij"));
        apps.push(String::from("LazyGit"));
        apps.push(String::from("Obsidian"));
        apps.push(String::from("Brave"));
        apps.push(String::from("Spotify"));
        apps.push(String::from("1Password"));
        Self {
            installed_apps: apps,
            docker_enabled: true,
            default_browser: String::from("Brave"),
            default_terminal: String::from("Ghostty"),
        }
    }

    pub fn is_app_installed(&self, app_name: &str) -> bool {
        self.installed_apps
            .iter()
            .any(|a| a.eq_ignore_ascii_case(app_name))
    }
}

impl Default for OmarchyOmakubBootstrapEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Walker fuzzy application launcher & D-Bus provider engine (websearch, calc, cliphist, emojis).
#[derive(Debug, Clone)]
pub struct OmarchyWalkerLauncherEngine {
    pub active_modules: Vec<String>,
    pub theme_name: String,
    pub placeholder_prompt: String,
}

impl OmarchyWalkerLauncherEngine {
    pub fn new() -> Self {
        Self {
            active_modules: vec![
                "desktop_applications".to_string(),
                "websearch".to_string(),
                "calculator".to_string(),
                "clipboard_history".to_string(),
                "emoji_picker".to_string(),
            ],
            theme_name: "tokyonight".to_string(),
            placeholder_prompt: "Type to launch application or search...".to_string(),
        }
    }

    pub fn generate_walker_config_json(&self) -> String {
        format!(
            "{{\"theme\": \"{}\", \"placeholder\": \"{}\", \"modules\": [\"{}\"]}}",
            self.theme_name,
            self.placeholder_prompt,
            self.active_modules.join("\", \"")
        )
    }
}

impl Default for OmarchyWalkerLauncherEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Ghostty GPU-accelerated terminal configuration generator engine.
#[derive(Debug, Clone)]
pub struct OmarchyGhosttyConfigEngine {
    pub font_family: String,
    pub font_size: u32,
    pub theme_name: String,
    pub window_padding_x: u32,
    pub window_padding_y: u32,
}

impl OmarchyGhosttyConfigEngine {
    pub fn new() -> Self {
        Self {
            font_family: "JetBrainsMono Nerd Font".to_string(),
            font_size: 13,
            theme_name: "catppuccin-mocha".to_string(),
            window_padding_x: 12,
            window_padding_y: 12,
        }
    }

    pub fn generate_ghostty_config(&self) -> String {
        format!(
            "font-family = {}\nfont-size = {}\ntheme = {}\nwindow-padding-x = {}\nwindow-padding-y = {}\ncursor-style = block\ncursor-style-blink = false\n",
            self.font_family, self.font_size, self.theme_name, self.window_padding_x, self.window_padding_y
        )
    }
}

impl Default for OmarchyGhosttyConfigEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Hypridle idle management daemon configuration engine.
#[derive(Debug, Clone)]
pub struct OmarchyHypridleEngine {
    pub lock_timeout_secs: u32,
    pub dpms_off_timeout_secs: u32,
    pub suspend_timeout_secs: u32,
}

impl OmarchyHypridleEngine {
    pub fn new() -> Self {
        Self {
            lock_timeout_secs: 300,     // 5 mins
            dpms_off_timeout_secs: 600, // 10 mins
            suspend_timeout_secs: 1800, // 30 mins
        }
    }

    pub fn generate_hypridle_conf(&self) -> String {
        format!(
            "listener {{\n  timeout = {}\n  on-timeout = hyprlock\n}}\nlistener {{\n  timeout = {}\n  on-timeout = hyprctl dispatch dpms off\n  on-resume = hyprctl dispatch dpms on\n}}\nlistener {{\n  timeout = {}\n  on-timeout = systemctl suspend\n}}\n",
            self.lock_timeout_secs, self.dpms_off_timeout_secs, self.suspend_timeout_secs
        )
    }
}

impl Default for OmarchyHypridleEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Zellij terminal multiplexer session layout & theme manager engine.
#[derive(Debug, Clone)]
pub struct OmarchyZellijSessionManager {
    pub default_layout: String,
    pub active_theme: String,
    pub pane_frames_visible: bool,
}

impl OmarchyZellijSessionManager {
    pub fn new() -> Self {
        Self {
            default_layout: String::from("compact"),
            active_theme: String::from("catppuccin-mocha"),
            pane_frames_visible: false,
        }
    }

    pub fn generate_kdl_layout(&self) -> String {
        format!(
            "layout name=\"{}\" {{\n  default_tab_template {{\n    pane size=1 borderless=true {{\n      plugin location=\"zellij:tab-bar\"\n    }}\n    children\n    pane size=1 borderless=true {{\n      plugin location=\"zellij:status-bar\"\n    }}\n  }}\n  tab name=\"Dev\" {{\n    pane split_direction=\"vertical\" {{\n      pane size=\"60%\" name=\"Editor\"\n      pane size=\"40%\" name=\"Terminal\"\n    }}\n  }}\n}}\ntheme \"{}\"\n",
            self.default_layout, self.active_theme
        )
    }
}

impl Default for OmarchyZellijSessionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Hyprlock lock screen generator engine with background blur and widgets.
#[derive(Debug, Clone)]
pub struct OmarchyHyprlockScreenLocker {
    pub blur_passes: u32,
    pub blur_size: u32,
    pub font_family: String,
    pub pam_module: String,
}

impl OmarchyHyprlockScreenLocker {
    pub fn new() -> Self {
        Self {
            blur_passes: 3,
            blur_size: 8,
            font_family: String::from("JetBrainsMono Nerd Font"),
            pam_module: String::from("hyprlock"),
        }
    }

    pub fn generate_hyprlock_conf(&self) -> String {
        format!(
            "background {{\n  monitor =\n  path = screenshot\n  blur_passes = {}\n  blur_size = {}\n}}\ninput-field {{\n  monitor =\n  size = 250, 50\n  font_family = {}\n  pam_module = {}\n}}\n",
            self.blur_passes, self.blur_size, self.font_family, self.pam_module
        )
    }
}

impl Default for OmarchyHyprlockScreenLocker {
    fn default() -> Self {
        Self::new()
    }
}

/// Rofi application launcher and clipboard manager configuration engine.
#[derive(Debug, Clone)]
pub struct OmarchyRofiAppLauncher {
    pub theme_name: String,
    pub enable_cliphist: bool,
    pub enable_emoji_picker: bool,
}

impl OmarchyRofiAppLauncher {
    pub fn new() -> Self {
        Self {
            theme_name: String::from("tokyonight-rofi"),
            enable_cliphist: true,
            enable_emoji_picker: true,
        }
    }

    pub fn generate_rofi_rasi_theme(&self) -> String {
        format!(
            "* {{\n  bg: #1a1b26;\n  fg: #c0caf5;\n  accent: #7aa2f7;\n  font: \"JetBrainsMono Nerd Font 12\";\n}}\nwindow {{\n  background-color: @bg;\n  border: 2px;\n  border-color: @accent;\n  border-radius: 10px;\n}}\n"
        )
    }
}

impl Default for OmarchyRofiAppLauncher {
    fn default() -> Self {
        Self::new()
    }
}

/// Thunar custom file manager actions engine (Ghostty, Neovim, Image Convert).
#[derive(Debug, Clone)]
pub struct OmarchyThunarFileActions {
    pub custom_actions_count: usize,
}

impl OmarchyThunarFileActions {
    pub fn new() -> Self {
        Self {
            custom_actions_count: 5,
        }
    }

    pub fn generate_uca_xml(&self) -> String {
        String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<actions>\n  <action>\n    <name>Open in Ghostty</name>\n    <command>ghostty --working-directory=%f</command>\n  </action>\n  <action>\n    <name>Open in Neovim</name>\n    <command>ghostty -e nvim %f</command>\n  </action>\n</actions>\n")
    }
}

impl Default for OmarchyThunarFileActions {
    fn default() -> Self {
        Self::new()
    }
}

/// Sovereign Master Suite Coordinator for Expanded Omarchy Parity.
#[derive(Debug, Clone)]
pub struct SovereignOmarchyExpandedParitySuite {
    pub omakub: OmarchyOmakubBootstrapEngine,
    pub walker: OmarchyWalkerLauncherEngine,
    pub ghostty: OmarchyGhosttyConfigEngine,
    pub hypridle: OmarchyHypridleEngine,
    pub zellij: OmarchyZellijSessionManager,
    pub hyprlock: OmarchyHyprlockScreenLocker,
    pub rofi: OmarchyRofiAppLauncher,
    pub thunar: OmarchyThunarFileActions,
}

impl SovereignOmarchyExpandedParitySuite {
    pub fn new() -> Self {
        Self {
            omakub: OmarchyOmakubBootstrapEngine::new(),
            walker: OmarchyWalkerLauncherEngine::new(),
            ghostty: OmarchyGhosttyConfigEngine::new(),
            hypridle: OmarchyHypridleEngine::new(),
            zellij: OmarchyZellijSessionManager::new(),
            hyprlock: OmarchyHyprlockScreenLocker::new(),
            rofi: OmarchyRofiAppLauncher::new(),
            thunar: OmarchyThunarFileActions::new(),
        }
    }

    pub fn verify_suite(&self) -> bool {
        self.omakub.is_app_installed("Ghostty")
            && self
                .walker
                .generate_walker_config_json()
                .contains("desktop_applications")
            && self
                .ghostty
                .generate_ghostty_config()
                .contains("theme = catppuccin-mocha")
            && self.hypridle.generate_hypridle_conf().contains("hyprlock")
            && self.zellij.generate_kdl_layout().contains("compact")
            && self
                .hyprlock
                .generate_hyprlock_conf()
                .contains("blur_passes = 3")
            && self
                .rofi
                .generate_rofi_rasi_theme()
                .contains("JetBrainsMono Nerd Font")
            && self.thunar.generate_uca_xml().contains("Open in Ghostty")
    }
}

impl Default for SovereignOmarchyExpandedParitySuite {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_omarchy_expanded_parity_suite() {
        let suite = SovereignOmarchyExpandedParitySuite::new();
        assert!(suite.verify_suite());
        assert!(suite.omakub.is_app_installed("Neovim"));
        assert!(suite
            .walker
            .generate_walker_config_json()
            .contains("calculator"));
        assert!(suite
            .ghostty
            .generate_ghostty_config()
            .contains("JetBrainsMono"));
        assert!(suite.hypridle.generate_hypridle_conf().contains("dpms off"));
        assert!(suite.zellij.generate_kdl_layout().contains("tab-bar"));
        assert!(suite.hyprlock.generate_hyprlock_conf().contains("hyprlock"));
    }
}
