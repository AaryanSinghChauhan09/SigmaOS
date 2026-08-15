//! Desktop Compositor Framework (GNOME + KDE + macOS Inspiration)
//! Implements modern desktop environment with compositor, window management, and accessibility

#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;
use alloc::string::ToString;

/// Window management modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowMode {
    /// Tiling (i3/bspwm inspiration)
    Tiling,
    /// Binary Space Partitioning Tiling (bspwm inspiration)
    BspTiling,
    /// Master-Stack Tiling (dwwm/xmonad inspiration)
    MasterStack,
    /// Stacking (Openbox inspiration)
    Stacking,
    /// Floating (macOS inspiration)
    Floating,
    /// Dynamic (GNOME Shell inspiration)
    Dynamic,
}

/// Window decorations
#[derive(Debug, Clone)]
pub struct WindowDecoration {
    pub title_bar: bool,
    pub borders: bool,
    pub shadows: bool,
    pub blur: bool,
    pub theme: String,
}

impl WindowDecoration {
    pub fn new() -> Self {
        Self {
            title_bar: true,
            borders: true,
            shadows: true,
            blur: false,
            theme: "default".to_string(),
        }
    }

    pub fn set_theme(&mut self, theme: &str) {
        self.theme = theme.to_string();
    }

    pub fn enable_blur(&mut self) {
        self.blur = true;
    }
}

/// Window state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
    Hidden,
}

/// Window
pub struct Window {
    pub id: u32,
    pub title: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub state: WindowState,
    pub decoration: WindowDecoration,
    pub workspace: u32,
}

