use core::sync::atomic::{AtomicUsize, Ordering};
use std::collections::BTreeMap;
/// OOP-based Localization Manager for SigmaOS India Stack
/// Implements 22 scheduled languages of India and regional formatting
/// Based on Roadmap Item: India-first architecture
use std::string::{String, ToString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IndianLanguage {
    Assamese,
    Bengali,
    Gujarati,
    Hindi,
    Kannada,
    Kashmiri,
    Konkani,
    Malayalam,
    Manipuri,
    Marathi,
    Nepali,
    Oriya,
    Punjabi,
    Sanskrit,
    Sindhi,
    Tamil,
    Telugu,
    Urdu,
    Bodo,
    Santhali,
    Maithili,
    Dogri,
}

pub trait LocalizationProvider {
    fn language(&self) -> IndianLanguage;
    fn get_string(&self, key: &str) -> Option<String>;
    fn format_currency(&self, amount: f64) -> String;
}

pub struct LocalizationManager {
    current_language: AtomicUsize,
    dictionaries: BTreeMap<IndianLanguage, BTreeMap<String, String>>,
}

impl LocalizationManager {
    pub fn new() -> Self {
        let mut dictionaries = BTreeMap::new();

        // Initialize dictionaries for all 22 languages
        let languages = [
            IndianLanguage::Assamese,
            IndianLanguage::Bengali,
            IndianLanguage::Gujarati,
            IndianLanguage::Hindi,
            IndianLanguage::Kannada,
            IndianLanguage::Kashmiri,
            IndianLanguage::Konkani,
            IndianLanguage::Malayalam,
            IndianLanguage::Manipuri,
            IndianLanguage::Marathi,
            IndianLanguage::Nepali,
            IndianLanguage::Oriya,
            IndianLanguage::Punjabi,
            IndianLanguage::Sanskrit,
            IndianLanguage::Sindhi,
            IndianLanguage::Tamil,
            IndianLanguage::Telugu,
            IndianLanguage::Urdu,
            IndianLanguage::Bodo,
            IndianLanguage::Santhali,
            IndianLanguage::Maithili,
            IndianLanguage::Dogri,
        ];

        for lang in languages {
            dictionaries.insert(lang, BTreeMap::new());
        }

        LocalizationManager {
            current_language: AtomicUsize::new(IndianLanguage::Hindi as usize),
            dictionaries,
        }
    }

    pub fn set_language(&self, lang: IndianLanguage) {
        self.current_language.store(lang as usize, Ordering::SeqCst);
    }

    pub fn get_language(&self) -> IndianLanguage {
        let val = self.current_language.load(Ordering::SeqCst);
        match val {
            0 => IndianLanguage::Assamese,
            1 => IndianLanguage::Bengali,
            2 => IndianLanguage::Gujarati,
            3 => IndianLanguage::Hindi,
            4 => IndianLanguage::Kannada,
            5 => IndianLanguage::Kashmiri,
            6 => IndianLanguage::Konkani,
            7 => IndianLanguage::Malayalam,
            8 => IndianLanguage::Manipuri,
            9 => IndianLanguage::Marathi,
            10 => IndianLanguage::Nepali,
            11 => IndianLanguage::Oriya,
            12 => IndianLanguage::Punjabi,
            13 => IndianLanguage::Sanskrit,
            14 => IndianLanguage::Sindhi,
            15 => IndianLanguage::Tamil,
            16 => IndianLanguage::Telugu,
            17 => IndianLanguage::Urdu,
            18 => IndianLanguage::Bodo,
            19 => IndianLanguage::Santhali,
            20 => IndianLanguage::Maithili,
            21 => IndianLanguage::Dogri,
            _ => IndianLanguage::Hindi,
        }
    }

    pub fn add_string(&mut self, lang: IndianLanguage, key: &str, value: &str) {
        if let Some(dict) = self.dictionaries.get_mut(&lang) {
            dict.insert(key.to_string(), value.to_string());
        }
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        let lang = self.get_language();
        self.dictionaries
            .get(&lang)
            .and_then(|dict| dict.get(key).cloned())
    }

    pub fn format_inr(&self, amount: f64) -> String {
        // Simple Indian numbering system formatting (e.g., 1,00,000)
        let amount_str = format!("{:.2}", amount);
        let parts: Vec<&str> = amount_str.split('.').collect();
        let integer_part = parts[0];
        let decimal_part = parts.get(1).unwrap_or(&"00");

        let mut chars = integer_part.chars().rev().collect::<Vec<char>>();
        let mut formatted = String::new();

        for (i, c) in chars.iter().enumerate() {
            if i == 3 || (i > 3 && (i - 3) % 2 == 0) {
                formatted.push(',');
            }
            formatted.push(*c);
        }

        let formatted_integer = formatted.chars().rev().collect::<String>();
        format!("₹{}.{}", formatted_integer, decimal_part)
    }
}

impl Default for LocalizationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_switching() {
        let mut manager = LocalizationManager::new();
        assert_eq!(manager.get_language(), IndianLanguage::Hindi);

        manager.set_language(IndianLanguage::Tamil);
        assert_eq!(manager.get_language(), IndianLanguage::Tamil);

        manager.set_language(IndianLanguage::Bodo);
        assert_eq!(manager.get_language(), IndianLanguage::Bodo);
    }

    #[test]
    fn test_string_translation() {
        let mut manager = LocalizationManager::new();
        manager.add_string(IndianLanguage::Hindi, "hello", "नमस्ते");
        manager.add_string(IndianLanguage::Tamil, "hello", "வணக்கம்");

        manager.set_language(IndianLanguage::Hindi);
        assert_eq!(manager.get_string("hello").unwrap(), "नमस्ते");

        manager.set_language(IndianLanguage::Tamil);
        assert_eq!(manager.get_string("hello").unwrap(), "வணக்கம்");
    }

    #[test]
    fn test_inr_formatting() {
        let manager = LocalizationManager::new();
        assert_eq!(manager.format_inr(100.0), "₹100.00");
        assert_eq!(manager.format_inr(1000.0), "₹1,000.00");
        assert_eq!(manager.format_inr(100000.0), "₹1,00,000.00");
        assert_eq!(manager.format_inr(12345678.90), "₹1,23,45,678.90");
    }
}
