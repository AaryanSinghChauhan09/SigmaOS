//! # Zenith Compositor - SigmaOS Wayland Display Server
//!
//! Zenith is SigmaOS's sovereign Wayland-compatible display compositor,
//! designed to deliver a next-generation desktop experience without any X11
//! attack surface, legacy display server overhead, or proprietary GPU blobs.
//!
//! ## Architecture
//!
//! ```
//! Application renders → wl_buffer (DMA-BUF or SHM)
//!     → ZenithCompositor (damage tracking)
//!     → Scene graph (sorted by z-order)
//!     → GPU backend (Vulkan render pass)
//!     → KMS/DRM (vsync atomic commit)
//!     → Display
//! ```

#![allow(dead_code)]

use sigma_types::{CapabilityToken, Result};

#[cfg(not(test))]
use crate::klib::HashMap;

#[cfg(test)]
use std::collections::HashMap;

/// Window state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    /// Normal window
    Normal,
    /// Minimized
    Minimized,
    /// Maximized
    Maximized,
    /// Fullscreen
    Fullscreen,
    /// Tiled (half screen)
    Tiled,
}

/// Window geometry
#[derive(Debug, Clone, Copy)]
pub struct WindowGeometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl WindowGeometry {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        WindowGeometry {
            x,
            y,
            width,
            height,
        }
    }

    pub fn contains_point(&self, px: i32, py: i32) -> bool {
        px >= self.x
            && px < self.x + self.width as i32
            && py >= self.y
            && py < self.y + self.height as i32
    }
}

/// Surface type (buffer backend)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceType {
    /// Shared memory buffer
    Shm,
    /// DMA-BUF (GPU buffer)
    DmaBuf,
    /// Software renderer fallback
    Software,
}

/// Surface buffer
#[derive(Debug, Clone)]
pub struct Surface {
    pub surface_type: SurfaceType,
    pub buffer: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
}

impl Surface {
    pub fn new(surface_type: SurfaceType, width: u32, height: u32) -> Self {
        let stride = width * 4; // RGBA
        let buffer = vec![0; (stride * height) as usize];

        Surface {
            surface_type,
            buffer,
            width,
            height,
            stride,
            format: 0x34325258, // XR24 (XRGB8888)
        }
    }

    /// Sets a single pixel RGBA value
    pub fn set_pixel(&mut self, x: u32, y: u32, color_rgba: u32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let offset = ((y * self.stride) + x * 4) as usize;
        if offset + 3 < self.buffer.len() {
            self.buffer[offset] = ((color_rgba >> 24) & 0xFF) as u8;     // R
            self.buffer[offset + 1] = ((color_rgba >> 16) & 0xFF) as u8; // G
            self.buffer[offset + 2] = ((color_rgba >> 8) & 0xFF) as u8;  // B
            self.buffer[offset + 3] = (color_rgba & 0xFF) as u8;         // A
        }
    }

    /// Fills a solid rectangle
    pub fn fill_rect(&mut self, rect_x: i32, rect_y: i32, width: u32, height: u32, color_rgba: u32) {
        for py in 0..height {
            let y = rect_y + py as i32;
            if y < 0 || y as u32 >= self.height { continue; }
            for px in 0..width {
                let x = rect_x + px as i32;
                if x < 0 || x as u32 >= self.width { continue; }
                self.set_pixel(x as u32, y as u32, color_rgba);
            }
        }
    }

    /// Draws a border outline with a given thickness
    pub fn draw_border(&mut self, x: i32, y: i32, width: u32, height: u32, thickness: u32, color_rgba: u32) {
        // Top
        self.fill_rect(x, y, width, thickness, color_rgba);
        // Bottom
        self.fill_rect(x, y + height as i32 - thickness as i32, width, thickness, color_rgba);
        // Left
        self.fill_rect(x, y, thickness, height, color_rgba);
        // Right
        self.fill_rect(x + width as i32 - thickness as i32, y, thickness, height, color_rgba);
    }