impl Window {
    pub fn new(id: u32, title: &str) -> Self {
        Self {
            id,
            title: title.to_string(),
            x: 0,
            y: 0,
            width: 800,
            height: 600,
            state: WindowState::Normal,
            decoration: WindowDecoration::new(),
            workspace: 0,
        }
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn maximize(&mut self) {
        self.state = WindowState::Maximized;
    }

    pub fn minimize(&mut self) {
        self.state = WindowState::Minimized;
    }

    pub fn fullscreen(&mut self) {
        self.state = WindowState::Fullscreen;
    }

    pub fn restore(&mut self) {
        self.state = WindowState::Normal;
    }
}

/// Workspace
pub struct Workspace {
    pub id: u32,
    pub name: String,
    pub windows: Vec<u32>,
    pub active: bool,
}

impl Workspace {
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            windows: Vec::new(),
            active: false,
        }
    }

    pub fn add_window(&mut self, window_id: u32) {
        self.windows.push(window_id);
    }

    pub fn remove_window(&mut self, window_id: u32) {
        self.windows.retain(|&id| id != window_id);
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

/// Compositor (Wayland inspiration)
pub struct Compositor {
    pub windows: Vec<Window>,
    pub workspaces: Vec<Workspace>,
    pub active_workspace: u32,
    pub window_mode: WindowMode,
    pub next_window_id: u32,
}

impl Compositor {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            workspaces: Vec::new(),
            active_workspace: 0,
            window_mode: WindowMode::Dynamic,
            next_window_id: 1,
        }
    }

    pub fn create_window(&mut self, title: &str) -> u32 {
        let id = self.next_window_id;
        let window = Window::new(id, title);
        self.windows.push(window);
        self.next_window_id += 1;
        id
    }

    pub fn destroy_window(&mut self, id: u32) {
        self.windows.retain(|w| w.id != id);
        for workspace in &mut self.workspaces {
            workspace.remove_window(id);
        }
    }

    pub fn get_window(&mut self, id: u32) -> Option<&mut Window> {
        self.windows.iter_mut().find(|w| w.id == id)
    }

    pub fn create_workspace(&mut self, name: &str) -> u32 {
        let id = self.workspaces.len() as u32;
        let workspace = Workspace::new(id, name);
        self.workspaces.push(workspace);
        id
    }

    pub fn switch_workspace(&mut self, id: u32) {
        if let Some(idx) = self.workspaces.iter().position(|w| w.id == id) {
            self.workspaces[self.active_workspace as usize].set_active(false);
            self.active_workspace = id;
            self.workspaces[idx].set_active(true);
        }
    }

    pub fn set_window_mode(&mut self, mode: WindowMode) {
        self.window_mode = mode;
    }

    pub fn layout_windows(&mut self) {
        match self.window_mode {
            WindowMode::Tiling => self.layout_tiling(),
            WindowMode::BspTiling => self.layout_bsp_tiling(),
            WindowMode::MasterStack => self.layout_master_stack(),
            WindowMode::Stacking => self.layout_stacking(),
            WindowMode::Floating => self.layout_floating(),
            WindowMode::Dynamic => self.layout_dynamic(),
        }
    }

    fn layout_master_stack(&mut self) {
        let active_ws = self.active_workspace;
        let mut workspace_windows: Vec<&mut Window> = self.windows
            .iter_mut()
            .filter(|w| w.workspace == active_ws && w.state == WindowState::Normal)
            .collect();

        let count = workspace_windows.len();
        if count == 0 {
            return;
        }

        let screen_width = 1920u32;
        let screen_height = 1080u32;

        if count == 1 {
            workspace_windows[0].set_position(0, 0);
            workspace_windows[0].set_size(screen_width, screen_height);
        } else {
            let master_width = screen_width / 2;
            let stack_width = screen_width - master_width;
            let stack_height = screen_height / (count as u32 - 1);

            workspace_windows[0].set_position(0, 0);
            workspace_windows[0].set_size(master_width, screen_height);

            for (i, window) in workspace_windows.iter_mut().skip(1).enumerate() {
                let y = (i as u32 * stack_height) as i32;
                window.set_position(master_width as i32, y);
                window.set_size(stack_width, stack_height);
            }
        }
    }

    fn layout_bsp_tiling(&mut self) {
        let active_ws = self.active_workspace;
        let mut workspace_windows: Vec<&mut Window> = self.windows
            .iter_mut()
            .filter(|w| w.workspace == active_ws && w.state == WindowState::Normal)
            .collect();

        let count = workspace_windows.len();
        if count == 0 {
            return;
        }

        let mut curr_x = 0i32;
        let mut curr_y = 0i32;
        let mut curr_w = 1920u32;
        let mut curr_h = 1080u32;

        for (i, window) in workspace_windows.iter_mut().enumerate() {
            if i == count - 1 {
                window.set_position(curr_x, curr_y);
                window.set_size(curr_w, curr_h);
            } else if i % 2 == 0 {
                // Split vertically
                let half_w = curr_w / 2;
                window.set_position(curr_x, curr_y);
                window.set_size(half_w, curr_h);
                curr_x += half_w as i32;
                curr_w -= half_w;
            } else {
                // Split horizontally
                let half_h = curr_h / 2;
                window.set_position(curr_x, curr_y);
                window.set_size(curr_w, half_h);
                curr_y += half_h as i32;
                curr_h -= half_h;
            }
        }
    }

    fn layout_tiling(&mut self) {
        let active_ws = self.active_workspace;
        // Tiling layout (i3 inspiration)
        let mut workspace_windows: Vec<&mut Window> = self.windows
            .iter_mut()
            .filter(|w| w.workspace == active_ws && w.state == WindowState::Normal)
            .collect();
        
        let count = workspace_windows.len();
        if count == 0 {
            return;
        }

        let screen_width = 1920;
        let screen_height = 1080;
        
        for (i, window) in workspace_windows.iter_mut().enumerate() {
            if count == 1 {
                window.set_position(0, 0);
                window.set_size(screen_width, screen_height);
            } else if count == 2 {
                if i == 0 {
                    window.set_position(0, 0);
                    window.set_size(screen_width / 2, screen_height);
                } else {
                    window.set_position((screen_width / 2) as i32, 0);
                    window.set_size(screen_width / 2, screen_height);
                }
            } else {
                // Grid layout for more windows
                let cols = (count as f32).sqrt().ceil() as u32;
                let rows = (count as f32 / cols as f32).ceil() as u32;
                let window_width = screen_width / cols;
                let window_height = screen_height / rows;
                
                let col = i as u32 % cols;
                let row = i as u32 / cols;
                
                window.set_position((col * window_width) as i32, (row * window_height) as i32);
                window.set_size(window_width, window_height);
            }
        }
    }

    fn layout_stacking(&mut self) {
        let active_ws = self.active_workspace;
        // Stacking layout (Openbox inspiration)
        let mut workspace_windows: Vec<&mut Window> = self.windows
            .iter_mut()
            .filter(|w| w.workspace == active_ws && w.state == WindowState::Normal)
            .collect();
        
        let screen_width = 1920;
        let screen_height = 1080;
        let margin = 50;
        
        for (i, window) in workspace_windows.iter_mut().enumerate() {
            let offset = (i * 20) as i32;
            window.set_position(margin + offset, margin + offset);
            window.set_size(
                screen_width - (2 * margin) as u32,
                screen_height - (2 * margin) as u32,
            );
        }
    }

    fn layout_floating(&mut self) {
        // Floating layout (macOS inspiration)
        // Windows maintain their manual positions
    }

    fn layout_dynamic(&mut self) {
        let active_ws = self.active_workspace;
        let count = self.windows
            .iter()
            .filter(|w| w.workspace == active_ws && w.state == WindowState::Normal)
            .count();

        if count <= 1 {
            self.layout_tiling();
        } else {
            self.layout_stacking();
        }
    }
}

