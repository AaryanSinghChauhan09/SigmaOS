// SigmaOS Desktop Menu Manager
// Inspired by Linux Mint's menu and Omarchy's menu utilities

use std::collections::HashMap;

/// Menu entry type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuEntryType {
    Application,
    Category,
    Separator,
    Submenu,
}

impl MenuEntryType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MenuEntryType::Application => "Application",
            MenuEntryType::Category => "Category",
            MenuEntryType::Separator => "Separator",
            MenuEntryType::Submenu => "Submenu",
        }
    }
}

/// Menu entry
#[derive(Debug, Clone)]
pub struct MenuEntry {
    pub id: String,
    pub name: String,
    pub entry_type: MenuEntryType,
    pub icon: Option<String>,
    pub command: Option<String>,
    pub parent_id: Option<String>,
    pub is_visible: bool,
}

impl MenuEntry {
    pub fn new(id: String, name: String, entry_type: MenuEntryType) -> Self {
        MenuEntry {
            id,
            name,
            entry_type,
            icon: None,
            command: None,
            parent_id: None,
            is_visible: true,
        }
    }

    pub fn set_icon(&mut self, icon: String) {
        self.icon = Some(icon);
    }

    pub fn set_command(&mut self, command: String) {
        self.command = Some(command);
    }

    pub fn set_parent(&mut self, parent_id: String) {
        self.parent_id = Some(parent_id);
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }
}

/// Menu Manager
pub struct MenuManager {
    entries: HashMap<String, MenuEntry>,
    categories: Vec<String>,
    next_entry_id: u32,
}

impl MenuManager {
    pub fn new() -> Self {
        let mut manager = MenuManager {
            entries: HashMap::new(),
            categories: Vec::new(),
            next_entry_id: 1,
        };

        // Add default categories and entries
        manager.add_default_menu();

        manager
    }

    fn add_default_menu(&mut self) {
        // Accessories category
        let accessories_id = format!("cat_{}", self.next_entry_id);
        let mut accessories = MenuEntry::new(
            accessories_id.clone(),
            "Accessories".to_string(),
            MenuEntryType::Category,
        );
        accessories.set_icon("accessories".to_string());
        self.entries.insert(accessories_id.clone(), accessories);
        self.categories.push(accessories_id.clone());
        self.next_entry_id += 1;

        // Terminal
        let mut terminal = MenuEntry::new(
            format!("entry_{}", self.next_entry_id),
            "Terminal".to_string(),
            MenuEntryType::Application,
        );
        terminal.set_icon("terminal".to_string());
        terminal.set_command("sigma-terminal".to_string());
        terminal.set_parent(accessories_id.clone());
        self.entries.insert(terminal.id.clone(), terminal);
        self.next_entry_id += 1;

        // Text Editor
        let mut editor = MenuEntry::new(
            format!("entry_{}", self.next_entry_id),
            "Text Editor".to_string(),
            MenuEntryType::Application,
        );
        editor.set_icon("text-editor".to_string());
        editor.set_command("sigma-editor".to_string());
        editor.set_parent(accessories_id.clone());
        self.entries.insert(editor.id.clone(), editor);
        self.next_entry_id += 1;

        // Internet category
        let internet_id = format!("cat_{}", self.next_entry_id);
        let mut internet = MenuEntry::new(
            internet_id.clone(),
            "Internet".to_string(),
            MenuEntryType::Category,
        );
        internet.set_icon("internet".to_string());
        self.entries.insert(internet_id.clone(), internet);
        self.categories.push(internet_id.clone());
        self.next_entry_id += 1;

        // Web Browser
        let mut browser = MenuEntry::new(
            format!("entry_{}", self.next_entry_id),
            "Web Browser".to_string(),
            MenuEntryType::Application,
        );
        browser.set_icon("web-browser".to_string());
        browser.set_command("sigma-browser".to_string());
        browser.set_parent(internet_id.clone());
        self.entries.insert(browser.id.clone(), browser);
        self.next_entry_id += 1;

        // Settings category
        let settings_id = format!("cat_{}", self.next_entry_id);
        let mut settings = MenuEntry::new(
            settings_id.clone(),
            "Settings".to_string(),
            MenuEntryType::Category,
        );
        settings.set_icon("settings".to_string());
        self.entries.insert(settings_id.clone(), settings);
        self.categories.push(settings_id.clone());
        self.next_entry_id += 1;

        // System Settings
        let mut sys_settings = MenuEntry::new(
            format!("entry_{}", self.next_entry_id),
            "System Settings".to_string(),
            MenuEntryType::Application,
        );
        sys_settings.set_icon("system-settings".to_string());
        sys_settings.set_command("sigma-settings".to_string());
        sys_settings.set_parent(settings_id.clone());
        self.entries.insert(sys_settings.id.clone(), sys_settings);
        self.next_entry_id += 1;
    }

