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

use sigma_types::{CapabilityToken, Result};
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

/// Output (display) configuration
#[derive(Debug, Clone)]
pub struct Output {
    pub id: u64,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
    pub scale: f32,
    pub primary: bool,
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

/// Zenith Compositor main structure
pub struct ZenithCompositor {
    windows: HashMap<u64, ZenithWindow>,
    outputs: Vec<Output>,
    surfaces: HashMap<u64, Surface>,
    damage_regions: Vec<DamageRegion>,
    active_window: Option<u64>,
    next_window_id: u64,
    capability: CapabilityToken,
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
        }
    }

    /// Add an output (display)
    pub fn add_output(&mut self, output: Output) {
        self.outputs.push(output);
    }

    /// Create a new window
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
        let mut keys: Vec<u64> = self.windows.keys().copied().collect();
        keys.reverse();
        for window_id in keys {
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
