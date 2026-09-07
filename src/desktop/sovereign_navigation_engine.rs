//! Sovereign Universal Navigation Engine for SigmaOS
//!
//! Takes inspiration from top Linux & BSD distribution navigation paradigms to deliver
//! seamless, keyboard-driven, spatial, fuzzy, and hierarchical navigation across the OS:
//! - GNOME Shell / Pop!_OS COSMIC Launcher: Category-aware fuzzy application search and instant launch.
//! - KDE KRunner / Rofi Universal Command HUD: Multi-source command palette for math evaluation, file jumps, and window switching.
//! - Ranger / Dolphin Dual-Pane Miller Column Navigation: Vim-like spatial file movement (`H/J/K/L`), breadcrumbs, and fast bookmarks.
//! - i3 / Hyprland Tiling Spatial Window Navigation: Directional focus movement (Left, Right, Up, Down) and workspace switching.
//! - openSUSE YaST / FreeBSD bsdconfig Control Tree Navigation: Structured hierarchical system configuration menu navigation.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

/// Navigation direction for spatial movements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavDirection {
    Up,
    Down,
    Left,
    Right,
    Parent,
    Child,
    Next,
    Prev,
}

/// Category filter for GNOME / Pop!_OS launcher navigation
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AppCategory {
    #[default]
    All,
    System,
    Development,
    Internet,
    Office,
    Graphics,
    AudioVideo,
    Utilities,
}

/// Application launcher item entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherAppItem {
    pub id: String,
    pub name: String,
    pub exec_cmd: String,
    pub category: AppCategory,
    pub keywords: Vec<String>,
    pub launch_count: u32,
}

/// Pop!_OS / GNOME Shell inspired Launcher Navigation Engine
#[derive(Debug, Clone, Default)]
pub struct GnomePopLauncherNav {
    pub apps: Vec<LauncherAppItem>,
    pub active_category: AppCategory,
    pub selected_index: usize,
}

impl GnomePopLauncherNav {
    pub fn new() -> Self {
        let mut nav = Self {
            apps: Vec::new(),
            active_category: AppCategory::All,
            selected_index: 0,
        };
        nav.populate_default_apps();
        nav
    }

    fn populate_default_apps(&mut self) {
        self.apps.push(LauncherAppItem {
            id: String::from("sigma-term"),
            name: String::from("Sigma Terminal"),
            exec_cmd: String::from("sigterm"),
            category: AppCategory::System,
            keywords: vec![String::from("cli"), String::from("bash"), String::from("shell")],
            launch_count: 10,
        });
        self.apps.push(LauncherAppItem {
            id: String::from("sigma-files"),
            name: String::from("Sigma File Manager"),
            exec_cmd: String::from("sigfiles"),
            category: AppCategory::Utilities,
            keywords: vec![String::from("ranger"), String::from("dolphin"), String::from("explore")],
            launch_count: 8,
        });
        self.apps.push(LauncherAppItem {
            id: String::from("sigma-browser"),
            name: String::from("Sovereign Web Browser"),
            exec_cmd: String::from("sigbrowser"),
            category: AppCategory::Internet,
            keywords: vec![String::from("web"), String::from("http"), String::from("net")],
            launch_count: 15,
        });
        self.apps.push(LauncherAppItem {
            id: String::from("sigma-control"),
            name: String::from("Control Center"),
            exec_cmd: String::from("yast-control"),
            category: AppCategory::System,
            keywords: vec![String::from("yast"), String::from("bsdconfig"), String::from("settings")],
            launch_count: 5,
        });
    }

