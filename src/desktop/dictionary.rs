// SPDX-License-Identifier: MIT
// SigmaOS - Dictionary
// Inspired by GNOME Dictionary and GoldenDict
// Provides dictionary and word lookup

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Dictionary type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DictionaryType {
    English,
    Thesaurus,
    Translation,
    Medical,
    Legal,
}

/// Word definition
#[derive(Debug, Clone)]
pub struct Definition {
    pub part_of_speech: String,
    pub definition: String,
    pub example: String,
}

/// Synonym
#[derive(Debug, Clone)]
pub struct Synonym {
    pub word: String,
    pub context: String,
}

/// Word
#[derive(Debug, Clone)]
pub struct Word {
    pub text: String,
    pub pronunciation: String,
    pub definitions: Vec<Definition>,
    pub synonyms: Vec<Synonym>,
    pub antonyms: Vec<String>,
    pub etymology: String,
    pub frequency: u32,
}

/// Dictionary
pub struct Dictionary {
    pub dictionary_type: DictionaryType,
    pub source_language: String,
    pub target_language: String,
    pub words: BTreeMap<String, Word>,
    pub history: Vec<String>,
    pub favorites: Vec<String>,
}

impl Dictionary {
    pub fn new(dictionary_type: DictionaryType) -> Self {
        Self {
            dictionary_type,
            source_language: "en".to_string(),
            target_language: "en".to_string(),
            words: BTreeMap::new(),
            history: Vec::new(),
            favorites: Vec::new(),
        }
    }

    /// Add word
    pub fn add_word(&mut self, word: Word) {
        self.words.insert(word.text.clone(), word);
    }

    /// Remove word
    pub fn remove_word(&mut self, text: String) -> Result<(), &'static str> {
        if !self.words.contains_key(&text) {
            return Err("Word not found");
        }
        self.words.remove(&text);
        Ok(())
    }

    /// Lookup word
    pub fn lookup(&mut self, text: String) -> Option<&Word> {
        // Add to history
        if self.words.contains_key(&text) {
            if let Some(pos) = self.history.iter().position(|w| w == &text) {
                self.history.remove(pos);
            }
            self.history.insert(0, text.clone());
            if self.history.len() > 100 {
                self.history.pop();
            }
        }
        self.words.get(&text)
    }

    /// Add to favorites
    pub fn add_favorite(&mut self, text: String) {
        if !self.favorites.contains(&text) {
            self.favorites.push(text);
        }
    }

    /// Remove from favorites
    pub fn remove_favorite(&mut self, text: String) -> Result<(), &'static str> {
        let index = self
            .favorites
            .iter()
            .position(|w| w == &text)
            .ok_or("Word not in favorites")?;
        self.favorites.remove(index);
        Ok(())
    }

    /// Search
    pub fn search(&self, query: String) -> Vec<&Word> {
        let query_lower = query.to_lowercase();
        self.words
            .values()
            .filter(|w| w.text.to_lowercase().contains(&query_lower))
            .collect()
    }

    /// Get history
    pub fn get_history(&self) -> &[String] {
        &self.history
    }

    /// Get favorites
    pub fn get_favorites(&self) -> &[String] {
        &self.favorites
    }

    /// Word count
    pub fn word_count(&self) -> usize {
        self.words.len()
    }
}

/// Global dictionary
static mut DICTIONARY: Option<Dictionary> = None;

/// Initialize dictionary
pub fn init_dictionary() {
    unsafe {
        DICTIONARY = Some(Dictionary::new(DictionaryType::English));
    }
}

/// Get dictionary
pub fn get_dictionary() -> &'static mut Dictionary {
    unsafe { DICTIONARY.as_mut().expect("Dictionary not initialized") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dictionary_creation() {
        let d = Dictionary::new(DictionaryType::English);
        assert_eq!(d.word_count(), 0);
    }

    #[test]
    fn test_add_word() {
        let mut d = Dictionary::new(DictionaryType::English);
        let word = Word {
            text: "hello".to_string(),
            pronunciation: "/həˈloʊ/".to_string(),
            definitions: vec![Definition {
                part_of_speech: "interjection".to_string(),
                definition: "Used as a greeting".to_string(),
                example: "Hello, world!".to_string(),
            }],
            synonyms: Vec::new(),
            antonyms: Vec::new(),
            etymology: String::new(),
            frequency: 1000,
        };
        d.add_word(word);
        assert_eq!(d.word_count(), 1);
    }

    #[test]
    fn test_lookup() {
        let mut d = Dictionary::new(DictionaryType::English);
        let word = Word {
            text: "test".to_string(),
            pronunciation: String::new(),
            definitions: Vec::new(),
            synonyms: Vec::new(),
            antonyms: Vec::new(),
            etymology: String::new(),
            frequency: 0,
        };
        d.add_word(word);
        let result = d.lookup("test".to_string());
        assert!(result.is_some());
    }
}
