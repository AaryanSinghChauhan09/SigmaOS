// SPDX-License-Identifier: MIT
// SigmaOS Linux & BSD Inspired GTK Toolkit Engine
// Features:
// - Libadwaita & GNOME style AdwStyleManager (Dark/Light theme & accent color maps)
// - GTK3/4 CSS Provider Engine with class selectors (.suggested-action, .destructive-action, .flat, .pill, .card)
// - Client-Side Decoration (CSD) HeaderBar (GtkHeaderBar / AdwHeaderBar) with window controls & sub-titles
// - Libadwaita Preferences Framework (AdwPreferencesWindow, AdwPreferencesGroup, AdwActionRow, AdwSwitchRow, AdwComboRow)
// - Adaptive Responsive Container (AdwNavigationSplitView) for desktop & mobile viewports
// - Libadwaita Toast Overlay & Notification Dispatcher (GtkToastOverlay)
// - Linux Mint XApps System Tray & Status Icon Manager (XAppStatusIconManager) with badge counters and menus
// - OpenBSD Pledge/Unveil & FreeBSD Jail GTK Sandbox Guard (BsdGtkSandboxGuard)
// - Master Unified GTK Toolkit Engine (SovereignGtkToolkitEngine)


use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// GTK Theme Mode (AdwStyleManager inspired)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GtkThemeMode {
    Default,
    PreferLight,
    PreferDark,
}

/// Accent Color Palette (Libadwaita / Ubuntu Yaru / elementaryOS Granite)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GtkAccentColor {
    Blue,
    Teal,
    Green,
    Yellow,
    Orange,
    Red,
    Pink,
    Purple,
    Slate,
}

impl GtkAccentColor {
    pub fn to_hex(&self) -> &'static str {
        match self {
            GtkAccentColor::Blue => "#3584e4",
            GtkAccentColor::Teal => "#129eaf",
            GtkAccentColor::Green => "#2ec27e",
            GtkAccentColor::Yellow => "#f5c211",
            GtkAccentColor::Orange => "#e66100",
            GtkAccentColor::Red => "#e01b24",
            GtkAccentColor::Pink => "#d56199",
            GtkAccentColor::Purple => "#9141ac",
            GtkAccentColor::Slate => "#77767b",
        }
    }
}

/// GTK CSS Style Provider (Parses and applies GTK CSS rules)
#[derive(Debug, Clone)]
pub struct GtkCssProvider {
    pub active_theme: String,
    pub accent_color: GtkAccentColor,
    pub theme_mode: GtkThemeMode,
    pub custom_css_rules: BTreeMap<String, String>,
}

impl GtkCssProvider {
    pub fn new(theme_name: &str) -> Self {
        let mut provider = Self {
            active_theme: theme_name.to_string(),
            accent_color: GtkAccentColor::Blue,
            theme_mode: GtkThemeMode::PreferDark,
            custom_css_rules: BTreeMap::new(),
        };
        provider.load_default_rules();
        provider
    }

    fn load_default_rules(&mut self) {
        self.custom_css_rules.insert(
            ".suggested-action".to_string(),
            format!("background-color: {}; color: #ffffff; border-radius: 8px;", self.accent_color.to_hex()),
        );
        self.custom_css_rules.insert(
            ".destructive-action".to_string(),
            "background-color: #e01b24; color: #ffffff; border-radius: 8px;".to_string(),
        );
        self.custom_css_rules.insert(
            ".flat".to_string(),
            "background-color: transparent; border: none; box-shadow: none;".to_string(),
        );
        self.custom_css_rules.insert(
            ".pill".to_string(),
            "border-radius: 9999px; padding: 6px 16px;".to_string(),
        );
        self.custom_css_rules.insert(
            ".card".to_string(),
            "background-color: rgba(255, 255, 255, 0.05); border-radius: 12px; padding: 12px;".to_string(),
        );
    }

