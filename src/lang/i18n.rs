//! Internationalization (i18n) and localization (l10n) for SigmaOS.
//!
//! Covers locale management, timezone selection, keyboard layouts,
//! RTL language support, and translation catalog infrastructure.

#![allow(dead_code)]

/// A BCP-47 locale string with metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Locale {
    pub code: &'static str,
    pub language: &'static str,
    pub territory: &'static str,
    pub encoding: &'static str,
    pub is_rtl: bool,
}

impl Locale {
    pub fn display_name(&self) -> String {
        format!("{} ({})", self.language, self.territory)
    }
}

/// All locales supported in the V1 SigmaOS release.
pub const SUPPORTED_LOCALES: &[Locale] = &[
    Locale { code: "en_US.UTF-8", language: "English",    territory: "United States", encoding: "UTF-8", is_rtl: false },
    Locale { code: "en_GB.UTF-8", language: "English",    territory: "United Kingdom", encoding: "UTF-8", is_rtl: false },
    Locale { code: "de_DE.UTF-8", language: "German",     territory: "Germany",        encoding: "UTF-8", is_rtl: false },
    Locale { code: "fr_FR.UTF-8", language: "French",     territory: "France",         encoding: "UTF-8", is_rtl: false },
    Locale { code: "es_ES.UTF-8", language: "Spanish",    territory: "Spain",          encoding: "UTF-8", is_rtl: false },
    Locale { code: "pt_BR.UTF-8", language: "Portuguese", territory: "Brazil",         encoding: "UTF-8", is_rtl: false },
    Locale { code: "zh_CN.UTF-8", language: "Chinese",    territory: "China",          encoding: "UTF-8", is_rtl: false },
    Locale { code: "ja_JP.UTF-8", language: "Japanese",   territory: "Japan",          encoding: "UTF-8", is_rtl: false },
    Locale { code: "ko_KR.UTF-8", language: "Korean",     territory: "South Korea",    encoding: "UTF-8", is_rtl: false },
    Locale { code: "hi_IN.UTF-8", language: "Hindi",      territory: "India",          encoding: "UTF-8", is_rtl: false },
    Locale { code: "ar_SA.UTF-8", language: "Arabic",     territory: "Saudi Arabia",   encoding: "UTF-8", is_rtl: true  },
    Locale { code: "he_IL.UTF-8", language: "Hebrew",     territory: "Israel",         encoding: "UTF-8", is_rtl: true  },
    Locale { code: "ru_RU.UTF-8", language: "Russian",    territory: "Russia",         encoding: "UTF-8", is_rtl: false },
    Locale { code: "tr_TR.UTF-8", language: "Turkish",    territory: "Turkey",         encoding: "UTF-8", is_rtl: false },
    Locale { code: "pl_PL.UTF-8", language: "Polish",     territory: "Poland",         encoding: "UTF-8", is_rtl: false },
    Locale { code: "it_IT.UTF-8", language: "Italian",    territory: "Italy",          encoding: "UTF-8", is_rtl: false },
    Locale { code: "nl_NL.UTF-8", language: "Dutch",      territory: "Netherlands",    encoding: "UTF-8", is_rtl: false },
    Locale { code: "sv_SE.UTF-8", language: "Swedish",    territory: "Sweden",         encoding: "UTF-8", is_rtl: false },
    Locale { code: "fa_IR.UTF-8", language: "Persian",    territory: "Iran",           encoding: "UTF-8", is_rtl: true  },
    Locale { code: "id_ID.UTF-8", language: "Indonesian", territory: "Indonesia",      encoding: "UTF-8", is_rtl: false },
];

/// Keyboard layout descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyboardLayout {
    pub xkb_code: &'static str,
    pub name: &'static str,
    pub variant: Option<&'static str>,
}

impl KeyboardLayout {
    pub fn xkb_string(&self) -> String {
        match self.variant {
            Some(v) => format!("{}({})", self.xkb_code, v),
            None => self.xkb_code.to_string(),
        }
    }
}