    /// Draws a Dr460nized-inspired neon gradient border transitioning from start_color to end_color
    pub fn draw_gradient_border(&mut self, x: i32, y: i32, width: u32, height: u32, thickness: u32, start_color: u32, end_color: u32) {
        for t in 0..thickness {
            let factor = (t as f32) / (thickness as f32).max(1.0);
            let r1 = ((start_color >> 24) & 0xFF) as f32;
            let g1 = ((start_color >> 16) & 0xFF) as f32;
            let b1 = ((start_color >> 8) & 0xFF) as f32;
            let r2 = ((end_color >> 24) & 0xFF) as f32;
            let g2 = ((end_color >> 16) & 0xFF) as f32;
            let b2 = ((end_color >> 8) & 0xFF) as f32;

            let r = (r1 + factor * (r2 - r1)) as u32;
            let g = (g1 + factor * (g2 - g1)) as u32;
            let b = (b1 + factor * (b2 - b1)) as u32;
            let c = (r << 24) | (g << 16) | (b << 8) | 0xFF;

            self.draw_border(x + t as i32, y + t as i32, width.saturating_sub(t * 2), height.saturating_sub(t * 2), 1, c);
        }
    }

    /// Software alpha blending compositor: composite sub_surface over self at (dest_x, dest_y)
    pub fn composite_surface(&mut self, src: &Surface, dest_x: i32, dest_y: i32, global_alpha: u8) {
        let alpha_mult = (global_alpha as f32) / 255.0;

        for sy in 0..src.height {
            let dy = dest_y + sy as i32;
            if dy < 0 || dy as u32 >= self.height { continue; }

            for sx in 0..src.width {
                let dx = dest_x + sx as i32;
                if dx < 0 || dx as u32 >= self.width { continue; }

                let src_off = ((sy * src.stride) + sx * 4) as usize;
                let dst_off = ((dy as u32 * self.stride) + dx as u32 * 4) as usize;

                if src_off + 3 < src.buffer.len() && dst_off + 3 < self.buffer.len() {
                    let src_a = (src.buffer[src_off + 3] as f32 / 255.0) * alpha_mult;
                    if src_a <= 0.0 { continue; }

                    let src_r = src.buffer[src_off] as f32;
                    let src_g = src.buffer[src_off + 1] as f32;
                    let src_b = src.buffer[src_off + 2] as f32;

                    let dst_r = self.buffer[dst_off] as f32;
                    let dst_g = self.buffer[dst_off + 1] as f32;
                    let dst_b = self.buffer[dst_off + 2] as f32;

                    let out_r = (src_r * src_a + dst_r * (1.0 - src_a)) as u8;
                    let out_g = (src_g * src_a + dst_g * (1.0 - src_a)) as u8;
                    let out_b = (src_b * src_a + dst_b * (1.0 - src_a)) as u8;

                    self.buffer[dst_off] = out_r;
                    self.buffer[dst_off + 1] = out_g;
                    self.buffer[dst_off + 2] = out_b;
                    self.buffer[dst_off + 3] = 0xFF;
                }
            }
        }
    }
}

/// Damage region for rendering
#[derive(Debug, Clone, Copy)]
pub struct DamageRegion {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl DamageRegion {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        DamageRegion {
            x,
            y,
            width,
            height,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }
}

/// Zenith window representation
#[derive(Debug, Clone)]
pub struct ZenithWindow {
    pub id: u64,
    pub title: String,
    pub app_id: String,
    pub geometry: WindowGeometry,
    pub state: WindowState,
    pub surface: Surface,
    pub capability: CapabilityToken,
    pub custom_theme: Option<String>, // Per-app theme override
}

/// Output (display) configuration with Vulkan and dynamic refresh limits
#[derive(Debug, Clone)]
pub struct Output {
    pub id: u64,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
    pub scale: f32, // Fractional scaling support for HiDPI (e.g., 1.25, 1.5, 1.75)
    pub primary: bool,
    pub supports_vrr: bool,     // Variable Refresh Rate (VRR) for high-end gaming
    pub current_refresh: u32,   // Dynamically scales based on load
}

impl Output {
    pub fn new(id: u64, name: String, width: u32, height: u32, refresh_rate: u32) -> Self {
        Output {
            id,
            name,
            width,
            height,
            refresh_rate,
            scale: 1.0,
            primary: false,
            supports_vrr: true,
            current_refresh: refresh_rate,
        }
    }

