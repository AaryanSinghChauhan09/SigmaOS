//! SigmaOS Desktop — Wayland compositor, input handling, theme synthesis,
//! and window management.
//!
//! # Architecture
//!
//! The crate is structured around four composable subsystems, each with a
//! single, clear responsibility (SRP / separation of concerns):
//!
//! - [`WaylandCompositor`] — Wayland protocol mechanism (policy-free)
//! - [`InputHandler`]      — Keyboard/pointer/touch event routing
//! - [`ThemeEngine`]       — Color scheme synthesis and hot-reload
//! - [`WindowManager`]     — Layout, focus, and workspace management
//!
//! Subsystems communicate through plain data types; none holds a reference to
//! another (loose coupling, YAGNI, composition over inheritance).

use thiserror::Error;

// ──────────────────────────────────────────────────────────────────────────────
// Shared geometry
// ──────────────────────────────────────────────────────────────────────────────

/// Axis-aligned rectangle on the display, in logical pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }

    /// Returns `true` when the point `(px, py)` falls inside this rectangle.
    pub fn contains(&self, px: i32, py: i32) -> bool {
        px >= self.x
            && px < self.x + self.width as i32
            && py >= self.y
            && py < self.y + self.height as i32
    }

    /// Returns `true` when `other` overlaps this rectangle.
    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width as i32
            && self.x + self.width as i32 > other.x
            && self.y < other.y + other.height as i32
            && self.y + self.height as i32 > other.y
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Errors
// ──────────────────────────────────────────────────────────────────────────────

/// Errors emitted by the compositor subsystem.
#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum CompositorError {
    #[error("compositor already initialised")]
    AlreadyInitialised,
    #[error("renderer backend unavailable")]
    RendererUnavailable,
    #[error("surface {0} not found")]
    SurfaceNotFound(u64),
    #[error("invalid geometry")]
    InvalidGeometry,
}

/// Errors emitted by the input subsystem.
#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum InputError {
    #[error("no device available")]
    NoDevice,
    #[error("event queue overflow")]
    QueueOverflow,
    #[error("unknown event type")]
    UnknownEventType,
}

/// Errors emitted by the theme subsystem.
#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum ThemeError {
    #[error("theme not found")]
    ThemeNotFound,
    #[error("invalid color format")]
    InvalidColor,
    #[error("invalid image data")]
    InvalidImage,
    #[error("cannot remove built-in theme")]
    CannotRemoveBuiltin,
}

/// Errors emitted by the window manager subsystem.
#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum WmError {
    #[error("window {0} not found")]
    WindowNotFound(u64),
    #[error("workspace {0} not found")]
    WorkspaceNotFound(usize),
    #[error("layout engine error")]
    LayoutError,
}

// ──────────────────────────────────────────────────────────────────────────────
// Wayland Compositor
// ──────────────────────────────────────────────────────────────────────────────

/// Opaque surface handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SurfaceId(pub u64);

/// Wayland compositor — owns the display socket and the set of client surfaces.
///
/// Mechanism only; no window-management policy is embedded here.
/// Policy lives in [`WindowManager`].
#[derive(Debug)]
pub struct WaylandCompositor {
    socket_path: &'static str,
    surfaces: Vec<Surface>,
    next_surface_id: u64,
    running: bool,
}

/// A Wayland client surface.
#[derive(Debug)]
pub struct Surface {
    pub id: SurfaceId,
    pub geometry: Rect,
    pub buffer: Option<SurfaceBuffer>,
    pub opaque: bool,
}

/// Pixel buffer attached to a [`Surface`].
#[derive(Debug, Clone)]
pub struct SurfaceBuffer {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    /// Pixel data (ARGB8888, stride * height bytes).
    pub data: Vec<u8>,
}

impl WaylandCompositor {
    /// Creates a new compositor that will listen on `socket_path`.
    pub fn new(socket_path: &'static str) -> Self {
        Self {
            socket_path,
            surfaces: Vec::new(),
            next_surface_id: 1,
            running: false,
        }
    }

