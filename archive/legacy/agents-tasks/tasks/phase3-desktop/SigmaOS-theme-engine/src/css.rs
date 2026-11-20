//! CSS variable resolution for SigmaOS-theme-engine.
//!
//! Maps semantic design tokens to CSS custom property strings that can be
//! injected into a GTK4 CSS provider.

use std::collections::HashMap;
use crate::ColorPalette;

/// A CSS variable map: `--token-name -> value`.
#[derive(Debug, Default, Clone)]
pub struct CssVariableMap {
    vars: HashMap<String, String>,
}

impl CssVariableMap {
    pub fn new() -> Self { Self::default() }

    /// Insert a CSS variable (key should include the `--` prefix).
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.vars.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.vars.get(key).map(String::as_str)
    }

    pub fn len(&self) -> usize { self.vars.len() }
    pub fn is_empty(&self) -> bool { self.vars.is_empty() }

    /// Render to a `:root { ... }` CSS block.
    pub fn to_css_block(&self) -> String {
        let mut lines: Vec<String> = self.vars.iter()
            .map(|(k, v)| format!("  {}: {};", k, v))
            .collect();
        lines.sort();
        format!(":root {{\n{}\n}}", lines.join("\n"))
    }
}

/// Convert a `ColorPalette` to a `CssVariableMap` using `--sigma-<token>` naming.
pub fn palette_to_css(palette: &ColorPalette) -> CssVariableMap {
    let renderer = crate::ThemeRenderer::default();
    let hex_map = renderer.render_css(palette);
    let mut css = CssVariableMap::new();
    for (name, hex) in hex_map {
        css.set(format!("--sigma-{}", name), hex);
    }
    css
}

/// Dark/light mode indicator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSchemePreference {
    Dark,
    Light,
}

impl Default for ColorSchemePreference {
    fn default() -> Self { Self::Dark }
}
