//! SigmaOS Zenith Wayland Compositor — Core Protocol Types
//!
//! Sovereign Wayland compositor protocol implementation.
//! This module defines the core types, object registry, event system,
//! and display buffer management for the Zenith compositor.
//!
//! Inspired by:
//! - wlroots (compositor toolkit, DRM/KMS backend)
//! - Smithay (pure-Rust Wayland compositor library)
//! - Sway (i3-compatible Wayland compositor)
//! - River (tag-based Wayland compositor)
//! - macOS WindowServer (smooth animation, damage tracking)
//! - GNOME Mutter (smooth animations, accessibility)
//! - KDE Plasma (extensive widget system)
//! - COSMIC (multi-threaded tiling)
//!
//! Architecture:
//! - `WlDisplay` — top-level display object (singleton per compositor)
//! - `WlSurface` — client drawable surface with attached buffer
//! - `WlBuffer` — shared memory buffer (wl_shm) or DMA-buf
//! - `WlOutput` — physical display output (DRM/KMS plane)
//! - `WlSeat` — input device seat (keyboard, pointer, touch)
//! - `ZenithCompositor` — main compositor state machine
//! - Enhanced with GNOME accessibility, KDE customization, and COSMIC performance

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;
use std::format;

// ─── Wayland Object ID ─────────────────────────────────────────────────────────

/// Wayland protocol object ID (u32, client-assigned)
pub type WlObjectId = u32;

/// Next object ID allocator
pub struct WlIdAllocator {
    next: WlObjectId,
}

impl WlIdAllocator {
    pub fn new() -> Self { WlIdAllocator { next: 1 } }
    pub fn alloc(&mut self) -> WlObjectId {
        let id = self.next;
        self.next += 1;
        id
    }
}

// ─── Pixel Format ─────────────────────────────────────────────────────────────

/// Wayland SHM pixel formats (subset of DRM fourcc)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlShmFormat {
    /// ARGB 8888 — standard composited format
    Argb8888,
    /// XRGB 8888 — opaque format
    Xrgb8888,
    /// ABGR 8888
    Abgr8888,
    /// RGB 565 — lower memory footprint
    Rgb565,
}

impl WlShmFormat {
    pub fn bytes_per_pixel(&self) -> u32 {
        match self {
            WlShmFormat::Argb8888 | WlShmFormat::Xrgb8888 | WlShmFormat::Abgr8888 => 4,
            WlShmFormat::Rgb565 => 2,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            WlShmFormat::Argb8888 => "ARGB8888",
            WlShmFormat::Xrgb8888 => "XRGB8888",
            WlShmFormat::Abgr8888 => "ABGR8888",
            WlShmFormat::Rgb565 => "RGB565",
        }
    }
}

// ─── Geometry ─────────────────────────────────────────────────────────────────

/// Integer 2D point
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WlPoint { pub x: i32, pub y: i32 }

/// Integer 2D size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WlSize { pub width: u32, pub height: u32 }

/// Integer rectangle (position + size)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WlRect { pub pos: WlPoint, pub size: WlSize }

impl WlRect {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        WlRect { pos: WlPoint { x, y }, size: WlSize { width: w, height: h } }
    }

    /// Returns true if point is inside this rect
    pub fn contains(&self, p: WlPoint) -> bool {
        p.x >= self.pos.x
            && p.x < self.pos.x + self.size.width as i32
            && p.y >= self.pos.y
            && p.y < self.pos.y + self.size.height as i32
    }

    /// Returns true if this rect overlaps with another
    pub fn intersects(&self, other: &WlRect) -> bool {
        self.pos.x < other.pos.x + other.size.width as i32
            && self.pos.x + self.size.width as i32 > other.pos.x
            && self.pos.y < other.pos.y + other.size.height as i32
            && self.pos.y + self.size.height as i32 > other.pos.y
    }

    /// Area of this rectangle in pixels
    pub fn area(&self) -> u64 {
        self.size.width as u64 * self.size.height as u64
    }
}

// ─── WlBuffer ─────────────────────────────────────────────────────────────────

/// A Wayland shared memory buffer (wl_buffer object)
#[derive(Debug, Clone)]
pub struct WlBuffer {
    pub id: WlObjectId,
    /// Backing pixel data (row-major, stride-padded)
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub stride: u32,  // bytes per row
    pub format: WlShmFormat,
    /// Number of times this buffer is currently referenced by a surface
    pub ref_count: u32,
    /// Whether this buffer has been released back to the client
    pub released: bool,
}

impl WlBuffer {
    /// Allocate a new zeroed buffer
    pub fn new(id: WlObjectId, width: u32, height: u32, format: WlShmFormat) -> Self {
        let stride = width * format.bytes_per_pixel();
        let data_len = (stride * height) as usize;
        WlBuffer {
            id,
            data: vec![0u8; data_len],
            width,
            height,
            stride,
            format,
            ref_count: 0,
            released: false,
        }
    }