    /// Initialises the compositor backend.
    ///
    /// Returns [`CompositorError::AlreadyInitialised`] if called more than once.
    pub fn init(&mut self) -> Result<(), CompositorError> {
        if self.running {
            return Err(CompositorError::AlreadyInitialised);
        }
        self.running = true;
        Ok(())
    }

    /// Creates a new surface and returns its id.
    pub fn create_surface(&mut self, geometry: Rect) -> SurfaceId {
        let id = SurfaceId(self.next_surface_id);
        self.next_surface_id += 1;
        self.surfaces.push(Surface {
            id,
            geometry,
            buffer: None,
            opaque: false,
        });
        id
    }

    /// Attaches a buffer to the given surface.
    pub fn attach_buffer(
        &mut self,
        id: SurfaceId,
        buffer: SurfaceBuffer,
    ) -> Result<(), CompositorError> {
        let surface = self
            .surfaces
            .iter_mut()
            .find(|s| s.id == id)
            .ok_or(CompositorError::SurfaceNotFound(id.0))?;
        surface.buffer = Some(buffer);
        Ok(())
    }

    /// Destroys a surface, releasing its buffer.
    pub fn destroy_surface(&mut self, id: SurfaceId) -> Result<(), CompositorError> {
        let pos = self
            .surfaces
            .iter()
            .position(|s| s.id == id)
            .ok_or(CompositorError::SurfaceNotFound(id.0))?;
        self.surfaces.swap_remove(pos);
        Ok(())
    }

    /// Returns `true` when the compositor loop is running.
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Shuts the compositor down.
    pub fn shutdown(&mut self) {
        self.running = false;
    }

    /// Returns the Wayland socket path.
    pub fn socket_path(&self) -> &str {
        self.socket_path
    }

    /// Render a single frame — stub; real implementation calls the GPU backend.
    pub fn render_frame(&mut self) -> Result<(), CompositorError> {
        // Composition order: back-to-front by insertion order (stub).
        let _ = self.surfaces.len();
        Ok(())
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Input Handler
// ──────────────────────────────────────────────────────────────────────────────

/// Normalised input event — device-independent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputEvent {
    KeyPress { keycode: u32, modifiers: Modifiers },
    KeyRelease { keycode: u32, modifiers: Modifiers },
    PointerMove { x: i32, y: i32 },
    ButtonPress { button: u32, x: i32, y: i32 },
    ButtonRelease { button: u32, x: i32, y: i32 },
    TouchBegin { id: u32, x: i32, y: i32 },
    TouchUpdate { id: u32, x: i32, y: i32 },
    TouchEnd { id: u32 },
    Scroll { delta_x: i32, delta_y: i32 },
}

/// Keyboard modifier flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub super_key: bool,
}

/// Routes raw device events to focused surfaces.
///
/// Separation of mechanism (evdev reading) from policy (which window gets
/// focus) is maintained: `InputHandler` dispatches but does not decide focus.
#[derive(Debug, Default)]
pub struct InputHandler {
    queue: Vec<InputEvent>,
    focused_surface: Option<SurfaceId>,
    pointer_x: i32,
    pointer_y: i32,
}

impl InputHandler {
    /// Creates a new, idle input handler.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enqueues a raw event for later dispatch.
    pub fn push_event(&mut self, event: InputEvent) -> Result<(), InputError> {
        const MAX_QUEUE: usize = 256;
        if self.queue.len() >= MAX_QUEUE {
            return Err(InputError::QueueOverflow);
        }
        // Track pointer position eagerly so focus queries are always current.
        match &event {
            InputEvent::PointerMove { x, y } => {
                self.pointer_x = *x;
                self.pointer_y = *y;
            }
            _ => {}
        }
        self.queue.push(event);
        Ok(())
    }

    /// Drains the event queue and returns the events.
    pub fn drain_events(&mut self) -> Vec<InputEvent> {
        std::mem::take(&mut self.queue)
    }

