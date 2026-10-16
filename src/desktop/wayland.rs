// SPDX-License-Identifier: MIT
// SigmaOS - Wayland Protocol Core
// Inspired by Wayland protocol (wayland.xml)
// Provides core Wayland protocol implementation for display server

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Wayland object type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaylandObjectType {
    Display,
    Registry,
    Compositor,
    ShmPool,
    Buffer,
    Surface,
    Seat,
    Pointer,
    Keyboard,
    Touch,
    Output,
    Shell,
    ShellSurface,
    LayerShell,
    Unknown,
}

/// Wayland object
#[derive(Debug, Clone)]
pub struct WaylandObject {
    pub id: u32,
    pub object_type: WaylandObjectType,
    pub version: u32,
    pub interface: String,
}

/// Wayland message
#[derive(Debug, Clone)]
pub struct WaylandMessage {
    pub sender_id: u32,
    pub opcode: u16,
    pub args: Vec<WaylandArg>,
}

/// Wayland argument
#[derive(Debug, Clone)]
pub enum WaylandArg {
    Int(i32),
    Uint(u32),
    Fixed(i32),
    String(String),
    Object(u32),
    NewId(u32),
    Array(Vec<u8>),
    Fd(i32),
}

/// Wayland display (core display object)
pub struct WaylandDisplay {
    pub id: u32,
    pub objects: BTreeMap<u32, WaylandObject>,
    pub next_id: AtomicU32,
    pub active: AtomicBool,
}

impl WaylandDisplay {
    pub fn new() -> Self {
        let display = Self {
            id: 1,
            objects: BTreeMap::new(),
            next_id: AtomicU32::new(2),
            active: AtomicBool::new(false),
        };

        display
    }

    /// Initialize display
    pub fn init(&mut self) -> Result<(), &'static str> {
        self.active.store(true, Ordering::Relaxed);

        // Add display object
        self.objects.insert(
            1,
            WaylandObject {
                id: 1,
                object_type: WaylandObjectType::Display,
                version: 1,
                interface: "wl_display".to_string(),
            },
        );

        Ok(())
    }

    /// Create new object
    pub fn create_object(
        &mut self,
        object_type: WaylandObjectType,
        interface: &str,
        version: u32,
    ) -> u32 {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let object = WaylandObject {
            id,
            object_type,
            version,
            interface: interface.to_string(),
        };
        self.objects.insert(id, object);
        id
    }

    /// Get object by ID
    pub fn get_object(&self, id: u32) -> Option<&WaylandObject> {
        self.objects.get(&id)
    }

    /// Delete object
    pub fn delete_object(&mut self, id: u32) -> Result<(), &'static str> {
        self.objects.remove(&id).ok_or("Object not found")?;
        Ok(())
    }

    /// Send message
    pub fn send_message(&self, message: WaylandMessage) -> Result<(), &'static str> {
        // In real implementation, would send to client
        Ok(())
    }

    /// Receive message
    pub fn receive_message(&mut self, message: WaylandMessage) -> Result<(), &'static str> {
        // In real implementation, would process message
        Ok(())
    }

    /// Get registry
    pub fn get_registry(&self) -> Option<&WaylandObject> {
        self.objects
            .values()
            .find(|o| o.object_type == WaylandObjectType::Registry)
    }

    /// Get compositor
    pub fn get_compositor(&self) -> Option<&WaylandObject> {
        self.objects
            .values()
            .find(|o| o.object_type == WaylandObjectType::Compositor)
    }

    /// Get seat
    pub fn get_seat(&self) -> Option<&WaylandObject> {
        self.objects
            .values()
            .find(|o| o.object_type == WaylandObjectType::Seat)
    }

    /// Get output
    pub fn get_output(&self) -> Option<&WaylandObject> {
        self.objects
            .values()
            .find(|o| o.object_type == WaylandObjectType::Output)
    }

    /// Get shell
    pub fn get_shell(&self) -> Option<&WaylandObject> {
        self.objects
            .values()
            .find(|o| o.object_type == WaylandObjectType::Shell)
    }
}

impl Default for WaylandDisplay {
    fn default() -> Self {
        Self::new()
    }
}

/// Wayland compositor
pub struct WaylandCompositor {
    pub display: WaylandDisplay,
    pub surfaces: BTreeMap<u32, WaylandSurface>,
    pub next_surface_id: AtomicU32,
}

impl WaylandCompositor {
    pub fn new() -> Self {
        Self {
            display: WaylandDisplay::new(),
            surfaces: BTreeMap::new(),
            next_surface_id: AtomicU32::new(1),
        }
    }

    /// Initialize compositor
    pub fn init(&mut self) -> Result<(), &'static str> {
        self.display.init()?;

        // Create compositor object
        self.display
            .create_object(WaylandObjectType::Compositor, "wl_compositor", 4);

        Ok(())
    }

    /// Create surface
    pub fn create_surface(&mut self) -> u32 {
        let id = self.next_surface_id.fetch_add(1, Ordering::Relaxed);
        let surface = WaylandSurface {
            id,
            buffers: Vec::new(),
            damage: Vec::new(),
            frame_callbacks: Vec::new(),
            pending_state: None,
            active: true,
        };
        self.surfaces.insert(id, surface);
        id
    }

    /// Get surface
    pub fn get_surface(&self, id: u32) -> Option<&WaylandSurface> {
        self.surfaces.get(&id)
    }

    /// Get mutable surface
    pub fn get_surface_mut(&mut self, id: u32) -> Option<&mut WaylandSurface> {
        self.surfaces.get_mut(&id)
    }

    /// Destroy surface
    pub fn destroy_surface(&mut self, id: u32) -> Result<(), &'static str> {
        self.surfaces.remove(&id).ok_or("Surface not found")?;
        Ok(())
    }
}

