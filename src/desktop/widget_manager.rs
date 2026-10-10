// SigmaOS Desktop Widget Manager
// Inspired by Linux Mint's desklets and Omarchy's widget utilities

use std::collections::HashMap;

/// Widget type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopWidgetType {
    Clock,
    Calendar,
    Weather,
    SystemMonitor,
    NetworkMonitor,
    DiskMonitor,
    CPUUsage,
    MemoryUsage,
    Battery,
    Notes,
    Custom,
}

impl DesktopWidgetType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopWidgetType::Clock => "Clock",
            DesktopWidgetType::Calendar => "Calendar",
            DesktopWidgetType::Weather => "Weather",
            DesktopWidgetType::SystemMonitor => "System Monitor",
            DesktopWidgetType::NetworkMonitor => "Network Monitor",
            DesktopWidgetType::DiskMonitor => "Disk Monitor",
            DesktopWidgetType::CPUUsage => "CPU Usage",
            DesktopWidgetType::MemoryUsage => "Memory Usage",
            DesktopWidgetType::Battery => "Battery",
            DesktopWidgetType::Notes => "Notes",
            DesktopWidgetType::Custom => "Custom",
        }
    }
}

/// Widget position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopWidgetPosition {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    Custom(i32, i32),
}

impl DesktopWidgetPosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopWidgetPosition::TopLeft => "Top Left",
            DesktopWidgetPosition::TopCenter => "Top Center",
            DesktopWidgetPosition::TopRight => "Top Right",
            DesktopWidgetPosition::MiddleLeft => "Middle Left",
            DesktopWidgetPosition::MiddleCenter => "Middle Center",
            DesktopWidgetPosition::MiddleRight => "Middle Right",
            DesktopWidgetPosition::BottomLeft => "Bottom Left",
            DesktopWidgetPosition::BottomCenter => "Bottom Center",
            DesktopWidgetPosition::BottomRight => "Bottom Right",
            DesktopWidgetPosition::Custom(_, _) => "Custom",
        }
    }
}

/// Desktop widget
#[derive(Debug, Clone)]
pub struct DesktopWidget {
    pub id: String,
    pub name: String,
    pub widget_type: DesktopWidgetType,
    pub position: DesktopWidgetPosition,
    pub size: (u32, u32), // width, height
    pub is_visible: bool,
    pub is_locked: bool,
    pub config: HashMap<String, String>,
}

impl DesktopWidget {
    pub fn new(id: String, name: String, widget_type: DesktopWidgetType) -> Self {
        DesktopWidget {
            id,
            name,
            widget_type,
            position: DesktopWidgetPosition::TopRight,
            size: (200, 200),
            is_visible: true,
            is_locked: false,
            config: HashMap::new(),
        }
    }

    pub fn set_position(&mut self, position: DesktopWidgetPosition) {
        self.position = position;
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.size = (width, height);
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }

    pub fn set_locked(&mut self, locked: bool) {
        self.is_locked = locked;
    }

    pub fn set_config(&mut self, key: String, value: String) {
        self.config.insert(key, value);
    }

    pub fn get_config(&self, key: &str) -> Option<&String> {
        self.config.get(key)
    }
}

/// Widget Manager
pub struct DesktopWidgetManager {
    widgets: HashMap<String, DesktopWidget>,
    next_widget_id: u32,
}

impl DesktopWidgetManager {
    pub fn new() -> Self {
        let mut manager = DesktopWidgetManager {
            widgets: HashMap::new(),
            next_widget_id: 1,
        };

        // Add default widgets
        manager.add_default_widgets();

        manager
    }

    fn add_default_widgets(&mut self) {
        // Clock widget
        let mut clock = DesktopWidget::new(
            format!("widget_{}", self.next_widget_id),
            "Clock".to_string(),
            DesktopWidgetType::Clock,
        );
        clock.set_position(DesktopWidgetPosition::TopRight);
        clock.set_size(200, 100);
        clock.set_config("format".to_string(), "%H:%M".to_string());
        self.widgets.insert(clock.id.clone(), clock);
        self.next_widget_id += 1;

        // System monitor widget
        let mut sysmon = DesktopWidget::new(
            format!("widget_{}", self.next_widget_id),
            "System Monitor".to_string(),
            DesktopWidgetType::SystemMonitor,
        );
        sysmon.set_position(DesktopWidgetPosition::BottomRight);
        sysmon.set_size(250, 150);
        self.widgets.insert(sysmon.id.clone(), sysmon);
        self.next_widget_id += 1;
    }

    pub fn add_widget(&mut self, name: String, widget_type: DesktopWidgetType) -> String {
        let id = format!("widget_{}", self.next_widget_id);
        let widget = DesktopWidget::new(id.clone(), name, widget_type);
        self.widgets.insert(id.clone(), widget);
        self.next_widget_id += 1;
        id
    }

    pub fn remove_widget(&mut self, id: &str) -> bool {
        if let Some(widget) = self.widgets.get(id) {
            if widget.is_locked {
                return false; // Cannot remove locked widgets
            }
        }
        self.widgets.remove(id).is_some()
    }

    pub fn get_widget(&self, id: &str) -> Option<&DesktopWidget> {
        self.widgets.get(id)
    }

