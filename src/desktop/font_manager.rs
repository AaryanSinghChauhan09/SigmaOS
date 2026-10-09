// Desktop Font Manager
// Linux Mint & Omarchy inspiration for comprehensive font management

use std::collections::HashMap;

/// Font Family
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontFamily {
    pub name: String,
    pub generic: bool,
    pub styles: Vec<FontStyle>,
}

impl FontFamily {
    pub fn new(name: String, generic: bool) -> Self {
        Self {
            name,
            generic,
            styles: Vec::new(),
        }
    }

    pub fn add_style(&mut self, style: FontStyle) {
        self.styles.push(style);
    }
}

/// Font Style
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontStyle {
    pub name: String,
    pub weight: FontWeight,
    pub slant: FontSlant,
    pub width: FontWidth,
    pub file_path: String,
}

impl FontStyle {
    pub fn new(
        name: String,
        weight: FontWeight,
        slant: FontSlant,
        width: FontWidth,
        file_path: String,
    ) -> Self {
        Self {
            name,
            weight,
            slant,
            width,
            file_path,
        }
    }
}

/// Font Weight
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Regular,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

impl FontWeight {
    pub fn as_str(&self) -> &'static str {
        match self {
            FontWeight::Thin => "thin",
            FontWeight::ExtraLight => "extralight",
            FontWeight::Light => "light",
            FontWeight::Regular => "regular",
            FontWeight::Medium => "medium",
            FontWeight::SemiBold => "semibold",
            FontWeight::Bold => "bold",
            FontWeight::ExtraBold => "extrabold",
            FontWeight::Black => "black",
        }
    }

    pub fn as_number(&self) -> u32 {
        match self {
            FontWeight::Thin => 100,
            FontWeight::ExtraLight => 200,
            FontWeight::Light => 300,
            FontWeight::Regular => 400,
            FontWeight::Medium => 500,
            FontWeight::SemiBold => 600,
            FontWeight::Bold => 700,
            FontWeight::ExtraBold => 800,
            FontWeight::Black => 900,
        }
    }
}

/// Font Slant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontSlant {
    Normal,
    Italic,
    Oblique,
}

impl FontSlant {
    pub fn as_str(&self) -> &'static str {
        match self {
            FontSlant::Normal => "normal",
            FontSlant::Italic => "italic",
            FontSlant::Oblique => "oblique",
        }
    }
}

/// Font Width
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWidth {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

impl FontWidth {
    pub fn as_str(&self) -> &'static str {
        match self {
            FontWidth::UltraCondensed => "ultracondensed",
            FontWidth::ExtraCondensed => "extracondensed",
            FontWidth::Condensed => "condensed",
            FontWidth::SemiCondensed => "semicondensed",
            FontWidth::Normal => "normal",
            FontWidth::SemiExpanded => "semiexpanded",
            FontWidth::Expanded => "expanded",
            FontWidth::ExtraExpanded => "extraexpanded",
            FontWidth::UltraExpanded => "ultraexpanded",
        }
    }
}

/// Font Usage Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontUsageType {
    Default,
    Monospace,
    SansSerif,
    Serif,
    Document,
    Interface,
    Title,
    Heading,
}

/// Desktop Font Manager
pub struct DesktopFontManager {
    families: HashMap<String, FontFamily>,
    default_font: Option<String>,
    monospace_font: Option<String>,
    sans_serif_font: Option<String>,
    serif_font: Option<String>,
    document_font: Option<String>,
    interface_font: Option<String>,
    title_font: Option<String>,
    heading_font: Option<String>,
    font_size: u32, // 6-72
    counter: u32,
}

impl DesktopFontManager {
    pub fn new() -> Self {
        let mut manager = Self {
            families: HashMap::new(),
            default_font: None,
            monospace_font: None,
            sans_serif_font: None,
            serif_font: None,
            document_font: None,
            interface_font: None,
            title_font: None,
            heading_font: None,
            font_size: 12,
            counter: 1000,
        };

        // Add default fonts
        manager.add_default_fonts();

        manager
    }

