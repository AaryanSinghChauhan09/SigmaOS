// SigmaOS Desktop Notification Area Manager
// Inspired by Linux Mint's notification area and Omarchy's system tray utilities

use std::collections::HashMap;

/// Notification item type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayNotificationItemType {
    Application,
    System,
    Network,
    Volume,
    Battery,
    Bluetooth,
    InputMethod,
    Custom,
}

impl TrayNotificationItemType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TrayNotificationItemType::Application => "Application",
            TrayNotificationItemType::System => "System",
            TrayNotificationItemType::Network => "Network",
            TrayNotificationItemType::Volume => "Volume",
            TrayNotificationItemType::Battery => "Battery",
            TrayNotificationItemType::Bluetooth => "Bluetooth",
            TrayNotificationItemType::InputMethod => "Input Method",
            TrayNotificationItemType::Custom => "Custom",
        }
    }
}

/// Notification item
#[derive(Debug, Clone)]
pub struct TrayNotificationItem {
    pub id: String,
    pub name: String,
    pub item_type: TrayNotificationItemType,
    pub icon: Option<String>,
    pub tooltip: Option<String>,
    pub is_visible: bool,
    pub has_menu: bool,
}

impl TrayNotificationItem {
    pub fn new(id: String, name: String, item_type: TrayNotificationItemType) -> Self {
        TrayNotificationItem {
            id,
            name,
            item_type,
            icon: None,
            tooltip: None,
            is_visible: true,
            has_menu: false,
        }
    }

    pub fn set_icon(&mut self, icon: String) {
        self.icon = Some(icon);
    }

    pub fn set_tooltip(&mut self, tooltip: String) {
        self.tooltip = Some(tooltip);
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }

    pub fn set_has_menu(&mut self, has_menu: bool) {
        self.has_menu = has_menu;
    }
}

/// Notification Area Manager
pub struct NotificationAreaManager {
    items: HashMap<String, TrayNotificationItem>,
    auto_hide: bool,
    show_icons: bool,
    show_labels: bool,
    next_item_id: u32,
}

impl NotificationAreaManager {
    pub fn new() -> Self {
        let mut manager = NotificationAreaManager {
            items: HashMap::new(),
            auto_hide: false,
            show_icons: true,
            show_labels: false,
            next_item_id: 1,
        };

        // Add default items
        manager.add_default_items();

        manager
    }

    fn add_default_items(&mut self) {
        // Network
        let mut network = TrayNotificationItem::new(
            format!("item_{}", self.next_item_id),
            "Network".to_string(),
            TrayNotificationItemType::Network,
        );
        network.set_icon("network-wired".to_string());
        network.set_tooltip("Network status".to_string());
        network.set_has_menu(true);
        self.items.insert(network.id.clone(), network);
        self.next_item_id += 1;

        // Volume
        let mut volume = TrayNotificationItem::new(
            format!("item_{}", self.next_item_id),
            "Volume".to_string(),
            TrayNotificationItemType::Volume,
        );
        volume.set_icon("audio-volume-high".to_string());
        volume.set_tooltip("Volume control".to_string());
        volume.set_has_menu(true);
        self.items.insert(volume.id.clone(), volume);
        self.next_item_id += 1;

        // Battery
        let mut battery = TrayNotificationItem::new(
            format!("item_{}", self.next_item_id),
            "Battery".to_string(),
            TrayNotificationItemType::Battery,
        );
        battery.set_icon("battery-full".to_string());
        battery.set_tooltip("Battery status".to_string());
        battery.set_has_menu(true);
        self.items.insert(battery.id.clone(), battery);
        self.next_item_id += 1;

        // Bluetooth
        let mut bluetooth = TrayNotificationItem::new(
            format!("item_{}", self.next_item_id),
            "Bluetooth".to_string(),
            TrayNotificationItemType::Bluetooth,
        );
        bluetooth.set_icon("bluetooth-active".to_string());
        bluetooth.set_tooltip("Bluetooth status".to_string());
        bluetooth.set_has_menu(true);
        self.items.insert(bluetooth.id.clone(), bluetooth);
        self.next_item_id += 1;
    }

    pub fn add_item(&mut self, name: String, item_type: TrayNotificationItemType) -> String {
        let id = format!("item_{}", self.next_item_id);
        let item = TrayNotificationItem::new(id.clone(), name, item_type);
        self.items.insert(id.clone(), item);
        self.next_item_id += 1;
        id
    }

    pub fn remove_item(&mut self, id: &str) -> bool {
        self.items.remove(id).is_some()
    }

    pub fn get_item(&self, id: &str) -> Option<&TrayNotificationItem> {
        self.items.get(id)
    }

    pub fn get_items(&self) -> Vec<&TrayNotificationItem> {
        self.items.values().collect()
    }

    pub fn get_visible_items(&self) -> Vec<&TrayNotificationItem> {
        self.items.values().filter(|i| i.is_visible).collect()
    }

    pub fn get_items_by_type(
        &self,
        item_type: TrayNotificationItemType,
    ) -> Vec<&TrayNotificationItem> {
        self.items
            .values()
            .filter(|i| i.item_type == item_type)
            .collect()
    }

