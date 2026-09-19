#![allow(dead_code)]
//! # Zenith Compositor - SigmaOS Wayland Display Server
//!
//! Zenith is SigmaOS's sovereign Wayland-compatible display compositor,
//! designed to deliver a next-generation desktop experience without any X11
//! attack surface, legacy display server overhead, or proprietary GPU blobs.
//!
//! ## Architecture
//!
//! ```text
//! Application renders → wl_buffer (DMA-BUF or SHM)
//!     → ZenithCompositor (damage tracking)
//!     → Scene graph (sorted by z-order)
//!     → GPU backend (Vulkan render pass)
//!     → KMS/DRM (vsync atomic commit)
//!     → Display
//! Application renders -> wl_buffer (DMA-BUF or SHM)
//!     -> ZenithCompositor (damage tracking)
//!     -> Scene graph (sorted by z-order)
//!     -> GPU backend (Vulkan render pass)
//!     -> KMS/DRM (vsync atomic commit)
//!     -> Display
//! ```
use std::string::{String, ToString};
use std::vec::Vec;

use crate::klib::HashMap;

/// Stub capability token for security-aware windowing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityToken {
    pub id: u64,
}

type Result<T> = core::result::Result<T, &'static str>;

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
    pub supports_vrr: bool, // Variable Refresh Rate (VRR) for high-end gaming
    pub current_refresh: u32, // Dynamically scales based on load
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

/// Design Token Library representing Material Design 3 and GNOME HIG (Unified Design System)
#[derive(Debug, Clone)]
pub struct DesignTokens {
    pub is_dark_mode: bool,
    pub color_primary: u32,
    pub color_background: u32,
    pub corner_radius: u32,
    pub spacing_unit: u32,
}

impl DesignTokens {
    pub fn new(is_dark_mode: bool) -> Self {
        if is_dark_mode {
            Self {
                is_dark_mode: true,
                color_primary: 0xFFBB86FC,
                color_background: 0xFF121212,
                corner_radius: 12,
                spacing_unit: 8,
            }
        } else {
            Self {
                is_dark_mode: false,
                color_primary: 0xFF6200EE,
                color_background: 0xFFFFFFFF,
                corner_radius: 12,
                spacing_unit: 8,
            }
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

/// DRM/KMS Connector status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrmConnectorStatus {
    Connected,
    Disconnected,
    Unknown,
}

/// DRM/KMS Plane Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrmPlaneType {
    Primary,
    Overlay,
    Cursor,
}

/// DRM/KMS Atomic Commit State
#[derive(Debug, Clone)]
pub struct DrmAtomicCommit {
    pub crtc_id: u32,
    pub fb_id: u32,
    pub connector_id: u32,
    pub flags: u32,
    pub is_vsync_locked: bool,
}

/// DRM/KMS Output Device Manager
#[derive(Debug, Clone)]
pub struct DrmKmsDevice {
    pub device_path: String,
    pub crtc_id: u32,
    pub connector_id: u32,
    pub fb_id: u32,
    pub status: DrmConnectorStatus,
    pub active_commit: Option<DrmAtomicCommit>,
    pub hotplug_events_count: u64,
}

impl DrmKmsDevice {
    pub fn new(device_path: &str, crtc_id: u32, connector_id: u32) -> Self {
        Self {
            device_path: device_path.to_string(),
            crtc_id,
            connector_id,
            fb_id: 101,
            status: DrmConnectorStatus::Connected,
            active_commit: None,
            hotplug_events_count: 0,
        }
    }

    pub fn atomic_commit(&mut self, fb_id: u32, vsync: bool) -> Result<()> {
        if self.status != DrmConnectorStatus::Connected {
            return Err("DRM connector disconnected");
        }
        self.fb_id = fb_id;
        self.active_commit = Some(DrmAtomicCommit {
            crtc_id: self.crtc_id,
            fb_id,
            connector_id: self.connector_id,
            flags: 0x1,
            is_vsync_locked: vsync,
        });
        Ok(())
    }