    pub fn set_accent_color(&mut self, accent: GtkAccentColor) {
        self.accent_color = accent;
        self.load_default_rules();
    }

    pub fn add_custom_rule(&mut self, selector: &str, css_declarations: &str) {
        self.custom_css_rules.insert(selector.to_string(), css_declarations.to_string());
    }

    pub fn get_style_for_selector(&self, selector: &str) -> Option<&String> {
        self.custom_css_rules.get(selector)
    }
}

/// Window Control Buttons for HeaderBar
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowControlLayout {
    RightCloseMinMax, // Linux Mint / GNOME default ("appmenu:minimize,maximize,close")
    LeftCloseMinMax,  // macOS / elementaryOS style ("close,minimize,maximize:")
    CloseOnly,        // Clean Libadwaita ("appmenu:close")
}

/// Action Item for HeaderBar
#[derive(Debug, Clone)]
pub struct HeaderBarAction {
    pub id: String,
    pub icon_name: String,
    pub tooltip: String,
    pub is_suggested: bool,
    pub is_pack_start: bool,
}

/// GTK3/4 & Libadwaita Client-Side Decoration HeaderBar
#[derive(Debug, Clone)]
pub struct GtkHeaderBar {
    pub title: String,
    pub subtitle: Option<String>,
    pub show_window_controls: bool,
    pub control_layout: WindowControlLayout,
    pub actions: Vec<HeaderBarAction>,
    pub has_search_entry: bool,
}

impl GtkHeaderBar {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            subtitle: None,
            show_window_controls: true,
            control_layout: WindowControlLayout::RightCloseMinMax,
            actions: Vec::new(),
            has_search_entry: false,
        }
    }

    pub fn set_subtitle(&mut self, subtitle: &str) {
        self.subtitle = Some(subtitle.to_string());
    }

    pub fn add_action(&mut self, id: &str, icon_name: &str, tooltip: &str, is_suggested: bool, pack_start: bool) {
        self.actions.push(HeaderBarAction {
            id: id.to_string(),
            icon_name: icon_name.to_string(),
            tooltip: tooltip.to_string(),
            is_suggested,
            is_pack_start: pack_start,
        });
    }

    pub fn render_header_bar_summary(&self) -> String {
        let controls = match self.control_layout {
            WindowControlLayout::RightCloseMinMax => "[ -  +  X ]",
            WindowControlLayout::LeftCloseMinMax => "[ X  -  + ]",
            WindowControlLayout::CloseOnly => "[ X ]",
        };
        let subtitle_str = self.subtitle.as_deref().unwrap_or("");
        format!("HeaderBar: '{}' ({}) | Controls: {} | Actions: {}", self.title, subtitle_str, controls, self.actions.len())
    }
}

/// Libadwaita Preference Row Item Types
#[derive(Debug, Clone)]
pub enum AdwPreferenceRow {
    ActionRow {
        id: String,
        title: String,
        subtitle: String,
        icon_name: String,
    },
    SwitchRow {
        id: String,
        title: String,
        subtitle: String,
        is_active: bool,
    },
    ComboRow {
        id: String,
        title: String,
        options: Vec<String>,
        selected_index: usize,
    },
    EntryRow {
        id: String,
        title: String,
        text: String,
        placeholder: String,
    },
}

/// Libadwaita Preference Group
#[derive(Debug, Clone)]
pub struct AdwPreferencesGroup {
    pub title: String,
    pub description: Option<String>,
    pub rows: Vec<AdwPreferenceRow>,
}

