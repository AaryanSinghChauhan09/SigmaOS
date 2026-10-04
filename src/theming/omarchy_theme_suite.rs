#![no_std]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::string::ToString;

#[derive(Clone)]
pub struct ThemePalette {
    pub name: String,
    pub is_sigma_exclusive: bool,
    pub accent: String,
    pub background: String,
    pub foreground: String,
    pub term_colors: [String; 16],
}

impl ThemePalette {
    pub fn new(name: &str, accent: &str, background: &str, foreground: &str) -> Self {
        // Default 16 terminal colors placeholder
        let term_colors = [
            background.to_string(), "#cc241d".to_string(), "#98971a".to_string(), "#d79921".to_string(),
            "#458588".to_string(), "#b16286".to_string(), "#689d6a".to_string(), "#a89984".to_string(),
            "#928374".to_string(), "#fb4934".to_string(), "#b8bb26".to_string(), "#fabd2f".to_string(),
            "#83a598".to_string(), "#d3869b".to_string(), "#8ec07c".to_string(), foreground.to_string(),
        ];

        Self {
            name: name.to_string(),
            is_sigma_exclusive: name.starts_with("sigma-"),
            accent: accent.to_string(),
            background: background.to_string(),
            foreground: foreground.to_string(),
            term_colors,
        }
    }
}

pub struct OmarchyThemeSuite {
    pub available_themes: Vec<ThemePalette>,
}

impl OmarchyThemeSuite {
    pub fn new() -> Self {
        let mut themes = Vec::new();
        
        let omarchy_themes = [
            "catppuccin", "catppuccin-latte", "ethereal", "everforest", "flexoki-light", 
            "gruvbox", "hackerman", "kanagawa", "last-horizon", "lumon", "lupine", 
            "matte-black", "miasma", "nord", "osaka-jade", "retro-82", "ristretto", 
            "rose-pine", "solitude", "tokyo-night", "vantablack", "white"
        ];
        
        for t in omarchy_themes.iter() {
            themes.push(ThemePalette::new(t, "#7aa2f7", "#1a1b26", "#c0caf5"));
        }

        let sigma_themes = [
            "sigma-dark", "sigma-neon", "sigma-sovereign", "sigma-crystal",
            "sigma-obsidian", "sigma-aurora", "sigma-eclipse", "sigma-ultraviolet"
        ];

        for t in sigma_themes.iter() {
            themes.push(ThemePalette::new(t, "#00e5ff", "#0b0f14", "#e6edf3"));
        }

        let mint_themes = [
            "mint-y-dark", "mint-y-light", "mint-y-blue-dark", "mint-y-ocean"
        ];

        for t in mint_themes.iter() {
            let palette = match *t {
                "mint-y-light" => ThemePalette::new(t, "#86be43", "#ffffff", "#2e3436"),
                "mint-y-blue-dark" => ThemePalette::new(t, "#6495ed", "#2c3e50", "#ecf0f1"),
                "mint-y-ocean" => ThemePalette::new(t, "#17a2b8", "#103c48", "#e8f6f3"),
                _ => ThemePalette::new(t, "#86be43", "#222222", "#eeeeec"),
            };
            themes.push(palette);
        }

        Self { available_themes: themes }
    }
}

pub struct SovereignThemeEngine {
    pub current_theme: Option<ThemePalette>,
}

impl SovereignThemeEngine {
    pub fn new() -> Self {
        Self { current_theme: None }
    }
    
    pub fn hot_reload(&mut self, theme: ThemePalette) {
        self.current_theme = Some(theme);
    }
    
    // Stub for Unix signal handler logic
    pub fn setup_hot_reload_signal_handler(&self) {
        // In a std environment, we would use signal(SIGUSR1, handler)
        // Since we are no_std, this acts as a design placeholder.
    }

    pub fn export_colors_css(&self) -> String {
        match &self.current_theme {
            Some(t) => {
                let mut out = String::from(":root {\n");
                out.push_str(&format!("  --background: {};\n", t.background));
                out.push_str(&format!("  --foreground: {};\n", t.foreground));
                out.push_str(&format!("  --accent: {};\n", t.accent));
                for (i, col) in t.term_colors.iter().enumerate() {
                    out.push_str(&format!("  --color{}: {};\n", i, col));
                }
                out.push_str("}\n");
                out
            }
            None => String::from(":root { --background: #000; --foreground: #fff; --accent: #7aa2f7; }\n"),
        }
    }

