// SPDX-License-Identifier: MIT
// SigmaOS - wlr-layer-shell Protocol
// Inspired by wlr-layer-shell (layer-shell-unstable-v1.xml)
// Provides layer shell protocol for overlay rendering (HUD, bars, etc.)

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Layer layer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    Background,
    Bottom,
    Top,
    Overlay,
}

/// Layer shell anchor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    None,
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Layer surface exclusive zone
#[derive(Debug, Clone, Copy)]
pub struct ExclusiveZone {
    pub enabled: bool,
    pub size: u32,
}

/// Layer surface state
#[derive(Debug, Clone, Copy)]
pub struct LayerSurfaceState {
    pub layer: Layer,
    pub anchor: Anchor,
    pub exclusive_zone: ExclusiveZone,
    pub margin_top: u32,
    pub margin_bottom: u32,
    pub margin_left: u32,
    pub margin_right: u32,
    pub keyboard_interactivity: bool,
}

impl Default for LayerSurfaceState {
    fn default() -> Self {
        Self {
            layer: Layer::Overlay,
            anchor: Anchor::None,
            exclusive_zone: ExclusiveZone {
                enabled: false,
                size: 0,
            },
            margin_top: 0,
            margin_bottom: 0,
            margin_left: 0,
            margin_right: 0,
            keyboard_interactivity: false,
        }
    }
}

/// Layer surface
pub struct LayerSurface {
    pub id: u32,
    pub surface_id: u32, // Reference to Wayland surface
    pub state: LayerSurfaceState,
    pub configured: AtomicBool,
    pub committed: AtomicBool,
    pub closed: AtomicBool,
}

impl LayerSurface {
    pub fn new(id: u32, surface_id: u32) -> Self {
        Self {
            id,
            surface_id,
            state: LayerSurfaceState::default(),
            configured: AtomicBool::new(false),
            committed: AtomicBool::new(false),
            closed: AtomicBool::new(false),
        }
    }

    /// Set layer
    pub fn set_layer(&mut self, layer: Layer) {
        self.state.layer = layer;
    }

    /// Set anchor
    pub fn set_anchor(&mut self, anchor: Anchor) {
        self.state.anchor = anchor;
    }

    /// Set exclusive zone
    pub fn set_exclusive_zone(&mut self, enabled: bool, size: u32) {
        self.state.exclusive_zone = ExclusiveZone { enabled, size };
    }

    /// Set margin
    pub fn set_margin(&mut self, top: u32, bottom: u32, left: u32, right: u32) {
        self.state.margin_top = top;
        self.state.margin_bottom = bottom;
        self.state.margin_left = left;
        self.state.margin_right = right;
    }

    /// Set keyboard interactivity
    pub fn set_keyboard_interactivity(&mut self, enabled: bool) {
        self.state.keyboard_interactivity = enabled;
    }

    /// Configure surface
    pub fn configure(&self) {
        self.configured.store(true, Ordering::Relaxed);
    }

    /// Commit surface
    pub fn commit(&self) {
        self.committed.store(true, Ordering::Relaxed);
    }

    /// Close surface
    pub fn close(&self) {
        self.closed.store(true, Ordering::Relaxed);
    }

    /// Is configured
    pub fn is_configured(&self) -> bool {
        self.configured.load(Ordering::Relaxed)
    }

    /// Is committed
    pub fn is_committed(&self) -> bool {
        self.committed.load(Ordering::Relaxed)
    }

    /// Is closed
    pub fn is_closed(&self) -> bool {
        self.closed.load(Ordering::Relaxed)
    }
}

/// Layer shell manager
pub struct LayerShellManager {
    pub surfaces: BTreeMap<u32, LayerSurface>,
    pub next_surface_id: AtomicU32,
    pub background_layer: Vec<u32>,
    pub bottom_layer: Vec<u32>,
    pub top_layer: Vec<u32>,
    pub overlay_layer: Vec<u32>,
}

impl LayerShellManager {
    pub fn new() -> Self {
        Self {
            surfaces: BTreeMap::new(),
            next_surface_id: AtomicU32::new(1),
            background_layer: Vec::new(),
            bottom_layer: Vec::new(),
            top_layer: Vec::new(),
            overlay_layer: Vec::new(),
        }
    }

    /// Create layer surface
    pub fn create_layer_surface(&mut self, surface_id: u32, layer: Layer) -> u32 {
        let id = self.next_surface_id.fetch_add(1, Ordering::Relaxed);
        let mut layer_surface = LayerSurface::new(id, surface_id);
        layer_surface.set_layer(layer);
        self.surfaces.insert(id, layer_surface);

        // Add to appropriate layer
        match layer {
            Layer::Background => self.background_layer.push(id),
            Layer::Bottom => self.bottom_layer.push(id),
            Layer::Top => self.top_layer.push(id),
            Layer::Overlay => self.overlay_layer.push(id),
        }

        id
    }

    /// Get layer surface
    pub fn get_layer_surface(&self, id: u32) -> Option<&LayerSurface> {
        self.surfaces.get(&id)
    }

    /// Get mutable layer surface
    pub fn get_layer_surface_mut(&mut self, id: u32) -> Option<&mut LayerSurface> {
        self.surfaces.get_mut(&id)
    }

    /// Destroy layer surface
    pub fn destroy_layer_surface(&mut self, id: u32) -> Result<(), &'static str> {
        let surface = self.surfaces.get(&id).ok_or("Surface not found")?;

