// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// desktop/zenith_compositor.rs — Zenith Desktop Compositor
//
// Implements the Zenith Desktop compositor with DRM/KMS support,
// window management, and rendering pipeline for SigmaOS desktop environment.
//
// Language: Rust (std for userland services)

use std::collections::HashMap;

// ─── Compositor Constants ───────────────────────────────────────────────────────

pub const MAX_WINDOWS: usize = 64;
pub const DEFAULT_WIDTH: u32 = 1920;
pub const DEFAULT_HEIGHT: u32 = 1080;

// ─── Window Structure ───────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Window {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub visible: bool,
    pub focused: bool,
    pub minimized: bool,
}

// ─── Surface/Buffer Structure ───────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Surface {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: PixelFormat,
    pub buffer: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PixelFormat {
    ARGB8888,
    XRGB8888,
    RGB565,
}

// ─── Layout Types ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Layout {
    MasterStack,    // One large window + side panel
    Stack,          // All windows equal size
    Tabbed,         // Tabbed interface
}

// ─── Zenith Compositor ─────────────────────────────────────────────────────────

pub struct ZenithCompositor {
    pub width: u32,
    pub height: u32,
    pub framebuffer: Vec<u8>,
    pub windows: HashMap<u32, Window>,
    pub surfaces: HashMap<u32, Surface>,
    pub focused_window: Option<u32>,
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub layout: Layout,
    pub master_window: Option<u32>,
    pub initialized: bool,
}

impl ZenithCompositor {
    pub fn new(width: u32, height: u32) -> Self {
        let framebuffer_size = (width * height * 4) as usize;
        ZenithCompositor {
            width,
            height,
            framebuffer: vec![0u8; framebuffer_size],
            windows: HashMap::new(),
            surfaces: HashMap::new(),
            focused_window: None,
            cursor_x: (width / 2) as i32,
            cursor_y: (height / 2) as i32,
            layout: Layout::MasterStack,
            master_window: None,
            initialized: false,
        }
    }

    /// Initialize compositor with DRM/KMS
    pub fn init(&mut self) -> Result<(), String> {
        // In a real implementation, this would:
        // 1. Open DRM device (/dev/dri/card0)
        // 2. Get DRM resources (connectors, encoders, CRTCs)
        // 3. Find connected connector
        // 4. Create dumb buffer or GBM buffer
        // 5. Map framebuffer
        // 6. Set mode (CRTC configuration)
        
        self.initialized = true;
        Ok(())
    }

    /// Create a new window
    pub fn create_window(&mut self, title: String, x: i32, y: i32, width: u32, height: u32) -> u32 {
        let window_id = self.windows.len() as u32 + 1;
        
        let window = Window {
            id: window_id,
            x,
            y,
            width,
            height,
            title,
            visible: true,
            focused: false,
            minimized: false,
        };
        
        self.windows.insert(window_id, window);
        window_id
    }

    /// Destroy a window
    pub fn destroy_window(&mut self, window_id: u32) -> Result<(), String> {
        if self.windows.remove(&window_id).is_some() {
            self.surfaces.remove(&window_id);
            if self.focused_window == Some(window_id) {
                self.focused_window = None;
            }
            Ok(())
        } else {
            Err("Window not found".to_string())
        }
    }

    /// Focus a window
    pub fn focus_window(&mut self, window_id: u32) -> Result<(), String> {
        if self.windows.contains_key(&window_id) {
            // Unfocus previous
            if let Some(prev_id) = self.focused_window {
                if let Some(window) = self.windows.get_mut(&prev_id) {
                    window.focused = false;
                }
            }
            
            // Focus new window
            if let Some(window) = self.windows.get_mut(&window_id) {
                window.focused = true;
            }
            
            self.focused_window = Some(window_id);
            Ok(())
        } else {
            Err("Window not found".to_string())
        }
    }

