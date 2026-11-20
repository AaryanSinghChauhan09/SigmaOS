//! SigmaOS libappindicator
//!
//! System-tray / application-indicator compatibility layer.
//! Provides the AppIndicator model used by tray-aware applications,
//! mapping to the StatusNotifierItem D-Bus spec (KDE/GNOME).
//! Upstream: libappindicator (https://launchpad.net/libappindicator),
//!           StatusNotifierItem spec (https://freedesktop.org/wiki/Specifications/StatusNotifierItem/)

use std::collections::HashMap;

/// Indicator status (maps to StatusNotifierItem Status property).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndicatorStatus {
    Passive,    // hidden
    Active,     // normal visible
    Attention,  // highlighted / urgent
}

impl std::fmt::Display for IndicatorStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Passive   => write!(f, "Passive"),
            Self::Active    => write!(f, "Active"),
            Self::Attention => write!(f, "NeedsAttention"),
        }
    }
}

/// A menu item inside an indicator's context menu.
#[derive(Debug, Clone)]
pub struct IndicatorMenuItem {
    pub id:      String,
    pub label:   String,
    pub enabled: bool,
    pub visible: bool,
}

impl IndicatorMenuItem {
    pub fn new(id: &str, label: &str) -> Self {
        Self { id: id.to_owned(), label: label.to_owned(), enabled: true, visible: true }
    }
}

/// An application indicator (system tray icon + context menu).
#[derive(Debug, Clone)]
pub struct AppIndicator {
    pub app_id:      String,
    pub icon_name:   String,
    pub title:       String,
    pub status:      IndicatorStatus,
    pub tooltip:     String,
    menu_items:      Vec<IndicatorMenuItem>,
    props:           HashMap<String, String>,
}

impl AppIndicator {
    pub fn new(app_id: &str, icon_name: &str, title: &str) -> Self {
        Self {
            app_id:    app_id.to_owned(),
            icon_name: icon_name.to_owned(),
            title:     title.to_owned(),
            status:    IndicatorStatus::Active,
            tooltip:   String::new(),
            menu_items: Vec::new(),
            props:      HashMap::new(),
        }
    }

    pub fn status(mut self, s: IndicatorStatus) -> Self { self.status = s; self }
    pub fn tooltip(mut self, t: &str) -> Self { self.tooltip = t.to_owned(); self }
    pub fn set_icon(&mut self, icon: &str) { self.icon_name = icon.to_owned(); }
    pub fn set_status(&mut self, s: IndicatorStatus) { self.status = s; }

    pub fn add_menu_item(&mut self, item: IndicatorMenuItem) -> bool {
        if self.menu_items.iter().any(|m| m.id == item.id) { return false; }
        self.menu_items.push(item);
        true
    }

    pub fn remove_menu_item(&mut self, id: &str) -> bool {
        let before = self.menu_items.len();
        self.menu_items.retain(|m| m.id != id);
        self.menu_items.len() < before
    }

    pub fn menu_items(&self) -> &[IndicatorMenuItem] { &self.menu_items }
    pub fn is_visible(&self) -> bool { self.status != IndicatorStatus::Passive }

    pub fn set_prop(&mut self, k: &str, v: &str) { self.props.insert(k.to_owned(), v.to_owned()); }
    pub fn prop(&self, k: &str) -> Option<&str> { self.props.get(k).map(|s| s.as_str()) }

    /// Serialize to a minimal StatusNotifierItem-compatible property map.
    pub fn to_status_notifier_props(&self) -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("Id".into(),       self.app_id.clone());
        m.insert("Title".into(),    self.title.clone());
        m.insert("Status".into(),   self.status.to_string());
        m.insert("IconName".into(), self.icon_name.clone());
        m.insert("ToolTip".into(),  self.tooltip.clone());
        m
    }
}

/// Registry of all registered application indicators.
#[derive(Default)]
pub struct IndicatorRegistry {
    indicators: HashMap<String, AppIndicator>,
}

impl IndicatorRegistry {
    pub fn new() -> Self { Self::default() }

    pub fn register(&mut self, ind: AppIndicator) -> bool {
        if self.indicators.contains_key(&ind.app_id) { return false; }
        self.indicators.insert(ind.app_id.clone(), ind);
        true
    }

    pub fn unregister(&mut self, app_id: &str) -> bool {
        self.indicators.remove(app_id).is_some()
    }

    pub fn get(&self, app_id: &str) -> Option<&AppIndicator> { self.indicators.get(app_id) }
    pub fn get_mut(&mut self, app_id: &str) -> Option<&mut AppIndicator> { self.indicators.get_mut(app_id) }

    pub fn visible(&self) -> Vec<&AppIndicator> {
        self.indicators.values().filter(|i| i.is_visible()).collect()
    }

    pub fn count(&self) -> usize { self.indicators.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_lookup() {
        let mut reg = IndicatorRegistry::new();
        let ind = AppIndicator::new("dropbox", "dropbox-indicator", "Dropbox");
        assert!(reg.register(ind));
        assert_eq!(reg.count(), 1);
        assert!(!reg.register(AppIndicator::new("dropbox", "dropbox", "Dropbox Dup")));
    }

    #[test]
    fn unregister() {
        let mut reg = IndicatorRegistry::new();
        reg.register(AppIndicator::new("app1", "icon1", "App1"));
        assert!(reg.unregister("app1"));
        assert!(!reg.unregister("app1"));
    }

    #[test]
    fn status_visibility() {
        let ind = AppIndicator::new("a", "i", "t").status(IndicatorStatus::Passive);
        assert!(!ind.is_visible());
        let ind2 = AppIndicator::new("b", "i", "t").status(IndicatorStatus::Active);
        assert!(ind2.is_visible());
    }

    #[test]
    fn menu_item_dedup() {
        let mut ind = AppIndicator::new("a", "i", "t");
        assert!(ind.add_menu_item(IndicatorMenuItem::new("quit", "Quit")));
        assert!(!ind.add_menu_item(IndicatorMenuItem::new("quit", "Quit Again")));
        assert_eq!(ind.menu_items().len(), 1);
    }

    #[test]
    fn status_notifier_props() {
        let ind = AppIndicator::new("myapp", "myapp-tray", "My App")
            .tooltip("My App is running");
        let props = ind.to_status_notifier_props();
        assert_eq!(props["Id"],       "myapp");
        assert_eq!(props["Status"],   "Active");
        assert_eq!(props["IconName"], "myapp-tray");
    }
}
