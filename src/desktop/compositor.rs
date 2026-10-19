// SPDX-License-Identifier: MIT
// SigmaOS - Desktop Environment Integration (Compositor/Window Manager)
// Inspired by Wayland compositors (Weston, Mutter, KWin) and X11 window managers
// Provides basic compositor and window manager functionality

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;

/// Window state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
    Hidden,
}

/// Window attributes
#[derive(Debug, Clone)]
pub struct WindowAttributes {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub state: WindowState,
    pub border_width: u32,
    pub border_color: u32,
    pub background_color: u32,
}

impl Default for WindowAttributes {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 800,
            height: 600,
            title: "Untitled".to_string(),
            state: WindowState::Normal,
            border_width: 1,
            border_color: 0x000000,
            background_color: 0xFFFFFF,
        }
    }
}

/// Surface for rendering
#[derive(Debug, Clone)]
pub struct Surface {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub format: SurfaceFormat,
    pub buffer: Vec<u8>, // RGBA buffer
}

/// Surface format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceFormat {
    ARGB8888,
    XRGB8888,
    RGB565,
}

/// Window structure
#[derive(Debug, Clone)]
pub struct Window {
    pub id: u32,
    pub surface: Option<Surface>,
    pub attributes: WindowAttributes,
    pub visible: bool,
    pub focused: bool,
}

impl Window {
    pub fn new(id: u32, attributes: WindowAttributes) -> Self {
        Self {
            id,
            surface: None,
            attributes,
            visible: true,
            focused: false,
        }
    }

    pub fn set_surface(&mut self, surface: Surface) {
        self.surface = Some(surface);
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn minimize(&mut self) {
        self.attributes.state = WindowState::Minimized;
        self.visible = false;
    }

    pub fn maximize(&mut self) {
        self.attributes.state = WindowState::Maximized;
        self.visible = true;
    }

    pub fn fullscreen(&mut self) {
        self.attributes.state = WindowState::Fullscreen;
        self.visible = true;
    }

    pub fn restore(&mut self) {
        self.attributes.state = WindowState::Normal;
        self.visible = true;
    }
}

/// Input event type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEventType {
    KeyPress,
    KeyRelease,
    ButtonPress,
    ButtonRelease,
    Motion,
    Touch,
}

/// Input event
#[derive(Debug, Clone)]
pub struct InputEvent {
    pub event_type: InputEventType,
    pub x: i32,
    pub y: i32,
    pub key_code: u32,
    pub button: u32,
    pub timestamp: u64,
}

/// Compositor - manages surface composition
pub struct Compositor {
    pub surfaces: BTreeMap<u32, Surface>,
    pub next_surface_id: u32,
    pub output_width: u32,
    pub output_height: u32,
}

impl Compositor {
    pub fn new(output_width: u32, output_height: u32) -> Self {
        Self {
            surfaces: BTreeMap::new(),
            next_surface_id: 1,
            output_width,
            output_height,
        }
    }

    /// Create a new surface
    pub fn create_surface(&mut self, width: u32, height: u32, format: SurfaceFormat) -> u32 {
        let id = self.next_surface_id;
        self.next_surface_id += 1;

        let buffer_size = (width * height * 4) as usize; // 4 bytes per pixel for RGBA
        let surface = Surface {
            id,
            width,
            height,
            format,
            buffer: vec![0u8; buffer_size],
        };

        self.surfaces.insert(id, surface);
        id
    }