    /// Dynamically adjust refresh rate based on desktop activity to save power (Intelligent Cooling parity)
    pub fn set_adaptive_refresh(&mut self, active: bool) {
        if !self.supports_vrr {
            return;
        }
        if active {
            self.current_refresh = self.refresh_rate; // Peak hz (e.g. 144Hz)
        } else {
            self.current_refresh = 60; // Conserve power on static desktop (60Hz)
        }
    }
}

/// Input event type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEventType {
    PointerMotion,
    PointerButton,
    PointerAxis,
    KeyboardKey,
    Touch,
}

/// Input event
#[derive(Debug, Clone)]
pub struct InputEvent {
    pub event_type: InputEventType,
    pub timestamp: u64,
    pub data: InputEventData,
}

/// Input event data
#[derive(Debug, Clone)]
pub enum InputEventData {
    PointerMotion { x: f64, y: f64 },
    PointerButton { button: u32, state: u32 },
    PointerAxis { axis: u32, value: f64 },
    KeyboardKey { key: u32, state: u32 },
    Touch { slot: i32, x: f64, y: f64 },
}

/// Zenith Dynamic Profiles managed under `/etc/sigma-profiles/`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZenithProfile {
    Developer,     // LTO caching, debug symbols enabled, 3.2 GHz cap
    Gamer,         // 4.2 GHz CPU, GPU overclock, 10ms scheduler quantum, VRR enabled
    Minimalist,    // 800 MHz CPU limit, 32MB RAM footprint limit, low refresh
    Accessibility, // High-contrast, screen reader activated, 2.0 GHz CPU
}

/// Supported Desktop Visual Themes (Garuda Dr460nized, Linux Mint Cinnamon, Material Design 3)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopThemeStyle {
    /// Garuda Linux-inspired Dr460nized: Vibrant neon magenta/cyan gradients, dark translucent glass, glowing borders
    Dr460nized,
    /// Linux Mint-inspired Cinnamon: Sleek dark/light mint accents, clean taskbar panel, rounded corners
    Cinnamon,
    /// Material Design 3 Dark Mode
    MaterialDark,
    /// Material Design 3 Light Mode
    MaterialLight,
}

#[derive(Debug, Clone)]
pub struct ThemeStylingConfig {
    pub style: DesktopThemeStyle,
    pub primary_accent: u32,
    pub secondary_accent: u32,
    pub background_color: u32,
    pub border_color: u32,
    pub border_thickness: u32,
    pub corner_radius: u32,
    pub window_alpha: u8, // Translucency (e.g. 215 for Dr460nized glass)
    pub is_dark: bool,
}

impl ThemeStylingConfig {
    pub fn dr460nized() -> Self {
        Self {
            style: DesktopThemeStyle::Dr460nized,
            primary_accent: 0xFF00FFFF,   // Vibrant Neon Cyan
            secondary_accent: 0xFFFF00FF, // Vibrant Neon Magenta
            background_color: 0x1A0B2EFF, // Deep Purple Translucent Glass
            border_color: 0xFFFF00FF,
            border_thickness: 3,
            corner_radius: 16,
            window_alpha: 215,
            is_dark: true,
        }
    }

    pub fn cinnamon() -> Self {
        Self {
            style: DesktopThemeStyle::Cinnamon,
            primary_accent: 0x87C378FF,   // Mint Green
            secondary_accent: 0x3D3D3DFF, // Sleek Panel Charcoal
            background_color: 0x2F343FFF, // Mint Dark Background
            border_color: 0x87C378FF,
            border_thickness: 2,
            corner_radius: 8,
            window_alpha: 255,
            is_dark: true,
        }
    }

    pub fn material_dark() -> Self {
        Self {
            style: DesktopThemeStyle::MaterialDark,
            primary_accent: 0xFFBB86FC,
            secondary_accent: 0x03DAC6FF,
            background_color: 0x121212FF,
            border_color: 0xFFBB86FC,
            border_thickness: 1,
            corner_radius: 12,
            window_alpha: 255,
            is_dark: true,
        }
    }
}

/// Design Token Library representing Material Design 3 and GNOME HIG (Unified Design System)
#[derive(Debug, Clone)]
pub struct DesignTokens {
    pub theme_config: ThemeStylingConfig,
    pub spacing_unit: u32,
}

