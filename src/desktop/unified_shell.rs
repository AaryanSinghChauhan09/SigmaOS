// SPDX-License-Identifier: MIT
// SigmaOS - Unified Shell Engine
// Inspired by Omarchy Linux Quickshell architecture
// Provides unified shell, plugin system, keyboard-first workflow, and coordinated theming

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Theme variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeVariant {
    Light,
    Dark,
    Auto,
}

/// Theme definition
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub variant: ThemeVariant,
    pub accent_color: (u8, u8, u8),
    pub background: (u8, u8, u8),
    pub foreground: (u8, u8, u8),
}

/// Plugin type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginType {
    Widget,
    Bar,
    Notification,
    Launcher,
    OSD,
    Panel,
    Policy,
}

/// Plugin state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginState {
    Enabled,
    Disabled,
    Error,
}

/// Plugin
#[derive(Debug, Clone)]
pub struct Plugin {
    pub id: u32,
    pub name: String,
    pub plugin_type: PluginType,
    pub version: String,
    pub state: PluginState,
    pub config: String,
}

/// Key binding
#[derive(Debug, Clone)]
pub struct KeyBinding {
    pub id: u32,
    pub modifiers: Vec<String>, // e.g., ["Ctrl", "Shift"]
    pub key: String,
    pub action: String,
    pub context: String,
}

/// Keyboard layout
#[derive(Debug, Clone)]
pub struct KeyboardLayout {
    pub name: String,
    pub bindings: Vec<KeyBinding>,
}

/// Notification urgency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationUrgency {
    Low,
    Normal,
    Critical,
}

/// Notification
#[derive(Debug, Clone)]
pub struct UnifiedNotification {
    pub id: u32,
    pub app_name: String,
    pub title: String,
    pub body: String,
    pub urgency: NotificationUrgency,
    pub timeout: u32, // milliseconds
    pub actions: Vec<String>,
}

/// Widget position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidgetPosition {
    Top,
    Bottom,
    Left,
    Right,
    Center,
}

/// Widget
#[derive(Debug, Clone)]
pub struct Widget {
    pub id: u32,
    pub name: String,
    pub position: WidgetPosition,
    pub visible: bool,
    pub config: String,
}

/// Panel configuration
#[derive(Debug, Clone)]
pub struct Panel {
    pub id: u32,
    pub position: WidgetPosition,
    pub height: u32,
    pub widgets: Vec<u32>, // widget IDs
    pub autohide: bool,
}

/// OSD (On-Screen Display) element
#[derive(Debug, Clone)]
pub struct OSDElement {
    pub id: u32,
    pub osd_type: String,
    pub position: (f32, f32),
    pub duration: u32,
    pub content: String,
}

/// Policy agent rule
#[derive(Debug, Clone)]
pub struct PolicyRule {
    pub id: u32,
    pub rule_type: String,
    pub condition: String,
    pub action: String,
    pub enabled: bool,
}

/// Unified shell engine
pub struct UnifiedShellEngine {
    pub active: AtomicBool,
    pub current_theme: Theme,
    pub themes: BTreeMap<String, Theme>,
    pub plugins: BTreeMap<u32, Plugin>,
    pub keyboard_layout: KeyboardLayout,
    pub notifications: Vec<UnifiedNotification>,
    pub widgets: BTreeMap<u32, Widget>,
    pub panels: BTreeMap<u32, Panel>,
    pub osd_elements: Vec<OSDElement>,
    pub policy_rules: BTreeMap<u32, PolicyRule>,
    pub next_plugin_id: AtomicU32,
    pub next_widget_id: AtomicU32,
    pub next_panel_id: AtomicU32,
    pub next_notification_id: AtomicU32,
    pub next_rule_id: AtomicU32,
}

impl UnifiedShellEngine {
    pub fn new() -> Self {
        Self {
            active: AtomicBool::new(false),
            current_theme: Theme {
                name: "Default".to_string(),
                variant: ThemeVariant::Auto,
                accent_color: (35, 132, 228),
                background: (255, 255, 255),
                foreground: (0, 0, 0),
            },
            themes: BTreeMap::new(),
            plugins: BTreeMap::new(),
            keyboard_layout: KeyboardLayout {
                name: "US".to_string(),
                bindings: Vec::new(),
            },
            notifications: Vec::new(),
            widgets: BTreeMap::new(),
            panels: BTreeMap::new(),
            osd_elements: Vec::new(),
            policy_rules: BTreeMap::new(),
            next_plugin_id: AtomicU32::new(1),
            next_widget_id: AtomicU32::new(1),
            next_panel_id: AtomicU32::new(1),
            next_notification_id: AtomicU32::new(1),
            next_rule_id: AtomicU32::new(1),
        }
    }

