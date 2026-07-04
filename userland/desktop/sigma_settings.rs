// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// userland/desktop/sigma_settings.rs — Unified Settings Hub
// Language: Rust (std) — OOP via SettingsHub + SettingsPanel trait

use std::collections::BTreeMap;

// ── Setting Value ─────────────────────────────────────────────────────────────
#[derive(Clone, Debug, PartialEq)]
pub enum SettingValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Enum { value: String, options: Vec<String> },
    Color(u32),  // RGBA packed
}

impl SettingValue {
    pub fn as_bool(&self) -> Option<bool> { if let Self::Bool(v) = self { Some(*v) } else { None } }
    pub fn as_str(&self)  -> Option<&str> { if let Self::String(v) = self { Some(v) } else { None } }
    pub fn as_int(&self)  -> Option<i64>  { if let Self::Int(v) = self { Some(*v) } else { None } }
}

// ── Setting ───────────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct Setting {
    pub key:         String,
    pub label:       String,
    pub description: String,
    pub value:       SettingValue,
    pub default:     SettingValue,
    pub persistent:  bool,
}

impl Setting {
    pub fn bool(key: &str, label: &str, default: bool) -> Self {
        Self { key: key.to_owned(), label: label.to_owned(), description: String::new(),
               value: SettingValue::Bool(default), default: SettingValue::Bool(default), persistent: true }
    }
    pub fn int(key: &str, label: &str, default: i64) -> Self {
        Self { key: key.to_owned(), label: label.to_owned(), description: String::new(),
               value: SettingValue::Int(default), default: SettingValue::Int(default), persistent: true }
    }
    pub fn string(key: &str, label: &str, default: &str) -> Self {
        Self { key: key.to_owned(), label: label.to_owned(), description: String::new(),
               value: SettingValue::String(default.to_owned()),
               default: SettingValue::String(default.to_owned()), persistent: true }
    }
    pub fn enum_setting(key: &str, label: &str, default: &str, opts: &[&str]) -> Self {
        let options: Vec<String> = opts.iter().map(|s| s.to_string()).collect();
        let val = SettingValue::Enum { value: default.to_owned(), options: options.clone() };
        Self { key: key.to_owned(), label: label.to_owned(), description: String::new(),
               value: val.clone(), default: val, persistent: true }
    }
    pub fn with_desc(mut self, d: &str) -> Self { self.description = d.to_owned(); self }
    pub fn reset(&mut self) { self.value = self.default.clone(); }
}

// ── Settings Panel Trait ──────────────────────────────────────────────────────
pub trait SettingsPanel: Send {
    fn id(&self)     -> &'static str;
    fn title(&self)  -> &'static str;
    fn icon(&self)   -> &'static str;
    fn settings(&self) -> &[Setting];
    fn settings_mut(&mut self) -> &mut Vec<Setting>;
    fn apply(&mut self, key: &str, value: SettingValue);
    fn load(&mut self) {}
    fn save(&self) {}
}

// ── Appearance Panel ──────────────────────────────────────────────────────────
pub struct AppearancePanel { settings: Vec<Setting> }
impl AppearancePanel {
    pub fn new() -> Self {
        Self { settings: vec![
            Setting::enum_setting("theme", "Theme", "zenith-dark", &["zenith-dark","zenith-light","high-contrast"]),
            Setting::int("corner_radius", "Corner Radius", 12).with_desc("Window corner radius (0-20px)"),
            Setting::int("gap", "Window Gap", 8).with_desc("Gap between tiled windows (px)"),
            Setting::float_setting("scale", "UI Scale", 1.0),
            Setting::bool("animations", "Enable Animations", true),
            Setting::bool("blur", "Blur Effect", true),
            Setting::bool("auto_theme", "Auto Dark/Light by Time", false),
        ] }
    }
}
impl SettingsPanel for AppearancePanel {
    fn id(&self)    -> &'static str { "appearance" }
    fn title(&self) -> &'static str { "Appearance" }
    fn icon(&self)  -> &'static str { "🎨" }
    fn settings(&self) -> &[Setting] { &self.settings }
    fn settings_mut(&mut self) -> &mut Vec<Setting> { &mut self.settings }
    fn apply(&mut self, key: &str, value: SettingValue) {
        for s in &mut self.settings { if s.key == key { s.value = value.clone(); return; } }
    }
}

