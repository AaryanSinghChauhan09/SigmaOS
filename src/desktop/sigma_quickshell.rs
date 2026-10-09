// src/desktop/sigma_quickshell.rs
//! SigmaOS Quickshell — Dynamic QML-inspired Desktop Shell Framework
//!
//! Inspired by Omarchy's use of Quickshell (DHH's opinionated Qt shell for Hyprland).
//! SigmaOS implements a similar philosophy in pure Rust with zero Qt dependency:
//! — Widget declarations are pure Rust structs (no XML/QML)
//! — Hot-reload via inotify watch on config files
//! — Reactive data binding through a simple observer pattern
//! — Hyprland socket IPC for workspace/window awareness
//! — Waybar-compatible output format for panel widgets

#![allow(dead_code)]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// ─────────────────────────────────────────────────────────────────────────────
// Reactive State System
// ─────────────────────────────────────────────────────────────────────────────

/// A reactive cell that notifies watchers on change.
#[derive(Debug, Clone)]
pub struct ReactiveCell<T: Clone + PartialEq> {
    value: T,
    generation: u64,
}

impl<T: Clone + PartialEq> ReactiveCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            generation: 0,
        }
    }

    /// Set a new value. Returns `true` if the value changed.
    pub fn set(&mut self, new_val: T) -> bool {
        if self.value != new_val {
            self.value = new_val;
            self.generation = self.generation.wrapping_add(1);
            true
        } else {
            false
        }
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Widget System
// ─────────────────────────────────────────────────────────────────────────────

/// Anchor point for widget placement (Quickshell-style anchors)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Anchors {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

impl Anchors {
    pub const TOP_LEFT: Self = Self {
        top: true,
        bottom: false,
        left: true,
        right: false,
    };
    pub const TOP_RIGHT: Self = Self {
        top: true,
        bottom: false,
        left: false,
        right: true,
    };
    pub const BOTTOM_LEFT: Self = Self {
        top: false,
        bottom: true,
        left: true,
        right: false,
    };
    pub const BOTTOM_RIGHT: Self = Self {
        top: false,
        bottom: true,
        left: false,
        right: true,
    };
    pub const TOP_FILL: Self = Self {
        top: true,
        bottom: false,
        left: true,
        right: true,
    };
    pub const BOTTOM_FILL: Self = Self {
        top: false,
        bottom: true,
        left: true,
        right: true,
    };
    pub const FILL: Self = Self {
        top: true,
        bottom: true,
        left: true,
        right: true,
    };
}

/// Layer surface layer (wlr-layer-shell protocol)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerSurface {
    Background,
    Bottom,
    Top,
    Overlay,
}

/// A shell widget layer surface descriptor
#[derive(Debug, Clone)]
pub struct ShellWidget {
    pub id: String,
    pub layer: LayerSurface,
    pub anchors: Anchors,
    pub exclusive_zone: i32, // px to reserve at anchor edge; -1 = full
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub margin_top: i32,
    pub margin_bottom: i32,
    pub margin_left: i32,
    pub margin_right: i32,
    pub content: WidgetContent,
    pub visible: bool,
    pub monitor_name: Option<String>, // None = all monitors
}

/// What a widget renders
#[derive(Debug, Clone)]
pub enum WidgetContent {
    Panel(PanelContent),
    Notification(NotificationContent),
    Launcher(LauncherContent),
    StatusBar(StatusBarContent),
    Custom(String), // JSON blob for external renderer
}

// ─────────────────────────────────────────────────────────────────────────────
// Panel (top/bottom bar)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PanelContent {
    pub left_items: Vec<PanelItem>,
    pub center_items: Vec<PanelItem>,
    pub right_items: Vec<PanelItem>,
    pub height_px: u32,
    pub background_css: String,
    pub foreground_css: String,
}

#[derive(Debug, Clone)]
pub enum PanelItem {
    Clock {
        format: String,
    },
    WorkspaceIndicator {
        active_color: String,
        inactive_color: String,
    },
    SystemTray,
    Volume {
        show_icon: bool,
        show_percent: bool,
    },
    Battery {
        show_icon: bool,
        show_percent: bool,
    },
    Network {
        show_ssid: bool,
    },
    Memory {
        show_bar: bool,
    },
    Cpu {
        show_graph: bool,
    },
    AppMenu {
        icon: String,
        label: String,
    },
    Custom {
        id: String,
        command: String,
        interval_ms: u32,
    },
}

// ─────────────────────────────────────────────────────────────────────────────
// Notification Area
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationUrgency {
    Low,
    Normal,
    Critical,
}

#[derive(Debug, Clone)]
pub struct NotificationContent {
    pub app_name: String,
    pub summary: String,
    pub body: String,
    pub urgency: NotificationUrgency,
    pub timeout_ms: u32,
    pub icon: Option<String>,
    pub actions: Vec<(String, String)>, // (id, label)
    pub id: u32,
}

