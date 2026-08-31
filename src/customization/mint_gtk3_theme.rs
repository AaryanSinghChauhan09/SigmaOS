extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;
use crate::klib::BTreeMap;

/// Linux Mint GTK3 Mint-Y and Mint-X accent color variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MintYColorVariant {
    Aqua,
    Blue,
    Teal,
    Pink,
    Purple,
    Orange,
    Sand,
    Red,
    Yaru,
    Emerald,
}

impl MintYColorVariant {
    pub fn hex_code(&self) -> &'static str {
        match self {
            MintYColorVariant::Aqua => "#00b0ff",
            MintYColorVariant::Blue => "#3584e4",
            MintYColorVariant::Teal => "#00897b",
            MintYColorVariant::Pink => "#ec407a",
            MintYColorVariant::Purple => "#8e24aa",
            MintYColorVariant::Orange => "#f57c00",
            MintYColorVariant::Sand => "#8d6e63",
            MintYColorVariant::Red => "#e53935",
            MintYColorVariant::Yaru => "#e95420",
            MintYColorVariant::Emerald => "#92b989",
        }
    }

    pub fn rgb_tuple(&self) -> (u8, u8, u8) {
        match self {
            MintYColorVariant::Aqua => (0, 176, 255),
            MintYColorVariant::Blue => (53, 132, 228),
            MintYColorVariant::Teal => (0, 137, 123),
            MintYColorVariant::Pink => (236, 64, 122),
            MintYColorVariant::Purple => (142, 36, 170),
            MintYColorVariant::Orange => (245, 124, 0),
            MintYColorVariant::Sand => (141, 110, 99),
            MintYColorVariant::Red => (229, 57, 53),
            MintYColorVariant::Yaru => (233, 84, 32),
            MintYColorVariant::Emerald => (146, 185, 137),
        }
    }

    pub fn variant_name(&self) -> &'static str {
        match self {
            MintYColorVariant::Aqua => "Aqua",
            MintYColorVariant::Blue => "Blue",
            MintYColorVariant::Teal => "Teal",
            MintYColorVariant::Pink => "Pink",
            MintYColorVariant::Purple => "Purple",
            MintYColorVariant::Orange => "Orange",
            MintYColorVariant::Sand => "Sand",
            MintYColorVariant::Red => "Red",
            MintYColorVariant::Yaru => "Yaru",
            MintYColorVariant::Emerald => "Emerald",
        }
    }
}

/// GTK3 CSS Variable Store representing @define-color definitions
#[derive(Debug, Clone)]
pub struct Gtk3CssVariableStore {
    pub theme_bg_color: String,
    pub theme_fg_color: String,
    pub theme_base_color: String,
    pub theme_text_color: String,
    pub theme_selected_bg_color: String,
    pub theme_selected_fg_color: String,
    pub wm_title_color: String,
    pub accent_color: String,
    pub warning_color: String,
    pub error_color: String,
    pub success_color: String,
    pub border_color: String,
    pub headerbar_bg_color: String,
}

impl Gtk3CssVariableStore {
    pub fn new_mint_y(variant: MintYColorVariant, dark_mode: bool) -> Self {
        let accent = variant.hex_code().to_string();

        if dark_mode {
            Self {
                theme_bg_color: "#2f343f".to_string(),
                theme_fg_color: "#f3f4f5".to_string(),
                theme_base_color: "#242831".to_string(),
                theme_text_color: "#f3f4f5".to_string(),
                theme_selected_bg_color: accent.clone(),
                theme_selected_fg_color: "#ffffff".to_string(),
                wm_title_color: "#ffffff".to_string(),
                accent_color: accent,
                warning_color: "#f57c00".to_string(),
                error_color: "#e53935".to_string(),
                success_color: "#43a047".to_string(),
                border_color: "#1c1f26".to_string(),
                headerbar_bg_color: "#232730".to_string(),
            }
        } else {
            Self {
                theme_bg_color: "#f5f6f7".to_string(),
                theme_fg_color: "#2f343f".to_string(),
                theme_base_color: "#ffffff".to_string(),
                theme_text_color: "#2f343f".to_string(),
                theme_selected_bg_color: accent.clone(),
                theme_selected_fg_color: "#ffffff".to_string(),
                wm_title_color: "#2f343f".to_string(),
                accent_color: accent,
                warning_color: "#fb8c00".to_string(),
                error_color: "#e53935".to_string(),
                success_color: "#43a047".to_string(),
                border_color: "#d3d4d5".to_string(),
                headerbar_bg_color: "#e8e9ea".to_string(),
            }
        }
    }

