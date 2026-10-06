// Bar Widgets System
// Inspired by Omarchy v4.0 Quickshell bar widgets
// Manages interactive widgets in the status bar with positioning and interactions

use std::collections::HashMap;

/// Bar position (left, center, right)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BarPosition {
    Left,
    Center,
    Right,
}

/// Widget interaction type
#[derive(Debug, Clone, PartialEq)]
pub enum WidgetInteraction {
    None,
    LeftClick,
    RightClick,
    MiddleClick,
    Scroll,
}

/// Bar widget configuration
#[derive(Debug, Clone)]
pub struct BarWidget {
    pub id: String,
    pub name: String,
    pub position: BarPosition,
    pub icon: Option<String>,
    pub enabled: bool,
    pub visible: bool,
    pub interactions: Vec<WidgetInteraction>,
    pub config: HashMap<String, String>,
}

impl BarWidget {
    pub fn new(id: String, name: String, position: BarPosition) -> Self {
        Self {
            id,
            name,
            position,
            icon: None,
            enabled: true,
            visible: true,
            interactions: Vec::new(),
            config: HashMap::new(),
        }
    }

    pub fn with_icon(mut self, icon: String) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn with_interactions(mut self, interactions: Vec<WidgetInteraction>) -> Self {
        self.interactions = interactions;
        self
    }

    pub fn with_config(mut self, config: HashMap<String, String>) -> Self {
        self.config = config;
        self
    }

    pub fn set_config(&mut self, key: String, value: String) {
        self.config.insert(key, value);
    }

    pub fn get_config(&self, key: &str) -> Option<&String> {
        self.config.get(key)
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn move_to(&mut self, position: BarPosition) {
        self.position = position;
    }
}

/// Bar Widgets Manager
#[derive(Debug, Clone)]
pub struct BarWidgetsManager {
    widgets: HashMap<String, BarWidget>,
    left_widgets: Vec<String>,
    center_widgets: Vec<String>,
    right_widgets: Vec<String>,
}

impl BarWidgetsManager {
    pub fn new() -> Self {
        Self {
            widgets: HashMap::new(),
            left_widgets: Vec::new(),
            center_widgets: Vec::new(),
            right_widgets: Vec::new(),
        }
    }

    /// Register a widget
    pub fn register_widget(&mut self, widget: BarWidget) {
        let id = widget.id.clone();
        let position = widget.position.clone();
        
        self.widgets.insert(id.clone(), widget);
        
        match position {
            BarPosition::Left => {
                if !self.left_widgets.contains(&id) {
                    self.left_widgets.push(id);
                }
            }
            BarPosition::Center => {
                if !self.center_widgets.contains(&id) {
                    self.center_widgets.push(id);
                }
            }
            BarPosition::Right => {
                if !self.right_widgets.contains(&id) {
                    self.right_widgets.push(id);
                }
            }
        }
    }

    /// Get all widgets
    pub fn get_widgets(&self) -> Vec<&BarWidget> {
        self.widgets.values().collect()
    }

    /// Get widgets by position
    pub fn get_widgets_by_position(&self, position: BarPosition) -> Vec<&BarWidget> {
        let ids = match position {
            BarPosition::Left => &self.left_widgets,
            BarPosition::Center => &self.center_widgets,
            BarPosition::Right => &self.right_widgets,
        };
        
        ids.iter()
            .filter_map(|id| self.widgets.get(id))
            .collect()
    }

    /// Get enabled widgets
    pub fn get_enabled_widgets(&self) -> Vec<&BarWidget> {
        self.widgets
            .values()
            .filter(|w| w.enabled)
            .collect()
    }

    /// Get visible widgets
    pub fn get_visible_widgets(&self) -> Vec<&BarWidget> {
        self.widgets
            .values()
            .filter(|w| w.visible)
            .collect()
    }