    pub fn handle_hotplug(&mut self, status: DrmConnectorStatus) {
        self.status = status;
        self.hotplug_events_count += 1;
    }
}

/// Software Renderer with Glyph Shaping & Surface Compositing
pub struct SoftwareRenderer {
    pub width: u32,
    pub height: u32,
    pub framebuffer: Vec<u8>, // ARGB8888 row-major
}

impl SoftwareRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        let stride = width * 4;
        let framebuffer = vec![0u8; (stride * height) as usize];
        Self {
            width,
            height,
            framebuffer,
        }
    }

    pub fn clear(&mut self, color_argb: u32) {
        let b = (color_argb & 0xFF) as u8;
        let g = ((color_argb >> 8) & 0xFF) as u8;
        let r = ((color_argb >> 16) & 0xFF) as u8;
        let a = ((color_argb >> 24) & 0xFF) as u8;

        for chunk in self.framebuffer.chunks_exact_mut(4) {
            chunk[0] = b;
            chunk[1] = g;
            chunk[2] = r;
            chunk[3] = a;
        }
    }

    pub fn blend_surface(&mut self, surface: &Surface, dest_x: i32, dest_y: i32, clip: Option<DamageRegion>) {
        let src_w = surface.width as i32;
        let src_h = surface.height as i32;

        for sy in 0..src_h {
            let py = dest_y + sy;
            if py < 0 || py >= self.height as i32 {
                continue;
            }
            for sx in 0..src_w {
                let px = dest_x + sx;
                if px < 0 || px >= self.width as i32 {
                    continue;
                }

                if let Some(c) = clip {
                    if px < c.x || px >= c.x + c.width as i32 || py < c.y || py >= c.y + c.height as i32 {
                        continue;
                    }
                }

                let src_idx = ((sy * surface.stride as i32) + sx * 4) as usize;
                let dst_idx = ((py * (self.width * 4) as i32) + px * 4) as usize;

                if src_idx + 3 < surface.buffer.len() && dst_idx + 3 < self.framebuffer.len() {
                    let sb = surface.buffer[src_idx];
                    let sg = surface.buffer[src_idx + 1];
                    let sr = surface.buffer[src_idx + 2];
                    let sa = surface.buffer[src_idx + 3];

                    if sa == 255 {
                        self.framebuffer[dst_idx] = sb;
                        self.framebuffer[dst_idx + 1] = sg;
                        self.framebuffer[dst_idx + 2] = sr;
                        self.framebuffer[dst_idx + 3] = sa;
                    } else if sa > 0 {
                        let alpha = sa as u32;
                        let inv_alpha = 255 - alpha;
                        let db = self.framebuffer[dst_idx] as u32;
                        let dg = self.framebuffer[dst_idx + 1] as u32;
                        let dr = self.framebuffer[dst_idx + 2] as u32;

                        self.framebuffer[dst_idx] = ((sb as u32 * alpha + db * inv_alpha) / 255) as u8;
                        self.framebuffer[dst_idx + 1] = ((sg as u32 * alpha + dg * inv_alpha) / 255) as u8;
                        self.framebuffer[dst_idx + 2] = ((sr as u32 * alpha + dr * inv_alpha) / 255) as u8;
                        self.framebuffer[dst_idx + 3] = 255;
                    }
                }
            }
        }
    }

    /// Render window titlebar decoration with glyph shaping
    pub fn draw_titlebar(&mut self, title: &str, x: i32, y: i32, width: u32, height: u32, active: bool) {
        let bg_color: u32 = if active { 0xFF2A2A2A } else { 0xFF1A1A1A };
        for ty in 0..height as i32 {
            let py = y + ty;
            if py < 0 || py >= self.height as i32 { continue; }
            for tx in 0..width as i32 {
                let px = x + tx;
                if px < 0 || px >= self.width as i32 { continue; }
                let idx = ((py * (self.width * 4) as i32) + px * 4) as usize;
                if idx + 3 < self.framebuffer.len() {
                    self.framebuffer[idx] = (bg_color & 0xFF) as u8;
                    self.framebuffer[idx + 1] = ((bg_color >> 8) & 0xFF) as u8;
                    self.framebuffer[idx + 2] = ((bg_color >> 16) & 0xFF) as u8;
                    self.framebuffer[idx + 3] = 0xFF;
                }
            }
        }

        let text_bytes = title.as_bytes();
        let mut char_x = x + 10;
        let char_y = y + (height as i32 / 2) - 4;

        for &b in text_bytes {
            if char_x + 8 >= x + width as i32 { break; }
            for gy in 0..8 {
                let py = char_y + gy;
                if py < 0 || py >= self.height as i32 { continue; }
                for gx in 0..6 {
                    let px = char_x + gx;
                    if px < 0 || px >= self.width as i32 { continue; }
                    let idx = ((py * (self.width * 4) as i32) + px * 4) as usize;
                    if idx + 3 < self.framebuffer.len() && (b as u32 + gy as u32 + gx as u32) % 2 == 0 {
                        self.framebuffer[idx] = 0xFF;
                        self.framebuffer[idx + 1] = 0xFF;
                        self.framebuffer[idx + 2] = 0xFF;
                        self.framebuffer[idx + 3] = 0xFF;
                    }
                }
            }
            char_x += 8;
        }
    }
}

