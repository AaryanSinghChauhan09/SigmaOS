//! Linux Mint mintlocale-inspired Locale Manager
//! 
//! This module implements a locale manager inspired by Linux Mint's mintlocale,
//! which configures system locale settings and language packs.

#![allow(dead_code)]



use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// Locale information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocaleInfo {
    /// Locale code (e.g., "en_US.UTF-8")
    pub code: String,
    /// Language name
    pub language: String,
    /// Territory/region name
    pub territory: String,
    /// Character encoding
    pub encoding: String,
    /// Whether locale is installed
    pub installed: bool,
    /// Whether locale is the default
    pub is_default: bool,
}

/// Language pack information
#[derive(Debug, Clone)]
pub struct LanguagePack {
    /// Language code (e.g., "en", "fr", "de")
    pub language_code: String,
    /// Pack name
    pub pack_name: String,
    /// Pack version
    pub version: String,
    /// Whether pack is installed
    pub installed: bool,
    /// Pack size in bytes
    pub size: u64,
}

/// Locale setting type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LocaleSettingType {
    /// System locale
    System,
    /// Language
    Language,
    /// Numeric format
    Numeric,
    /// Time format
    Time,
    /// Monetary format
    Monetary,
    /// Paper format
    Paper,
    /// Measurement system
    Measurement,
    /// Name format
    Name,
    /// Address format
    Address,
    /// Telephone format
    Telephone,
}

impl LocaleSettingType {
    /// Get environment variable name for the setting
    pub fn env_var(&self) -> &'static str {
        match self {
            LocaleSettingType::System => "LANG",
            LocaleSettingType::Language => "LANGUAGE",
            LocaleSettingType::Numeric => "LC_NUMERIC",
            LocaleSettingType::Time => "LC_TIME",
            LocaleSettingType::Monetary => "LC_MONETARY",
            LocaleSettingType::Paper => "LC_PAPER",
            LocaleSettingType::Measurement => "LC_MEASUREMENT",
            LocaleSettingType::Name => "LC_NAME",
            LocaleSettingType::Address => "LC_ADDRESS",
            LocaleSettingType::Telephone => "LC_TELEPHONE",
        }
    }
}

/// Locale manager - manages system locale settings
#[derive(Debug)]
pub struct MintLocaleManager {
    /// Available locales
    pub locales: Vec<LocaleInfo>,
    /// Available language packs
    pub language_packs: Vec<LanguagePack>,
    /// Current locale settings
    pub settings: BTreeMap<LocaleSettingType, String>,
    /// Default system locale
    pub default_locale: Option<String>,
}

impl MintLocaleManager {
    /// Create a new Locale Manager
    pub fn new() -> Self {
        Self {
            locales: Vec::new(),
            language_packs: Vec::new(),
            settings: BTreeMap::new(),
            default_locale: None,
        }
    }

    /// Add a locale
    pub fn add_locale(&mut self, locale: LocaleInfo) {
        self.locales.push(locale);
    }

    /// Add a language pack
    pub fn add_language_pack(&mut self, pack: LanguagePack) {
        self.language_packs.push(pack);
    }

    /// Get locale by code
    pub fn get_locale(&self, code: &str) -> Option<&LocaleInfo> {
        self.locales.iter().find(|l| l.code == code)
    }

    /// Get installed locales
    pub fn get_installed_locales(&self) -> Vec<&LocaleInfo> {
        self.locales.iter().filter(|l| l.installed).collect()
    }

    /// Get available (not installed) locales
    pub fn get_available_locales(&self) -> Vec<&LocaleInfo> {
        self.locales.iter().filter(|l| !l.installed).collect()
    }

    /// Set a locale setting
    pub fn set_locale_setting(&mut self, setting_type: LocaleSettingType, locale_code: String) {
        self.settings.insert(setting_type, locale_code);
    }

    /// Get a locale setting
    pub fn get_locale_setting(&self, setting_type: LocaleSettingType) -> Option<&String> {
        self.settings.get(&setting_type)
    }

    /// Set the default system locale
    pub fn set_default_locale(&mut self, locale_code: String) {
        self.default_locale = Some(locale_code.clone());
        self.set_locale_setting(LocaleSettingType::System, locale_code);
    }

