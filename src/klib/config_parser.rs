#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SigmaOS Self-Hosted INI/Key-Value Config Parser
// Zero-dependency configuration store replacing external parser libraries
// Supports [sections] and key = value pairs

use crate::klib::HashMap;
use std::string::{String, ToString};
use std::vec::Vec;

/// INI-style configuration store
pub struct ConfigStore {
    sections: HashMap<String, HashMap<String, String>>,
}

impl ConfigStore {
    /// Create a new empty configuration store
    pub fn new() -> Self {
        ConfigStore {
            sections: HashMap::new(),
        }
    }

    /// Parse INI-formatted text into the store
    pub fn parse(&mut self, input: &str) -> Result<(), &'static str> {
        let mut current_section = String::new();

        for (_line_no, line) in input.lines().enumerate() {
            let trimmed = trim_line(line);

            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
                continue;
            }

            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                current_section = trimmed[1..trimmed.len() - 1].to_string();
                self.sections.entry(current_section.clone()).or_default();
                continue;
            }

            if let Some(sep_pos) = find_key_value_sep(&trimmed) {
                let key = trimmed[..sep_pos].trim().to_string();
                let value = trimmed[sep_pos + 1..].trim().to_string();
                let section = self.sections.entry(current_section.clone()).or_default();
                section.insert(key, value);
            } else {
                return Err("Invalid config line");
            }
        }

        Ok(())
    }

    /// Get a string value from a section/key
    pub fn get(&self, section: &str, key: &str) -> Option<&str> {
        self.sections.get(section)?.get(key).map(|s| s.as_str())
    }

    /// Get a boolean value from a section/key
    pub fn get_bool(&self, section: &str, key: &str) -> Option<bool> {
        self.get(section, key).and_then(|v| match v {
            "true" | "yes" | "1" | "on" => Some(true),
            "false" | "no" | "0" | "off" => Some(false),
            _ => None,
        })
    }

    /// Get a u64 value from a section/key
    pub fn get_u64(&self, section: &str, key: &str) -> Option<u64> {
        self.get(section, key)?.parse().ok()
    }

    /// Get all keys in a section
    pub fn keys(&self, section: &str) -> Option<Vec<&str>> {
        self.sections
            .get(section)
            .map(|map| map.keys().map(|k| k.as_str()).collect())
    }

    /// Get all section names
    pub fn sections(&self) -> Vec<&str> {
        self.sections.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for ConfigStore {
    fn default() -> Self {
        Self::new()
    }
}

fn trim_line(s: &str) -> &str {
    let start = s.chars().take_while(|c| c.is_whitespace()).count();
    let end = s.chars().rev().take_while(|c| c.is_whitespace()).count();
    if start + end >= s.len() {
        ""
    } else {
        &s[start..s.len() - end]
    }
}

fn find_key_value_sep(s: &str) -> Option<usize> {
    s.find('=')
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    const SAMPLE_CONFIG: &str = r#"
# Global comment
[main]
name = SigmaOS
version = 1.0
enabled = true
port = 8080

[network]
interface = eth0
mtu = 1500
dhcp = yes

[database]
host = localhost
port = 5432
"#;

    #[test]
    fn test_config_store_parse() {
        let mut store = ConfigStore::new();
        assert!(store.parse(SAMPLE_CONFIG).is_ok());

        assert_eq!(store.get("main", "name"), Some("SigmaOS"));
        assert_eq!(store.get("main", "version"), Some("1.0"));
        assert_eq!(store.get("main", "enabled"), Some("true"));
        assert_eq!(store.get("main", "port"), Some("8080"));

        assert_eq!(store.get("network", "interface"), Some("eth0"));
        assert_eq!(store.get("database", "host"), Some("localhost"));
    }

    #[test]
    fn test_config_store_get_bool() {
        let mut store = ConfigStore::new();
        store.parse(SAMPLE_CONFIG).unwrap();

        assert_eq!(store.get_bool("main", "enabled"), Some(true));
        assert_eq!(store.get_bool("network", "dhcp"), Some(true));
        assert_eq!(store.get_bool("main", "port"), None);
    }

    #[test]
    fn test_config_store_get_u64() {
        let mut store = ConfigStore::new();
        store.parse(SAMPLE_CONFIG).unwrap();

        assert_eq!(store.get_u64("main", "port"), Some(8080));
        assert_eq!(store.get_u64("network", "mtu"), Some(1500));
        assert_eq!(store.get_u64("database", "port"), Some(5432));
        assert_eq!(store.get_u64("main", "name"), None);
    }

    #[test]
    fn test_config_store_sections() {
        let mut store = ConfigStore::new();
        store.parse(SAMPLE_CONFIG).unwrap();

        let sections = store.sections();
        assert!(sections.contains(&"main"));
        assert!(sections.contains(&"network"));
        assert!(sections.contains(&"database"));
    }

    #[test]
    fn test_config_store_keys() {
        let mut store = ConfigStore::new();
        store.parse(SAMPLE_CONFIG).unwrap();

        let keys = store.keys("main").unwrap();
        assert!(keys.contains(&"name"));
        assert!(keys.contains(&"version"));
        assert!(keys.contains(&"enabled"));
        assert!(keys.contains(&"port"));
    }

    #[test]
    fn test_config_store_empty() {
        let store = ConfigStore::new();
        assert!(store.sections().is_empty());
        assert!(store.get("missing", "key").is_none());
    }

    #[test]
    fn test_config_store_malformed() {
        let mut store = ConfigStore::new();
        // A line with no '=' and that is not a comment or section header is malformed.
        assert!(store.parse("no_equals_here but also no section").is_err());
        // A key = value with no enclosing section is still valid (global section).
        assert!(store.parse("key = value").is_ok());
    }
}