impl AdwPreferencesGroup {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            description: None,
            rows: Vec::new(),
        }
    }

    pub fn add_switch(&mut self, id: &str, title: &str, subtitle: &str, is_active: bool) {
        self.rows.push(AdwPreferenceRow::SwitchRow {
            id: id.to_string(),
            title: title.to_string(),
            subtitle: subtitle.to_string(),
            is_active,
        });
    }

    pub fn add_combo(&mut self, id: &str, title: &str, options: &[&str], selected: usize) {
        self.rows.push(AdwPreferenceRow::ComboRow {
            id: id.to_string(),
            title: title.to_string(),
            options: options.iter().map(|s| s.to_string()).collect(),
            selected_index: selected,
        });
    }

    pub fn add_action(&mut self, id: &str, title: &str, subtitle: &str, icon_name: &str) {
        self.rows.push(AdwPreferenceRow::ActionRow {
            id: id.to_string(),
            title: title.to_string(),
            subtitle: subtitle.to_string(),
            icon_name: icon_name.to_string(),
        });
    }
}

/// Libadwaita Preference Page
#[derive(Debug, Clone)]
pub struct AdwPreferencesPage {
    pub title: String,
    pub icon_name: String,
    pub groups: Vec<AdwPreferencesGroup>,
}

impl AdwPreferencesPage {
    pub fn new(title: &str, icon_name: &str) -> Self {
        Self {
            title: title.to_string(),
            icon_name: icon_name.to_string(),
            groups: Vec::new(),
        }
    }

    pub fn add_group(&mut self, group: AdwPreferencesGroup) {
        self.groups.push(group);
    }
}

/// Libadwaita Preferences Window Engine
#[derive(Debug, Clone)]
pub struct AdwPreferencesEngine {
    pub title: String,
    pub pages: Vec<AdwPreferencesPage>,
}

impl AdwPreferencesEngine {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            pages: Vec::new(),
        }
    }

    pub fn add_page(&mut self, page: AdwPreferencesPage) {
        self.pages.push(page);
    }

    pub fn total_settings_count(&self) -> usize {
        self.pages.iter().flat_map(|p| p.groups.iter()).map(|g| g.rows.len()).sum()
    }

    pub fn search_rows(&self, query: &str) -> Vec<&AdwPreferenceRow> {
        let q = query.to_lowercase();
        let mut results = Vec::new();
        for page in &self.pages {
            for group in &page.groups {
                for row in &group.rows {
                    let title = match row {
                        AdwPreferenceRow::ActionRow { title, .. } => title,
                        AdwPreferenceRow::SwitchRow { title, .. } => title,
                        AdwPreferenceRow::ComboRow { title, .. } => title,
                        AdwPreferenceRow::EntryRow { title, .. } => title,
                    };
                    if title.to_lowercase().contains(&q) {
                        results.push(row);
                    }
                }
            }
        }
        results
    }
}

/// Libadwaita Banner Severity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdwBannerSeverity {
    Info,
    Warning,
    Error,
}

/// Libadwaita Banner Notification (AdwBanner inspired)
#[derive(Debug, Clone)]
pub struct AdwBanner {
    pub title: String,
    pub button_label: Option<String>,
    pub severity: AdwBannerSeverity,
    pub revealed: bool,
}

impl AdwBanner {
    pub fn new(title: &str, severity: AdwBannerSeverity) -> Self {
        Self {
            title: title.to_string(),
            button_label: None,
            severity,
            revealed: true,
        }
    }

    pub fn with_button(mut self, label: &str) -> Self {
        self.button_label = Some(label.to_string());
        self
    }

    pub fn dismiss(&mut self) {
        self.revealed = false;
    }

    pub fn reveal(&mut self) {
        self.revealed = true;
    }
}

/// Libadwaita View Switcher Tab Item (AdwViewSwitcher inspired)
#[derive(Debug, Clone)]
pub struct AdwViewSwitcherTab {
    pub id: String,
    pub title: String,
    pub icon_name: String,
    pub badge_number: u32,
}

/// Libadwaita View Switcher Tab Bar Engine
#[derive(Debug, Clone)]
pub struct AdwViewSwitcher {
    pub tabs: Vec<AdwViewSwitcherTab>,
    pub active_tab_id: String,
}