    pub fn search(&self, query: &str) -> Vec<&LauncherAppItem> {
        let query_lower = query.to_lowercase();
        let mut results: Vec<&LauncherAppItem> = self
            .apps
            .iter()
            .filter(|app| {
                if self.active_category != AppCategory::All && app.category != self.active_category {
                    return false;
                }
                if query_lower.is_empty() {
                    return true;
                }
                app.name.to_lowercase().contains(&query_lower)
                    || app.exec_cmd.to_lowercase().contains(&query_lower)
                    || app.keywords.iter().any(|k| k.to_lowercase().contains(&query_lower))
            })
            .collect();

        // Sort by launch frequency relevance
        results.sort_by(|a, b| b.launch_count.cmp(&a.launch_count));
        results
    }

    pub fn filter_by_category(&mut self, category: AppCategory) {
        self.active_category = category;
        self.selected_index = 0;
    }
}

/// Action type for KRunner / Rofi Universal Command HUD
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HudActionResult {
    LaunchApp(String),
    FocusWindow(u32),
    ExecuteSysCommand(String),
    MathCalculation(String),
    NavigatePath(String),
}

/// KRunner / Rofi inspired Universal Command HUD Navigation Engine
#[derive(Debug, Clone, Default)]
pub struct KrunnerRofiCommandHud {
    pub history: Vec<String>,
}

impl KrunnerRofiCommandHud {
    pub fn new() -> Self {
        Self { history: Vec::new() }
    }

    pub fn evaluate_input(&mut self, input: &str) -> Option<HudActionResult> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return None;
        }

        self.history.push(String::from(trimmed));

        // Quick math solver (e.g. "calc: 12 + 34" or "= 12 * 4")
        if trimmed.starts_with("calc:") || trimmed.starts_with('=') {
            let expr = trimmed.trim_start_matches("calc:").trim_start_matches('=').trim();
            let result = self.simple_math_eval(expr);
            return Some(HudActionResult::MathCalculation(result));
        }

        // Fast file path navigation (e.g. "/usr/bin" or "~/Documents")
        if trimmed.starts_with('/') || trimmed.starts_with('~') {
            return Some(HudActionResult::NavigatePath(String::from(trimmed)));
        }

        // Fast window focus switch (e.g. "win:2" or "focus 3")
        if trimmed.starts_with("win:") || trimmed.starts_with("focus:") {
            if let Ok(id) = trimmed[5..].trim().parse::<u32>() {
                return Some(HudActionResult::FocusWindow(id));
            }
        }

        // System shutdown / reboot / lock command shortcuts
        if trimmed == "lock" || trimmed == "reboot" || trimmed == "poweroff" || trimmed == "suspend" {
            return Some(HudActionResult::ExecuteSysCommand(String::from(trimmed)));
        }

        // Default: treat as app launch request
        Some(HudActionResult::LaunchApp(String::from(trimmed)))
    }

    fn simple_math_eval(&self, expr: &str) -> String {
        // Simple evaluator for integers with +, -, *, /
        let parts: Vec<&str> = expr.split_whitespace().collect();
        if parts.len() == 3 {
            if let (Ok(a), Ok(b)) = (parts[0].parse::<i64>(), parts[2].parse::<i64>()) {
                let res = match parts[1] {
                    "+" => Ok(a + b),
                    "-" => Ok(a - b),
                    "*" => Ok(a * b),
                    "/" => if b != 0 { Ok(a / b) } else { Err("Division by zero") },
                    _ => Err("Unsupported operator"),
                };
                return match res {
                    Ok(val) => String::from(alloc::format!("Result: {}", val)),
                    Err(err) => String::from(err),
                };
            }
        }
        String::from("Invalid Math Expression")
    }
}

/// Ranger / Dolphin inspired Spatial File Navigation Engine
#[derive(Debug, Clone)]
pub struct RangerDolphinSpatialFileNav {
    pub current_path: Vec<String>,
    pub miller_parent_contents: Vec<String>,
    pub miller_current_contents: Vec<String>,
    pub miller_preview_contents: Vec<String>,
    pub selected_entry_index: usize,
    pub bookmarks: Vec<(String, Vec<String>)>, // (Label, Path components)
}