    pub fn get_css_variable(&self, var_name: &str) -> Option<&str> {
        match var_name {
            "theme_bg_color" => Some(&self.theme_bg_color),
            "theme_fg_color" => Some(&self.theme_fg_color),
            "theme_base_color" => Some(&self.theme_base_color),
            "theme_text_color" => Some(&self.theme_text_color),
            "theme_selected_bg_color" => Some(&self.theme_selected_bg_color),
            "theme_selected_fg_color" => Some(&self.theme_selected_fg_color),
            "wm_title" => Some(&self.wm_title_color),
            "accent_color" => Some(&self.accent_color),
            "warning_color" => Some(&self.warning_color),
            "error_color" => Some(&self.error_color),
            "success_color" => Some(&self.success_color),
            "border_color" => Some(&self.border_color),
            "headerbar_bg_color" => Some(&self.headerbar_bg_color),
            _ => None,
        }
    }

    pub fn to_gtk3_css(&self) -> String {
        let mut css = String::new();
        css.push_str(&format!("@define-color theme_bg_color {};\n", self.theme_bg_color));
        css.push_str(&format!("@define-color theme_fg_color {};\n", self.theme_fg_color));
        css.push_str(&format!("@define-color theme_base_color {};\n", self.theme_base_color));
        css.push_str(&format!("@define-color theme_text_color {};\n", self.theme_text_color));
        css.push_str(&format!("@define-color theme_selected_bg_color {};\n", self.theme_selected_bg_color));
        css.push_str(&format!("@define-color theme_selected_fg_color {};\n", self.theme_selected_fg_color));
        css.push_str(&format!("@define-color wm_title {};\n", self.wm_title_color));
        css.push_str(&format!("@define-color accent_color {};\n", self.accent_color));
        css.push_str(&format!("@define-color warning_color {};\n", self.warning_color));
        css.push_str(&format!("@define-color error_color {};\n", self.error_color));
        css.push_str(&format!("@define-color success_color {};\n", self.success_color));
        css.push_str(&format!("@define-color border_color {};\n", self.border_color));
        css.push_str(&format!("@define-color headerbar_bg_color {};\n", self.headerbar_bg_color));
        css
    }
}

/// GTK3 CSS Widget Selectors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Gtk3WidgetCssSelector {
    Window,
    Headerbar,
    SuggestedButton,
    FocusedEntry,
    CheckedTab,
    CheckedSwitch,
    SelectedRow,
    Custom(String),
}

impl Gtk3WidgetCssSelector {
    pub fn selector_string(&self) -> String {
        match self {
            Gtk3WidgetCssSelector::Window => "window.background".to_string(),
            Gtk3WidgetCssSelector::Headerbar => "headerbar, .titlebar".to_string(),
            Gtk3WidgetCssSelector::SuggestedButton => "button.suggested-action".to_string(),
            Gtk3WidgetCssSelector::FocusedEntry => "entry:focus, entry.focused".to_string(),
            Gtk3WidgetCssSelector::CheckedTab => "notebook tab:checked".to_string(),
            Gtk3WidgetCssSelector::CheckedSwitch => "switch:checked".to_string(),
            Gtk3WidgetCssSelector::SelectedRow => "treeview.view row:selected, row:selected".to_string(),
            Gtk3WidgetCssSelector::Custom(s) => s.clone(),
        }
    }
}

/// GTK3 CSS Rule Builder
#[derive(Debug, Clone)]
pub struct Gtk3CssRule {
    pub selector: Gtk3WidgetCssSelector,
    pub properties: Vec<(String, String)>,
}

impl Gtk3CssRule {
    pub fn new(selector: Gtk3WidgetCssSelector) -> Self {
        Self {
            selector,
            properties: Vec::new(),
        }
    }

    pub fn add_property(&mut self, key: &str, value: &str) -> &mut Self {
        self.properties.push((key.to_string(), value.to_string()));
        self
    }

    pub fn generate_rule_css(&self) -> String {
        let mut css = format!("{} {{\n", self.selector.selector_string());
        for (k, v) in &self.properties {
            css.push_str(&format!("  {}: {};\n", k, v));
        }
        css.push_str("}\n");
        css
    }
}

/// GTK3 `settings.ini` Configuration Manager
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gtk3Settings {
    pub gtk_theme_name: String,
    pub gtk_icon_theme_name: String,
    pub gtk_font_name: String,
    pub gtk_cursor_theme_name: String,
    pub gtk_application_prefer_dark_theme: bool,
    pub gtk_enable_animations: bool,
    pub gtk_overlay_scrolling: bool,
}