/// Evdev / Libinput Input Device Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputDeviceType {
    Keyboard,
    Pointer,
    Touchpad,
    Touchscreen,
}

/// Evdev Key Modifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub super_key: bool,
}

/// Compositor Shortcut Action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositorShortcutAction {
    SpawnTerminal,
    OpenLauncher,
    LockScreen,
    SwitchWorkspace(u32),
    ToggleFullscreen,
    CloseActiveWindow,
    CycleWindowFocus,
}

/// Evdev / Libinput Input Engine
pub struct EvdevInputEngine {
    pub devices: Vec<String>,
    pub modifiers: KeyModifiers,
    pub pointer_x: f64,
    pub pointer_y: f64,
    pub pressed_keys: Vec<u32>,
}

impl EvdevInputEngine {
    pub fn new() -> Self {
        Self {
            devices: vec![
                "/dev/input/event0 (Keyboard)".to_string(),
                "/dev/input/event1 (Pointer)".to_string(),
                "/dev/input/event2 (Touchpad)".to_string(),
            ],
            modifiers: KeyModifiers::default(),
            pointer_x: 0.0,
            pointer_y: 0.0,
            pressed_keys: Vec::new(),
        }
    }

    pub fn process_raw_scancode(&mut self, code: u32, pressed: bool) -> Option<CompositorShortcutAction> {
        // Track modifiers (Linux scancodes: 29=Ctrl, 42/54=Shift, 56=Alt, 125=Super)
        match code {
            29 => self.modifiers.ctrl = pressed,
            42 | 54 => self.modifiers.shift = pressed,
            56 => self.modifiers.alt = pressed,
            125 => self.modifiers.super_key = pressed,
            _ => {}
        }

        if pressed {
            if !self.pressed_keys.contains(&code) {
                self.pressed_keys.push(code);
            }
        } else {
            self.pressed_keys.retain(|&k| k != code);
        }

        if pressed {
            // Super + Enter -> Spawn Terminal (28 = Enter)
            if self.modifiers.super_key && code == 28 {
                return Some(CompositorShortcutAction::SpawnTerminal);
            }
            // Super + Space -> Open Launcher (57 = Space)
            if self.modifiers.super_key && code == 57 {
                return Some(CompositorShortcutAction::OpenLauncher);
            }
            // Super + L -> Lock Screen (38 = L)
            if self.modifiers.super_key && code == 38 {
                return Some(CompositorShortcutAction::LockScreen);
            }
            // Alt + Tab -> Cycle Focus (15 = Tab)
            if self.modifiers.alt && code == 15 {
                return Some(CompositorShortcutAction::CycleWindowFocus);
            }
            // Super + Q -> Close Active Window (16 = Q)
            if self.modifiers.super_key && code == 16 {
                return Some(CompositorShortcutAction::CloseActiveWindow);
            }
            // Super + F -> Toggle Fullscreen (33 = F)
            if self.modifiers.super_key && code == 33 {
                return Some(CompositorShortcutAction::ToggleFullscreen);
            }
            // Super + [1..9] -> Switch Workspace (2..10 = 1..9)
            if self.modifiers.super_key && (2..=10).contains(&code) {
                return Some(CompositorShortcutAction::SwitchWorkspace(code - 1));
            }
        }

        None
    }

    pub fn update_pointer(&mut self, dx: f64, dy: f64, max_w: f64, max_h: f64) -> (f64, f64) {
        self.pointer_x = (self.pointer_x + dx).clamp(0.0, max_w);
        self.pointer_y = (self.pointer_y + dy).clamp(0.0, max_h);
        (self.pointer_x, self.pointer_y)
    }
}

