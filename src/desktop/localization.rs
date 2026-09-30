// localization.rs

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TextDirection {
    LTR,
    RTL,
    Auto,
}

pub struct LocaleManager {
    pub current_locale: String,
    pub fallback_chain: Vec<String>,
}

impl LocaleManager {
    pub fn new(locale: &str) -> Self {
        Self {
            current_locale: locale.to_string(),
            fallback_chain: vec!["en_US".to_string(), "en".to_string()],
        }
    }
}

pub struct TranslationCatalog {
    messages: HashMap<String, String>,
}

impl TranslationCatalog {
    pub fn new() -> Self {
        Self { messages: HashMap::new() }
    }
    pub fn add_message(&mut self, key: &str, value: &str) {
        self.messages.insert(key.to_string(), value.to_string());
    }
    pub fn get_message(&self, key: &str) -> Option<&String> {
        self.messages.get(key)
    }
}

pub struct InputMethodFramework {
    pub active_engine: String,
}
impl InputMethodFramework {
    pub fn new() -> Self { Self { active_engine: "ibus".to_string() } }
    pub fn switch_engine(&mut self, engine: &str) { self.active_engine = engine.to_string(); }
}

pub struct DateTimeFormatter {
    pub locale: String,
}
impl DateTimeFormatter {
    pub fn format(&self, _timestamp: i64) -> String {
        "formatted date".to_string()
    }
}

pub struct NumberFormatter {
    pub locale: String,
}
impl NumberFormatter {
    pub fn format(&self, number: f64) -> String {
        format!("{:.2}", number)
    }
}

pub struct KeyboardLayoutManager {
    pub layouts: Vec<String>,
    pub current_index: usize,
}
impl KeyboardLayoutManager {
    pub fn new() -> Self { Self { layouts: vec!["us".to_string()], current_index: 0 } }
    pub fn add_layout(&mut self, layout: &str) { self.layouts.push(layout.to_string()); }
    pub fn switch_next(&mut self) {
        if !self.layouts.is_empty() {
            self.current_index = (self.current_index + 1) % self.layouts.len();
        }
    }
}

pub struct SpellChecker {
    pub language: String,
}
impl SpellChecker {
    pub fn check(&self, word: &str) -> bool { word != "teh" }
}

pub struct FontFallbackResolver;
impl FontFallbackResolver {
    pub fn resolve(&self, script: &str) -> String {
        match script {
            "CJK" => "Noto Sans CJK".to_string(),
            "Arabic" => "Noto Naskh Arabic".to_string(),
            _ => "Noto Sans".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_manager() {
        let lm = LocaleManager::new("fr_FR");
        assert_eq!(lm.current_locale, "fr_FR");
        assert_eq!(lm.fallback_chain[0], "en_US");
    }

    #[test]
    fn test_translation_catalog() {
        let mut catalog = TranslationCatalog::new();
        catalog.add_message("hello", "bonjour");
        assert_eq!(catalog.get_message("hello"), Some(&"bonjour".to_string()));
    }

    #[test]
    fn test_imf() {
        let mut imf = InputMethodFramework::new();
        assert_eq!(imf.active_engine, "ibus");
        imf.switch_engine("fcitx");
        assert_eq!(imf.active_engine, "fcitx");
    }

    #[test]
    fn test_keyboard_manager() {
        let mut km = KeyboardLayoutManager::new();
        km.add_layout("fr");
        km.switch_next();
        assert_eq!(km.layouts[km.current_index], "fr");
    }

    #[test]
    fn test_spell_checker() {
        let sc = SpellChecker { language: "en".to_string() };
        assert!(sc.check("the"));
        assert!(!sc.check("teh"));
    }

    #[test]
    fn test_font_fallback() {
        let resolver = FontFallbackResolver;
        assert_eq!(resolver.resolve("CJK"), "Noto Sans CJK");
        assert_eq!(resolver.resolve("Latin"), "Noto Sans");
    }
}
