#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::string::ToString;

pub struct ThemePalette {
    pub name: String,
    pub is_sigma_exclusive: bool,
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
            themes.push(ThemePalette { name: t.to_string(), is_sigma_exclusive: false });
        }
        
        let sigma_themes = [
            "sigma-dark", "sigma-neon", "sigma-sovereign", "sigma-crystal", 
            "sigma-obsidian", "sigma-aurora", "sigma-eclipse", "sigma-ultraviolet"
        ];
        
        for t in sigma_themes.iter() {
            themes.push(ThemePalette { name: t.to_string(), is_sigma_exclusive: true });
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
    
    pub fn export_colors_css(&self) -> String {
        String::from(":root { --bg: #000; }")
    }
    
    pub fn export_colors_gtk(&self) -> String {
        String::from("@define-color bg #000;")
    }
    
    pub fn export_colors_hyprland(&self) -> String {
        String::from("$bg = 0xff000000")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_omarchy_theme_suite_initialization() {
        let suite = OmarchyThemeSuite::new();
        assert_eq!(suite.available_themes.len(), 22 + 8);
        
        let exclusive_count = suite.available_themes.iter().filter(|t| t.is_sigma_exclusive).count();
        assert_eq!(exclusive_count, 8);
    }
    
    #[test]
    fn test_sovereign_theme_engine() {
        let mut engine = SovereignThemeEngine::new();
        engine.hot_reload(ThemePalette { name: String::from("sigma-dark"), is_sigma_exclusive: true });
        assert!(engine.current_theme.is_some());
        assert_eq!(engine.current_theme.as_ref().unwrap().name, "sigma-dark");
        
        assert_eq!(engine.export_colors_css(), ":root { --bg: #000; }");
        assert_eq!(engine.export_colors_gtk(), "@define-color bg #000;");
        assert_eq!(engine.export_colors_hyprland(), "$bg = 0xff000000");
    }
}
