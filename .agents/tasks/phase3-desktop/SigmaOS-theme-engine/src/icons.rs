//! Icon theme lookup for SigmaOS-theme-engine.
//!
//! Implements the XDG icon theme spec inheritance chain: each theme lists
//! parent themes, and lookup falls back up the chain to `hicolor`.

use std::collections::HashMap;
use std::path::PathBuf;

/// A single icon theme with its icon registry.
#[derive(Debug, Clone)]
pub struct IconTheme {
    pub name: String,
    pub parents: Vec<String>,
    /// icon_name → path
    icons: HashMap<String, PathBuf>,
}

impl IconTheme {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            parents: Vec::new(),
            icons: HashMap::new(),
        }
    }

    /// Register an icon.
    pub fn add_icon(&mut self, name: impl Into<String>, path: PathBuf) {
        self.icons.insert(name.into(), path);
    }

    /// Look up an icon in this theme only (no fallback).
    pub fn lookup_local(&self, name: &str) -> Option<&PathBuf> {
        self.icons.get(name)
    }

    pub fn icon_count(&self) -> usize { self.icons.len() }
}

/// Icon size variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IconSize { Small = 16, Medium = 24, Large = 48, Scalable = 0 }

impl Default for IconSize { fn default() -> Self { Self::Medium } }

/// Registry of icon themes with fallback chain resolution.
#[derive(Debug, Default)]
pub struct IconThemeRegistry {
    themes: HashMap<String, IconTheme>,
}

impl IconThemeRegistry {
    pub fn new() -> Self { Self::default() }

    pub fn register(&mut self, theme: IconTheme) {
        self.themes.insert(theme.name.clone(), theme);
    }

    /// Look up an icon, walking the inheritance chain.
    /// Falls back to `hicolor` if the primary theme doesn't have it.
    pub fn lookup(&self, theme_name: &str, icon_name: &str) -> Option<&PathBuf> {
        let theme = self.themes.get(theme_name)?;
        if let Some(path) = theme.lookup_local(icon_name) {
            return Some(path);
        }
        // Walk parent themes.
        for parent_name in &theme.parents {
            if let Some(path) = self.lookup(parent_name, icon_name) {
                return Some(path);
            }
        }
        // Final fallback: hicolor.
        if theme_name != "hicolor" {
            return self.lookup("hicolor", icon_name);
        }
        None
    }
}
