// SigmaOS Desktop Icon Manager
// Inspired by Linux Mint's desktop icons and Omarchy's icon utilities

use std::collections::HashMap;
use std::path::PathBuf;

/// Icon type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopIconType {
    Application,
    File,
    Folder,
    Link,
    Drive,
    Trash,
}

impl DesktopIconType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopIconType::Application => "Application",
            DesktopIconType::File => "File",
            DesktopIconType::Folder => "Folder",
            DesktopIconType::Link => "Link",
            DesktopIconType::Drive => "Drive",
            DesktopIconType::Trash => "Trash",
        }
    }
}

/// Desktop icon
#[derive(Debug, Clone)]
pub struct DesktopDesktopIcon {
    pub id: String,
    pub name: String,
    pub icon_type: DesktopIconType,
    pub path: PathBuf,
    pub icon_path: Option<PathBuf>,
    pub position: (i32, i32), // x, y
    pub size: u32,
    pub is_visible: bool,
    pub is_locked: bool,
}

impl DesktopDesktopIcon {
    pub fn new(
        id: String,
        name: String,
        icon_type: DesktopIconType,
        path: PathBuf,
    ) -> Self {
        DesktopDesktopIcon {
            id,
            name,
            icon_type,
            path,
            icon_path: None,
            position: (0, 0),
            size: 48,
            is_visible: true,
            is_locked: false,
        }
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.position = (x, y);
    }

    pub fn set_size(&mut self, size: u32) {
        self.size = size;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }

    pub fn set_locked(&mut self, locked: bool) {
        self.is_locked = locked;
    }

    pub fn set_icon_path(&mut self, icon_path: PathBuf) {
        self.icon_path = Some(icon_path);
    }
}

/// Icon grid alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridAlignment {
    None,
    LeftToRight,
    TopToBottom,
}

impl GridAlignment {
    pub fn as_str(&self) -> &'static str {
        match self {
            GridAlignment::None => "None",
            GridAlignment::LeftToRight => "Left to Right",
            GridAlignment::TopToBottom => "Top to Bottom",
        }
    }
}

/// Desktop Icon Manager
pub struct DesktopIconManager {
    icons: HashMap<String, DesktopDesktopIcon>,
    grid_alignment: GridAlignment,
    icon_size: u32,
    spacing: u32,
    next_icon_id: u32,
}

impl DesktopIconManager {
    pub fn new() -> Self {
        let mut manager = DesktopIconManager {
            icons: HashMap::new(),
            grid_alignment: GridAlignment::None,
            icon_size: 48,
            spacing: 10,
            next_icon_id: 1,
        };

        // Add default icons
        manager.add_default_icons();

        manager
    }

    fn add_default_icons(&mut self) {
        // Home folder
        let mut home = DesktopDesktopIcon::new(
            format!("icon_{}", self.next_icon_id),
            "Home".to_string(),
            DesktopIconType::Folder,
            PathBuf::from("/home/user"),
        );
        home.set_position(10, 10);
        self.icons.insert(home.id.clone(), home);
        self.next_icon_id += 1;

        // Trash
        let mut trash = DesktopDesktopIcon::new(
            format!("icon_{}", self.next_icon_id),
            "Trash".to_string(),
            DesktopIconType::Trash,
            PathBuf::from("trash://"),
        );
        trash.set_position(10, 70);
        trash.set_locked(true);
        self.icons.insert(trash.id.clone(), trash);
        self.next_icon_id += 1;

        // Filesystem
        let mut fs = DesktopDesktopIcon::new(
            format!("icon_{}", self.next_icon_id),
            "File System".to_string(),
            DesktopIconType::Drive,
            PathBuf::from("/"),
        );
        fs.set_position(10, 130);
        self.icons.insert(fs.id.clone(), fs);
        self.next_icon_id += 1;
    }