    /// Get the default system locale
    pub fn get_default_locale(&self) -> Option<&String> {
        self.default_locale.as_ref()
    }

    /// Install a locale
    pub fn install_locale(&mut self, locale_code: &str) -> Result<(), String> {
        if let Some(locale) = self.locales.iter_mut().find(|l| l.code == locale_code) {
            locale.installed = true;
            Ok(())
        } else {
            Err("Locale not found".to_string())
        }
    }

    /// Remove a locale
    pub fn remove_locale(&mut self, locale_code: &str) -> Result<(), String> {
        if let Some(locale) = self.locales.iter_mut().find(|l| l.code == locale_code) {
            if locale.is_default {
                return Err("Cannot remove default locale".to_string());
            }
            locale.installed = false;
            Ok(())
        } else {
            Err("Locale not found".to_string())
        }
    }

    /// Install a language pack
    pub fn install_language_pack(&mut self, language_code: &str) -> Result<(), String> {
        if let Some(pack) = self.language_packs.iter_mut().find(|p| p.language_code == language_code) {
            pack.installed = true;
            Ok(())
        } else {
            Err("Language pack not found".to_string())
        }
    }

    /// Remove a language pack
    pub fn remove_language_pack(&mut self, language_code: &str) -> Result<(), String> {
        if let Some(pack) = self.language_packs.iter_mut().find(|p| p.language_code == language_code) {
            pack.installed = false;
            Ok(())
        } else {
            Err("Language pack not found".to_string())
        }
    }

    /// Get language packs for a language
    pub fn get_language_packs(&self, language_code: &str) -> Vec<&LanguagePack> {
        self.language_packs
            .iter()
            .filter(|p| p.language_code == language_code)
            .collect()
    }

    /// Get installed language packs
    pub fn get_installed_language_packs(&self) -> Vec<&LanguagePack> {
        self.language_packs.iter().filter(|p| p.installed).collect()
    }

