
use std::boxed::Box;
use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Widget ID counter
static NEXT_GTK_WIDGET_ID: AtomicUsize = AtomicUsize::new(1000);

/// GTK Orientation for containers
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GtkOrientation {
    Horizontal = 0,
    Vertical = 1,
}

/// Libadwaita Color Scheme Mode
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdwColorScheme {
    Default = 0,
    ForceLight = 1,
    PreferDark = 2,
    ForceDark = 3,
}

/// GTK Signal Callback type representation
pub type GtkSignalCallback = fn(widget_id: usize, signal_name: &str) -> bool;

/// GTK Signal Dispatcher (`g_signal_connect`, `g_signal_emit`)
pub struct GtkSignalDispatcher {
    handlers: BTreeMap<String, Vec<GtkSignalCallback>>,
}

impl GtkSignalDispatcher {
    pub fn new() -> Self {
        Self {
            handlers: BTreeMap::new(),
        }
    }

    pub fn g_signal_connect(&mut self, signal_name: &str, callback: GtkSignalCallback) {
        self.handlers
            .entry(signal_name.to_string())
            .or_insert_with(Vec::new)
            .push(callback);
    }

    pub fn g_signal_emit(&self, widget_id: usize, signal_name: &str) -> bool {
        if let Some(callbacks) = self.handlers.get(signal_name) {
            let mut handled = false;
            for cb in callbacks {
                if cb(widget_id, signal_name) {
                    handled = true;
                }
            }
            handled
        } else {
            false
        }
    }
}

impl Default for GtkSignalDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// CSS Property Map for GTK CSS Provider
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GtkCssRule {
    pub selector: String,
    pub properties: BTreeMap<String, String>,
}

/// GTK CSS Style Provider (`GtkCssProvider`)
pub struct GtkCssProvider {
    rules: Vec<GtkCssRule>,
    active_color_scheme: AdwColorScheme,
}

impl GtkCssProvider {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            active_color_scheme: AdwColorScheme::Default,
        }
    }

    pub fn set_color_scheme(&mut self, scheme: AdwColorScheme) {
        self.active_color_scheme = scheme;
    }

    pub fn get_color_scheme(&self) -> AdwColorScheme {
        self.active_color_scheme
    }

    /// Parses CSS string matching GTK/Libadwaita format
    pub fn load_from_data(&mut self, css_data: &str) -> Result<usize, String> {
        let mut parsed_rules = 0;
        for block in css_data.split('}') {
            let parts: Vec<&str> = block.split('{').collect();
            if parts.len() == 2 {
                let selector = parts[0].trim().to_string();
                let body = parts[1].trim();
                let mut props = BTreeMap::new();
                for decl in body.split(';') {
                    let kv: Vec<&str> = decl.split(':').collect();
                    if kv.len() == 2 {
                        let k = kv[0].trim().to_string();
                        let v = kv[1].trim().to_string();
                        props.insert(k, v);
                    }
                }
                if !selector.is_empty() && !props.is_empty() {
                    self.rules.push(GtkCssRule {
                        selector,
                        properties: props,
                    });
                    parsed_rules += 1;
                }
            }
        }
        Ok(parsed_rules)
    }

    pub fn resolve_property(&self, selector: &str, property: &str) -> Option<String> {
        for rule in self.rules.iter().rev() {
            if rule.selector == selector || rule.selector.contains(selector) {
                if let Some(val) = rule.properties.get(property) {
                    return Some(val.clone());
                }
            }
        }
        None
    }

    pub fn resolve_property_or_default(&self, selector: &str, property: &str, default_val: &str) -> String {
        self.resolve_property(selector, property)
            .unwrap_or_else(|| default_val.to_string())
    }
}

impl Default for GtkCssProvider {
    fn default() -> Self {
        Self::new()
    }
}

/// GTK Client-Side Decoration HeaderBar (`GtkHeaderBar`)
pub struct GtkHeaderBar {
    pub id: usize,
    pub title: String,
    pub subtitle: String,
    pub show_title_buttons: bool,
    pub start_children: Vec<Box<dyn GtkWidget>>,
    pub end_children: Vec<Box<dyn GtkWidget>>,
}