impl Default for RangerDolphinSpatialFileNav {
    fn default() -> Self {
        Self::new()
    }
}

impl RangerDolphinSpatialFileNav {
    pub fn new() -> Self {
        let mut nav = Self {
            current_path: vec![String::from("root"), String::from("home"), String::from("user")],
            miller_parent_contents: vec![String::from("user"), String::from("shared"), String::from("guest")],
            miller_current_contents: vec![
                String::from("Documents"),
                String::from("Downloads"),
                String::from("Pictures"),
                String::from("Projects"),
            ],
            miller_preview_contents: vec![String::from("README.md"), String::from("Cargo.toml")],
            selected_entry_index: 0,
            bookmarks: Vec::new(),
        };
        nav.add_bookmark("Home", vec![String::from("root"), String::from("home"), String::from("user")]);
        nav.add_bookmark("System", vec![String::from("root"), String::from("sys")]);
        nav
    }

    pub fn add_bookmark(&mut self, label: &str, path: Vec<String>) {
        self.bookmarks.push((String::from(label), path));
    }

    pub fn get_breadcrumb_string(&self) -> String {
        let mut result = String::new();
        for comp in &self.current_path {
            result.push('/');
            result.push_str(comp);
        }
        result
    }

    pub fn handle_vim_key(&mut self, key: char) -> bool {
        match key {
            'k' | 'K' => {
                // Move selection up
                if self.selected_entry_index > 0 {
                    self.selected_entry_index -= 1;
                    return true;
                }
            }
            'j' | 'J' => {
                // Move selection down
                if self.selected_entry_index + 1 < self.miller_current_contents.len() {
                    self.selected_entry_index += 1;
                    return true;
                }
            }
            'h' | 'H' => {
                // Move to parent directory
                if self.current_path.len() > 1 {
                    self.current_path.pop();
                    self.selected_index_reset();
                    return true;
                }
            }
            'l' | 'L' => {
                // Enter selected directory
                if !self.miller_current_contents.is_empty() {
                    let entry = self.miller_current_contents[self.selected_entry_index].clone();
                    self.current_path.push(entry);
                    self.selected_index_reset();
                    return true;
                }
            }
            _ => {}
        }
        false
    }

    fn selected_index_reset(&mut self) {
        self.selected_entry_index = 0;
    }
}

/// Window item for i3 / Hyprland directional spatial navigation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowNode {
    pub id: u32,
    pub workspace_id: u32,
    pub title: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// i3 / Hyprland inspired Tiling Spatial Window Navigation Engine
#[derive(Debug, Clone, Default)]
pub struct TilingWindowManagerNav {
    pub windows: Vec<WindowNode>,
    pub focused_window_id: Option<u32>,
    pub active_workspace: u32,
}

impl TilingWindowManagerNav {
    pub fn new() -> Self {
        let mut nav = Self {
            windows: Vec::new(),
            focused_window_id: None,
            active_workspace: 1,
        };
        nav.populate_sample_windows();
        nav
    }

    fn populate_sample_windows(&mut self) {
        // Create left and right tile windows on Workspace 1
        self.windows.push(WindowNode {
            id: 101,
            workspace_id: 1,
            title: String::from("Sigma Terminal"),
            x: 0,
            y: 0,
            width: 960,
            height: 1080,
        });
        self.windows.push(WindowNode {
            id: 102,
            workspace_id: 1,
            title: String::from("Sigma Web Browser"),
            x: 960,
            y: 0,
            width: 960,
            height: 1080,
        });
        self.focused_window_id = Some(101);
    }

