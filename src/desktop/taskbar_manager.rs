// SigmaOS Taskbar Manager
// Inspired by Linux Mint's panel/taskbar and Omarchy's taskbar utilities

use std::collections::HashMap;

/// Taskbar position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopTaskbarPosition {
    Top,
    Bottom,
    Left,
    Right,
}

impl DesktopTaskbarPosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopTaskbarPosition::Top => "Top",
            DesktopTaskbarPosition::Bottom => "Bottom",
            DesktopTaskbarPosition::Left => "Left",
            DesktopTaskbarPosition::Right => "Right",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "top" => Some(DesktopTaskbarPosition::Top),
            "bottom" => Some(DesktopTaskbarPosition::Bottom),
            "left" => Some(DesktopTaskbarPosition::Left),
            "right" => Some(DesktopTaskbarPosition::Right),
            _ => None,
        }
    }
}

/// Taskbar item type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopTaskbarItemType {
    Launcher,
    Running,
    Pinned,
    SystemTray,
    StatusIndicator,
}

impl DesktopTaskbarItemType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopTaskbarItemType::Launcher => "Launcher",
            DesktopTaskbarItemType::Running => "Running",
            DesktopTaskbarItemType::Pinned => "Pinned",
            DesktopTaskbarItemType::SystemTray => "System Tray",
            DesktopTaskbarItemType::StatusIndicator => "Status Indicator",
        }
    }
}

/// Taskbar item
#[derive(Debug, Clone)]
pub struct DesktopTaskbarItem {
    pub id: String,
    pub name: String,
    pub item_type: DesktopTaskbarItemType,
    pub icon: Option<String>,
    pub is_active: bool,
    pub is_minimized: bool,
    pub window_count: u32,
}

impl DesktopTaskbarItem {
    pub fn new(id: String, name: String, item_type: DesktopTaskbarItemType) -> Self {
        DesktopTaskbarItem {
            id,
            name,
            item_type,
            icon: None,
            is_active: false,
            is_minimized: false,
            window_count: 0,
        }
    }

    pub fn set_icon(&mut self, icon: String) {
        self.icon = Some(icon);
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn set_minimized(&mut self, minimized: bool) {
        self.is_minimized = minimized;
    }

    pub fn set_window_count(&mut self, count: u32) {
        self.window_count = count;
    }
}

/// Taskbar Manager
pub struct DesktopTaskbarManager {
    items: HashMap<String, DesktopTaskbarItem>,
    position: DesktopTaskbarPosition,
    auto_hide: bool,
    size: u32,
    icon_size: u32,
    show_system_tray: bool,
    show_status_indicators: bool,
    next_item_id: u32,
}

impl DesktopTaskbarManager {
    pub fn new() -> Self {
        let mut manager = DesktopTaskbarManager {
            items: HashMap::new(),
            position: DesktopTaskbarPosition::Bottom,
            auto_hide: false,
            size: 48,
            icon_size: 32,
            show_system_tray: true,
            show_status_indicators: true,
            next_item_id: 1,
        };

        // Add default items
        manager.add_default_items();

        manager
    }

    fn add_default_items(&mut self) {
        // Application launcher
        let mut launcher = DesktopTaskbarItem::new(
            format!("item_{}", self.next_item_id),
            "Launcher".to_string(),
            DesktopTaskbarItemType::Launcher,
        );
        launcher.set_icon("applications-menu".to_string());
        self.items.insert(launcher.id.clone(), launcher);
        self.next_item_id += 1;

        // Terminal
        let mut terminal = DesktopTaskbarItem::new(
            format!("item_{}", self.next_item_id),
            "Terminal".to_string(),
            DesktopTaskbarItemType::Pinned,
        );
        terminal.set_icon("terminal".to_string());
        self.items.insert(terminal.id.clone(), terminal);
        self.next_item_id += 1;

        // File manager
        let mut file_manager = DesktopTaskbarItem::new(
            format!("item_{}", self.next_item_id),
            "Files".to_string(),
            DesktopTaskbarItemType::Pinned,
        );
        file_manager.set_icon("folder".to_string());
        self.items.insert(file_manager.id.clone(), file_manager);
        self.next_item_id += 1;

        // Web browser
        let mut browser = DesktopTaskbarItem::new(
            format!("item_{}", self.next_item_id),
            "Web Browser".to_string(),
            DesktopTaskbarItemType::Pinned,
        );
        browser.set_icon("web-browser".to_string());
        self.items.insert(browser.id.clone(), browser);
        self.next_item_id += 1;
    }