    /// Initialize unified shell
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
    }

    /// Add theme
    pub fn add_theme(&mut self, theme: Theme) {
        self.themes.insert(theme.name.clone(), theme);
    }

    /// Set theme
    pub fn set_theme(&mut self, name: String) -> Result<(), &'static str> {
        let theme = self.themes.get(&name).ok_or("Theme not found")?;
        self.current_theme = theme.clone();
        Ok(())
    }

    /// Register plugin
    pub fn register_plugin(
        &mut self,
        name: String,
        plugin_type: PluginType,
        version: String,
        config: String,
    ) -> u32 {
        let id = self.next_plugin_id.fetch_add(1, Ordering::Relaxed);
        let plugin = Plugin {
            id,
            name,
            plugin_type,
            version,
            state: PluginState::Enabled,
            config,
        };
        self.plugins.insert(id, plugin);
        id
    }

    /// Enable plugin
    pub fn enable_plugin(&mut self, plugin_id: u32) -> Result<(), &'static str> {
        let plugin = self.plugins.get_mut(&plugin_id).ok_or("Plugin not found")?;
        plugin.state = PluginState::Enabled;
        Ok(())
    }

    /// Disable plugin
    pub fn disable_plugin(&mut self, plugin_id: u32) -> Result<(), &'static str> {
        let plugin = self.plugins.get_mut(&plugin_id).ok_or("Plugin not found")?;
        plugin.state = PluginState::Disabled;
        Ok(())
    }

    /// Add key binding
    pub fn add_key_binding(
        &mut self,
        modifiers: Vec<String>,
        key: String,
        action: String,
        context: String,
    ) -> u32 {
        let id = self.keyboard_layout.bindings.len() as u32;
        let binding = KeyBinding {
            id,
            modifiers,
            key,
            action,
            context,
        };
        self.keyboard_layout.bindings.push(binding);
        id
    }

    /// Send notification
    pub fn send_notification(
        &mut self,
        app_name: String,
        title: String,
        body: String,
        urgency: NotificationUrgency,
        timeout: u32,
    ) -> u32 {
        let id = self.next_notification_id.fetch_add(1, Ordering::Relaxed);
        let notification = UnifiedNotification {
            id,
            app_name,
            title,
            body,
            urgency,
            timeout,
            actions: Vec::new(),
        };
        self.notifications.push(notification);
        id
    }

    /// Add widget
    pub fn add_widget(&mut self, name: String, position: WidgetPosition, config: String) -> u32 {
        let id = self.next_widget_id.fetch_add(1, Ordering::Relaxed);
        let widget = Widget {
            id,
            name,
            position,
            visible: true,
            config,
        };
        self.widgets.insert(id, widget);
        id
    }

    /// Create panel
    pub fn create_panel(&mut self, position: WidgetPosition, height: u32, autohide: bool) -> u32 {
        let id = self.next_panel_id.fetch_add(1, Ordering::Relaxed);
        let panel = Panel {
            id,
            position,
            height,
            widgets: Vec::new(),
            autohide,
        };
        self.panels.insert(id, panel);
        id
    }

    /// Add widget to panel
    pub fn add_widget_to_panel(
        &mut self,
        panel_id: u32,
        widget_id: u32,
    ) -> Result<(), &'static str> {
        let panel = self.panels.get_mut(&panel_id).ok_or("Panel not found")?;
        if !panel.widgets.contains(&widget_id) {
            panel.widgets.push(widget_id);
        }
        Ok(())
    }

    /// Show OSD
    pub fn show_osd(
        &mut self,
        osd_type: String,
        position: (f32, f32),
        duration: u32,
        content: String,
    ) -> u32 {
        let id = self.osd_elements.len() as u32;
        let osd = OSDElement {
            id,
            osd_type,
            position,
            duration,
            content,
        };
        self.osd_elements.push(osd);
        id
    }

    /// Add policy rule
    pub fn add_policy_rule(&mut self, rule_type: String, condition: String, action: String) -> u32 {
        let id = self.next_rule_id.fetch_add(1, Ordering::Relaxed);
        let rule = PolicyRule {
            id,
            rule_type,
            condition,
            action,
            enabled: true,
        };
        self.policy_rules.insert(id, rule);
        id
    }

    /// Enable policy rule
    pub fn enable_policy_rule(&mut self, rule_id: u32) -> Result<(), &'static str> {
        let rule = self
            .policy_rules
            .get_mut(&rule_id)
            .ok_or("Rule not found")?;
        rule.enabled = true;
        Ok(())
    }

    /// Disable policy rule
    pub fn disable_policy_rule(&mut self, rule_id: u32) -> Result<(), &'static str> {
        let rule = self
            .policy_rules
            .get_mut(&rule_id)
            .ok_or("Rule not found")?;
        rule.enabled = false;
        Ok(())
    }

    /// Toggle widget visibility
    pub fn toggle_widget(&mut self, widget_id: u32) -> Result<(), &'static str> {
        let widget = self.widgets.get_mut(&widget_id).ok_or("Widget not found")?;
        widget.visible = !widget.visible;
        Ok(())
    }

    /// Is active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
}