    /// Read a pixel at (x, y) as RGBA bytes
    pub fn read_pixel(&self, x: u32, y: u32) -> Option<[u8; 4]> {
        if x >= self.width || y >= self.height { return None; }
        let bpp = self.format.bytes_per_pixel() as usize;
        let off = (y as usize * self.stride as usize) + (x as usize * bpp);
        if off + 4 > self.data.len() { return None; }
        Some([self.data[off], self.data[off+1], self.data[off+2], self.data[off+3]])
    }

    /// Write a pixel at (x, y) from ARGB bytes
    pub fn write_pixel(&mut self, x: u32, y: u32, argb: [u8; 4]) -> bool {
        if x >= self.width || y >= self.height { return false; }
        let bpp = self.format.bytes_per_pixel() as usize;
        let off = (y as usize * self.stride as usize) + (x as usize * bpp);
        if off + 4 > self.data.len() { return false; }
        self.data[off..off+4].copy_from_slice(&argb);
        true
    }

    pub fn size_bytes(&self) -> usize { self.data.len() }
}

// ─── WlSurface ────────────────────────────────────────────────────────────────

/// Transform applied to surface contents
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WlTransform {
    Normal,
    Rot90,
    Rot180,
    Rot270,
    Flipped,
    FlippedRot90,
}

/// Damage tracking region (list of dirty rects for partial repaint)
#[derive(Debug, Clone)]
pub struct WlDamageRegion {
    pub rects: Vec<WlRect>,
}

impl WlDamageRegion {
    pub fn new() -> Self { WlDamageRegion { rects: Vec::new() } }

    pub fn add_damage(&mut self, rect: WlRect) {
        self.rects.push(rect);
    }

    pub fn clear(&mut self) { self.rects.clear(); }

    pub fn is_empty(&self) -> bool { self.rects.is_empty() }

    /// Total damaged area (may overlap — not de-duplicated for performance)
    pub fn total_area(&self) -> u64 {
        self.rects.iter().map(|r| r.area()).sum()
    }
}

/// A Wayland surface — the fundamental unit of rendering
#[derive(Debug, Clone)]
pub struct WlSurface {
    pub id: WlObjectId,
    /// Client PID that owns this surface
    pub client_pid: u32,
    /// Currently committed buffer (None = no content yet)
    pub current_buffer: Option<WlObjectId>,
    /// Pending buffer (to be committed on next wl_surface.commit)
    pub pending_buffer: Option<WlObjectId>,
    /// Surface position in compositor space
    pub position: WlPoint,
    /// Surface size (from buffer dimensions)
    pub size: WlSize,
    /// Damage region (dirty pixels since last repaint)
    pub damage: WlDamageRegion,
    /// Surface transform
    pub transform: WlTransform,
    /// Buffer scale factor (for HiDPI displays)
    pub scale: u32,
    /// Whether this surface is mapped (visible)
    pub mapped: bool,
    /// Z-order (lower = rendered first, higher = on top)
    pub z_order: i32,
    /// Optional title (for XDG toplevel surfaces)
    pub title: Option<String>,
    /// Optional app ID
    pub app_id: Option<String>,
}

impl WlSurface {
    pub fn new(id: WlObjectId, client_pid: u32) -> Self {
        WlSurface {
            id,
            client_pid,
            current_buffer: None,
            pending_buffer: None,
            position: WlPoint { x: 0, y: 0 },
            size: WlSize { width: 0, height: 0 },
            damage: WlDamageRegion::new(),
            transform: WlTransform::Normal,
            scale: 1,
            mapped: false,
            z_order: 0,
            title: None,
            app_id: None,
        }
    }

    /// Commit pending state to current
    pub fn commit(&mut self) {
        if self.pending_buffer.is_some() {
            self.current_buffer = self.pending_buffer.take();
        }
    }

    /// Returns the bounding rect of this surface in compositor space
    pub fn rect(&self) -> WlRect {
        WlRect::new(self.position.x, self.position.y, self.size.width, self.size.height)
    }
}

// ─── WlOutput ─────────────────────────────────────────────────────────────────

/// Display output mode
#[derive(Debug, Clone, Copy)]
pub struct WlOutputMode {
    pub width: u32,
    pub height: u32,
    /// Refresh rate in mHz (e.g. 60000 = 60Hz, 144000 = 144Hz)
    pub refresh_mhz: u32,
    pub preferred: bool,
}