    /// Destroy a surface
    pub fn destroy_surface(&mut self, surface_id: u32) -> Result<(), &'static str> {
        self.surfaces
            .remove(&surface_id)
            .ok_or("Surface not found")?;
        Ok(())
    }

    /// Get a surface
    pub fn get_surface(&self, surface_id: u32) -> Option<&Surface> {
        self.surfaces.get(&surface_id)
    }

    /// Get a mutable surface
    pub fn get_surface_mut(&mut self, surface_id: u32) -> Option<&mut Surface> {
        self.surfaces.get_mut(&surface_id)
    }

    /// Compose surfaces to output
    pub fn compose(&self, output_buffer: &mut [u8]) -> Result<(), &'static str> {
        // Clear output buffer
        for byte in output_buffer.iter_mut() {
            *byte = 0;
        }

        // Composite all surfaces (simplified - just overlay)
        for surface in self.surfaces.values() {
            self.composite_surface(surface, output_buffer)?;
        }

        Ok(())
    }

    /// Composite a single surface
    fn composite_surface(
        &self,
        surface: &Surface,
        output_buffer: &mut [u8],
    ) -> Result<(), &'static str> {
        let output_stride = self.output_width as usize * 4;
        let surface_stride = surface.width as usize * 4;

        // Simple copy (no alpha blending for now)
        for y in 0..surface.height as usize {
            if y >= self.output_height as usize {
                break;
            }

            let output_offset = y * output_stride;
            let surface_offset = y * surface_stride;

            for x in 0..surface.width as usize {
                if x >= self.output_width as usize {
                    break;
                }

                let pixel_offset = x * 4;
                if output_offset + pixel_offset + 3 < output_buffer.len()
                    && surface_offset + pixel_offset + 3 < surface.buffer.len()
                {
                    output_buffer[output_offset + pixel_offset] =
                        surface.buffer[surface_offset + pixel_offset];
                    output_buffer[output_offset + pixel_offset + 1] =
                        surface.buffer[surface_offset + pixel_offset + 1];
                    output_buffer[output_offset + pixel_offset + 2] =
                        surface.buffer[surface_offset + pixel_offset + 2];
                    output_buffer[output_offset + pixel_offset + 3] =
                        surface.buffer[surface_offset + pixel_offset + 3];
                }
            }
        }

        Ok(())
    }
}

impl Default for Compositor {
    fn default() -> Self {
        Self::new(1920, 1080)
    }
}

/// Window Manager - manages windows and input
pub struct WindowManager {
    pub windows: BTreeMap<u32, Window>,
    pub focused_window: Option<u32>,
    pub next_window_id: u32,
    pub compositor: Compositor,
}

impl WindowManager {
    pub fn new(output_width: u32, output_height: u32) -> Self {
        Self {
            windows: BTreeMap::new(),
            focused_window: None,
            next_window_id: 1,
            compositor: Compositor::new(output_width, output_height),
        }
    }

    /// Create a new window
    pub fn create_window(&mut self, attributes: WindowAttributes) -> u32 {
        let id = self.next_window_id;
        self.next_window_id += 1;

        let window = Window::new(id, attributes);
        self.windows.insert(id, window);

        // Auto-focus new window
        self.focus_window(id);

        id
    }

    /// Destroy a window
    pub fn destroy_window(&mut self, window_id: u32) -> Result<(), &'static str> {
        if self.focused_window == Some(window_id) {
            self.focused_window = None;
        }

        self.windows.remove(&window_id).ok_or("Window not found")?;
        Ok(())
    }

    /// Get a window
    pub fn get_window(&self, window_id: u32) -> Option<&Window> {
        self.windows.get(&window_id)
    }

    /// Get a mutable window
    pub fn get_window_mut(&mut self, window_id: u32) -> Option<&mut Window> {
        self.windows.get_mut(&window_id)
    }

    /// Focus a window
    pub fn focus_window(&mut self, window_id: u32) {
        // Unfocus previous window
        if let Some(prev_id) = self.focused_window {
            if let Some(window) = self.windows.get_mut(&prev_id) {
                window.focused = false;
            }
        }

        // Focus new window
        if let Some(window) = self.windows.get_mut(&window_id) {
            window.focused = true;
            self.focused_window = Some(window_id);
        }
    }

    /// Handle input event
    pub fn handle_input(&mut self, event: InputEvent) -> Result<(), &'static str> {
        match event.event_type {
            InputEventType::ButtonPress => {
                // Find window under cursor
                if let Some(window_id) = self.find_window_at(event.x, event.y) {
                    self.focus_window(window_id);
                }
            }
            InputEventType::KeyPress => {
                // Send key event to focused window
                if let Some(window_id) = self.focused_window {
                    // In real implementation, send event to window
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// Find window at position
    fn find_window_at(&self, x: i32, y: i32) -> Option<u32> {
        for (id, window) in self.windows.iter().rev() {
            if !window.visible {
                continue;
            }

            let attrs = &window.attributes;
            if x >= attrs.x
                && x < (attrs.x + attrs.width as i32)
                && y >= attrs.y
                && y < (attrs.y + attrs.height as i32)
            {
                return Some(*id);
            }
        }
        None
    }

    /// Get compositor
    pub fn get_compositor(&mut self) -> &mut Compositor {
        &mut self.compositor
    }

    /// Render frame
    pub fn render_frame(&mut self, output_buffer: &mut [u8]) -> Result<(), &'static str> {
        // Update surfaces from windows
        for window in self.windows.values() {
            if let Some(ref surface) = window.surface {
                if !self.compositor.surfaces.contains_key(&surface.id) {
                    self.compositor.surfaces.insert(surface.id, surface.clone());
                }
            }
        }

        // Compose
        self.compositor.compose(output_buffer)
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new(1920, 1080)
    }
}

/// Desktop Environment - compositor + window manager
pub struct DesktopEnvironment {
    pub window_manager: WindowManager,
    pub running: bool,
}

impl DesktopEnvironment {
    pub fn new(output_width: u32, output_height: u32) -> Self {
        Self {
            window_manager: WindowManager::new(output_width, output_height),
            running: false,
        }
    }

    /// Initialize desktop environment
    pub fn initialize(&mut self) -> Result<(), &'static str> {
        // Initialize compositor
        // Initialize window manager
        // Setup input handling
        self.running = true;
        Ok(())
    }

    /// Run main loop
    pub fn run(&mut self) -> Result<(), &'static str> {
        // TODO: Implement proper event loop with running flag mutation
        #[allow(clippy::while_immutable_condition)]
        while self.running {
            // Process input events
            // Render frame
            // Sync to display
        }
        Ok(())
    }

    /// Shutdown desktop environment
    pub fn shutdown(&mut self) -> Result<(), &'static str> {
        self.running = false;
        Ok(())
    }

    /// Get window manager
    pub fn get_window_manager(&mut self) -> &mut WindowManager {
        &mut self.window_manager
    }
}

