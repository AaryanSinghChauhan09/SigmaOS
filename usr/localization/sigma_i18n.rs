// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/localization/sigma_i18n.rs — Sigma Internationalization (i18n)
//
// Implements multilingual support for Hindi, Gujarati, Tamil, and Bengali
// to make SigmaOS accessible to Indian users in their native languages.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Language Types ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    Hindi,
    Gujarati,
    Tamil,
    Bengali,
}

#[derive(Debug, Clone)]
pub struct Translation {
    pub key: String,
    pub translations: HashMap<Language, String>,
}

// ─── Internationalization Manager ───────────────────────────────────────────────

pub struct I18nManager {
    pub current_language: Language,
    pub translations: HashMap<String, Translation>,
}

impl I18nManager {
    pub fn new() -> Self {
        let mut manager = I18nManager {
            current_language: Language::English,
            translations: HashMap::new(),
        };
        
        manager.init_translations();
        manager
    }

    /// Initialize translations for all supported languages
    fn init_translations(&mut self) {
        // Common UI elements
        self.add_translation("file", vec![
            (Language::English, "File".to_string()),
            (Language::Hindi, "फ़ाइल".to_string()),
            (Language::Gujarati, "ફાઇલ".to_string()),
            (Language::Tamil, "கோப்பு".to_string()),
            (Language::Bengali, "ফাইল".to_string()),
        ]);

        self.add_translation("edit", vec![
            (Language::English, "Edit".to_string()),
            (Language::Hindi, "संपादित करें".to_string()),
            (Language::Gujarati, "સંપાદિત કરો".to_string()),
            (Language::Tamil, "திருத்து".to_string()),
            (Language::Bengali, "সম্পাদনা".to_string()),
        ]);

        self.add_translation("view", vec![
            (Language::English, "View".to_string()),
            (Language::Hindi, "देखें".to_string()),
            (Language::Gujarati, "જુઓ".to_string()),
            (Language::Tamil, "காண்க".to_string()),
            (Language::Bengali, "দেখুন".to_string()),
        ]);

        self.add_translation("settings", vec![
            (Language::English, "Settings".to_string()),
            (Language::Hindi, "सेटिंग्स".to_string()),
            (Language::Gujarati, "સેટિંગ્સ".to_string()),
            (Language::Tamil, "அமைப்புகள்".to_string()),
            (Language::Bengali, "সেটিংস".to_string()),
        ]);

        self.add_translation("save", vec![
            (Language::English, "Save".to_string()),
            (Language::Hindi, "सहेजें".to_string()),
            (Language::Gujarati, "સાચવો".to_string()),
            (Language::Tamil, "சேமி".to_string()),
            (Language::Bengali, "সংরক্ষণ করুন".to_string()),
        ]);

        self.add_translation("open", vec![
            (Language::English, "Open".to_string()),
            (Language::Hindi, "खोलें".to_string()),
            (Language::Gujarati, "ખોલો".to_string()),
            (Language::Tamil, "திறக்க".to_string()),
            (Language::Bengali, "খুলুন".to_string()),
        ]);

        self.add_translation("close", vec![
            (Language::English, "Close".to_string()),
            (Language::Hindi, "बंद करें".to_string()),
            (Language::Gujarati, "બંધ કરો".to_string()),
            (Language::Tamil, "மூடு".to_string()),
            (Language::Bengali, "বন্ধ করুন".to_string()),
        ]);

        self.add_translation("cancel", vec![
            (Language::English, "Cancel".to_string()),
            (Language::Hindi, "रद्द करें".to_string()),
            (Language::Gujarati, "રદ કરો".to_string()),
            (Language::Tamil, "ரத்து".to_string()),
            (Language::Bengali, "বাতিল করুন".to_string()),
        ]);

        self.add_translation("ok", vec![
            (Language::English, "OK".to_string()),
            (Language::Hindi, "ठीक है".to_string()),
            (Language::Gujarati, "ઠીક છે".to_string()),
            (Language::Tamil, "சரி".to_string()),
            (Language::Bengali, "ঠিক আছে".to_string()),
        ]);

        self.add_translation("yes", vec![
            (Language::English, "Yes".to_string()),
            (Language::Hindi, "हाँ".to_string()),
            (Language::Gujarati, "હા".to_string()),
            (Language::Tamil, "ஆம்".to_string()),
            (Language::Bengali, "হ্যাঁ".to_string()),
        ]);

        self.add_translation("no", vec![
            (Language::English, "No".to_string()),
            (Language::Hindi, "नहीं".to_string()),
            (Language::Gujarati, "ના".to_string()),
            (Language::Tamil, "இல்லை".to_string()),
            (Language::Bengali, "না".to_string()),
        ]);

        // Application-specific strings
        self.add_translation("welcome", vec![
            (Language::English, "Welcome to SigmaOS".to_string()),
            (Language::Hindi, "SigmaOS में आपका स्वागत है".to_string()),
            (Language::Gujarati, "SigmaOS માં તમારું સ્વાગત છે".to_string()),
            (Language::Tamil, "SigmaOS க்கு வரவேற்பு".to_string()),
            (Language::Bengali, "SigmaOS-এ স্বাগতম".to_string()),
        ]);

        self.add_translation("language_changed", vec![
            (Language::English, "Language changed successfully".to_string()),
            (Language::Hindi, "भाषा सफलतापूर्वक बदल दी गई".to_string()),
            (Language::Gujarati, "ભાષા સફળતાપૂર્વક બદલાઈ".to_string()),
            (Language::Tamil, "மொழி வெற்றிகரமாக மாற்றப்பட்டது".to_string()),
            (Language::Bengali, "ভাষা সফলভাবে পরিবর্তিত হয়েছে".to_string()),
        ]);

        self.add_translation("select_language", vec![
            (Language::English, "Select Language".to_string()),
            (Language::Hindi, "भाषा चुनें".to_string()),
            (Language::Gujarati, "ભાષા પસંદ કરો".to_string()),
            (Language::Tamil, "மொழியைத் தேர்ந்தெடுக்கவும்".to_string()),
            (Language::Bengali, "ভাষা নির্বাচন করুন".to_string()),
        ]);
    }

