//! OOP-based Zenith Desktop Core for SigmaOS
//! Implements desktop environment using OOP principles with traits and structs
//! No dependency on external desktop frameworks
//! Based on Roadmap Item 41: Zenith Desktop core



use std::boxed::Box;
use std::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Window ID
pub type WindowID = usize;

/// Window state
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Minimized = 0,
    Normal = 1,
    Maximized = 2,
    Fullscreen = 3,
    Hidden = 4,
}

impl WindowState {
    pub fn from_usize(val: usize) -> Self {
        match val {
            0 => WindowState::Minimized,
            1 => WindowState::Normal,
            2 => WindowState::Maximized,
            3 => WindowState::Fullscreen,
            _ => WindowState::Hidden,
        }
    }
}

/// Window trait (OOP interface)
pub trait Window {
    /// Get window ID
    fn id(&self) -> WindowID;
    /// Get window title
    fn title(&self) -> &[u8];
    /// Show window
    fn show(&mut self) -> Result<(), DesktopError>;
    /// Hide window
    fn hide(&mut self) -> Result<(), DesktopError>;
    /// Minimize window
    fn minimize(&mut self) -> Result<(), DesktopError>;
    /// Maximize window
    fn maximize(&mut self) -> Result<(), DesktopError>;
    /// Close window
    fn close(&mut self) -> Result<(), DesktopError>;
    /// Get window state
    fn state(&self) -> WindowState;
    /// Get window info
    fn info(&self) -> WindowInfo;
}

/// Desktop error types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopError {
    Success = 0,
    AlreadyVisible = 1,
    AlreadyHidden = 2,
    PermissionDenied = 3,
    InvalidState = 4,
}

/// Window info
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WindowInfo {
    pub id: WindowID,
    pub title: [u8; 128],
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub state: WindowState,
    pub capability: WindowCapability,
}

impl WindowInfo {
    pub fn new(id: WindowID) -> Self {
        WindowInfo {
            id,
            title: [0; 128],
            x: 0,
            y: 0,
            width: 800,
            height: 600,
            state: WindowState::Normal,
            capability: WindowCapability::new(),
        }
    }
}

/// Window capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WindowCapability {
    pub can_move: bool,
    pub can_resize: bool,
    pub can_close: bool,
}

impl Default for WindowCapability {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowCapability {
    pub fn new() -> Self {
        WindowCapability {
            can_move: false,
            can_resize: false,
            can_close: false,
        }
    }

    pub fn full() -> Self {
        WindowCapability {
            can_move: true,
            can_resize: true,
            can_close: true,
        }
    }
}

/// Simple window (OOP: Concrete window class)
#[repr(C)]
pub struct SimpleWindow {
    pub id: WindowID,
    pub title: [u8; 128],
    pub title_len: u8,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub state: AtomicUsize, // WindowState as usize
    pub capability: WindowCapability,
}

impl SimpleWindow {
    pub fn new(id: WindowID, title: &[u8], capability: WindowCapability) -> Self {
        let mut title_array = [0u8; 128];
        let title_len = title.len().min(127);
        title_array[..title_len].copy_from_slice(&title[..title_len]);

        SimpleWindow {
            id,
            title: title_array,
            title_len: title_len as u8,
            x: 100,
            y: 100,
            width: 800,
            height: 600,
            state: AtomicUsize::new(WindowState::Normal as usize),
            capability,
        }
    }

    pub fn set_position(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn get_state(&self) -> WindowState {
        WindowState::from_usize(self.state.load(Ordering::SeqCst))
    }

    pub fn set_state(&self, state: WindowState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }
}

impl Window for SimpleWindow {
    fn id(&self) -> WindowID {
        self.id
    }

    fn title(&self) -> &[u8] {
        // Bolt ⚡ Optimization: Store explicit title length on creation to eliminate
        // O(N) zero-byte linear scanning (.position(|&b| b == 0)) on every window title query,
        // reducing slice lookup to instantaneous O(1) constant time.
        &self.title[..self.title_len as usize]
    }

    fn show(&mut self) -> Result<(), DesktopError> {
        if !self.capability.can_move {
            return Err(DesktopError::PermissionDenied);
        }

        let current_state = self.get_state();
        if current_state == WindowState::Normal {
            return Err(DesktopError::AlreadyVisible);
        }

        self.set_state(WindowState::Normal);
        Ok(())
    }