    pub fn add_entry(&mut self, name: String, entry_type: MenuEntryType) -> String {
        let id = format!("entry_{}", self.next_entry_id);
        let entry = MenuEntry::new(id.clone(), name, entry_type);
        self.entries.insert(id.clone(), entry);
        self.next_entry_id += 1;
        id
    }

    pub fn add_category(&mut self, name: String) -> String {
        let id = format!("cat_{}", self.next_entry_id);
        let mut entry = MenuEntry::new(id.clone(), name, MenuEntryType::Category);
        entry.set_icon("folder".to_string());
        self.entries.insert(id.clone(), entry);
        self.categories.push(id.clone());
        self.next_entry_id += 1;
        id
    }

    pub fn remove_entry(&mut self, id: &str) -> bool {
        let removed = self.entries.remove(id).is_some();

        // Remove from categories if it was a category
        self.categories.retain(|c| c != id);

        // Remove children
        let mut to_remove: Vec<String> = Vec::new();
        for entry in self.entries.values() {
            if let Some(parent) = &entry.parent_id {
                if parent == id {
                    to_remove.push(entry.id.clone());
                }
            }
        }

        for child_id in to_remove {
            self.remove_entry(&child_id);
        }

        removed
    }

    pub fn get_entry(&self, id: &str) -> Option<&MenuEntry> {
        self.entries.get(id)
    }

    pub fn get_entries(&self) -> Vec<&MenuEntry> {
        self.entries.values().collect()
    }

    pub fn get_categories(&self) -> Vec<&MenuEntry> {
        self.categories
            .iter()
            .filter_map(|id| self.entries.get(id))
            .collect()
    }

    pub fn get_entries_by_type(&self, entry_type: MenuEntryType) -> Vec<&MenuEntry> {
        self.entries
            .values()
            .filter(|e| e.entry_type == entry_type)
            .collect()
    }

    pub fn get_entries_by_category(&self, category_id: &str) -> Vec<&MenuEntry> {
        self.entries
            .values()
            .filter(|e| e.parent_id.as_ref() == Some(&category_id.to_string()))
            .collect()
    }

    pub fn get_visible_entries(&self) -> Vec<&MenuEntry> {
        self.entries
            .values()
            .filter(|e| e.is_visible)
            .collect()
    }

    pub fn set_entry_icon(&mut self, id: &str, icon: String) -> bool {
        if let Some(entry) = self.entries.get_mut(id) {
            entry.set_icon(icon);
            true
        } else {
            false
        }
    }

    pub fn set_entry_command(&mut self, id: &str, command: String) -> bool {
        if let Some(entry) = self.entries.get_mut(id) {
            entry.set_command(command);
            true
        } else {
            false
        }
    }

    pub fn set_entry_parent(&mut self, id: &str, parent_id: String) -> bool {
        if let Some(entry) = self.entries.get_mut(id) {
            entry.set_parent(parent_id);
            true
        } else {
            false
        }
    }