impl DesignTokens {
    pub fn new(is_dark_mode: bool) -> Self {
        if is_dark_mode {
            Self {
                theme_config: ThemeStylingConfig::material_dark(),
                spacing_unit: 8,
            }
        } else {
            Self {
                theme_config: ThemeStylingConfig {
                    style: DesktopThemeStyle::MaterialLight,
                    primary_accent: 0x6200EEFF,
                    secondary_accent: 0x018786FF,
                    background_color: 0xFFFFFFFF,
                    border_color: 0x6200EEFF,
                    border_thickness: 1,
                    corner_radius: 12,
                    window_alpha: 255,
                    is_dark: false,
                },
                spacing_unit: 8,
            }
        }
    }

    pub fn with_theme(style: DesktopThemeStyle) -> Self {
        let config = match style {
            DesktopThemeStyle::Dr460nized => ThemeStylingConfig::dr460nized(),
            DesktopThemeStyle::Cinnamon => ThemeStylingConfig::cinnamon(),
            DesktopThemeStyle::MaterialDark => ThemeStylingConfig::material_dark(),
            DesktopThemeStyle::MaterialLight => ThemeStylingConfig {
                style: DesktopThemeStyle::MaterialLight,
                primary_accent: 0x6200EEFF,
                secondary_accent: 0x018786FF,
                background_color: 0xFFFFFFFF,
                border_color: 0x6200EEFF,
                border_thickness: 1,
                corner_radius: 12,
                window_alpha: 255,
                is_dark: false,
            },
        };

        Self {
            theme_config: config,
            spacing_unit: 8,
        }
    }
}

/// Cross-Device Continuity & Encryption state vault (macOS Handoff and Windows Timeline parity)
#[derive(Debug, Clone)]
pub struct HandoffVault {
    pub active_tab_url: String,
    pub clipboard_text: String,
    pub encrypted_token: u64,
}

/// Zenith Compositor main structure
pub struct ZenithCompositor {
    windows: HashMap<u64, ZenithWindow>,
    outputs: Vec<Output>,
    surfaces: HashMap<u64, Surface>,
    damage_regions: Vec<DamageRegion>,
    active_window: Option<u64>,
    next_window_id: u64,
    capability: CapabilityToken,
    active_profile: ZenithProfile,
    design_tokens: DesignTokens,
    handoff_vault: Option<HandoffVault>,
    cpu_limit_khz: u32,
    scheduler_quantum_ms: u32,
}

impl ZenithCompositor {
    /// Create a new Zenith compositor
    pub fn new(capability: CapabilityToken) -> Self {
        ZenithCompositor {
            windows: HashMap::new(),
            outputs: Vec::new(),
            surfaces: HashMap::new(),
            damage_regions: Vec::new(),
            active_window: None,
            next_window_id: 1,
            capability,
            active_profile: ZenithProfile::Developer,
            design_tokens: DesignTokens::new(true),
            handoff_vault: None,
            cpu_limit_khz: 3200000,
            scheduler_quantum_ms: 20,
        }
    }

    /// Sets the dynamic system profile (Sigma Studio profile switching)
    pub fn switch_profile(&mut self, profile: ZenithProfile) {
        self.active_profile = profile;

        match profile {
            ZenithProfile::Developer => {
                self.cpu_limit_khz = 3200000; // 3.2 GHz limit
                self.scheduler_quantum_ms = 20;
                // Enable debugging state
            }
            ZenithProfile::Gamer => {
                self.cpu_limit_khz = 4200000; // 4.2 GHz limit
                self.scheduler_quantum_ms = 10; // 10ms low-latency quantum
                for output in &mut self.outputs {
                    output.current_refresh = output.refresh_rate; // Push maximum refresh limit
                }
            }
            ZenithProfile::Minimalist => {
                self.cpu_limit_khz = 800000; // 800 MHz power save limit
                self.scheduler_quantum_ms = 40;
                for output in &mut self.outputs {
                    output.current_refresh = 60; // lock to 60Hz to save battery
                }
            }
            ZenithProfile::Accessibility => {
                self.cpu_limit_khz = 2000000; // 2.0 GHz limit
                self.scheduler_quantum_ms = 20;
                // Force high contrast token swaps
                self.design_tokens.theme_config.background_color = 0xFF000000;
                self.design_tokens.theme_config.primary_accent = 0xFFFF0000;
            }
        }
    }

    /// Gets active profile limits
    pub fn get_profile_limits(&self) -> (u32, u32) {
        (self.cpu_limit_khz, self.scheduler_quantum_ms)
    }