    fn hide(&mut self) -> Result<(), DesktopError> {
        let current_state = self.get_state();
        if current_state == WindowState::Hidden {
            return Err(DesktopError::AlreadyHidden);
        }

        self.set_state(WindowState::Hidden);
        Ok(())
    }

    fn minimize(&mut self) -> Result<(), DesktopError> {
        self.set_state(WindowState::Minimized);
        Ok(())
    }

    fn maximize(&mut self) -> Result<(), DesktopError> {
        self.set_state(WindowState::Maximized);
        Ok(())
    }

    fn close(&mut self) -> Result<(), DesktopError> {
        if !self.capability.can_close {
            return Err(DesktopError::PermissionDenied);
        }

        self.set_state(WindowState::Hidden);
        Ok(())
    }

    fn state(&self) -> WindowState {
        self.get_state()
    }

    fn info(&self) -> WindowInfo {
        WindowInfo {
            id: self.id,
            title: self.title,
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            state: self.get_state(),
            capability: self.capability,
        }
    }
}

/// Desktop compositor trait (OOP interface)
pub trait DesktopCompositor {
    /// Create window
    fn create_window(&mut self, title: &[u8], capability: WindowCapability) -> Result<WindowID, DesktopError>;
    /// Destroy window
    fn destroy_window(&mut self, id: WindowID) -> Result<(), DesktopError>;
    /// Get window
    fn get_window(&self, id: WindowID) -> Option<&dyn Window>;
    /// Focus window
    fn focus_window(&mut self, id: WindowID) -> Result<(), DesktopError>;
    /// List windows
    fn list_windows(&self) -> Vec<WindowID>;
    /// Get compositor statistics
    fn stats(&self) -> DesktopStats;
}

/// Desktop statistics
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DesktopStats {
    pub total_windows: usize,
    pub visible_windows: usize,
    pub minimized_windows: usize,
    pub maximized_windows: usize,
}

impl Default for DesktopStats {
    fn default() -> Self {
        Self::new()
    }
}

impl DesktopStats {
    pub fn new() -> Self {
        DesktopStats {
            total_windows: 0,
            visible_windows: 0,
            minimized_windows: 0,
            maximized_windows: 0,
        }
    }
}

/// Simple desktop compositor (OOP: Concrete compositor class)
pub struct SimpleDesktopCompositor {
    windows: Vec<Option<Box<dyn Window>>>,
    next_id: AtomicUsize,
    focused_window: AtomicUsize,
    stats: DesktopStats,
    capability: CompositorCapability,
}

/// Compositor capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CompositorCapability {
    pub can_create: bool,
    pub can_destroy: bool,
    pub can_focus: bool,
}

impl Default for CompositorCapability {
    fn default() -> Self {
        Self::new()
    }
}

impl CompositorCapability {
    pub fn new() -> Self {
        CompositorCapability {
            can_create: false,
            can_destroy: false,
            can_focus: false,
        }
    }

    pub fn full() -> Self {
        CompositorCapability {
            can_create: true,
            can_destroy: true,
            can_focus: true,
        }
    }
}

impl SimpleDesktopCompositor {
    pub fn new(capability: CompositorCapability) -> Self {
        SimpleDesktopCompositor {
            windows: Vec::new(),
            next_id: AtomicUsize::new(1),
            focused_window: AtomicUsize::new(0),
            stats: DesktopStats::new(),
            capability,
        }
    }
}

// =========================================================================
// LINUX & BSD INSPIRED ZENITH DESKTOP COMPOSITOR EXTENSIONS
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TilingLayoutMode {
    Dwindle,
    MasterStack,
    FloatingGrid,
    Tabbed,
}

/// Hyprland-inspired dynamic tiling layout engine
pub struct HyprlandTilingLayoutEngine {
    pub layout_mode: TilingLayoutMode,
    pub gap_size_px: u32,
    pub border_size_px: u32,
}

impl HyprlandTilingLayoutEngine {
    pub fn new() -> Self {
        Self {
            layout_mode: TilingLayoutMode::Dwindle,
            gap_size_px: 10,
            border_size_px: 2,
        }
    }