    fn add_default_fonts(&mut self) {
        // Add sans-serif font
        let mut sans_serif = FontFamily::new("Sans".to_string(), true);
        sans_serif.add_style(FontStyle::new(
            "Regular".to_string(),
            FontWeight::Regular,
            FontSlant::Normal,
            FontWidth::Normal,
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf".to_string(),
        ));
        sans_serif.add_style(FontStyle::new(
            "Bold".to_string(),
            FontWeight::Bold,
            FontSlant::Normal,
            FontWidth::Normal,
            "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf".to_string(),
        ));
        sans_serif.add_style(FontStyle::new(
            "Italic".to_string(),
            FontWeight::Regular,
            FontSlant::Italic,
            FontWidth::Normal,
            "/usr/share/fonts/truetype/dejavu/DejaVuSans-Oblique.ttf".to_string(),
        ));
        self.families.insert("sans".to_string(), sans_serif);

        // Add serif font
        let mut serif = FontFamily::new("Serif".to_string(), true);
        serif.add_style(FontStyle::new(
            "Regular".to_string(),
            FontWeight::Regular,
            FontSlant::Normal,
            FontWidth::Normal,
            "/usr/share/fonts/truetype/dejavu/DejaVuSerif.ttf".to_string(),
        ));
        serif.add_style(FontStyle::new(
            "Bold".to_string(),
            FontWeight::Bold,
            FontSlant::Normal,
            FontWidth::Normal,
            "/usr/share/fonts/truetype/dejavu/DejaVuSerif-Bold.ttf".to_string(),
        ));
        self.families.insert("serif".to_string(), serif);

        // Add monospace font
        let mut monospace = FontFamily::new("Monospace".to_string(), true);
        monospace.add_style(FontStyle::new(
            "Regular".to_string(),
            FontWeight::Regular,
            FontSlant::Normal,
            FontWidth::Normal,
            "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf".to_string(),
        ));
        monospace.add_style(FontStyle::new(
            "Bold".to_string(),
            FontWeight::Bold,
            FontSlant::Normal,
            FontWidth::Normal,
            "/usr/share/fonts/truetype/dejavu/DejaVuSansMono-Bold.ttf".to_string(),
        ));
        self.families.insert("monospace".to_string(), monospace);

        // Set defaults
        self.default_font = Some("sans".to_string());
        self.monospace_font = Some("monospace".to_string());
        self.sans_serif_font = Some("sans".to_string());
        self.serif_font = Some("serif".to_string());
        self.document_font = Some("serif".to_string());
        self.interface_font = Some("sans".to_string());
        self.title_font = Some("sans".to_string());
        self.heading_font = Some("sans".to_string());
    }

    pub fn add_family(&mut self, family: FontFamily) -> String {
        let id = format!("family_{}", self.counter);
        self.counter += 1;

        let family = FontFamily {
            name: id.clone(),
            ..family
        };

        self.families.insert(id.clone(), family);
        id
    }

    pub fn remove_family(&mut self, id: &str) -> bool {
        if Some(id.to_string()) == self.default_font
            || Some(id.to_string()) == self.monospace_font
            || Some(id.to_string()) == self.sans_serif_font
            || Some(id.to_string()) == self.serif_font
        {
            return false;
        }
        self.families.remove(id).is_some()
    }

    pub fn get_family(&self, id: &str) -> Option<&FontFamily> {
        self.families.get(id)
    }

    pub fn get_families(&self) -> Vec<&FontFamily> {
        self.families.values().collect()
    }

