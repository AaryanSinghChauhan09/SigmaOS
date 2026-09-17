use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

/// Sovereign Universal Desktop Navigation Engine for SigmaOS
/// Inspired by Linux & BSD desktop navigation paradigms:
/// - GNOME Shell / Pop!_OS COSMIC App Launcher
/// - KDE KRunner / Rofi Universal Command HUD
/// - Ranger / Dolphin Miller Column Spatial File Manager Navigation
/// - i3 / Hyprland Directional Tiling Window Focus Navigation
/// - openSUSE YaST / FreeBSD bsdconfig System Settings Tree Navigation

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavigationMode {
    Launcher,    // Pop!_OS COSMIC / GNOME Shell fuzzy launcher
    CommandHud,  // KDE KRunner / Rofi calc, command & window launcher
    SpatialFile, // Ranger / Dolphin Miller column navigation
    TilingWindow,// i3 / Hyprland directional focus switcher
    ControlTree, // openSUSE YaST / FreeBSD bsdconfig settings navigator
}

/// GNOME / Pop!_OS COSMIC Launcher Navigation
#[derive(Debug, Clone)]
pub struct AppLauncherItem {
    pub id: String,
    pub name: String,
    pub category: String,
    pub exec: String,
    pub icon: String,
}

#[derive(Debug, Clone, Default)]
pub struct GnomePopLauncherNav {
    pub items: Vec<AppLauncherItem>,
}

impl GnomePopLauncherNav {
    pub fn new() -> Self {
        let mut nav = Self { items: Vec::new() };
        nav.register("term", "Terminal", "System", "sigma-terminal", "utilities-terminal");
        nav.register("files", "File Manager", "System", "sigma-fm", "system-file-manager");
        nav.register("settings", "Settings", "Setup", "sigma-control", "preferences-system");
        nav
    }

    pub fn register(&mut self, id: &str, name: &str, category: &str, exec: &str, icon: &str) {
        self.items.push(AppLauncherItem {
            id: id.to_string(),
            name: name.to_string(),
            category: category.to_string(),
            exec: exec.to_string(),
            icon: icon.to_string(),
        });
    }

    pub fn search(&self, query: &str) -> Vec<&AppLauncherItem> {
        let q = query.to_lowercase();
        self.items
            .iter()
            .filter(|i| i.name.to_lowercase().contains(&q) || i.category.to_lowercase().contains(&q))
            .collect()
    }
}

/// KDE KRunner / Rofi Universal Command HUD
#[derive(Debug, Clone)]
pub struct CommandHudEntry {
    pub prefix: String, // e.g. "=", ">", "win"
    pub description: String,
    pub action: String,
}

#[derive(Debug, Clone, Default)]
pub struct KrunnerRofiCommandHud {
    pub entries: Vec<CommandHudEntry>,
}

impl KrunnerRofiCommandHud {
    pub fn new() -> Self {
        let mut hud = Self { entries: Vec::new() };
        hud.entries.push(CommandHudEntry {
            prefix: "=".to_string(),
            description: "Calculator Evaluator".to_string(),
            action: "calc".to_string(),
        });
        hud.entries.push(CommandHudEntry {
            prefix: ">".to_string(),
            description: "Shell Command Execution".to_string(),
            action: "exec".to_string(),
        });
        hud.entries.push(CommandHudEntry {
            prefix: "win".to_string(),
            description: "Active Window Focus Switcher".to_string(),
            action: "focus_window".to_string(),
        });
        hud
    }

    pub fn query(&self, input: &str) -> String {
        if input.starts_with('=') {
            format!("HUD Result: Eval expression '{}'", &input[1..])
        } else if input.starts_with('>') {
            format!("HUD Exec: Running command '{}'", &input[1..])
        } else {
            format!("HUD Match: Searching '{}'", input)
        }
    }
}

/// Ranger / Dolphin Miller Column Spatial Navigation
#[derive(Debug, Clone)]
pub struct FileNode {
    pub name: String,
    pub is_dir: bool,
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct RangerDolphinSpatialFileNav {
    pub current_path: String,
    pub history: Vec<String>,
}

impl RangerDolphinSpatialFileNav {
    pub fn new(root_path: &str) -> Self {
        Self {
            current_path: root_path.to_string(),
            history: vec![root_path.to_string()],
        }
    }

    pub fn navigate_into(&mut self, dir_name: &str) {
        if self.current_path.ends_with('/') {
            self.current_path = format!("{}{}", self.current_path, dir_name);
        } else {
            self.current_path = format!("{}/{}", self.current_path, dir_name);
        }
        self.history.push(self.current_path.clone());
    }

    pub fn navigate_parent(&mut self) -> Option<String> {
        if let Some(pos) = self.current_path.rfind('/') {
            if pos == 0 {
                self.current_path = "/".to_string();
            } else {
                self.current_path = self.current_path[..pos].to_string();
            }
            self.history.push(self.current_path.clone());
            Some(self.current_path.clone())
        } else {
            None
        }
    }
}

/// i3 / Hyprland Directional Tiling Window Navigation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone)]
pub struct WindowNode {
    pub id: u32,
    pub title: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Default)]