impl AdwViewSwitcher {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active_tab_id: String::new(),
        }
    }

    pub fn add_tab(&mut self, id: &str, title: &str, icon_name: &str) {
        let tab = AdwViewSwitcherTab {
            id: id.to_string(),
            title: title.to_string(),
            icon_name: icon_name.to_string(),
            badge_number: 0,
        };
        if self.tabs.is_empty() {
            self.active_tab_id = id.to_string();
        }
        self.tabs.push(tab);
    }

    pub fn set_badge(&mut self, id: &str, badge_number: u32) -> bool {
        if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == id) {
            tab.badge_number = badge_number;
            true
        } else {
            false
        }
    }

    pub fn switch_to(&mut self, id: &str) -> bool {
        if self.tabs.iter().any(|t| t.id == id) {
            self.active_tab_id = id.to_string();
            true
        } else {
            false
        }
    }
}

impl Default for AdwViewSwitcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Adaptive Responsive Navigation Split View Container (Libadwaita / Libhandy)
#[derive(Debug, Clone)]
pub struct AdwNavigationSplitView {
    pub sidebar_width: u32,
    pub collapsed_breakpoint: u32,
    pub is_collapsed: bool,
    pub show_sidebar: bool,
}

impl AdwNavigationSplitView {
    pub fn new() -> Self {
        Self {
            sidebar_width: 280,
            collapsed_breakpoint: 600,
            is_collapsed: false,
            show_sidebar: true,
        }
    }

    pub fn update_window_width(&mut self, window_width: u32) {
        self.is_collapsed = window_width < self.collapsed_breakpoint;
        if self.is_collapsed {
            self.show_sidebar = false;
        } else {
            self.show_sidebar = true;
        }
    }
}

impl Default for AdwNavigationSplitView {
    fn default() -> Self {
        Self::new()
    }
}

/// Toast Notification Item for Overlay
#[derive(Debug, Clone)]
pub struct GtkToast {
    pub id: String,
    pub title: String,
    pub button_label: Option<String>,
    pub timeout_seconds: u32,
}

/// Libadwaita Toast Overlay Manager
#[derive(Debug, Clone)]
pub struct GtkToastOverlay {
    pub active_toasts: Vec<GtkToast>,
}

impl GtkToastOverlay {
    pub fn new() -> Self {
        Self {
            active_toasts: Vec::new(),
        }
    }

    pub fn add_toast(&mut self, id: &str, title: &str, button_label: Option<&str>, timeout: u32) {
        self.active_toasts.push(GtkToast {
            id: id.to_string(),
            title: title.to_string(),
            button_label: button_label.map(|s| s.to_string()),
            timeout_seconds: timeout,
        });
    }

    pub fn dismiss_toast(&mut self, id: &str) -> bool {
        if let Some(pos) = self.active_toasts.iter().position(|t| t.id == id) {
            self.active_toasts.remove(pos);
            true
        } else {
            false
        }
    }
}

impl Default for GtkToastOverlay {
    fn default() -> Self {
        Self::new()
    }
}

/// Linux Mint XApp Inspired System Tray Status Icon
#[derive(Debug, Clone)]
pub struct XAppStatusIcon {
    pub id: String,
    pub title: String,
    pub icon_name: String,
    pub tooltip: String,
    pub badge_count: u32,
    pub visible: bool,
    pub menu_items: Vec<String>,
}

/// Linux Mint XApp Status Icon Manager
#[derive(Debug, Clone)]
pub struct XAppStatusIconManager {
    pub icons: BTreeMap<String, XAppStatusIcon>,
}

impl XAppStatusIconManager {
    pub fn new() -> Self {
        Self {
            icons: BTreeMap::new(),
        }
    }

    pub fn register_status_icon(&mut self, id: &str, title: &str, icon_name: &str, tooltip: &str) {
        let icon = XAppStatusIcon {
            id: id.to_string(),
            title: title.to_string(),
            icon_name: icon_name.to_string(),
            tooltip: tooltip.to_string(),
            badge_count: 0,
            visible: true,
            menu_items: Vec::new(),
        };
        self.icons.insert(id.to_string(), icon);
    }

