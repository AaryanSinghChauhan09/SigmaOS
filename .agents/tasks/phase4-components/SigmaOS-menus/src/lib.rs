//! SigmaOS Menus
//!
//! Desktop application menu model: hierarchical menus with items,
//! separators, and keyboard accelerators. Backend-agnostic; renderers
//! (GTK4, Wayland layer-shell popup) consume this model.
//! Upstream: GMenu (https://docs.gtk.org/gio/class.Menu.html),
//!           D-Bus application menu spec (https://wiki.ubuntu.com/AppMenu)

use std::collections::HashMap;

/// A keyboard accelerator (e.g. Ctrl+S).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Accelerator {
    pub key:   String,
    pub ctrl:  bool,
    pub shift: bool,
    pub alt:   bool,
}

impl Accelerator {
    pub fn ctrl(key: &str) -> Self {
        Self { key: key.to_owned(), ctrl: true, shift: false, alt: false }
    }
    pub fn display(&self) -> String {
        let mut parts = Vec::new();
        if self.ctrl  { parts.push("Ctrl");  }
        if self.shift { parts.push("Shift"); }
        if self.alt   { parts.push("Alt");   }
        parts.push(&self.key);
        parts.join("+")
    }
}

/// A single entry in a menu.
#[derive(Debug, Clone)]
pub enum MenuItem {
    Action {
        id:          String,
        label:       String,
        enabled:     bool,
        accel:       Option<Accelerator>,
    },
    Submenu {
        label:   String,
        menu:    Menu,
    },
    Separator,
}

/// A hierarchical menu.
#[derive(Debug, Clone, Default)]
pub struct Menu {
    pub id:    String,
    items:     Vec<MenuItem>,
}

impl Menu {
    pub fn new(id: &str) -> Self {
        Self { id: id.to_owned(), items: Vec::new() }
    }

    pub fn action(mut self, id: &str, label: &str) -> Self {
        self.items.push(MenuItem::Action {
            id: id.to_owned(), label: label.to_owned(),
            enabled: true, accel: None,
        });
        self
    }

    pub fn action_accel(mut self, id: &str, label: &str, accel: Accelerator) -> Self {
        self.items.push(MenuItem::Action {
            id: id.to_owned(), label: label.to_owned(),
            enabled: true, accel: Some(accel),
        });
        self
    }

    pub fn separator(mut self) -> Self {
        self.items.push(MenuItem::Separator);
        self
    }

    pub fn submenu(mut self, label: &str, sub: Menu) -> Self {
        self.items.push(MenuItem::Submenu { label: label.to_owned(), menu: sub });
        self
    }

    pub fn items(&self) -> &[MenuItem] { &self.items }

    pub fn item_count(&self) -> usize {
        self.items.iter().filter(|i| !matches!(i, MenuItem::Separator)).count()
    }

    /// Flat list of all action IDs (depth-first, recursive into submenus).
    pub fn all_action_ids(&self) -> Vec<&str> {
        let mut ids = Vec::new();
        for item in &self.items {
            match item {
                MenuItem::Action { id, .. } => ids.push(id.as_str()),
                MenuItem::Submenu { menu, .. } => ids.extend(menu.all_action_ids()),
                MenuItem::Separator => {}
            }
        }
        ids
    }
}

/// A menu bar: a named collection of top-level menus.
#[derive(Debug, Default)]
pub struct MenuBar {
    menus: Vec<Menu>,
}

impl MenuBar {
    pub fn new() -> Self { Self::default() }
    pub fn add(mut self, menu: Menu) -> Self { self.menus.push(menu); self }
    pub fn menus(&self) -> &[Menu] { &self.menus }
    pub fn find(&self, id: &str) -> Option<&Menu> {
        self.menus.iter().find(|m| m.id == id)
    }
}

/// Build the default SigmaOS application menu bar.
pub fn default_menu_bar() -> MenuBar {
    let file = Menu::new("file")
        .action_accel("new",  "New",  Accelerator::ctrl("N"))
        .action_accel("open", "Open", Accelerator::ctrl("O"))
        .separator()
        .action_accel("save", "Save", Accelerator::ctrl("S"))
        .action_accel("save-as", "Save As…", Accelerator { key: "S".into(), ctrl: true, shift: true, alt: false })
        .separator()
        .action_accel("quit", "Quit", Accelerator::ctrl("Q"));

    let edit = Menu::new("edit")
        .action_accel("undo",  "Undo",  Accelerator::ctrl("Z"))
        .action_accel("redo",  "Redo",  Accelerator { key: "Z".into(), ctrl: true, shift: true, alt: false })
        .separator()
        .action_accel("cut",   "Cut",   Accelerator::ctrl("X"))
        .action_accel("copy",  "Copy",  Accelerator::ctrl("C"))
        .action_accel("paste", "Paste", Accelerator::ctrl("V"))
        .separator()
        .action_accel("find",  "Find…", Accelerator::ctrl("F"));

    let view = Menu::new("view")
        .action("zoom-in",  "Zoom In")
        .action("zoom-out", "Zoom Out")
        .action("zoom-fit", "Fit to Window")
        .separator()
        .action("fullscreen", "Fullscreen");

    let help = Menu::new("help")
        .action("about",       "About SigmaOS")
        .action("user-guide",  "User Guide")
        .action("report-bug",  "Report a Bug");

    MenuBar::new().add(file).add(edit).add(view).add(help)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_item_count_excludes_separators() {
        let m = Menu::new("file")
            .action("new", "New")
            .separator()
            .action("quit", "Quit");
        assert_eq!(m.item_count(), 2);
        assert_eq!(m.items().len(), 3);
    }

    #[test]
    fn accelerator_display() {
        let a = Accelerator::ctrl("S");
        assert_eq!(a.display(), "Ctrl+S");
        let b = Accelerator { key: "Z".into(), ctrl: true, shift: true, alt: false };
        assert_eq!(b.display(), "Ctrl+Shift+Z");
    }

    #[test]
    fn all_action_ids_recurse() {
        let sub = Menu::new("submenu").action("sub-action", "Sub");
        let m = Menu::new("root")
            .action("top-action", "Top")
            .submenu("More", sub);
        let ids = m.all_action_ids();
        assert!(ids.contains(&"top-action"));
        assert!(ids.contains(&"sub-action"));
    }

    #[test]
    fn default_menu_bar_structure() {
        let bar = default_menu_bar();
        assert!(bar.find("file").is_some());
        assert!(bar.find("edit").is_some());
        assert!(bar.find("help").is_some());
        let file_ids = bar.find("file").unwrap().all_action_ids();
        assert!(file_ids.contains(&"quit"));
        assert!(file_ids.contains(&"save"));
    }
}