pub struct TilingWindowManagerNav {
    pub windows: Vec<WindowNode>,
    pub focused_id: Option<u32>,
}

impl TilingWindowManagerNav {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            focused_id: None,
        }
    }

    pub fn add_window(&mut self, id: u32, title: &str, x: i32, y: i32) {
        self.windows.push(WindowNode {
            id,
            title: title.to_string(),
            x,
            y,
        });
        if self.focused_id.is_none() {
            self.focused_id = Some(id);
        }
    }

    pub fn focus_dir(&mut self, dir: FocusDirection) -> Option<u32> {
        let current = self.windows.iter().find(|w| Some(w.id) == self.focused_id)?;

        let target = match dir {
            FocusDirection::Left => self
                .windows
                .iter()
                .filter(|w| w.x < current.x)
                .min_by_key(|w| (current.x - w.x).abs()),
            FocusDirection::Right => self
                .windows
                .iter()
                .filter(|w| w.x > current.x)
                .min_by_key(|w| (w.x - current.x).abs()),
            FocusDirection::Up => self
                .windows
                .iter()
                .filter(|w| w.y < current.y)
                .min_by_key(|w| (current.y - w.y).abs()),
            FocusDirection::Down => self
                .windows
                .iter()
                .filter(|w| w.y > current.y)
                .min_by_key(|w| (w.y - current.y).abs()),
        };

        if let Some(next_win) = target {
            self.focused_id = Some(next_win.id);
            Some(next_win.id)
        } else {
            self.focused_id
        }
    }
}

/// openSUSE YaST / FreeBSD bsdconfig Control Tree Navigation
#[derive(Debug, Clone)]
pub struct ControlTreeNode {
    pub id: String,
    pub label: String,
    pub module_path: String,
}

#[derive(Debug, Clone, Default)]
pub struct YastBsdConfigControlTreeNav {
    pub modules: Vec<ControlTreeNode>,
}

impl YastBsdConfigControlTreeNav {
    pub fn new() -> Self {
        let mut tree = Self { modules: Vec::new() };
        tree.modules.push(ControlTreeNode {
            id: "net".to_string(),
            label: "Network Devices & Interfaces".to_string(),
            module_path: "yast::network".to_string(),
        });
        tree.modules.push(ControlTreeNode {
            id: "storage".to_string(),
            label: "Storage & ZFS Partitioning".to_string(),
            module_path: "yast::storage".to_string(),
        });
        tree.modules.push(ControlTreeNode {
            id: "sec".to_string(),
            label: "Security & Landlock/Pledge Auditing".to_string(),
            module_path: "yast::security".to_string(),
        });
        tree
    }

    pub fn lookup(&self, query: &str) -> Vec<&ControlTreeNode> {
        let q = query.to_lowercase();
        self.modules
            .iter()
            .filter(|m| m.label.to_lowercase().contains(&q) || m.id.to_lowercase().contains(&q))
            .collect()
    }
}

/// Master Universal Navigation Engine combining all Linux & BSD desktop navigation paradigms
pub struct SovereignUniversalNavigationEngine {
    pub active_mode: NavigationMode,
    pub launcher: GnomePopLauncherNav,
    pub hud: KrunnerRofiCommandHud,
    pub spatial_file: RangerDolphinSpatialFileNav,
    pub tiling_wm: TilingWindowManagerNav,
    pub control_tree: YastBsdConfigControlTreeNav,
}

impl SovereignUniversalNavigationEngine {
    pub fn new() -> Self {
        Self {
            active_mode: NavigationMode::Launcher,
            launcher: GnomePopLauncherNav::new(),
            hud: KrunnerRofiCommandHud::new(),
            spatial_file: RangerDolphinSpatialFileNav::new("/home/sovereign"),
            tiling_wm: TilingWindowManagerNav::new(),
            control_tree: YastBsdConfigControlTreeNav::new(),
        }
    }

    pub fn set_mode(&mut self, mode: NavigationMode) {
        self.active_mode = mode;
    }
}

impl Default for SovereignUniversalNavigationEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_launcher_nav() {
        let nav = GnomePopLauncherNav::new();
        let matches = nav.search("term");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].id, "term");
    }

    #[test]
    fn test_command_hud() {
        let hud = KrunnerRofiCommandHud::new();
        let res = hud.query("=2+2");
        assert!(res.contains("2+2"));
    }

    #[test]
    fn test_spatial_file_nav() {
        let mut nav = RangerDolphinSpatialFileNav::new("/home/user");
        nav.navigate_into("documents");
        assert_eq!(nav.current_path, "/home/user/documents");
        nav.navigate_parent();
        assert_eq!(nav.current_path, "/home/user");
    }

    #[test]
    fn test_tiling_wm_focus_nav() {
        let mut wm = TilingWindowManagerNav::new();
        wm.add_window(1, "Term Left", 0, 0);
        wm.add_window(2, "Editor Right", 800, 0);

        let focused = wm.focus_dir(FocusDirection::Right);
        assert_eq!(focused, Some(2));
    }

    #[test]
    fn test_control_tree_nav() {
        let tree = YastBsdConfigControlTreeNav::new();
        let results = tree.lookup("network");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "net");
    }
}