    pub fn add_icon(
        &mut self,
        name: String,
        icon_type: DesktopIconType,
        path: PathBuf,
    ) -> String {
        let id = format!("icon_{}", self.next_icon_id);
        let icon = DesktopDesktopIcon::new(id.clone(), name, icon_type, path);
        self.icons.insert(id.clone(), icon);
        self.next_icon_id += 1;
        id
    }

    pub fn remove_icon(&mut self, id: &str) -> bool {
        if let Some(icon) = self.icons.get(id) {
            if icon.is_locked {
                return false; // Cannot remove locked icons
            }
        }
        self.icons.remove(id).is_some()
    }

    pub fn get_icon(&self, id: &str) -> Option<&DesktopDesktopIcon> {
        self.icons.get(id)
    }

    pub fn get_icons(&self) -> Vec<&DesktopDesktopIcon> {
        self.icons.values().collect()
    }

    pub fn get_visible_icons(&self) -> Vec<&DesktopDesktopIcon> {
        self.icons
            .values()
            .filter(|i| i.is_visible)
            .collect()
    }

    pub fn get_icons_by_type(&self, icon_type: DesktopIconType) -> Vec<&DesktopDesktopIcon> {
        self.icons
            .values()
            .filter(|i| i.icon_type == icon_type)
            .collect()
    }

    pub fn set_icon_position(&mut self, id: &str, x: i32, y: i32) -> bool {
        if let Some(icon) = self.icons.get_mut(id) {
            icon.set_position(x, y);
            true
        } else {
            false
        }
    }

    pub fn set_icon_size(&mut self, id: &str, size: u32) -> bool {
        if let Some(icon) = self.icons.get_mut(id) {
            icon.set_size(size);
            true
        } else {
            false
        }
    }

    pub fn set_icon_visible(&mut self, id: &str, visible: bool) -> bool {
        if let Some(icon) = self.icons.get_mut(id) {
            icon.set_visible(visible);
            true
        } else {
            false
        }
    }

    pub fn set_icon_locked(&mut self, id: &str, locked: bool) -> bool {
        if let Some(icon) = self.icons.get_mut(id) {
            icon.set_locked(locked);
            true
        } else {
            false
        }
    }

    pub fn set_icon_path(&mut self, id: &str, icon_path: PathBuf) -> bool {
        if let Some(icon) = self.icons.get_mut(id) {
            icon.set_icon_path(icon_path);
            true
        } else {
            false
        }
    }

    pub fn get_grid_alignment(&self) -> GridAlignment {
        self.grid_alignment
    }

    pub fn set_grid_alignment(&mut self, alignment: GridAlignment) {
        self.grid_alignment = alignment;
    }

    pub fn get_icon_size(&self) -> u32 {
        self.icon_size
    }

    pub fn set_default_icon_size(&mut self, size: u32) {
        self.icon_size = size;
    }

    pub fn get_spacing(&self) -> u32 {
        self.spacing
    }

    pub fn set_spacing(&mut self, spacing: u32) {
        self.spacing = spacing;
    }

    pub fn auto_arrange(&mut self) {
        let mut visible_icons: Vec<_> = self.icons
            .values_mut()
            .filter(|i| i.is_visible)
            .collect();

        match self.grid_alignment {
            GridAlignment::LeftToRight => {
                let mut x = 10;
                let mut y = 10;
                let size = self.icon_size as i32 + self.spacing as i32;

                for icon in &mut visible_icons {
                    icon.set_position(x, y);
                    x += size;
                    if x > 800 {
                        x = 10;
                        y += size;
                    }
                }
            }
            GridAlignment::TopToBottom => {
                let mut x = 10;
                let mut y = 10;
                let size = self.icon_size as i32 + self.spacing as i32;

                for icon in &mut visible_icons {
                    icon.set_position(x, y);
                    y += size;
                    if y > 600 {
                        x += size;
                        y = 10;
                    }
                }
            }
            GridAlignment::None => {}
        }
    }

