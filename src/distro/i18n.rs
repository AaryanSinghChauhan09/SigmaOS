#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

use crate::klib::BTreeMap;

/// Represents a Language Pack for translating system messages.
#[derive(Debug, Clone)]
pub struct LanguagePack {
    pub language_code: String,
    pub name: String,
    pub translations: BTreeMap<String, String>,
}

impl LanguagePack {
    pub fn new(language_code: &str, name: &str) -> Self {
        Self {
            language_code: language_code.to_string(),
            name: name.to_string(),
            translations: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        self.translations.insert(key.to_string(), value.to_string());
    }

    pub fn translate(&self, key: &str) -> Option<&str> {
        self.translations.get(key).map(|s: &String| s.as_str())
    }

    /// Formats the translation by replacing placeholders like `{0}`, `{1}` with arguments.
    pub fn translate_with_args(&self, key: &str, args: &[&str]) -> Option<String> {
        let template = self.translate(key)?;
        let mut result = template.to_string();
        for (i, arg) in args.iter().enumerate() {
            let placeholder = format!("{{{}}}", i);
            result = result.replace(&placeholder, arg);
        }
        Some(result)
    }

    /// Handles basic English/Universal plural rules (one, other) or custom forms.
    pub fn translate_plural(&self, key: &str, count: usize) -> Option<&str> {
        let plural_key = if count == 1 {
            format!("{}_one", key)
        } else {
            format!("{}_other", key)
        };
        self.translate(&plural_key).or_else(|| self.translate(key))
    }
}

/// Represents an Input Method Engine (IME) candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImeCandidate {
    pub text: String,
    pub description: String,
}

/// Input Method Engine (IME) for handling complex scripts (e.g., Pinyin, Romaji, Indic).
#[derive(Debug, Clone)]
pub struct InputMethodEngine {
    pub script_name: String,
    pub dictionary: BTreeMap<String, Vec<ImeCandidate>>,
}

impl InputMethodEngine {
    pub fn new(script_name: &str) -> Self {
        Self {
            script_name: script_name.to_string(),
            dictionary: BTreeMap::new(),
        }
    }

    pub fn register_conversion(&mut self, input: &str, candidates: Vec<ImeCandidate>) {
        self.dictionary.insert(input.to_string(), candidates);
    }

    pub fn get_candidates(&self, input: &str) -> Vec<ImeCandidate> {
        self.dictionary.get(input).cloned().unwrap_or_default()
    }

    /// Automatically converts input if a single perfect candidate exists, or returns the original.
    pub fn convert_auto(&self, input: &str) -> String {
        if let Some(candidates) = self.dictionary.get(input) {
            let candidates: &Vec<ImeCandidate> = candidates;
            if !candidates.is_empty() {
                return candidates[0].text.clone();
            }
        }
        input.to_string()
    }
}

/// Regional formatting settings (currency, date/time, numbers, etc.).
#[derive(Debug, Clone)]
pub struct RegionalSettings {
    pub locale_id: String,
    pub currency_symbol: String,
    pub currency_code: String,
    pub date_format: String, // e.g. "DD/MM/YYYY"
    pub decimal_separator: char,
    pub thousands_separator: char,
    pub timezone_offset_secs: i32,
}

impl RegionalSettings {
    pub fn new_india() -> Self {
        Self {
            locale_id: "en_IN".to_string(),
            currency_symbol: "₹".to_string(),
            currency_code: "INR".to_string(),
            date_format: "DD/MM/YYYY".to_string(),
            decimal_separator: '.',
            thousands_separator: ',',
            timezone_offset_secs: 19800, // UTC+5:30
        }
    }

    pub fn new_us() -> Self {
        Self {
            locale_id: "en_US".to_string(),
            currency_symbol: "$".to_string(),
            currency_code: "USD".to_string(),
            date_format: "MM/DD/YYYY".to_string(),
            decimal_separator: '.',
            thousands_separator: ',',
            timezone_offset_secs: -18000, // UTC-5:00 (EST)
        }
    }

