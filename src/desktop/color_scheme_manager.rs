// Desktop Color Scheme Manager
// Linux Mint & Omarchy inspiration for comprehensive color scheme management

use std::collections::HashMap;

/// Color Scheme Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSchemeType {
    Light,
    Dark,
    HighContrast,
    Custom,
}

impl ColorSchemeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ColorSchemeType::Light => "light",
            ColorSchemeType::Dark => "dark",
            ColorSchemeType::HighContrast => "high-contrast",
            ColorSchemeType::Custom => "custom",
        }
    }
}

/// Color Scheme
#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub id: String,
    pub name: String,
    pub scheme_type: ColorSchemeType,
    pub background: String,
    pub foreground: String,
    pub accent: String,
    pub secondary: String,
    pub success: String,
    pub warning: String,
    pub error: String,
    pub author: String,
}

impl ColorScheme {
    pub fn new(
        id: String,
        name: String,
        scheme_type: ColorSchemeType,
        background: String,
        foreground: String,
        accent: String,
        author: String,
    ) -> Self {
        Self {
            id,
            name,
            scheme_type,
            background,
            foreground,
            accent,
            secondary: String::from("#808080"),
            success: String::from("#4CAF50"),
            warning: String::from("#FF9800"),
            error: String::from("#F44336"),
            author,
        }
    }

    pub fn with_secondary(mut self, secondary: String) -> Self {
        self.secondary = secondary;
        self
    }

    pub fn with_success(mut self, success: String) -> Self {
        self.success = success;
        self
    }

    pub fn with_warning(mut self, warning: String) -> Self {
        self.warning = warning;
        self
    }

    pub fn with_error(mut self, error: String) -> Self {
        self.error = error;
        self
    }
}

/// Desktop Color Scheme Manager
pub struct DesktopColorSchemeManager {
    schemes: HashMap<String, ColorScheme>,
    current_scheme: Option<String>,
    counter: u32,
}

impl DesktopColorSchemeManager {
    pub fn new() -> Self {
        let mut manager = Self {
            schemes: HashMap::new(),
            current_scheme: None,
            counter: 1000,
        };

        // Add default schemes
        manager.add_default_schemes();

        manager
    }

    fn add_default_schemes(&mut self) {
        // Light scheme
        let light = ColorScheme::new(
            "light_1".to_string(),
            "Adwaita Light".to_string(),
            ColorSchemeType::Light,
            "#FFFFFF".to_string(),
            "#1E1E1E".to_string(),
            "#3584E4".to_string(),
            "GNOME Project".to_string(),
        )
        .with_secondary("#DADADA".to_string())
        .with_success("#26A269".to_string())
        .with_warning("#E5A50A".to_string())
        .with_error("#C01C28".to_string());

        // Dark scheme
        let dark = ColorScheme::new(
            "dark_1".to_string(),
            "Adwaita Dark".to_string(),
            ColorSchemeType::Dark,
            "#1E1E1E".to_string(),
            "#FFFFFF".to_string(),
            "#3584E4".to_string(),
            "GNOME Project".to_string(),
        )
        .with_secondary("#2E2E2E".to_string())
        .with_success("#26A269".to_string())
        .with_warning("#E5A50A".to_string())
        .with_error("#C01C28".to_string());

        // Mint Light
        let mint_light = ColorScheme::new(
            "light_2".to_string(),
            "Mint Light".to_string(),
            ColorSchemeType::Light,
            "#F8F8F8".to_string(),
            "#333333".to_string(),
            "#3F81F6".to_string(),
            "Linux Mint".to_string(),
        )
        .with_secondary("#E0E0E0".to_string())
        .with_success("#3BA55D".to_string())
        .with_warning("#E6A22C".to_string())
        .with_error("#D43838".to_string());

        // Mint Dark
        let mint_dark = ColorScheme::new(
            "dark_2".to_string(),
            "Mint Dark".to_string(),
            ColorSchemeType::Dark,
            "#262626".to_string(),
            "#E0E0E0".to_string(),
            "#3F81F6".to_string(),
            "Linux Mint".to_string(),
        )
        .with_secondary("#333333".to_string())
        .with_success("#3BA55D".to_string())
        .with_warning("#E6A22C".to_string())
        .with_error("#D43838".to_string());

        // High Contrast Light
        let hc_light = ColorScheme::new(
            "hc_light_1".to_string(),
            "High Contrast Light".to_string(),
            ColorSchemeType::HighContrast,
            "#FFFFFF".to_string(),
            "#000000".to_string(),
            "#0000FF".to_string(),
            "Universal".to_string(),
        )
        .with_secondary("#000000".to_string())
        .with_success("#008000".to_string())
        .with_warning("#FFA500".to_string())
        .with_error("#FF0000".to_string());

        // High Contrast Dark
        let hc_dark = ColorScheme::new(
            "hc_dark_1".to_string(),
            "High Contrast Dark".to_string(),
            ColorSchemeType::HighContrast,
            "#000000".to_string(),
            "#FFFFFF".to_string(),
            "#FFFF00".to_string(),
            "Universal".to_string(),
        )
        .with_secondary("#FFFFFF".to_string())
        .with_success("#00FF00".to_string())
        .with_warning("#FFFF00".to_string())
        .with_error("#FF0000".to_string());

        // Add schemes
        self.schemes.insert(light.id.clone(), light);
        self.schemes.insert(dark.id.clone(), dark);
        self.schemes.insert(mint_light.id.clone(), mint_light);
        self.schemes.insert(mint_dark.id.clone(), mint_dark);
        self.schemes.insert(hc_light.id.clone(), hc_light);
        self.schemes.insert(hc_dark.id.clone(), hc_dark);

        // Set default to dark
        self.current_scheme = Some("dark_1".to_string());
    }

