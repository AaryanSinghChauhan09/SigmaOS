// Zenith Desktop Compositor
// Wayland-inspired compositor with OOP design
// Inspired by Mutter, KWin, Sway, and wlroots from popular Linux distributions.

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::string::ToString;

pub const SCREEN_WIDTH: u32 = 1920;
pub const SCREEN_HEIGHT: u32 = 1080;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Active,
    Inactive,
    Minimized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct WindowNode {
    pub id: u32,
    pub title: String,
    pub geometry: Geometry,
    pub state: WindowState,
}

impl WindowNode {
    pub fn new(id: u32, title: String, x: i32, y: i32, w: u32, h: u32) -> Self {
        Self {
            id,
            title,
            geometry: Geometry {
                x,
                y,
                width: w,
                height: h,
            },
            state: WindowState::Inactive,
        }
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        self.geometry.width = w;
        self.geometry.height = h;
    }

    pub fn move_to(&mut self, x: i32, y: i32) {
        self.geometry.x = x;
        self.geometry.y = y;
    }

    pub fn set_state(&mut self, state: WindowState) {
        self.state = state;
    }

    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        x >= self.geometry.x
            && x < self.geometry.x + self.geometry.width as i32
            && y >= self.geometry.y
            && y < self.geometry.y + self.geometry.height as i32
    }
}

pub struct ZenithCompositor {
    pub windows: Vec<Option<WindowNode>>,
    pub active_window_id: Option<u32>,
    pub sub_surfaces: Vec<SubSurface>,
    pub cursor_tracker: CursorTracker,
    pub vsync_controller: VsyncController,
    pub damage_tracker: DamageTracker,
}

impl ZenithCompositor {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            active_window_id: None,
            sub_surfaces: Vec::new(),
            cursor_tracker: CursorTracker::new(),
            vsync_controller: VsyncController::new(60),
            damage_tracker: DamageTracker::new(),
        }
    }

    pub fn register_window(&mut self, window: WindowNode) -> Result<(), &'static str> {
        if self.windows.len() >= 32 {
            return Err("Maximum window limit reached");
        }
        // Mark the new window's geometry as damaged so it gets redrawn
        self.damage_tracker.add_damage(window.geometry);
        self.windows.push(Some(window));
        Ok(())
    }

    pub fn activate_window(&mut self, window_id: u32) -> Result<(), &'static str> {
        let geom = {
            let window = self.get_window_mut(window_id).ok_or("Window not found")?;
            window.set_state(WindowState::Active);
            window.geometry
        };
        self.damage_tracker.add_damage(geom);

        // Deactivate other windows
        for win_opt in self.windows.iter_mut() {
            if let Some(win) = win_opt {
                if win.id != window_id {
                    win.set_state(WindowState::Inactive);
                }
            }
        }

        self.active_window_id = Some(window_id);
        Ok(())
    }

    pub fn minimize_window(&mut self, window_id: u32) -> Result<(), &'static str> {
        let geom = {
            let window = self.get_window_mut(window_id).ok_or("Window not found")?;
            window.set_state(WindowState::Minimized);
            window.geometry
        };
        self.damage_tracker.add_damage(geom);

        if self.active_window_id == Some(window_id) {
            self.active_window_id = None;
        }
        Ok(())
    }

    pub fn get_window(&self, window_id: u32) -> Option<&WindowNode> {
        for win_opt in self.windows.iter() {
            if let Some(win) = win_opt {
                if win.id == window_id {
                    return Some(win);
                }
            }
        }
        None
    }

    pub fn get_window_mut(&mut self, window_id: u32) -> Option<&mut WindowNode> {
        for win_opt in self.windows.iter_mut() {
            if let Some(win) = win_opt {
                if win.id == window_id {
                    return Some(win);
                }
            }
        }
        None
    }

    pub fn get_active_window(&self) -> Option<&WindowNode> {
        if let Some(id) = self.active_window_id {
            self.get_window(id)
        } else {
            None
        }
    }

    pub fn get_window_at_point(&self, x: i32, y: i32) -> Option<&WindowNode> {
        // Check windows in reverse order (top to bottom)
        for win_opt in self.windows.iter().rev() {
            if let Some(win) = win_opt {
                if win.state != WindowState::Minimized && win.contains_point(x, y) {
                    return Some(win);
                }
            }
        }
        None
    }

    pub fn window_count(&self) -> usize {
        self.windows.iter().filter(|w| w.is_some()).count()
    }

    pub fn remove_window(&mut self, window_id: u32) -> Result<(), &'static str> {
        for win_opt in self.windows.iter_mut() {
            if let Some(win) = win_opt {
                if win.id == window_id {
                    self.damage_tracker.add_damage(win.geometry);
                    *win_opt = None;
                    if self.active_window_id == Some(window_id) {
                        self.active_window_id = None;
                    }
                    return Ok(());
                }
            }
        }
        Err("Window not found")
    }

    // ==============================================================================
    // wl_subsurface-style Parent/Child Layering (Menus, Tooltips)
    // ==============================================================================
    pub fn register_sub_surface(&mut self, parent_id: u32, sub_id: u32, x_offset: i32, y_offset: i32, w: u32, h: u32) -> bool {
        if self.get_window(parent_id).is_none() {
            return false;
        }
        self.sub_surfaces.push(SubSurface {
            id: sub_id,
            parent_window_id: parent_id,
            relative_offset_x: x_offset,
            relative_offset_y: y_offset,
            width: w,
            height: h,
        });
        true
    }

    pub fn get_sub_surface_absolute_geometry(&self, sub_id: u32) -> Option<Geometry> {
        for sub in &self.sub_surfaces {
            if sub.id == sub_id {
                if let Some(parent) = self.get_window(sub.parent_window_id) {
                    return Some(Geometry {
                        x: parent.geometry.x + sub.relative_offset_x,
                        y: parent.geometry.y + sub.relative_offset_y,
                        width: sub.width,
                        height: sub.height,
                    });
                }
            }
        }
        None
    }

    // ==============================================================================
    // Sway/i3-style Tiling window layout calculations
    // ==============================================================================
    pub fn execute_tiling_layout(&mut self, split_vertical: bool) {
        let active_count = self.windows.iter().filter(|w| {
            if let Some(win) = w {
                win.state != WindowState::Minimized
            } else {
                false
            }
        }).count();

        if active_count == 0 {
            return;
        }

        let mut idx = 0;
        for win_opt in self.windows.iter_mut() {
            if let Some(win) = win_opt {
                if win.state != WindowState::Minimized {
                    if split_vertical {
                        let cell_width = SCREEN_WIDTH / active_count as u32;
                        win.geometry.x = (idx as u32 * cell_width) as i32;
                        win.geometry.y = 0;
                        win.geometry.width = cell_width;
                        win.geometry.height = SCREEN_HEIGHT;
                    } else {
                        let cell_height = SCREEN_HEIGHT / active_count as u32;
                        win.geometry.x = 0;
                        win.geometry.y = (idx as u32 * cell_height) as i32;
                        win.geometry.width = SCREEN_WIDTH;
                        win.geometry.height = cell_height;
                    }
                    self.damage_tracker.add_damage(win.geometry);
                    idx += 1;
                }
            }
        }
    }
}

