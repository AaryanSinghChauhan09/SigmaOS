#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

use core::sync::atomic::{AtomicUsize, Ordering};
use std::boxed::Box;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// Modifier bitmasks inspired by Linux & BSD window managers (i3, Sway, Hyprland, xmonad, dwm)
pub const MOD_NONE: u8 = 0b0000_0000;
pub const MOD_SHIFT: u8 = 0b0000_0001;
pub const MOD_CTRL: u8 = 0b0000_0010;
pub const MOD_ALT: u8 = 0b0000_0100;
pub const MOD_SUPER: u8 = 0b0000_1000; // Meta / Win / Cmd key
pub const MOD_HYPER: u8 = 0b0001_0000;

pub type HotkeyID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotkeyError {
    Success = 0,
    NotFound = 1,
    Conflict = 2,
    InvalidProfile = 3,
}

pub trait Hotkey: Send + Sync {
    fn id(&self) -> HotkeyID;
    fn modifiers(&self) -> u8;
    fn key(&self) -> u8;
    fn action(&self) -> &[u8];
}

#[repr(C)]
pub struct SimpleHotkey {
    pub id: HotkeyID,
    pub modifiers: AtomicUsize,
    pub key: AtomicUsize,
    pub action: [u8; 64],
}

impl SimpleHotkey {
    pub fn new(id: HotkeyID, modifiers: u8, key: u8, action: &[u8]) -> Self {
        let mut action_array = [0u8; 64];
        let action_len = action.len().min(63);
        for i in 0..action_len {
            action_array[i] = action[i];
        }
        SimpleHotkey {
            id,
            modifiers: AtomicUsize::new(modifiers as usize),
            key: AtomicUsize::new(key as usize),
            action: action_array,
        }
    }
}

impl Hotkey for SimpleHotkey {
    fn id(&self) -> HotkeyID {
        self.id
    }
    fn modifiers(&self) -> u8 {
        self.modifiers.load(Ordering::SeqCst) as u8
    }
    fn key(&self) -> u8 {
        self.key.load(Ordering::SeqCst) as u8
    }
    fn action(&self) -> &[u8] {
        let len = self.action.iter().position(|&b| b == 0).unwrap_or(64);
        &self.action[..len]
    }
}

pub trait HotkeyManager {
    fn register_hotkey(
        &mut self,
        modifiers: u8,
        key: u8,
        action: &[u8],
    ) -> Result<HotkeyID, HotkeyError>;
    fn unregister_hotkey(&mut self, id: HotkeyID) -> Result<(), HotkeyError>;
    fn trigger_hotkey(&self, modifiers: u8, key: u8) -> Option<&[u8]>;
}

#[repr(C)]
pub struct SimpleHotkeyManager {
    pub hotkeys: Vec<Option<Box<dyn Hotkey>>>,
    pub next_id: AtomicUsize,
}