/// Physical display output (DRM/KMS connector)
#[derive(Debug, Clone)]
pub struct WlOutput {
    pub id: WlObjectId,
    /// Physical width in mm
    pub physical_width_mm: u32,
    /// Physical height in mm
    pub physical_height_mm: u32,
    /// Current active mode
    pub current_mode: WlOutputMode,
    /// All supported modes
    pub modes: Vec<WlOutputMode>,
    /// Output name (e.g. "eDP-1", "HDMI-A-1")
    pub name: String,
    /// EDID description
    pub description: String,
    /// Output scale factor (1 = 100%, 2 = 200% HiDPI)
    pub scale: u32,
    /// Whether this output is enabled
    pub enabled: bool,
}

impl WlOutput {
    pub fn new_1080p(id: WlObjectId, name: &str) -> Self {
        WlOutput {
            id,
            physical_width_mm: 344,
            physical_height_mm: 194,
            current_mode: WlOutputMode { width: 1920, height: 1080, refresh_mhz: 60000, preferred: true },
            modes: vec![
                WlOutputMode { width: 1920, height: 1080, refresh_mhz: 60000, preferred: true },
                WlOutputMode { width: 1920, height: 1080, refresh_mhz: 144000, preferred: false },
                WlOutputMode { width: 1280, height: 720, refresh_mhz: 60000, preferred: false },
            ],
            name: String::from(name),
            description: format!("{}: 1920x1080 @60Hz", name),
            scale: 1,
            enabled: true,
        }
    }

    /// Returns display PPI (pixels per inch)
    pub fn ppi(&self) -> f32 {
        let diag_px = (
            (self.current_mode.width as f32).powi(2) +
            (self.current_mode.height as f32).powi(2)
        ).sqrt();
        let diag_mm = (
            (self.physical_width_mm as f32).powi(2) +
            (self.physical_height_mm as f32).powi(2)
        ).sqrt();
        let diag_in = diag_mm / 25.4;
        if diag_in > 0.0 { diag_px / diag_in } else { 0.0 }
    }
}

// ─── Input Events ─────────────────────────────────────────────────────────────

/// Wayland pointer (mouse/touchpad) event
#[derive(Debug, Clone)]
pub enum WlPointerEvent {
    /// Pointer entered a surface
    Enter { surface_id: WlObjectId, sx: f64, sy: f64 },
    /// Pointer left a surface
    Leave { surface_id: WlObjectId },
    /// Pointer moved within a surface
    Motion { sx: f64, sy: f64, time_ms: u32 },
    /// Button pressed or released
    Button { button: u32, pressed: bool, time_ms: u32 },
    /// Scroll axis
    Axis { axis: u32, value: f64, time_ms: u32 },
}

/// Wayland keyboard event
#[derive(Debug, Clone)]
pub enum WlKeyEvent {
    /// Key pressed or released
    Key { key: u32, pressed: bool, time_ms: u32 },
    /// Modifier state changed
    Modifiers { depressed: u32, latched: u32, locked: u32, group: u32 },
}

// ─── Zenith Compositor ────────────────────────────────────────────────────────

/// Zenith compositor statistics
#[derive(Debug, Clone, Default)]
pub struct ZenithStats {
    pub frames_rendered: u64,
    pub surfaces_created: u64,
    pub surfaces_destroyed: u64,
    pub buffers_allocated: u64,
    pub total_damage_area: u64,
    pub pointer_events: u64,
    pub key_events: u64,
    pub accessibility_events: u64, // GNOME-inspired screen reader events
    pub widget_events: u64, // KDE-inspired widget interactions
}

/// Window tiling layout (COSMIC/i3-inspired)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TilingLayout {
    Floating,
    Stack,
    Tabbed,
    HorizontalSplit,
    VerticalSplit,
    Grid,
}

/// Accessibility mode (GNOME-inspired)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessibilityMode {
    None,
    HighContrast,
    LargeText,
    ScreenReader,
    ReducedMotion,
}

/// Widget type (KDE-inspired)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidgetType {
    None,
    Panel,
    Dock,
    Launcher,
    Notification,
    SystemTray,
    Clock,
    StatusIndicator,
}

/// Zenith Wayland Compositor — main compositor state
pub struct ZenithCompositor {
    /// All surfaces (active + unmapped)
    pub surfaces: BTreeMap<WlObjectId, WlSurface>,
    /// All buffers
    pub buffers: BTreeMap<WlObjectId, WlBuffer>,
    /// All outputs
    pub outputs: BTreeMap<WlObjectId, WlOutput>,
    /// Object ID allocator
    id_alloc: WlIdAllocator,
    /// Currently focused surface (for keyboard events)
    pub keyboard_focus: Option<WlObjectId>,
    /// Currently pointer-entered surface
    pub pointer_focus: Option<WlObjectId>,
    /// Compositor statistics
    pub stats: ZenithStats,
    /// Current frame timestamp (ms)
    pub frame_time_ms: u64,
    /// Current tiling layout (COSMIC/i3-inspired)
    pub tiling_layout: TilingLayout,
    /// Accessibility mode (GNOME-inspired)
    pub accessibility_mode: AccessibilityMode,
    /// Active widgets (KDE-inspired)
    pub active_widgets: BTreeMap<WlObjectId, WidgetType>,
    /// Screen reader enabled (GNOME accessibility)
    pub screen_reader_enabled: bool,
    /// High contrast mode (GNOME accessibility)
    pub high_contrast_mode: bool,
    /// Animation speed multiplier (for reduced motion)
    pub animation_speed: f32,
}