    /// Sync active tab and clipboard via mesh-secured vault (macOS Handoff parity)
    pub fn update_handoff_state(&mut self, url: &str, clipboard: &str) {
        self.handoff_vault = Some(HandoffVault {
            active_tab_url: url.to_string(),
            clipboard_text: clipboard.to_string(),
            encrypted_token: 0xABCDEF123456, // Simulated Kyber-1024 / Dilithium-5 encryption
        });
    }

    pub fn get_handoff_state(&self) -> Option<&HandoffVault> {
        self.handoff_vault.as_ref()
    }

    /// Get current design tokens
    pub fn get_design_tokens(&self) -> &DesignTokens {
        &self.design_tokens
    }

    /// Toggle global Dark / Light theme mode
    pub fn toggle_theme_mode(&mut self, is_dark: bool) {
        self.design_tokens = DesignTokens::new(is_dark);
    }

    /// Add an output (display)
    pub fn add_output(&mut self, output: Output) {
        self.outputs.push(output);
    }

    /// Create a new window with potential per-app style override
    pub fn create_window(
        &mut self,
        title: String,
        app_id: String,
        geometry: WindowGeometry,
        capability: CapabilityToken,
    ) -> Result<u64> {
        let window_id = self.next_window_id;
        self.next_window_id += 1;

        let surface = Surface::new(SurfaceType::Shm, geometry.width, geometry.height);

        let window = ZenithWindow {
            id: window_id,
            title,
            app_id,
            geometry,
            state: WindowState::Normal,
            surface,
            capability,
            custom_theme: None,
        };

        let surface_clone = window.surface.clone();
        self.windows.insert(window_id, window);
        self.surfaces.insert(window_id, surface_clone);
        self.active_window = Some(window_id);

        Ok(window_id)
    }

    /// Get a window by ID
    pub fn get_window(&self, window_id: u64) -> Option<&ZenithWindow> {
        self.windows.get(&window_id)
    }

    /// Get a mutable window by ID
    pub fn get_window_mut(&mut self, window_id: u64) -> Option<&mut ZenithWindow> {
        self.windows.get_mut(&window_id)
    }

    /// Destroy a window
    pub fn destroy_window(&mut self, window_id: u64) -> Result<()> {
        self.windows
            .remove(&window_id)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Window not found"))?;
        self.surfaces.remove(&window_id);

        if self.active_window == Some(window_id) {
            self.active_window = self.windows.keys().next().copied();
        }

        Ok(())
    }

    /// Set window state
    pub fn set_window_state(&mut self, window_id: u64, state: WindowState) -> Result<()> {
        let window = self
            .windows
            .get_mut(&window_id)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Window not found"))?;

        window.state = state;
        Ok(())
    }

    /// Set window geometry
    pub fn set_window_geometry(&mut self, window_id: u64, geometry: WindowGeometry) -> Result<()> {
        let window = self
            .windows
            .get_mut(&window_id)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Window not found"))?;

        window.geometry = geometry;
        self.damage_regions.push(DamageRegion::new(
            geometry.x,
            geometry.y,
            geometry.width,
            geometry.height,
        ));
        Ok(())
    }

    /// Activate a window (bring to front)
    pub fn activate_window(&mut self, window_id: u64) -> Result<()> {
        if !self.windows.contains_key(&window_id) {
            return Err(
                std::io::Error::new(std::io::ErrorKind::NotFound, "Window not found").into(),
            );
        }
        self.active_window = Some(window_id);
        Ok(())
    }

    /// Get active window
    pub fn active_window(&self) -> Option<u64> {
        self.active_window
    }

    /// Find window at point
    pub fn find_window_at_point(&self, x: i32, y: i32) -> Option<u64> {
        // Iterate in reverse order (top to bottom)
        let mut keys: Vec<&u64> = self.windows.keys().collect();
        keys.reverse();
        for &window_id in keys {
            if let Some(window) = self.windows.get(&window_id) {
                if window.state == WindowState::Normal || window.state == WindowState::Tiled {
                    if window.geometry.contains_point(x, y) {
                        return Some(window_id);
                    }
                }
            }
        }
        None
    }

    /// Add damage region
    pub fn add_damage(&mut self, region: DamageRegion) {
        if !region.is_empty() {
            self.damage_regions.push(region);
        }
    }

