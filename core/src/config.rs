#![no_std]
/// core/src/config.rs — Sovereign Config Engine
/// Zero-dependency silicon primitives.

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

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
                while let Some(&n) = chars.peek() { if n == ':' { chars.next(); break; } chars.next(); }
                while let Some(&n) = chars.peek() { if Self::is_space(n) { chars.next(); } else { break; } }
                let mut val = String::new();
                if let Some(&n) = chars.peek() {
                    if n == '"' { chars.next(); val = Self::read_string(&mut chars); }
                    else { while let Some(&v) = chars.peek() { if v == ',' || v == '}' || v == ']' { break; } val.push(chars.next().unwrap()); } }
                }
                cfg.set(Self::manual_trim(&key), Self::manual_trim(&val));
            }
        }
        cfg
    }

    fn is_space(c: char) -> bool { c == ' ' || c == '\n' || c == '\r' || c == '\t' }

    fn manual_trim(s: &str) -> &str {
        let b = s.as_bytes();
        let mut start = 0;
        while start < b.len() && (b[start] == b' ' || b[start] == b'\n' || b[start] == b'\r' || b[start] == b'\t') { start += 1; }
        let mut end = b.len();
        while end > start && (b[end-1] == b' ' || b[end-1] == b'\n' || b[end-1] == b'\r' || b[end-1] == b'\t') { end -= 1; }
        if start >= end { "" } else { &s[start..end] }
    }

    fn read_string(chars: &mut core::iter::Peekable<core::str::Chars>) -> String {
        let mut s = String::new();
        while let Some(c) = chars.next() { 
            if c == '"' { break; } 
            if s.len() > 1024 { break; } // Loophole: Prevent memory exhaustion
            s.push(c); 
        }
        s
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.pairs.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn set(&mut self, key: &str, val: &str) {
        if let Some(p) = self.pairs.iter().position(|(k, _)| k == key) { self.pairs[p].1 = val.to_string(); }
        else { self.pairs.push((key.to_string(), val.to_string())); }
    }
}