    pub fn set_default_font(&mut self, id: &str) -> bool {
        if self.families.contains_key(id) {
            self.default_font = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_default_font(&self) -> Option<&FontFamily> {
        self.default_font
            .as_ref()
            .and_then(|id| self.families.get(id))
    }

    pub fn set_font_for_usage(&mut self, usage: FontUsageType, id: &str) -> bool {
        if !self.families.contains_key(id) {
            return false;
        }

        match usage {
            FontUsageType::Default => self.default_font = Some(id.to_string()),
            FontUsageType::Monospace => self.monospace_font = Some(id.to_string()),
            FontUsageType::SansSerif => self.sans_serif_font = Some(id.to_string()),
            FontUsageType::Serif => self.serif_font = Some(id.to_string()),
            FontUsageType::Document => self.document_font = Some(id.to_string()),
            FontUsageType::Interface => self.interface_font = Some(id.to_string()),
            FontUsageType::Title => self.title_font = Some(id.to_string()),
            FontUsageType::Heading => self.heading_font = Some(id.to_string()),
        }
        true
    }

    pub fn get_font_for_usage(&self, usage: FontUsageType) -> Option<&FontFamily> {
        let id = match usage {
            FontUsageType::Default => &self.default_font,
            FontUsageType::Monospace => &self.monospace_font,
            FontUsageType::SansSerif => &self.sans_serif_font,
            FontUsageType::Serif => &self.serif_font,
            FontUsageType::Document => &self.document_font,
            FontUsageType::Interface => &self.interface_font,
            FontUsageType::Title => &self.title_font,
            FontUsageType::Heading => &self.heading_font,
        };
        id.as_ref().and_then(|id| self.families.get(id))
    }

    pub fn set_font_size(&mut self, size: u32) {
        self.font_size = size.clamp(6, 72);
    }

    pub fn get_font_size(&self) -> u32 {
        self.font_size
    }

    pub fn search_families(&self, query: &str) -> Vec<&FontFamily> {
        self.families
            .values()
            .filter(|f| f.name.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }

    pub fn get_statistics(&self) -> FontManagerStatistics {
        FontManagerStatistics {
            total_families: self.families.len(),
            default_font_set: self.default_font.is_some(),
            font_size: self.font_size,
        }
    }
}

impl Default for DesktopFontManager {
    fn default() -> Self {
        Self::new()
    }
}

/// FontManagerStatistics
#[derive(Debug, Clone, Copy)]
pub struct FontManagerStatistics {
    pub total_families: usize,
    pub default_font_set: bool,
    pub font_size: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_manager_state() {
        let manager = DesktopFontManager::new();
        let stats = manager.get_statistics();

        assert_eq!(stats.total_families, 3);
        assert!(stats.default_font_set);
        assert_eq!(stats.font_size, 12);
    }

    #[test]
    fn test_add_family() {
        let mut manager = DesktopFontManager::new();
        let initial_count = manager.get_families().len();

        let family = FontFamily::new("Custom".to_string(), false);

        let id = manager.add_family(family);
        assert!(manager.get_family(&id).is_some());
        assert_eq!(manager.get_families().len(), initial_count + 1);
    }

    #[test]
    fn test_remove_family() {
        let mut manager = DesktopFontManager::new();

        let family = FontFamily::new("Custom".to_string(), false);

        let id = manager.add_family(family);
        assert!(manager.remove_family(&id));
        assert!(manager.get_family(&id).is_none());
    }

    #[test]
    fn test_remove_default_family() {
        let mut manager = DesktopFontManager::new();

        // Try to remove default font (should fail)
        let default_name = manager.get_default_font().unwrap().name.clone();
        let result = manager.remove_family(&default_name);
        assert!(!result);
    }

    #[test]
    fn test_set_default_font() {
        let mut manager = DesktopFontManager::new();

        let family = FontFamily::new("Custom".to_string(), false);

        let id = manager.add_family(family);
        assert!(manager.set_default_font(&id));

        let default = manager.get_default_font().unwrap();
        assert_eq!(default.name, id);
    }

    #[test]
    fn test_font_for_usage() {
        let mut manager = DesktopFontManager::new();

        let family = FontFamily::new("Custom".to_string(), false);

        let id = manager.add_family(family);
        assert!(manager.set_font_for_usage(FontUsageType::Monospace, &id));

        let font = manager
            .get_font_for_usage(FontUsageType::Monospace)
            .unwrap();
        assert_eq!(font.name, id);
    }

    #[test]
    fn test_font_size() {
        let mut manager = DesktopFontManager::new();

        manager.set_font_size(16);
        assert_eq!(manager.get_font_size(), 16);

        manager.set_font_size(100);
        assert_eq!(manager.get_font_size(), 72);

        manager.set_font_size(4);
        assert_eq!(manager.get_font_size(), 6);
    }

    #[test]
    fn test_search_families() {
        let manager = DesktopFontManager::new();

        let results = manager.search_families("sans");
        assert!(!results.is_empty());

        let results = manager.search_families("nonexistent");
        assert!(results.is_empty());
    }
}
