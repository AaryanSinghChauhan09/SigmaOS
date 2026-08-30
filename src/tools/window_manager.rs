// Window Manager (i3/sway/dwm/bspwm/OpenBSD cwm Inspiration)
// Tiling and dynamic window manager with workspaces, containers, scratchpads, gaps, and output rules

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

#[cfg(not(test))]
use crate::klib::HashMap;

/// Window geometry (x, y, width, height)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

/// Window representation
#[derive(Debug, Clone)]
pub struct Window {
    pub id: String,
    pub title: String,
    pub class: String,
    pub pid: u32,
    pub workspace: u32,
    pub floating: bool,
    pub fullscreen: bool,
    pub sticky: bool,
    pub urgent: bool,
    pub geometry: Rect,
    pub group_id: Option<String>,
}

impl Window {
    pub fn new(id: &str, title: &str, pid: u32) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            class: "generic".to_string(),
            pid,
            workspace: 1,
            floating: false,
            fullscreen: false,
            sticky: false,
            urgent: false,
            geometry: Rect::new(0, 0, 800, 600),
            group_id: None,
        }
    }

    pub fn with_class(mut self, class: &str) -> Self {
        self.class = class.to_string();
        self
    }

    pub fn set_workspace(&mut self, workspace: u32) {
        self.workspace = workspace;
    }

    pub fn set_floating(&mut self, floating: bool) {
        self.floating = floating;
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.fullscreen = fullscreen;
    }

    pub fn set_sticky(&mut self, sticky: bool) {
        self.sticky = sticky;
    }

    pub fn set_urgent(&mut self, urgent: bool) {
        self.urgent = urgent;
    }
}

/// Layout paradigms inspired by dwm, xmonad, bspwm, and i3/sway
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutType {
    SplitH,
    SplitV,
    Stacked,
    Tabbed,
    MasterStack,            // dwm / xmonad master & stack layout
    BinarySpacePartition,   // bspwm tree-based splitting
    Grid,                   // Equal tile grid
    Spiral,                 // Fibonacci spiral layout
}

/// Container tree node
#[derive(Debug, Clone)]
pub struct Container {
    pub id: String,
    pub layout: LayoutType,
    pub windows: Vec<String>,
    pub children: Vec<String>,
}

impl Container {
    pub fn new(id: &str, layout: LayoutType) -> Self {
        Self {
            id: id.to_string(),
            layout,
            windows: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn add_window(&mut self, window_id: &str) {
        self.windows.push(window_id.to_string());
    }

    pub fn add_child(&mut self, container_id: &str) {
        self.children.push(container_id.to_string());
    }
}

/// i3-gaps & bspwm style gaps and border configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WmGapsConfig {
    pub inner_gaps: u32,
    pub outer_gaps: u32,
    pub border_width: u32,
    pub active_border_color: u32,   // 0xRRGGBB
    pub inactive_border_color: u32, // 0xRRGGBB
}

impl Default for WmGapsConfig {
    fn default() -> Self {
        Self {
            inner_gaps: 8,
            outer_gaps: 12,
            border_width: 2,
            active_border_color: 0x0055D4,
            inactive_border_color: 0x444444,
        }
    }
}

/// Multi-monitor / Output display configuration (sway/spectrwm parity)
#[derive(Debug, Clone)]
pub struct MonitorOutputConfig {
    pub name: String,
    pub resolution: (u32, u32),
    pub scale_factor: f32,
    pub primary: bool,
    pub active_workspace: u32,
}

impl MonitorOutputConfig {
    pub fn new(name: &str, width: u32, height: u32) -> Self {
        Self {
            name: name.to_string(),
            resolution: (width, height),
            scale_factor: 1.0,
            primary: false,
            active_workspace: 1,
        }
    }
}

/// i3 / sway style Scratchpad manager for quick floating overlay tools
#[derive(Debug, Clone)]
pub struct ScratchpadManager {
    pub windows: Vec<String>,
    pub active_scratchpad: Option<String>,
}

impl ScratchpadManager {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            active_scratchpad: None,
        }
    }

    pub fn add_to_scratchpad(&mut self, window_id: &str) {
        if !self.windows.contains(&window_id.to_string()) {
            self.windows.push(window_id.to_string());
        }
    }

    pub fn toggle_scratchpad(&mut self) -> Option<String> {
        if self.windows.is_empty() {
            return None;
        }

        if let Some(active) = &self.active_scratchpad {
            let next_idx = self
                .windows
                .iter()
                .position(|w| w == active)
                .map(|i| (i + 1) % self.windows.len())
                .unwrap_or(0);
            self.active_scratchpad = Some(self.windows[next_idx].clone());
        } else {
            self.active_scratchpad = Some(self.windows[0].clone());
        }

        self.active_scratchpad.clone()
    }
}

