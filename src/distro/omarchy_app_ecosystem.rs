use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// 1. Hyprland Tiling Window Manager Configuration & Keybindings Engine (`Hyprland` parity)
#[derive(Debug, Clone)]
pub struct HyprlandWindowRule {
    pub match_class: String,
    pub action: String,
}

#[derive(Debug, Clone)]
pub struct OmarchyHyprlandCompositorConfigEngine {
    pub border_size: u32,
    pub gaps_in: u32,
    pub gaps_out: u32,
    pub window_rules: Vec<HyprlandWindowRule>,
}

impl OmarchyHyprlandCompositorConfigEngine {
    pub fn new() -> Self {
        Self {
            border_size: 2,
            gaps_in: 5,
            gaps_out: 10,
            window_rules: vec![
                HyprlandWindowRule {
                    match_class: "ghostty".to_string(),
                    action: "opacity 0.95".to_string(),
                },
                HyprlandWindowRule {
                    match_class: "pavucontrol".to_string(),
                    action: "float".to_string(),
                },
            ],
        }
    }

    /// Renders `hyprland.conf` configuration string
    pub fn render_hyprland_conf(&self) -> String {
        let mut conf = String::new();
        conf.push_str(&format!("general {{\n    border_size = {}\n    gaps_in = {}\n    gaps_out = {}\n}}\n", self.border_size, self.gaps_in, self.gaps_out));
        for rule in &self.window_rules {
            conf.push_str(&format!("windowrulev2 = {}, class:^{}$\n", rule.action, rule.match_class));
        }
        conf
    }
}

/// 2. `mise` Polyglot Dev Tool Version Manager Engine (`mise` / `rtx` parity)
#[derive(Debug, Clone)]
pub struct OmarchyMiseVersionManagerEngine {
    pub tool_versions: BTreeMap<String, String>,
}

impl OmarchyMiseVersionManagerEngine {
    pub fn new() -> Self {
        let mut versions = BTreeMap::new();
        versions.insert("node".to_string(), "20.11.0".to_string());
        versions.insert("python".to_string(), "3.12.1".to_string());
        versions.insert("rust".to_string(), "1.77.0".to_string());

        Self { tool_versions: versions }
    }

    pub fn get_tool_version(&self, tool: &str) -> Option<String> {
        self.tool_versions.get(tool).cloned()
    }
}

/// 3. `lazygit` Terminal Git TUI Config Engine (`lazygit` parity)
#[derive(Debug, Clone)]
pub struct OmarchyLazyGitConfigurationEngine {
    pub active_theme: String,
    pub side_by_side_diff: bool,
}

impl OmarchyLazyGitConfigurationEngine {
    pub fn new() -> Self {
        Self {
            active_theme: "ayu-dark".to_string(),
            side_by_side_diff: true,
        }
    }

    pub fn generate_config_yaml(&self) -> String {
        format!(
            "gui:\n  theme:\n    activeBorderColor:\n      - '#e6b450'\n  sideBySideDiff: {}\n",
            self.side_by_side_diff
        )
    }
}

/// 4. Ayu Dark / Light Theme Color Palette Engine (`Ayu` theme parity)
#[derive(Debug, Clone)]
pub struct OmarchyAyuThemeEngine {
    pub bg_color: String,
    pub fg_color: String,
    pub accent_color: String,
}

impl OmarchyAyuThemeEngine {
    pub fn ayu_dark() -> Self {
        Self {
            bg_color: "#0f1419".to_string(),
            fg_color: "#e6e1cf".to_string(),
            accent_color: "#e6b450".to_string(),
        }
    }
}

/// 5. Starship Cross-Shell Prompt Generator Engine (`starship` TOML parity)
#[derive(Debug, Clone)]
pub struct OmarchyStarshipPromptConfigEngine;

impl OmarchyStarshipPromptConfigEngine {
    pub fn generate_starship_toml() -> String {
        r#"[character]
success_symbol = "[➜](bold green)"
error_symbol = "[➜](bold red)"

[directory]
truncation_length = 3
style = "bold cyan"
"#
        .to_string()
    }
}

/// Omarchy CLI Command Group Routing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OmarchyCliGroup {
    Commands,
    Refresh,
    Restart,
    Toggle,
    Theme,
    Bar,
    Plugin,
    Hook,
    Install,
    Launch,
    Capture,
    Reminder,
    Pkg,
    Setup,
    Update,
    Debug,
}