impl GtkHeaderBar {
    pub fn new(title: &str, subtitle: &str) -> Self {
        Self {
            id: NEXT_GTK_WIDGET_ID.fetch_add(1, Ordering::SeqCst),
            title: title.to_string(),
            subtitle: subtitle.to_string(),
            show_title_buttons: true,
            start_children: Vec::new(),
            end_children: Vec::new(),
        }
    }

    pub fn pack_start(&mut self, widget: Box<dyn GtkWidget>) {
        self.start_children.push(widget);
    }

    pub fn pack_end(&mut self, widget: Box<dyn GtkWidget>) {
        self.end_children.push(widget);
    }
}

/// Libadwaita Action Row (`AdwActionRow`)
pub struct AdwActionRow {
    pub id: usize,
    pub title: String,
    pub subtitle: String,
    pub prefixes: Vec<Box<dyn GtkWidget>>,
    pub suffixes: Vec<Box<dyn GtkWidget>>,
    pub activatable: bool,
}

impl AdwActionRow {
    pub fn new(title: &str, subtitle: &str) -> Self {
        Self {
            id: NEXT_GTK_WIDGET_ID.fetch_add(1, Ordering::SeqCst),
            title: title.to_string(),
            subtitle: subtitle.to_string(),
            prefixes: Vec::new(),
            suffixes: Vec::new(),
            activatable: true,
        }
    }

    pub fn add_prefix(&mut self, widget: Box<dyn GtkWidget>) {
        self.prefixes.push(widget);
    }

    pub fn add_suffix(&mut self, widget: Box<dyn GtkWidget>) {
        self.suffixes.push(widget);
    }
}

/// Libadwaita Preferences Group (`AdwPreferencesGroup`)
pub struct AdwPreferencesGroup {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub rows: Vec<AdwActionRow>,
}

impl AdwPreferencesGroup {
    pub fn new(title: &str, description: &str) -> Self {
        Self {
            id: NEXT_GTK_WIDGET_ID.fetch_add(1, Ordering::SeqCst),
            title: title.to_string(),
            description: description.to_string(),
            rows: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: AdwActionRow) {
        self.rows.push(row);
    }
}

/// Libadwaita Preferences Page (`AdwPreferencesPage`)
pub struct AdwPreferencesPage {
    pub id: usize,
    pub title: String,
    pub icon_name: String,
    pub groups: Vec<AdwPreferencesGroup>,
}

impl AdwPreferencesPage {
    pub fn new(title: &str, icon_name: &str) -> Self {
        Self {
            id: NEXT_GTK_WIDGET_ID.fetch_add(1, Ordering::SeqCst),
            title: title.to_string(),
            icon_name: icon_name.to_string(),
            groups: Vec::new(),
        }
    }

    pub fn add_group(&mut self, group: AdwPreferencesGroup) {
        self.groups.push(group);
    }
}

/// GTK Container Box (`GtkBox`)
pub struct GtkBox {
    pub id: usize,
    pub orientation: GtkOrientation,
    pub spacing: u32,
    pub children: Vec<Box<dyn GtkWidget>>,
}

impl GtkBox {
    pub fn new(orientation: GtkOrientation, spacing: u32) -> Self {
        Self {
            id: NEXT_GTK_WIDGET_ID.fetch_add(1, Ordering::SeqCst),
            orientation,
            spacing,
            children: Vec::new(),
        }
    }

    pub fn append(&mut self, widget: Box<dyn GtkWidget>) {
        self.children.push(widget);
    }
}

/// GTK Generic Widget Trait
pub trait GtkWidget {
    fn widget_id(&self) -> usize;
    fn type_name(&self) -> &'static str;
    fn render_summary(&self) -> String;
}

impl GtkWidget for GtkHeaderBar {
    fn widget_id(&self) -> usize {
        self.id
    }
    fn type_name(&self) -> &'static str {
        "GtkHeaderBar"
    }
    fn render_summary(&self) -> String {
        let mut s = String::new();
        s.push_str("GtkHeaderBar[title=");
        s.push_str(&self.title);
        s.push_str(", start_children=");
        s.push_str(&self.start_children.len().to_string());
        s.push_str(", end_children=");
        s.push_str(&self.end_children.len().to_string());
        s.push(']');
        s
    }
}