// ── Network Panel ─────────────────────────────────────────────────────────────
pub struct NetworkPanel { settings: Vec<Setting> }
impl NetworkPanel {
    pub fn new() -> Self {
        Self { settings: vec![
            Setting::bool("auto_connect", "Auto-connect on boot", true),
            Setting::bool("ipv6", "Enable IPv6", true),
            Setting::bool("doh", "DNS over HTTPS", true),
            Setting::string("dns_primary", "Primary DNS", "1.1.1.1"),
            Setting::string("dns_secondary", "Secondary DNS", "9.9.9.9"),
            Setting::bool("firewall", "Enable Firewall", true),
            Setting::bool("vpn_autostart", "Auto-start VPN on boot", false),
        ] }
    }
}
impl SettingsPanel for NetworkPanel {
    fn id(&self)    -> &'static str { "network" }
    fn title(&self) -> &'static str { "Network" }
    fn icon(&self)  -> &'static str { "🌐" }
    fn settings(&self) -> &[Setting] { &self.settings }
    fn settings_mut(&mut self) -> &mut Vec<Setting> { &mut self.settings }
    fn apply(&mut self, key: &str, value: SettingValue) {
        for s in &mut self.settings { if s.key == key { s.value = value.clone(); return; } }
    }
}

// ── Privacy Panel ─────────────────────────────────────────────────────────────
pub struct PrivacyPanel { settings: Vec<Setting> }
impl PrivacyPanel {
    pub fn new() -> Self {
        Self { settings: vec![
            Setting::bool("telemetry", "Send Telemetry", false).with_desc("Always off by default — your data stays local"),
            Setting::bool("crash_reports", "Auto-send Crash Reports", false),
            Setting::bool("analytics", "Analytics", false),
            Setting::bool("clear_tmp_on_shutdown", "Clear /tmp on Shutdown", true),
            Setting::bool("clipboard_guard", "Block Background Clipboard Access", true),
            Setting::bool("mic_indicator", "Show Microphone Access Indicator", true),
            Setting::bool("cam_indicator", "Show Camera Access Indicator", true),
        ] }
    }
}
impl SettingsPanel for PrivacyPanel {
    fn id(&self)    -> &'static str { "privacy" }
    fn title(&self) -> &'static str { "Privacy" }
    fn icon(&self)  -> &'static str { "🔒" }
    fn settings(&self) -> &[Setting] { &self.settings }
    fn settings_mut(&mut self) -> &mut Vec<Setting> { &mut self.settings }
    fn apply(&mut self, key: &str, value: SettingValue) {
        for s in &mut self.settings { if s.key == key { s.value = value.clone(); return; } }
    }
}

impl Setting {
    pub fn float_setting(key: &str, label: &str, default: f64) -> Self {
        Self { key: key.to_owned(), label: label.to_owned(), description: String::new(),
               value: SettingValue::Float(default), default: SettingValue::Float(default), persistent: true }
    }
}

// ── Settings Hub ──────────────────────────────────────────────────────────────
pub struct SettingsHub {
    panels:       BTreeMap<String, Box<dyn SettingsPanel>>,
    active_panel: String,
    dirty:        bool,
}

impl SettingsHub {
    pub fn new() -> Self {
        let mut hub = Self {
            panels: BTreeMap::new(),
            active_panel: "appearance".to_owned(),
            dirty: false,
        };
        hub.panels.insert("appearance".to_owned(), Box::new(AppearancePanel::new()));
        hub.panels.insert("network".to_owned(),    Box::new(NetworkPanel::new()));
        hub.panels.insert("privacy".to_owned(),    Box::new(PrivacyPanel::new()));
        hub
    }

    pub fn set_active(&mut self, id: &str) { self.active_panel = id.to_owned(); }
    pub fn active(&self) -> Option<&dyn SettingsPanel> { self.panels.get(&self.active_panel).map(|p| p.as_ref()) }

    pub fn apply(&mut self, panel_id: &str, key: &str, value: SettingValue) {
        if let Some(p) = self.panels.get_mut(panel_id) { p.apply(key, value); self.dirty = true; }
    }

    pub fn get(&self, panel_id: &str, key: &str) -> Option<&SettingValue> {
        self.panels.get(panel_id)?.settings().iter().find(|s| s.key == key).map(|s| &s.value)
    }

    pub fn save_all(&mut self) {
        for p in self.panels.values() { p.save(); }
        self.dirty = false;
    }

    pub fn panel_list(&self) -> Vec<(&str, &str, &str)> {
        self.panels.values().map(|p| (p.id(), p.title(), p.icon())).collect()
    }

    pub fn is_dirty(&self) -> bool { self.dirty }
}