impl Default for ScratchpadManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Workspace representation
#[derive(Debug, Clone)]
pub struct Workspace {
    pub number: u32,
    pub name: String,
    pub layout: LayoutType,
    pub windows: Vec<String>,
    pub output: String,
    pub master_count: u32,  // dwm master window count
    pub master_ratio: f32, // dwm master area width fraction (e.g. 0.55)
}

impl Workspace {
    pub fn new(number: u32, name: &str) -> Self {
        Self {
            number,
            name: name.to_string(),
            layout: LayoutType::MasterStack,
            windows: Vec::new(),
            output: "HDMI-1".to_string(),
            master_count: 1,
            master_ratio: 0.55,
        }
    }

    pub fn add_window(&mut self, window_id: &str) {
        if !self.windows.contains(&window_id.to_string()) {
            self.windows.push(window_id.to_string());
        }
    }

    pub fn remove_window(&mut self, window_id: &str) {
        self.windows.retain(|w| w != window_id);
    }
}

/// Key binding configuration
#[derive(Debug, Clone)]
pub struct KeyBinding {
    pub key: String,
    pub modifiers: Vec<String>,
    pub command: String,
}

impl KeyBinding {
    pub fn new(key: &str, command: &str) -> Self {
        Self {
            key: key.to_string(),
            modifiers: Vec::new(),
            command: command.to_string(),
        }
    }

    pub fn add_modifier(&mut self, modifier: &str) {
        self.modifiers.push(modifier.to_string());
    }
}

/// Tiling window manager with Linux & BSD multi-paradigm support
pub struct TilingWindowManager {
    pub workspaces: Vec<Workspace>,
    pub containers: Vec<Container>,
    pub windows: Vec<Window>,
    pub bindings: Vec<KeyBinding>,
    pub outputs: Vec<MonitorOutputConfig>,
    pub gaps: WmGapsConfig,
    pub scratchpad: ScratchpadManager,
    pub focus: Option<String>,
    pub active_workspace: u32,
}

impl TilingWindowManager {
    pub fn new() -> Self {
        let mut wm = Self {
            workspaces: Vec::new(),
            containers: Vec::new(),
            windows: Vec::new(),
            bindings: Vec::new(),
            outputs: vec![MonitorOutputConfig::new("HDMI-1", 1920, 1080)],
            gaps: WmGapsConfig::default(),
            scratchpad: ScratchpadManager::new(),
            focus: None,
            active_workspace: 1,
        };

        // Create default workspace
        wm.add_workspace(Workspace::new(1, "1: Main"));
        wm
    }

    pub fn add_workspace(&mut self, workspace: Workspace) {
        self.workspaces.push(workspace);
    }

    pub fn add_container(&mut self, container: Container) {
        self.containers.push(container);
    }

    pub fn add_window(&mut self, window: Window) {
        let wid = window.id.clone();
        let ws_num = window.workspace;
        self.windows.push(window);

        if let Some(ws) = self.workspaces.iter_mut().find(|w| w.number == ws_num) {
            ws.add_window(&wid);
        }

        self.focus_window(&wid);
    }

    pub fn add_binding(&mut self, binding: KeyBinding) {
        self.bindings.push(binding);
    }

    pub fn get_window(&mut self, id: &str) -> Option<&mut Window> {
        self.windows.iter_mut().find(|w| w.id == id)
    }

    pub fn focus_window(&mut self, id: &str) {
        self.focus = Some(id.to_string());
    }

    pub fn switch_workspace(&mut self, number: u32) {
        if self.workspaces.iter().any(|w| w.number == number) {
            self.active_workspace = number;
        }
    }

    pub fn move_window_to_workspace(
        &mut self,
        window_id: &str,
        target_ws: u32,
    ) -> Result<(), WMError> {
        let old_ws = if let Some(win) = self.windows.iter_mut().find(|w| w.id == window_id) {
            let prev_ws = win.workspace;
            win.set_workspace(target_ws);
            prev_ws
        } else {
            return Err(WMError::WindowNotFound);
        };

        if let Some(ws) = self.workspaces.iter_mut().find(|w| w.number == old_ws) {
            ws.remove_window(window_id);
        }

        if let Some(ws) = self.workspaces.iter_mut().find(|w| w.number == target_ws) {
            ws.add_window(window_id);
        }

        Ok(())
    }