impl Default for Compositor {
    fn default() -> Self {
        Self::new()
    }
}

/// Accessibility features (GNOME/KDE inspiration)
pub struct Accessibility {
    pub screen_reader_enabled: bool,
    pub magnifier_enabled: bool,
    pub high_contrast_enabled: bool,
    pub screen_keyboard_enabled: bool,
}

impl Accessibility {
    pub fn new() -> Self {
        Self {
            screen_reader_enabled: false,
            magnifier_enabled: false,
            high_contrast_enabled: false,
            screen_keyboard_enabled: false,
        }
    }

    pub fn enable_screen_reader(&mut self) {
        self.screen_reader_enabled = true;
    }

    pub fn enable_magnifier(&mut self) {
        self.magnifier_enabled = true;
    }

    pub fn enable_high_contrast(&mut self) {
        self.high_contrast_enabled = true;
    }

    pub fn enable_screen_keyboard(&mut self) {
        self.screen_keyboard_enabled = true;
    }

    pub fn get_status(&self) -> AccessibilityStatus {
        AccessibilityStatus {
            screen_reader_enabled: self.screen_reader_enabled,
            magnifier_enabled: self.magnifier_enabled,
            high_contrast_enabled: self.high_contrast_enabled,
            screen_keyboard_enabled: self.screen_keyboard_enabled,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AccessibilityStatus {
    pub screen_reader_enabled: bool,
    pub magnifier_enabled: bool,
    pub high_contrast_enabled: bool,
    pub screen_keyboard_enabled: bool,
}

impl Default for Accessibility {
    fn default() -> Self {
        Self::new()
    }
}

/// Desktop environment
pub struct DesktopEnvironment {
    pub compositor: Compositor,
    pub accessibility: Accessibility,
    pub theme: String,
    pub font_size: u32,
}

impl DesktopEnvironment {
    pub fn new() -> Self {
        Self {
            compositor: Compositor::new(),
            accessibility: Accessibility::new(),
            theme: "default".to_string(),
            font_size: 12,
        }
    }

    pub fn initialize(&mut self) {
        // Create default workspace
        self.compositor.create_workspace("Main");
        self.compositor.switch_workspace(0);
    }

    pub fn set_theme(&mut self, theme: &str) {
        self.theme = theme.to_string();
    }

    pub fn set_font_size(&mut self, size: u32) {
        self.font_size = size;
    }

    pub fn create_application_window(&mut self, app_name: &str) -> u32 {
        let active_ws = self.compositor.active_workspace;
        let window_id = self.compositor.create_window(app_name);
        if let Some(window) = self.compositor.get_window(window_id) {
            window.workspace = active_ws;
        }
        window_id
    }

    pub fn close_window(&mut self, window_id: u32) {
        self.compositor.destroy_window(window_id);
    }

    pub fn layout(&mut self) {
        self.compositor.layout_windows();
    }
}

impl Default for DesktopEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_creation() {
        let mut compositor = Compositor::new();
        let window_id = compositor.create_window("Test Window");
        assert_eq!(window_id, 1);
        assert_eq!(compositor.windows.len(), 1);
    }

    #[test]
    fn test_workspace_management() {
        let mut compositor = Compositor::new();
        let workspace_id = compositor.create_workspace("Main");
        assert_eq!(workspace_id, 0);
        compositor.switch_workspace(0);
        assert_eq!(compositor.active_workspace, 0);
    }

    #[test]
    fn test_window_modes() {
        let mut compositor = Compositor::new();
        compositor.set_window_mode(WindowMode::Tiling);
        assert_eq!(compositor.window_mode, WindowMode::Tiling);
    }

    #[test]
    fn test_accessibility() {
        let mut accessibility = Accessibility::new();
        accessibility.enable_screen_reader();
        assert!(accessibility.screen_reader_enabled);
    }

    #[test]
    fn test_desktop_environment() {
        let mut desktop = DesktopEnvironment::new();
        desktop.initialize();
        let window_id = desktop.create_application_window("Test App");
        assert!(window_id > 0);
    }
}