pub const KEYBOARD_LAYOUTS: &[KeyboardLayout] = &[
    KeyboardLayout { xkb_code: "us",    name: "English (US)",    variant: None },
    KeyboardLayout { xkb_code: "gb",    name: "English (UK)",    variant: None },
    KeyboardLayout { xkb_code: "de",    name: "German",          variant: None },
    KeyboardLayout { xkb_code: "fr",    name: "French",          variant: None },
    KeyboardLayout { xkb_code: "es",    name: "Spanish",         variant: None },
    KeyboardLayout { xkb_code: "br",    name: "Portuguese (BR)", variant: None },
    KeyboardLayout { xkb_code: "cn",    name: "Chinese (Pinyin)",variant: Some("pinyin") },
    KeyboardLayout { xkb_code: "jp",    name: "Japanese",        variant: None },
    KeyboardLayout { xkb_code: "kr",    name: "Korean",          variant: None },
    KeyboardLayout { xkb_code: "in",    name: "Hindi (Devanagari)",variant: Some("deva") },
    KeyboardLayout { xkb_code: "ara",   name: "Arabic",          variant: None },
    KeyboardLayout { xkb_code: "il",    name: "Hebrew",          variant: None },
    KeyboardLayout { xkb_code: "ru",    name: "Russian",         variant: None },
    KeyboardLayout { xkb_code: "tr",    name: "Turkish",         variant: None },
    KeyboardLayout { xkb_code: "pl",    name: "Polish",          variant: None },
    KeyboardLayout { xkb_code: "it",    name: "Italian",         variant: None },
    KeyboardLayout { xkb_code: "dvp",   name: "Dvorak",          variant: Some("dvp") },
    KeyboardLayout { xkb_code: "colemak", name: "Colemak",       variant: Some("colemak") },
];

/// Locale manager: resolves and applies locale settings.
pub struct LocaleManager {
    current: &'static Locale,
    keyboard: &'static KeyboardLayout,
    timezone: String,
}

impl LocaleManager {
    pub fn new() -> Self {
        Self {
            current: &SUPPORTED_LOCALES[0],   // en_US.UTF-8
            keyboard: &KEYBOARD_LAYOUTS[0],    // us
            timezone: "UTC".to_string(),
        }
    }

    pub fn set_locale(&mut self, code: &str) -> bool {
        if let Some(l) = SUPPORTED_LOCALES.iter().find(|l| l.code == code) {
            self.current = l;
            true
        } else {
            false
        }
    }

    pub fn set_keyboard(&mut self, xkb: &str) -> bool {
        if let Some(k) = KEYBOARD_LAYOUTS.iter().find(|k| k.xkb_code == xkb) {
            self.keyboard = k;
            true
        } else {
            false
        }
    }

    pub fn set_timezone(&mut self, tz: impl Into<String>) { self.timezone = tz.into(); }

    pub fn current_locale(&self) -> &Locale { self.current }
    pub fn current_keyboard(&self) -> &KeyboardLayout { self.keyboard }
    pub fn timezone(&self) -> &str { &self.timezone }
    pub fn is_rtl(&self) -> bool { self.current.is_rtl }

    /// Generate /etc/locale.conf content.
    pub fn render_locale_conf(&self) -> String {
        format!("LANG={}\nLC_ALL={}\n", self.current.code, self.current.code)
    }

    /// Generate /etc/vconsole.conf content.
    pub fn render_vconsole_conf(&self) -> String {
        format!("KEYMAP={}\n", self.current_keyboard().xkb_code)
    }
}

impl Default for LocaleManager { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_locales_at_least_20() { assert!(SUPPORTED_LOCALES.len() >= 20); }

    #[test]
    fn test_arabic_is_rtl() {
        let ar = SUPPORTED_LOCALES.iter().find(|l| l.code == "ar_SA.UTF-8").unwrap();
        assert!(ar.is_rtl);
    }

    #[test]
    fn test_english_not_rtl() {
        let en = &SUPPORTED_LOCALES[0];
        assert!(!en.is_rtl);
    }

    #[test]
    fn test_set_locale_valid() {
        let mut mgr = LocaleManager::new();
        assert!(mgr.set_locale("de_DE.UTF-8"));
        assert_eq!(mgr.current_locale().code, "de_DE.UTF-8");
    }

    #[test]
    fn test_set_locale_invalid_returns_false() {
        let mut mgr = LocaleManager::new();
        assert!(!mgr.set_locale("xx_XX.UTF-8"));
    }

    #[test]
    fn test_locale_conf_contains_lang() {
        let mgr = LocaleManager::new();
        let conf = mgr.render_locale_conf();
        assert!(conf.contains("LANG=en_US.UTF-8"));
    }

    #[test]
    fn test_keyboard_xkb_string_with_variant() {
        let dvorak = KEYBOARD_LAYOUTS.iter().find(|k| k.xkb_code == "dvp").unwrap();
        assert_eq!(dvorak.xkb_string(), "dvp(dvp)");
    }

    #[test]
    fn test_keyboard_xkb_string_no_variant() {
        let us = &KEYBOARD_LAYOUTS[0];
        assert_eq!(us.xkb_string(), "us");
    }
}