    /// Move a window
    pub fn move_window(&mut self, window_id: u32, x: i32, y: i32) -> Result<(), String> {
        if let Some(window) = self.windows.get_mut(&window_id) {
            window.x = x;
            window.y = y;
            Ok(())
        } else {
            Err("Window not found".to_string())
        }
    }

    /// Resize a window
    pub fn resize_window(&mut self, window_id: u32, width: u32, height: u32) -> Result<(), String> {
        if let Some(window) = self.windows.get_mut(&window_id) {
            window.width = width;
            window.height = height;
            Ok(())
        } else {
            Err("Window not found".to_string())
        }
    }

    /// Minimize a window
    pub fn minimize_window(&mut self, window_id: u32) -> Result<(), String> {
        if let Some(window) = self.windows.get_mut(&window_id) {
            window.minimized = true;
            window.visible = false;
            Ok(())
        } else {
            Err("Window not found".to_string())
        }
    }

    /// Restore a minimized window
    pub fn restore_window(&mut self, window_id: u32) -> Result<(), String> {
        if let Some(window) = self.windows.get_mut(&window_id) {
            window.minimized = false;
            window.visible = true;
            Ok(())
        } else {
            Err("Window not found".to_string())
        }
    }

    /// Attach surface to window
    pub fn attach_surface(&mut self, window_id: u32, surface: Surface) -> Result<(), String> {
        if self.windows.contains_key(&window_id) {
            self.surfaces.insert(window_id, surface);
            Ok(())
        } else {
            Err("Window not found".to_string())
        }
    }

    /// Compose and render all windows to framebuffer
    pub fn compose(&mut self) -> Result<(), String> {
        if !self.initialized {
            return Err("Compositor not initialized".to_string());
        }

        // Clear framebuffer to background color
        let bg_color = [0x20, 0x20, 0x30, 0xFF]; // Dark blue-gray
        for i in (0..self.framebuffer.len()).step_by(4) {
            self.framebuffer[i] = bg_color[0];
            self.framebuffer[i + 1] = bg_color[1];
            self.framebuffer[i + 2] = bg_color[2];
            self.framebuffer[i + 3] = bg_color[3];
        }

        // Render windows in order (back to front)
        let mut window_ids: Vec<u32> = self.windows.keys().cloned().collect();
        window_ids.sort(); // Simple z-ordering by ID

        for window_id in window_ids {
            if let Some(window) = self.windows.get(&window_id) {
                if !window.visible || window.minimized {
                    continue;
                }

                if let Some(surface) = self.surfaces.get(&window_id) {
                    self.blit_surface(surface, window.x, window.y);
                }

                // Draw window border if focused
                if window.focused {
                    self.draw_border(window.x, window.y, window.width, window.height, [0x00, 0xFF, 0xFF, 0xFF]);
                }
            }
        }

        // Draw cursor
        self.draw_cursor();

        Ok(())
    }

    /// Blit surface to framebuffer
    fn blit_surface(&mut self, surface: &Surface, dst_x: i32, dst_y: i32) {
        let src_width = surface.width as usize;
        let src_height = surface.height as usize;
        let dst_width = self.width as usize;
        let dst_height = self.height as usize;

        for y in 0..src_height {
            for x in 0..src_width {
                let dst_x_abs = dst_x as usize + x;
                let dst_y_abs = dst_y as usize + y;

                if dst_x_abs < dst_width && dst_y_abs < dst_height {
                    let src_idx = (y * src_width + x) * 4;
                    let dst_idx = (dst_y_abs * dst_width + dst_x_abs) * 4;

                    if src_idx + 3 < surface.buffer.len() && dst_idx + 3 < self.framebuffer.len() {
                        // Simple alpha blending
                        let alpha = surface.buffer[src_idx + 3] as f32 / 255.0;
                        let inv_alpha = 1.0 - alpha;

                        for c in 0..4 {
                            let src_val = surface.buffer[src_idx + c] as f32;
                            let dst_val = self.framebuffer[dst_idx + c] as f32;
                            self.framebuffer[dst_idx + c] = (src_val * alpha + dst_val * inv_alpha) as u8;
                        }
                    }
                }
            }
        }
    }