impl Default for UnifiedShellEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Global unified shell
static mut UNIFIED_SHELL: Option<UnifiedShellEngine> = None;

/// Initialize unified shell
pub fn init_unified_shell() {
    unsafe {
        UNIFIED_SHELL = Some(UnifiedShellEngine::new());
    }
}

/// Get global unified shell
pub fn get_unified_shell() -> Option<&'static mut UnifiedShellEngine> {
    unsafe { UNIFIED_SHELL.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_shell_init() {
        let mut shell = UnifiedShellEngine::new();
        shell.init();
        assert!(shell.is_active());
    }

    #[test]
    fn test_themes() {
        let mut shell = UnifiedShellEngine::new();
        shell.init();
        let theme = Theme {
            name: "Dark".to_string(),
            variant: ThemeVariant::Dark,
            accent_color: (35, 132, 228),
            background: (30, 30, 30),
            foreground: (255, 255, 255),
        };
        shell.add_theme(theme);
        shell.set_theme("Dark".to_string()).unwrap();
        assert_eq!(shell.current_theme.name, "Dark");
    }

    #[test]
    fn test_plugins() {
        let mut shell = UnifiedShellEngine::new();
        shell.init();
        let plugin_id = shell.register_plugin(
            "Launcher".to_string(),
            PluginType::Launcher,
            "1.0".to_string(),
            "{}".to_string(),
        );
        assert!(shell.plugins.contains_key(&plugin_id));
        shell.disable_plugin(plugin_id).unwrap();
        assert_eq!(shell.plugins[&plugin_id].state, PluginState::Disabled);
    }

    #[test]
    fn test_key_bindings() {
        let mut shell = UnifiedShellEngine::new();
        shell.init();
        let binding_id = shell.add_key_binding(
            vec!["Ctrl".to_string()],
            "Space".to_string(),
            "launcher".to_string(),
            "global".to_string(),
        );
        assert_eq!(
            shell.keyboard_layout.bindings[binding_id as usize].action,
            "launcher"
        );
    }

    #[test]
    fn test_notifications() {
        let mut shell = UnifiedShellEngine::new();
        shell.init();
        let id = shell.send_notification(
            "TestApp".to_string(),
            "Test Title".to_string(),
            "Test Body".to_string(),
            NotificationUrgency::Normal,
            5000,
        );
        assert_eq!(shell.notifications.len(), 1);
        assert_eq!(shell.notifications[0].title, "Test Title");
    }

    #[test]
    fn test_widgets() {
        let mut shell = UnifiedShellEngine::new();
        shell.init();
        let widget_id =
            shell.add_widget("Clock".to_string(), WidgetPosition::Top, "{}".to_string());
        assert!(shell.widgets.contains_key(&widget_id));
        shell.toggle_widget(widget_id).unwrap();
        assert!(!shell.widgets[&widget_id].visible);
    }

    #[test]
    fn test_panels() {
        let mut shell = UnifiedShellEngine::new();
        shell.init();
        let panel_id = shell.create_panel(WidgetPosition::Bottom, 30, false);
        assert!(shell.panels.contains_key(&panel_id));
        let widget_id = shell.add_widget(
            "Status".to_string(),
            WidgetPosition::Bottom,
            "{}".to_string(),
        );
        shell.add_widget_to_panel(panel_id, widget_id).unwrap();
        assert_eq!(shell.panels[&panel_id].widgets.len(), 1);
    }

    #[test]
    fn test_osd() {
        let mut shell = UnifiedShellEngine::new();
        shell.init();
        let osd_id = shell.show_osd(
            "volume".to_string(),
            (0.5, 0.5),
            2000,
            "Volume: 50%".to_string(),
        );
        assert_eq!(shell.osd_elements.len(), 1);
        assert_eq!(shell.osd_elements[0].osd_type, "volume");
    }

    #[test]
    fn test_policy_rules() {
        let mut shell = UnifiedShellEngine::new();
        shell.init();
        let rule_id = shell.add_policy_rule(
            "auto_lock".to_string(),
            "idle > 600".to_string(),
            "lock_screen".to_string(),
        );
        assert!(shell.policy_rules.contains_key(&rule_id));
        shell.disable_policy_rule(rule_id).unwrap();
        assert!(!shell.policy_rules[&rule_id].enabled);
    }
}