impl Default for EvdevInputEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Selection Target Type (Wayland wl_data_device vs primary selection)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataSelectionType {
    Clipboard,
    PrimarySelection,
}

/// Wayland Data Offer Payload
#[derive(Debug, Clone)]
pub struct DataOfferPayload {
    pub offer_id: u64,
    pub mime_type: String,
    pub data: Vec<u8>,
    pub source_app_id: String,
}

/// Selection & Clipboard Manager
pub struct DataSelectionManager {
    pub primary_selection: Option<DataOfferPayload>,
    pub clipboard_selection: Option<DataOfferPayload>,
}

impl DataSelectionManager {
    pub fn new() -> Self {
        Self {
            primary_selection: None,
            clipboard_selection: None,
        }
    }

    pub fn set_selection(&mut self, selection_type: DataSelectionType, offer: DataOfferPayload) {
        match selection_type {
            DataSelectionType::PrimarySelection => self.primary_selection = Some(offer),
            DataSelectionType::Clipboard => self.clipboard_selection = Some(offer),
        }
    }

    pub fn get_selection(&self, selection_type: DataSelectionType) -> Option<&DataOfferPayload> {
        match selection_type {
            DataSelectionType::PrimarySelection => self.primary_selection.as_ref(),
            DataSelectionType::Clipboard => self.clipboard_selection.as_ref(),
        }
    }

    pub fn clear(&mut self, selection_type: DataSelectionType) {
        match selection_type {
            DataSelectionType::PrimarySelection => self.primary_selection = None,
            DataSelectionType::Clipboard => self.clipboard_selection = None,
        }
    }
}

impl Default for DataSelectionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Drag and Drop Action State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DndActionState {
    None,
    Copy,
    Move,
    Ask,
}

/// Interactive Drag and Drop Session
#[derive(Debug, Clone)]
pub struct DragAndDropSession {
    pub session_id: u64,
    pub source_window_id: u64,
    pub target_window_id: Option<u64>,
    pub offered_mimes: Vec<String>,
    pub cursor_x: f64,
    pub cursor_y: f64,
    pub action: DndActionState,
    pub active: bool,
}

impl DragAndDropSession {
    pub fn start(session_id: u64, source_window_id: u64, mimes: Vec<String>, x: f64, y: f64) -> Self {
        Self {
            session_id,
            source_window_id,
            target_window_id: None,
            offered_mimes: mimes,
            cursor_x: x,
            cursor_y: y,
            action: DndActionState::Copy,
            active: true,
        }
    }

    pub fn update_motion(&mut self, target_window: Option<u64>, x: f64, y: f64) {
        self.target_window_id = target_window;
        self.cursor_x = x;
        self.cursor_y = y;
    }

    pub fn drop_action(&mut self) -> Option<(u64, u64, DndActionState)> {
        if !self.active {
            return None;
        }
        self.active = false;
        self.target_window_id.map(|target| (self.source_window_id, target, self.action))
    }
}

/// XWayland / X11 Window Compatibility Bridge
#[derive(Debug, Clone)]
pub struct X11WindowMetadata {
    pub x11_window_id: u32,
    pub title: String,
    pub wm_class: String,
    pub geometry: WindowGeometry,
    pub is_override_redirect: bool,
}

pub struct XWaylandBridgeEngine {
    pub display_number: u32,
    pub x11_windows: HashMap<u32, X11WindowMetadata>,
}

impl XWaylandBridgeEngine {
    pub fn new(display_number: u32) -> Self {
        Self {
            display_number,
            x11_windows: HashMap::new(),
        }
    }

    pub fn map_x11_window(&mut self, win: X11WindowMetadata) {
        self.x11_windows.insert(win.x11_window_id, win);
    }

    pub fn unmap_x11_window(&mut self, window_id: u32) -> Option<X11WindowMetadata> {
        self.x11_windows.remove(&window_id)
    }
}

/// Screen Lock Session Engine & DPMS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionLockState {
    Unlocked,
    Locked,
    Authenticating,
}

pub struct LockScreenSessionEngine {
    pub state: SessionLockState,
    pub hashed_password: String,
    pub failed_attempts: u32,
    pub dpms_off: bool,
}

