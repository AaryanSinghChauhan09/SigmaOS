//! Omarchy Dotfiles, Event Hooks & Menu Extensions Engine
//! Handles user dotfiles in ~/.config vs system templates in /usr/share/omarchy,
//! session autostart scripts, event hook triggers, custom menu extensions, and config resets.

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

/// Omarchy event hooks fired during system lifecycle events
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OmarchyHookEvent {
    PostBoot,
    PostUpdate,
    PreRefreshPacman,
    ThemeSet,
    FontSet,
    BatteryLow,
}

impl OmarchyHookEvent {
    pub fn directory_name(&self) -> &'static str {
        match self {
            OmarchyHookEvent::PostBoot => "post-boot.d",
            OmarchyHookEvent::PostUpdate => "post-update.d",
            OmarchyHookEvent::PreRefreshPacman => "pre-refresh-pacman.d",
            OmarchyHookEvent::ThemeSet => "theme-set.d",
            OmarchyHookEvent::FontSet => "font-set.d",
            OmarchyHookEvent::BatteryLow => "battery-low.d",
        }
    }
}

/// Custom Omarchy Menu Extension entry (added in ~/.config/omarchy/extensions/omarchy-menu.jsonc)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OmarchyMenuExtension {
    pub id: String,
    pub icon: String,
    pub label: String,
    pub action: String,
}

/// Omarchy Dotfiles & Event Hooks Manager Engine
pub struct OmarchyDotfileManagerEngine {
    pub user_config_dir: String,
    pub sys_share_dir: String,
    pub registered_hooks: BTreeMap<OmarchyHookEvent, Vec<String>>,
    pub menu_extensions: Vec<OmarchyMenuExtension>,
    pub autostart_commands: Vec<String>,
}

impl OmarchyDotfileManagerEngine {
    pub fn new() -> Self {
        Self {
            user_config_dir: "~/.config/omarchy".to_string(),
            sys_share_dir: "/usr/share/omarchy".to_string(),
            registered_hooks: BTreeMap::new(),
            menu_extensions: Vec::new(),
            autostart_commands: Vec::new(),
        }
    }

    pub fn register_hook_script(&mut self, event: OmarchyHookEvent, script_path: &str) {
        self.registered_hooks
            .entry(event)
            .or_insert_with(Vec::new)
            .push(script_path.to_string());
    }

    pub fn add_menu_extension(&mut self, ext: OmarchyMenuExtension) {
        if let Some(pos) = self.menu_extensions.iter().position(|m| m.id == ext.id) {
            self.menu_extensions[pos] = ext; // Override existing entry
        } else {
            self.menu_extensions.push(ext);
        }
    }

    pub fn add_autostart_command(&mut self, cmd: &str) {
        if !self.autostart_commands.contains(&cmd.to_string()) {
            self.autostart_commands.push(cmd.to_string());
        }
    }

    pub fn execute_hooks(&self, event: OmarchyHookEvent, arg: Option<&str>) -> Vec<String> {
        let mut executed = Vec::new();
        if let Some(scripts) = self.registered_hooks.get(&event) {
            for script in scripts {
                let exec_cmd = match arg {
                    Some(a) => format!("{} {}", script, a),
                    None => script.clone(),
                };
                executed.push(exec_cmd);
            }
        }
        executed
    }

    pub fn reset_configs_to_defaults(&mut self) -> Result<String, &'static str> {
        self.registered_hooks.clear();
        self.menu_extensions.clear();
        self.autostart_commands.clear();
        Ok(format!("Successfully restored dotfiles from {} to {}", self.sys_share_dir, self.user_config_dir))
    }
}

impl Default for OmarchyDotfileManagerEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_omarchy_hooks_execution() {
        let mut engine = OmarchyDotfileManagerEngine::new();
        engine.register_hook_script(
            OmarchyHookEvent::ThemeSet,
            "~/.config/omarchy/hooks/theme-set.d/10-hyprland.sh",
        );
        engine.register_hook_script(
            OmarchyHookEvent::ThemeSet,
            "~/.config/omarchy/hooks/theme-set.d/20-waybar.sh",
        );

        let executed = engine.execute_hooks(OmarchyHookEvent::ThemeSet, Some("rose-pine"));
        assert_eq!(executed.len(), 2);
        assert_eq!(
            executed[0],
            "~/.config/omarchy/hooks/theme-set.d/10-hyprland.sh rose-pine"
        );
        assert_eq!(
            executed[1],
            "~/.config/omarchy/hooks/theme-set.d/20-waybar.sh rose-pine"
        );

        assert_eq!(
            OmarchyHookEvent::ThemeSet.directory_name(),
            "theme-set.d"
        );
        assert_eq!(
            OmarchyHookEvent::PostBoot.directory_name(),
            "post-boot.d"
        );
    }

    #[test]
    fn test_omarchy_menu_extensions_and_autostart() {
        let mut engine = OmarchyDotfileManagerEngine::new();
        let ext1 = OmarchyMenuExtension {
            id: "system_info".to_string(),
            icon: "info".to_string(),
            label: "System Information".to_string(),
            action: "fastfetch".to_string(),
        };
        engine.add_menu_extension(ext1);

        assert_eq!(engine.menu_extensions.len(), 1);
        assert_eq!(engine.menu_extensions[0].action, "fastfetch");

        // Test override on duplicate ID
        let ext1_updated = OmarchyMenuExtension {
            id: "system_info".to_string(),
            icon: "info".to_string(),
            label: "System Info Summary".to_string(),
            action: "fastfetch --verbose".to_string(),
        };
        engine.add_menu_extension(ext1_updated);
        assert_eq!(engine.menu_extensions.len(), 1);
        assert_eq!(engine.menu_extensions[0].label, "System Info Summary");

        // Autostart commands
        engine.add_autostart_command("waybar &");
        engine.add_autostart_command("waybar &"); // duplicate
        engine.add_autostart_command("hyprpaper &");
        assert_eq!(engine.autostart_commands.len(), 2);
    }

    #[test]
    fn test_omarchy_config_reset() {
        let mut engine = OmarchyDotfileManagerEngine::new();
        engine.register_hook_script(OmarchyHookEvent::PostUpdate, "/usr/bin/notify.sh");
        engine.add_autostart_command("dunst &");
        engine.add_menu_extension(OmarchyMenuExtension {
            id: "test".to_string(),
            icon: "test".to_string(),
            label: "Test".to_string(),
            action: "true".to_string(),
        });

        let res = engine.reset_configs_to_defaults();
        assert!(res.is_ok());
        assert!(engine.registered_hooks.is_empty());
        assert!(engine.menu_extensions.is_empty());
        assert!(engine.autostart_commands.is_empty());
    }
}
