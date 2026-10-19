//! SigmaOS Emoji Picker
//!
//! Inspired by: Omarchy's emoji picker (SUPER+CTRL+E)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, USER_FRIENDLY,
//! SIMPLE_USER_INTERFACE, CLEAR_ABSTRACTION, COMPOSITION_OVER_INHERITANCE.

use std::collections::HashMap;

/// Emoji category
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmojiCategory {
    Smileys,
    Animals,
    Food,
    Activities,
    Travel,
    Objects,
    Symbols,
    Flags,
    Custom,
}

/// Emoji entry
#[derive(Debug, Clone)]
pub struct Emoji {
    pub character: String,
    pub name: String,
    pub category: EmojiCategory,
    pub keywords: Vec<String>,
    pub shortcode: Option<String>,
}

/// Emoji picker
pub struct EmojiPicker {
    pub emojis: HashMap<String, Emoji>,
    pub categories: Vec<EmojiCategory>,
    pub recent: Vec<String>,
    pub max_recent: usize,
    pub search_index: HashMap<String, Vec<String>>, // keyword -> emoji chars
}

impl EmojiPicker {
    pub fn new() -> Self {
        let mut picker = Self {
            emojis: HashMap::new(),
            categories: vec![
                EmojiCategory::Smileys,
                EmojiCategory::Animals,
                EmojiCategory::Food,
                EmojiCategory::Activities,
                EmojiCategory::Travel,
                EmojiCategory::Objects,
                EmojiCategory::Symbols,
                EmojiCategory::Flags,
                EmojiCategory::Custom,
            ],
            recent: Vec::new(),
            max_recent: 20,
            search_index: HashMap::new(),
        };
        picker.populate_default_emojis();
        picker
    }

    fn populate_default_emojis(&mut self) {
        let default_emojis = vec![
            ("😀", "grinning face", EmojiCategory::Smileys, vec!["smile", "happy", "face"], Some(":grinning:")),
            ("😃", "grinning face with big eyes", EmojiCategory::Smileys, vec!["smile", "happy", "eyes"], Some(":grin:")),
            ("😄", "grinning face with smiling eyes", EmojiCategory::Smileys, vec!["smile", "happy", "eyes"], Some(":smile:")),
            ("😁", "beaming face with smiling eyes", EmojiCategory::Smileys, vec!["smile", "happy"], Some(":beaming:")),
            ("😆", "grinning squinting face", EmojiCategory::Smileys, vec!["laugh", "squint"], Some(":laughing:")),
            ("😅", "grinning face with sweat", EmojiCategory::Smileys, vec!["nervous", "sweat"], Some(":sweat_smile:")),
            ("🤣", "rolling on the floor laughing", EmojiCategory::Smileys, vec!["laugh", "floor"], Some(":rolling_on_the_floor_laughing:")),
            ("😂", "face with tears of joy", EmojiCategory::Smileys, vec!["cry", "laugh", "tears"], Some(":joy:")),
            ("🙂", "slightly smiling face", EmojiCategory::Smileys, vec!["smile", "slight"], Some(":slightly_smiling_face:")),
            ("🙃", "upside-down face", EmojiCategory::Smileys, vec!["upside", "down"], Some(":upside_down_face:")),
            ("🐶", "dog face", EmojiCategory::Animals, vec!["dog", "pet", "animal"], Some(":dog:")),
            ("🐱", "cat face", EmojiCategory::Animals, vec!["cat", "pet", "animal"], Some(":cat:")),
            ("🍎", "red apple", EmojiCategory::Food, vec!["fruit", "apple", "food"], Some(":apple:")),
            ("🍕", "pizza", EmojiCategory::Food, vec!["food", "pizza"], Some(":pizza:")),
            ("🚀", "rocket", EmojiCategory::Travel, vec!["space", "rocket", "launch"], Some(":rocket:")),
            ("💻", "laptop", EmojiCategory::Objects, vec!["computer", "laptop", "tech"], Some(":computer:")),
            ("❤️", "red heart", EmojiCategory::Symbols, vec!["love", "heart"], Some(":heart:")),
            ("🎉", "party popper", EmojiCategory::Activities, vec!["party", "celebration"], Some(":tada:")),
        ];

        for (ch, name, category, keywords, shortcode) in default_emojis {
            let emoji = Emoji {
                character: ch.to_string(),
                name: name.to_string(),
                category,
                keywords,
                shortcode: shortcode.map(|s| s.to_string()),
            };
            self.emojis.insert(ch.to_string(), emoji);

            // Build search index
            for keyword in &emoji.keywords {
                self.search_index
                    .entry(keyword.clone())
                    .or_default()
                    .push(ch.to_string());
            }
            // Also index by name
            self.search_index
                .entry(name.to_string())
                .or_default()
                .push(ch.to_string());
        }
    }

    /// Search emojis by keyword
    pub fn search(&self, query: &str) -> Vec<&Emoji> {
        let query_lower = query.to_lowercase();
        let mut results: Vec<&Emoji> = self
            .emojis
            .values()
            .filter(|e| {
                e.name.to_lowercase().contains(&query_lower)
                    || e.character.contains(query)
                    || e.keywords.iter().any(|k| k.to_lowercase().contains(&query_lower))
            })
            .collect();
        results.sort_by(|a, b| a.name.cmp(&b.name));
        results
    }

    /// Get emojis by category
    pub fn get_by_category(&self, category: EmojiCategory) -> Vec<&Emoji> {
        self.emojis
            .values()
            .filter(|e| e.category == category)
            .collect()
    }

    /// Add to recent
    pub fn add_recent(&mut self, character: &str) {
        if self.recent.contains(&character.to_string()) {
            self.recent.retain(|c| c != character);
        }
        self.recent.insert(0, character.to_string());
        while self.recent.len() > self.max_recent {
            self.recent.pop();
        }
    }

    /// Insert emoji (add to recent and return)
    pub fn insert(&mut self, character: &str) -> Option<&Emoji> {
        self.add_recent(character);
        self.emojis.get(character)
    }

    /// List recent emojis
    pub fn list_recent(&self) -> Vec<&Emoji> {
        self.recent
            .iter()
            .filter_map(|c| self.emojis.get(c))
            .collect()
    }
}

impl Default for EmojiPicker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_emoji() {
        let picker = EmojiPicker::new();
        let results = picker.search("dog");
        assert!(!results.is_empty());
        assert!(results.iter().any(|e| e.character == "🐶"));
    }

    #[test]
    fn test_search_by_name() {
        let picker = EmojiPicker::new();
        let results = picker.search("pizza");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_get_by_category() {
        let picker = EmojiPicker::new();
        let animals = picker.get_by_category(EmojiCategory::Animals);
        assert!(!animals.is_empty());
        assert!(animals.iter().any(|e| e.character == "🐶"));
    }

    #[test]
    fn test_insert_adds_recent() {
        let mut picker = EmojiPicker::new();
        picker.insert("😀");
        assert_eq!(picker.list_recent().len(), 1);
        picker.insert("🐶");
        assert_eq!(picker.list_recent().len(), 2);
    }

    #[test]
    fn test_recent_dedup() {
        let mut picker = EmojiPicker::new();
        picker.insert("😀");
        picker.insert("😀"); // duplicate
        assert_eq!(picker.list_recent().len(), 1);
    }
}