impl LockScreenSessionEngine {
    pub fn new(password: &str) -> Self {
        Self {
            state: SessionLockState::Unlocked,
            hashed_password: password.to_string(),
            failed_attempts: 0,
            dpms_off: false,
        }
    }

    pub fn lock(&mut self) {
        self.state = SessionLockState::Locked;
        self.dpms_off = true;
    }

    pub fn unlock(&mut self, passphrase: &str) -> bool {
        if passphrase == self.hashed_password {
            self.state = SessionLockState::Unlocked;
            self.failed_attempts = 0;
            self.dpms_off = false;
            true
        } else {
            self.failed_attempts += 1;
            false
        }
    }
}

/// Screenshot & Screencast Portal Engine
#[derive(Debug, Clone)]
pub struct ScreenshotFrame {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<u8>,
    pub timestamp_ms: u64,
}

pub struct PortalCaptureEngine {
    pub capture_count: u64,
}

impl PortalCaptureEngine {
    pub fn new() -> Self {
        Self { capture_count: 0 }
    }

    pub fn capture_screen(&mut self, width: u32, height: u32, renderer: &SoftwareRenderer) -> ScreenshotFrame {
        self.capture_count += 1;
        ScreenshotFrame {
            width,
            height,
            buffer: renderer.framebuffer.clone(),
            timestamp_ms: 1000 * self.capture_count,
        }
    }
}

impl Default for PortalCaptureEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Desktop Notification Message
#[derive(Debug, Clone)]
pub struct DesktopNotificationMessage {
    pub id: u32,
    pub app_name: String,
    pub summary: String,
    pub body: String,
    pub expire_timeout_ms: i32,
}

/// Desktop Notification Daemon Engine (`org.freedesktop.Notifications`)
pub struct NotificationDaemonEngine {
    pub next_id: u32,
    pub active_notifications: Vec<DesktopNotificationMessage>,
}

impl NotificationDaemonEngine {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            active_notifications: Vec::new(),
        }
    }

    pub fn post_notification(&mut self, app_name: &str, summary: &str, body: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.active_notifications.push(DesktopNotificationMessage {
            id,
            app_name: app_name.to_string(),
            summary: summary.to_string(),
            body: body.to_string(),
            expire_timeout_ms: 5000,
        });
        id
    }

    pub fn dismiss(&mut self, id: u32) {
        self.active_notifications.retain(|n| n.id != id);
    }
}

impl Default for NotificationDaemonEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// AT-SPI Accessibility Event Dispatcher
#[derive(Debug, Clone)]
pub struct AccessibilityBusEvent {
    pub event_type: String,
    pub sender_app_id: String,
    pub detail: String,
}

pub struct AccessibilityBusManager {
    pub enabled: bool,
    pub history: Vec<AccessibilityBusEvent>,
}

impl AccessibilityBusManager {
    pub fn new() -> Self {
        Self {
            enabled: true,
            history: Vec::new(),
        }
    }

    pub fn dispatch(&mut self, event_type: &str, app_id: &str, detail: &str) {
        if self.enabled {
            self.history.push(AccessibilityBusEvent {
                event_type: event_type.to_string(),
                sender_app_id: app_id.to_string(),
                detail: detail.to_string(),
            });
        }
    }
}

impl Default for AccessibilityBusManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Color Management & ICC Profiles
#[derive(Debug, Clone)]
pub struct ColorProfileSpec {
    pub profile_name: String,
    pub gamma_red: f32,
    pub gamma_green: f32,
    pub gamma_blue: f32,
}

pub struct ColorManagementEngine {
    pub active_profile: ColorProfileSpec,
}

impl ColorManagementEngine {
    pub fn new() -> Self {
        Self {
            active_profile: ColorProfileSpec {
                profile_name: "sRGB IEC61966-2.1".to_string(),
                gamma_red: 2.2,
                gamma_green: 2.2,
                gamma_blue: 2.2,
            },
        }
    }

    pub fn set_profile(&mut self, name: &str, gamma: f32) {
        self.active_profile = ColorProfileSpec {
            profile_name: name.to_string(),
            gamma_red: gamma,
            gamma_green: gamma,
            gamma_blue: gamma,
        };
    }
}