    /// Draw window border
    fn draw_border(&mut self, x: i32, y: i32, width: u32, height: u32, color: [u8; 4]) {
        let x = x as usize;
        let y = y as usize;
        let width = width as usize;
        let height = height as usize;
        let fb_width = self.width as usize;

        // Top border
        for bx in x..(x + width).min(self.width as usize) {
            let idx = (y * fb_width + bx) * 4;
            if idx + 3 < self.framebuffer.len() {
                self.framebuffer[idx..idx + 4].copy_from_slice(&color);
            }
        }

        // Bottom border
        for bx in x..(x + width).min(self.width as usize) {
            let idx = ((y + height - 1) * fb_width + bx) * 4;
            if idx + 3 < self.framebuffer.len() {
                self.framebuffer[idx..idx + 4].copy_from_slice(&color);
            }
        }

        // Left border
        for by in y..(y + height).min(self.height as usize) {
            let idx = (by * fb_width + x) * 4;
            if idx + 3 < self.framebuffer.len() {
                self.framebuffer[idx..idx + 4].copy_from_slice(&color);
            }
        }

        // Right border
        for by in y..(y + height).min(self.height as usize) {
            let idx = (by * fb_width + x + width - 1) * 4;
            if idx + 3 < self.framebuffer.len() {
                self.framebuffer[idx..idx + 4].copy_from_slice(&color);
            }
        }
    }

    /// Draw cursor
    fn draw_cursor(&mut self) {
        let cursor_size = 16;
        let cursor_color = [0xFF, 0xFF, 0xFF, 0xFF];
        let fb_width = self.width as usize;

        for dy in 0..cursor_size {
            for dx in 0..cursor_size {
                let dst_x = self.cursor_x as usize + dx;
                let dst_y = self.cursor_y as usize + dy;

                if dst_x < self.width as usize && dst_y < self.height as usize {
                    let idx = (dst_y * fb_width + dst_x) * 4;
                    if idx + 3 < self.framebuffer.len() {
                        // Simple arrow shape
                        if dy <= dx && dx + dy < cursor_size {
                            self.framebuffer[idx..idx + 4].copy_from_slice(&cursor_color);
                        }
                    }
                }
            }
        }
    }

    /// Update cursor position
    pub fn move_cursor(&mut self, x: i32, y: i32) {
        self.cursor_x = x.max(0).min(self.width as i32 - 1);
        self.cursor_y = y.max(0).min(self.height as i32 - 1);
    }

    /// Get list of windows
    pub fn list_windows(&self) -> Vec<&Window> {
        self.windows.values().collect()
    }

    /// Switch layout (Super+{1,2,3})
    pub fn switch_layout(&mut self, layout: Layout) {
        self.layout = layout;
        self.apply_layout();
    }

    /// Set master window for MasterStack layout
    pub fn set_master_window(&mut self, window_id: u32) -> Result<(), String> {
        if self.windows.contains_key(&window_id) {
            self.master_window = Some(window_id);
            self.apply_layout();
            Ok(())
        } else {
            Err("Window not found".to_string())
        }
    }

    /// Apply current layout to all windows
    fn apply_layout(&mut self) {
        let window_ids: Vec<u32> = self.windows.keys().cloned().collect();
        
        match self.layout {
            Layout::MasterStack => {
                self.apply_master_stack(&window_ids);
            }
            Layout::Stack => {
                self.apply_stack(&window_ids);
            }
            Layout::Tabbed => {
                self.apply_tabbed(&window_ids);
            }
        }
    }