    pub fn navigate_direction(&mut self, dir: NavDirection) -> Option<u32> {
        let focused_id = self.focused_window_id?;
        let current_win = self.windows.iter().find(|w| w.id == focused_id && w.workspace_id == self.active_workspace)?.clone();

        let candidates: Vec<&WindowNode> = self
            .windows
            .iter()
            .filter(|w| w.id != current_win.id && w.workspace_id == self.active_workspace)
            .collect();

        let mut best_target: Option<&WindowNode> = None;
        let mut min_distance = i64::MAX;

        for cand in candidates {
            let is_valid = match dir {
                NavDirection::Left => cand.x < current_win.x,
                NavDirection::Right => cand.x > current_win.x,
                NavDirection::Up => cand.y < current_win.y,
                NavDirection::Down => cand.y > current_win.y,
                _ => false,
            };

            if is_valid {
                let dx = (cand.x - current_win.x) as i64;
                let dy = (cand.y - current_win.y) as i64;
                let dist = dx * dx + dy * dy;
                if dist < min_distance {
                    min_distance = dist;
                    best_target = Some(cand);
                }
            }
        }

        if let Some(target) = best_target {
            self.focused_window_id = Some(target.id);
            return Some(target.id);
        }

        None
    }

    pub fn switch_workspace(&mut self, workspace_id: u32) {
        self.active_workspace = workspace_id;
        self.focused_window_id = self
            .windows
            .iter()
            .find(|w| w.workspace_id == workspace_id)
            .map(|w| w.id);
    }
}

/// Control node for openSUSE YaST / FreeBSD bsdconfig navigation tree
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemControlNode {
    pub id: String,
    pub title: String,
    pub sub_nodes: Vec<SystemControlNode>,
}

/// openSUSE YaST / FreeBSD bsdconfig Control Tree Navigation Engine
#[derive(Debug, Clone, Default)]
pub struct YastBsdConfigControlTreeNav {
    pub root_nodes: Vec<SystemControlNode>,
    pub selected_path: Vec<usize>,
}

impl YastBsdConfigControlTreeNav {
    pub fn new() -> Self {
        let mut nav = Self {
            root_nodes: Vec::new(),
            selected_path: vec![0],
        };
        nav.build_system_control_tree();
        nav
    }

    fn build_system_control_tree(&mut self) {
        self.root_nodes.push(SystemControlNode {
            id: String::from("hardware"),
            title: String::from("Hardware & Drivers"),
            sub_nodes: vec![
                SystemControlNode { id: String::from("gpu"), title: String::from("Display & Graphics"), sub_nodes: Vec::new() },
                SystemControlNode { id: String::from("sound"), title: String::from("Audio & Sound Cards"), sub_nodes: Vec::new() },
                SystemControlNode { id: String::from("network"), title: String::from("Network Adapters"), sub_nodes: Vec::new() },
            ],
        });
        self.root_nodes.push(SystemControlNode {
            id: String::from("security"),
            title: String::from("Security & Users"),
            sub_nodes: vec![
                SystemControlNode { id: String::from("landlock"), title: String::from("Landlock Policies"), sub_nodes: Vec::new() },
                SystemControlNode { id: String::from("users"), title: String::from("User Accounts & Groups"), sub_nodes: Vec::new() },
            ],
        });
    }

    pub fn get_current_selected_title(&self) -> Option<&str> {
        if self.selected_path.is_empty() {
            return None;
        }

        let mut current_level = &self.root_nodes;
        let mut target_title = None;

        for &idx in &self.selected_path {
            if idx < current_level.len() {
                target_title = Some(current_level[idx].title.as_str());
                current_level = &current_level[idx].sub_nodes;
            } else {
                return None;
            }
        }

        target_title
    }