impl ZenithCompositor {
    /// Create a new Zenith compositor instance
    pub fn new() -> Self {
        ZenithCompositor {
            surfaces: BTreeMap::new(),
            buffers: BTreeMap::new(),
            outputs: BTreeMap::new(),
            id_alloc: WlIdAllocator::new(),
            keyboard_focus: None,
            pointer_focus: None,
            stats: ZenithStats::default(),
            frame_time_ms: 0,
            tiling_layout: TilingLayout::Floating,
            accessibility_mode: AccessibilityMode::None,
            active_widgets: BTreeMap::new(),
            screen_reader_enabled: false,
            high_contrast_mode: false,
            animation_speed: 1.0,
        }
    }

    /// Add a physical output
    pub fn add_output(&mut self, output: WlOutput) -> WlObjectId {
        let id = output.id;
        self.outputs.insert(id, output);
        id
    }

    // ── Surface Management ────────────────────────────────────────────────────

    /// Create a new surface for the given client PID
    pub fn create_surface(&mut self, client_pid: u32) -> WlObjectId {
        let id = self.id_alloc.alloc();
        let surface = WlSurface::new(id, client_pid);
        self.surfaces.insert(id, surface);
        self.stats.surfaces_created += 1;
        id
    }

    /// Destroy a surface by ID
    pub fn destroy_surface(&mut self, id: WlObjectId) -> bool {
        if self.surfaces.remove(&id).is_some() {
            self.stats.surfaces_destroyed += 1;
            if self.keyboard_focus == Some(id) { self.keyboard_focus = None; }
            if self.pointer_focus == Some(id) { self.pointer_focus = None; }
            true
        } else {
            false
        }
    }

    /// Attach a buffer to a surface (pending, committed on next commit)
    pub fn surface_attach_buffer(&mut self, surface_id: WlObjectId, buffer_id: WlObjectId) -> bool {
        if let Some(surface) = self.surfaces.get_mut(&surface_id) {
            surface.pending_buffer = Some(buffer_id);
            true
        } else {
            false
        }
    }

    /// Commit a surface (applies pending buffer)
    pub fn surface_commit(&mut self, surface_id: WlObjectId) -> bool {
        if let Some(surface) = self.surfaces.get_mut(&surface_id) {
            // Apply pending buffer size
            if let Some(buf_id) = surface.pending_buffer {
                if let Some(buf) = self.buffers.get(&buf_id) {
                    surface.size = WlSize { width: buf.width, height: buf.height };
                }
            }
            surface.commit();
            surface.mapped = surface.current_buffer.is_some();
            true
        } else {
            false
        }
    }

    /// Add damage to a surface (marks pixels dirty)
    pub fn surface_damage(&mut self, surface_id: WlObjectId, rect: WlRect) -> bool {
        if let Some(surface) = self.surfaces.get_mut(&surface_id) {
            surface.damage.add_damage(rect);
            self.stats.total_damage_area += rect.area();
            true
        } else {
            false
        }
    }

    // ── Buffer Management ─────────────────────────────────────────────────────

    /// Allocate a new shared memory buffer
    pub fn create_buffer(
        &mut self,
        width: u32,
        height: u32,
        format: WlShmFormat,
    ) -> WlObjectId {
        let id = self.id_alloc.alloc();
        let buf = WlBuffer::new(id, width, height, format);
        self.buffers.insert(id, buf);
        self.stats.buffers_allocated += 1;
        id
    }

    /// Release a buffer back to the client (wl_buffer.release event)
    pub fn release_buffer(&mut self, buffer_id: WlObjectId) -> bool {
        if let Some(buf) = self.buffers.get_mut(&buffer_id) {
            buf.released = true;
            true
        } else {
            false
        }
    }

    // ── Frame Rendering ───────────────────────────────────────────────────────