impl GtkWidget for AdwActionRow {
    fn widget_id(&self) -> usize {
        self.id
    }
    fn type_name(&self) -> &'static str {
        "AdwActionRow"
    }
    fn render_summary(&self) -> String {
        let mut s = String::new();
        s.push_str("AdwActionRow[title=");
        s.push_str(&self.title);
        s.push_str(", suffixes=");
        s.push_str(&self.suffixes.len().to_string());
        s.push(']');
        s
    }
}

impl GtkWidget for GtkBox {
    fn widget_id(&self) -> usize {
        self.id
    }
    fn type_name(&self) -> &'static str {
        "GtkBox"
    }
    fn render_summary(&self) -> String {
        let mut s = String::new();
        s.push_str("GtkBox[children=");
        s.push_str(&self.children.len().to_string());
        s.push(']');
        s
    }
}

/// GTK Simple Label / Button Widget
pub struct GtkButton {
    pub id: usize,
    pub label: String,
    pub icon_name: String,
}

impl GtkButton {
    pub fn new(label: &str, icon_name: &str) -> Self {
        Self {
            id: NEXT_GTK_WIDGET_ID.fetch_add(1, Ordering::SeqCst),
            label: label.to_string(),
            icon_name: icon_name.to_string(),
        }
    }
}

impl GtkWidget for GtkButton {
    fn widget_id(&self) -> usize {
        self.id
    }
    fn type_name(&self) -> &'static str {
        "GtkButton"
    }
    fn render_summary(&self) -> String {
        let mut s = String::new();
        s.push_str("GtkButton[label=");
        s.push_str(&self.label);
        s.push(']');
        s
    }
}

/// Sovereign GTK & Libadwaita Application Window
pub struct SovereignGtkToolkit {
    pub header_bar: GtkHeaderBar,
    pub css_provider: GtkCssProvider,
    pub signal_dispatcher: GtkSignalDispatcher,
    pub pages: Vec<AdwPreferencesPage>,
}

impl SovereignGtkToolkit {
    pub fn new(app_title: &str) -> Self {
        Self {
            header_bar: GtkHeaderBar::new(app_title, "SigmaOS Native"),
            css_provider: GtkCssProvider::new(),
            signal_dispatcher: GtkSignalDispatcher::new(),
            pages: Vec::new(),
        }
    }

    pub fn add_preferences_page(&mut self, page: AdwPreferencesPage) {
        self.pages.push(page);
    }
}

impl Default for SovereignGtkToolkit {
    fn default() -> Self {
        Self::new("SigmaOS App")
    }
}

/// System Panel Tray Applet
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemTrayApplet {
    pub id: String,
    pub name: String,
    pub icon_name: String,
    pub status_text: String,
    pub active: bool,
}

/// KDE Plasma / GNOME 45 inspired top bar status panel (`SovereignSystemStatusPanel`)
pub struct SovereignSystemStatusPanel {
    pub applets: Vec<SystemTrayApplet>,
    pub clock_text: String,
    pub battery_percent: u8,
    pub wifi_signal_percent: u8,
    pub volume_percent: u8,
}