    pub fn add_item(&mut self, name: String, item_type: DesktopTaskbarItemType) -> String {
        let id = format!("item_{}", self.next_item_id);
        let item = DesktopTaskbarItem::new(id.clone(), name, item_type);
        self.items.insert(id.clone(), item);
        self.next_item_id += 1;
        id
    }

    pub fn remove_item(&mut self, id: &str) -> bool {
        self.items.remove(id).is_some()
    }

    pub fn get_item(&self, id: &str) -> Option<&DesktopTaskbarItem> {
        self.items.get(id)
    }

    pub fn get_items(&self) -> Vec<&DesktopTaskbarItem> {
        self.items.values().collect()
    }

    pub fn get_items_by_type(&self, item_type: DesktopTaskbarItemType) -> Vec<&DesktopTaskbarItem> {
        self.items
            .values()
            .filter(|i| i.item_type == item_type)
            .collect()
    }

    pub fn get_running_items(&self) -> Vec<&DesktopTaskbarItem> {
        self.items
            .values()
            .filter(|i| i.item_type == DesktopTaskbarItemType::Running)
            .collect()
    }

    pub fn get_pinned_items(&self) -> Vec<&DesktopTaskbarItem> {
        self.items
            .values()
            .filter(|i| i.item_type == DesktopTaskbarItemType::Pinned)
            .collect()
    }

    pub fn set_item_active(&mut self, id: &str, active: bool) -> bool {
        if let Some(item) = self.items.get_mut(id) {
            item.set_active(active);
            true
        } else {
            false
        }
    }

    pub fn set_item_minimized(&mut self, id: &str, minimized: bool) -> bool {
        if let Some(item) = self.items.get_mut(id) {
            item.set_minimized(minimized);
            true
        } else {
            false
        }
    }

    pub fn set_item_window_count(&mut self, id: &str, count: u32) -> bool {
        if let Some(item) = self.items.get_mut(id) {
            item.set_window_count(count);
            true
        } else {
            false
        }
    }

    pub fn set_item_icon(&mut self, id: &str, icon: String) -> bool {
        if let Some(item) = self.items.get_mut(id) {
            item.set_icon(icon);
            true
        } else {
            false
        }
    }

    pub fn pin_item(&mut self, id: &str) -> bool {
        if let Some(item) = self.items.get_mut(id) {
            item.item_type = DesktopTaskbarItemType::Pinned;
            true
        } else {
            false
        }
    }

    pub fn unpin_item(&mut self, id: &str) -> bool {
        if let Some(item) = self.items.get_mut(id) {
            if item.item_type == DesktopTaskbarItemType::Pinned {
                item.item_type = DesktopTaskbarItemType::Running;
            }
            true
        } else {
            false
        }
    }

    pub fn get_position(&self) -> DesktopTaskbarPosition {
        self.position
    }

    pub fn set_position(&mut self, position: DesktopTaskbarPosition) {
        self.position = position;
    }

    pub fn is_auto_hide(&self) -> bool {
        self.auto_hide
    }

    pub fn set_auto_hide(&mut self, auto_hide: bool) {
        self.auto_hide = auto_hide;
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }

    pub fn set_size(&mut self, size: u32) {
        self.size = size;
    }

    pub fn get_icon_size(&self) -> u32 {
        self.icon_size
    }

    pub fn set_icon_size(&mut self, size: u32) {
        self.icon_size = size;
    }

    pub fn is_system_tray_visible(&self) -> bool {
        self.show_system_tray
    }

    pub fn set_system_tray_visible(&mut self, visible: bool) {
        self.show_system_tray = visible;
    }

    pub fn is_status_indicators_visible(&self) -> bool {
        self.show_status_indicators
    }

    pub fn set_status_indicators_visible(&mut self, visible: bool) {
        self.show_status_indicators = visible;
    }