    /// Sets the surface that will receive keyboard events.
    pub fn set_focus(&mut self, surface: Option<SurfaceId>) {
        self.focused_surface = surface;
    }

    /// Returns the currently focused surface.
    pub fn focused_surface(&self) -> Option<SurfaceId> {
        self.focused_surface
    }

    /// Returns the current pointer position.
    pub fn pointer_pos(&self) -> (i32, i32) {
        (self.pointer_x, self.pointer_y)
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Theme Engine
// ──────────────────────────────────────────────────────────────────────────────

/// RGBA colour (0–255 per channel).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Relative luminance (0.0 = black, 1.0 = white).
    pub fn luminance(self) -> f32 {
        let linearize = |c: u8| (c as f32 / 255.0).powf(2.2);
        0.2126 * linearize(self.r) + 0.7152 * linearize(self.g) + 0.0722 * linearize(self.b)
    }

    /// Returns `true` when this colour should be treated as a light colour.
    pub fn is_light(self) -> bool {
        self.luminance() > 0.5
    }

    /// Linear blend: `self * (1 - t) + other * t`.
    pub fn blend(self, other: Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        let inv = 1.0 - t;
        Color::rgba(
            ((self.r as f32 * inv) + (other.r as f32 * t)) as u8,
            ((self.g as f32 * inv) + (other.g as f32 * t)) as u8,
            ((self.b as f32 * inv) + (other.b as f32 * t)) as u8,
            ((self.a as f32 * inv) + (other.a as f32 * t)) as u8,
        )
    }
}

/// Complete colour scheme for one theme.
#[derive(Debug, Clone, PartialEq)]
pub struct ColorScheme {
    pub background: Color,
    pub foreground: Color,
    pub primary: Color,
    pub accent: Color,
    pub surface: Color,
    pub border: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
}

impl ColorScheme {
    /// Built-in dark default palette.
    pub fn dark_default() -> Self {
        Self {
            background: Color::rgb(30, 30, 30),
            foreground: Color::rgb(230, 230, 230),
            primary: Color::rgb(100, 149, 237),
            accent: Color::rgb(255, 110, 180),
            surface: Color::rgb(40, 40, 40),
            border: Color::rgb(60, 60, 60),
            success: Color::rgb(76, 175, 80),
            warning: Color::rgb(255, 152, 0),
            error: Color::rgb(244, 67, 54),
        }
    }

    /// Built-in light default palette.
    pub fn light_default() -> Self {
        Self {
            background: Color::rgb(255, 255, 255),
            foreground: Color::rgb(20, 20, 20),
            primary: Color::rgb(25, 118, 210),
            accent: Color::rgb(255, 64, 129),
            surface: Color::rgb(245, 245, 245),
            border: Color::rgb(220, 220, 220),
            success: Color::rgb(56, 142, 60),
            warning: Color::rgb(251, 140, 0),
            error: Color::rgb(211, 47, 47),
        }
    }
}

/// A named, immutable theme.
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub colors: ColorScheme,
    pub font_family: String,
    pub font_size_base: u32,
    pub border_radius: u32,
    pub animation_duration_ms: u32,
}

impl Theme {
    /// Creates the built-in `"SigmaDefault"` dark theme.
    pub fn sigma_default() -> Self {
        Self {
            name: "SigmaDefault".into(),
            colors: ColorScheme::dark_default(),
            font_family: "Inter".into(),
            font_size_base: 14,
            border_radius: 8,
            animation_duration_ms: 200,
        }
    }
}

/// Synthesises, stores, and hot-reloads themes.
///
/// New themes can be registered at runtime without modifying existing ones
/// (open/closed principle).
#[derive(Debug)]
pub struct ThemeEngine {
    active: String,
    themes: std::collections::HashMap<String, Theme>,
}

impl ThemeEngine {
    /// Creates the engine pre-loaded with `"SigmaDefault"`.
    pub fn new() -> Self {
        let default = Theme::sigma_default();
        let name = default.name.clone();
        let mut themes = std::collections::HashMap::new();
        themes.insert(name.clone(), default);
        Self { active: name, themes }
    }