impl SovereignSystemStatusPanel {
    pub fn new() -> Self {
        let mut applets = Vec::new();
        applets.push(SystemTrayApplet {
            id: "net".to_string(),
            name: "Network".to_string(),
            icon_name: "network-wireless-symbolic".to_string(),
            status_text: "Connected (Wi-Fi)".to_string(),
            active: true,
        });
        applets.push(SystemTrayApplet {
            id: "vol".to_string(),
            name: "Volume".to_string(),
            icon_name: "audio-volume-high-symbolic".to_string(),
            status_text: "100%".to_string(),
            active: true,
        });
        applets.push(SystemTrayApplet {
            id: "power".to_string(),
            name: "Power".to_string(),
            icon_name: "battery-good-symbolic".to_string(),
            status_text: "Balanced Profile (98%)".to_string(),
            active: true,
        });

        Self {
            applets,
            clock_text: "12:00 PM".to_string(),
            battery_percent: 98,
            wifi_signal_percent: 95,
            volume_percent: 100,
        }
    }

    pub fn update_clock(&mut self, time_str: &str) {
        self.clock_text = time_str.to_string();
    }

    pub fn render_bar_summary(&self) -> String {
        let mut s = String::new();
        s.push_str("SystemPanel[Clock=");
        s.push_str(&self.clock_text);
        s.push_str(", WiFi=");
        s.push_str(&self.wifi_signal_percent.to_string());
        s.push_str("%, Battery=");
        s.push_str(&self.battery_percent.to_string());
        s.push_str("%, Applets=");
        s.push_str(&self.applets.len().to_string());
        s.push(']');
        s
    }
}

impl Default for SovereignSystemStatusPanel {
    fn default() -> Self {
        Self::new()
    }
}

/// Dock Item for Desktop Dock
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DockItem {
    pub app_id: String,
    pub title: String,
    pub icon_name: String,
    pub is_running: bool,
    pub is_pinned: bool,
    pub magnification_factor: u32, // e.g. 100 = 1.0x, 150 = 1.5x on hover
}

/// macOS / elementaryOS / Deepin inspired desktop dock (`SovereignDockBar`)
pub struct SovereignDockBar {
    pub items: Vec<DockItem>,
    pub position_bottom: bool,
    pub auto_hide: bool,
}

impl SovereignDockBar {
    pub fn new() -> Self {
        let mut items = Vec::new();
        items.push(DockItem {
            app_id: "org.sigmaos.terminal".to_string(),
            title: "Terminal".to_string(),
            icon_name: "utilities-terminal-symbolic".to_string(),
            is_running: true,
            is_pinned: true,
            magnification_factor: 100,
        });
        items.push(DockItem {
            app_id: "org.sigmaos.browser".to_string(),
            title: "Web Browser".to_string(),
            icon_name: "web-browser-symbolic".to_string(),
            is_running: false,
            is_pinned: true,
            magnification_factor: 100,
        });
        items.push(DockItem {
            app_id: "org.sigmaos.files".to_string(),
            title: "File Manager".to_string(),
            icon_name: "system-file-manager-symbolic".to_string(),
            is_running: true,
            is_pinned: true,
            magnification_factor: 100,
        });

        Self {
            items,
            position_bottom: true,
            auto_hide: false,
        }
    }

    pub fn pin_app(&mut self, app_id: &str, title: &str, icon_name: &str) {
        if !self.items.iter().any(|item| item.app_id == app_id) {
            self.items.push(DockItem {
                app_id: app_id.to_string(),
                title: title.to_string(),
                icon_name: icon_name.to_string(),
                is_running: false,
                is_pinned: true,
                magnification_factor: 100,
            });
        }
    }

    pub fn set_hover_magnification(&mut self, app_id: &str, factor: u32) {
        if let Some(item) = self.items.iter_mut().find(|item| item.app_id == app_id) {
            item.magnification_factor = factor;
        }
    }
}

impl Default for SovereignDockBar {
    fn default() -> Self {
        Self::new()
    }
}

/// Overview Workspace Switcher
pub struct SovereignOverviewWorkspaceSwitcher {
    pub active_workspace_index: usize,
    pub total_workspaces: usize,
    pub workspace_names: Vec<String>,
}

impl SovereignOverviewWorkspaceSwitcher {
    pub fn new(count: usize) -> Self {
        let mut names = Vec::new();
        for i in 1..=count {
            let mut name = String::from("Workspace ");
            name.push_str(&i.to_string());
            names.push(name);
        }
        Self {
            active_workspace_index: 0,
            total_workspaces: count,
            workspace_names: names,
        }
    }

