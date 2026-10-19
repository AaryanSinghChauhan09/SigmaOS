//! SigmaOS On-Screen Display Manager
//!
//! Inspired by: Omarchy OSD (omancy-osd - volume, brightness, and status overlays)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! SECURE_DEFAULTS, DIRECT_DEVICE_CONTROL, USER_FEEDBACK, TRANSPARENCY.

/// OSD type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OsdType {
    Volume,
    Brightness,
    Media,
    Network,
    Battery,
    Custom(String),
}

/// OSD entry
#[derive(Debug, Clone)]
pub struct OsdEntry {
    pub id: u64,
    pub osd_type: OsdType,
    pub value: f32,
    pub max_value: f32,
    pub text: Option<String>,
    pub icon: Option<String>,
    pub duration_ms: u64,
    pub shown_at: u64,
    pub visible: bool,
}

/// OSD manager
pub struct OsdManager {
    pub entries: Vec<OsdEntry>,
    pub next_id: u64,
    pub max_entries: usize,
    pub default_duration_ms: u64,
}

impl OsdManager {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_id: 1,
            max_entries: 10,
            default_duration_ms: 3000,
        }
    }

    /// Show volume OSD
    pub fn show_volume(&mut self, volume: f32, max_volume: f32) -> u64 {
        self.show(
            OsdType::Volume,
            volume,
            max_volume,
            None,
            Some("🔊".to_string()),
        )
    }

    /// Show brightness OSD
    pub fn show_brightness(&mut self, brightness: f32, max_brightness: f32) -> u64 {
        self.show(
            OsdType::Brightness,
            brightness,
            max_brightness,
            None,
            Some("💡".to_string()),
        )
    }

    /// Show media OSD
    pub fn show_media(&mut self, title: String, artist: Option<String>) -> u64 {
        let text = match artist {
            Some(a) => Some(format!("{} - {}", title, a)),
            None => Some(title),
        };
        self.show(OsdType::Media, 0.0, 0.0, text, Some("🎵".to_string()))
    }

    /// Show network OSD
    pub fn show_network(&mut self, signal: f32, max_signal: f32) -> u64 {
        self.show(
            OsdType::Network,
            signal,
            max_signal,
            None,
            Some("🌐".to_string()),
        )
    }

    /// Show battery OSD
    pub fn show_battery(&mut self, percent: f32, charging: bool) -> u64 {
        let icon = if charging { Some("🔌".to_string()) } else { Some("🔋".to_string()) };
        self.show(
            OsdType::Battery,
            percent,
            100.0,
            Some(format!("{:.0}%", percent)),
            icon,
        )
    }

    /// Show custom OSD
    pub fn show_custom(
        &mut self,
        name: String,
        value: f32,
        max_value: f32,
        text: Option<String>,
        icon: Option<String>,
    ) -> u64 {
        self.show(OsdType::Custom(name), value, max_value, text, icon)
    }

    fn show(
        &mut self,
        osd_type: OsdType,
        value: f32,
        max_value: f32,
        text: Option<String>,
        icon: Option<String>,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let entry = OsdEntry {
            id,
            osd_type,
            value,
            max_value,
            text,
            icon,
            duration_ms: self.default_duration_ms,
            shown_at: now_secs(),
            visible: true,
        };

        self.entries.push(entry);
        self.trim_entries();
        id
    }

    /// Hide an OSD entry
    pub fn hide(&mut self, id: u64) -> Result<(), String> {
        let entry = self
            .entries
            .iter_mut()
            .find(|e| e.id == id)
            .ok_or_else(|| format!("OSD entry {} not found", id))?;
        entry.visible = false;
        Ok(())
    }

    /// Hide all OSD entries
    pub fn hide_all(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.visible = false;
        }
    }

    /// Get visible entries
    pub fn get_visible(&self) -> Vec<&OsdEntry> {
        self.entries.iter().filter(|e| e.visible).collect()
    }

    /// Get entry by ID
    pub fn get(&self, id: u64) -> Option<&OsdEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    /// Set default duration
    pub fn set_default_duration(&mut self, ms: u64) {
        self.default_duration_ms = ms;
    }

    /// Entry count
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    fn trim_entries(&mut self) {
        while self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }
}

impl Default for OsdManager {
    fn default() -> Self {
        Self::new()
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_volume() {
        let mut mgr = OsdManager::new();
        let id = mgr.show_volume(75.0, 100.0);
        assert_eq!(id, 1);
        assert_eq!(mgr.entry_count(), 1);
        let entry = mgr.get(id).unwrap();
        assert!(matches!(entry.osd_type, OsdType::Volume));
        assert!(entry.visible);
    }

    #[test]
    fn test_show_brightness() {
        let mut mgr = OsdManager::new();
        let id = mgr.show_brightness(60.0, 100.0);
        let entry = mgr.get(id).unwrap();
        assert!(matches!(entry.osd_type, OsdType::Brightness));
    }

    #[test]
    fn test_show_media() {
        let mut mgr = OsdManager::new();
        let id = mgr.show_media("Song Title".to_string(), Some("Artist".to_string()));
        let entry = mgr.get(id).unwrap();
        assert_eq!(entry.text, Some("Song Title - Artist".to_string()));
    }

    #[test]
    fn test_show_battery() {
        let mut mgr = OsdManager::new();
        let id = mgr.show_battery(85.0, true);
        let entry = mgr.get(id).unwrap();
        assert_eq!(entry.icon, Some("🔌".to_string()));
    }

    #[test]
    fn test_hide() {
        let mut mgr = OsdManager::new();
        let id = mgr.show_volume(50.0, 100.0);
        assert!(mgr.hide(id).is_ok());
        assert!(!mgr.get(id).unwrap().visible);
    }

    #[test]
    fn test_hide_all() {
        let mut mgr = OsdManager::new();
        mgr.show_volume(50.0, 100.0);
        mgr.show_brightness(70.0, 100.0);
        mgr.hide_all();
        assert!(mgr.get_visible().is_empty());
    }

    #[test]
    fn test_get_visible() {
        let mut mgr = OsdManager::new();
        mgr.show_volume(50.0, 100.0);
        mgr.show_brightness(70.0, 100.0);
        assert_eq!(mgr.get_visible().len(), 2);
    }
}