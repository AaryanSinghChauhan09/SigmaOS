//! SigmaOS Panel
//!
//! Desktop status bar / panel with an applet registry and IPC-ready
//! message bus stub. Mirrors the Waybar / GNOME Shell panel model.
//! Upstream: Waybar (https://github.com/Alexays/Waybar),
//!           GNOME Shell panel (https://gitlab.gnome.org/GNOME/gnome-shell)

use std::collections::HashMap;

/// Position of the panel on screen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PanelPosition { Top, Bottom }

/// A panel applet with a rendered label / icon string.
#[derive(Debug, Clone)]
pub struct Applet {
    pub id:       String,
    pub label:    String,
    pub tooltip:  String,
    pub visible:  bool,
    pub priority: u8,   // lower = displayed first (left-to-right)
}

impl Applet {
    pub fn new(id: &str, label: &str, priority: u8) -> Self {
        Self {
            id:      id.to_owned(),
            label:   label.to_owned(),
            tooltip: String::new(),
            visible: true,
            priority,
        }
    }
    pub fn tooltip(mut self, t: &str) -> Self { self.tooltip = t.to_owned(); self }
    pub fn hidden(mut self) -> Self { self.visible = false; self }
}

/// Panel message passed over IPC.
#[derive(Debug, Clone)]
pub struct PanelMessage {
    pub applet_id: String,
    pub payload:   String,
}

/// The panel itself: holds the applet registry and IPC queue.
pub struct Panel {
    pub position:  PanelPosition,
    pub height_px: u32,
    applets:       HashMap<String, Applet>,
    messages:      Vec<PanelMessage>,
}

impl Panel {
    pub fn new(position: PanelPosition, height_px: u32) -> Self {
        Self {
            position,
            height_px,
            applets:  HashMap::new(),
            messages: Vec::new(),
        }
    }

    /// Register an applet with the panel.
    pub fn register(&mut self, applet: Applet) -> bool {
        if self.applets.contains_key(&applet.id) {
            return false;  // already registered
        }
        self.applets.insert(applet.id.clone(), applet);
        true
    }

    /// Unregister an applet by ID.
    pub fn unregister(&mut self, id: &str) -> bool {
        self.applets.remove(id).is_some()
    }

    /// Return visible applets ordered by priority.
    pub fn visible_applets(&self) -> Vec<&Applet> {
        let mut v: Vec<&Applet> = self.applets.values()
            .filter(|a| a.visible)
            .collect();
        v.sort_by_key(|a| a.priority);
        v
    }

    /// Post a message from an applet (IPC stub).
    pub fn post(&mut self, applet_id: &str, payload: &str) -> bool {
        if !self.applets.contains_key(applet_id) {
            return false;
        }
        self.messages.push(PanelMessage {
            applet_id: applet_id.to_owned(),
            payload:   payload.to_owned(),
        });
        true
    }

    pub fn applet_count(&self) -> usize { self.applets.len() }
    pub fn message_count(&self) -> usize { self.messages.len() }

    /// Render the panel as a simple string (for headless/debug use).
    pub fn render(&self) -> String {
        self.visible_applets()
            .iter()
            .map(|a| format!("[{}]", a.label))
            .collect::<Vec<_>>()
            .join(" | ")
    }
}

pub fn init() {
    let mut panel = Panel::new(PanelPosition::Top, 30);
    panel.register(Applet::new("clock",      "12:00",   10));
    panel.register(Applet::new("workspaces", "1 2 3",    1));
    panel.register(Applet::new("battery",    "🔋 80%", 20));
    println!("[panel] {}", panel.render());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_count() {
        let mut p = Panel::new(PanelPosition::Top, 30);
        assert!(p.register(Applet::new("clock", "12:00", 10)));
        assert_eq!(p.applet_count(), 1);
        // Duplicate rejected
        assert!(!p.register(Applet::new("clock", "13:00", 10)));
        assert_eq!(p.applet_count(), 1);
    }

    #[test]
    fn unregister() {
        let mut p = Panel::new(PanelPosition::Top, 30);
        p.register(Applet::new("tray", "🖥", 5));
        assert!(p.unregister("tray"));
        assert!(!p.unregister("tray"));
    }

    #[test]
    fn visible_ordering() {
        let mut p = Panel::new(PanelPosition::Top, 30);
        p.register(Applet::new("z-last",  "Z", 99));
        p.register(Applet::new("a-first", "A",  1));
        p.register(Applet::new("m-mid",   "M", 50));
        let ids: Vec<&str> = p.visible_applets().iter().map(|a| a.id.as_str()).collect();
        assert_eq!(ids, ["a-first", "m-mid", "z-last"]);
    }

    #[test]
    fn hidden_applet_excluded() {
        let mut p = Panel::new(PanelPosition::Bottom, 24);
        p.register(Applet::new("visible", "V", 1));
        p.register(Applet::new("hidden", "H", 2).hidden());
        assert_eq!(p.visible_applets().len(), 1);
    }

    #[test]
    fn post_message() {
        let mut p = Panel::new(PanelPosition::Top, 30);
        p.register(Applet::new("battery", "🔋", 20));
        assert!(p.post("battery", r#"{"level":80}"#));
        assert!(!p.post("unknown", "data"));
        assert_eq!(p.message_count(), 1);
    }

    #[test]
    fn render_output() {
        let mut p = Panel::new(PanelPosition::Top, 30);
        p.register(Applet::new("ws",    "1 2",  1));
        p.register(Applet::new("clock", "9:00", 2));
        let rendered = p.render();
        assert!(rendered.contains("1 2"));
        assert!(rendered.contains("9:00"));
    }
}