impl SimpleHotkeyManager {
    pub fn new() -> Self {
        SimpleHotkeyManager {
            hotkeys: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl HotkeyManager for SimpleHotkeyManager {
    fn register_hotkey(
        &mut self,
        modifiers: u8,
        key: u8,
        action: &[u8],
    ) -> Result<HotkeyID, HotkeyError> {
        for hotkey_option in &self.hotkeys {
            if let Some(ref hotkey) = *hotkey_option {
                if hotkey.modifiers() == modifiers && hotkey.key() == key {
                    return Err(HotkeyError::Conflict);
                }
            }
        }
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let hotkey = SimpleHotkey::new(id, modifiers, key, action);
        self.hotkeys.push(Some(Box::new(hotkey)));
        Ok(id)
    }

    fn unregister_hotkey(&mut self, id: HotkeyID) -> Result<(), HotkeyError> {
        for hotkey_option in &mut self.hotkeys {
            if let Some(ref hotkey) = *hotkey_option {
                if hotkey.id() == id {
                    *hotkey_option = None;
                    return Ok(());
                }
            }
        }
        Err(HotkeyError::NotFound)
    }

    fn trigger_hotkey(&self, modifiers: u8, key: u8) -> Option<&[u8]> {
        for hotkey_option in &self.hotkeys {
            if let Some(ref hotkey) = *hotkey_option {
                if hotkey.modifiers() == modifiers && hotkey.key() == key {
                    return Some(hotkey.action());
                }
            }
        }
        None
    }
}

/// Linux & BSD Window Manager Shortcut Preset Profiles
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowManagerProfileType {
    I3Sway,          // i3wm / Sway tiling window manager shortcuts
    Hyprland,        // Hyprland Wayland compositor shortcuts
    FreeBsdXmonadDwm, // FreeBSD xmonad / dwm keyboard shortcuts
    MacOsParity,     // macOS desktop hotkeys
}

#[derive(Debug, Clone)]
pub struct HotkeyBinding {
    pub name: String,
    pub modifiers: u8,
    pub key: u8,
    pub action_command: String,
}

pub struct SovereignWindowManagerHotkeyEngine {
    pub active_profile: WindowManagerProfileType,
    pub manager: SimpleHotkeyManager,
    pub loaded_bindings: Vec<HotkeyBinding>,
}

impl SovereignWindowManagerHotkeyEngine {
    pub fn new(profile: WindowManagerProfileType) -> Self {
        let mut engine = SovereignWindowManagerHotkeyEngine {
            active_profile: profile.clone(),
            manager: SimpleHotkeyManager::new(),
            loaded_bindings: Vec::new(),
        };
        engine.load_preset_profile(profile);
        engine
    }

    pub fn load_preset_profile(&mut self, profile: WindowManagerProfileType) {
        self.active_profile = profile.clone();
        self.loaded_bindings.clear();

        match profile {
            WindowManagerProfileType::I3Sway => {
                // i3 / Sway shortcuts: Super+Enter (Terminal), Super+Shift+Q (Kill), Super+D (Launcher)
                self.add_binding("terminal", MOD_SUPER, b'\n', "exec terminal");
                self.add_binding("kill_window", MOD_SUPER | MOD_SHIFT, b'q', "kill");
                self.add_binding("app_launcher", MOD_SUPER, b'd', "exec dmenu");
                self.add_binding("workspace_1", MOD_SUPER, b'1', "workspace 1");
                self.add_binding("workspace_2", MOD_SUPER, b'2', "workspace 2");
                self.add_binding("fullscreen", MOD_SUPER, b'f', "fullscreen toggle");
            }
            WindowManagerProfileType::Hyprland => {
                // Hyprland shortcuts: Super+Q (Terminal), Super+C (Kill), Super+M (Exit)
                self.add_binding("terminal", MOD_SUPER, b'q', "exec hyprctl dispatch exec kitty");
                self.add_binding("kill_window", MOD_SUPER, b'c', "killactive");
                self.add_binding("exit_hyprland", MOD_SUPER | MOD_SHIFT, b'm', "exit");
                self.add_binding("toggle_floating", MOD_SUPER, b'v', "togglefloating");
                self.add_binding("screenshot", MOD_NONE, 110, "exec hyprshot -m region"); // F10 screenshot
            }
            WindowManagerProfileType::FreeBsdXmonadDwm => {
                // xmonad / dwm: Alt+Shift+Enter (Terminal), Alt+Shift+C (Close)
                self.add_binding("terminal", MOD_ALT | MOD_SHIFT, b'\n', "spawn st");
                self.add_binding("close_window", MOD_ALT | MOD_SHIFT, b'c', "killclient");
                self.add_binding("next_layout", MOD_ALT, b' ', "nextlayout");
                self.add_binding("toggle_bar", MOD_ALT, b'b', "togglebar");
            }
            WindowManagerProfileType::MacOsParity => {
                // macOS shortcuts: Cmd+Space (Spotlight), Cmd+Q (Quit), Cmd+Tab (App Switcher)
                self.add_binding("spotlight", MOD_SUPER, b' ', "open spotlight");
                self.add_binding("quit_app", MOD_SUPER, b'q', "quit");
                self.add_binding("app_switcher", MOD_SUPER, b'\t', "next window");
            }
        }
    }

    fn add_binding(&mut self, name: &str, modifiers: u8, key: u8, command: &str) {
        let binding = HotkeyBinding {
            name: name.to_string(),
            modifiers,
            key,
            action_command: command.to_string(),
        };
        let _ = self.manager.register_hotkey(modifiers, key, command.as_bytes());
        self.loaded_bindings.push(binding);
    }

    pub fn trigger(&self, modifiers: u8, key: u8) -> Option<String> {
        self.manager
            .trigger_hotkey(modifiers, key)
            .map(|action| String::from_utf8_lossy(action).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modifier_bitmasks_and_hotkey_registration() {
        let mut mgr = SimpleHotkeyManager::new();
        let id1 = mgr
            .register_hotkey(MOD_SUPER | MOD_SHIFT, b'q', b"close_window")
            .unwrap();
        assert_eq!(id1, 1);

        let conflict = mgr.register_hotkey(MOD_SUPER | MOD_SHIFT, b'q', b"duplicate");
        assert_eq!(conflict.err(), Some(HotkeyError::Conflict));

        let triggered = mgr.trigger_hotkey(MOD_SUPER | MOD_SHIFT, b'q').unwrap();
        assert_eq!(triggered, b"close_window");
    }

    #[test]
    fn test_i3_sway_preset_profile() {
        let engine = SovereignWindowManagerHotkeyEngine::new(WindowManagerProfileType::I3Sway);
        assert_eq!(
            engine.trigger(MOD_SUPER, b'd'),
            Some("exec dmenu".to_string())
        );
        assert_eq!(
            engine.trigger(MOD_SUPER | MOD_SHIFT, b'q'),
            Some("kill".to_string())
        );
    }

    #[test]
    fn test_hyprland_preset_profile() {
        let engine = SovereignWindowManagerHotkeyEngine::new(WindowManagerProfileType::Hyprland);
        assert_eq!(
            engine.trigger(MOD_SUPER, b'c'),
            Some("killactive".to_string())
        );
    }

    #[test]
    fn test_freebsd_xmonad_dwm_profile() {
        let engine =
            SovereignWindowManagerHotkeyEngine::new(WindowManagerProfileType::FreeBsdXmonadDwm);
        assert_eq!(
            engine.trigger(MOD_ALT | MOD_SHIFT, b'c'),
            Some("killclient".to_string())
        );
    }
}
