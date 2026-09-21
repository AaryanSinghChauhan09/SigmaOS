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
}

/// 6. Omarchy FAQ Helper Engine (Keyboard layouts, Clock format, Capture Dirs, Printing, & Preinstall Removal)
#[derive(Debug, Clone)]
pub struct OmarchyFaqHelperEngine {
    pub kb_layout: String,
    pub kb_options: String,
    pub clock_format: String,
    pub screenshot_dir: String,
    pub screenrecord_dir: String,
}

impl OmarchyFaqHelperEngine {
    pub fn new() -> Self {
        Self {
            kb_layout: "us,fr".to_string(),
            kb_options: "compose:caps,shift:both_capslock_cancel,grp:alts_toggle".to_string(),
            clock_format: "dddd h:mm AP".to_string(),
            screenshot_dir: "~/Pictures/Screenshots".to_string(),
            screenrecord_dir: "~/Videos/Screenrecordings".to_string(),
        }
    }

    pub fn set_keyboard_layouts(&mut self, layouts: &str, options: &str) {
        self.kb_layout = layouts.to_string();
        self.kb_options = options.to_string();
    }

    pub fn set_clock_format(&mut self, format_str: &str) {
        self.clock_format = format_str.to_string();
    }

    pub fn configure_capture_environment(&mut self, screenshot_dir: &str, record_dir: &str) -> Vec<(String, String)> {
        self.screenshot_dir = screenshot_dir.to_string();
        self.screenrecord_dir = record_dir.to_string();
        vec![
            ("OMASNAP_SCREENSHOT_DIR".to_string(), self.screenshot_dir.clone()),
            ("OMARCHY_SCREENRECORD_DIR".to_string(), self.screenrecord_dir.clone()),
        ]
    }

    pub fn sweep_preinstalls<'a>(&self, preinstalled_packages: &'a [&'a str], keep_list: &[&str]) -> Vec<&'a str> {
        preinstalled_packages
            .iter()
            .copied()
            .filter(|pkg| !keep_list.contains(pkg))
            .collect()
    }

    pub fn configure_ipp_printer(&self, address: &str, queue: &str) -> String {
        format!("ipp://{}/{}", address.trim_matches('/'), queue.trim_matches('/'))
    }
}

impl Default for OmarchyFaqHelperEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod faq_tests {
    use super::*;

    #[test]
    fn test_omarchy_faq_helper_engine() {
        let mut faq = OmarchyFaqHelperEngine::new();
        faq.set_keyboard_layouts("us,fr", "grp:alts_toggle");
        assert_eq!(faq.kb_layout, "us,fr");

        faq.set_clock_format("dddd h:mm AP");
        assert_eq!(faq.clock_format, "dddd h:mm AP");

        let env_vars = faq.configure_capture_environment("~/Pictures/Captures", "~/Videos/Captures");
        assert_eq!(env_vars[0].1, "~/Pictures/Captures");

        let preinstalls = &["obsidian", "libreoffice", "ghostty", "pavucontrol"];
        let remaining = faq.sweep_preinstalls(preinstalls, &["ghostty"]);
        assert_eq!(remaining, vec!["obsidian", "libreoffice", "pavucontrol"]);

        let printer_uri = faq.configure_ipp_printer("192.168.1.50", "ipp/print");
        assert_eq!(printer_uri, "ipp://192.168.1.50/ipp/print");
    }
}

/// 7. Omarchy Countdown Timer Reminder Engine (`omarchy reminder <minutes> <msg>`)
#[derive(Debug, Clone)]
pub struct OmarchyCountdownReminder {
    pub id: u64,
    pub duration_minutes: u32,
    pub message: String,
    pub created_timestamp_sec: u64,
    pub expire_timestamp_sec: u64,
    pub is_expired: bool,
}

#[derive(Debug, Clone)]
pub struct OmarchyTimerReminderEngine {
    pub reminders: Vec<OmarchyCountdownReminder>,
    pub next_id: u64,
}

impl OmarchyTimerReminderEngine {
    pub fn new() -> Self {
        Self {
            reminders: Vec::new(),
            next_id: 1,
        }
    }

    /// Set countdown reminder (`Super + Ctrl + R` or `omarchy reminder <minutes> <message>`)
    pub fn add_reminder(&mut self, duration_minutes: u32, message: &str, current_time_sec: u64) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let expire_time = current_time_sec + (duration_minutes as u64 * 60);
        self.reminders.push(OmarchyCountdownReminder {
            id,
            duration_minutes,
            message: message.to_string(),
            created_timestamp_sec: current_time_sec,
            expire_timestamp_sec: expire_time,
            is_expired: false,
        });
        id
    }

    /// List all set reminders (`Super + Ctrl + Alt + R`)
    pub fn list_active_reminders(&self) -> Vec<&OmarchyCountdownReminder> {
        self.reminders.iter().filter(|r| !r.is_expired).collect()
    }

    /// Clear all set reminders (`Super + Ctrl + Shift + R`)
    pub fn clear_all_reminders(&mut self) -> usize {
        let count = self.reminders.len();
        self.reminders.clear();
        count
    }

    /// Parse CLI arguments (`omarchy reminder 7 'Tea ready'`)
    pub fn parse_cli_args(&mut self, args: &[&str], current_time_sec: u64) -> Result<String, &'static str> {
        if args.len() < 2 {
            return Err("Usage: omarchy reminder <minutes> <message>");
        }
        let minutes: u32 = args[0].parse().map_err(|_| "Invalid minutes duration")?;
        let message = args[1];
        let id = self.add_reminder(minutes, message, current_time_sec);
        Ok(format!("Reminder #{}: '{}' set for {} minute(s).", id, message, minutes))
    }
}

impl Default for OmarchyTimerReminderEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod reminder_tests {
    use super::*;

    #[test]
    fn test_omarchy_timer_reminder_engine() {
        let mut engine = OmarchyTimerReminderEngine::new();
        let now = 1700000000;

        let id = engine.add_reminder(7, "Tea ready", now);
        assert_eq!(id, 1);
        assert_eq!(engine.list_active_reminders().len(), 1);

        let cli_res = engine.parse_cli_args(&["15", "Meeting in 15 mins"], now);
        assert!(cli_res.is_ok());
        assert_eq!(engine.list_active_reminders().len(), 2);

        let cleared = engine.clear_all_reminders();
        assert_eq!(cleared, 2);
        assert_eq!(engine.list_active_reminders().len(), 0);
    }
}