impl Default for WaylandCompositor {
    fn default() -> Self {
        Self::new()
    }
}

/// Wayland surface
#[derive(Debug, Clone)]
pub struct WaylandSurface {
    pub id: u32,
    pub buffers: Vec<u32>,                 // Buffer IDs
    pub damage: Vec<(i32, i32, i32, i32)>, // Damage rectangles (x, y, width, height)
    pub frame_callbacks: Vec<u32>,         // Callback IDs
    pub pending_state: Option<SurfaceState>,
    pub active: bool,
}

/// Surface state
#[derive(Debug, Clone, Copy)]
pub struct SurfaceState {
    pub buffer: Option<u32>,
    pub opaque_region: Option<u32>,
    pub input_region: Option<u32>,
    pub scale: f32,
    pub transform: u32,
}

/// Wayland seat (input device)
pub struct WaylandSeat {
    pub id: u32,
    pub pointer: Option<WaylandPointer>,
    pub keyboard: Option<WaylandKeyboard>,
    pub touch: Option<WaylandTouch>,
    pub capabilities: u32,
}

impl WaylandSeat {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            pointer: None,
            keyboard: None,
            touch: None,
            capabilities: 0,
        }
    }

    /// Add pointer
    pub fn add_pointer(&mut self, pointer: WaylandPointer) {
        self.pointer = Some(pointer);
        self.capabilities |= 1; // Pointer capability
    }

    /// Add keyboard
    pub fn add_keyboard(&mut self, keyboard: WaylandKeyboard) {
        self.keyboard = Some(keyboard);
        self.capabilities |= 2; // Keyboard capability
    }

    /// Add touch
    pub fn add_touch(&mut self, touch: WaylandTouch) {
        self.touch = Some(touch);
        self.capabilities |= 4; // Touch capability
    }
}

/// Wayland pointer
#[derive(Debug, Clone)]
pub struct WaylandPointer {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub buttons: u32,
}

/// Wayland keyboard
#[derive(Debug, Clone)]
pub struct WaylandKeyboard {
    pub id: u32,
    pub modifiers: u32,
    pub keymap: Option<u32>,
}

/// Wayland touch
#[derive(Debug, Clone)]
pub struct WaylandTouch {
    pub id: u32,
    pub points: Vec<(u32, f32, f32)>, // (touch_id, x, y)
}

/// Global Wayland compositor
static mut WAYLAND_COMPOSITOR: Option<WaylandCompositor> = None;

/// Initialize Wayland compositor
pub fn init_wayland_compositor() {
    unsafe {
        WAYLAND_COMPOSITOR = Some(WaylandCompositor::new());
    }
}

/// Get global Wayland compositor
pub fn get_wayland_compositor() -> Option<&'static mut WaylandCompositor> {
    unsafe { WAYLAND_COMPOSITOR.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wayland_display_init() {
        let mut display = WaylandDisplay::new();
        display.init().unwrap();
        assert!(display.active.load(Ordering::Relaxed));
        assert!(display.get_object(1).is_some());
    }

    #[test]
    fn test_create_object() {
        let mut display = WaylandDisplay::new();
        display.init().unwrap();
        let id = display.create_object(WaylandObjectType::Surface, "wl_surface", 1);
        assert!(display.get_object(id).is_some());
    }

    #[test]
    fn test_delete_object() {
        let mut display = WaylandDisplay::new();
        display.init().unwrap();
        let id = display.create_object(WaylandObjectType::Surface, "wl_surface", 1);
        display.delete_object(id).unwrap();
        assert!(display.get_object(id).is_none());
    }

    #[test]
    fn test_wayland_compositor_init() {
        let mut compositor = WaylandCompositor::new();
        compositor.init().unwrap();
        assert!(compositor.display.active.load(Ordering::Relaxed));
    }

    #[test]
    fn test_create_surface() {
        let mut compositor = WaylandCompositor::new();
        compositor.init().unwrap();
        let id = compositor.create_surface();
        assert!(compositor.get_surface(id).is_some());
    }

    #[test]
    fn test_destroy_surface() {
        let mut compositor = WaylandCompositor::new();
        compositor.init().unwrap();
        let id = compositor.create_surface();
        compositor.destroy_surface(id).unwrap();
        assert!(compositor.get_surface(id).is_none());
    }

    #[test]
    fn test_wayland_seat() {
        let mut seat = WaylandSeat::new(1);
        let pointer = WaylandPointer {
            id: 2,
            x: 100.0,
            y: 200.0,
            buttons: 0,
        };
        seat.add_pointer(pointer);
        assert!(seat.pointer.is_some());
        assert_eq!(seat.capabilities, 1);
    }

    #[test]
    fn test_wayland_seat_keyboard() {
        let mut seat = WaylandSeat::new(1);
        let keyboard = WaylandKeyboard {
            id: 2,
            modifiers: 0,
            keymap: None,
        };
        seat.add_keyboard(keyboard);
        assert!(seat.keyboard.is_some());
        assert_eq!(seat.capabilities, 2);
    }
}