    pub fn update_badge(&mut self, id: &str, count: u32) -> bool {
        if let Some(icon) = self.icons.get_mut(id) {
            icon.badge_count = count;
            true
        } else {
            false
        }
    }

    pub fn add_menu_item(&mut self, id: &str, menu_label: &str) -> bool {
        if let Some(icon) = self.icons.get_mut(id) {
            icon.menu_items.push(menu_label.to_string());
            true
        } else {
            false
        }
    }
}

impl Default for XAppStatusIconManager {
    fn default() -> Self {
        Self::new()
    }
}

/// FreeBSD Capsicum GTK Sandbox Capability Guard
#[derive(Debug, Clone)]
pub struct FreeBsdCapsicumGtkGuard {
    pub in_capability_mode: bool,
    pub allowed_fd_rights: Vec<String>,
}

impl FreeBsdCapsicumGtkGuard {
    pub fn new() -> Self {
        Self {
            in_capability_mode: false,
            allowed_fd_rights: Vec::new(),
        }
    }

    pub fn enter_capability_mode(&mut self) -> Result<(), &'static str> {
        self.allowed_fd_rights.push("CAP_READ".to_string());
        self.allowed_fd_rights.push("CAP_WRITE".to_string());
        self.allowed_fd_rights.push("CAP_SEEK".to_string());
        self.allowed_fd_rights.push("CAP_MMAP".to_string());
        self.in_capability_mode = true;
        Ok(())
    }

    pub fn is_right_allowed(&self, right: &str) -> bool {
        if !self.in_capability_mode {
            return true;
        }
        self.allowed_fd_rights.iter().any(|r| r == right)
    }
}

impl Default for FreeBsdCapsicumGtkGuard {
    fn default() -> Self {
        Self::new()
    }
}

/// OpenBSD Pledge/Unveil & FreeBSD Sandbox Guard for GTK Processes
#[derive(Debug, Clone)]
pub struct BsdGtkSandboxGuard {
    pub pledged_promises: String,
    pub unveiled_paths: Vec<(String, String)>,
    pub sandbox_active: bool,
    pub capsicum_guard: FreeBsdCapsicumGtkGuard,
}

impl BsdGtkSandboxGuard {
    pub fn new() -> Self {
        Self {
            pledged_promises: "stdio rpath wpath cpath unix inet prot_exec tty gpath".to_string(),
            unveiled_paths: Vec::new(),
            sandbox_active: false,
            capsicum_guard: FreeBsdCapsicumGtkGuard::new(),
        }
    }

    pub fn unveil_path(&mut self, path: &str, permissions: &str) {
        self.unveiled_paths.push((path.to_string(), permissions.to_string()));
    }

    pub fn apply_pledge_sandbox(&mut self) -> Result<(), &'static str> {
        // Enforce standard GTK asset directory unveiled access
        if !self.unveiled_paths.iter().any(|(p, _)| p == "/usr/share/themes") {
            self.unveil_path("/usr/share/themes", "r");
            self.unveil_path("/usr/share/icons", "r");
            self.unveil_path("/usr/share/fonts", "r");
        }
        self.capsicum_guard.enter_capability_mode()?;
        self.sandbox_active = true;
        Ok(())
    }

    pub fn is_path_allowed(&self, path: &str) -> bool {
        if !self.sandbox_active {
            return true;
        }
        self.unveiled_paths.iter().any(|(p, _)| path.starts_with(p))
    }
}

impl Default for BsdGtkSandboxGuard {
    fn default() -> Self {
        Self::new()
    }
}