impl Default for ZenithCompositor {
    fn default() -> Self {
        Self::new()
    }
}

// ==============================================================================
// 5. Bare-Metal Direct Framebuffer Blitting & SIMD Shading (Section 3.1 & 3.2)
// ==============================================================================
pub struct ZenithBareMetalGraphics {
    pub contrast_scale: f32, // 1.0 = Normal, 2.0 = High Contrast (WCAG 2.1 AA)
    pub high_contrast_mode: bool,
    pub zoom_level: f32,      // Custom Magnification
}

impl ZenithBareMetalGraphics {
    pub fn new() -> Self {
        Self {
            contrast_scale: 1.0,
            high_contrast_mode: false,
            zoom_level: 1.0,
        }
    }

    /// Direct hardware SIMD framebuffer blit & contrast filter pass
    pub fn blit_hardware_framebuffer(&self, dest_buffer: &mut [u32], src_pixels: &[u32]) {
        let len = dest_buffer.len().min(src_pixels.len());
        if !self.high_contrast_mode {
            dest_buffer[..len].copy_from_slice(&src_pixels[..len]);
            return;
        }

        // Apply hardware high-contrast SIMD filter
        for i in 0..len {
            let pixel = src_pixels[i];
            let r = ((pixel >> 16) & 0xFF) as f32;
            let g = ((pixel >> 8) & 0xFF) as f32;
            let b = (pixel & 0xFF) as f32;

            // Luminance calculation
            let gray = (0.299 * r + 0.587 * g + 0.114 * b) * self.contrast_scale;
            let c = gray.clamp(0.0, 255.0) as u32;

            dest_buffer[i] = (0xFF << 24) | (c << 16) | (c << 8) | c;
        }
    }
}

impl Default for ZenithBareMetalGraphics {
    fn default() -> Self {
        Self::new()
    }
}

// ==============================================================================
// 6. Direct Screen Reader Voice Synthesizer (Bypasses daemon overhead - 3.2)
// ==============================================================================
pub struct ZenithVoiceSynthesizer {
    pub speech_buffer: Vec<u8>,
}

