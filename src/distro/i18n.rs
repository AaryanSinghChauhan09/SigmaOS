//! Localization and Internationalization (i18n) Engine for SigmaOS
//! Implements high-performance, zero-allocation localized message routing.

use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    Hindi,
    Bengali,
    Tamil,
    Telugu,
    Marathi,
    Gujarati,
    Kannada,
}

pub struct LocalizationEngine {
    pub current_language: Language,
}

impl LocalizationEngine {
    pub const fn new(lang: Language) -> Self {
        Self {
            current_language: lang,
        }
    }

    pub fn get_translation(&self, key: &str) -> &'static str {
        match self.current_language {
            Language::Hindi => match key {
                "welcome" => "सिग्मा ओएस में आपका स्वागत है।",
                "installer_title" => "सिग्मा ओएस स्थापना प्रबंधक",
                _ => "सिग्मा ओएस",
            },
            Language::English => match key {
                "welcome" => "Welcome to SigmaOS.",
                "installer_title" => "SigmaOS Installer Manager",
                _ => "SigmaOS",
            },
            _ => "SigmaOS",
        }
    }
}