    /// Add translation for a key
    fn add_translation(&mut self, key: String, translations: Vec<(Language, String)>) {
        let mut trans_map = HashMap::new();
        for (lang, text) in translations {
            trans_map.insert(lang, text);
        }
        
        self.translations.insert(key, Translation {
            key,
            translations: trans_map,
        });
    }

    /// Get translation for key in current language
    pub fn translate(&self, key: &str) -> String {
        if let Some(translation) = self.translations.get(key) {
            if let Some(text) = translation.translations.get(&self.current_language) {
                text.clone()
            } else {
                // Fallback to English if translation not available
                translation.translations.get(&Language::English)
                    .cloned()
                    .unwrap_or_else(|| key.to_string())
            }
        } else {
            key.to_string()
        }
    }

    /// Set current language
    pub fn set_language(&mut self, language: Language) {
        self.current_language = language;
    }

    /// Get current language
    pub fn get_language(&self) -> Language {
        self.current_language
    }

    /// Get all supported languages
    pub fn get_supported_languages(&self) -> Vec<Language> {
        vec![
            Language::English,
            Language::Hindi,
            Language::Gujarati,
            Language::Tamil,
            Language::Bengali,
        ]
    }

    /// Get language name
    pub fn get_language_name(&self, language: Language) -> &str {
        match language {
            Language::English => "English",
            Language::Hindi => "हिन्दी (Hindi)",
            Language::Gujarati => "ગુજરાતી (Gujarati)",
            Language::Tamil => "தமிழ் (Tamil)",
            Language::Bengali => "বাংলা (Bengali)",
        }
    }

    /// Add custom translation
    pub fn add_custom_translation(&mut self, key: String, language: Language, text: String) {
        if let Some(translation) = self.translations.get_mut(&key) {
            translation.translations.insert(language, text);
        } else {
            let mut trans_map = HashMap::new();
            trans_map.insert(language, text);
            self.translations.insert(key, Translation {
                key,
                translations: trans_map,
            });
        }
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut i18n = I18nManager::new();
    
    println!("{}", i18n.translate("welcome"));
    println!();
    
    loop {
        println!("--- {} ---", i18n.translate("select_language"));
        for lang in i18n.get_supported_languages() {
            let marker = if i18n.get_language() == lang { " >" } else { "  " };
            println!("{}{}", marker, i18n.get_language_name(lang));
        }
        
        println!("\nCommands: set <language>, translate <key>, add <key> <lang> <text>, quit");
        println!("Languages: english, hindi, gujarati, tamil, bengali");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "set" => {
                if let Some(arg) = parts.get(1) {
                    let language = match *arg {
                        "english" => Language::English,
                        "hindi" => Language::Hindi,
                        "gujarati" => Language::Gujarati,
                        "tamil" => Language::Tamil,
                        "bengali" => Language::Bengali,
                        _ => {
                            println!("Unknown language");
                            continue;
                        }
                    };
                    i18n.set_language(language);
                    println!("{}", i18n.translate("language_changed"));
                }
            }
            "translate" => {
                if let Some(arg) = parts.get(1) {
                    println!("{}: {}", arg, i18n.translate(arg));
                }
            }
            "add" => {
                if parts.len() >= 4 {
                    let key = parts[1].to_string();
                    let language = match parts[2] {
                        "english" => Language::English,
                        "hindi" => Language::Hindi,
                        "gujarati" => Language::Gujarati,
                        "tamil" => Language::Tamil,
                        "bengali" => Language::Bengali,
                        _ => {
                            println!("Unknown language");
                            continue;
                        }
                    };
                    let text = parts[3..].join(" ");
                    i18n.add_custom_translation(key, language, text);
                    println!("Translation added");
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