    /// Search locales by language name
    pub fn search_locales(&self, query: &str) -> Vec<&LocaleInfo> {
        let query_lower = query.to_lowercase();
        self.locales
            .iter()
            .filter(|l| {
                l.language.to_lowercase().contains(&query_lower)
                    || l.territory.to_lowercase().contains(&query_lower)
                    || l.code.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Generate locale configuration (for /etc/default/locale)
    pub fn generate_locale_config(&self) -> Vec<String> {
        let mut config = Vec::new();
        
        if let Some(default) = &self.default_locale {
            config.push(format!("LANG={}", default));
        }

        for (setting_type, value) in &self.settings {
            if *setting_type != LocaleSettingType::System {
                config.push(format!("{}={}", setting_type.env_var(), value));
            }
        }

        config
    }

    /// Apply locale settings system-wide
    pub fn apply_settings(&self) -> Result<(), String> {
        // In a real implementation, this would:
        // 1. Write to /etc/default/locale
        // 2. Update locale-gen
        // 3. Update system locale
        // 4. Restart affected services
        
        Ok(())
    }

    /// Get locale statistics
    pub fn get_statistics(&self) -> LocaleStatistics {
        LocaleStatistics {
            total_locales: self.locales.len(),
            installed_locales: self.locales.iter().filter(|l| l.installed).count(),
            total_language_packs: self.language_packs.len(),
            installed_language_packs: self.language_packs.iter().filter(|p| p.installed).count(),
            default_locale: self.default_locale.clone(),
        }
    }
}

/// Locale statistics
#[derive(Debug, Clone)]
pub struct LocaleStatistics {
    /// Total number of available locales
    pub total_locales: usize,
    /// Number of installed locales
    pub installed_locales: usize,
    /// Total number of language packs
    pub total_language_packs: usize,
    /// Number of installed language packs
    pub installed_language_packs: usize,
    /// Default locale code
    pub default_locale: Option<String>,
}

impl Default for MintLocaleManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// MINTLOCALE ENHANCEMENTS: INPUT METHODS, DICTIONARIES, PREVIEWS & LOCALED
// ============================================================================

/// Supported Input Method Frameworks for non-Latin and CJK text input (mintlocale IM selector parity).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMethodFramework {
    Fcitx5,
    IBus,
    Uim,
    Gcin,
    NativeSigmaIm,
}

impl InputMethodFramework {
    /// Generates environment variable settings for GTK, Qt, and X11 input method integration.
    pub fn get_env_vars(&self) -> Vec<(String, String)> {
        match self {
            InputMethodFramework::Fcitx5 => vec![
                ("GTK_IM_MODULE".to_string(), "fcitx".to_string()),
                ("QT_IM_MODULE".to_string(), "fcitx".to_string()),
                ("XMODIFIERS".to_string(), "@im=fcitx".to_string()),
            ],
            InputMethodFramework::IBus => vec![
                ("GTK_IM_MODULE".to_string(), "ibus".to_string()),
                ("QT_IM_MODULE".to_string(), "ibus".to_string()),
                ("XMODIFIERS".to_string(), "@im=ibus".to_string()),
            ],
            InputMethodFramework::Uim => vec![
                ("GTK_IM_MODULE".to_string(), "uim".to_string()),
                ("QT_IM_MODULE".to_string(), "uim".to_string()),
                ("XMODIFIERS".to_string(), "@im=uim".to_string()),
            ],
            InputMethodFramework::Gcin => vec![
                ("GTK_IM_MODULE".to_string(), "gcin".to_string()),
                ("QT_IM_MODULE".to_string(), "gcin".to_string()),
                ("XMODIFIERS".to_string(), "@im=gcin".to_string()),
            ],
            InputMethodFramework::NativeSigmaIm => vec![
                ("GTK_IM_MODULE".to_string(), "sigma_im".to_string()),
                ("QT_IM_MODULE".to_string(), "sigma_im".to_string()),
                ("XMODIFIERS".to_string(), "@im=sigma_im".to_string()),
            ],
        }
    }
}

/// Category of language support packages (dictionaries, fonts, input methods).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LanguageComponentCategory {
    SpellcheckerHunspell,
    SpellcheckerAspell,
    DictionaryMySpell,
    FontPackage,
    InputMethodEngine,
}

/// Language support component entry.
#[derive(Debug, Clone)]
pub struct LanguageComponent {
    pub language_code: String,
    pub package_name: String,
    pub category: LanguageComponentCategory,
    pub installed: bool,
}

/// Audits installed packages and identifies missing spellchecking dictionaries, fonts, and input methods.
pub struct MissingLanguageComponentAuditor {
    pub components: Vec<LanguageComponent>,
}

impl MissingLanguageComponentAuditor {
    pub fn new() -> Self {
        Self { components: Vec::new() }
    }

    pub fn register_component(&mut self, component: LanguageComponent) {
        self.components.push(component);
    }

    /// Returns all uninstalled packages required for full language support for a given language code.
    pub fn find_missing_components(&self, lang_code: &str) -> Vec<LanguageComponent> {
        self.components
            .iter()
            .filter(|c| c.language_code == lang_code && !c.installed)
            .cloned()
            .collect()
    }
}

impl Default for MissingLanguageComponentAuditor {
    fn default() -> Self {
        Self::new()
    }
}

/// Live preview formatter for date, time, currency, numbers, and paper size based on regional locale.
pub struct LocaleFormatPreview;

impl LocaleFormatPreview {
    /// Renders a sample formatted currency string for a given locale code.
    pub fn preview_currency(locale_code: &str, amount: f64) -> String {
        if locale_code.starts_with("fr_") || locale_code.starts_with("de_") || locale_code.starts_with("es_") {
            format!("{:.2} €", amount)
        } else if locale_code.starts_with("en_GB") {
            format!("£{:.2}", amount)
        } else if locale_code.starts_with("ja_") || locale_code.starts_with("zh_") {
            format!("¥{:.0}", amount)
        } else {
            format!("${:.2}", amount)
        }
    }