    pub fn format_currency(&self, amount: f64) -> String {
        let int_part = amount.trunc() as i64;
        let frac_part = ((amount.fract().abs() * 100.0).round()) as i64;

        let mut int_str = int_part.to_string();
        if self.thousands_separator != '\0' && int_str.len() > 3 {
            let mut formatted = String::new();
            let chars: Vec<char> = int_str.chars().collect();
            let len = chars.len();
            for (i, ch) in chars.iter().enumerate() {
                formatted.push(*ch);
                if i + 1 < len && (len - 1 - i) % 3 == 0 {
                    formatted.push(self.thousands_separator);
                }
            }
            int_str = formatted;
        }

        format!(
            "{}{}{}{:02}",
            self.currency_symbol, int_str, self.decimal_separator, frac_part
        )
    }

    pub fn format_date(&self, day: u32, month: u32, year: u32) -> String {
        let d = format!("{:02}", day);
        let m = format!("{:02}", month);
        let y = format!("{:04}", year);
        self.date_format
            .replace("DD", &d)
            .replace("MM", &m)
            .replace("YYYY", &y)
    }
}

/// Global coordinator for Internationalization and Localization settings.
/// Inspired by GNU gettext & Linux/BSD distros (Ubuntu, Fedora, FreeBSD) multi-tier translation resolution.
#[derive(Debug, Clone)]
pub struct LocaleManager {
    pub current_locale: String,
    pub fallback_locale: String,
    pub language_packs: BTreeMap<String, LanguagePack>,
    pub imes: BTreeMap<String, InputMethodEngine>,
    pub regional_settings: BTreeMap<String, RegionalSettings>,
}

impl LocaleManager {
    pub fn new(default_locale: &str) -> Self {
        Self {
            current_locale: default_locale.to_string(),
            fallback_locale: "en_US".to_string(),
            language_packs: BTreeMap::new(),
            imes: BTreeMap::new(),
            regional_settings: BTreeMap::new(),
        }
    }

    pub fn register_language_pack(&mut self, pack: LanguagePack) {
        self.language_packs.insert(pack.language_code.clone(), pack);
    }

    pub fn register_ime(&mut self, ime: InputMethodEngine) {
        self.imes.insert(ime.script_name.clone(), ime);
    }

    pub fn register_regional_settings(&mut self, settings: RegionalSettings) {
        self.regional_settings
            .insert(settings.locale_id.clone(), settings);
    }