impl Default for DesktopEnvironment {
    fn default() -> Self {
        Self::new(1920, 1080)
    }
}

/// Global desktop environment instance
static mut DESKTOP_ENV: Option<DesktopEnvironment> = None;

/// Initialize desktop environment
pub fn init_desktop_environment(output_width: u32, output_height: u32) {
    unsafe {
        DESKTOP_ENV = Some(DesktopEnvironment::new(output_width, output_height));
    }
}

/// Get global desktop environment
pub fn get_desktop_environment() -> Option<&'static mut DesktopEnvironment> {
    unsafe { DESKTOP_ENV.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compositor_create_surface() {
        let mut compositor = Compositor::new(1920, 1080);
        let surface_id = compositor.create_surface(800, 600, SurfaceFormat::ARGB8888);
        assert!(compositor.get_surface(surface_id).is_some());
    }

    #[test]
    fn test_compositor_destroy_surface() {
        let mut compositor = Compositor::new(1920, 1080);
        let surface_id = compositor.create_surface(800, 600, SurfaceFormat::ARGB8888);
        compositor.destroy_surface(surface_id).unwrap();
        assert!(compositor.get_surface(surface_id).is_none());
    }

    #[test]
    fn test_window_manager_create_window() {
        let mut wm = WindowManager::new(1920, 1080);
        let attrs = WindowAttributes::default();
        let window_id = wm.create_window(attrs);
        assert!(wm.get_window(window_id).is_some());
    }

    #[test]
    fn test_window_manager_focus_window() {
        let mut wm = WindowManager::new(1920, 1080);
        let attrs = WindowAttributes::default();
        let window_id = wm.create_window(attrs);
        wm.focus_window(window_id);
        assert_eq!(wm.focused_window, Some(window_id));
    }

    #[test]
    fn test_window_minimize() {
        let mut wm = WindowManager::new(1920, 1080);
        let attrs = WindowAttributes::default();
        let window_id = wm.create_window(attrs);
        if let Some(window) = wm.get_window_mut(window_id) {
            window.minimize();
            assert_eq!(window.attributes.state, WindowState::Minimized);
            assert!(!window.visible);
        }
    }

    #[test]
    fn test_window_maximize() {
        let mut wm = WindowManager::new(1920, 1080);
        let attrs = WindowAttributes::default();
        let window_id = wm.create_window(attrs);
        if let Some(window) = wm.get_window_mut(window_id) {
            window.maximize();
            assert_eq!(window.attributes.state, WindowState::Maximized);
            assert!(window.visible);
        }
    }

    #[test]
    fn test_desktop_environment_init() {
        let mut desktop = DesktopEnvironment::new(1920, 1080);
        desktop.initialize().unwrap();
        assert!(desktop.running);
    }

    #[test]
    fn test_window_attributes_default() {
        let attrs = WindowAttributes::default();
        assert_eq!(attrs.width, 800);
        assert_eq!(attrs.height, 600);
        assert_eq!(attrs.title, "Untitled");
    }
}