    pub fn export_colors_waybar(&self) -> String {
        match &self.current_theme {
            Some(t) => {
                let mut out = String::from("/* Waybar Colors */\n");
                out.push_str(&format!("@define-color background {};\n", t.background));
                out.push_str(&format!("@define-color foreground {};\n", t.foreground));
                out.push_str(&format!("@define-color accent {};\n", t.accent));
                out
            }
            None => String::from("@define-color background #000000;\n@define-color foreground #ffffff;\n@define-color accent #7aa2f7;\n"),
        }
    }

    pub fn export_colors_hyprland(&self) -> String {
        match &self.current_theme {
            Some(t) => format!(
                "$background = 0xff{}\n$foreground = 0xff{}\n$accent = 0xff{}\n",
                &t.background[1..], &t.foreground[1..], &t.accent[1..]
            ),
            None => String::from("$background = 0xff000000\n$foreground = 0xffffffff\n$accent = 0xff7aa2f7\n"),
        }
    }

    pub fn export_colors_ghostty(&self) -> String {
        match &self.current_theme {
            Some(t) => {
                let mut out = String::from("theme = custom\n");
                out.push_str(&format!("background = {}\n", &t.background[1..]));
                out.push_str(&format!("foreground = {}\n", &t.foreground[1..]));
                out.push_str(&format!("cursor-color = {}\n", &t.accent[1..]));
                
                for (i, col) in t.term_colors.iter().enumerate() {
                    out.push_str(&format!("palette = {}={}\n", i, col));
                }
                out
            }
            None => String::from("theme = dark\n"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_omarchy_theme_suite_initialization() {
        let suite = OmarchyThemeSuite::new();
        assert_eq!(suite.available_themes.len(), 22 + 8 + 4);
        
        let exclusive_count = suite.available_themes.iter().filter(|t| t.is_sigma_exclusive).count();
        assert_eq!(exclusive_count, 8);
    }
    
    #[test]
    fn test_sovereign_theme_engine_css_export() {
        let mut engine = SovereignThemeEngine::new();
        engine.hot_reload(ThemePalette::new("sigma-dark", "#00e5ff", "#0b0f14", "#e6edf3"));
        
        let css = engine.export_colors_css();
        assert!(css.contains("--accent: #00e5ff;"));
        assert!(css.contains("--background: #0b0f14;"));
        assert!(css.contains("--color0:"));
        assert!(css.contains("--color15:"));
    }

    #[test]
    fn test_sovereign_theme_engine_hyprland_export() {
        let mut engine = SovereignThemeEngine::new();
        engine.hot_reload(ThemePalette::new("sigma-dark", "#00e5ff", "#0b0f14", "#e6edf3"));

        let hypr = engine.export_colors_hyprland();
        assert!(hypr.contains("$accent = 0xff00e5ff"));
        assert!(hypr.contains("$background = 0xff0b0f14"));
    }

    #[test]
    fn test_sovereign_theme_engine_ghostty_export() {
        let mut engine = SovereignThemeEngine::new();
        engine.hot_reload(ThemePalette::new("sigma-dark", "#00e5ff", "#0b0f14", "#e6edf3"));

        let ghostty = engine.export_colors_ghostty();
        assert!(ghostty.contains("background = 0b0f14"));
        assert!(ghostty.contains("cursor-color = 00e5ff"));
        assert!(ghostty.contains("palette = 0="));
        assert!(ghostty.contains("palette = 15="));
    }

    #[test]
    fn test_sovereign_theme_engine_waybar_export() {
        let mut engine = SovereignThemeEngine::new();
        engine.hot_reload(ThemePalette::new("sigma-dark", "#00e5ff", "#0b0f14", "#e6edf3"));

        let waybar = engine.export_colors_waybar();
        assert!(waybar.contains("@define-color background #0b0f14;"));
        assert!(waybar.contains("@define-color accent #00e5ff;"));
    }
}