/// Master Unified GTK Toolkit Engine for SigmaOS
#[derive(Debug, Clone)]
pub struct SovereignGtkToolkitEngine {
    pub css_provider: GtkCssProvider,
    pub header_bar: GtkHeaderBar,
    pub preference_engine: AdwPreferencesEngine,
    pub navigation_split_view: AdwNavigationSplitView,
    pub view_switcher: AdwViewSwitcher,
    pub banner: Option<AdwBanner>,
    pub toast_overlay: GtkToastOverlay,
    pub status_icon_manager: XAppStatusIconManager,
    pub bsd_sandbox_guard: BsdGtkSandboxGuard,
}

impl SovereignGtkToolkitEngine {
    pub fn new(app_name: &str) -> Self {
        let mut engine = Self {
            css_provider: GtkCssProvider::new("Adwaita-Dark"),
            header_bar: GtkHeaderBar::new(app_name),
            preference_engine: AdwPreferencesEngine::new(&format!("{} Preferences", app_name)),
            navigation_split_view: AdwNavigationSplitView::new(),
            view_switcher: AdwViewSwitcher::new(),
            banner: None,
            toast_overlay: GtkToastOverlay::new(),
            status_icon_manager: XAppStatusIconManager::new(),
            bsd_sandbox_guard: BsdGtkSandboxGuard::new(),
        };

        // Standard default GTK application setup
        engine.header_bar.add_action("btn_search", "system-search-symbolic", "Search", false, false);
        engine.header_bar.add_action("btn_menu", "open-menu-symbolic", "Main Menu", false, false);

        engine.status_icon_manager.register_status_icon(
            "app_status_tray",
            app_name,
            "system-run-symbolic",
            &format!("{} active background service", app_name),
        );

        engine
    }

    pub fn set_banner(&mut self, banner: AdwBanner) {
        self.banner = Some(banner);
    }

