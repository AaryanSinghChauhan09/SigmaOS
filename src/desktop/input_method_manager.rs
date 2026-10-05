// SigmaOS Desktop Input Method Manager
// Inspired by Linux Mint's input method framework and Omarchy's IME utilities

use std::collections::HashMap;

/// Input method type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMethodType {
    IBus,
    Fcitx,
    Xim,
    None,
}

impl InputMethodType {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputMethodType::IBus => "IBus",
            InputMethodType::Fcitx => "Fcitx",
            InputMethodType::Xim => "XIM",
            InputMethodType::None => "None",
        }
    }
}

/// Input method engine
#[derive(Debug, Clone)]
pub struct InputMethodEngine {
    pub id: String,
    pub name: String,
    pub language: String,
    pub layout: String,
    pub variant: Option<String>,
}

impl InputMethodEngine {
    pub fn new(
        id: String,
        name: String,
        language: String,
        layout: String,
    ) -> Self {
        InputMethodEngine {
            id,
            name,
            language,
            layout,
            variant: None,
        }
    }

    pub fn set_variant(&mut self, variant: String) {
        self.variant = Some(variant);
    }
}

/// Desktop Input Method Manager
pub struct DesktopInputMethodManager {
    engines: HashMap<String, InputMethodEngine>,
    current_engine_id: Option<String>,
    input_method_type: InputMethodType,
    next_engine_id: u32,
}

impl DesktopInputMethodManager {
    pub fn new() -> Self {
        let mut manager = DesktopInputMethodManager {
            engines: HashMap::new(),
            current_engine_id: None,
            input_method_type: InputMethodType::IBus,
            next_engine_id: 1,
        };

        manager.add_default_engines();
        manager
    }

    fn add_default_engines(&mut self) {
        // US English
        let us_id = format!("engine_{}", self.next_engine_id);
        self.next_engine_id += 1;
        let us = InputMethodEngine::new(
            us_id.clone(),
            "English (US)".to_string(),
            "en".to_string(),
            "us".to_string(),
        );
        self.engines.insert(us_id.clone(), us);
        self.current_engine_id = Some(us_id);

        // UK English
        let uk_id = format!("engine_{}", self.next_engine_id);
        self.next_engine_id += 1;
        let uk = InputMethodEngine::new(
            uk_id.clone(),
            "English (UK)".to_string(),
            "en".to_string(),
            "gb".to_string(),
        );
        self.engines.insert(uk_id, uk);

        // German
        let de_id = format!("engine_{}", self.next_engine_id);
        self.next_engine_id += 1;
        let de = InputMethodEngine::new(
            de_id.clone(),
            "German".to_string(),
            "de".to_string(),
            "de".to_string(),
        );
        self.engines.insert(de_id, de);

        // French
        let fr_id = format!("engine_{}", self.next_engine_id);
        self.next_engine_id += 1;
        let fr = InputMethodEngine::new(
            fr_id.clone(),
            "French".to_string(),
            "fr".to_string(),
            "fr".to_string(),
        );
        self.engines.insert(fr_id, fr);

        // Japanese
        let ja_id = format!("engine_{}", self.next_engine_id);
        self.next_engine_id += 1;
        let mut ja = InputMethodEngine::new(
            ja_id.clone(),
            "Japanese".to_string(),
            "ja".to_string(),
            "jp".to_string(),
        );
        ja.set_variant("kana".to_string());
        self.engines.insert(ja_id, ja);
    }

    pub fn set_input_method_type(&mut self, im_type: InputMethodType) {
        self.input_method_type = im_type;
    }

    pub fn add_engine(
        &mut self,
        name: String,
        language: String,
        layout: String,
    ) -> String {
        let id = format!("engine_{}", self.next_engine_id);
        self.next_engine_id += 1;

        let engine = InputMethodEngine::new(id.clone(), name, language, layout);
        self.engines.insert(id.clone(), engine);
        id
    }

    pub fn remove_engine(&mut self, id: &str) -> bool {
        if self.current_engine_id.as_ref() == Some(&id.to_string()) {
            return false;
        }
        self.engines.remove(id).is_some()
    }

    pub fn set_current_engine(&mut self, id: &str) -> bool {
        if self.engines.contains_key(id) {
            self.current_engine_id = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_engine(&self, id: &str) -> Option<&InputMethodEngine> {
        self.engines.get(id)
    }

    pub fn get_engines(&self) -> Vec<&InputMethodEngine> {
        self.engines.values().collect()
    }

    pub fn get_engines_by_language(&self, language: &str) -> Vec<&InputMethodEngine> {
        self.engines
            .values()
            .filter(|e| e.language == language)
            .collect()
    }

    pub fn get_current_engine(&self) -> Option<&InputMethodEngine> {
        if let Some(current_id) = &self.current_engine_id {
            self.engines.get(current_id)
        } else {
            None
        }
    }

    pub fn get_input_method_type(&self) -> InputMethodType {
        self.input_method_type
    }

    pub fn get_statistics(&self) -> InputMethodManagerStatistics {
        InputMethodManagerStatistics {
            total_engines: self.engines.len(),
            current_engine: self.current_engine_id.is_some(),
            input_method_type: self.input_method_type,
        }
    }
}

impl Default for DesktopInputMethodManager {
    fn default() -> Self {
        Self::new()
    }
}

/// InputMethodManagerStatistics
#[derive(Debug, Clone, Copy)]
pub struct InputMethodManagerStatistics {
    pub total_engines: usize,
    pub current_engine: bool,
    pub input_method_type: InputMethodType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_method_manager_initialization() {
        let manager = DesktopInputMethodManager::new();
        assert_eq!(manager.get_engines().len(), 5);
        assert!(manager.get_current_engine().is_some());
    }

    #[test]
    fn test_add_engine() {
        let mut manager = DesktopInputMethodManager::new();
        let id = manager.add_engine("Spanish".to_string(), "es".to_string(), "es".to_string());
        assert!(manager.get_engine(&id).is_some());
        assert_eq!(manager.get_engines().len(), 6);
    }

    #[test]
    fn test_remove_engine() {
        let mut manager = DesktopInputMethodManager::new();
        let id = manager.add_engine("Spanish".to_string(), "es".to_string(), "es".to_string());
        assert!(manager.remove_engine(&id));
        assert_eq!(manager.get_engines().len(), 5);
    }

    #[test]
    fn test_set_current_engine() {
        let mut manager = DesktopInputMethodManager::new();
        let id = manager.add_engine("Spanish".to_string(), "es".to_string(), "es".to_string());
        assert!(manager.set_current_engine(&id));
        assert_eq!(manager.get_current_engine().unwrap().id, id);
    }

    #[test]
    fn test_set_input_method_type() {
        let mut manager = DesktopInputMethodManager::new();
        manager.set_input_method_type(InputMethodType::Fcitx);
        assert_eq!(manager.get_input_method_type(), InputMethodType::Fcitx);
    }

    #[test]
    fn test_get_engines_by_language() {
        let manager = DesktopInputMethodManager::new();
        let english = manager.get_engines_by_language("en");
        assert_eq!(english.len(), 2);
    }

    #[test]
    fn test_statistics() {
        let manager = DesktopInputMethodManager::new();
        let stats = manager.get_statistics();
        assert_eq!(stats.total_engines, 5);
    }
}