    /// Registers a new theme. Replaces any existing theme with the same name.
    pub fn register(&mut self, theme: Theme) {
        self.themes.insert(theme.name.clone(), theme);
    }

    /// Activates the named theme.
    pub fn activate(&mut self, name: &str) -> Result<(), ThemeError> {
        if !self.themes.contains_key(name) {
            return Err(ThemeError::ThemeNotFound);
        }
        self.active = name.to_owned();
        Ok(())
    }

    /// Returns a reference to the currently active theme.
    pub fn active_theme(&self) -> &Theme {
        self.themes.get(&self.active).expect("active theme always present")
    }

    /// Removes a theme by name.  The built-in `"SigmaDefault"` cannot be removed.
    pub fn remove(&mut self, name: &str) -> Result<(), ThemeError> {
        if name == "SigmaDefault" {
            return Err(ThemeError::CannotRemoveBuiltin);
        }
        if self.active == name {
            self.active = "SigmaDefault".into();
        }
        self.themes.remove(name).ok_or(ThemeError::ThemeNotFound)?;
        Ok(())
    }

    /// Returns the names of all registered themes.
    pub fn theme_names(&self) -> Vec<&str> {
        self.themes.keys().map(String::as_str).collect()
    }

    /// Derives a [`ColorScheme`] from raw ARGB8888 image bytes.
    ///
    /// Uses simple median-cut sampling; suitable for wallpaper-based synthesis.
    pub fn synthesise_from_image(&self, data: &[u8]) -> Result<ColorScheme, ThemeError> {
        if data.len() < 4 {
            return Err(ThemeError::InvalidImage);
        }
        let mut pixels: Vec<Color> = data
            .chunks_exact(4)
            .map(|p| Color::rgba(p[1], p[2], p[3], p[0]))
            .collect();
        pixels.sort_by(|a, b| a.luminance().partial_cmp(&b.luminance()).unwrap_or(std::cmp::Ordering::Equal));
        let bg = *pixels.first().unwrap();
        let fg = *pixels.last().unwrap();
        let accent = pixels[pixels.len() / 2];
        Ok(ColorScheme {
            background: bg,
            foreground: fg,
            primary: accent,
            accent,
            surface: bg.blend(fg, 0.05),
            border: bg.blend(fg, 0.15),
            success: Color::rgb(76, 175, 80),
            warning: Color::rgb(255, 152, 0),
            error: Color::rgb(244, 67, 54),
        })
    }
}

impl Default for ThemeEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Window Manager
// ──────────────────────────────────────────────────────────────────────────────

/// Unique, monotonically increasing window identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowId(pub u64);

/// Lifecycle / presentation state of a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WindowState {
    #[default]
    Normal,
    Maximised,
    Minimised,
    Fullscreen,
    Tiled,
}

/// Tiling strategy applied to a workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LayoutStrategy {
    #[default]
    Dwindle,
    MasterStack,
    Columns,
    Rows,
    Grid,
    Floating,
}

/// A managed window.
#[derive(Debug)]
pub struct ManagedWindow {
    pub id: WindowId,
    pub title: String,
    pub app_id: String,
    pub geometry: Rect,
    pub state: WindowState,
    pub surface: Option<SurfaceId>,
    pub floating: bool,
}

/// A virtual workspace (desktop).
#[derive(Debug)]
pub struct Workspace {
    pub id: usize,
    pub name: String,
    pub windows: Vec<WindowId>,
    pub layout: LayoutStrategy,
    pub focused: Option<WindowId>,
}

impl Workspace {
    pub fn new(id: usize, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            windows: Vec::new(),
            layout: LayoutStrategy::default(),
            focused: None,
        }
    }
}