    pub fn set_item_icon(&mut self, id: &str, icon: String) -> bool {
        if let Some(item) = self.items.get_mut(id) {
            item.set_icon(icon);
            true
        } else {
            false
        }
    }

    pub fn set_item_tooltip(&mut self, id: &str, tooltip: String) -> bool {
        if let Some(item) = self.items.get_mut(id) {
            item.set_tooltip(tooltip);
            true
        } else {
            false
        }
    }

    pub fn set_item_visible(&mut self, id: &str, visible: bool) -> bool {
        if let Some(item) = self.items.get_mut(id) {
            item.set_visible(visible);
            true
        } else {
            false
        }
    }

    pub fn set_item_has_menu(&mut self, id: &str, has_menu: bool) -> bool {
        if let Some(item) = self.items.get_mut(id) {
            item.set_has_menu(has_menu);
            true
        } else {
            false
        }
    }

    pub fn is_auto_hide(&self) -> bool {
        self.auto_hide
    }

    pub fn set_auto_hide(&mut self, auto_hide: bool) {
        self.auto_hide = auto_hide;
    }

    pub fn is_show_icons(&self) -> bool {
        self.show_icons
    }

    pub fn set_show_icons(&mut self, show: bool) {
        self.show_icons = show;
    }

    pub fn is_show_labels(&self) -> bool {
        self.show_labels
    }

    pub fn set_show_labels(&mut self, show: bool) {
        self.show_labels = show;
    }

    pub fn get_statistics(&self) -> NotificationAreaStatistics {
        NotificationAreaStatistics {
            total_items: self.items.len(),
            visible_items: self.get_visible_items().len(),
            items_with_menu: self.items.values().filter(|i| i.has_menu).count(),
        }
    }
}

impl Default for NotificationAreaManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Notification Area statistics
#[derive(Debug, Clone, Copy)]
pub struct NotificationAreaStatistics {
    pub total_items: usize,
    pub visible_items: usize,
    pub items_with_menu: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_area_manager_initialization() {
        let manager = NotificationAreaManager::new();
        assert_eq!(manager.get_items().len(), 4);
        assert!(manager.is_show_icons());
    }

    #[test]
    fn test_add_item() {
        let mut manager = NotificationAreaManager::new();
        let id = manager.add_item("Clock".to_string(), TrayNotificationItemType::System);
        assert!(manager.get_item(&id).is_some());
        assert_eq!(manager.get_items().len(), 5);
    }

    #[test]
    fn test_remove_item() {
        let mut manager = NotificationAreaManager::new();
        let id = manager.add_item("Clock".to_string(), TrayNotificationItemType::System);
        assert!(manager.remove_item(&id));
        assert!(!manager.get_item(&id).is_some());
        assert_eq!(manager.get_items().len(), 4);
    }

    #[test]
    fn test_set_item_icon() {
        let mut manager = NotificationAreaManager::new();
        let id = manager.add_item("Clock".to_string(), TrayNotificationItemType::System);
        assert!(manager.set_item_icon(&id, "clock".to_string()));
        assert_eq!(
            manager.get_item(&id).unwrap().icon,
            Some("clock".to_string())
        );
    }

    #[test]
    fn test_set_item_tooltip() {
        let mut manager = NotificationAreaManager::new();
        let id = manager.add_item("Clock".to_string(), TrayNotificationItemType::System);
        assert!(manager.set_item_tooltip(&id, "Current time".to_string()));
        assert_eq!(
            manager.get_item(&id).unwrap().tooltip,
            Some("Current time".to_string())
        );
    }

    #[test]
    fn test_set_item_visible() {
        let mut manager = NotificationAreaManager::new();
        let id = manager.add_item("Clock".to_string(), TrayNotificationItemType::System);
        assert!(manager.set_item_visible(&id, false));
        assert!(!manager.get_item(&id).unwrap().is_visible);
    }

    #[test]
    fn test_get_items_by_type() {
        let manager = NotificationAreaManager::new();
        let network = manager.get_items_by_type(TrayNotificationItemType::Network);
        assert_eq!(network.len(), 1);

        let volume = manager.get_items_by_type(TrayNotificationItemType::Volume);
        assert_eq!(volume.len(), 1);
    }

    #[test]
    fn test_auto_hide() {
        let mut manager = NotificationAreaManager::new();
        manager.set_auto_hide(true);
        assert!(manager.is_auto_hide());
    }

    #[test]
    fn test_show_icons() {
        let mut manager = NotificationAreaManager::new();
        manager.set_show_icons(false);
        assert!(!manager.is_show_icons());
    }

    #[test]
    fn test_show_labels() {
        let mut manager = NotificationAreaManager::new();
        manager.set_show_labels(true);
        assert!(manager.is_show_labels());
    }

    #[test]
    fn test_statistics() {
        let manager = NotificationAreaManager::new();
        let stats = manager.get_statistics();
        assert_eq!(stats.total_items, 4);
        assert_eq!(stats.visible_items, 4);
    }
}
