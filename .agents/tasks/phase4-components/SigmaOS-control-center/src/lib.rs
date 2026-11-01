//! SigmaOS-control-center: Unified system control center library.
//!
//! This crate provides the panel registration, discovery, and host infrastructure
//! used by the SigmaOS control center application.

use std::collections::HashMap;
use std::fmt;

// ---------------------------------------------------------------------------
// Panel metadata
// ---------------------------------------------------------------------------

/// Categories that a settings panel can belong to.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PanelCategory {
    System,
    Hardware,
    Network,
    PersonalInfo,
    Appearance,
    Accessibility,
    Other,
}

impl fmt::Display for PanelCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PanelCategory::System => write!(f, "System"),
            PanelCategory::Hardware => write!(f, "Hardware"),
            PanelCategory::Network => write!(f, "Network"),
            PanelCategory::PersonalInfo => write!(f, "Personal Info"),
            PanelCategory::Appearance => write!(f, "Appearance"),
            PanelCategory::Accessibility => write!(f, "Accessibility"),
            PanelCategory::Other => write!(f, "Other"),
        }
    }
}

/// Static metadata describing a settings panel.
#[derive(Debug, Clone)]
pub struct PanelInfo {
    /// Unique identifier used in deep-link URIs (e.g. "display", "sound").
    pub id: &'static str,
    /// Human-readable panel name shown in the panel list.
    pub title: &'static str,
    /// Short description used in search results.
    pub description: &'static str,
    /// Icon name from the system icon theme.
    pub icon: &'static str,
    /// Category grouping for the panel list sidebar.
    pub category: PanelCategory,
    /// Keywords that boost search match ranking.
    pub keywords: &'static [&'static str],
    /// Whether PolicyKit elevation is required to open this panel.
    pub requires_admin: bool,
}

// ---------------------------------------------------------------------------
// Panel registry
// ---------------------------------------------------------------------------

/// Registry of all available settings panels.
#[derive(Debug, Default)]
pub struct PanelRegistry {
    panels: HashMap<&'static str, PanelInfo>,
}

impl PanelRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a panel. Returns `false` if the id was already registered.
    pub fn register(&mut self, panel: PanelInfo) -> bool {
        if self.panels.contains_key(panel.id) {
            return false;
        }
        self.panels.insert(panel.id, panel);
        true
    }

    /// Look up a panel by its unique id.
    pub fn get(&self, id: &str) -> Option<&PanelInfo> {
        self.panels.get(id)
    }

    /// Return all panels in the given category, sorted by title.
    pub fn by_category(&self, category: &PanelCategory) -> Vec<&PanelInfo> {
        let mut result: Vec<&PanelInfo> = self
            .panels
            .values()
            .filter(|p| &p.category == category)
            .collect();
        result.sort_by_key(|p| p.title);
        result
    }

    /// Search panels by query string, matching against title, description, and keywords.
    pub fn search(&self, query: &str) -> Vec<&PanelInfo> {
        let q = query.to_lowercase();
        let mut results: Vec<&PanelInfo> = self
            .panels
            .values()
            .filter(|p| {
                p.title.to_lowercase().contains(&q)
                    || p.description.to_lowercase().contains(&q)
                    || p.keywords.iter().any(|kw| kw.to_lowercase().contains(&q))
            })
            .collect();
        results.sort_by_key(|p| p.title);
        results
    }

    /// Total number of registered panels.
    pub fn len(&self) -> usize {
        self.panels.len()
    }

    /// Returns `true` if no panels are registered.
    pub fn is_empty(&self) -> bool {
        self.panels.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Built-in panel definitions
// ---------------------------------------------------------------------------

/// Return a registry pre-populated with the built-in SigmaOS panels.
pub fn default_registry() -> PanelRegistry {
    let mut reg = PanelRegistry::new();

    reg.register(PanelInfo {
        id: "display",
        title: "Display",
        description: "Configure monitors, resolution, and display arrangement.",
        icon: "video-display",
        category: PanelCategory::Hardware,
        keywords: &["screen", "monitor", "resolution", "brightness", "hdmi"],
        requires_admin: false,
    });

    reg.register(PanelInfo {
        id: "sound",
        title: "Sound",
        description: "Adjust volume, output and input devices, and alert sounds.",
        icon: "audio-speakers",
        category: PanelCategory::Hardware,
        keywords: &["audio", "volume", "microphone", "speakers", "bluetooth"],
        requires_admin: false,
    });

    reg.register(PanelInfo {
        id: "network",
        title: "Network",
        description: "Manage Wi-Fi, wired connections, VPN, and proxy settings.",
        icon: "network-wired",
        category: PanelCategory::Network,
        keywords: &["wifi", "ethernet", "vpn", "proxy", "bluetooth", "internet"],
        requires_admin: true,
    });

    reg.register(PanelInfo {
        id: "users",
        title: "Users",
        description: "Add, remove, and configure user accounts and groups.",
        icon: "system-users",
        category: PanelCategory::System,
        keywords: &["account", "password", "login", "groups", "admin"],
        requires_admin: true,
    });

    reg.register(PanelInfo {
        id: "appearance",
        title: "Appearance",
        description: "Change the desktop theme, wallpaper, and fonts.",
        icon: "applications-graphics",
        category: PanelCategory::Appearance,
        keywords: &["theme", "wallpaper", "font", "icons", "cursor"],
        requires_admin: false,
    });

    reg
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_retrieve_panel() {
        let reg = default_registry();
        let panel = reg.get("display").expect("display panel should exist");
        assert_eq!(panel.title, "Display");
    }

    #[test]
    fn search_returns_matching_panels() {
        let reg = default_registry();
        let results = reg.search("wifi");
        assert!(results.iter().any(|p| p.id == "network"));
    }

    #[test]
    fn search_case_insensitive() {
        let reg = default_registry();
        let results = reg.search("VOLUME");
        assert!(results.iter().any(|p| p.id == "sound"));
    }

    #[test]
    fn duplicate_registration_returns_false() {
        let mut reg = PanelRegistry::new();
        let panel = PanelInfo {
            id: "test",
            title: "Test",
            description: "A test panel.",
            icon: "dialog-information",
            category: PanelCategory::Other,
            keywords: &[],
            requires_admin: false,
        };
        assert!(reg.register(panel.clone()));
        assert!(!reg.register(panel));
    }

    #[test]
    fn by_category_returns_sorted_results() {
        let reg = default_registry();
        let hw = reg.by_category(&PanelCategory::Hardware);
        // Display comes before Sound alphabetically
        assert_eq!(hw[0].id, "display");
        assert_eq!(hw[1].id, "sound");
    }
}