/// Manages window lifecycle, focus, layout, and workspaces.
///
/// Policy lives here; the compositor only knows surfaces, not windows.
#[derive(Debug)]
pub struct WindowManager {
    workspaces: Vec<Workspace>,
    active_workspace: usize,
    windows: std::collections::HashMap<WindowId, ManagedWindow>,
    next_window_id: u64,
    gap: u32,
}

impl WindowManager {
    /// Creates a window manager with a single default workspace.
    pub fn new() -> Self {
        Self {
            workspaces: vec![Workspace::new(0, "1")],
            active_workspace: 0,
            windows: std::collections::HashMap::new(),
            next_window_id: 1,
            gap: 8,
        }
    }

    // ── Workspace management ──────────────────────────────────────────────────

    /// Adds a new workspace and returns its id.
    pub fn add_workspace(&mut self, name: impl Into<String>) -> usize {
        let id = self.workspaces.len();
        self.workspaces.push(Workspace::new(id, name));
        id
    }

    /// Switches to the workspace with the given id.
    pub fn switch_workspace(&mut self, id: usize) -> Result<(), WmError> {
        if id >= self.workspaces.len() {
            return Err(WmError::WorkspaceNotFound(id));
        }
        self.active_workspace = id;
        Ok(())
    }

    /// Returns a reference to the currently active workspace.
    pub fn active_workspace(&self) -> &Workspace {
        &self.workspaces[self.active_workspace]
    }

    // ── Window lifecycle ──────────────────────────────────────────────────────

    /// Registers a new window on the active workspace and returns its id.
    pub fn create_window(
        &mut self,
        title: impl Into<String>,
        app_id: impl Into<String>,
        geometry: Rect,
    ) -> WindowId {
        let id = WindowId(self.next_window_id);
        self.next_window_id += 1;
        let window = ManagedWindow {
            id,
            title: title.into(),
            app_id: app_id.into(),
            geometry,
            state: WindowState::Normal,
            surface: None,
            floating: false,
        };
        self.windows.insert(id, window);
        self.workspaces[self.active_workspace].windows.push(id);
        self.workspaces[self.active_workspace].focused = Some(id);
        id
    }

    /// Removes a window from all workspaces and the manager.
    pub fn destroy_window(&mut self, id: WindowId) -> Result<(), WmError> {
        if !self.windows.contains_key(&id) {
            return Err(WmError::WindowNotFound(id.0));
        }
        self.windows.remove(&id);
        for ws in &mut self.workspaces {
            ws.windows.retain(|&w| w != id);
            if ws.focused == Some(id) {
                ws.focused = ws.windows.last().copied();
            }
        }
        Ok(())
    }

    /// Returns a reference to a window.
    pub fn window(&self, id: WindowId) -> Result<&ManagedWindow, WmError> {
        self.windows.get(&id).ok_or(WmError::WindowNotFound(id.0))
    }

    /// Sets focus to the given window.
    pub fn focus(&mut self, id: WindowId) -> Result<(), WmError> {
        if !self.windows.contains_key(&id) {
            return Err(WmError::WindowNotFound(id.0));
        }
        self.workspaces[self.active_workspace].focused = Some(id);
        Ok(())
    }

    // ── Layout ────────────────────────────────────────────────────────────────

    /// Re-tiles the active workspace within `screen`.
    pub fn retile(&mut self, screen: Rect) -> Result<(), WmError> {
        let ws = &self.workspaces[self.active_workspace];
        let strategy = ws.layout;
        let window_ids: Vec<WindowId> = ws.windows.clone();
        match strategy {
            LayoutStrategy::Dwindle => self.layout_dwindle(&window_ids, screen),
            LayoutStrategy::MasterStack => self.layout_master(&window_ids, screen),
            LayoutStrategy::Columns => self.layout_columns(&window_ids, screen),
            LayoutStrategy::Rows => self.layout_rows(&window_ids, screen),
            LayoutStrategy::Grid => self.layout_grid(&window_ids, screen),
            LayoutStrategy::Floating => {} // no-op
        }
        Ok(())
    }