        // Remove from layer
        match surface.state.layer {
            Layer::Background => self.background_layer.retain(|&x| x != id),
            Layer::Bottom => self.bottom_layer.retain(|&x| x != id),
            Layer::Top => self.top_layer.retain(|&x| x != id),
            Layer::Overlay => self.overlay_layer.retain(|&x| x != id),
        }

        self.surfaces.remove(&id).ok_or("Surface not found")?;
        Ok(())
    }

    /// Get surfaces in layer
    pub fn get_layer_surfaces(&self, layer: Layer) -> Vec<&LayerSurface> {
        let ids = match layer {
            Layer::Background => &self.background_layer,
            Layer::Bottom => &self.bottom_layer,
            Layer::Top => &self.top_layer,
            Layer::Overlay => &self.overlay_layer,
        };

        ids.iter()
            .filter_map(|&id| self.surfaces.get(&id))
            .collect()
    }

    /// Configure all surfaces in layer
    pub fn configure_layer(&self, layer: Layer) {
        for surface in self.get_layer_surfaces(layer) {
            surface.configure();
        }
    }

    /// Commit all surfaces in layer
    pub fn commit_layer(&self, layer: Layer) {
        for surface in self.get_layer_surfaces(layer) {
            surface.commit();
        }
    }
}

impl Default for LayerShellManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global layer shell manager
static mut LAYER_SHELL_MANAGER: Option<LayerShellManager> = None;

/// Initialize layer shell manager
pub fn init_layer_shell_manager() {
    unsafe {
        LAYER_SHELL_MANAGER = Some(LayerShellManager::new());
    }
}

/// Get global layer shell manager
pub fn get_layer_shell_manager() -> Option<&'static mut LayerShellManager> {
    unsafe { LAYER_SHELL_MANAGER.as_mut() }
}

/// Create HUD overlay surface (for gaming HUD)
pub fn create_hud_overlay(surface_id: u32) -> u32 {
    if let Some(manager) = get_layer_shell_manager() {
        let mut id = manager.create_layer_surface(surface_id, Layer::Overlay);
        if let Some(surface) = manager.get_layer_surface_mut(id) {
            surface.set_anchor(Anchor::TopRight);
            surface.set_exclusive_zone(false, 0);
            surface.set_keyboard_interactivity(false);
        }
        id
    } else {
        0
    }
}

/// Create status bar surface
pub fn create_status_bar(surface_id: u32) -> u32 {
    if let Some(manager) = get_layer_shell_manager() {
        let mut id = manager.create_layer_surface(surface_id, Layer::Top);
        if let Some(surface) = manager.get_layer_surface_mut(id) {
            surface.set_anchor(Anchor::Top);
            surface.set_exclusive_zone(true, 30); // 30px height
            surface.set_keyboard_interactivity(false);
        }
        id
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_surface_init() {
        let surface = LayerSurface::new(1, 100);
        assert_eq!(surface.id, 1);
        assert_eq!(surface.surface_id, 100);
    }

    #[test]
    fn test_set_layer() {
        let mut surface = LayerSurface::new(1, 100);
        surface.set_layer(Layer::Overlay);
        assert_eq!(surface.state.layer, Layer::Overlay);
    }

    #[test]
    fn test_set_anchor() {
        let mut surface = LayerSurface::new(1, 100);
        surface.set_anchor(Anchor::TopRight);
        assert_eq!(surface.state.anchor, Anchor::TopRight);
    }

    #[test]
    fn test_set_exclusive_zone() {
        let mut surface = LayerSurface::new(1, 100);
        surface.set_exclusive_zone(true, 30);
        assert!(surface.state.exclusive_zone.enabled);
        assert_eq!(surface.state.exclusive_zone.size, 30);
    }

    #[test]
    fn test_layer_shell_manager() {
        let mut manager = LayerShellManager::new();
        let id = manager.create_layer_surface(100, Layer::Overlay);
        assert!(manager.get_layer_surface(id).is_some());
    }

    #[test]
    fn test_get_layer_surfaces() {
        let mut manager = LayerShellManager::new();
        let id1 = manager.create_layer_surface(100, Layer::Overlay);
        let id2 = manager.create_layer_surface(101, Layer::Overlay);
        let surfaces = manager.get_layer_surfaces(Layer::Overlay);
        assert_eq!(surfaces.len(), 2);
    }

    #[test]
    fn test_destroy_layer_surface() {
        let mut manager = LayerShellManager::new();
        let id = manager.create_layer_surface(100, Layer::Overlay);
        manager.destroy_layer_surface(id).unwrap();
        assert!(manager.get_layer_surface(id).is_none());
    }

    #[test]
    fn test_create_hud_overlay() {
        init_layer_shell_manager();
        let id = create_hud_overlay(100);
        assert!(id > 0);
        if let Some(manager) = get_layer_shell_manager() {
            let surface = manager.get_layer_surface(id).unwrap();
            assert_eq!(surface.state.layer, Layer::Overlay);
            assert_eq!(surface.state.anchor, Anchor::TopRight);
        }
    }

    #[test]
    fn test_create_status_bar() {
        init_layer_shell_manager();
        let id = create_status_bar(100);
        assert!(id > 0);
        if let Some(manager) = get_layer_shell_manager() {
            let surface = manager.get_layer_surface(id).unwrap();
            assert_eq!(surface.state.layer, Layer::Top);
            assert_eq!(surface.state.anchor, Anchor::Top);
            assert!(surface.state.exclusive_zone.enabled);
        }
    }
}