    /// Render one frame — returns list of (surface_id, damage) that need repaint.
    ///
    /// In a real compositor, this triggers DRM/KMS scanout.
    pub fn render_frame(&mut self, time_ms: u64) -> Vec<(WlObjectId, WlDamageRegion)> {
        self.frame_time_ms = time_ms;
        self.stats.frames_rendered += 1;

        let mut dirty: Vec<(WlObjectId, WlDamageRegion)> = Vec::new();

        // Collect surfaces with damage, sorted by z_order
        let mut to_repaint: Vec<WlObjectId> = self.surfaces.iter()
            .filter(|(_, s)| s.mapped && !s.damage.is_empty())
            .map(|(id, _)| *id)
            .collect();
        to_repaint.sort_by_key(|id| self.surfaces.get(id).map(|s| s.z_order).unwrap_or(0));

        for id in to_repaint {
            if let Some(surface) = self.surfaces.get_mut(&id) {
                let damage = surface.damage.clone();
                surface.damage.clear();
                dirty.push((id, damage));
            }
        }

        dirty
    }

    // ── Input Dispatch ────────────────────────────────────────────────────────

    /// Dispatch a pointer event to the appropriate surface
    pub fn dispatch_pointer_event(&mut self, event: WlPointerEvent) {
        self.stats.pointer_events += 1;
        match &event {
            WlPointerEvent::Enter { surface_id, .. } => {
                self.pointer_focus = Some(*surface_id);
            }
            WlPointerEvent::Leave { .. } => {
                self.pointer_focus = None;
            }
            WlPointerEvent::Motion { sx, sy, .. } => {
                // Find which surface is under the cursor
                let pos = WlPoint { x: *sx as i32, y: *sy as i32 };
                self.pointer_focus = self.surfaces.iter()
                    .filter(|(_, s)| s.mapped && s.rect().contains(pos))
                    .max_by_key(|(_, s)| s.z_order)
                    .map(|(id, _)| *id);
            }
            _ => {}
        }
    }

    /// Dispatch a keyboard event to the focused surface
    pub fn dispatch_key_event(&mut self, _event: WlKeyEvent) -> Option<WlObjectId> {
        self.stats.key_events += 1;
        self.keyboard_focus
    }

    /// Set keyboard focus to a specific surface
    pub fn set_keyboard_focus(&mut self, surface_id: WlObjectId) -> bool {
        if self.surfaces.contains_key(&surface_id) {
            self.keyboard_focus = Some(surface_id);
            true
        } else {
            false
        }
    }

    // ── Status ────────────────────────────────────────────────────────────────

    /// Returns compositor status string
    pub fn status(&self) -> String {
        let mapped = self.surfaces.values().filter(|s| s.mapped).count();
        format!(
            "Zenith | {} surfaces ({} mapped) | {} buffers | {} outputs | {} frames | {}fps target | Layout: {:?} | A11y: {:?}",
            self.surfaces.len(),
            mapped,
            self.buffers.len(),
            self.outputs.len(),
            self.stats.frames_rendered,
            self.outputs.values().next().map(|o| o.current_mode.refresh_mhz / 1000).unwrap_or(60),
            self.tiling_layout,
            self.accessibility_mode
        )
    }

    // ========== GNOME Accessibility Features ==========

    /// Set accessibility mode (GNOME-inspired)
    pub fn set_accessibility_mode(&mut self, mode: AccessibilityMode) {
        self.accessibility_mode = mode;
        self.screen_reader_enabled = matches!(mode, AccessibilityMode::ScreenReader);
        self.high_contrast_mode = matches!(mode, AccessibilityMode::HighContrast);

        if matches!(mode, AccessibilityMode::ReducedMotion) {
            self.animation_speed = 0.0; // Disable animations
        } else {
            self.animation_speed = 1.0; // Normal speed
        }

        self.stats.accessibility_events += 1;
    }

    /// Toggle screen reader (GNOME Orca-inspired)
    pub fn toggle_screen_reader(&mut self) {
        self.screen_reader_enabled = !self.screen_reader_enabled;
        if self.screen_reader_enabled {
            self.accessibility_mode = AccessibilityMode::ScreenReader;
        } else if self.accessibility_mode == AccessibilityMode::ScreenReader {
            self.accessibility_mode = AccessibilityMode::None;
        }
        self.stats.accessibility_events += 1;
    }

    /// Toggle high contrast mode (GNOME accessibility)
    pub fn toggle_high_contrast(&mut self) {
        self.high_contrast_mode = !self.high_contrast_mode;
        if self.high_contrast_mode {
            self.accessibility_mode = AccessibilityMode::HighContrast;
        } else if self.accessibility_mode == AccessibilityMode::HighContrast {
            self.accessibility_mode = AccessibilityMode::None;
        }
        self.stats.accessibility_events += 1;
    }

    /// Set animation speed (for reduced motion)
    pub fn set_animation_speed(&mut self, speed: f32) {
        self.animation_speed = speed.clamp(0.0, 2.0);
        if self.animation_speed == 0.0 {
            self.accessibility_mode = AccessibilityMode::ReducedMotion;
        } else if self.accessibility_mode == AccessibilityMode::ReducedMotion {
            self.accessibility_mode = AccessibilityMode::None;
        }
    }