    pub fn compute_window_geometry(&self, window_idx: usize, total_windows: usize, screen_width: u32, screen_height: u32) -> (u32, u32, u32, u32) {
        if total_windows == 0 {
            return (0, 0, screen_width, screen_height);
        }

        match self.layout_mode {
            TilingLayoutMode::Dwindle | TilingLayoutMode::MasterStack => {
                if total_windows == 1 {
                    (
                        self.gap_size_px,
                        self.gap_size_px,
                        screen_width.saturating_sub(2 * self.gap_size_px),
                        screen_height.saturating_sub(2 * self.gap_size_px),
                    )
                } else if window_idx == 0 {
                    (
                        self.gap_size_px,
                        self.gap_size_px,
                        (screen_width / 2).saturating_sub(self.gap_size_px),
                        screen_height.saturating_sub(2 * self.gap_size_px),
                    )
                } else {
                    let stack_count = (total_windows - 1) as u32;
                    let stack_height = (screen_height.saturating_sub(2 * self.gap_size_px)) / stack_count;
                    (
                        screen_width / 2 + self.gap_size_px,
                        self.gap_size_px + (window_idx as u32 - 1) * stack_height,
                        (screen_width / 2).saturating_sub(2 * self.gap_size_px),
                        stack_height.saturating_sub(self.gap_size_px),
                    )
                }
            }
            _ => (100, 100, 800, 600),
        }
    }
}

impl Default for HyprlandTilingLayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Wayland / COSMIC-inspired direct scanout pipeline with explicit synchronization
pub struct WaylandCosmicScanoutPipeline {
    pub explicit_sync_enabled: bool,
    pub vblank_target_fps: u32,
    pub direct_scanout_active: bool,
}

impl WaylandCosmicScanoutPipeline {
    pub fn new() -> Self {
        Self {
            explicit_sync_enabled: true,
            vblank_target_fps: 144,
            direct_scanout_active: false,
        }
    }

    pub fn acquire_scanout_buffer(&mut self, is_fullscreen: bool) -> bool {
        self.direct_scanout_active = is_fullscreen && self.explicit_sync_enabled;
        self.direct_scanout_active
    }
}

impl Default for WaylandCosmicScanoutPipeline {
    fn default() -> Self {
        Self::new()
    }
}

/// Omarchy / Catppuccin theme palette sync engine
pub struct OmarchyThemeSyncEngine {
    pub active_theme: &'static str,
    pub primary_accent_rgb: (u8, u8, u8),
    pub background_rgb: (u8, u8, u8),
}

impl OmarchyThemeSyncEngine {
    pub fn new() -> Self {
        Self {
            active_theme: "TokyoNight",
            primary_accent_rgb: (122, 162, 247),
            background_rgb: (26, 27, 38),
        }
    }

    pub fn set_theme(&mut self, name: &'static str, accent: (u8, u8, u8), bg: (u8, u8, u8)) {
        self.active_theme = name;
        self.primary_accent_rgb = accent;
        self.background_rgb = bg;
    }
}

impl Default for OmarchyThemeSyncEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// GNOME / KDE / macOS fractional display scaling manager
pub struct FractionalDisplayScaler {
    pub scale_factor: f32, // e.g. 1.25, 1.5, 2.0
}

impl FractionalDisplayScaler {
    pub fn new(scale_factor: f32) -> Self {
        Self { scale_factor }
    }

    pub fn scale_dimension(&self, px: u32) -> u32 {
        (px as f32 * self.scale_factor) as u32
    }
}

impl Default for FractionalDisplayScaler {
    fn default() -> Self {
        Self::new(1.25)
    }
}

impl DesktopCompositor for SimpleDesktopCompositor {
    fn create_window(&mut self, title: &[u8], capability: WindowCapability) -> Result<WindowID, DesktopError> {
        if !self.capability.can_create {
            return Err(DesktopError::PermissionDenied);
        }

        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let window = SimpleWindow::new(id, title, capability);
        self.windows.push(Some(Box::new(window)));
        self.stats.total_windows += 1;
        self.stats.visible_windows += 1;
        Ok(id)
    }

