

use std::format;
use std::string::String;
use std::vec;
use std::vec::Vec;

/// MintMenu-inspired application menu system
/// Provides the main application menu for the desktop environment
/// similar to Linux Mint's mintmenu which is the main menu for MATE edition

#[derive(Debug, Clone, PartialEq)]
pub enum MenuItemType {
    Application,
    Category,
    Separator,
    System,
    Favorites,
    Recent,
}

#[derive(Debug, Clone)]
pub struct MenuItem {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub command: String,
    pub item_type: MenuItemType,
    pub category: Option<String>,
    pub is_visible: bool,
    pub is_favorite: bool,
}

#[derive(Debug, Clone)]
pub struct MenuCategory {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub items: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MenuSearchResult {
    pub item_id: String,
    pub relevance_score: f32,
    pub match_type: String,
}

/// MintMenu - Main application menu
pub struct MintMenu {
    pub items: Vec<MenuItem>,
    pub categories: Vec<MenuCategory>,
    pub favorites: Vec<String>,
    pub recent_items: Vec<String>,
    pub search_index: Vec<String>,
    pub current_category: Option<String>,
}

impl MintMenu {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            categories: Vec::new(),
            favorites: Vec::new(),
            recent_items: Vec::new(),
            search_index: Vec::new(),
            current_category: None,
        }
    }

    pub fn add_item(&mut self, item: MenuItem) {
        self.search_index.push(item.name.clone());
        self.items.push(item);
    }

    pub fn add_category(&mut self, category: MenuCategory) {
        self.categories.push(category);
    }

    pub fn add_to_favorites(&mut self, item_id: &str) {
        let id_string = String::from(item_id);
        if !self.favorites.iter().any(|id| id == &id_string) {
            self.favorites.push(id_string);
        }
        if let Some(item) = self.items.iter_mut().find(|i| i.id == item_id) {
            item.is_favorite = true;
        }
    }

    pub fn remove_from_favorites(&mut self, item_id: &str) {
        let id_string = String::from(item_id);
        self.favorites.retain(|id| id != &id_string);
        if let Some(item) = self.items.iter_mut().find(|i| i.id == item_id) {
            item.is_favorite = false;
        }
    }

    pub fn add_to_recent(&mut self, item_id: &str) {
        let id_string = String::from(item_id);
        self.recent_items.retain(|id| id != &id_string);
        self.recent_items.insert(0, id_string);
        if self.recent_items.len() > 10 {
            self.recent_items.pop();
        }
    }

    pub fn set_current_category(&mut self, category_id: Option<&str>) {
        self.current_category = category_id.map(|s| String::from(s));
    }

    pub fn search(&self, query: &str) -> Vec<MenuSearchResult> {
        let mut results = Vec::new();
        
        for item in &self.items {
            if !item.is_visible {
                continue;
            }
            
            let name_contains = item.name.contains(query);
            let command_contains = item.command.contains(query);
            
            if name_contains || command_contains {
                let relevance = if item.name.starts_with(query) {
                    1.0
                } else if name_contains {
                    0.7
                } else {
                    0.5
                };
                
                results.push(MenuSearchResult {
                    item_id: item.id.clone(),
                    relevance_score: relevance,
                    match_type: if item.name.starts_with(query) {
                        String::from("prefix")
                    } else {
                        String::from("contains")
                    },
                });
            }
        }
        
        results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
        results
    }

    pub fn get_items_by_category(&self, category_id: &str) -> Vec<&MenuItem> {
        if let Some(category) = self.categories.iter().find(|c| c.id == category_id) {
            self.items.iter()
                .filter(|item| category.items.contains(&item.id) && item.is_visible)
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_favorites(&self) -> Vec<&MenuItem> {
        self.items.iter()
            .filter(|item| self.favorites.iter().any(|id| id == &item.id) && item.is_visible)
            .collect()
    }

    pub fn get_recent_items(&self) -> Vec<&MenuItem> {
        self.recent_items.iter()
            .filter_map(|id| self.items.iter().find(|item| item.id == *id && item.is_visible))
            .collect()
    }

    pub fn get_all_items(&self) -> Vec<&MenuItem> {
        self.items.iter()
            .filter(|item| item.is_visible)
            .collect()
    }

    pub fn execute_item(&self, item_id: &str) -> Result<String, String> {
        match self.items.iter().find(|item| item.id == item_id) {
            Some(item) => Ok(item.command.clone()),
            None => Err(format!("Item {} not found", item_id)),
        }
    }

    pub fn display_menu(&self) -> String {
        let mut output = String::from("=== MintMenu ===\n\n");
        
        if let Some(category_id) = &self.current_category {
            let items = self.get_items_by_category(category_id);
            output.push_str(&format!("Category: {}\n", category_id));
            output.push_str(&format!("Items: {}\n\n", items.len()));
            
            for item in items {
                output.push_str(&format!("  - {} ({})\n", item.name, item.command));
            }
        } else {
            output.push_str("Favorites:\n");
            for item in self.get_favorites() {
                output.push_str(&format!("  - {}\n", item.name));
            }
            
            output.push_str("\nRecent:\n");
            for item in self.get_recent_items() {
                output.push_str(&format!("  - {}\n", item.name));
            }
            
            output.push_str("\nCategories:\n");
            for category in &self.categories {
                output.push_str(&format!("  - {} ({})\n", category.name, category.id));
            }
        }
        
        output
    }

    pub fn display_search_results(&self, query: &str) -> String {
        let results = self.search(query);
        let mut output = String::from("=== Search Results ===\n\n");
        output.push_str(&format!("Query: {}\n", query));
        output.push_str(&format!("Results: {}\n\n", results.len()));
        
        for result in &results {
            if let Some(item) = self.items.iter().find(|i| i.id == result.item_id) {
                output.push_str(&format!("  - {} ({}) [relevance: {:.2}]\n", 
                    item.name, item.command, result.relevance_score));
            }
        }
        
        output
    }

    pub fn initialize_default_categories(&mut self) {
        self.categories = vec![
            MenuCategory {
                id: String::from("accessories"),
                name: String::from("Accessories"),
                icon: String::from("accessories"),
                items: Vec::new(),
            },
            MenuCategory {
                id: String::from("development"),
                name: String::from("Development"),
                icon: String::from("development"),
                items: Vec::new(),
            },
            MenuCategory {
                id: String::from("games"),
                name: String::from("Games"),
                icon: String::from("games"),
                items: Vec::new(),
            },
            MenuCategory {
                id: String::from("graphics"),
                name: String::from("Graphics"),
                icon: String::from("graphics"),
                items: Vec::new(),
            },
            MenuCategory {
                id: String::from("internet"),
                name: String::from("Internet"),
                icon: String::from("internet"),
                items: Vec::new(),
            },
            MenuCategory {
                id: String::from("multimedia"),
                name: String::from("Multimedia"),
                icon: String::from("multimedia"),
                items: Vec::new(),
            },
            MenuCategory {
                id: String::from("office"),
                name: String::from("Office"),
                icon: String::from("office"),
                items: Vec::new(),
            },
            MenuCategory {
                id: String::from("system"),
                name: String::from("System"),
                icon: String::from("system"),
                items: Vec::new(),
            },
        ];
    }
}

impl Default for MintMenu {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mint_menu_creation() {
        let menu = MintMenu::new();
        assert!(menu.items.is_empty());
        assert!(menu.categories.is_empty());
    }

    #[test]
    fn test_add_item() {
        let mut menu = MintMenu::new();
        let item = MenuItem {
            id: String::from("vim"),
            name: String::from("Vim"),
            icon: String::from("vim"),
            command: String::from("vim"),
            item_type: MenuItemType::Application,
            category: Some(String::from("development")),
            is_visible: true,
            is_favorite: false,
        };
        
        menu.add_item(item);
        assert_eq!(menu.items.len(), 1);
        assert_eq!(menu.search_index.len(), 1);
    }

    #[test]
    fn test_add_to_favorites() {
        let mut menu = MintMenu::new();
        menu.add_item(MenuItem {
            id: String::from("vim"),
            name: String::from("Vim"),
            icon: String::from("vim"),
            command: String::from("vim"),
            item_type: MenuItemType::Application,
            category: None,
            is_visible: true,
            is_favorite: false,
        });
        
        menu.add_to_favorites("vim");
        assert!(menu.favorites.iter().any(|id| id == "vim"));
        assert!(menu.items[0].is_favorite);
    }

    #[test]
    fn test_remove_from_favorites() {
        let mut menu = MintMenu::new();
        menu.add_item(MenuItem {
            id: String::from("vim"),
            name: String::from("Vim"),
            icon: String::from("vim"),
            command: String::from("vim"),
            item_type: MenuItemType::Application,
            category: None,
            is_visible: true,
            is_favorite: true,
        });
        menu.favorites.push(String::from("vim"));
        
        menu.remove_from_favorites("vim");
        assert!(!menu.favorites.iter().any(|id| id == "vim"));
        assert!(!menu.items[0].is_favorite);
    }

    #[test]
    fn test_add_to_recent() {
        let mut menu = MintMenu::new();
        menu.add_to_recent("vim");
        menu.add_to_recent("git");
        menu.add_to_recent("vim");
        
        assert_eq!(menu.recent_items.len(), 2);
        assert_eq!(menu.recent_items[0], "vim");
    }

    #[test]
    fn test_search() {
        let mut menu = MintMenu::new();
        menu.add_item(MenuItem {
            id: String::from("vim"),
            name: String::from("Vim"),
            icon: String::from("vim"),
            command: String::from("vim"),
            item_type: MenuItemType::Application,
            category: None,
            is_visible: true,
            is_favorite: false,
        });
        
        let results = menu.search("vim");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].item_id, "vim");
    }

    #[test]
    fn test_get_favorites() {
        let mut menu = MintMenu::new();
        menu.add_item(MenuItem {
            id: String::from("vim"),
            name: String::from("Vim"),
            icon: String::from("vim"),
            command: String::from("vim"),
            item_type: MenuItemType::Application,
            category: None,
            is_visible: true,
            is_favorite: true,
        });
        menu.favorites.push(String::from("vim"));
        
        let favorites = menu.get_favorites();
        assert_eq!(favorites.len(), 1);
        assert_eq!(favorites[0].name, "Vim");
    }

    #[test]
    fn test_execute_item() {
        let mut menu = MintMenu::new();
        menu.add_item(MenuItem {
            id: String::from("vim"),
            name: String::from("Vim"),
            icon: String::from("vim"),
            command: String::from("vim"),
            item_type: MenuItemType::Application,
            category: None,
            is_visible: true,
            is_favorite: false,
        });
        
        let result = menu.execute_item("vim");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "vim");
    }

    #[test]
    fn test_initialize_default_categories() {
        let mut menu = MintMenu::new();
        menu.initialize_default_categories();
        
        assert_eq!(menu.categories.len(), 8);
        assert!(menu.categories.iter().any(|c| c.id == "development"));
    }
}