    pub fn add_scheme(&mut self, scheme: ColorScheme) -> String {
        let id = format!("scheme_{}", self.counter);
        self.counter += 1;

        let scheme = ColorScheme {
            id: id.clone(),
            ..scheme
        };

        self.schemes.insert(id.clone(), scheme);
        id
    }

    pub fn remove_scheme(&mut self, id: &str) -> bool {
        // Don't remove if it's currently in use
        if Some(id.to_string()) == self.current_scheme {
            return false;
        }

        self.schemes.remove(id).is_some()
    }

    pub fn get_scheme(&self, id: &str) -> Option<&ColorScheme> {
        self.schemes.get(id)
    }

    pub fn get_schemes(&self) -> Vec<&ColorScheme> {
        self.schemes.values().collect()
    }

    pub fn get_schemes_by_type(&self, scheme_type: ColorSchemeType) -> Vec<&ColorScheme> {
        self.schemes
            .values()
            .filter(|s| s.scheme_type == scheme_type)
            .collect()
    }

    pub fn set_current_scheme(&mut self, id: &str) -> bool {
        if self.schemes.contains_key(id) {
            self.current_scheme = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_current_scheme(&self) -> Option<&ColorScheme> {
        self.current_scheme
            .as_ref()
            .and_then(|id| self.schemes.get(id))
    }

    pub fn toggle_light_dark(&mut self) -> bool {
        let current = self.get_current_scheme();
        if let Some(current) = current {
            let target_type = match current.scheme_type {
                ColorSchemeType::Light => ColorSchemeType::Dark,
                ColorSchemeType::Dark => ColorSchemeType::Light,
                _ => ColorSchemeType::Dark,
            };

            let schemes: Vec<String> = self
                .get_schemes_by_type(target_type)
                .iter()
                .map(|s| s.id.clone())
                .collect();

            if !schemes.is_empty() {
                self.set_current_scheme(&schemes[0]);
                return true;
            }
        }
        false
    }

    pub fn search_schemes(&self, query: &str) -> Vec<&ColorScheme> {
        self.schemes
            .values()
            .filter(|s| {
                s.name.to_lowercase().contains(&query.to_lowercase())
                    || s.author.to_lowercase().contains(&query.to_lowercase())
            })
            .collect()
    }

    pub fn get_statistics(&self) -> ColorSchemeStatistics {
        ColorSchemeStatistics {
            total_schemes: self.schemes.len(),
            light_schemes: self.get_schemes_by_type(ColorSchemeType::Light).len(),
            dark_schemes: self.get_schemes_by_type(ColorSchemeType::Dark).len(),
            high_contrast_schemes: self.get_schemes_by_type(ColorSchemeType::HighContrast).len(),
            custom_schemes: self.get_schemes_by_type(ColorSchemeType::Custom).len(),
            current_set: self.current_scheme.is_some(),
        }
    }
}

impl Default for DesktopColorSchemeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// ColorSchemeStatistics
#[derive(Debug, Clone, Copy)]
pub struct ColorSchemeStatistics {
    pub total_schemes: usize,
    pub light_schemes: usize,
    pub dark_schemes: usize,
    pub high_contrast_schemes: usize,
    pub custom_schemes: usize,
    pub current_set: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_manager_state() {
        let manager = DesktopColorSchemeManager::new();
        let stats = manager.get_statistics();

        assert!(stats.total_schemes >= 6);
        assert!(stats.light_schemes >= 2);
        assert!(stats.dark_schemes >= 2);
        assert!(stats.high_contrast_schemes >= 2);
        assert!(stats.current_set);
    }

    #[test]
    fn test_add_scheme() {
        let mut manager = DesktopColorSchemeManager::new();
        let initial_count = manager.get_schemes().len();

        let new_scheme = ColorScheme::new(
            "custom".to_string(),
            "Custom Scheme".to_string(),
            ColorSchemeType::Custom,
            "#000000".to_string(),
            "#FFFFFF".to_string(),
            "#FF0000".to_string(),
            "Author".to_string(),
        );

        let id = manager.add_scheme(new_scheme);
        assert!(manager.get_scheme(&id).is_some());
        assert_eq!(manager.get_schemes().len(), initial_count + 1);
    }

    #[test]
    fn test_remove_scheme() {
        let mut manager = DesktopColorSchemeManager::new();

        // Add a custom scheme
        let new_scheme = ColorScheme::new(
            "custom".to_string(),
            "Custom Scheme".to_string(),
            ColorSchemeType::Custom,
            "#000000".to_string(),
            "#FFFFFF".to_string(),
            "#FF0000".to_string(),
            "Author".to_string(),
        );

        let id = manager.add_scheme(new_scheme);
        assert!(manager.remove_scheme(&id));
        assert!(manager.get_scheme(&id).is_none());
    }

    #[test]
    fn test_remove_current_scheme() {
        let mut manager = DesktopColorSchemeManager::new();

        // Try to remove current scheme (should fail)
        let current_id = manager.get_current_scheme().unwrap().id.clone();
        let result = manager.remove_scheme(&current_id);
        assert!(!result);
    }

    #[test]
    fn test_set_current_scheme() {
        let mut manager = DesktopColorSchemeManager::new();
        let schemes = manager.get_schemes();

        if schemes.len() > 1 {
            let new_scheme_id = schemes[1].id.clone();
            assert!(manager.set_current_scheme(&new_scheme_id));
            assert_eq!(
                manager.get_current_scheme().unwrap().id,
                new_scheme_id
            );
        }
    }

    #[test]
    fn test_toggle_light_dark() {
        let mut manager = DesktopColorSchemeManager::new();
        let initial = manager.get_current_scheme().unwrap().scheme_type;

        let result = manager.toggle_light_dark();
        assert!(result);

        let toggled = manager.get_current_scheme().unwrap().scheme_type;
        assert_ne!(initial, toggled);
    }

    #[test]
    fn test_filter_by_type() {
        let manager = DesktopColorSchemeManager::new();
        let light_schemes = manager.get_schemes_by_type(ColorSchemeType::Light);
        let dark_schemes = manager.get_schemes_by_type(ColorSchemeType::Dark);

        assert!(!light_schemes.is_empty());
        assert!(!dark_schemes.is_empty());

        for scheme in light_schemes {
            assert_eq!(scheme.scheme_type, ColorSchemeType::Light);
        }

        for scheme in dark_schemes {
            assert_eq!(scheme.scheme_type, ColorSchemeType::Dark);
        }
    }

    #[test]
    fn test_search_schemes() {
        let manager = DesktopColorSchemeManager::new();
        let results = manager.search_schemes("Adwaita");

        assert!(!results.is_empty());
        for scheme in results {
            assert!(scheme.name.to_lowercase().contains("adwaita"));
        }
    }

    #[test]
    fn test_custom_scheme_colors() {
        let mut manager = DesktopColorSchemeManager::new();

        let custom = ColorScheme::new(
            "custom".to_string(),
            "Custom".to_string(),
            ColorSchemeType::Custom,
            "#111111".to_string(),
            "#EEEEEE".to_string(),
            "#123456".to_string(),
            "Author".to_string(),
        )
        .with_secondary("#222222".to_string())
        .with_success("#00FF00".to_string())
        .with_warning("#FFFF00".to_string())
        .with_error("#FF0000".to_string());

        let id = manager.add_scheme(custom);
        let scheme = manager.get_scheme(&id).unwrap();

        assert_eq!(scheme.background, "#111111");
        assert_eq!(scheme.foreground, "#EEEEEE");
        assert_eq!(scheme.accent, "#123456");
        assert_eq!(scheme.secondary, "#222222");
        assert_eq!(scheme.success, "#00FF00");
        assert_eq!(scheme.warning, "#FFFF00");
        assert_eq!(scheme.error, "#FF0000");
    }
}
