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

/// Extended Profile Configuration
#[derive(Debug, Clone)]
pub struct ProfileConfig {
    pub name: String,
    pub sync_interval: u64,
    pub shards: Vec<String>,
    pub theme: String,
    pub accent: String,
    pub blur: u32,
    pub auto_sync: bool,
    pub self_heal: bool,
    pub layout_panels: Vec<String>,
    pub shortcuts: HashMap<String, String>,
}

impl ProfileConfig {
    pub fn load(path: &std::path::Path) -> Result<Self, String> {
        let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        
        // Lightweight manual parsing for extended profiles
        // Note: In a real system we'd use a zero-dep JSON parser, 
        // but here we match the "minimal parsing" requirement.
        
        let name = Self::extract(&content, "name").unwrap_or_else(|| "default".into());
        let sync_interval = Self::extract(&content, "sync_interval")
            .and_then(|s| s.parse().ok())
            .unwrap_or(300);
        let theme = Self::extract(&content, "theme").unwrap_or_else(|| "dark".into());
        let accent = Self::extract(&content, "accent").unwrap_or_else(|| "#00f0ff".into());
        let blur = Self::extract(&content, "blur")
            .and_then(|s| s.parse().ok())
            .unwrap_or(25);
        let auto_sync = content.contains("\"auto_sync\": true");
        let self_heal = content.contains("\"self_heal\": true");

        // Extract shards list
        let mut shards = Vec::new();
        if let Some(start) = content.find("\"shards\": [") {
            if let Some(end) = content[start..].find(']') {
                let list = &content[start + 10 .. start + end];
                for s in list.split(',') {
                    let cleaned = s.trim().trim_matches('"').to_string();
                    if !cleaned.is_empty() { shards.push(cleaned); }
                }
            }
        }

        // Extract layout panels
        let mut layout_panels = Vec::new();
        if let Some(start) = content.find("\"dashboard_panels\": [") {
            if let Some(end) = content[start..].find(']') {
                let list = &content[start + 20 .. start + end];
                for s in list.split(',') {
                    let cleaned = s.trim().trim_matches('"').to_string();
                    if !cleaned.is_empty() { layout_panels.push(cleaned); }
                }
            }
        }

        // Extract shortcuts
        let mut shortcuts = HashMap::new();
        if let Some(start) = content.find("\"shortcuts\": {") {
            if let Some(end) = content[start..].find('}') {
                let obj = &content[start + 13 .. start + end];
                for pair in obj.split(',') {
                    if let Some((k, v)) = pair.split_once(':') {
                        let key = k.trim().trim_matches('"').to_string();
                        let val = v.trim().trim_matches('"').to_string();
                        if !key.is_empty() { shortcuts.insert(key, val); }
                    }
                }
            }
        }

        Ok(ProfileConfig {
            name, sync_interval, shards, theme, accent, blur,
            auto_sync, self_heal, layout_panels, shortcuts,
        })
    }

    fn extract(content: &str, key: &str) -> Option<String> {
        let pattern = format!("\"{}\":", key);
        if let Some(pos) = content.find(&pattern) {
            let start = pos + pattern.len();
            let mut end = start;
            let mut in_quotes = false;
            let bytes = content.as_bytes();
            
            // Skip whitespace
            while end < bytes.len() && (bytes[end] as char).is_whitespace() || bytes[end] == b':' {
                end += 1;
            }
            
            let val_start = end;
            while end < bytes.len() {
                let c = bytes[end] as char;
                if c == '"' {
                    if in_quotes { end += 1; break; }
                    in_quotes = true;
                } else if !in_quotes && (c == ',' || c == '}' || c == ']' || c.is_whitespace()) {
                    break;
                }
                end += 1;
            }
            let val = &content[val_start..end];
            return Some(val.trim().trim_matches('"').to_string());
        }
        None
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
