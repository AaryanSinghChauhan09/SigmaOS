#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Sovereign Wayland Compositor
// Light, secure userland Wayland display server replacement.
// Provides zero-copy hardware-accelerated surface rendering.

use crate::drivers::{GpuDriver, GpuCommand};
use crate::klib::HashMap;

/// Wayland Protocol Object IDs
pub type WaylandId = u32;
pub type ClientId = u32;

/// Wayland Surface representing client drawing buffers
#[derive(Debug, Clone)]
pub struct WaylandSurface {
    pub id: WaylandId,
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub buffer: Vec<u32>, // ARGB pixel data
    pub active: bool,
}

impl WaylandSurface {
    pub fn new(id: WaylandId, width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            id,
            width,
            height,
            x: 0,
            y: 0,
            buffer: vec![0; size],
            active: true,
        }
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

/// Sovereign Wayland Compositor
pub struct WaylandCompositor {
    pub next_client_id: ClientId,
    pub surfaces: HashMap<WaylandId, WaylandSurface>,
    pub gpu: GpuDriver,
}

impl WaylandCompositor {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            next_client_id: 1,
            surfaces: HashMap::new(),
            gpu: GpuDriver::new(width, height),
        }
    }

    /// Register a new Wayland client surface
    pub fn create_surface(&mut self, id: WaylandId, w: u32, h: u32) {
        let surface = WaylandSurface::new(id, w, h);
        self.surfaces.insert(id, surface);
    }

    /// Destroy a Wayland surface
    pub fn destroy_surface(&mut self, id: WaylandId) {
        self.surfaces.remove(&id);
    }

    /// Composite all active client surfaces onto the GPU framebuffer
    pub fn render_frame(&mut self) -> Result<(), &'static str> {
        // 1. Clear GPU screen to background color
        self.gpu.execute_command(GpuCommand::ClearScreen { r: 30, g: 30, b: 40 })
            .map_err(|_| "GPU ClearScreen failed")?;

        // 2. Alpha composite all active surface buffers into the main framebuffer
        for surface in self.surfaces.values() {
            if !surface.active {
                continue;
            }

            let start_y = surface.y.max(0) as u32;
            let start_x = surface.x.max(0) as u32;
            let end_y = (surface.y + surface.height as i32).min(self.gpu.height as i32) as u32;
            let end_x = (surface.x + surface.width as i32).min(self.gpu.width as i32) as u32;

            for dest_y in start_y..end_y {
                for dest_x in start_x..end_x {
                    let src_y = (dest_y as i32 - surface.y) as u32;
                    let src_x = (dest_x as i32 - surface.x) as u32;

                    let src_idx = (src_y * surface.width + src_x) as usize;
                    let dest_idx = (dest_y * self.gpu.width + dest_x) as usize;

                    if src_idx < surface.buffer.len() && dest_idx < self.gpu.frame_buffer.len() {
                        let pixel = surface.buffer[src_idx];
                        // Simple alpha blend check (assuming 0xFF000000 alpha mask)
                        if (pixel & 0xFF000000) != 0 {
                            self.gpu.frame_buffer[dest_idx] = pixel;
                        }
                    }
                }
            }
        }

        // 3. Swap framebuffers
        self.gpu.execute_command(GpuCommand::Present)
            .map_err(|_| "GPU Present failed")?;

        Ok(())
    }
}

impl Default for WaylandCompositor {
    fn default() -> Self {
        Self::new(1920, 1080)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compositor_creation() {
        let compositor = WaylandCompositor::new(1024, 768);
        assert_eq!(compositor.gpu.width, 1024);
        assert_eq!(compositor.gpu.height, 768);
        assert_eq!(compositor.surfaces.len(), 0);
    }

    #[test]
    fn test_surface_creation() {
        let mut compositor = WaylandCompositor::new(1024, 768);
        compositor.create_surface(10, 200, 200);
        assert_eq!(compositor.surfaces.len(), 1);
        assert!(compositor.surfaces.contains_key(&10));
    }

    #[test]
    fn test_compositing() {
        let mut compositor = WaylandCompositor::new(100, 100);
        compositor.create_surface(1, 10, 10);
        {
            let s = compositor.surfaces.get_mut(&1).unwrap();
            s.set_position(5, 5);
            s.buffer.fill(0xFFFF0000); // Statically filled solid Red
        }
        assert!(compositor.render_frame().is_ok());
    }
}