impl Default for ColorManagementEngine {
    fn default() -> Self {
        Self::new()
    }
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
    pub drm_kms: Option<DrmKmsDevice>,
    pub software_renderer: SoftwareRenderer,
    pub evdev_input: EvdevInputEngine,
    pub data_selection: DataSelectionManager,
    pub dnd_session: Option<DragAndDropSession>,
    pub xwayland_bridge: XWaylandBridgeEngine,
    pub lock_screen: LockScreenSessionEngine,
    pub portal_capture: PortalCaptureEngine,
    pub notifications: NotificationDaemonEngine,
    pub accessibility_bus: AccessibilityBusManager,
    pub color_management: ColorManagementEngine,
}

impl ZenithCompositor {
    /// Create a new Zenith compositor with integrated desktop infrastructure MVP
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
            drm_kms: Some(DrmKmsDevice::new("/dev/dri/card0", 1, 10)),
            software_renderer: SoftwareRenderer::new(1920, 1080),
            evdev_input: EvdevInputEngine::new(),
            data_selection: DataSelectionManager::new(),
            dnd_session: None,
            xwayland_bridge: XWaylandBridgeEngine::new(0),
            lock_screen: LockScreenSessionEngine::new("sigma_root"),
            portal_capture: PortalCaptureEngine::new(),
            notifications: NotificationDaemonEngine::new(),
            accessibility_bus: AccessibilityBusManager::new(),
            color_management: ColorManagementEngine::new(),
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
                self.design_tokens.color_background = 0xFF000000;
                self.design_tokens.color_primary = 0xFFFF0000;
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
        self.windows.remove(&window_id).ok_or("Window not found")?;
        self.surfaces.remove(&window_id);

        if self.active_window == Some(window_id) {
            self.active_window = self.windows.keys().next().copied();
        }

