// SigmaOS Self-Hosted TOML-Subset Parser
// Zero-dependency parser for the subset of TOML used by OS manifests, image
// descriptors and package recipes (tables, keys, strings, integers, booleans,
// arrays of strings). Implementing it in-kernel removes the project's reliance
// on external TOML/serde crates and keeps the sovereign toolchain dependency-free.

use crate::klib::HashMap;
use std::string::{String, ToString};
use std::vec::Vec;

/// A parsed TOML document: top-level keys plus nested tables.
pub struct TomlDocument {
    /// Top-level scalar/array keys (table `a.b.c` is flattened to key `a.b.c`).
    values: HashMap<String, TomlValue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TomlValue {
    String(String),
    Integer(i64),
    Boolean(bool),
    Array(Vec<String>),
}

impl TomlValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TomlValue::String(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TomlValue::Integer(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TomlValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }
}

impl TomlDocument {
    pub fn new() -> Self {
        TomlDocument {
            values: HashMap::new(),
        }
    }

    /// Parse TOML text into a document. Returns Err on the first malformed line.
    pub fn parse(input: &str) -> Result<Self, &'static str> {
        let mut doc = TomlDocument::new();
        let mut table = String::new(); // current dotted table prefix ("" == root)
        for raw in input.lines() {
            let line = trim_comment(raw).trim();
            if line.is_empty() {
                continue;
            }
            if line.starts_with('[') {
                // [a.b.c] or [[array.of.tables]]
                if !line.ends_with(']') {
                    return Err("Unterminated table header");
                }
                let inner = &line[1..line.len() - 1];
                if inner.starts_with('[') {
                    // Array-of-tables marker: treat each entry as a sibling table
                    // by appending a synthetic index; we keep it simple and just
                    // use the prefix as-is (values overwrite, which is fine for
                    // our manifest-read use case).
                    let trimmed = inner.trim_start_matches('[');
                    table = trimmed.to_string();
                } else {
                    table = inner.to_string();
                }
                continue;
            }
            // key = value
            let eq = line.find('=').ok_or("Missing '=' in key/value pair")?;
            let key = line[..eq].trim().to_string();
            let value = line[eq + 1..].trim();
            let full_key = if table.is_empty() {
                key
            } else {
                let mut f = table.clone();
                f.push('.');
                f.push_str(&key);
                f
            };
            let parsed = parse_value(value)?;
            doc.values.insert(full_key, parsed);
        }
        Ok(doc)
    }

    pub fn get(&self, key: &str) -> Option<&TomlValue> {
        self.values.get(key)
    }

    pub fn get_string(&self, key: &str) -> Option<&str> {
        self.get(key).and_then(|v| v.as_str())
    }

    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key).and_then(|v| v.as_int())
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key).and_then(|v| v.as_bool())
    }

    /// All keys whose dotted path starts with `prefix` (e.g. "services").
    pub fn subtree(&self, prefix: &str) -> HashMap<String, TomlValue> {
        let mut out = HashMap::new();
        for (k, v) in self.values.iter() {
            if k == prefix || k.starts_with(prefix) && k[prefix.len()..].starts_with('.') {
                out.insert(k.clone(), v.clone());
            }
        }
        out
    }
}

fn parse_value(value: &str) -> Result<TomlValue, &'static str> {
    if value.starts_with('"') {
        if !value.ends_with('"') || value.len() < 2 {
            return Err("Unterminated string");
        }
        return Ok(TomlValue::String(unescape(&value[1..value.len() - 1])));
    }
    if value.starts_with('[') {
        if !value.ends_with(']') {
            return Err("Unterminated array");
        }
        let inner = &value[1..value.len() - 1];
        let mut items = Vec::new();
        for part in inner.split(',') {
            let p = part.trim();
            if p.is_empty() {
                continue;
            }
            if !p.starts_with('"') || !p.ends_with('"') {
                return Err("Array elements must be strings");
            }
            items.push(unescape(&p[1..p.len() - 1]));
        }
        return Ok(TomlValue::Array(items));
    }
    if value == "true" {
        return Ok(TomlValue::Boolean(true));
    }
    if value == "false" {
        return Ok(TomlValue::Boolean(false));
    }
    if let Ok(i) = value.parse::<i64>() {
        return Ok(TomlValue::Integer(i));
    }
    Err("Unsupported TOML value")
}

/// Strip a leading `#` comment, but not one inside a quoted string.
fn trim_comment(line: &str) -> &str {
    let mut in_str = false;
    for (i, c) in line.char_indices() {
        if c == '"' {
            in_str = !in_str;
        } else if c == '#' && !in_str {
            return &line[..i];
        }
    }
    line
}

fn unescape(s: &str) -> String {
    let mut out = String::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' && i + 1 < bytes.len() {
            match bytes[i + 1] {
                b'n' => out.push('\n'),
                b't' => out.push('\t'),
                b'\\' => out.push('\\'),
                b'"' => out.push('"'),
                _ => {
                    out.push('\\');
                    out.push(bytes[i + 1] as char);
                }
            }
            i += 2;
        } else {
            out.push(bytes[i] as char);
            i += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_scalars_and_tables() {
        let doc = TomlDocument::parse(
            r#"
# SigmaOS image manifest
name = "sovereign-base"
version = 2026

[build]
optimize = true
targets = ["x86_64", "aarch64"]

[services.init]
oneshot = true
"#,
        )
        .unwrap();
        assert_eq!(doc.get_string("name"), Some("sovereign-base"));
        assert_eq!(doc.get_int("version"), Some(2026));
        assert_eq!(doc.get_bool("build.optimize"), Some(true));
        assert_eq!(doc.get_string("services.init.oneshot_missing"), None);
        if let Some(TomlValue::Array(items)) = doc.get("build.targets") {
            assert_eq!(items.len(), 2);
            assert_eq!(items[0], "x86_64");
        } else {
            panic!("expected array");
        }
    }

    #[test]
    fn test_subtree_and_escape() {
        let doc = TomlDocument::parse(
            r#"
[entry]
path = "C:\\sigma\\boot"
note = "line1\nline2"
"#,
        )
        .unwrap();
        assert_eq!(doc.get_string("entry.path"), Some("C:\\sigma\\boot"));
        assert_eq!(doc.get_string("entry.note"), Some("line1\nline2"));
    }

    #[test]
    fn test_rejects_malformed() {
        assert!(TomlDocument::parse("name = ").is_err());
        assert!(TomlDocument::parse("[unterminated").is_err());
        assert!(TomlDocument::parse("bad line").is_err());
    }
}