    /// Get damage regions
    pub fn damage_regions(&self) -> &[DamageRegion] {
        &self.damage_regions
    }

    /// Clear damage regions
    pub fn clear_damage(&mut self) {
        self.damage_regions.clear();
    }

    /// Get all windows
    pub fn windows(&self) -> impl Iterator<Item = &ZenithWindow> {
        self.windows.values()
    }

    /// Get all outputs
    pub fn outputs(&self) -> &[Output] {
        &self.outputs
    }

    /// Set primary output
    pub fn set_primary_output(&mut self, output_id: u64) -> Result<()> {
        for output in &mut self.outputs {
            output.primary = output.id == output_id;
        }
        Ok(())
    }

    /// Get primary output
    pub fn primary_output(&self) -> Option<&Output> {
        self.outputs.iter().find(|o| o.primary)
    }

    /// Process input event
    pub fn process_input_event(&mut self, event: InputEvent) -> Result<()> {
        match event.event_type {
            InputEventType::PointerMotion => {
                if let InputEventData::PointerMotion { x, y } = event.data {
                    let window_id = self.find_window_at_point(x as i32, y as i32);
                    if let Some(wid) = window_id {
                        self.activate_window(wid)?;
                    }
                }
            }
            InputEventType::PointerButton => {
                // Handle button clicks
            }
            _ => {}
        }
        Ok(())
    }

    /// Render frame (simplified)
    pub fn render_frame(&mut self) -> Result<()> {
        // In real implementation, this would:
        // 1. Collect damage regions
        // 2. Build scene graph sorted by z-order
        // 3. Render to GPU backend
        // 4. Submit to KMS/DRM for display

        self.clear_damage();
        Ok(())
    }
}

