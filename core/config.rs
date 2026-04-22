/// core/config.rs — Custom config parser (zero dependencies)
/// Parses flat key=value files and JSON-like profile files
/// without serde, toml, or any external crate.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Default)]
pub struct Config {
    map: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Self { Self::default() }

    /// Load from a flat `key=value` file (lines starting with # are comments)
    pub fn load_kv(path: &Path) -> Result<Self, String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let mut cfg = Self::new();
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }
            if let Some((k, v)) = line.split_once('=') {
                cfg.map.insert(k.trim().to_string(), v.trim().to_string());
            }
        }
        Ok(cfg)
    }

    /// Minimal JSON string extractor — no full parser, handles flat {"k":"v"} objects
    pub fn load_json(path: &Path) -> Result<Self, String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let mut cfg = Self::new();
        // Strip outer braces, split by comma, parse "key": "value" / "key": number / "key": bool
        let inner = content.trim().trim_start_matches('{').trim_end_matches('}');
        for chunk in inner.split(',') {
            let chunk = chunk.trim();
            if let Some((k, v)) = chunk.split_once(':') {
                let key = k.trim().trim_matches('"').to_string();
                let val = v.trim().trim_matches('"').to_string();
                if !key.is_empty() {
                    cfg.map.insert(key, val);
                }
            }
        }
        Ok(cfg)
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.map.get(key).map(|s| s.as_str())
    }

    pub fn get_or(&self, key: &str, default: &str) -> String {
        self.map.get(key).cloned().unwrap_or_else(|| default.to_string())
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.map.insert(key.to_string(), value.to_string());
    }

    pub fn save_kv(&self, path: &Path) -> Result<(), String> {
        let mut lines = vec!["# SigmaOS config — auto-generated".to_string()];
        let mut keys: Vec<_> = self.map.keys().collect();
        keys.sort();
        for k in keys {
            lines.push(format!("{}={}", k, self.map[k]));
        }
        fs::write(path, lines.join("\n")).map_err(|e| e.to_string())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.map.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_kv_roundtrip() {
        let mut cfg = Config::new();
        cfg.set("theme", "MATRIX");
        cfg.set("blur", "25");
        assert_eq!(cfg.get("theme"), Some("MATRIX"));
        assert_eq!(cfg.get_or("blur", "0"), "25");
        assert_eq!(cfg.get("missing"), None);
    }
}