    /// Get widget by ID
    pub fn get_widget(&self, id: &str) -> Option<&BarWidget> {
        self.widgets.get(id)
    }

    /// Enable a widget
    pub fn enable_widget(&mut self, id: &str) -> Result<(), String> {
        if let Some(widget) = self.widgets.get_mut(id) {
            widget.enable();
            Ok(())
        } else {
            Err(format!("Widget {} not found", id))
        }
    }

    /// Disable a widget
    pub fn disable_widget(&mut self, id: &str) -> Result<(), String> {
        if let Some(widget) = self.widgets.get_mut(id) {
            widget.disable();
            Ok(())
        } else {
            Err(format!("Widget {} not found", id))
        }
    }

    /// Show a widget
    pub fn show_widget(&mut self, id: &str) -> Result<(), String> {
        if let Some(widget) = self.widgets.get_mut(id) {
            widget.show();
            Ok(())
        } else {
            Err(format!("Widget {} not found", id))
        }
    }

    /// Hide a widget
    pub fn hide_widget(&mut self, id: &str) -> Result<(), String> {
        if let Some(widget) = self.widgets.get_mut(id) {
            widget.hide();
            Ok(())
        } else {
            Err(format!("Widget {} not found", id))
        }
    }

    /// Move widget to position
    pub fn move_widget(&mut self, id: &str, position: BarPosition) -> Result<(), String> {
        if !self.widgets.contains_key(id) {
            return Err(format!("Widget {} not found", id));
        }

        // Remove from old position
        self.left_widgets.retain(|x| x != id);
        self.center_widgets.retain(|x| x != id);
        self.right_widgets.retain(|x| x != id);

        // Add to new position
        match position {
            BarPosition::Left => self.left_widgets.push(id.to_string()),
            BarPosition::Center => self.center_widgets.push(id.to_string()),
            BarPosition::Right => self.right_widgets.push(id.to_string()),
        }

        if let Some(widget) = self.widgets.get_mut(id) {
            widget.move_to(position);
        }

        Ok(())
    }

    /// Reorder widgets within a position
    pub fn reorder_widgets(&mut self, position: BarPosition, ids: Vec<String>) -> Result<(), String> {
        match position {
            BarPosition::Left => {
                for id in &ids {
                    if !self.widgets.contains_key(id) {
                        return Err(format!("Widget {} not found", id));
                    }
                }
                self.left_widgets = ids;
            }
            BarPosition::Center => {
                for id in &ids {
                    if !self.widgets.contains_key(id) {
                        return Err(format!("Widget {} not found", id));
                    }
                }
                self.center_widgets = ids;
            }
            BarPosition::Right => {
                for id in &ids {
                    if !self.widgets.contains_key(id) {
                        return Err(format!("Widget {} not found", id));
                    }
                }
                self.right_widgets = ids;
            }
        }
        Ok(())
    }

    /// Remove a widget
    pub fn remove_widget(&mut self, id: &str) -> Result<(), String> {
        if !self.widgets.contains_key(id) {
            return Err(format!("Widget {} not found", id));
        }

        self.widgets.remove(id);
        self.left_widgets.retain(|x| x != id);
        self.center_widgets.retain(|x| x != id);
        self.right_widgets.retain(|x| x != id);

        Ok(())
    }

    /// Get statistics
    pub fn get_statistics(&self) -> WidgetStatistics {
        let total = self.widgets.len();
        let enabled = self.widgets.values().filter(|w| w.enabled).count();
        let visible = self.widgets.values().filter(|w| w.visible).count();
        let left = self.left_widgets.len();
        let center = self.center_widgets.len();
        let right = self.right_widgets.len();

        WidgetStatistics {
            total,
            enabled,
            visible,
            left,
            center,
            right,
        }
    }
}

impl Default for BarWidgetsManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Widget statistics
#[derive(Debug, Clone, PartialEq)]
pub struct WidgetStatistics {
    pub total: usize,
    pub enabled: usize,
    pub visible: usize,
    pub left: usize,
    pub center: usize,
    pub right: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = BarWidgetsManager::new();
        assert_eq!(manager.get_widgets().len(), 0);
    }

