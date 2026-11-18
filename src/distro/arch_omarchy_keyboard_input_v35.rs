// SPDX-License-Identifier: MIT OR Apache-2.0
//! SigmaOS Keyboard Binds & Input Remapping Engine
//!
//! Inspired by Omarchy's keyboard-centric Hyprland bind architecture and
//! Arch Linux's xkbcommon / kanata input remapping.
//! Manages Super-key modifier layers, mouse sensitivity curves, workspace switching,
//! and directional window navigation (hjkl / arrow keys).



/// Modifier key bitflags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyModifier {
    Super,
    Alt,
    Ctrl,
    Shift,
    SuperShift,
    SuperCtrl,
}

impl KeyModifier {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Super => "SUPER",
            Self::Alt => "ALT",
            Self::Ctrl => "CONTROL",
            Self::Shift => "SHIFT",
            Self::SuperShift => "SUPER_SHIFT",
            Self::SuperCtrl => "SUPER_CONTROL",
        }
    }
}

/// Action dispatched by keybind
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindAction {
    Exec(String),
    KillActive,
    ToggleFloating,
    Fullscreen,
    Workspace(u8),
    MoveToWorkspace(u8),
    FocusDirection(String), // "l", "r", "u", "d"
}

/// A registered keyboard shortcut bind
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyBind {
    pub modifier: KeyModifier,
    pub key: String,
    pub action: BindAction,
    pub description: String,
}

impl KeyBind {
    pub fn new(modifier: KeyModifier, key: &str, action: BindAction, desc: &str) -> Self {
        Self {
            modifier,
            key: key.to_string(),
            action,
            description: desc.to_string(),
        }
    }

    /// Render to standard Hyprland `bind = ` line
    pub fn to_hyprland_bind(&self) -> String {
        let action_str = match &self.action {
            BindAction::Exec(cmd) => format!("exec, {}", cmd),
            BindAction::KillActive => "killactive,".to_string(),
            BindAction::ToggleFloating => "togglefloating,".to_string(),
            BindAction::Fullscreen => "fullscreen, 0".to_string(),
            BindAction::Workspace(num) => format!("workspace, {}", num),
            BindAction::MoveToWorkspace(num) => format!("movetoworkspace, {}", num),
            BindAction::FocusDirection(dir) => format!("movefocus, {}", dir),
        };
        format!("bind = {}, {}, {}", self.modifier.as_str(), self.key, action_str)
    }
}

/// Collection of all active keybinds
#[derive(Debug, Default)]
pub struct KeybindRegistry {
    binds: Vec<KeyBind>,
}

impl KeybindRegistry {
    pub fn omarchy_defaults() -> Self {
        let mut reg = Self { binds: Vec::new() };
        reg.binds.push(KeyBind::new(KeyModifier::Super, "Return", BindAction::Exec("kitty".to_string()), "Launch Terminal"));
        reg.binds.push(KeyBind::new(KeyModifier::Super, "D", BindAction::Exec("wofi --show drun".to_string()), "Application Menu"));
        reg.binds.push(KeyBind::new(KeyModifier::Super, "Q", BindAction::KillActive, "Close Window"));
        reg.binds.push(KeyBind::new(KeyModifier::Super, "F", BindAction::Fullscreen, "Toggle Fullscreen"));
        reg.binds.push(KeyBind::new(KeyModifier::Super, "Space", BindAction::ToggleFloating, "Toggle Floating"));

        // Directional navigation (vim keys)
        reg.binds.push(KeyBind::new(KeyModifier::Super, "h", BindAction::FocusDirection("l".to_string()), "Focus Left"));
        reg.binds.push(KeyBind::new(KeyModifier::Super, "l", BindAction::FocusDirection("r".to_string()), "Focus Right"));
        reg.binds.push(KeyBind::new(KeyModifier::Super, "k", BindAction::FocusDirection("u".to_string()), "Focus Up"));
        reg.binds.push(KeyBind::new(KeyModifier::Super, "j", BindAction::FocusDirection("d".to_string()), "Focus Down"));

        // Workspaces 1-5
        for i in 1..=5 {
            reg.binds.push(KeyBind::new(KeyModifier::Super, &i.to_string(), BindAction::Workspace(i), &format!("Switch to Workspace {}", i)));
            reg.binds.push(KeyBind::new(KeyModifier::SuperShift, &i.to_string(), BindAction::MoveToWorkspace(i), &format!("Move to Workspace {}", i)));
        }
        reg
    }

    pub fn to_conf(&self) -> String {
        let mut out = String::from("# SigmaOS Omarchy Keyboard Shortcuts\n");
        for b in &self.binds {
            out.push_str(&b.to_hyprland_bind());
            out.push('\n');
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyprland_bind_rendering_aaa() {
        let bind = KeyBind::new(KeyModifier::Super, "Return", BindAction::Exec("kitty".to_string()), "Terminal");
        assert_eq!(bind.to_hyprland_bind(), "bind = SUPER, Return, exec, kitty");

        let kill = KeyBind::new(KeyModifier::Super, "Q", BindAction::KillActive, "Close");
        assert_eq!(kill.to_hyprland_bind(), "bind = SUPER, Q, killactive,");
    }

    #[test]
    fn test_directional_focus_binds_aaa() {
        let bind = KeyBind::new(KeyModifier::Super, "h", BindAction::FocusDirection("l".to_string()), "Left");
        assert_eq!(bind.to_hyprland_bind(), "bind = SUPER, h, movefocus, l");
    }

    #[test]
    fn test_workspace_switch_binds_aaa() {
        let bind = KeyBind::new(KeyModifier::SuperShift, "3", BindAction::MoveToWorkspace(3), "Move WS3");
        assert_eq!(bind.to_hyprland_bind(), "bind = SUPER_SHIFT, 3, movetoworkspace, 3");
    }

    #[test]
    fn test_omarchy_default_registry_aaa() {
        let reg = KeybindRegistry::omarchy_defaults();
        let conf = reg.to_conf();
        assert!(conf.contains("SUPER, Return, exec, kitty"));
        assert!(conf.contains("SUPER, D, exec, wofi --show drun"));
        assert!(conf.contains("SUPER, Q, killactive,"));
        assert!(conf.contains("SUPER, 1, workspace, 1"));
    }
}