    fn destroy_window(&mut self, id: WindowID) -> Result<(), DesktopError> {
        if !self.capability.can_destroy {
            return Err(DesktopError::PermissionDenied);
        }

        let mut index = None;
        for (i, window_option) in self.windows.iter().enumerate() {
            if let Some(ref window) = *window_option {
                if window.id() == id {
                    index = Some(i);
                    break;
                }
            }
        }

        if let Some(i) = index {
            self.windows[i] = None;
            self.stats.total_windows -= 1;
            self.stats.visible_windows -= 1;
            Ok(())
        } else {
            Err(DesktopError::PermissionDenied)
        }
    }

    fn get_window(&self, id: WindowID) -> Option<&dyn Window> {
        for window_option in &self.windows {
            if let Some(ref window) = *window_option {
                if window.id() == id {
                    return Some(window.as_ref());
                }
            }
        }
        None
    }

    fn focus_window(&mut self, id: WindowID) -> Result<(), DesktopError> {
        if !self.capability.can_focus {
            return Err(DesktopError::PermissionDenied);
        }

        if self.get_window(id).is_some() {
            self.focused_window.store(id, Ordering::SeqCst);
            Ok(())
        } else {
            Err(DesktopError::PermissionDenied)
        }
    }

    fn list_windows(&self) -> Vec<WindowID> {
        let mut ids = Vec::new();
        for window_option in &self.windows {
            if let Some(ref window) = *window_option {
                ids.push(window.id());
            }
        }
        ids
    }

    fn stats(&self) -> DesktopStats {
        self.stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_window_creation_and_state() {
        let mut window = SimpleWindow::new(1, b"Terminal", WindowCapability::full());
        assert_eq!(window.id(), 1);
        assert_eq!(window.title(), b"Terminal");
        assert_eq!(window.state(), WindowState::Normal);

        assert!(window.minimize().is_ok());
        assert_eq!(window.state(), WindowState::Minimized);

        assert!(window.maximize().is_ok());
        assert_eq!(window.state(), WindowState::Maximized);
    }

    #[test]
    fn test_simple_desktop_compositor_operations() {
        let mut compositor = SimpleDesktopCompositor::new(CompositorCapability::full());
        let id = compositor.create_window(b"Browser", WindowCapability::full()).unwrap();
        assert_eq!(id, 1);

        assert!(compositor.focus_window(id).is_ok());
        let list = compositor.list_windows();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0], 1);

        assert!(compositor.destroy_window(id).is_ok());
        assert_eq!(compositor.list_windows().len(), 0);
    }

    #[test]
    fn test_hyprland_tiling_layout_engine() {
        let tiling = HyprlandTilingLayoutEngine::new();
        let (x, y, w, h) = tiling.compute_window_geometry(0, 2, 1920, 1080);
        assert_eq!(x, 10);
        assert_eq!(y, 10);
        assert_eq!(w, 950);
        assert_eq!(h, 1060);

        let (x2, y2, w2, h2) = tiling.compute_window_geometry(1, 2, 1920, 1080);
        assert_eq!(x2, 970);
        assert_eq!(y2, 10);
        assert_eq!(w2, 940);
        assert_eq!(h2, 1050);
    }

    #[test]
    fn test_wayland_cosmic_scanout_pipeline() {
        let mut scanout = WaylandCosmicScanoutPipeline::new();
        assert!(scanout.explicit_sync_enabled);
        assert!(!scanout.acquire_scanout_buffer(false));
        assert!(scanout.acquire_scanout_buffer(true));
        assert!(scanout.direct_scanout_active);
    }

    #[test]
    fn test_omarchy_theme_sync_engine() {
        let mut theme_sync = OmarchyThemeSyncEngine::new();
        assert_eq!(theme_sync.active_theme, "TokyoNight");
        theme_sync.set_theme("CatppuccinMocha", (203, 166, 247), (30, 30, 46));
        assert_eq!(theme_sync.active_theme, "CatppuccinMocha");
        assert_eq!(theme_sync.primary_accent_rgb, (203, 166, 247));
    }

    #[test]
    fn test_fractional_display_scaler() {
        let scaler = FractionalDisplayScaler::new(1.5);
        assert_eq!(scaler.scale_dimension(100), 150);
        assert_eq!(scaler.scale_dimension(1920), 2880);
    }
}