    pub fn get_statistics(&self) -> DesktopTaskbarStatistics {
        DesktopTaskbarStatistics {
            total_items: self.items.len(),
            running_items: self.get_running_items().len(),
            pinned_items: self.get_pinned_items().len(),
            position: self.position,
            auto_hide: self.auto_hide,
        }
    }
}

impl Default for DesktopTaskbarManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Taskbar statistics
#[derive(Debug, Clone, Copy)]
pub struct DesktopTaskbarStatistics {
    pub total_items: usize,
    pub running_items: usize,
    pub pinned_items: usize,
    pub position: DesktopTaskbarPosition,
    pub auto_hide: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_taskbar_position_from_str() {
        assert_eq!(
            DesktopTaskbarPosition::from_str("top"),
            Some(DesktopTaskbarPosition::Top)
        );
        assert_eq!(
            DesktopTaskbarPosition::from_str("bottom"),
            Some(DesktopTaskbarPosition::Bottom)
        );
        assert_eq!(DesktopTaskbarPosition::from_str("invalid"), None);
    }

    #[test]
    fn test_taskbar_manager_initialization() {
        let manager = DesktopTaskbarManager::new();
        assert_eq!(manager.get_items().len(), 4);
        assert_eq!(manager.get_position(), DesktopTaskbarPosition::Bottom);
        assert_eq!(manager.get_size(), 48);
    }

    #[test]
    fn test_add_item() {
        let mut manager = DesktopTaskbarManager::new();
        let id = manager.add_item("Text Editor".to_string(), DesktopTaskbarItemType::Pinned);
        assert!(manager.get_item(&id).is_some());
        assert_eq!(manager.get_items().len(), 5);
    }

    #[test]
    fn test_remove_item() {
        let mut manager = DesktopTaskbarManager::new();
        let id = manager.add_item("Text Editor".to_string(), DesktopTaskbarItemType::Pinned);
        assert!(manager.remove_item(&id));
        assert!(!manager.get_item(&id).is_some());
        assert_eq!(manager.get_items().len(), 4);
    }

    #[test]
    fn test_set_item_active() {
        let mut manager = DesktopTaskbarManager::new();
        let id = manager.add_item("Text Editor".to_string(), DesktopTaskbarItemType::Running);
        assert!(manager.set_item_active(&id, true));
        assert!(manager.get_item(&id).unwrap().is_active);
    }

    #[test]
    fn test_set_item_minimized() {
        let mut manager = DesktopTaskbarManager::new();
        let id = manager.add_item("Text Editor".to_string(), DesktopTaskbarItemType::Running);
        assert!(manager.set_item_minimized(&id, true));
        assert!(manager.get_item(&id).unwrap().is_minimized);
    }

    #[test]
    fn test_pin_item() {
        let mut manager = DesktopTaskbarManager::new();
        let id = manager.add_item("Text Editor".to_string(), DesktopTaskbarItemType::Running);
        assert!(manager.pin_item(&id));
        assert_eq!(
            manager.get_item(&id).unwrap().item_type,
            DesktopTaskbarItemType::Pinned
        );
    }

    #[test]
    fn test_unpin_item() {
        let mut manager = DesktopTaskbarManager::new();
        let id = manager.add_item("Text Editor".to_string(), DesktopTaskbarItemType::Pinned);
        assert!(manager.unpin_item(&id));
        assert_eq!(
            manager.get_item(&id).unwrap().item_type,
            DesktopTaskbarItemType::Running
        );
    }

    #[test]
    fn test_get_items_by_type() {
        let manager = DesktopTaskbarManager::new();
        let pinned = manager.get_items_by_type(DesktopTaskbarItemType::Pinned);
        assert_eq!(pinned.len(), 3);

        let launchers = manager.get_items_by_type(DesktopTaskbarItemType::Launcher);
        assert_eq!(launchers.len(), 1);
    }

    #[test]
    fn test_position() {
        let mut manager = DesktopTaskbarManager::new();
        manager.set_position(DesktopTaskbarPosition::Top);
        assert_eq!(manager.get_position(), DesktopTaskbarPosition::Top);
    }

    #[test]
    fn test_auto_hide() {
        let mut manager = DesktopTaskbarManager::new();
        manager.set_auto_hide(true);
        assert!(manager.is_auto_hide());
    }

    #[test]
    fn test_statistics() {
        let manager = DesktopTaskbarManager::new();
        let stats = manager.get_statistics();
        assert_eq!(stats.total_items, 4);
        assert_eq!(stats.pinned_items, 3);
    }
}
