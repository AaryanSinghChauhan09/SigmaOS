//! SigmaOS Plymouth Boot Theme Manager
//!
//! Inspired by: Omarchy plymouth (omancy-plymouth-current, omancy-plymouth-list, omancy-plymouth-preview, omancy-plymouth-reset, omancy-plymouth-set, omancy-plymouth-set-by-theme, omancy-plymouth-switcher)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! BOOTLOADER_INTEGRATION, SECURE_DEFAULTS, USER_CUSTOMIZATION, AESTHETICS.

/// Plymouth theme
#[derive(Debug, Clone)]
pub struct PlymouthTheme {
    pub name: String,
    pub description: String,
    pub path: String,
    pub preview_path: Option<String>,
    pub is_installed: bool,
}

/// Plymouth manager
pub struct PlymouthManager {
    pub themes: Vec<PlymouthTheme>,
    pub current_theme: Option<String>,
    pub themes_dir: String,
}

impl PlymouthManager {
    pub fn new() -> Self {
        Self {
            themes: Vec::new(),
            current_theme: None,
            themes_dir: "/usr/share/plymouth/themes".to_string(),
        }
    }

    /// List available themes
    pub fn list_themes(&self) -> Vec<&PlymouthTheme> {
        self.themes.iter().collect()
    }

    /// Install a theme (register it)
    pub fn install_theme(&mut self, theme: PlymouthTheme) -> Result<(), String> {
        if self
            .themes
            .iter()
            .any(|t| t.name == theme.name)
        {
            return Err(format!("Theme {} already installed", theme.name));
        }
        self.themes.push(theme);
        Ok(())
    }

    /// Set current theme
    pub fn set_theme(&mut self, name: &str) -> Result<(), String> {
        if !self.themes.iter().any(|t| t.name == name) {
            return Err(format!("Theme {} not found", name));
        }
        self.current_theme = Some(name.to_string());
        Ok(())
    }

    /// Reset to default theme
    pub fn reset(&mut self) {
        self.current_theme = None;
    }

    /// Get current theme
    pub fn current_theme(&self) -> Option<&String> {
        self.current_theme.as_ref()
    }

    /// Preview a theme
    pub fn preview(&self, name: &str) -> Result<&PlymouthTheme, String> {
        self.themes
            .iter()
            .find(|t| t.name == name)
            .ok_or_else(|| format!("Theme {} not found", name))
    }

    /// Remove a theme
    pub fn remove_theme(&mut self, name: &str) -> Result<PlymouthTheme, String> {
        let pos = self
            .themes
            .iter()
            .position(|t| t.name == name)
            .ok_or_else(|| format!("Theme {} not found", name))?;
        if self.current_theme.as_deref() == Some(name) {
            self.current_theme = None;
        }
        Ok(self.themes.remove(pos))
    }

    /// Theme count
    pub fn theme_count(&self) -> usize {
        self.themes.len()
    }
}

impl Default for PlymouthManager {
    fn default() -> Self {
        let mut mgr = Self::new();
        // Register default themes
        mgr.install_theme(PlymouthTheme {
            name: "sigmaos".to_string(),
            description: "Default SigmaOS boot theme".to_string(),
            path: "/usr/share/plymouth/themes/sigmaos".to_string(),
            preview_path: Some("/usr/share/plymouth/themes/sigmaos/preview.png".to_string()),
            is_installed: true,
        }).ok();
        mgr.install_theme(PlymouthTheme {
            name: "text".to_string(),
            description: "Text-only boot theme".to_string(),
            path: "/usr/share/plymouth/themes/text".to_string(),
            preview_path: None,
            is_installed: true,
        }).ok();
        mgr.install_theme(PlymouthTheme {
            name: "spinfinity".to_string(),
            description: "Spinning infinity symbol".to_string(),
            path: "/usr/share/plymouth/themes/spinfinity".to_string(),
            preview_path: None,
            is_installed: true,
        }).ok();
        mgr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_themes() {
        let mgr = PlymouthManager::default();
        assert_eq!(mgr.theme_count(), 3);
    }

    #[test]
    fn test_set_theme() {
        let mut mgr = PlymouthManager::new();
        assert!(mgr.set_theme("sigmaos").is_ok());
        assert_eq!(mgr.current_theme(), Some(&"sigmaos".to_string()));
    }

    #[test]
    fn test_set_nonexistent_theme_fails() {
        let mut mgr = PlymouthManager::new();
        assert!(mgr.set_theme("nonexistent").is_err());
    }

    #[test]
    fn test_install_theme() {
        let mut mgr = PlymouthManager::new();
        let theme = PlymouthTheme {
            name: "custom".to_string(),
            description: "Custom theme".to_string(),
            path: "/usr/share/plymouth/themes/custom".to_string(),
            preview_path: None,
            is_installed: true,
        };
        assert!(mgr.install_theme(theme).is_ok());
        assert_eq!(mgr.theme_count(), 1);
    }

    #[test]
    fn test_install_duplicate_fails() {
        let mut mgr = PlymouthManager::new();
        let theme = PlymouthTheme {
            name: "dup".to_string(),
            description: "Dup".to_string(),
            path: "/dup".to_string(),
            preview_path: None,
            is_installed: true,
        };
        mgr.install_theme(theme.clone()).ok();
        let result = mgr.install_theme(theme);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_theme() {
        let mut mgr = PlymouthManager::new();
        let theme = PlymouthTheme {
            name: "temp".to_string(),
            description: "Temp".to_string(),
            path: "/temp".to_string(),
            preview_path: None,
            is_installed: true,
        };
        mgr.install_theme(theme).ok();
        let removed = mgr.remove_theme("temp").unwrap();
        assert_eq!(removed.name, "temp");
        assert_eq!(mgr.theme_count(), 0);
    }

    #[test]
    fn test_preview() {
        let mgr = PlymouthManager::default();
        let theme = mgr.preview("text").unwrap();
        assert_eq!(theme.name, "text");
    }
}