    fn layout_dwindle(&mut self, ids: &[WindowId], screen: Rect) {
        let gap = self.gap as i32;
        let mut x = screen.x + gap;
        let mut y = screen.y + gap;
        let mut w = screen.width.saturating_sub(2 * self.gap);
        let mut h = screen.height.saturating_sub(2 * self.gap);
        let mut horizontal = true;
        for id in ids {
            if let Some(win) = self.windows.get_mut(id) {
                win.geometry = Rect::new(x, y, w, h);
            }
            if horizontal {
                let half = w / 2;
                x += half as i32 + gap;
                w = half;
            } else {
                let half = h / 2;
                y += half as i32 + gap;
                h = half;
            }
            horizontal = !horizontal;
        }
    }

    fn layout_master(&mut self, ids: &[WindowId], screen: Rect) {
        if ids.is_empty() {
            return;
        }
        let gap = self.gap;
        let master_w = screen.width * 55 / 100;
        if let Some(first) = ids.first() {
            if let Some(win) = self.windows.get_mut(first) {
                win.geometry = Rect::new(
                    screen.x + gap as i32,
                    screen.y + gap as i32,
                    master_w.saturating_sub(2 * gap),
                    screen.height.saturating_sub(2 * gap),
                );
            }
        }
        let stack = &ids[1..];
        if !stack.is_empty() {
            let each_h = screen.height / stack.len() as u32;
            for (i, id) in stack.iter().enumerate() {
                if let Some(win) = self.windows.get_mut(id) {
                    win.geometry = Rect::new(
                        screen.x + master_w as i32 + gap as i32,
                        screen.y + (i as u32 * each_h) as i32 + gap as i32,
                        screen.width.saturating_sub(master_w).saturating_sub(2 * gap),
                        each_h.saturating_sub(2 * gap),
                    );
                }
            }
        }
    }

    fn layout_columns(&mut self, ids: &[WindowId], screen: Rect) {
        let n = ids.len() as u32;
        if n == 0 {
            return;
        }
        let col_w = screen.width / n;
        let gap = self.gap;
        for (i, id) in ids.iter().enumerate() {
            if let Some(win) = self.windows.get_mut(id) {
                win.geometry = Rect::new(
                    screen.x + (i as u32 * col_w) as i32 + gap as i32,
                    screen.y + gap as i32,
                    col_w.saturating_sub(2 * gap),
                    screen.height.saturating_sub(2 * gap),
                );
            }
        }
    }

    fn layout_rows(&mut self, ids: &[WindowId], screen: Rect) {
        let n = ids.len() as u32;
        if n == 0 {
            return;
        }
        let row_h = screen.height / n;
        let gap = self.gap;
        for (i, id) in ids.iter().enumerate() {
            if let Some(win) = self.windows.get_mut(id) {
                win.geometry = Rect::new(
                    screen.x + gap as i32,
                    screen.y + (i as u32 * row_h) as i32 + gap as i32,
                    screen.width.saturating_sub(2 * gap),
                    row_h.saturating_sub(2 * gap),
                );
            }
        }
    }

    fn layout_grid(&mut self, ids: &[WindowId], screen: Rect) {
        let n = ids.len() as u32;
        if n == 0 {
            return;
        }
        let cols = (n as f32).sqrt().ceil() as u32;
        let rows = n.div_ceil(cols);
        let cw = screen.width / cols;
        let ch = screen.height / rows;
        let gap = self.gap;
        for (i, id) in ids.iter().enumerate() {
            let col = i as u32 % cols;
            let row = i as u32 / cols;
            if let Some(win) = self.windows.get_mut(id) {
                win.geometry = Rect::new(
                    screen.x + (col * cw) as i32 + gap as i32,
                    screen.y + (row * ch) as i32 + gap as i32,
                    cw.saturating_sub(2 * gap),
                    ch.saturating_sub(2 * gap),
                );
            }
        }
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Re-exports
// ──────────────────────────────────────────────────────────────────────────────

pub use std::string::String;