    #[test]
    fn test_register_widget() {
        let mut manager = BarWidgetsManager::new();
        let widget = BarWidget::new("clock".to_string(), "Clock".to_string(), BarPosition::Center);
        manager.register_widget(widget);
        assert_eq!(manager.get_widgets().len(), 1);
    }

    #[test]
    fn test_widget_positioning() {
        let mut manager = BarWidgetsManager::new();
        manager.register_widget(BarWidget::new("clock".to_string(), "Clock".to_string(), BarPosition::Center));
        manager.register_widget(BarWidget::new("network".to_string(), "Network".to_string(), BarPosition::Right));
        
        let center = manager.get_widgets_by_position(BarPosition::Center);
        let right = manager.get_widgets_by_position(BarPosition::Right);
        
        assert_eq!(center.len(), 1);
        assert_eq!(right.len(), 1);
    }

    #[test]
    fn test_enable_disable() {
        let mut manager = BarWidgetsManager::new();
        manager.register_widget(BarWidget::new("clock".to_string(), "Clock".to_string(), BarPosition::Center));
        
        manager.disable_widget("clock").unwrap();
        assert!(!manager.get_widget("clock").unwrap().enabled);
        
        manager.enable_widget("clock").unwrap();
        assert!(manager.get_widget("clock").unwrap().enabled);
    }

    #[test]
    fn test_show_hide() {
        let mut manager = BarWidgetsManager::new();
        manager.register_widget(BarWidget::new("clock".to_string(), "Clock".to_string(), BarPosition::Center));
        
        manager.hide_widget("clock").unwrap();
        assert!(!manager.get_widget("clock").unwrap().visible);
        
        manager.show_widget("clock").unwrap();
        assert!(manager.get_widget("clock").unwrap().visible);
    }

    #[test]
    fn test_move_widget() {
        let mut manager = BarWidgetsManager::new();
        manager.register_widget(BarWidget::new("clock".to_string(), "Clock".to_string(), BarPosition::Center));
        
        manager.move_widget("clock", BarPosition::Right).unwrap();
        assert_eq!(manager.get_widget("clock").unwrap().position, BarPosition::Right);
    }

    #[test]
    fn test_reorder_widgets() {
        let mut manager = BarWidgetsManager::new();
        manager.register_widget(BarWidget::new("w1".to_string(), "W1".to_string(), BarPosition::Left));
        manager.register_widget(BarWidget::new("w2".to_string(), "W2".to_string(), BarPosition::Left));
        
        manager.reorder_widgets(BarPosition::Left, vec!["w2".to_string(), "w1".to_string()]).unwrap();
        
        let left = manager.get_widgets_by_position(BarPosition::Left);
        assert_eq!(left[0].id, "w2");
        assert_eq!(left[1].id, "w1");
    }

    #[test]
    fn test_remove_widget() {
        let mut manager = BarWidgetsManager::new();
        manager.register_widget(BarWidget::new("clock".to_string(), "Clock".to_string(), BarPosition::Center));
        
        manager.remove_widget("clock").unwrap();
        assert_eq!(manager.get_widgets().len(), 0);
    }

    #[test]
    fn test_statistics() {
        let mut manager = BarWidgetsManager::new();
        manager.register_widget(BarWidget::new("clock".to_string(), "Clock".to_string(), BarPosition::Center));
        manager.register_widget(BarWidget::new("network".to_string(), "Network".to_string(), BarPosition::Right));
        manager.disable_widget("network").unwrap();
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total, 2);
        assert_eq!(stats.enabled, 1);
        assert_eq!(stats.visible, 2);
        assert_eq!(stats.center, 1);
        assert_eq!(stats.right, 1);
    }
}