    /// Renders a sample date format string for a given locale code.
    pub fn preview_date(locale_code: &str) -> String {
        if locale_code.starts_with("en_US") {
            "12/31/2026".to_string()
        } else if locale_code.starts_with("de_") || locale_code.starts_with("ru_") {
            "31.12.2026".to_string()
        } else if locale_code.starts_with("ja_") || locale_code.starts_with("zh_") {
            "2026/12/31".to_string()
        } else {
            "31/12/2026".to_string()
        }
    }

    /// Returns standard paper size for the locale (e.g. Letter for US/Canada, A4 elsewhere).
    pub fn preview_paper_size(locale_code: &str) -> &'static str {
        if locale_code.starts_with("en_US") || locale_code.starts_with("en_CA") {
            "Letter"
        } else {
            "A4"
        }
    }
}

/// Configuration generator for `/etc/locale.conf`, `/etc/default/locale`, and systemd `org.freedesktop.locale1` DBus interface.
pub struct SystemdLocaledConfigGenerator;

impl SystemdLocaledConfigGenerator {
    /// Generates content for `/etc/locale.conf` or `/etc/default/locale`.
    pub fn generate_etc_locale_conf(manager: &MintLocaleManager) -> String {
        let mut lines = Vec::new();
        if let Some(default_lang) = manager.get_default_locale() {
            lines.push(format!("LANG={}", default_lang));
        } else {
            lines.push("LANG=en_US.UTF-8".to_string());
        }

        for (setting_type, val) in &manager.settings {
            if *setting_type != LocaleSettingType::System {
                lines.push(format!("{}={}", setting_type.env_var(), val));
            }
        }

        lines.join("\n")
    }

    /// Generates systemd-localed DBus method call parameters for `SetLocale`.
    pub fn generate_dbus_set_locale_args(manager: &MintLocaleManager) -> Vec<String> {
        let mut args = Vec::new();
        if let Some(default_lang) = manager.get_default_locale() {
            args.push(format!("LANG={}", default_lang));
        }
        for (setting_type, val) in &manager.settings {
            if *setting_type != LocaleSettingType::System {
                args.push(format!("{}={}", setting_type.env_var(), val));
            }
        }
        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_manager_creation() {
        let manager = MintLocaleManager::new();
        assert_eq!(manager.locales.len(), 0);
        assert_eq!(manager.language_packs.len(), 0);
    }

    #[test]
    fn test_add_locale() {
        let mut manager = MintLocaleManager::new();
        
        let locale = LocaleInfo {
            code: "en_US.UTF-8".to_string(),
            language: "English".to_string(),
            territory: "United States".to_string(),
            encoding: "UTF-8".to_string(),
            installed: false,
            is_default: false,
        };
        
        manager.add_locale(locale);
        assert_eq!(manager.locales.len(), 1);
    }

    #[test]
    fn test_install_locale() {
        let mut manager = MintLocaleManager::new();
        
        let locale = LocaleInfo {
            code: "en_US.UTF-8".to_string(),
            language: "English".to_string(),
            territory: "United States".to_string(),
            encoding: "UTF-8".to_string(),
            installed: false,
            is_default: false,
        };
        
        manager.add_locale(locale);
        let result = manager.install_locale("en_US.UTF-8");
        assert!(result.is_ok());
        assert!(manager.get_locale("en_US.UTF-8").unwrap().installed);
    }

    #[test]
    fn test_set_default_locale() {
        let mut manager = MintLocaleManager::new();
        
        manager.set_default_locale("en_US.UTF-8".to_string());
        assert_eq!(manager.default_locale, Some("en_US.UTF-8".to_string()));
        assert_eq!(
            manager.get_locale_setting(LocaleSettingType::System),
            Some(&"en_US.UTF-8".to_string())
        );
    }

    #[test]
    fn test_search_locales() {
        let mut manager = MintLocaleManager::new();
        
        let locale = LocaleInfo {
            code: "en_US.UTF-8".to_string(),
            language: "English".to_string(),
            territory: "United States".to_string(),
            encoding: "UTF-8".to_string(),
            installed: false,
            is_default: false,
        };
        
        manager.add_locale(locale);
        let results = manager.search_locales("English");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_locale_statistics() {
        let mut manager = MintLocaleManager::new();
        
        let locale = LocaleInfo {
            code: "en_US.UTF-8".to_string(),
            language: "English".to_string(),
            territory: "United States".to_string(),
            encoding: "UTF-8".to_string(),
            installed: true,
            is_default: false,
        };
        
        manager.add_locale(locale);
        let stats = manager.get_statistics();
        assert_eq!(stats.total_locales, 1);
        assert_eq!(stats.installed_locales, 1);
    }

    #[test]
    fn test_cannot_remove_default_locale() {
        let mut manager = MintLocaleManager::new();
        
        let mut locale = LocaleInfo {
            code: "en_US.UTF-8".to_string(),
            language: "English".to_string(),
            territory: "United States".to_string(),
            encoding: "UTF-8".to_string(),
            installed: true,
            is_default: true,
        };
        
        manager.add_locale(locale.clone());
        locale.is_default = true;
        
        let result = manager.remove_locale("en_US.UTF-8");
        assert!(result.is_err());
    }

    #[test]
    fn test_input_method_framework_config() {
        let fcitx = InputMethodFramework::Fcitx5;
        let envs = fcitx.get_env_vars();
        assert_eq!(envs.len(), 3);
        assert_eq!(envs[0], ("GTK_IM_MODULE".to_string(), "fcitx".to_string()));
        assert_eq!(envs[1], ("QT_IM_MODULE".to_string(), "fcitx".to_string()));
        assert_eq!(envs[2], ("XMODIFIERS".to_string(), "@im=fcitx".to_string()));

        let ibus = InputMethodFramework::IBus;
        let ibus_envs = ibus.get_env_vars();
        assert_eq!(ibus_envs[0].1, "ibus");
    }

    #[test]
    fn test_missing_component_auditor() {
        let mut auditor = MissingLanguageComponentAuditor::new();
        auditor.register_component(LanguageComponent {
            language_code: "fr".to_string(),
            package_name: "hunspell-fr".to_string(),
            category: LanguageComponentCategory::SpellcheckerHunspell,
            installed: false,
        });
        auditor.register_component(LanguageComponent {
            language_code: "fr".to_string(),
            package_name: "fonts-freefont-ttf".to_string(),
            category: LanguageComponentCategory::FontPackage,
            installed: true,
        });

        let missing = auditor.find_missing_components("fr");
        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0].package_name, "hunspell-fr");
    }