    pub fn get_statistics(&self) -> DesktopIconStatistics {
        DesktopIconStatistics {
            total_icons: self.icons.len(),
            visible_icons: self.get_visible_icons().len(),
            locked_icons: self.icons.values().filter(|i| i.is_locked).count(),
            grid_alignment: self.grid_alignment,
        }
    }
}

impl Default for DesktopIconManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Icon statistics
#[derive(Debug, Clone, Copy)]
pub struct DesktopIconStatistics {
    pub total_icons: usize,
    pub visible_icons: usize,
    pub locked_icons: usize,
    pub grid_alignment: GridAlignment,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_manager_initialization() {
        let manager = DesktopIconManager::new();
        assert_eq!(manager.get_icons().len(), 3);
        assert_eq!(manager.get_icon_size(), 48);
    }

    #[test]
    fn test_add_icon() {
        let mut manager = DesktopIconManager::new();
        let id = manager.add_icon(
            "Documents".to_string(),
            DesktopIconType::Folder,
            PathBuf::from("/home/user/Documents"),
        );
        assert!(manager.get_icon(&id).is_some());
        assert_eq!(manager.get_icons().len(), 4);
    }

    #[test]
    fn test_remove_icon() {
        let mut manager = DesktopIconManager::new();
        let id = manager.add_icon(
            "Documents".to_string(),
            DesktopIconType::Folder,
            PathBuf::from("/home/user/Documents"),
        );
        assert!(manager.remove_icon(&id));
        assert!(!manager.get_icon(&id).is_some());
        assert_eq!(manager.get_icons().len(), 3);
    }

    #[test]
    fn test_remove_locked_icon() {
        let mut manager = DesktopIconManager::new();
        // Trash is locked by default
        let trash_id = manager.get_icons_by_type(DesktopIconType::Trash)[0].id.clone();
        assert!(!manager.remove_icon(&trash_id));
    }

    #[test]
    fn test_set_icon_position() {
        let mut manager = DesktopIconManager::new();
        let id = manager.add_icon(
            "Documents".to_string(),
            DesktopIconType::Folder,
            PathBuf::from("/home/user/Documents"),
        );
        assert!(manager.set_icon_position(&id, 100, 200));
        assert_eq!(manager.get_icon(&id).unwrap().position, (100, 200));
    }

    #[test]
    fn test_set_icon_visible() {
        let mut manager = DesktopIconManager::new();
        let id = manager.add_icon(
            "Documents".to_string(),
            DesktopIconType::Folder,
            PathBuf::from("/home/user/Documents"),
        );
        assert!(manager.set_icon_visible(&id, false));
        assert!(!manager.get_icon(&id).unwrap().is_visible);
    }

    #[test]
    fn test_set_icon_locked() {
        let mut manager = DesktopIconManager::new();
        let id = manager.add_icon(
            "Documents".to_string(),
            DesktopIconType::Folder,
            PathBuf::from("/home/user/Documents"),
        );
        assert!(manager.set_icon_locked(&id, true));
        assert!(manager.get_icon(&id).unwrap().is_locked);
    }

    #[test]
    fn test_get_icons_by_type() {
        let manager = DesktopIconManager::new();
        let folders = manager.get_icons_by_type(DesktopIconType::Folder);
        assert_eq!(folders.len(), 1);

        let trash = manager.get_icons_by_type(DesktopIconType::Trash);
        assert_eq!(trash.len(), 1);
    }

    #[test]
    fn test_auto_arrange() {
        let mut manager = DesktopIconManager::new();
        manager.set_grid_alignment(GridAlignment::LeftToRight);
        manager.auto_arrange();

        let icons = manager.get_visible_icons();
        // Icons should be arranged in a grid
        assert!(icons[0].position.0 < icons[1].position.0);
    }

    #[test]
    fn test_statistics() {
        let manager = DesktopIconManager::new();
        let stats = manager.get_statistics();
        assert_eq!(stats.total_icons, 3);
        assert_eq!(stats.visible_icons, 3);
        assert_eq!(stats.locked_icons, 1);
    }
}
