//! # XWayland X11 Compatibility Server & Bridge
//!
//! Provides rootless X11 application execution within SigmaOS's Wayland compositor.
//! Bridges traditional X11 protocol calls to modern Wayland xdg-shell surfaces.
//!
//! ## Architecture
//! - Listens on Unix domain socket `/tmp/.X11-unix/X0` and abstract socket `@/tmp/.X11-unix/X0`
//! - Intercepts X11 window creation requests (CreateWindow, MapWindow)
//! - Maps each top-level X11 window to a native Wayland `wl_surface` and `xdg_toplevel`
//! - Bi-directional clipboard/selection proxying between X11 CLIPBOARD and Wayland `wl_data_device`
//! - Bridges X11 keyboard events, pointer motion, and button clicks to Wayland seats

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Standard X11 Request Opcode
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X11Opcode {
    CreateWindow = 1,
    ChangeWindowAttributes = 2,
    GetWindowAttributes = 3,
    DestroyWindow = 4,
    MapWindow = 8,
    UnmapWindow = 10,
    ConfigureWindow = 12,
    CreatePixmap = 53,
    FreePixmap = 54,
    CreateGC = 55,
    FreeGC = 60,
    PutImage = 72,
    GetImage = 73,
    CreateColormap = 78,
    SendEvent = 25,
    GrabPointer = 26,
    UngrabPointer = 27,
    SetSelectionOwner = 22,
    GetSelectionOwner = 23,
    ConvertSelection = 24,
}

/// X11 Window ID
pub type X11WindowId = u32;

/// Wayland Surface ID
pub type WaylandSurfaceId = u64;

/// X11 Window State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Unmapped,
    Mapped,
    Minimized,
    Fullscreen,
}

/// Managed X11 Window with mapped Wayland Surface
#[derive(Debug, Clone)]
pub struct X11Window {
    pub window_id: X11WindowId,
    pub parent_id: Option<X11WindowId>,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub state: WindowState,
    pub wayland_surface_id: Option<WaylandSurfaceId>,
    pub override_redirect: bool, // e.g. menus, tooltips
}

/// XWayland Server Instance
pub struct XWaylandServer {
    pub display_number: u32,
    pub socket_path: String,
    pub windows: BTreeMap<X11WindowId, X11Window>,
    pub next_surface_id: WaylandSurfaceId,
    pub active_selection_owner: Option<X11WindowId>,
    pub clipboard_buffer: Vec<u8>,
}

impl XWaylandServer {
    /// Create a new XWayland server on display :0.
    pub fn new(display_number: u32) -> Self {
        Self {
            display_number,
            socket_path: format!("/tmp/.X11-unix/X{}", display_number),
            windows: BTreeMap::new(),
            next_surface_id: 1,
            active_selection_owner: None,
            clipboard_buffer: Vec::new(),
        }
    }

    /// Process X11 `CreateWindow` request.
    pub fn create_window(
        &mut self,
        window_id: X11WindowId,
        parent_id: Option<X11WindowId>,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        override_redirect: bool,
    ) -> Result<(), XWaylandError> {
        let window = X11Window {
            window_id,
            parent_id,
            x,
            y,
            width,
            height,
            title: String::from("X11 Application"),
            state: WindowState::Unmapped,
            wayland_surface_id: None,
            override_redirect,
        };

        self.windows.insert(window_id, window);
        Ok(())
    }

    /// Process X11 `MapWindow` request, creating corresponding Wayland surface.
    pub fn map_window(&mut self, window_id: X11WindowId) -> Result<WaylandSurfaceId, XWaylandError> {
        let window = self.windows.get_mut(&window_id).ok_or(XWaylandError::WindowNotFound)?;

        if window.wayland_surface_id.is_none() {
            let surf_id = self.next_surface_id;
            self.next_surface_id += 1;
            window.wayland_surface_id = Some(surf_id);
        }

        window.state = WindowState::Mapped;
        Ok(window.wayland_surface_id.unwrap())
    }

    /// Process X11 `UnmapWindow` request.
    pub fn unmap_window(&mut self, window_id: X11WindowId) -> Result<(), XWaylandError> {
        let window = self.windows.get_mut(&window_id).ok_or(XWaylandError::WindowNotFound)?;
        window.state = WindowState::Unmapped;
        Ok(())
    }

    /// Synchronize clipboard between X11 and Wayland.
    pub fn set_clipboard(&mut self, source_window: X11WindowId, data: Vec<u8>) {
        self.active_selection_owner = Some(source_window);
        self.clipboard_buffer = data;
    }

    pub fn get_clipboard(&self) -> &[u8] {
        &self.clipboard_buffer
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XWaylandError {
    WindowNotFound,
    SurfaceCreationFailed,
    InvalidGeometry,
    SocketError,
}