impl Gtk3Settings {
    pub fn new_mint_default(variant: MintYColorVariant, dark_mode: bool) -> Self {
        let mode_suffix = if dark_mode { "-Dark" } else { "" };
        let theme_name = format!("Mint-Y{}{}", mode_suffix, variant.variant_name());
        let icon_name = format!("Mint-Y-{}", variant.variant_name());

        Self {
            gtk_theme_name: theme_name,
            gtk_icon_theme_name: icon_name,
            gtk_font_name: "Ubuntu 10".to_string(),
            gtk_cursor_theme_name: "Bibata-Modern-Classic".to_string(),
            gtk_application_prefer_dark_theme: dark_mode,
            gtk_enable_animations: true,
            gtk_overlay_scrolling: true,
        }
    }

    pub fn parse_settings_ini(ini_content: &str) -> Self {
        let mut settings = Self::new_mint_default(MintYColorVariant::Emerald, false);

        for line in ini_content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('#') || trimmed.starts_with(';') || !trimmed.contains('=') {
                continue;
            }

            let mut parts = trimmed.splitn(2, '=');
            let key = parts.next().unwrap_or("").trim();
            let val = parts.next().unwrap_or("").trim();

            match key {
                "gtk-theme-name" => settings.gtk_theme_name = val.to_string(),
                "gtk-icon-theme-name" => settings.gtk_icon_theme_name = val.to_string(),
                "gtk-font-name" => settings.gtk_font_name = val.to_string(),
                "gtk-cursor-theme-name" => settings.gtk_cursor_theme_name = val.to_string(),
                "gtk-application-prefer-dark-theme" => {
                    settings.gtk_application_prefer_dark_theme = val == "1" || val == "true"
                }
                "gtk-enable-animations" => {
                    settings.gtk_enable_animations = val == "1" || val == "true"
                }
                "gtk-overlay-scrolling" => {
                    settings.gtk_overlay_scrolling = val == "1" || val == "true"
                }
                _ => {}
            }
        }

        settings
    }

    pub fn generate_settings_ini(&self) -> String {
        let mut ini = String::from("[Settings]\n");
        ini.push_str(&format!("gtk-theme-name={}\n", self.gtk_theme_name));
        ini.push_str(&format!("gtk-icon-theme-name={}\n", self.gtk_icon_theme_name));
        ini.push_str(&format!("gtk-font-name={}\n", self.gtk_font_name));
        ini.push_str(&format!("gtk-cursor-theme-name={}\n", self.gtk_cursor_theme_name));
        ini.push_str(&format!(
            "gtk-application-prefer-dark-theme={}\n",
            if self.gtk_application_prefer_dark_theme { "1" } else { "0" }
        ));
        ini.push_str(&format!(
            "gtk-enable-animations={}\n",
            if self.gtk_enable_animations { "1" } else { "0" }
        ));
        ini.push_str(&format!(
            "gtk-overlay-scrolling={}\n",
            if self.gtk_overlay_scrolling { "1" } else { "0" }
        ));
        ini
    }
}

/// FreeBSD Lumina / OpenBSD Xenocara / Distro GTK3 Styling Bridge
pub struct FreeBsdGtk3Bridge {
    pub gsettings_schema: String,
    pub xsettings_net_theme_name: String,
    pub shadow_elevation_radius_px: u32,
    pub window_border_radius_px: u32,
    pub backdrop_blur_radius_px: u32,
}

impl FreeBsdGtk3Bridge {
    pub fn new() -> Self {
        Self {
            gsettings_schema: "org.gnome.desktop.interface".to_string(),
            xsettings_net_theme_name: "Mint-Y-Dark-Emerald".to_string(),
            shadow_elevation_radius_px: 16,
            window_border_radius_px: 8,
            backdrop_blur_radius_px: 12,
        }
    }

    pub fn sync_from_settings(&mut self, settings: &Gtk3Settings) {
        self.xsettings_net_theme_name = settings.gtk_theme_name.clone();
    }

    pub fn export_xsettings_config(&self) -> String {
        format!(
            "Net/ThemeName \"{}\"\nNet/IconThemeName \"Mint-Y\"\nGtk/FontName \"Ubuntu 10\"\n",
            self.xsettings_net_theme_name
        )
    }

    pub fn generate_elevation_css(&self) -> String {
        format!(
            "window.background {{\n  border-radius: {}px;\n  box-shadow: 0 {}px {}px rgba(0, 0, 0, 0.35);\n}}\n",
            self.window_border_radius_px,
            self.shadow_elevation_radius_px / 2,
            self.shadow_elevation_radius_px
        )
    }
}

impl Default for FreeBsdGtk3Bridge {
    fn default() -> Self {
        Self::new()
    }
}