    pub fn set_entry_visible(&mut self, id: &str, visible: bool) -> bool {
        if let Some(entry) = self.entries.get_mut(id) {
            entry.set_visible(visible);
            true
        } else {
            false
        }
    }

    pub fn search_entries(&self, query: &str) -> Vec<&MenuEntry> {
        let query_lower = query.to_lowercase();
        self.entries
            .values()
            .filter(|e| {
                e.name.to_lowercase().contains(&query_lower)
                    || e.command.as_ref().map_or(false, |c| c.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    pub fn get_statistics(&self) -> MenuStatistics {
        MenuStatistics {
            total_entries: self.entries.len(),
            categories: self.categories.len(),
            applications: self.get_entries_by_type(MenuEntryType::Application).len(),
            visible_entries: self.get_visible_entries().len(),
        }
    }
}

impl Default for MenuManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Menu statistics
#[derive(Debug, Clone, Copy)]
pub struct MenuStatistics {
    pub total_entries: usize,
    pub categories: usize,
    pub applications: usize,
    pub visible_entries: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_manager_initialization() {
        let manager = MenuManager::new();
        assert_eq!(manager.get_entries().len(), 7);
        assert_eq!(manager.get_categories().len(), 3);
    }

    #[test]
    fn test_add_entry() {
        let mut manager = MenuManager::new();
        let id = manager.add_entry("Calculator".to_string(), MenuEntryType::Application);
        assert!(manager.get_entry(&id).is_some());
        assert_eq!(manager.get_entries().len(), 8);
    }

    #[test]
    fn test_add_category() {
        let mut manager = MenuManager::new();
        let id = manager.add_category("Games".to_string());
        assert!(manager.get_entry(&id).is_some());
        assert_eq!(manager.get_categories().len(), 4);
    }

    #[test]
    fn test_remove_entry() {
        let mut manager = MenuManager::new();
        let id = manager.add_entry("Calculator".to_string(), MenuEntryType::Application);
        assert!(manager.remove_entry(&id));
        assert!(!manager.get_entry(&id).is_some());
        assert_eq!(manager.get_entries().len(), 7);
    }

    #[test]
    fn test_remove_category_removes_children() {
        let mut manager = MenuManager::new();
        let cat_id = manager.add_category("Test".to_string());
        let entry_id = manager.add_entry("Test App".to_string(), MenuEntryType::Application);
        manager.set_entry_parent(&entry_id, cat_id.clone());

        assert!(manager.remove_entry(&cat_id));
        assert!(!manager.get_entry(&cat_id).is_some());
        assert!(!manager.get_entry(&entry_id).is_some());
    }

    #[test]
    fn test_set_entry_command() {
        let mut manager = MenuManager::new();
        let id = manager.add_entry("Calculator".to_string(), MenuEntryType::Application);
        assert!(manager.set_entry_command(&id, "calc".to_string()));
        assert_eq!(manager.get_entry(&id).unwrap().command, Some("calc".to_string()));
    }

    #[test]
    fn test_set_entry_parent() {
        let mut manager = MenuManager::new();
        let cat_id = manager.add_category("Test".to_string());
        let entry_id = manager.add_entry("Test App".to_string(), MenuEntryType::Application);
        assert!(manager.set_entry_parent(&entry_id, cat_id.clone()));
        assert_eq!(manager.get_entry(&entry_id).unwrap().parent_id, Some(cat_id));
    }

    #[test]
    fn test_get_entries_by_category() {
        let manager = MenuManager::new();
        let cat_id = manager.get_categories()[0].id.clone();
        let entries = manager.get_entries_by_category(&cat_id);
        assert!(entries.len() > 0);
    }

    #[test]
    fn test_search_entries() {
        let manager = MenuManager::new();
        let results = manager.search_entries("terminal");
        assert!(results.len() > 0);
    }

    #[test]
    fn test_statistics() {
        let manager = MenuManager::new();
        let stats = manager.get_statistics();
        assert_eq!(stats.total_entries, 7);
        assert_eq!(stats.categories, 3);
    }
}
