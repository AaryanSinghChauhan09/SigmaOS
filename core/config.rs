#![no_std]
/// core/config.rs — Sovereign Config Engine
/// Zero-dependency silicon primitives. No HashMap, No heavy libs.

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::panic::PanicInfo;

#[derive(Debug, Default)]
pub struct Config {
    pairs: Vec<(String, String)>,
}

impl Config {
    pub fn new() -> Self { Self { pairs: Vec::new() } }

    pub fn load_json(content: &str) -> Self {
        let mut cfg = Self::new();
        let mut chars = content.chars().peekable();
        
        while let Some(c) = chars.next() {
            if c == '"' {
                let key = Self::read_string(&mut chars);
                while let Some(&next) = chars.peek() {
                    if next == ':' { chars.next(); break; }
                    chars.next();
                }
                while let Some(&next) = chars.peek() {
                    if next.is_whitespace() { chars.next(); } else { break; }
                }
                
                let mut val = String::new();
                if let Some(&next) = chars.peek() {
                    if next == '"' {
                        chars.next();
                        val = Self::read_string(&mut chars);
                    } else {
                        while let Some(&v) = chars.peek() {
                            if v == ',' || v == '}' || v == ']' { break; }
                            val.push(chars.next().unwrap());
                        }
                    }
                }
                cfg.set(key.trim(), val.trim());
            }
        }
        cfg
    }

    fn read_string(chars: &mut core::iter::Peekable<core::str::Chars>) -> String {
        let mut s = String::new();
        while let Some(c) = chars.next() {
            if c == '"' { break; }
            s.push(c);
        }
        s
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.pairs.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn set(&mut self, key: &str, value: &str) {
        if let Some(pos) = self.pairs.iter().position(|(k, _)| k == key) {
            self.pairs[pos].1 = value.to_string();
        } else {
            self.pairs.push((key.to_string(), value.to_string()));
        }
    }

    pub fn save_json(&self) -> String {
        let mut s = String::from("{\n");
        for (i, (k, v)) in self.pairs.iter().enumerate() {
            s.push_str("  \"");
            s.push_str(k);
            s.push_str("\": \"");
            s.push_str(v);
            s.push_str("\"");
            if i < self.pairs.len() - 1 { s.push(','); }
            s.push('\n');
        }
        s.push_str("}\n");
        s
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.pairs.iter().map(|(k, v)| (k, v))
    }
}

pub struct ProfileConfig {
    pub name: String,
    pub theme: String,
    pub accent: String,
    pub auto_sync: bool,
    pub shards: Vec<String>,
}

impl ProfileConfig {
    pub fn load(content: &str) -> Self {
        let cfg = Config::load_json(content);
        let mut shards = Vec::new();
        if let Some(s_list) = cfg.get("shards") {
            for s in s_list.split(',') {
                let cleaned = s.trim().trim_matches('[').trim_matches(']').trim_matches('"').trim();
                if !cleaned.is_empty() { shards.push(cleaned.to_string()); }
            }
        }

        Self {
            name: cfg.get("name").cloned().unwrap_or_else(|| "default".into()),
            theme: cfg.get("theme").cloned().unwrap_or_else(|| "dark".into()),
            accent: cfg.get("accent").cloned().unwrap_or_else(|| "#00f0ff".into()),
            auto_sync: cfg.get("auto_sync").map(|s| s == "true").unwrap_or(true),
            shards,
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
