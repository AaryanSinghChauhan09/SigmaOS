use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

/// Standard folder color options inspired by Linux Mint Folder Color & Ubuntu Yaru
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FolderColor {
    Blue,
    Green,
    Red,
    Orange,
    Purple,
    Pink,
    Teal,
    Yellow,
    Grey,
    CustomHex(String),
}

impl FolderColor {
    pub fn to_hex(&self) -> String {
        match self {
            FolderColor::Blue => "#3584E4".to_string(),
            FolderColor::Green => "#2EC27E".to_string(),
            FolderColor::Red => "#E01B24".to_string(),
            FolderColor::Orange => "#FF7800".to_string(),
            FolderColor::Purple => "#9141AC".to_string(),
            FolderColor::Pink => "#F66151".to_string(),
            FolderColor::Teal => "#129893".to_string(),
            FolderColor::Yellow => "#F6D32D".to_string(),
            FolderColor::Grey => "#77767B".to_string(),
            FolderColor::CustomHex(hex) => hex.clone(),
        }
    }
}

/// Emblem overlays for folders (Papirus / Elementary OS style)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FolderEmblem {
    None,
    Documents,
    Downloads,
    Music,
    Pictures,
    Videos,
    Shared,
    Lock,
    Star,
    Custom(String),
}

/// Custom Folder Appearance Config
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FolderCustomization {
    pub color: FolderColor,
    pub emblem: FolderEmblem,
    pub inherit_to_subfolders: bool,
}

/// Folder Color Switcher Engine (Nautilus / Nemo / Thunar extension parity)
pub struct FolderColorSwitcherEngine {
    pub default_color: FolderColor,
    pub custom_folders: BTreeMap<String, FolderCustomization>,
}

impl FolderColorSwitcherEngine {
    pub fn new() -> Self {
        Self {
            default_color: FolderColor::Blue,
            custom_folders: BTreeMap::new(),
        }
    }

    pub fn set_folder_color(
        &mut self,
        folder_path: &str,
        color: FolderColor,
        emblem: FolderEmblem,
        inherit: bool,
    ) {
        let path = folder_path.trim_end_matches('/').to_string();
        self.custom_folders.insert(
            path,
            FolderCustomization {
                color,
                emblem,
                inherit_to_subfolders: inherit,
            },
        );
    }

    pub fn get_folder_customization(&self, folder_path: &str) -> FolderCustomization {
        let clean_path = folder_path.trim_end_matches('/');

        // 1. Exact match
        if let Some(config) = self.custom_folders.get(clean_path) {
            return config.clone();
        }

        // 2. Parent folder inheritance check
        let mut parent = clean_path;
        while let Some(idx) = parent.rfind('/') {
            if idx == 0 {
                break;
            }
            parent = &parent[..idx];
            if let Some(config) = self.custom_folders.get(parent) {
                if config.inherit_to_subfolders {
                    return config.clone();
                }
            }
        }

        FolderCustomization {
            color: self.default_color.clone(),
            emblem: FolderEmblem::None,
            inherit_to_subfolders: false,
        }
    }

    pub fn reset_folder_color(&mut self, folder_path: &str) -> bool {
        let clean_path = folder_path.trim_end_matches('/');
        self.custom_folders.remove(clean_path).is_some()
    }

    pub fn generate_folder_css_theme(&self, folder_path: &str) -> String {
        let custom = self.get_folder_customization(folder_path);
        let hex = custom.color.to_hex();
        format!(
            ".folder-icon[path=\"{}\"] {{ fill: {}; stroke: {}; }}",
            folder_path, hex, hex
        )
    }
}

impl Default for FolderColorSwitcherEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_folder_color_switcher_engine() {
        let mut engine = FolderColorSwitcherEngine::new();
        engine.set_folder_color(
            "/home/user/Projects",
            FolderColor::Green,
            FolderEmblem::Star,
            true,
        );

        // Test direct lookup
        let direct = engine.get_folder_customization("/home/user/Projects");
        assert_eq!(direct.color, FolderColor::Green);
        assert_eq!(direct.emblem, FolderEmblem::Star);

        // Test subfolder inheritance
        let sub = engine.get_folder_customization("/home/user/Projects/SigmaOS/src");
        assert_eq!(sub.color, FolderColor::Green);

        // Test non-inherited path
        let unrelated = engine.get_folder_customization("/home/user/Documents");
        assert_eq!(unrelated.color, FolderColor::Blue);

        // Test custom hex color
        engine.set_folder_color(
            "/home/user/Custom",
            FolderColor::CustomHex("#AABBCC".to_string()),
            FolderEmblem::Lock,
            false,
        );
        let custom = engine.get_folder_customization("/home/user/Custom");
        assert_eq!(custom.color.to_hex(), "#AABBCC");
        assert_eq!(custom.emblem, FolderEmblem::Lock);

        // Test reset folder color
        assert!(engine.reset_folder_color("/home/user/Custom"));
        let reset_res = engine.get_folder_customization("/home/user/Custom");
        assert_eq!(reset_res.color, FolderColor::Blue);

        // Test CSS generation
        let css = engine.generate_folder_css_theme("/home/user/Projects");
        assert!(css.contains("#2EC27E"));
    }
}
