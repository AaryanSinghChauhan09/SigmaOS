use std::collections::HashMap;
use std::path::PathBuf;

/// Represents a font family with various styles
#[derive(Debug, Clone)]
pub struct FontFamily {
    pub name: String,
    pub styles: Vec<String>, // e.g., "Regular", "Bold"
    pub file_paths: Vec<PathBuf>,
}

/// Settings for default system fonts
#[derive(Debug, Clone)]
pub struct SystemFontSettings {
    pub ui_font: String,
    pub monospace_font: String,
    pub document_font: String,
    pub title_font: String,
}

impl Default for SystemFontSettings {
    fn default() -> Self {
        SystemFontSettings {
            ui_font: "Ubuntu 11".to_string(),
            monospace_font: "Ubuntu Mono 11".to_string(),
            document_font: "Sans 11".to_string(),
            title_font: "Ubuntu Bold 11".to_string(),
        }
    }
}

/// Font cache similar to fc-cache
pub struct FontCache {
    fonts: HashMap<String, FontFamily>,
}

impl FontCache {
    pub fn new() -> Self {
        FontCache {
            fonts: HashMap::new(),
        }
    }

    pub fn generate(&mut self, dirs: &[PathBuf]) {
        // Simulating font discovery
        for _dir in dirs {
            let mut family = FontFamily {
                name: "DetectedSans".to_string(),
                styles: vec!["Regular".to_string(), "Bold".to_string()],
                file_paths: vec![],
            };
            self.fonts.insert(family.name.clone(), family);
        }
    }

    pub fn get_family(&self, name: &str) -> Option<&FontFamily> {
        self.fonts.get(name)
    }
}

pub struct FontInstaller;

impl FontInstaller {
    pub fn install(file: PathBuf) -> Result<(), String> {
        let ext = file.extension().and_then(|s| s.to_str()).unwrap_or("");
        if ["ttf", "otf", "woff2"].contains(&ext) {
            // copy to ~/.local/share/fonts
            Ok(())
        } else {
            Err("Unsupported font format".to_string())
        }
    }

    pub fn uninstall(_name: &str) -> Result<(), String> {
        Ok(())
    }
}

pub struct FontPreview;

impl FontPreview {
    pub fn render_sample(font_name: &str, size: u32, text: &str) -> String {
        format!("Rendering '{}' at {}pt: {}", font_name, size, text)
    }
}

pub struct FontFinder;

impl FontFinder {
    pub fn search(query: &str) -> Vec<String> {
        // Stub for searching Google Fonts
        if query.to_lowercase().contains("roboto") {
            vec!["Roboto".to_string(), "Roboto Mono".to_string()]
        } else {
            vec![]
        }
    }
}

pub struct FontConfig;

impl FontConfig {
    pub fn generate_rules(anti_aliasing: bool, hinting: &str) -> String {
        format!(
            "<?xml version=\"1.0\"?>\n<fontconfig>\n  <match target=\"font\">\n    <edit name=\"antialias\" mode=\"assign\"><bool>{}</bool></edit>\n    <edit name=\"hintstyle\" mode=\"assign\"><const>{}</const></edit>\n  </match>\n</fontconfig>",
            anti_aliasing, hinting
        )
    }
}

pub struct FontSubstitution {
    pub fallback_chains: HashMap<String, Vec<String>>,
}

impl FontSubstitution {
    pub fn new() -> Self {
        FontSubstitution {
            fallback_chains: HashMap::new(),
        }
    }

    pub fn add_fallback(&mut self, family: &str, fallback: &str) {
        self.fallback_chains
            .entry(family.to_string())
            .or_insert_with(Vec::new)
            .push(fallback.to_string());
    }
}

pub struct EmojiSupport;

impl EmojiSupport {
    pub fn configure_priority() -> String {
        // Generates fontconfig snippet for Noto Color Emoji priority
        "<alias><family>sans-serif</family><prefer><family>Noto Color Emoji</family></prefer></alias>".to_string()
    }
}

/// The main FontManager struct
pub struct FontManager {
    pub cache: FontCache,
    pub settings: SystemFontSettings,
    pub substitution: FontSubstitution,
}

impl FontManager {
    pub fn new() -> Self {
        FontManager {
            cache: FontCache::new(),
            settings: SystemFontSettings::default(),
            substitution: FontSubstitution::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_manager_creation() {
        let fm = FontManager::new();
        assert_eq!(fm.settings.ui_font, "Ubuntu 11");
    }

    #[test]
    fn test_font_installer() {
        assert!(FontInstaller::install(PathBuf::from("myfont.ttf")).is_ok());
        assert!(FontInstaller::install(PathBuf::from("myfont.txt")).is_err());
    }

    #[test]
    fn test_font_finder() {
        let results = FontFinder::search("Roboto");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_font_preview() {
        let sample = FontPreview::render_sample("Arial", 12, "Hello");
        assert_eq!(sample, "Rendering 'Arial' at 12pt: Hello");
    }

    #[test]
    fn test_font_config() {
        let xml = FontConfig::generate_rules(true, "hintslight");
        assert!(xml.contains("<bool>true</bool>"));
        assert!(xml.contains("<const>hintslight</const>"));
    }
    
    #[test]
    fn test_emoji_support() {
        let conf = EmojiSupport::configure_priority();
        assert!(conf.contains("Noto Color Emoji"));
    }
}