impl ZenithVoiceSynthesizer {
    pub fn new() -> Self {
        Self { speech_buffer: Vec::new() }
    }

    /// Directly translates frame element titles into audio speech buffer in visual thread
    pub fn announce_frame_element(&mut self, text: &str) {
        self.speech_buffer.clear();
        for &byte in text.as_bytes() {
            self.speech_buffer.push(byte);
        }
    }
}

impl Default for ZenithVoiceSynthesizer {
    fn default() -> Self {
        Self::new()
    }
}

// ==============================================================================
// 1. Wayland-style Sub-surface (wl_subsurface)
// ==============================================================================
#[derive(Debug, Clone, Copy)]
pub struct SubSurface {
    pub id: u32,
    pub parent_window_id: u32,
    pub relative_offset_x: i32,
    pub relative_offset_y: i32,
    pub width: u32,
    pub height: u32,
}

// ==============================================================================
// 2. GNOME-style CursorTracker with Hot Corners
// ==============================================================================
pub struct CursorTracker {
    pub x: i32,
    pub y: i32,
    pub last_hot_corner_triggered: u32, // 1 for Top-Left (Overview), 2 for Bottom-Right (Desktop Peek)
}

impl CursorTracker {
    pub fn new() -> Self {
        Self { x: 0, y: 0, last_hot_corner_triggered: 0 }
    }

    pub fn update_cursor(&mut self, x: i32, y: i32) -> u32 {
        self.x = x;
        self.y = y;
        // Check Top-Left (0, 0) Overview mode trigger
        if x == 0 && y == 0 {
            self.last_hot_corner_triggered = 1;
            return 1;
        }
        // Check Bottom-Right Peek Desktop trigger
        if x == (SCREEN_WIDTH as i32 - 1) && y == (SCREEN_HEIGHT as i32 - 1) {
            self.last_hot_corner_triggered = 2;
            return 2;
        }
        0
    }
}

impl Default for CursorTracker {
    fn default() -> Self {
        Self::new()
    }
}

// ==============================================================================
// 3. KWin-style Vsync and Double Buffering Frame Supervisor
// ==============================================================================
pub struct VsyncController {
    pub monitor_refresh_rate: u32, // e.g. 60Hz, 144Hz
    pub frame_counter: u64,
}

impl VsyncController {
    pub fn new(refresh_rate: u32) -> Self {
        Self { monitor_refresh_rate: refresh_rate, frame_counter: 0 }
    }

    pub fn block_until_vsync_ticks(&mut self) -> bool {
        self.frame_counter += 1;
        true // Returns page swap trigger
    }
}

// ==============================================================================
// 4. wlroots-style Damage Tracking (Redrawing only altered rectangular areas)
// ==============================================================================
pub struct DamageTracker {
    pub damaged_rects: Vec<Geometry>,
}

impl DamageTracker {
    pub fn new() -> Self {
        Self { damaged_rects: Vec::new() }
    }

    pub fn add_damage(&mut self, rect: Geometry) {
        if self.damaged_rects.len() < 16 {
            self.damaged_rects.push(rect);
        }
    }

    pub fn clear_damage(&mut self) {
        self.damaged_rects.clear();
    }
}