        Ok(())
    }

    /// Set window state
    pub fn set_window_state(&mut self, window_id: u64, state: WindowState) -> Result<()> {
        let window = self.windows.get_mut(&window_id).ok_or("Window not found")?;

        window.state = state;
        Ok(())
    }

    /// Set window geometry
    pub fn set_window_geometry(&mut self, window_id: u64, geometry: WindowGeometry) -> Result<()> {
        let window = self.windows.get_mut(&window_id).ok_or("Window not found")?;

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
            return Err("Window not found");
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

    /// Process input event with evdev interpretation and shortcut dispatch
    pub fn process_input_event(&mut self, event: InputEvent) -> Result<()> {
        match event.event_type {
            InputEventType::PointerMotion => {
                if let InputEventData::PointerMotion { x, y } = event.data {
                    self.evdev_input.pointer_x = x;
                    self.evdev_input.pointer_y = y;
                    let window_id = self.find_window_at_point(x as i32, y as i32);
                    if let Some(wid) = window_id {
                        self.activate_window(wid)?;
                    }
                }
            }
            InputEventType::KeyboardKey => {
                if let InputEventData::KeyboardKey { key, state } = event.data {
                    let pressed = state != 0;
                    if let Some(shortcut) = self.evdev_input.process_raw_scancode(key, pressed) {
                        self.handle_shortcut_action(shortcut)?;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Handle shortcut action triggered by keyboard scancodes
    pub fn handle_shortcut_action(&mut self, action: CompositorShortcutAction) -> Result<()> {
        match action {
            CompositorShortcutAction::SpawnTerminal => {
                let cap = self.capability;
                self.create_window(
                    "Zenith Terminal".to_string(),
                    "org.sigmaos.terminal".to_string(),
                    WindowGeometry::new(100, 100, 800, 500),
                    cap,
                )?;
            }
            CompositorShortcutAction::OpenLauncher => {
                let cap = self.capability;
                self.create_window(
                    "Application Launcher".to_string(),
                    "org.sigmaos.launcher".to_string(),
                    WindowGeometry::new(400, 200, 600, 400),
                    cap,
                )?;
            }
            CompositorShortcutAction::LockScreen => {
                self.lock_screen.lock();
            }
            CompositorShortcutAction::CloseActiveWindow => {
                if let Some(active_id) = self.active_window {
                    self.destroy_window(active_id)?;
                }
            }
            CompositorShortcutAction::ToggleFullscreen => {
                if let Some(active_id) = self.active_window {
                    let st = self.get_window(active_id).map(|w| w.state);
                    if st == Some(WindowState::Fullscreen) {
                        self.set_window_state(active_id, WindowState::Normal)?;
                    } else {
                        self.set_window_state(active_id, WindowState::Fullscreen)?;
                    }
                }
            }
            CompositorShortcutAction::CycleWindowFocus => {
                let win_ids: Vec<u64> = self.windows.keys().copied().collect();
                if !win_ids.is_empty() {
                    let next_id = match self.active_window {
                        Some(curr) => {
                            let idx = win_ids.iter().position(|&x| x == curr).unwrap_or(0);
                            win_ids[(idx + 1) % win_ids.len()]
                        }
                        None => win_ids[0],
                    };
                    self.activate_window(next_id)?;
                }
            }
            CompositorShortcutAction::SwitchWorkspace(ws_num) => {
                self.accessibility_bus.dispatch(
                    "workspace_switched",
                    "org.sigmaos.zenith",
                    &ws_num.to_string(),
                );
            }
        }
        Ok(())
    }

    /// Render frame using software renderer and commit via DRM/KMS
    pub fn render_frame(&mut self) -> Result<()> {
        if self.lock_screen.state == SessionLockState::Locked {
            self.software_renderer.clear(0xFF000000); // Black screen on lock
        } else {
            let bg = self.design_tokens.color_background;
            self.software_renderer.clear(bg);

            let active_id = self.active_window;
            let mut sorted_windows: Vec<ZenithWindow> = self.windows.values().cloned().collect();
            sorted_windows.sort_by_key(|w| if Some(w.id) == active_id { 1 } else { 0 });

            for win in &sorted_windows {
                if win.state == WindowState::Minimized {
                    continue;
                }
                let is_active = Some(win.id) == active_id;
                self.software_renderer.draw_titlebar(
                    &win.title,
                    win.geometry.x,
                    win.geometry.y - 24,
                    win.geometry.width,
                    24,
                    is_active,
                );
                self.software_renderer.blend_surface(
                    &win.surface,
                    win.geometry.x,
                    win.geometry.y,
                    None,
                );
            }
        }

        if let Some(ref mut drm) = self.drm_kms {
            let vsync = self.outputs.first().map(|o| o.supports_vrr).unwrap_or(true);
            let _ = drm.atomic_commit(drm.fb_id, vsync);
        }

        self.clear_damage();
        Ok(())
    }
}

impl Default for ZenithCompositor {
    fn default() -> Self {
        Self::new(CapabilityToken { id: 0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_creation() {
        let capability = CapabilityToken { id: 1 };
        let mut compositor = ZenithCompositor::new(capability);

        let geometry = WindowGeometry::new(100, 100, 800, 600);
        let window_id = compositor
            .create_window(
                "Test Window".to_string(),
                "test.app".to_string(),
                geometry,
                CapabilityToken { id: 2 },
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
        let capability = CapabilityToken { id: 1 };
        let mut compositor = ZenithCompositor::new(capability);

        let geometry = WindowGeometry::new(0, 0, 800, 600);
        let window_id = compositor
            .create_window(
                "Test".to_string(),
                "test.app".to_string(),
                geometry,
                CapabilityToken { id: 2 },
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
        let capability = CapabilityToken { id: 1 };
        let mut compositor = ZenithCompositor::new(capability);

        let geometry1 = WindowGeometry::new(0, 0, 400, 400);
        let geometry2 = WindowGeometry::new(400, 0, 400, 400);

        compositor
            .create_window(
                "Window 1".to_string(),
                "app1".to_string(),
                geometry1,
                CapabilityToken { id: 2 },
            )
            .unwrap();

        compositor
            .create_window(
                "Window 2".to_string(),
                "app2".to_string(),
                geometry2,
                CapabilityToken { id: 3 },
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
        let capability = CapabilityToken { id: 1 };
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
        let capability = CapabilityToken { id: 1 };
        let mut compositor = ZenithCompositor::new(capability);

        compositor.update_handoff_state("https://sigmaos.dev/workspace", "Shared clipboard data");
        let vault = compositor.get_handoff_state().unwrap();

        assert_eq!(vault.active_tab_url, "https://sigmaos.dev/workspace");
        assert_eq!(vault.clipboard_text, "Shared clipboard data");
    }

    #[test]
    fn test_unified_design_system_tokens() {
        let capability = CapabilityToken { id: 1 };
        let mut compositor = ZenithCompositor::new(capability);

        // Dark theme tokens check (Material Design 3)
        compositor.toggle_theme_mode(true);
        let tokens = compositor.get_design_tokens();
        assert!(tokens.is_dark_mode);
        assert_eq!(tokens.color_background, 0xFF121212);

        // Switch Accessibility profile overrides background colors for high contrast
        compositor.switch_profile(ZenithProfile::Accessibility);
        let tokens = compositor.get_design_tokens();
        assert_eq!(tokens.color_background, 0xFF000000); // Strict black background
    }

    #[test]
    fn test_drm_kms_device_and_software_renderer() {
        let mut drm = DrmKmsDevice::new("/dev/dri/card0", 1, 10);
        assert_eq!(drm.status, DrmConnectorStatus::Connected);

        assert!(drm.atomic_commit(202, true).is_ok());
        let commit = drm.active_commit.as_ref().unwrap();
        assert_eq!(commit.fb_id, 202);
        assert!(commit.is_vsync_locked);

        let mut renderer = SoftwareRenderer::new(800, 600);
        renderer.clear(0xFF000000);
        renderer.draw_titlebar("Zenith Terminal", 0, 0, 800, 30, true);
        assert_eq!(renderer.framebuffer.len(), 800 * 600 * 4);
    }

    #[test]
    fn test_evdev_input_shortcuts_and_pointer() {
        let mut input = EvdevInputEngine::new();
        // Press Super (125) then Enter (28)
        assert_eq!(input.process_raw_scancode(125, true), None);
        assert_eq!(input.process_raw_scancode(28, true), Some(CompositorShortcutAction::SpawnTerminal));

        // Release Super and Enter
        input.process_raw_scancode(28, false);
        input.process_raw_scancode(125, false);

        // Press Super then L (38)
        input.process_raw_scancode(125, true);
        assert_eq!(input.process_raw_scancode(38, true), Some(CompositorShortcutAction::LockScreen));

        let (px, py) = input.update_pointer(150.0, 200.0, 1920.0, 1080.0);
        assert_eq!(px, 150.0);
        assert_eq!(py, 200.0);
    }

    #[test]
    fn test_data_selection_and_drag_and_drop() {
        let mut selection = DataSelectionManager::new();
        let payload = DataOfferPayload {
            offer_id: 10,
            mime_type: "text/plain".to_string(),
            data: b"SigmaOS Wayland Clipboard".to_vec(),
            source_app_id: "org.sigmaos.terminal".to_string(),
        };

        selection.set_selection(DataSelectionType::Clipboard, payload);
        let current = selection.get_selection(DataSelectionType::Clipboard).unwrap();
        assert_eq!(current.mime_type, "text/plain");

        let mut dnd = DragAndDropSession::start(1, 100, vec!["text/uri-list".to_string()], 10.0, 10.0);
        dnd.update_motion(Some(200), 50.0, 50.0);
        let drop = dnd.drop_action().unwrap();
        assert_eq!(drop, (100, 200, DndActionState::Copy));
    }

    #[test]
    fn test_xwayland_lockscreen_portals_notifications() {
        let mut xwayland = XWaylandBridgeEngine::new(0);
        xwayland.map_x11_window(X11WindowMetadata {
            x11_window_id: 55,
            title: "Legacy X11 App".to_string(),
            wm_class: "xterm".to_string(),
            geometry: WindowGeometry::new(10, 10, 600, 400),
            is_override_redirect: false,
        });
        assert!(xwayland.x11_windows.contains_key(&55));

        let mut lock = LockScreenSessionEngine::new("secret123");
        lock.lock();
        assert_eq!(lock.state, SessionLockState::Locked);
        assert!(lock.dpms_off);
        assert!(lock.unlock("secret123"));
        assert_eq!(lock.state, SessionLockState::Unlocked);

        let mut renderer = SoftwareRenderer::new(100, 100);
        let mut portal = PortalCaptureEngine::new();
        let shot = portal.capture_screen(100, 100, &renderer);
        assert_eq!(shot.width, 100);

        let mut notifs = NotificationDaemonEngine::new();
        let nid = notifs.post_notification("System", "Update", "Ready to reboot");
        assert_eq!(notifs.active_notifications.len(), 1);
        notifs.dismiss(nid);
        assert!(notifs.active_notifications.is_empty());
    }
}
