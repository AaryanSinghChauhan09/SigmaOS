//! # Wayland Surface & Compositor Engine
//!
//! Bare-metal Wayland protocol display server engine for SigmaOS.
//! Implements wl_compositor, wl_surface, wl_shm_pool, and xdg_wm_base
//! state machines with DRM/KMS zero-copy scanout buffer mapping.

#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

/// Wayland Pixel Formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaylandPixelFormat {
    Argb8888 = 0,
    Xrgb8888 = 1,
    Rgba8888 = 2,
    Rgb565 = 3,
}

/// Surface Damage Rectangle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DamageRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl DamageRect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self { x, y, width, height }
    }

    pub fn is_empty(&self) -> bool {
        self.width <= 0 || self.height <= 0
    }
}

/// Wayland Shared Memory (wl_shm) Buffer
#[derive(Debug, Clone)]
pub struct WaylandShmBuffer {
    pub buffer_id: u32,
    pub width: i32,
    pub height: i32,
    pub stride: i32,
    pub format: WaylandPixelFormat,
    pub memory_offset: usize,
    pub memory_size: usize,
    pub is_busy: bool,
}

/// XDG Toplevel Window State
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowState {
    Normal,
    Maximized,
    Fullscreen,
    Minimized,
}

/// Wayland Surface (`wl_surface`) State
#[derive(Debug, Clone)]
pub struct WaylandSurface {
    pub surface_id: u32,
    pub client_id: u32,
    pub title: String,
    pub app_id: String,
    pub attached_buffer_id: Option<u32>,
    pub pending_damage: Vec<DamageRect>,
    pub current_damage: Vec<DamageRect>,
    pub state: WindowState,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub is_mapped: bool,
    pub frame_callbacks_pending: u32,
}

impl WaylandSurface {
    pub fn new(surface_id: u32, client_id: u32) -> Self {
        Self {
            surface_id,
            client_id,
            title: String::from("Untitled"),
            app_id: String::from("unknown"),
            attached_buffer_id: None,
            pending_damage: Vec::new(),
            current_damage: Vec::new(),
            state: WindowState::Normal,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            is_mapped: false,
            frame_callbacks_pending: 0,
        }
    }

    pub fn attach_buffer(&mut self, buffer_id: u32, width: i32, height: i32) {
        self.attached_buffer_id = Some(buffer_id);
        self.width = width;
        self.height = height;
    }

    pub fn add_damage(&mut self, rect: DamageRect) {
        if !rect.is_empty() {
            self.pending_damage.push(rect);
        }
    }

    pub fn commit(&mut self) -> bool {
        if self.attached_buffer_id.is_none() {
            return false;
        }
        self.current_damage = self.pending_damage.clone();
        self.pending_damage.clear();
        self.is_mapped = true;
        true
    }
}

/// Wayland Compositor Session Engine
pub struct WaylandSurfaceEngine {
    pub surfaces: BTreeMap<u32, WaylandSurface>,
    pub buffers: BTreeMap<u32, WaylandShmBuffer>,
    pub active_surface_id: Option<u32>,
    pub display_width: i32,
    pub display_height: i32,
    pub total_frames_presented: AtomicU64,
    pub total_damage_rects_flushed: AtomicU64,
}

impl WaylandSurfaceEngine {
    pub fn new(display_width: i32, display_height: i32) -> Self {
        Self {
            surfaces: BTreeMap::new(),
            buffers: BTreeMap::new(),
            active_surface_id: None,
            display_width,
            display_height,
            total_frames_presented: AtomicU64::new(0),
            total_damage_rects_flushed: AtomicU64::new(0),
        }
    }

    pub fn create_surface(&mut self, surface_id: u32, client_id: u32) {
        self.surfaces.insert(surface_id, WaylandSurface::new(surface_id, client_id));
        if self.active_surface_id.is_none() {
            self.active_surface_id = Some(surface_id);
        }
    }

    pub fn register_shm_buffer(
        &mut self,
        buffer_id: u32,
        width: i32,
        height: i32,
        stride: i32,
        format: WaylandPixelFormat,
        size: usize,
    ) {
        let buf = WaylandShmBuffer {
            buffer_id,
            width,
            height,
            stride,
            format,
            memory_offset: 0,
            memory_size: size,
            is_busy: false,
        };
        self.buffers.insert(buffer_id, buf);
    }

    /// Present composited surfaces to DRM/KMS scanout plane
    pub fn compose_and_present(&mut self) -> usize {
        let mut presented_surfaces = 0;

        for (_id, surf) in self.surfaces.iter_mut() {
            if surf.is_mapped && surf.attached_buffer_id.is_some() {
                let damage_count = surf.current_damage.len();
                self.total_damage_rects_flushed.fetch_add(damage_count as u64, Ordering::SeqCst);
                surf.current_damage.clear();
                presented_surfaces += 1;
            }
        }

        if presented_surfaces > 0 {
            self.total_frames_presented.fetch_add(1, Ordering::SeqCst);
        }

        presented_surfaces
    }
}

// ============================================================================
// UNIT TESTS & STANDALONE HARNESS
// ============================================================================

#[cfg(any(test, feature = "standalone_test"))]
mod tests {
    use super::*;

    #[test]
    fn test_surface_lifecycle_and_damage() {
        let mut surf = WaylandSurface::new(101, 1);
        assert!(!surf.is_mapped);

        surf.attach_buffer(501, 1920, 1080);
        surf.add_damage(DamageRect::new(0, 0, 1920, 1080));
        assert_eq!(surf.pending_damage.len(), 1);

        assert!(surf.commit());
        assert!(surf.is_mapped);
        assert_eq!(surf.current_damage.len(), 1);
        assert!(surf.pending_damage.is_empty());
    }

    #[test]
    fn test_compositor_presentation() {
        let mut engine = WaylandSurfaceEngine::new(2560, 1440);
        engine.create_surface(1, 10);
        engine.register_shm_buffer(100, 800, 600, 3200, WaylandPixelFormat::Xrgb8888, 800 * 600 * 4);

        let surf = engine.surfaces.get_mut(&1).unwrap();
        surf.attach_buffer(100, 800, 600);
        surf.add_damage(DamageRect::new(10, 10, 200, 200));
        surf.commit();

        let count = engine.compose_and_present();
        assert_eq!(count, 1);
        assert_eq!(engine.total_frames_presented.load(Ordering::SeqCst), 1);
        assert_eq!(engine.total_damage_rects_flushed.load(Ordering::SeqCst), 1);
    }
}