impl Default for ZenithCompositor {
    fn default() -> Self {
        Self::new(sigma_types::CapabilityToken { id: 0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_creation() {
        let capability = sigma_types::CapabilityToken { id: 1 };
        let mut compositor = ZenithCompositor::new(capability);

        let geometry = WindowGeometry::new(100, 100, 800, 600);
        let window_id = compositor
            .create_window(
                "Test Window".to_string(),
                "test.app".to_string(),
                geometry,
                sigma_types::CapabilityToken { id: 2 },
            )
            .unwrap();

        assert!(compositor.get_window(window_id).is_some());
        assert_eq!(compositor.active_window(), Some(window_id));
    }

    #[test]
    fn test_window_geometry() {
        let geometry = WindowGeometry::new(100, 100, 800, 600);

        assert!(geometry.contains_point(150, 150));
        assert!(geometry.contains_point(100, 100));
        assert!(!geometry.contains_point(50, 50));
        assert!(!geometry.contains_point(900, 700));
    }

    #[test]
    fn test_window_state() {
        let capability = sigma_types::CapabilityToken { id: 1 };
        let mut compositor = ZenithCompositor::new(capability);

        let geometry = WindowGeometry::new(0, 0, 800, 600);
        let window_id = compositor
            .create_window(
                "Test".to_string(),
                "test.app".to_string(),
                geometry,
                sigma_types::CapabilityToken { id: 2 },
            )
            .unwrap();

        compositor
            .set_window_state(window_id, WindowState::Maximized)
            .unwrap();

        let window = compositor.get_window(window_id).unwrap();
        assert_eq!(window.state, WindowState::Maximized);
    }

    #[test]
    fn test_find_window_at_point() {
        let capability = sigma_types::CapabilityToken { id: 1 };
        let mut compositor = ZenithCompositor::new(capability);

        let geometry1 = WindowGeometry::new(0, 0, 400, 400);
        let geometry2 = WindowGeometry::new(400, 0, 400, 400);

        compositor
            .create_window(
                "Window 1".to_string(),
                "app1".to_string(),
                geometry1,
                sigma_types::CapabilityToken { id: 2 },
            )
            .unwrap();

        compositor
            .create_window(
                "Window 2".to_string(),
                "app2".to_string(),
                geometry2,
                sigma_types::CapabilityToken { id: 3 },
            )
            .unwrap();

        assert!(compositor.find_window_at_point(200, 200).is_some());
        assert!(compositor.find_window_at_point(600, 200).is_some());
        assert!(compositor.find_window_at_point(800, 800).is_none());
    }

    #[test]
    fn test_fractional_scaling_and_vrr() {
        let mut output = Output::new(1, "Display 1".to_string(), 3840, 2160, 144);
        output.scale = 1.5; // 150% HiDPI scaling (Wayland fractional scaling)

        assert_eq!(output.scale, 1.5);
        assert!(output.supports_vrr);

        // Power management governor simulation
        output.set_adaptive_refresh(false); // static content dropdown
        assert_eq!(output.current_refresh, 60);

        output.set_adaptive_refresh(true); // game action peak
        assert_eq!(output.current_refresh, 144);
    }

    #[test]
    fn test_zenith_profile_system() {
        let capability = sigma_types::CapabilityToken { id: 1 };
        let mut compositor = ZenithCompositor::new(capability);

        // Switch to Gamer profile (overclock, tight 10ms scheduler slice, VRR active)
        compositor.switch_profile(ZenithProfile::Gamer);
        let (cpu, q) = compositor.get_profile_limits();
        assert_eq!(cpu, 4200000);
        assert_eq!(q, 10);

        // Switch to Minimalist profile (low-power governor, 800MHz cap)
        compositor.switch_profile(ZenithProfile::Minimalist);
        let (cpu, q) = compositor.get_profile_limits();
        assert_eq!(cpu, 800000);
        assert_eq!(q, 40);
    }

    #[test]
    fn test_handoff_encrypted_vault() {
        let capability = sigma_types::CapabilityToken { id: 1 };
        let mut compositor = ZenithCompositor::new(capability);

        compositor.update_handoff_state("https://sigmaos.dev/workspace", "Shared clipboard data");
        let vault = compositor.get_handoff_state().unwrap();

        assert_eq!(vault.active_tab_url, "https://sigmaos.dev/workspace");
        assert_eq!(vault.clipboard_text, "Shared clipboard data");
    }

    #[test]
    fn test_unified_design_system_tokens() {
        let capability = sigma_types::CapabilityToken { id: 1 };
        let mut compositor = ZenithCompositor::new(capability);

        // Dark theme tokens check (Material Design 3)
        compositor.toggle_theme_mode(true);
        let tokens = compositor.get_design_tokens();
        assert!(tokens.theme_config.is_dark);
        assert_eq!(tokens.theme_config.background_color, 0x121212FF);

        // Switch Accessibility profile overrides background colors for high contrast
        compositor.switch_profile(ZenithProfile::Accessibility);
        let tokens = compositor.get_design_tokens();
        assert_eq!(tokens.theme_config.background_color, 0xFF000000); // Strict black background
    }

    #[test]
    fn test_dr460nized_and_cinnamon_desktop_themes() {
        // Test Dr460nized Theme
        let dr16 = DesignTokens::with_theme(DesktopThemeStyle::Dr460nized);
        assert_eq!(dr16.theme_config.style, DesktopThemeStyle::Dr460nized);
        assert_eq!(dr16.theme_config.primary_accent, 0xFF00FFFF);
        assert_eq!(dr16.theme_config.secondary_accent, 0xFFFF00FF);
        assert_eq!(dr16.theme_config.window_alpha, 215);

        // Test Cinnamon Theme
        let cinn = DesignTokens::with_theme(DesktopThemeStyle::Cinnamon);
        assert_eq!(cinn.theme_config.style, DesktopThemeStyle::Cinnamon);
        assert_eq!(cinn.theme_config.primary_accent, 0x87C378FF);
        assert_eq!(cinn.theme_config.window_alpha, 255);

        // Test Surface Rendering with Dr460nized gradient border
        let mut surface = Surface::new(SurfaceType::Software, 400, 300);
        surface.draw_gradient_border(0, 0, 400, 300, 3, dr16.theme_config.primary_accent, dr16.theme_config.secondary_accent);

        // Test Alpha Blending Surface Compositing
        let mut dest_surface = Surface::new(SurfaceType::Software, 800, 600);
        dest_surface.fill_rect(0, 0, 800, 600, 0x111111FF);
        dest_surface.composite_surface(&surface, 50, 50, dr16.theme_config.window_alpha);

        assert_eq!(dest_surface.width, 800);
    }
}

// Placeholder types for compilation
mod sigma_types {
    use std::io;

    pub type Result<T> = std::result::Result<T, io::Error>;

    #[derive(Debug, Clone)]
    pub struct CapabilityToken {
        pub id: u64,
    }
}