/// Binary GTK3 CSS Asset Compiler & Rule Cache
pub struct Gtk3CssAssetCache {
    pub compiled_cache: BTreeMap<String, String>,
}

impl Gtk3CssAssetCache {
    pub fn new() -> Self {
        Self {
            compiled_cache: BTreeMap::new(),
        }
    }

    pub fn get_or_compile(&mut self, variant: MintYColorVariant, dark_mode: bool) -> &str {
        let key = format!("{}-{}", variant.variant_name(), if dark_mode { "dark" } else { "light" });

        if !self.compiled_cache.contains_key(&key) {
            let vars = Gtk3CssVariableStore::new_mint_y(variant, dark_mode);
            let bridge = FreeBsdGtk3Bridge::new();

            let mut stylesheet = vars.to_gtk3_css();
            stylesheet.push_str("\n");
            stylesheet.push_str(&bridge.generate_elevation_css());

            // Build standard button rule
            let mut btn_rule = Gtk3CssRule::new(Gtk3WidgetCssSelector::SuggestedButton);
            btn_rule.add_property("background-color", "@accent_color");
            btn_rule.add_property("color", "#ffffff");
            btn_rule.add_property("border-radius", "4px");
            stylesheet.push_str(&btn_rule.generate_rule_css());

            self.compiled_cache.insert(key.clone(), stylesheet);
        }

        self.compiled_cache.get(&key).unwrap()
    }
}

impl Default for Gtk3CssAssetCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mint_y_color_variants() {
        let emerald = MintYColorVariant::Emerald;
        assert_eq!(emerald.hex_code(), "#92b989");
        assert_eq!(emerald.rgb_tuple(), (146, 185, 137));
        assert_eq!(emerald.variant_name(), "Emerald");

        let aqua = MintYColorVariant::Aqua;
        assert_eq!(aqua.hex_code(), "#00b0ff");
    }

    #[test]
    fn test_gtk3_css_variable_store() {
        let store = Gtk3CssVariableStore::new_mint_y(MintYColorVariant::Blue, true);
        assert_eq!(store.get_css_variable("accent_color"), Some("#3584e4"));
        assert_eq!(store.get_css_variable("theme_bg_color"), Some("#2f343f"));

        let css = store.to_gtk3_css();
        assert!(css.contains("@define-color theme_bg_color #2f343f;"));
        assert!(css.contains("@define-color accent_color #3584e4;"));
    }

    #[test]
    fn test_gtk3_widget_css_rule_builder() {
        let mut rule = Gtk3CssRule::new(Gtk3WidgetCssSelector::SuggestedButton);
        rule.add_property("background-color", "#00b0ff");
        rule.add_property("color", "#ffffff");

        let css = rule.generate_rule_css();
        assert!(css.contains("button.suggested-action {"));
        assert!(css.contains("background-color: #00b0ff;"));
        assert!(css.contains("color: #ffffff;"));
    }

    #[test]
    fn test_gtk3_settings_ini_parser_and_generator() {
        let settings = Gtk3Settings::new_mint_default(MintYColorVariant::Teal, true);
        let ini = settings.generate_settings_ini();

        assert!(ini.contains("[Settings]"));
        assert!(ini.contains("gtk-theme-name=Mint-Y-DarkTeal"));
        assert!(ini.contains("gtk-application-prefer-dark-theme=1"));

        let parsed = Gtk3Settings::parse_settings_ini(&ini);
        assert_eq!(parsed.gtk_theme_name, "Mint-Y-DarkTeal");
        assert!(parsed.gtk_application_prefer_dark_theme);
    }

    #[test]
    fn test_freebsd_gtk3_bridge() {
        let mut bridge = FreeBsdGtk3Bridge::new();
        let settings = Gtk3Settings::new_mint_default(MintYColorVariant::Orange, false);
        bridge.sync_from_settings(&settings);

        let xsettings = bridge.export_xsettings_config();
        assert!(xsettings.contains("Net/ThemeName \"Mint-YOrange\""));

        let elevation_css = bridge.generate_elevation_css();
        assert!(elevation_css.contains("border-radius: 8px;"));
        assert!(elevation_css.contains("box-shadow:"));
    }

    #[test]
    fn test_gtk3_css_asset_cache() {
        let mut cache = Gtk3CssAssetCache::new();
        let compiled = cache.get_or_compile(MintYColorVariant::Aqua, true).to_string();

        assert!(compiled.contains("@define-color accent_color #00b0ff;"));
        assert!(compiled.contains("button.suggested-action"));
        assert_eq!(cache.compiled_cache.len(), 1);
    }
}