impl OmarchyCliGroup {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Commands => "commands",
            Self::Refresh => "refresh",
            Self::Restart => "restart",
            Self::Toggle => "toggle",
            Self::Theme => "theme",
            Self::Bar => "bar",
            Self::Plugin => "plugin",
            Self::Hook => "hook",
            Self::Install => "install",
            Self::Launch => "launch",
            Self::Capture => "capture",
            Self::Reminder => "reminder",
            Self::Pkg => "pkg",
            Self::Setup => "setup",
            Self::Update => "update",
            Self::Debug => "debug",
        }
    }
}

/// Privilege Escalation Mode for Omarchy Customization Tasks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OmarchyPrivilegeMode {
    Sudo,
    Pkexec,
    NoSudoPrint,
}

/// Omarchy Skill System Engine (CLI Dispatch, Safety Guards, & Privilege Elevation)
#[derive(Debug, Clone)]
pub struct OmarchySkillSystemEngine {
    pub read_only_system_path: String,
    pub user_config_path: String,
    pub privilege_mode: OmarchyPrivilegeMode,
}

impl OmarchySkillSystemEngine {
    pub fn new() -> Self {
        Self {
            read_only_system_path: String::from("/usr/share/omarchy/"),
            user_config_path: String::from("~/.config/"),
            privilege_mode: OmarchyPrivilegeMode::Sudo,
        }
    }

    pub fn validate_file_edit_permission(&self, file_path: &str) -> Result<(), &'static str> {
        if file_path.starts_with(&self.read_only_system_path) || file_path.starts_with("/usr/share/omarchy") {
            Err("CRITICAL: Editing /usr/share/omarchy/ is prohibited for end-user customization! Use ~/.config/ instead.")
        } else {
            Ok(())
        }
    }

    pub fn resolve_cli_command(&self, group: OmarchyCliGroup, action: &str) -> String {
        format!("omarchy {} {}", group.name(), action)
    }

    pub fn select_privilege_escalation(&self, is_interactive_terminal: bool, is_debug: bool) -> OmarchyPrivilegeMode {
        if is_debug {
            OmarchyPrivilegeMode::NoSudoPrint
        } else if is_interactive_terminal {
            OmarchyPrivilegeMode::Sudo
        } else {
            OmarchyPrivilegeMode::Pkexec
        }
    }

    pub fn generate_backup_filepath(&self, config_rel_path: &str, timestamp_sec: u64) -> String {
        format!("{}.bak.{}", config_rel_path, timestamp_sec)
    }
}

impl Default for OmarchySkillSystemEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_omarchy_app_ecosystem() {
        let hypr = OmarchyHyprlandCompositorConfigEngine::new();
        let conf = hypr.render_hyprland_conf();
        assert!(conf.contains("gaps_out = 10"));
        assert!(conf.contains("windowrulev2 = float, class:^pavucontrol$"));

        let mise = OmarchyMiseVersionManagerEngine::new();
        assert_eq!(mise.get_tool_version("rust").unwrap(), "1.77.0");

        let lazygit = OmarchyLazyGitConfigurationEngine::new();
        assert!(lazygit.generate_config_yaml().contains("sideBySideDiff: true"));

        let ayu = OmarchyAyuThemeEngine::ayu_dark();
        assert_eq!(ayu.bg_color, "#0f1419");

        let starship = OmarchyStarshipPromptConfigEngine::generate_starship_toml();
        assert!(starship.contains("success_symbol = \"[➜](bold green)\""));
    }

    #[test]
    fn test_omarchy_skill_system_engine() {
        let engine = OmarchySkillSystemEngine::new();
        assert!(engine.validate_file_edit_permission("~/.config/hypr/bindings.lua").is_ok());
        assert!(engine.validate_file_edit_permission("/usr/share/omarchy/bin/omarchy-theme").is_err());

        assert_eq!(engine.resolve_cli_command(OmarchyCliGroup::Theme, "set catppuccin"), "omarchy theme set catppuccin");
        assert_eq!(engine.resolve_cli_command(OmarchyCliGroup::Refresh, "shell"), "omarchy refresh shell");

        assert_eq!(engine.select_privilege_escalation(true, false), OmarchyPrivilegeMode::Sudo);
        assert_eq!(engine.select_privilege_escalation(false, false), OmarchyPrivilegeMode::Pkexec);
        assert_eq!(engine.select_privilege_escalation(false, true), OmarchyPrivilegeMode::NoSudoPrint);

        let backup = engine.generate_backup_filepath("~/.config/hypr/bindings.lua", 1700000000);
        assert_eq!(backup, "~/.config/hypr/bindings.lua.bak.1700000000");
    }
}