    /// OpenBSD cwm-inspired window search and filter helper
    pub fn search_windows_by_query(&self, query: &str) -> Vec<&Window> {
        let q = query.to_lowercase();
        self.windows
            .iter()
            .filter(|w| w.title.to_lowercase().contains(&q) || w.class.to_lowercase().contains(&q))
            .collect()
    }

    /// Recalculate geometries for windows in active workspace using dwm Master-Stack layout algorithm
    pub fn calculate_layout_geometries(&mut self, screen_width: u32, screen_height: u32) {
        let ws_num = self.active_workspace;
        let ws = match self.workspaces.iter().find(|w| w.number == ws_num) {
            Some(w) => w,
            None => return,
        };

        let visible_window_ids: Vec<String> = ws.windows.clone();

        let window_ptrs: Vec<&mut Window> = self
            .windows
            .iter_mut()
            .filter(|w| visible_window_ids.contains(&w.id) && !w.floating && !w.fullscreen)
            .collect();

        let count = window_ptrs.len();
        if count == 0 {
            return;
        }

        let inner_gaps = self.gaps.inner_gaps as i32;
        let outer_gaps = self.gaps.outer_gaps as i32;

        let usable_width = (screen_width as i32) - (outer_gaps * 2);
        let usable_height = (screen_height as i32) - (outer_gaps * 2);

        if count == 1 {
            let win = window_ptrs.into_iter().next().unwrap();
            win.geometry = Rect::new(
                outer_gaps + inner_gaps,
                outer_gaps + inner_gaps,
                (usable_width - inner_gaps * 2) as u32,
                (usable_height - inner_gaps * 2) as u32,
            );
            return;
        }

        // Master-Stack split
        let master_w = ((usable_width as f32) * ws.master_ratio) as i32;
        let stack_w = usable_width - master_w;

        let master_count = ws.master_count as usize;
        let m_count = core::cmp::min(count, master_count);
        let s_count = count - m_count;

        let mut idx = 0;
        for win in window_ptrs {
            if idx < m_count {
                // Master region
                let row_h = usable_height / (m_count as i32);
                win.geometry = Rect::new(
                    outer_gaps + inner_gaps,
                    outer_gaps + (idx as i32 * row_h) + inner_gaps,
                    (master_w - inner_gaps * 2) as u32,
                    (row_h - inner_gaps * 2) as u32,
                );
            } else {
                // Stack region
                let s_idx = idx - m_count;
                let row_h = usable_height / (s_count as i32);
                win.geometry = Rect::new(
                    outer_gaps + master_w + inner_gaps,
                    outer_gaps + (s_idx as i32 * row_h) + inner_gaps,
                    (stack_w - inner_gaps * 2) as u32,
                    (row_h - inner_gaps * 2) as u32,
                );
            }
            idx += 1;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WMError {
    WindowNotFound,
    ContainerNotFound,
    WorkspaceNotFound,
}

impl Default for TilingWindowManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_and_workspace_creation() {
        let window = Window::new("w1", "Terminal", 1024).with_class("Alacritty");
        assert_eq!(window.title, "Terminal");
        assert_eq!(window.class, "Alacritty");

        let workspace = Workspace::new(1, "1: Work");
        assert_eq!(workspace.number, 1);
    }

    #[test]
    fn test_tiling_wm_operations_and_layout() {
        let mut wm = TilingWindowManager::new();
        let win1 = Window::new("w1", "Code", 100);
        let win2 = Window::new("w2", "Terminal", 101);

        wm.add_window(win1);
        wm.add_window(win2);

        assert_eq!(wm.windows.len(), 2);

        // Recalculate layout geometries for 1920x1080 screen
        wm.calculate_layout_geometries(1920, 1080);

        let w1_geom = wm.get_window("w1").unwrap().geometry;
        let w2_geom = wm.get_window("w2").unwrap().geometry;

        assert!(w1_geom.width > 0);
        assert!(w2_geom.width > 0);
        assert_ne!(w1_geom.x, w2_geom.x); // Master and stack should have different X offsets
    }

    #[test]
    fn test_scratchpad_and_cwm_search() {
        let mut wm = TilingWindowManager::new();
        wm.scratchpad.add_to_scratchpad("term_scratch");

        let active = wm.scratchpad.toggle_scratchpad().unwrap();
        assert_eq!(active, "term_scratch");

        let win = Window::new("w1", "Firefox Browser", 200).with_class("firefox");
        wm.add_window(win);

        let matches = wm.search_windows_by_query("fire");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].id, "w1");
    }
}