    pub fn initialize_security_sandbox(&mut self) -> Result<(), &'static str> {
        self.bsd_sandbox_guard.apply_pledge_sandbox()
    }

    pub fn set_theme(&mut self, theme_name: &str, accent: GtkAccentColor, mode: GtkThemeMode) {
        self.css_provider.active_theme = theme_name.to_string();
        self.css_provider.set_accent_color(accent);
        self.css_provider.theme_mode = mode;
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_gtk_css_provider() {
        let mut provider = GtkCssProvider::new("Yaru-Dark");
        assert_eq!(provider.active_theme, "Yaru-Dark");
        assert!(provider.get_style_for_selector(".suggested-action").is_some());

        provider.set_accent_color(GtkAccentColor::Orange);
        assert_eq!(provider.accent_color, GtkAccentColor::Orange);
        assert!(provider.get_style_for_selector(".suggested-action").unwrap().contains("#e66100"));

        provider.add_custom_rule(".my-button", "padding: 10px;");
        assert_eq!(provider.get_style_for_selector(".my-button").unwrap(), "padding: 10px;");
    }

    #[test]
    fn test_gtk_header_bar() {
        let mut header = GtkHeaderBar::new("Text Editor");
        header.set_subtitle("Document1.txt");
        header.add_action("save", "document-save-symbolic", "Save File", true, false);

        let summary = header.render_header_bar_summary();
        assert!(summary.contains("Text Editor"));
        assert!(summary.contains("Document1.txt"));
        assert!(summary.contains("Actions: 1"));
    }

    #[test]
    fn test_adw_preferences_engine() {
        let mut engine = AdwPreferencesEngine::new("Settings");
        let mut page = AdwPreferencesPage::new("General", "emblem-system-symbolic");
        let mut group = AdwPreferencesGroup::new("Appearance");

        group.add_switch("dark_mode", "Dark Mode", "Use dark color scheme", true);
        group.add_combo("accent", "Accent Color", &["Blue", "Teal", "Green"], 0);
        page.add_group(group);
        engine.add_page(page);

        assert_eq!(engine.total_settings_count(), 2);
    }

    #[test]
    fn test_adw_navigation_split_view() {
        let mut split = AdwNavigationSplitView::new();
        assert!(!split.is_collapsed);
        assert!(split.show_sidebar);

        // Resize below 600px breakpoint
        split.update_window_width(480);
        assert!(split.is_collapsed);
        assert!(!split.show_sidebar);

        // Resize above breakpoint
        split.update_window_width(1024);
        assert!(!split.is_collapsed);
        assert!(split.show_sidebar);
    }

    #[test]
    fn test_gtk_toast_overlay() {
        let mut overlay = GtkToastOverlay::new();
        overlay.add_toast("toast_save", "File saved", Some("Undo"), 3);
        assert_eq!(overlay.active_toasts.len(), 1);

        assert!(overlay.dismiss_toast("toast_save"));
        assert_eq!(overlay.active_toasts.len(), 0);
    }

    #[test]
    fn test_xapp_status_icon_manager() {
        let mut manager = XAppStatusIconManager::new();
        manager.register_status_icon("mail_app", "Email Client", "mail-unread-symbolic", "3 new emails");

        assert!(manager.update_badge("mail_app", 3));
        assert!(manager.add_menu_item("mail_app", "Check Mail"));

        let icon = manager.icons.get("mail_app").unwrap();
        assert_eq!(icon.badge_count, 3);
        assert_eq!(icon.menu_items.len(), 1);
    }

    #[test]
    fn test_bsd_gtk_sandbox_guard() {
        let mut guard = BsdGtkSandboxGuard::new();
        assert!(guard.is_path_allowed("/usr/share/themes/Adwaita"));

        assert!(guard.apply_pledge_sandbox().is_ok());
        assert!(guard.sandbox_active);

        assert!(guard.is_path_allowed("/usr/share/themes/Adwaita"));
        assert!(guard.is_path_allowed("/usr/share/icons/hicolor"));
        assert!(!guard.is_path_allowed("/etc/shadow"));
    }

    #[test]
    fn test_adw_banner_and_view_switcher() {
        let mut banner = AdwBanner::new("Updates available", AdwBannerSeverity::Info).with_button("Restart");
        assert!(banner.revealed);
        assert_eq!(banner.severity, AdwBannerSeverity::Info);
        banner.dismiss();
        assert!(!banner.revealed);

        let mut switcher = AdwViewSwitcher::new();
        switcher.add_tab("home", "Home", "user-home-symbolic");
        switcher.add_tab("explore", "Explore", "compass-symbolic");

        assert_eq!(switcher.active_tab_id, "home");
        assert!(switcher.set_badge("explore", 5));
        assert!(switcher.switch_to("explore"));
        assert_eq!(switcher.active_tab_id, "explore");
    }

    #[test]
    fn test_freebsd_capsicum_gtk_guard() {
        let mut guard = FreeBsdCapsicumGtkGuard::new();
        assert!(guard.is_right_allowed("CAP_READ"));

        assert!(guard.enter_capability_mode().is_ok());
        assert!(guard.in_capability_mode);
        assert!(guard.is_right_allowed("CAP_READ"));
        assert!(!guard.is_right_allowed("CAP_SYS_ADMIN"));
    }

    #[test]
    fn test_sovereign_gtk_toolkit_engine() {
        let mut engine = SovereignGtkToolkitEngine::new("Sigma Terminal");
        engine.set_theme("Adwaita", GtkAccentColor::Purple, GtkThemeMode::PreferDark);

        engine.set_banner(AdwBanner::new("Low battery", AdwBannerSeverity::Warning));
        assert!(engine.banner.is_some());

        assert_eq!(engine.css_provider.accent_color, GtkAccentColor::Purple);
        assert!(engine.initialize_security_sandbox().is_ok());
        assert!(engine.bsd_sandbox_guard.sandbox_active);
        assert!(engine.bsd_sandbox_guard.capsicum_guard.in_capability_mode);
    }
}