    pub fn get_widgets(&self) -> Vec<&DesktopWidget> {
        self.widgets.values().collect()
    }

    pub fn get_visible_widgets(&self) -> Vec<&DesktopWidget> {
        self.widgets.values().filter(|w| w.is_visible).collect()
    }

    pub fn get_widgets_by_type(&self, widget_type: DesktopWidgetType) -> Vec<&DesktopWidget> {
        self.widgets
            .values()
            .filter(|w| w.widget_type == widget_type)
            .collect()
    }

    pub fn set_widget_position(&mut self, id: &str, position: DesktopWidgetPosition) -> bool {
        if let Some(widget) = self.widgets.get_mut(id) {
            widget.set_position(position);
            true
        } else {
            false
        }
    }

    pub fn set_widget_size(&mut self, id: &str, width: u32, height: u32) -> bool {
        if let Some(widget) = self.widgets.get_mut(id) {
            widget.set_size(width, height);
            true
        } else {
            false
        }
    }

    pub fn set_widget_visible(&mut self, id: &str, visible: bool) -> bool {
        if let Some(widget) = self.widgets.get_mut(id) {
            widget.set_visible(visible);
            true
        } else {
            false
        }
    }

    pub fn set_widget_locked(&mut self, id: &str, locked: bool) -> bool {
        if let Some(widget) = self.widgets.get_mut(id) {
            widget.set_locked(locked);
            true
        } else {
            false
        }
    }

    pub fn set_widget_config(&mut self, id: &str, key: String, value: String) -> bool {
        if let Some(widget) = self.widgets.get_mut(id) {
            widget.set_config(key, value);
            true
        } else {
            false
        }
    }

    pub fn get_widget_config(&self, id: &str, key: &str) -> Option<String> {
        if let Some(widget) = self.widgets.get(id) {
            widget.get_config(key).cloned()
        } else {
            None
        }
    }

    pub fn get_statistics(&self) -> DesktopWidgetStatistics {
        DesktopWidgetStatistics {
            total_widgets: self.widgets.len(),
            visible_widgets: self.get_visible_widgets().len(),
            locked_widgets: self.widgets.values().filter(|w| w.is_locked).count(),
        }
    }
}

impl Default for DesktopWidgetManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Widget statistics
#[derive(Debug, Clone, Copy)]
pub struct DesktopWidgetStatistics {
    pub total_widgets: usize,
    pub visible_widgets: usize,
    pub locked_widgets: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_widget_manager_initialization() {
        let manager = DesktopWidgetManager::new();
        assert_eq!(manager.get_widgets().len(), 2);
    }

    #[test]
    fn test_add_widget() {
        let mut manager = DesktopWidgetManager::new();
        let id = manager.add_widget("Weather".to_string(), DesktopWidgetType::Weather);
        assert!(manager.get_widget(&id).is_some());
        assert_eq!(manager.get_widgets().len(), 3);
    }

    #[test]
    fn test_remove_widget() {
        let mut manager = DesktopWidgetManager::new();
        let id = manager.add_widget("Weather".to_string(), DesktopWidgetType::Weather);
        assert!(manager.remove_widget(&id));
        assert!(!manager.get_widget(&id).is_some());
        assert_eq!(manager.get_widgets().len(), 2);
    }

    #[test]
    fn test_remove_locked_widget() {
        let mut manager = DesktopWidgetManager::new();
        let id = manager.add_widget("Weather".to_string(), DesktopWidgetType::Weather);
        manager.set_widget_locked(&id, true);
        assert!(!manager.remove_widget(&id));
    }

    #[test]
    fn test_set_widget_position() {
        let mut manager = DesktopWidgetManager::new();
        let id = manager.add_widget("Weather".to_string(), DesktopWidgetType::Weather);
        assert!(manager.set_widget_position(&id, DesktopWidgetPosition::TopLeft));
        assert_eq!(
            manager.get_widget(&id).unwrap().position,
            DesktopWidgetPosition::TopLeft
        );
    }

    #[test]
    fn test_set_widget_size() {
        let mut manager = DesktopWidgetManager::new();
        let id = manager.add_widget("Weather".to_string(), DesktopWidgetType::Weather);
        assert!(manager.set_widget_size(&id, 300, 200));
        assert_eq!(manager.get_widget(&id).unwrap().size, (300, 200));
    }

    #[test]
    fn test_set_widget_config() {
        let mut manager = DesktopWidgetManager::new();
        let id = manager.add_widget("Weather".to_string(), DesktopWidgetType::Weather);
        assert!(manager.set_widget_config(&id, "location".to_string(), "London".to_string()));
        assert_eq!(
            manager.get_widget_config(&id, "location"),
            Some("London".to_string())
        );
    }

    #[test]
    fn test_get_widgets_by_type() {
        let manager = DesktopWidgetManager::new();
        let clocks = manager.get_widgets_by_type(DesktopWidgetType::Clock);
        assert_eq!(clocks.len(), 1);
    }

    #[test]
    fn test_statistics() {
        let manager = DesktopWidgetManager::new();
        let stats = manager.get_statistics();
        assert_eq!(stats.total_widgets, 2);
        assert_eq!(stats.visible_widgets, 2);
    }
}