    // ========== KDE Widget System ==========

    /// Add a widget (KDE-inspired)
    pub fn add_widget(&mut self, surface_id: WlObjectId, widget_type: WidgetType) -> bool {
        if self.surfaces.contains_key(&surface_id) {
            self.active_widgets.insert(surface_id, widget_type);
            self.stats.widget_events += 1;
            true
        } else {
            false
        }
    }

    /// Remove a widget
    pub fn remove_widget(&mut self, surface_id: WlObjectId) -> bool {
        if self.active_widgets.remove(&surface_id).is_some() {
            self.stats.widget_events += 1;
            true
        } else {
            false
        }
    }

    /// Get widget type for a surface
    pub fn get_widget_type(&self, surface_id: WlObjectId) -> Option<WidgetType> {
        self.active_widgets.get(&surface_id).copied()
    }

    /// Get all widgets of a specific type
    pub fn get_widgets_by_type(&self, widget_type: WidgetType) -> Vec<WlObjectId> {
        self.active_widgets
            .iter()
            .filter(|(_, &wt)| wt == widget_type)
            .map(|(&id, _)| id)
            .collect()
    }

    // ========== COSMIC Tiling Features ==========

    /// Set tiling layout (COSMIC/i3-inspired)
    pub fn set_tiling_layout(&mut self, layout: TilingLayout) {
        self.tiling_layout = layout;
    }

    /// Get current tiling layout
    pub fn get_tiling_layout(&self) -> TilingLayout {
        self.tiling_layout
    }

    /// Auto-tile surfaces (COSMIC-inspired)
    pub fn auto_tile_surfaces(&mut self) {
        if self.tiling_layout == TilingLayout::Floating {
            return; // No auto-tiling in floating mode
        }

        let mut mapped_surfaces: Vec<_> = self.surfaces
            .iter()
            .filter(|(_, s)| s.mapped)
            .map(|(id, s)| (*id, s.size))
            .collect();

        let output_size = self.outputs.values()
            .next()
            .map(|o| (o.current_mode.width, o.current_mode.height))
            .unwrap_or((1920, 1080));

        match self.tiling_layout {
            TilingLayout::Grid => {
                let cols = ((mapped_surfaces.len() as f32).sqrt().ceil() as u32).max(1);
                let rows = ((mapped_surfaces.len() as f32) / cols as f32).ceil() as u32;

                let tile_width = output_size.0 / cols;
                let tile_height = output_size.1 / rows;

                for (i, (id, _)) in mapped_surfaces.iter_mut().enumerate() {
                    let col = (i as u32) % cols;
                    let row = (i as u32) / cols;

                    if let Some(surface) = self.surfaces.get_mut(id) {
                        surface.position = WlPoint {
                            x: (col * tile_width) as i32,
                            y: (row * tile_height) as i32,
                        };
                        surface.size = WlSize {
                            width: tile_width,
                            height: tile_height,
                        };
                    }
                }
            }
            TilingLayout::HorizontalSplit => {
                let tile_height = output_size.1 / mapped_surfaces.len() as u32;
                for (i, (id, _)) in mapped_surfaces.iter_mut().enumerate() {
                    if let Some(surface) = self.surfaces.get_mut(id) {
                        surface.position = WlPoint {
                            x: 0,
                            y: (i as u32 * tile_height) as i32,
                        };
                        surface.size = WlSize {
                            width: output_size.0,
                            height: tile_height,
                        };
                    }
                }
            }
            TilingLayout::VerticalSplit => {
                let tile_width = output_size.0 / mapped_surfaces.len() as u32;
                for (i, (id, _)) in mapped_surfaces.iter_mut().enumerate() {
                    if let Some(surface) = self.surfaces.get_mut(id) {
                        surface.position = WlPoint {
                            x: (i as u32 * tile_width) as i32,
                            y: 0,
                        };
                        surface.size = WlSize {
                            width: tile_width,
                            height: output_size.1,
                        };
                    }
                }
            }
            TilingLayout::Tabbed => {
                // Stack all windows in the same position
                for (id, _) in mapped_surfaces.iter_mut() {
                    if let Some(surface) = self.surfaces.get_mut(id) {
                        surface.position = WlPoint { x: 0, y: 0 };
                        surface.size = WlSize {
                            width: output_size.0,
                            height: output_size.1,
                        };
                    }
                }
            }
            _ => {} // Handle other layouts similarly
        }
    }