    pub fn switch_to_workspace(&mut self, index: usize) -> bool {
        if index < self.total_workspaces {
            self.active_workspace_index = index;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gtk_css_provider_fallback_resolution() {
        let css = GtkCssProvider::new();
        // Unloaded property falls back to programmatic default value without requiring external CSS
        let bg = css.resolve_property_or_default("window.main", "background-color", "#1e1e2e");
        assert_eq!(bg, "#1e1e2e");

        let mut css_with_data = GtkCssProvider::new();
        let _ = css_with_data.load_from_data("window.main { background-color: #000000; }");
        let bg_custom = css_with_data.resolve_property_or_default("window.main", "background-color", "#1e1e2e");
        assert_eq!(bg_custom, "#000000");
    }

    #[test]
    fn test_gtk_headerbar_and_packing() {
        let mut header = GtkHeaderBar::new("Settings", "System");
        let btn_back = GtkButton::new("Back", "go-previous-symbolic");
        let btn_menu = GtkButton::new("Menu", "open-menu-symbolic");

        header.pack_start(Box::new(btn_back));
        header.pack_end(Box::new(btn_menu));

        assert_eq!(header.start_children.len(), 1);
        assert_eq!(header.end_children.len(), 1);
        assert!(header.render_summary().contains("start_children=1"));
    }

    #[test]
    fn test_adw_action_row_and_preferences() {
        let mut page = AdwPreferencesPage::new("Appearance", "preferences-desktop-theme-symbolic");
        let mut group = AdwPreferencesGroup::new("Theme Options", "Customize GTK look and feel");
        let mut row = AdwActionRow::new("Dark Mode", "Prefer dark application theme");

        let switch_btn = GtkButton::new("Toggle", "emblem-ok-symbolic");
        row.add_suffix(Box::new(switch_btn));
        group.add_row(row);
        page.add_group(group);

        assert_eq!(page.groups.len(), 1);
        assert_eq!(page.groups[0].rows.len(), 1);
        assert_eq!(page.groups[0].rows[0].suffixes.len(), 1);
    }

    #[test]
    fn test_gtk_css_provider_resolution() {
        let mut css = GtkCssProvider::new();
        let sample_css = "
            window.main {
                background-color: #2d2d2d;
                border-radius: 12px;
            }
            .adw-action-row {
                padding: 10px;
            }
        ";
        let count = css.load_from_data(sample_css).unwrap();
        assert_eq!(count, 2);

        let bg = css.resolve_property("window.main", "background-color");
        assert_eq!(bg, Some("#2d2d2d".to_string()));

        let padding = css.resolve_property(".adw-action-row", "padding");
        assert_eq!(padding, Some("10px".to_string()));
    }

    #[test]
    fn test_gtk_signal_dispatcher() {
        let mut dispatcher = GtkSignalDispatcher::new();
        dispatcher.g_signal_connect("clicked", |_id, _sig| true);

        let handled = dispatcher.g_signal_emit(1001, "clicked");
        assert!(handled);

        let unhandled = dispatcher.g_signal_emit(1001, "activate");
        assert!(!unhandled);
    }

    #[test]
    fn test_system_status_panel_and_dock() {
        let mut panel = SovereignSystemStatusPanel::new();
        panel.update_clock("10:45 AM");
        assert!(panel.render_bar_summary().contains("Clock=10:45 AM"));

        let mut dock = SovereignDockBar::new();
        dock.pin_app("org.sigmaos.calculator", "Calculator", "accessories-calculator-symbolic");
        dock.set_hover_magnification("org.sigmaos.calculator", 140);
        assert_eq!(dock.items.len(), 4);
        assert_eq!(dock.items[3].magnification_factor, 140);

        let mut switcher = SovereignOverviewWorkspaceSwitcher::new(4);
        assert!(switcher.switch_to_workspace(2));
        assert_eq!(switcher.active_workspace_index, 2);
    }
}