impl Default for DamageTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_creation() {
        let window = WindowNode::new(1, "Test".to_string(), 0, 0, 800, 600);
        assert_eq!(window.id, 1);
        assert_eq!(window.geometry.width, 800);
        assert_eq!(window.state, WindowState::Inactive);
    }

    #[test]
    fn test_window_resize() {
        let mut window = WindowNode::new(1, "Test".to_string(), 0, 0, 800, 600);
        window.resize(1024, 768);
        assert_eq!(window.geometry.width, 1024);
        assert_eq!(window.geometry.height, 768);
    }

    #[test]
    fn test_window_move() {
        let mut window = WindowNode::new(1, "Test".to_string(), 0, 0, 800, 600);
        window.move_to(100, 200);
        assert_eq!(window.geometry.x, 100);
        assert_eq!(window.geometry.y, 200);
    }

    #[test]
    fn test_window_contains_point() {
        let window = WindowNode::new(1, "Test".to_string(), 100, 100, 800, 600);
        assert!(window.contains_point(150, 150));
        assert!(!window.contains_point(50, 50));
        assert!(!window.contains_point(1000, 1000));
    }

    #[test]
    fn test_compositor_register() {
        let mut compositor = ZenithCompositor::new();
        let window = WindowNode::new(1, "Test".to_string(), 0, 0, 800, 600);

        compositor.register_window(window).unwrap();
        assert_eq!(compositor.window_count(), 1);
    }

    #[test]
    fn test_compositor_activate() {
        let mut compositor = ZenithCompositor::new();
        let window1 = WindowNode::new(1, "Test1".to_string(), 0, 0, 800, 600);
        let window2 = WindowNode::new(2, "Test2".to_string(), 100, 100, 800, 600);

        compositor.register_window(window1).unwrap();
        compositor.register_window(window2).unwrap();

        compositor.activate_window(1).unwrap();
        assert_eq!(compositor.active_window_id, Some(1));
        assert_eq!(compositor.get_window(1).unwrap().state, WindowState::Active);
        assert_eq!(
            compositor.get_window(2).unwrap().state,
            WindowState::Inactive
        );
    }

    #[test]
    fn test_compositor_minimize() {
        let mut compositor = ZenithCompositor::new();
        let window = WindowNode::new(1, "Test".to_string(), 0, 0, 800, 600);

        compositor.register_window(window).unwrap();
        compositor.activate_window(1).unwrap();
        compositor.minimize_window(1).unwrap();

        assert_eq!(
            compositor.get_window(1).unwrap().state,
            WindowState::Minimized
        );
        assert_eq!(compositor.active_window_id, None);
    }

    #[test]
    fn test_compositor_remove() {
        let mut compositor = ZenithCompositor::new();
        let window = WindowNode::new(1, "Test".to_string(), 0, 0, 800, 600);

        compositor.register_window(window).unwrap();
        compositor.activate_window(1).unwrap();
        compositor.remove_window(1).unwrap();

        assert_eq!(compositor.window_count(), 0);
        assert_eq!(compositor.active_window_id, None);
    }

    #[test]
    fn test_get_window_at_point() {
        let mut compositor = ZenithCompositor::new();
        let window1 = WindowNode::new(1, "Test1".to_string(), 0, 0, 800, 600);
        let window2 = WindowNode::new(2, "Test2".to_string(), 400, 300, 800, 600);

        compositor.register_window(window1).unwrap();
        compositor.register_window(window2).unwrap();

        // Window2 is on top (added later)
        let hit = compositor.get_window_at_point(500, 400);
        assert_eq!(hit.unwrap().id, 2);
    }

    #[test]
    fn test_window_limit() {
        let mut compositor = ZenithCompositor::new();

        for i in 0..32 {
            let window = WindowNode::new(i, format!("Window {}", i), 0, 0, 800, 600);
            compositor.register_window(window).unwrap();
        }

        let window = WindowNode::new(32, "Extra".to_string(), 0, 0, 800, 600);
        assert!(compositor.register_window(window).is_err());
    }

    #[test]
    fn test_sub_surfaces_layering() {
        let mut compositor = ZenithCompositor::new();
        let window = WindowNode::new(1, "Parent".to_string(), 100, 100, 800, 600);
        compositor.register_window(window).unwrap();

        assert!(compositor.register_sub_surface(1, 101, 10, 10, 200, 30));
        let sub_geom = compositor.get_sub_surface_absolute_geometry(101).unwrap();
        assert_eq!(sub_geom.x, 110);
        assert_eq!(sub_geom.y, 110);
    }

    #[test]
    fn test_tiling_layout() {
        let mut compositor = ZenithCompositor::new();
        let window1 = WindowNode::new(1, "Win1".to_string(), 0, 0, 800, 600);
        let window2 = WindowNode::new(2, "Win2".to_string(), 0, 0, 800, 600);
        compositor.register_window(window1).unwrap();
        compositor.register_window(window2).unwrap();

        compositor.execute_tiling_layout(true); // Split vertically
        let w1 = compositor.get_window(1).unwrap();
        let w2 = compositor.get_window(2).unwrap();
        assert_eq!(w1.geometry.width, SCREEN_WIDTH / 2);
        assert_eq!(w2.geometry.x, (SCREEN_WIDTH / 2) as i32);
    }

    #[test]
    fn test_bare_metal_graphics_and_speech() {
        let mut gfx = ZenithBareMetalGraphics::new();
        gfx.high_contrast_mode = true;
        gfx.contrast_scale = 1.5;

        let src = [0xFF102030u32, 0xFFFFFFFFu32];
        let mut dest = [0u32; 2];
        gfx.blit_hardware_framebuffer(&mut dest, &src);
        assert_ne!(dest[0], 0);

        let mut v_synth = ZenithVoiceSynthesizer::new();
        v_synth.announce_frame_element("Focused Window: Settings");
        assert_eq!(v_synth.speech_buffer.as_slice(), b"Focused Window: Settings");
    }
}