    /// Get compositor capabilities
    pub fn get_capabilities(&self) -> Vec<String> {
        let mut caps = vec![
            String::from("wayland"),
            String::from("drm"),
            String::from("damage_tracking"),
        ];

        if self.screen_reader_enabled {
            caps.push(String::from("screen_reader"));
        }
        if self.high_contrast_mode {
            caps.push(String::from("high_contrast"));
        }
        if self.tiling_layout != TilingLayout::Floating {
            caps.push(String::from("tiling"));
        }
        if !self.active_widgets.is_empty() {
            caps.push(String::from("widgets"));
        }

        caps
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod zenith_tests {
    use super::*;

    fn setup() -> ZenithCompositor {
        let mut comp = ZenithCompositor::new();
        let output = WlOutput::new_1080p(999, "eDP-1");
        comp.add_output(output);
        comp
    }

    #[test]
    fn test_create_surface() {
        let mut comp = setup();
        let id = comp.create_surface(1234);
        assert!(comp.surfaces.contains_key(&id));
        assert_eq!(comp.surfaces[&id].client_pid, 1234);
        assert_eq!(comp.stats.surfaces_created, 1);
    }

    #[test]
    fn test_create_and_commit_buffer() {
        let mut comp = setup();
        let surf_id = comp.create_surface(100);
        let buf_id = comp.create_buffer(800, 600, WlShmFormat::Argb8888);

        assert!(comp.surface_attach_buffer(surf_id, buf_id));
        assert!(comp.surface_commit(surf_id));

        let surf = &comp.surfaces[&surf_id];
        assert!(surf.mapped);
        assert_eq!(surf.size.width, 800);
        assert_eq!(surf.size.height, 600);
    }

    #[test]
    fn test_destroy_surface_clears_focus() {
        let mut comp = setup();
        let id = comp.create_surface(42);
        comp.keyboard_focus = Some(id);
        comp.pointer_focus = Some(id);
        assert!(comp.destroy_surface(id));
        assert_eq!(comp.keyboard_focus, None);
        assert_eq!(comp.pointer_focus, None);
    }

    #[test]
    fn test_damage_tracking() {
        let mut comp = setup();
        let surf_id = comp.create_surface(1);
        let damage_rect = WlRect::new(0, 0, 100, 100);
        comp.surface_damage(surf_id, damage_rect);
        assert!(!comp.surfaces[&surf_id].damage.is_empty());
        assert_eq!(comp.stats.total_damage_area, 10000);
    }

    #[test]
    fn test_render_frame_clears_damage() {
        let mut comp = setup();
        let surf_id = comp.create_surface(1);
        let buf_id = comp.create_buffer(400, 300, WlShmFormat::Xrgb8888);
        comp.surface_attach_buffer(surf_id, buf_id);
        comp.surface_commit(surf_id);
        comp.surface_damage(surf_id, WlRect::new(0, 0, 400, 300));

        let dirty = comp.render_frame(16);
        assert!(!dirty.is_empty());
        assert_eq!(comp.stats.frames_rendered, 1);
        // After render, damage should be cleared
        assert!(comp.surfaces[&surf_id].damage.is_empty());
    }

    #[test]
    fn test_pointer_event_updates_focus() {
        let mut comp = setup();
        let surf_id = comp.create_surface(99);
        let buf_id = comp.create_buffer(800, 600, WlShmFormat::Argb8888);
        comp.surface_attach_buffer(surf_id, buf_id);
        comp.surface_commit(surf_id);

        comp.dispatch_pointer_event(WlPointerEvent::Motion { sx: 400.0, sy: 300.0, time_ms: 100 });
        assert_eq!(comp.pointer_focus, Some(surf_id));
    }

    #[test]
    fn test_rect_contains() {
        let r = WlRect::new(10, 10, 100, 100);
        assert!(r.contains(WlPoint { x: 50, y: 50 }));
        assert!(r.contains(WlPoint { x: 10, y: 10 }));
        assert!(!r.contains(WlPoint { x: 9, y: 50 }));
        assert!(!r.contains(WlPoint { x: 110, y: 50 }));
    }

    #[test]
    fn test_rect_intersects() {
        let r1 = WlRect::new(0, 0, 100, 100);
        let r2 = WlRect::new(50, 50, 100, 100);
        let r3 = WlRect::new(200, 200, 50, 50);
        assert!(r1.intersects(&r2));
        assert!(!r1.intersects(&r3));
    }

    #[test]
    fn test_buffer_pixel_rw() {
        let mut buf = WlBuffer::new(1, 4, 4, WlShmFormat::Argb8888);
        assert!(buf.write_pixel(2, 2, [0xFF, 0x00, 0x80, 0x40]));
        let px = buf.read_pixel(2, 2).unwrap();
        assert_eq!(px, [0xFF, 0x00, 0x80, 0x40]);
    }

    #[test]
    fn test_output_ppi() {
        let output = WlOutput::new_1080p(1, "eDP-1");
        let ppi = output.ppi();
        // 15.6" FHD display ≈ 141 PPI
        assert!(ppi > 80.0 && ppi < 300.0, "PPI out of range: {}", ppi);
    }

    #[test]
    fn test_compositor_status() {
        let comp = setup();
        let status = comp.status();
        assert!(status.contains("Zenith"));
        assert!(status.contains("0 surfaces"));
    }

    #[test]
    fn test_accessibility_modes() {
        let mut comp = setup();

        // Test screen reader toggle
        comp.toggle_screen_reader();
        assert!(comp.screen_reader_enabled);
        assert_eq!(comp.accessibility_mode, AccessibilityMode::ScreenReader);

        comp.toggle_screen_reader();
        assert!(!comp.screen_reader_enabled);

        // Test high contrast toggle
        comp.toggle_high_contrast();
        assert!(comp.high_contrast_mode);
        assert_eq!(comp.accessibility_mode, AccessibilityMode::HighContrast);

        // Test reduced motion
        comp.set_accessibility_mode(AccessibilityMode::ReducedMotion);
        assert_eq!(comp.animation_speed, 0.0);

        comp.set_animation_speed(1.5);
        assert_eq!(comp.animation_speed, 1.5);
        assert_ne!(comp.accessibility_mode, AccessibilityMode::ReducedMotion);
    }

    #[test]
    fn test_widget_system() {
        let mut comp = setup();
        let surf_id = comp.create_surface(1);

        // Add various widgets
        assert!(comp.add_widget(surf_id, WidgetType::Panel));
        assert!(comp.add_widget(surf_id, WidgetType::Clock));

        // Check widget type
        assert_eq!(comp.get_widget_type(surf_id), Some(WidgetType::Clock)); // Last added wins

        // Get widgets by type
        let clock_widgets = comp.get_widgets_by_type(WidgetType::Clock);
        assert_eq!(clock_widgets.len(), 1);

        // Remove widget
        assert!(comp.remove_widget(surf_id));
        assert_eq!(comp.get_widget_type(surf_id), None);
    }

    #[test]
    fn test_tiling_layouts() {
        let mut comp = setup();

        // Test different tiling layouts
        comp.set_tiling_layout(TilingLayout::Grid);
        assert_eq!(comp.get_tiling_layout(), TilingLayout::Grid);

        comp.set_tiling_layout(TilingLayout::HorizontalSplit);
        assert_eq!(comp.get_tiling_layout(), TilingLayout::HorizontalSplit);

        comp.set_tiling_layout(TilingLayout::Floating);
        assert_eq!(comp.get_tiling_layout(), TilingLayout::Floating);
    }

    #[test]
    fn test_auto_tiling() {
        let mut comp = setup();
        let output = WlOutput::new_1080p(999, "eDP-1");
        comp.add_output(output);

        // Create multiple surfaces
        let surf1 = comp.create_surface(1);
        let surf2 = comp.create_surface(2);
        let surf3 = comp.create_surface(3);

        // Attach buffers and commit
        let buf1 = comp.create_buffer(800, 600, WlShmFormat::Argb8888);
        let buf2 = comp.create_buffer(800, 600, WlShmFormat::Argb8888);
        let buf3 = comp.create_buffer(800, 600, WlShmFormat::Argb8888);

        comp.surface_attach_buffer(surf1, buf1);
        comp.surface_commit(surf1);
        comp.surface_attach_buffer(surf2, buf2);
        comp.surface_commit(surf2);
        comp.surface_attach_buffer(surf3, buf3);
        comp.surface_commit(surf3);

        // Set grid layout and auto-tile
        comp.set_tiling_layout(TilingLayout::Grid);
        comp.auto_tile_surfaces();

        // Check that surfaces were tiled
        let s1 = comp.surfaces.get(&surf1).unwrap();
        let s2 = comp.surfaces.get(&surf2).unwrap();
        let s3 = comp.surfaces.get(&surf3).unwrap();

        // In grid layout, surfaces should be positioned differently
        assert!(s1.position != s2.position || s1.position != s3.position);
    }

    #[test]
    fn test_compositor_capabilities() {
        let mut comp = setup();

        let caps = comp.get_capabilities();
        assert!(caps.contains(&String::from("wayland")));
        assert!(caps.contains(&String::from("drm")));

        // Enable screen reader
        comp.toggle_screen_reader();
        let caps = comp.get_capabilities();
        assert!(caps.contains(&String::from("screen_reader")));

        // Enable tiling
        comp.set_tiling_layout(TilingLayout::Grid);
        let caps = comp.get_capabilities();
        assert!(caps.contains(&String::from("tiling")));

        // Add widget
        let surf_id = comp.create_surface(1);
        comp.add_widget(surf_id, WidgetType::Panel);
        let caps = comp.get_capabilities();
        assert!(caps.contains(&String::from("widgets")));
    }
}