    pub fn navigate_tree(&mut self, dir: NavDirection) -> bool {
        if self.selected_path.is_empty() {
            return false;
        }

        match dir {
            NavDirection::Down => {
                let last_idx = self.selected_path.len() - 1;
                self.selected_path[last_idx] += 1;
                true
            }
            NavDirection::Up => {
                let last_idx = self.selected_path.len() - 1;
                if self.selected_path[last_idx] > 0 {
                    self.selected_path[last_idx] -= 1;
                    true
                } else {
                    false
                }
            }
            NavDirection::Child => {
                self.selected_path.push(0);
                true
            }
            NavDirection::Parent => {
                if self.selected_path.len() > 1 {
                    self.selected_path.pop();
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

/// Sovereign Master Universal Navigation Controller uniting all Linux & BSD distro navigation paradigms
#[derive(Debug, Clone, Default)]
pub struct SovereignUniversalNavigationEngine {
    pub pop_launcher: GnomePopLauncherNav,
    pub command_hud: KrunnerRofiCommandHud,
    pub spatial_file: RangerDolphinSpatialFileNav,
    pub tiling_windows: TilingWindowManagerNav,
    pub control_tree: YastBsdConfigControlTreeNav,
}

impl SovereignUniversalNavigationEngine {
    pub fn new() -> Self {
        Self {
            pop_launcher: GnomePopLauncherNav::new(),
            command_hud: KrunnerRofiCommandHud::new(),
            spatial_file: RangerDolphinSpatialFileNav::new(),
            tiling_windows: TilingWindowManagerNav::new(),
            control_tree: YastBsdConfigControlTreeNav::new(),
        }
    }

    pub fn evaluate_global_key_shortcut(&mut self, shortcut: &str) -> Option<String> {
        match shortcut {
            "Super" | "Alt+F1" => Some(String::from("Open POP Launcher")),
            "Alt+Space" | "Super+D" => Some(String::from("Open KRunner/Rofi HUD")),
            "Super+File" | "Super+E" => Some(String::from("Open Ranger Spatial File Manager")),
            "Super+Right" => {
                self.tiling_windows.navigate_direction(NavDirection::Right);
                Some(String::from("Focused Window Moved Right"))
            }
            "Super+Left" => {
                self.tiling_windows.navigate_direction(NavDirection::Left);
                Some(String::from("Focused Window Moved Left"))
            }
            "Super+Control" => Some(String::from("Open YaST/bsdconfig System Control Tree")),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pop_launcher_search() {
        let launcher = GnomePopLauncherNav::new();
        let results = launcher.search("term");
        assert!(!results.is_empty());
        assert_eq!(results[0].id, "sigma-term");
    }

    #[test]
    fn test_command_hud_math_eval() {
        let mut hud = KrunnerRofiCommandHud::new();
        let res = hud.evaluate_input("calc: 10 + 20");
        assert_eq!(res, Some(HudActionResult::MathCalculation(String::from("Result: 30"))));
    }

    #[test]
    fn test_ranger_spatial_navigation() {
        let mut ranger = RangerDolphinSpatialFileNav::new();
        assert_eq!(ranger.get_breadcrumb_string(), "/root/home/user");
        ranger.handle_vim_key('l');
        assert_eq!(ranger.get_breadcrumb_string(), "/root/home/user/Documents");
        ranger.handle_vim_key('h');
        assert_eq!(ranger.get_breadcrumb_string(), "/root/home/user");
    }

    #[test]
    fn test_tiling_spatial_navigation() {
        let mut tiling = TilingWindowManagerNav::new();
        assert_eq!(tiling.focused_window_id, Some(101));
        let next_id = tiling.navigate_direction(NavDirection::Right);
        assert_eq!(next_id, Some(102));
        assert_eq!(tiling.focused_window_id, Some(102));
    }

    #[test]
    fn test_yast_control_tree_navigation() {
        let mut tree = YastBsdConfigControlTreeNav::new();
        assert_eq!(tree.get_current_selected_title(), Some("Hardware & Drivers"));
        tree.navigate_tree(NavDirection::Child);
        assert_eq!(tree.get_current_selected_title(), Some("Display & Graphics"));
        tree.navigate_tree(NavDirection::Down);
        assert_eq!(tree.get_current_selected_title(), Some("Audio & Sound Cards"));
    }
}