    pub fn set_locale(&mut self, locale: &str) -> Result<(), &'static str> {
        let locale_str = locale.to_string();
        if self.language_packs.contains_key(&locale_str)
            || self.regional_settings.contains_key(&locale_str)
        {
            self.current_locale = locale.to_string();
            Ok(())
        } else {
            Err("Locale not registered")
        }
    }

    /// GNU gettext / Linux distro inspired multi-tier translation resolution chain:
    /// 1. Current Full Locale (e.g. "fr_CA.UTF-8" or "fr_CA")
    /// 2. Base Language Code (e.g. "fr")
    /// 3. Fallback Locale (e.g. "en_US" or "en")
    /// 4. Raw Message Key
    pub fn translate(&self, key: &str) -> String {
        // Strip encoding suffix if present (e.g., "fr_CA.UTF-8" -> "fr_CA")
        let clean_locale = self
            .current_locale
            .split('.')
            .next()
            .unwrap_or(&self.current_locale);

        // Tier 1: Try current exact/cleaned locale
        if let Some(pack) = self
            .language_packs
            .get(clean_locale)
            .or_else(|| self.language_packs.get(&self.current_locale))
        {
            let pack: &LanguagePack = pack;
            if let Some(translation) = pack.translate(key) {
                let translation: &str = translation;
                return translation.to_string();
            }
        }

        // Tier 2: Try base language code (e.g., "fr_CA" -> "fr")
        if let Some(base_lang) = clean_locale.split('_').next() {
            if base_lang != clean_locale {
                if let Some(pack) = self.language_packs.get(base_lang) {
                    let pack: &LanguagePack = pack;
                    if let Some(translation) = pack.translate(key) {
                        let translation: &str = translation;
                        return translation.to_string();
                    }
                }
            }
        }

        // Tier 3: Try fallback locale (e.g., "en_US" or base "en")
        if self.current_locale != self.fallback_locale && clean_locale != self.fallback_locale {
            if let Some(pack) = self.language_packs.get(&self.fallback_locale) {
                let pack: &LanguagePack = pack;
                if let Some(translation) = pack.translate(key) {
                    let translation: &str = translation;
                    return translation.to_string();
                }
            }
            let fallback_clean = self
                .fallback_locale
                .split('.')
                .next()
                .unwrap_or(&self.fallback_locale);
            if let Some(fallback_base) = fallback_clean.split('_').next() {
                if let Some(pack) = self.language_packs.get(fallback_base) {
                    let pack: &LanguagePack = pack;
                    if let Some(translation) = pack.translate(key) {
                        let translation: &str = translation;
                        return translation.to_string();
                    }
                }
            }
        }

        // Tier 4: Fallback to raw key
        key.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_pack_translation() {
        let mut pack = LanguagePack::new("en", "English");
        pack.insert("welcome", "Welcome back, {0}!");
        pack.insert("items_one", "You have 1 item");
        pack.insert("items_other", "You have {0} items");

        assert_eq!(pack.translate("welcome"), Some("Welcome back, {0}!"));
        assert_eq!(
            pack.translate_with_args("welcome", &["Alice"]),
            Some("Welcome back, Alice!".to_string())
        );
        assert_eq!(pack.translate_plural("items", 1), Some("You have 1 item"));
        assert_eq!(
            pack.translate_plural("items", 5),
            Some("You have {0} items")
        );
    }

    #[test]
    fn test_input_method_engine() {
        let mut ime = InputMethodEngine::new("Pinyin");
        ime.register_conversion(
            "nihao",
            vec![
                ImeCandidate {
                    text: "你好".to_string(),
                    description: "hello".to_string(),
                },
                ImeCandidate {
                    text: "泥好".to_string(),
                    description: "mud good".to_string(),
                },
            ],
        );

        let candidates = ime.get_candidates("nihao");
        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates[0].text, "你好");

        assert_eq!(ime.convert_auto("nihao"), "你好");
        assert_eq!(ime.convert_auto("unknown"), "unknown");
    }

    #[test]
    fn test_regional_settings() {
        let india = RegionalSettings::new_india();
        assert_eq!(india.format_currency(1234.56), "₹1,234.56");
        assert_eq!(india.format_date(15, 8, 2024), "15/08/2024");

        let us = RegionalSettings::new_us();
        assert_eq!(us.format_currency(1234.56), "$1,234.56");
        assert_eq!(us.format_date(15, 8, 2024), "08/15/2024");
    }

    #[test]
    fn test_locale_manager() {
        let mut lm = LocaleManager::new("en_US");
        let mut pack = LanguagePack::new("en_US", "English (US)");
        pack.insert("hello", "Hello");
        lm.register_language_pack(pack);

        let mut pack_in = LanguagePack::new("en_IN", "English (India)");
        pack_in.insert("hello", "Namaste");
        lm.register_language_pack(pack_in);

        assert_eq!(lm.translate("hello"), "Hello");
        assert!(lm.set_locale("en_IN").is_ok());
        assert_eq!(lm.translate("hello"), "Namaste");
    }

    #[test]
    fn test_locale_manager_multitier_fallback() {
        let mut lm = LocaleManager::new("fr_CA.UTF-8");

        let mut pack_en = LanguagePack::new("en_US", "English (US)");
        pack_en.insert("yes", "Yes");
        pack_en.insert("no", "No");
        pack_en.insert("cancel", "Cancel");
        lm.register_language_pack(pack_en);

        let mut pack_fr = LanguagePack::new("fr", "French");
        pack_fr.insert("yes", "Oui");
        pack_fr.insert("no", "Non");
        lm.register_language_pack(pack_fr);

        let mut pack_fr_ca = LanguagePack::new("fr_CA", "French (Canada)");
        pack_fr_ca.insert("yes", "Oui (CA)");
        lm.register_language_pack(pack_fr_ca);

        // Tier 1: "yes" exists in exact/clean "fr_CA"
        assert_eq!(lm.translate("yes"), "Oui (CA)");
        // Tier 2: "no" falls back from "fr_CA" to base language "fr"
        assert_eq!(lm.translate("no"), "Non");
        // Tier 3: "cancel" falls back to fallback locale "en_US"
        assert_eq!(lm.translate("cancel"), "Cancel");
        // Tier 4: Unknown key returns raw key
        assert_eq!(lm.translate("unknown_key"), "unknown_key");
    }
}