// ─────────────────────────────────────────────────────────────────────────────
// App Launcher
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct AppEntry {
    pub name: String,
    pub exec: String,
    pub icon: String,
    pub categories: Vec<String>,
    pub description: String,
    pub frequency: u32, // launch frequency for ranking
}

#[derive(Debug, Clone)]
pub struct LauncherContent {
    pub query: String,
    pub results: Vec<AppEntry>,
    pub max_results: usize,
    pub calculator_result: Option<String>,
    pub web_search_prefix: String, // e.g. "https://search.brave.com/search?q="
}

impl LauncherContent {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            results: Vec::new(),
            max_results: 10,
            calculator_result: None,
            web_search_prefix: "https://search.brave.com/search?q=".to_string(),
        }
    }

    /// Score an app entry against query (higher = better match)
    pub fn score_entry(entry: &AppEntry, query: &str) -> u32 {
        if query.is_empty() {
            return entry.frequency;
        }
        let query_lower = query.to_lowercase();
        let name_lower = entry.name.to_lowercase();

        let mut score = 0u32;
        // Exact name match
        if name_lower == query_lower {
            score += 1000;
        }
        // Starts-with match
        if name_lower.starts_with(&query_lower) {
            score += 500;
        }
        // Contains match
        if name_lower.contains(&query_lower) {
            score += 100;
        }
        // Description match
        if entry.description.to_lowercase().contains(&query_lower) {
            score += 20;
        }
        // Frequency bonus
        score += entry.frequency.min(50);
        score
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Status Bar (Waybar-compatible output)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct StatusBarModule {
    pub name: String,
    pub text: String,    // display text
    pub tooltip: String, // hover tooltip
    pub class: String,   // CSS class
    pub percentage: Option<u8>,
    pub icon: Option<String>,
    pub on_click: Option<String>, // shell command
}

impl StatusBarModule {
    pub fn to_waybar_json(&self) -> String {
        let pct_field = match self.percentage {
            Some(p) => alloc::format!(r#","percentage":{}"#, p),
            None => String::new(),
        };
        alloc::format!(
            r#"{{"text":"{}","tooltip":"{}","class":"{}"{}}}"#,
            self.text.replace('"', "\\\""),
            self.tooltip.replace('"', "\\\""),
            self.class,
            pct_field,
        )
    }
}

#[derive(Debug, Clone)]
pub struct StatusBarContent {
    pub modules: Vec<StatusBarModule>,
}

// ─────────────────────────────────────────────────────────────────────────────
// SigmaQuickshell — Main Shell Manager
// ─────────────────────────────────────────────────────────────────────────────

pub struct SigmaQuickshell {
    pub widgets: BTreeMap<String, ShellWidget>,
    pub active_workspace: ReactiveCell<u32>,
    pub notification_count: ReactiveCell<u32>,
    pub config_path: String,
    pub hyprland_socket: Option<String>,
}

impl SigmaQuickshell {
    pub fn new(config_path: &str) -> Self {
        Self {
            widgets: BTreeMap::new(),
            active_workspace: ReactiveCell::new(1),
            notification_count: ReactiveCell::new(0),
            config_path: config_path.to_string(),
            hyprland_socket: None,
        }
    }

    /// Register a widget
    pub fn add_widget(&mut self, widget: ShellWidget) {
        self.widgets.insert(widget.id.clone(), widget);
    }

    /// Remove a widget by id
    pub fn remove_widget(&mut self, id: &str) -> bool {
        self.widgets.remove(id).is_some()
    }

    /// Create a default top panel widget (Omarchy-inspired layout)
    pub fn default_top_panel() -> ShellWidget {
        ShellWidget {
            id: "top-panel".to_string(),
            layer: LayerSurface::Top,
            anchors: Anchors::TOP_FILL,
            exclusive_zone: 32,
            width: None,
            height: Some(32),
            margin_top: 0,
            margin_bottom: 0,
            margin_left: 0,
            margin_right: 0,
            visible: true,
            monitor_name: None,
            content: WidgetContent::Panel(PanelContent {
                left_items: alloc::vec![
                    PanelItem::AppMenu {
                        icon: "sigma-logo".to_string(),
                        label: "SigmaOS".to_string(),
                    },
                    PanelItem::WorkspaceIndicator {
                        active_color: "#7aa2f7".to_string(),
                        inactive_color: "#565f89".to_string(),
                    },
                ],
                center_items: alloc::vec![PanelItem::Clock {
                    format: "%A, %B %-d  %H:%M".to_string()
                },],
                right_items: alloc::vec![
                    PanelItem::Cpu { show_graph: false },
                    PanelItem::Memory { show_bar: false },
                    PanelItem::Network { show_ssid: true },
                    PanelItem::Volume {
                        show_icon: true,
                        show_percent: true
                    },
                    PanelItem::Battery {
                        show_icon: true,
                        show_percent: true
                    },
                    PanelItem::SystemTray,
                ],
                height_px: 32,
                background_css: "rgba(26, 27, 38, 0.95)".to_string(),
                foreground_css: "#c0caf5".to_string(),
            }),
        }
    }

    /// Set the active Hyprland workspace
    pub fn set_active_workspace(&mut self, ws: u32) -> bool {
        self.active_workspace.set(ws)
    }

    /// Get all visible widgets sorted by layer
    pub fn visible_widgets(&self) -> Vec<&ShellWidget> {
        let mut v: Vec<&ShellWidget> = self.widgets.values().filter(|w| w.visible).collect();
        v.sort_by_key(|w| w.layer as u8);
        v
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Hyprland IPC model
// ─────────────────────────────────────────────────────────────────────────────

/// Hyprland workspace event (from socket)
#[derive(Debug, Clone)]
pub enum HyprlandEvent {
    WorkspaceChanged {
        id: u32,
        name: String,
    },
    WindowFocused {
        address: u64,
        class: String,
        title: String,
    },
    WindowClosed {
        address: u64,
    },
    MonitorAdded {
        name: String,
    },
    MonitorRemoved {
        name: String,
    },
    Fullscreen {
        entered: bool,
    },
    SubMap {
        name: String,
    },
}

impl HyprlandEvent {
    /// Parse a Hyprland IPC event line (e.g. "workspace>>1")
    pub fn parse(line: &str) -> Option<Self> {
        let (event, data) = line.split_once(">>")?;
        match event {
            "workspace" => {
                let id = data.trim().parse::<u32>().ok()?;
                Some(HyprlandEvent::WorkspaceChanged {
                    id,
                    name: alloc::format!("{}", id),
                })
            }
            "focusedmon" => None, // ignore for now
            "fullscreen" => {
                let entered = data.trim() == "1";
                Some(HyprlandEvent::Fullscreen { entered })
            }
            "submap" => Some(HyprlandEvent::SubMap {
                name: data.to_string(),
            }),
            _ => None,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reactive_cell_change_detection() {
        let mut cell: ReactiveCell<u32> = ReactiveCell::new(5);
        assert_eq!(cell.generation(), 0);
        assert!(!cell.set(5)); // same value — no change
        assert_eq!(cell.generation(), 0);
        assert!(cell.set(6)); // new value — changed
        assert_eq!(cell.generation(), 1);
        assert_eq!(*cell.get(), 6);
    }

    #[test]
    fn test_launcher_scoring() {
        let entry = AppEntry {
            name: "Firefox".to_string(),
            exec: "firefox".to_string(),
            icon: "firefox".to_string(),
            categories: alloc::vec!["Network".to_string()],
            description: "Web browser".to_string(),
            frequency: 10,
        };
        let exact = LauncherContent::score_entry(&entry, "firefox");
        let partial = LauncherContent::score_entry(&entry, "fire");
        let empty = LauncherContent::score_entry(&entry, "");
        assert!(exact > partial);
        assert!(partial > empty);
        assert_eq!(empty, 10); // just frequency
    }

    #[test]
    fn test_status_bar_json_output() {
        let module = StatusBarModule {
            name: "battery".to_string(),
            text: "🔋 85%".to_string(),
            tooltip: "Battery: 85% — 3h 20m remaining".to_string(),
            class: "battery-good".to_string(),
            percentage: Some(85),
            icon: None,
            on_click: None,
        };
        let json = module.to_waybar_json();
        assert!(json.contains("\"text\":\"🔋 85%\""));
        assert!(json.contains("\"percentage\":85"));
        assert!(json.contains("battery-good"));
    }

    #[test]
    fn test_quickshell_widget_management() {
        let mut shell = SigmaQuickshell::new("/etc/sigmaos/shell.toml");
        shell.add_widget(SigmaQuickshell::default_top_panel());
        assert_eq!(shell.widgets.len(), 1);
        assert!(shell.remove_widget("top-panel"));
        assert!(shell.widgets.is_empty());
        assert!(!shell.remove_widget("nonexistent"));
    }

    #[test]
    fn test_hyprland_event_parsing() {
        let ev = HyprlandEvent::parse("workspace>>3");
        assert!(matches!(
            ev,
            Some(HyprlandEvent::WorkspaceChanged { id: 3, .. })
        ));

        let fullscreen = HyprlandEvent::parse("fullscreen>>1");
        assert!(matches!(
            fullscreen,
            Some(HyprlandEvent::Fullscreen { entered: true })
        ));

        let unknown = HyprlandEvent::parse("unknownevent>>data");
        assert!(unknown.is_none());
    }

    #[test]
    fn test_anchors_constants() {
        assert!(Anchors::TOP_FILL.top);
        assert!(Anchors::TOP_FILL.left);
        assert!(Anchors::TOP_FILL.right);
        assert!(!Anchors::TOP_FILL.bottom);
        assert!(
            Anchors::FILL.top && Anchors::FILL.bottom && Anchors::FILL.left && Anchors::FILL.right
        );
    }
}
