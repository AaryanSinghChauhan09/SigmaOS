// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS - ElementaryOS Pantheon desktop bridge
// Principles: MODULARITY, DISCOVERABILITY, LEAST ASTONISHMENT
use alloc::{collections::BTreeMap, string::String, vec::Vec};
#[derive(Debug, Clone)]
pub struct AppCenterEntry {
    pub id: String,
    pub name: String,
    pub version: String,
    pub installed: bool,
}
pub struct AppCenterApi {
    pub catalog: Vec<AppCenterEntry>,
}
impl AppCenterApi {
    pub fn new() -> Self {
        Self {
            catalog: Vec::new(),
        }
    }
    pub fn add_entry(&mut self, e: AppCenterEntry) {
        self.catalog.push(e);
    }
    pub fn search(&self, q: &str) -> Vec<&AppCenterEntry> {
        self.catalog.iter().filter(|e| e.name.contains(q)).collect()
    }
    pub fn install_app(&mut self, id: &str) -> Result<(), &'static str> {
        self.catalog
            .iter_mut()
            .find(|e| e.id == id)
            .ok_or("not found")
            .map(|e| e.installed = true)
    }
    pub fn update_app(&mut self, id: &str) -> Result<(), &'static str> {
        self.install_app(id)
    }
}
#[derive(Debug, Clone)]
pub enum GraniteWidget {
    ModeSwitch,
    WelcomeScreen,
    AlertDialog,
    SourceList,
    Toast,
}
pub struct GalaWmBridge {
    pub workspaces: u8,
}
impl GalaWmBridge {
    pub fn new() -> Self {
        Self { workspaces: 4 }
    }
    pub fn set_workspace(&mut self, n: u8) {
        self.workspaces = n.max(1);
    }
    pub fn add_keybinding(&self, _key: &str, _action: &str) {}
    pub fn toggle_maximize(&self, _window_id: u64) {}
}
pub struct PantheonDesktopConfig {
    pub dark_mode: bool,
    pub accent_color: u32,
    pub scaling_factor: f32,
}
impl Default for PantheonDesktopConfig {
    fn default() -> Self {
        Self {
            dark_mode: false,
            accent_color: 0x64BAFF,
            scaling_factor: 1.0,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_appcenter_search() {
        let mut api = AppCenterApi::new();
        api.add_entry(AppCenterEntry {
            id: "io.elementary.files".into(),
            name: "Files".into(),
            version: "6.5.0".into(),
            installed: false,
        });
        let res = api.search("Files");
        assert_eq!(res.len(), 1);
    }
    #[test]
    fn test_install() {
        let mut api = AppCenterApi::new();
        api.add_entry(AppCenterEntry {
            id: "a".into(),
            name: "App".into(),
            version: "1.0".into(),
            installed: false,
        });
        api.install_app("a").unwrap();
        assert!(api.catalog[0].installed);
    }
}