    /// Master-stack layout: one large window + side panel
    fn apply_master_stack(&mut self, window_ids: &[u32]) {
        if window_ids.is_empty() {
            return;
        }

        let master_id = self.master_window.or_else(|| window_ids.first().copied());
        
        if let Some(master) = master_id {
            // Master window takes 60% of screen width
            if let Some(window) = self.windows.get_mut(&master) {
                window.x = 0;
                window.y = 0;
                window.width = (self.width as f64 * 0.6) as u32;
                window.height = self.height;
            }

            // Stack windows in side panel (40% width)
            let panel_width = (self.width as f64 * 0.4) as u32;
            let panel_x = (self.width as f64 * 0.6) as i32;
            let stack_height = self.height / window_ids.len().max(1) as u32;

            let mut stack_idx = 0;
            for &wid in window_ids {
                if wid != master {
                    if let Some(window) = self.windows.get_mut(&wid) {
                        window.x = panel_x;
                        window.y = (stack_idx * stack_height) as i32;
                        window.width = panel_width;
                        window.height = stack_height;
                        stack_idx += 1;
                    }
                }
            }
        }
    }

    /// Stack layout: all windows equal size
    fn apply_stack(&mut self, window_ids: &[u32]) {
        if window_ids.is_empty() {
            return;
        }

        let cols = (window_ids.len() as f64).sqrt().ceil() as usize;
        let rows = (window_ids.len() as f64 / cols as f64).ceil() as usize;
        
        let win_width = self.width / cols as u32;
        let win_height = self.height / rows as u32;

        for (i, &wid) in window_ids.iter().enumerate() {
            if let Some(window) = self.windows.get_mut(&wid) {
                let col = i % cols;
                let row = i / cols;
                window.x = (col * win_width as usize) as i32;
                window.y = (row * win_height as usize) as i32;
                window.width = win_width;
                window.height = win_height;
            }
        }
    }

    /// Tabbed layout: all windows full screen (tabbed)
    fn apply_tabbed(&mut self, window_ids: &[u32]) {
        for &wid in window_ids {
            if let Some(window) = self.windows.get_mut(&wid) {
                window.x = 0;
                window.y = 0;
                window.width = self.width;
                window.height = self.height;
                // Only show focused window
                window.visible = Some(wid) == self.focused_window;
            }
        }
    }

    /// Snap window to edge
    pub fn snap_to_edge(&mut self, window_id: u32, edge: Edge) -> Result<(), String> {
        if let Some(window) = self.windows.get_mut(&window_id) {
            match edge {
                Edge::Left => {
                    window.x = 0;
                    window.y = 0;
                    window.width = self.width / 2;
                    window.height = self.height;
                }
                Edge::Right => {
                    window.x = (self.width / 2) as i32;
                    window.y = 0;
                    window.width = self.width / 2;
                    window.height = self.height;
                }
                Edge::Top => {
                    window.x = 0;
                    window.y = 0;
                    window.width = self.width;
                    window.height = self.height / 2;
                }
                Edge::Bottom => {
                    window.x = 0;
                    window.y = (self.height / 2) as i32;
                    window.width = self.width;
                    window.height = self.height / 2;
                }
                Edge::TopLeft => {
                    window.x = 0;
                    window.y = 0;
                    window.width = self.width / 2;
                    window.height = self.height / 2;
                }
                Edge::TopRight => {
                    window.x = (self.width / 2) as i32;
                    window.y = 0;
                    window.width = self.width / 2;
                    window.height = self.height / 2;
                }
                Edge::BottomLeft => {
                    window.x = 0;
                    window.y = (self.height / 2) as i32;
                    window.width = self.width / 2;
                    window.height = self.height / 2;
                }
                Edge::BottomRight => {
                    window.x = (self.width / 2) as i32;
                    window.y = (self.height / 2) as i32;
                    window.width = self.width / 2;
                    window.height = self.height / 2;
                }
            }
            Ok(())
        } else {
            Err("Window not found".to_string())
        }
    }
}

// ─── Edge Enum for Snap-to-Edge ─────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Edge {
    Left,
    Right,
    Top,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}
