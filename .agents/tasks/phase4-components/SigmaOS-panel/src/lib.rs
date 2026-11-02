//! SigmaOS Panel — desktop status bar with applets, indicators, and tray support.
//!
//! Wayland layer-shell-based panel providing system info widgets, workspace
//! indicators, notification badge, and an extensible applet plugin API.

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::fmt;

// ---------------------------------------------------------------------------
// Applet position
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PanelEdge {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AppletZone {
    Start,   // left / top
    Center,
    End,     // right / bottom
}

// ---------------------------------------------------------------------------
// Applet
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct AppletInfo {
    pub id: String,
    pub label: String,
    pub zone: AppletZone,
    pub visible: bool,
    pub priority: u32,   // lower = earlier in zone
}

impl AppletInfo {
    pub fn new(id: impl Into<String>, label: impl Into<String>, zone: AppletZone) -> Self {
        Self { id: id.into(), label: label.into(), zone, visible: true, priority: 50 }
    }
}

// ---------------------------------------------------------------------------
// Built-in applets
// ---------------------------------------------------------------------------

fn default_applets() -> Vec<AppletInfo> {
    vec![
        AppletInfo { id: "launcher".into(), label: "⊞ SigmaOS".into(), zone: AppletZone::Start, visible: true, priority: 10 },
        AppletInfo { id: "workspace-switcher".into(), label: "Workspaces".into(), zone: AppletZone::Start, visible: true, priority: 20 },
        AppletInfo { id: "app-list".into(), label: "Windows".into(), zone: AppletZone::Center, visible: true, priority: 50 },
        AppletInfo { id: "notification-bell".into(), label: "🔔".into(), zone: AppletZone::End, visible: true, priority: 10 },
        AppletInfo { id: "network".into(), label: "📶".into(), zone: AppletZone::End, visible: true, priority: 20 },
        AppletInfo { id: "audio".into(), label: "🔊".into(), zone: AppletZone::End, visible: true, priority: 30 },
        AppletInfo { id: "battery".into(), label: "🔋".into(), zone: AppletZone::End, visible: true, priority: 40 },
        AppletInfo { id: "clock".into(), label: "00:00".into(), zone: AppletZone::End, visible: true, priority: 50 },
        AppletInfo { id: "systray".into(), label: "Tray".into(), zone: AppletZone::End, visible: true, priority: 60 },
    ]
}

// ---------------------------------------------------------------------------
// Panel
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelState {
    Initializing,
    Running,
    Hidden,
    Stopped,
}

pub struct Panel {
    pub edge: PanelEdge,
    pub height_px: u32,
    pub state: PanelState,
    applets: Vec<AppletInfo>,
}

impl Panel {
    pub fn new(edge: PanelEdge) -> Self {
        Self {
            edge,
            height_px: 36,
            state: PanelState::Initializing,
            applets: default_applets(),
        }
    }

    pub fn start(&mut self) {
        self.state = PanelState::Running;
    }

    pub fn hide(&mut self) {
        self.state = PanelState::Hidden;
    }

    pub fn stop(&mut self) {
        self.state = PanelState::Stopped;
    }

    /// Add a custom applet.
    pub fn add_applet(&mut self, applet: AppletInfo) {
        self.applets.retain(|a| a.id != applet.id);
        self.applets.push(applet);
    }

    /// Remove an applet by id.
    pub fn remove_applet(&mut self, id: &str) -> bool {
        let before = self.applets.len();
        self.applets.retain(|a| a.id != id);
        self.applets.len() < before
    }

    /// Get applets in a zone, sorted by priority.
    pub fn applets_in(&self, zone: AppletZone) -> Vec<&AppletInfo> {
        let mut v: Vec<&AppletInfo> = self.applets.iter()
            .filter(|a| a.zone == zone && a.visible)
            .collect();
        v.sort_by_key(|a| a.priority);
        v
    }

    pub fn applet_count(&self) -> usize {
        self.applets.len()
    }

    /// Render a simple text layout of the panel (for testing without Wayland).
    pub fn render_text(&self) -> String {
        let start: Vec<_> = self.applets_in(AppletZone::Start).iter().map(|a| a.label.as_str()).collect();
        let center: Vec<_> = self.applets_in(AppletZone::Center).iter().map(|a| a.label.as_str()).collect();
        let end: Vec<_> = self.applets_in(AppletZone::End).iter().map(|a| a.label.as_str()).collect();
        format!("[{}] | [{}] | [{}]",
            start.join(" "), center.join(" "), end.join(" "))
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

pub fn init_panel(edge: PanelEdge) -> Panel {
    let mut p = Panel::new(edge);
    p.start();
    p
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panel_starts_with_default_applets() {
        let p = init_panel(PanelEdge::Top);
        assert!(p.applet_count() > 0);
        assert_eq!(p.state, PanelState::Running);
    }

    #[test]
    fn applets_in_zone_sorted_by_priority() {
        let p = init_panel(PanelEdge::Top);
        let end = p.applets_in(AppletZone::End);
        assert!(!end.is_empty());
        // priority ascending
        assert!(end.windows(2).all(|w| w[0].priority <= w[1].priority));
    }

    #[test]
    fn add_and_remove_applet() {
        let mut p = init_panel(PanelEdge::Bottom);
        let initial = p.applet_count();
        p.add_applet(AppletInfo::new("test-applet", "Test", AppletZone::Center));
        assert_eq!(p.applet_count(), initial + 1);
        assert!(p.remove_applet("test-applet"));
        assert_eq!(p.applet_count(), initial);
    }

    #[test]
    fn render_text_contains_applet_labels() {
        let p = init_panel(PanelEdge::Top);
        let rendered = p.render_text();
        assert!(rendered.contains("SigmaOS") || rendered.contains("⊞"));
    }

    #[test]
    fn panel_edge_variants() {
        for edge in [PanelEdge::Top, PanelEdge::Bottom, PanelEdge::Left, PanelEdge::Right] {
            let p = Panel::new(edge);
            assert_eq!(p.edge, edge);
        }
    }
}
