use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TrayCategory {
    Application,
    Communications,
    SystemServices,
    Hardware,
}

#[derive(Debug, Clone)]
pub struct TrayMenuItem {
    pub label: String,
    pub icon: Option<String>,
    pub shortcut: Option<String>,
    pub is_separator: bool,
    pub submenu: Option<Vec<TrayMenuItem>>,
}

#[derive(Debug, Clone)]
pub struct StatusIcon {
    pub id: String,
    pub category: TrayCategory,
    pub icon_name: String,
    pub tooltip: String,
    pub unread_count: u32,
    pub menu: Vec<TrayMenuItem>,
}

pub struct SystemTray {
    icons: HashMap<String, StatusIcon>,
    auto_hide: bool,
    active: bool,
}

impl SystemTray {
    pub fn new() -> Self {
        Self {
            icons: HashMap::new(),
            auto_hide: true,
            active: true,
        }
    }

    pub fn add_icon(&mut self, icon: StatusIcon) {
        self.icons.insert(icon.id.clone(), icon);
    }

    pub fn remove_icon(&mut self, id: &str) {
        self.icons.remove(id);
    }

    pub fn get_icon(&self, id: &str) -> Option<&StatusIcon> {
        self.icons.get(id)
    }

    pub fn get_icons_by_category(&self, category: TrayCategory) -> Vec<&StatusIcon> {
        self.icons.values().filter(|i| i.category == category).collect()
    }

    pub fn set_unread_count(&mut self, id: &str, count: u32) -> Result<(), String> {
        if let Some(icon) = self.icons.get_mut(id) {
            icon.unread_count = count;
            Ok(())
        } else {
            Err("Icon not found".to_string())
        }
    }

    pub fn should_hide(&self) -> bool {
        self.auto_hide && !self.active && self.icons.values().all(|i| i.unread_count == 0)
    }
    
    pub fn resolve_icon_theme(icon_name: &str) -> String {
        format!("/usr/share/icons/hicolor/scalable/apps/{}.svg", icon_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_icon() -> StatusIcon {
        StatusIcon {
            id: "network".to_string(),
            category: TrayCategory::SystemServices,
            icon_name: "network-wireless".to_string(),
            tooltip: "Connected to WiFi".to_string(),
            unread_count: 0,
            menu: vec![
                TrayMenuItem {
                    label: "Disconnect".to_string(),
                    icon: None,
                    shortcut: None,
                    is_separator: false,
                    submenu: None,
                }
            ],
        }
    }

    #[test]
    fn test_add_remove_icon() {
        let mut tray = SystemTray::new();
        let icon = create_test_icon();
        tray.add_icon(icon.clone());
        assert!(tray.get_icon("network").is_some());
        
        tray.remove_icon("network");
        assert!(tray.get_icon("network").is_none());
    }

    #[test]
    fn test_category_filter() {
        let mut tray = SystemTray::new();
        tray.add_icon(create_test_icon());
        tray.add_icon(StatusIcon {
            id: "chat".to_string(),
            category: TrayCategory::Communications,
            icon_name: "chat-app".to_string(),
            tooltip: "Chat".to_string(),
            unread_count: 0,
            menu: vec![],
        });
        
        let sys_icons = tray.get_icons_by_category(TrayCategory::SystemServices);
        assert_eq!(sys_icons.len(), 1);
        assert_eq!(sys_icons[0].id, "network");
    }

    #[test]
    fn test_unread_count() {
        let mut tray = SystemTray::new();
        tray.add_icon(create_test_icon());
        assert!(tray.set_unread_count("network", 5).is_ok());
        assert_eq!(tray.get_icon("network").unwrap().unread_count, 5);
        assert!(tray.set_unread_count("unknown", 1).is_err());
    }

    #[test]
    fn test_auto_hide() {
        let mut tray = SystemTray::new();
        tray.active = false;
        tray.add_icon(create_test_icon());
        assert!(tray.should_hide());
        
        tray.set_unread_count("network", 1).unwrap();
        assert!(!tray.should_hide());
    }

    #[test]
    fn test_resolve_icon() {
        let path = SystemTray::resolve_icon_theme("spotify");
        assert_eq!(path, "/usr/share/icons/hicolor/scalable/apps/spotify.svg");
    }
}