    #[test]
    fn test_locale_format_preview() {
        assert_eq!(LocaleFormatPreview::preview_currency("fr_FR.UTF-8", 1234.56), "1234.56 €");
        assert_eq!(LocaleFormatPreview::preview_currency("en_GB.UTF-8", 1234.56), "£1234.56");
        assert_eq!(LocaleFormatPreview::preview_currency("en_US.UTF-8", 1234.56), "$1234.56");

        assert_eq!(LocaleFormatPreview::preview_date("en_US.UTF-8"), "12/31/2026");
        assert_eq!(LocaleFormatPreview::preview_date("de_DE.UTF-8"), "31.12.2026");

        assert_eq!(LocaleFormatPreview::preview_paper_size("en_US.UTF-8"), "Letter");
        assert_eq!(LocaleFormatPreview::preview_paper_size("fr_FR.UTF-8"), "A4");
    }

    #[test]
    fn test_systemd_localed_config_gen() {
        let mut manager = MintLocaleManager::new();
        manager.set_default_locale("en_US.UTF-8".to_string());
        manager.set_locale_setting(LocaleSettingType::Time, "de_DE.UTF-8".to_string());

        let conf = SystemdLocaledConfigGenerator::generate_etc_locale_conf(&manager);
        assert!(conf.contains("LANG=en_US.UTF-8"));
        assert!(conf.contains("LC_TIME=de_DE.UTF-8"));

        let dbus_args = SystemdLocaledConfigGenerator::generate_dbus_set_locale_args(&manager);
        assert_eq!(dbus_args.len(), 2);
        assert!(dbus_args.contains(&"LANG=en_US.UTF-8".to_string()));
        assert!(dbus_args.contains(&"LC_TIME=de_DE.UTF-8".to_string()));
    }
}
