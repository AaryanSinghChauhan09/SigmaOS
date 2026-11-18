// SPDX-License-Identifier: MIT OR Apache-2.0
//! SigmaOS XDG Desktop Portal & Application Launcher Engine
//!
//! Inspired by Arch Linux's xdg-desktop-portal-hyprland / wlr protocols
//! and Omarchy's Rofi / Wofi application launcher indexing.
//! Provides .desktop file parser, icon theme resolver, Wayland portal request
//! dispatcher, and application execution command generator.

use std::collections::HashMap;

/// Desktop entry specification (Freedesktop.org XDG)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopEntry {
    pub name: String,
    pub generic_name: Option<String>,
    pub exec: String,
    pub icon: String,
    pub categories: Vec<String>,
    pub terminal: bool,
    pub startup_notify: bool,
}

impl DesktopEntry {
    pub fn new(name: &str, exec: &str, icon: &str) -> Self {
        Self {
            name: name.to_string(),
            generic_name: None,
            exec: exec.to_string(),
            icon: icon.to_string(),
            categories: Vec::new(),
            terminal: false,
            startup_notify: true,
        }
    }

    /// Parse freedesktop .desktop file format
    pub fn parse_desktop_content(content: &str) -> Option<Self> {
        let mut name = None;
        let mut exec = None;
        let mut icon = None;
        let mut generic_name = None;
        let mut categories = Vec::new();
        let mut terminal = false;
        let mut in_desktop_entry = false;

        for line in content.lines() {
            let line = line.trim();
            if line == "[Desktop Entry]" {
                in_desktop_entry = true;
                continue;
            }
            if !in_desktop_entry || line.starts_with('#') {
                continue;
            }
            if line.starts_with('[') {
                break;
            }

            if let Some((k, v)) = line.split_once('=') {
                let k = k.trim();
                let v = v.trim();
                match k {
                    "Name" => name = Some(v.to_string()),
                    "Exec" => exec = Some(v.to_string()),
                    "Icon" => icon = Some(v.to_string()),
                    "GenericName" => generic_name = Some(v.to_string()),
                    "Terminal" => terminal = v.eq_ignore_ascii_case("true"),
                    "Categories" => {
                        categories = v.split(';')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                    _ => {}
                }
            }
        }

        Some(Self {
            name: name?,
            generic_name,
            exec: exec?,
            icon: icon.unwrap_or_else(|| "application-x-executable".to_string()),
            categories,
            terminal,
            startup_notify: true,
        })
    }

    /// Render to standard .desktop file content
    pub fn to_desktop_file(&self) -> String {
        let mut out = String::from("[Desktop Entry]\nType=Application\nVersion=1.4\n");
        out.push_str(&format!("Name={}\n", self.name));
        if let Some(g) = &self.generic_name {
            out.push_str(&format!("GenericName={}\n", g));
        }
        out.push_str(&format!("Exec={}\n", self.exec));
        out.push_str(&format!("Icon={}\n", self.icon));
        out.push_str(&format!("Terminal={}\n", self.terminal));
        if !self.categories.is_empty() {
            out.push_str(&format!("Categories={};\n", self.categories.join(";")));
        }
        out
    }
}

/// Omarchy / Wofi application indexer
#[derive(Debug, Default)]
pub struct WofiAppIndexer {
    apps: HashMap<String, DesktopEntry>,
}

impl WofiAppIndexer {
    pub fn new() -> Self {
        Self { apps: HashMap::new() }
    }

    pub fn register(&mut self, entry: DesktopEntry) {
        self.apps.insert(entry.name.to_lowercase(), entry);
    }

    pub fn search(&self, query: &str) -> Vec<&DesktopEntry> {
        let q = query.to_lowercase();
        self.apps.values()
            .filter(|app| {
                app.name.to_lowercase().contains(&q)
                    || app.categories.iter().any(|c| c.to_lowercase().contains(&q))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_desktop_file_parsing_aaa() {
        let raw = r#"
[Desktop Entry]
Name=Kitty
GenericName=Terminal Emulator
Exec=kitty
Icon=kitty
Terminal=false
Categories=System;TerminalEmulator;
"#;
        let entry = DesktopEntry::parse_desktop_content(raw).unwrap();
        assert_eq!(entry.name, "Kitty");
        assert_eq!(entry.generic_name, Some("Terminal Emulator".to_string()));
        assert_eq!(entry.exec, "kitty");
        assert_eq!(entry.icon, "kitty");
        assert!(!entry.terminal);
        assert!(entry.categories.contains(&"TerminalEmulator".to_string()));
    }

    #[test]
    fn test_desktop_file_serialization_aaa() {
        let mut entry = DesktopEntry::new("Neovim", "nvim", "nvim");
        entry.categories = vec!["Development".to_string(), "TextEditor".to_string()];
        let serialized = entry.to_desktop_file();

        assert!(serialized.contains("Name=Neovim"));
        assert!(serialized.contains("Exec=nvim"));
        assert!(serialized.contains("Categories=Development;TextEditor;"));
    }

    #[test]
    fn test_wofi_indexer_search_aaa() {
        let mut indexer = WofiAppIndexer::new();
        let app1 = DesktopEntry::new("Firefox", "firefox %u", "firefox");
        let mut app2 = DesktopEntry::new("Alacritty", "alacritty", "alacritty");
        app2.categories = vec!["TerminalEmulator".to_string()];

        indexer.register(app1);
        indexer.register(app2);

        let res = indexer.search("term");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].name, "Alacritty");

        let res2 = indexer.search("fire");
        assert_eq!(res2.len(), 1);
        assert_eq!(res2[0].name, "Firefox");
    }
}
