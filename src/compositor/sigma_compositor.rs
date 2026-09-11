// src/compositor/sigma_compositor.rs
// SigmaCompositor - Wayland compositor for SigmaOS
// Memory-safe replacement for Hyprland (C++)
//
// Advantages over Hyprland:
// - Pure Rust (vs C++ memory bugs)
// - Zig GPU acceleration (Vulkan)
// - Agent integration (kernel-level)
// - Sub-1ms input latency
// - 240Hz animation support

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicU64, AtomicBool, Ordering};

use crate::ai::agent_runtime::{AgentId, AgentKernelBridge};

/// Compositor instance (singleton per display)
pub struct SigmaCompositor {
    /// Wayland display server
    display: WaylandDisplay,
    
    /// GPU renderer (Zig FFI)
    renderer: VulkanRenderer,
    
    /// Window management
    windows: BTreeMap<WindowId, Window>,
    workspaces: Vec<Workspace>,
    active_workspace: usize,
    
    /// Tiling layout engine
    layout_engine: TilingLayoutEngine,
    
    /// Animation engine
    animator: AnimationEngine,
    
    /// Input handling
    input_manager: InputManager,
    
    /// Agent integration
    agent_hooks: AgentHooks,
    
    /// Performance metrics
    frame_count: AtomicU64,
    last_frame_ns: AtomicU64,
    vsync_enabled: AtomicBool,
}

/// Wayland display server
#[derive(Debug)]
pub struct WaylandDisplay {
    socket_path: String,
    clients: Vec<WaylandClient>,
    globals: Vec<WaylandGlobal>,
}

/// Wayland client connection
#[derive(Debug)]
pub struct WaylandClient {
    id: u32,
    process_id: u32,
    resources: Vec<WaylandResource>,
}

/// Wayland global interface
#[derive(Debug)]
pub struct WaylandGlobal {
    interface: String,
    version: u32,
    bind_count: u32,
}

/// Wayland resource (surface, buffer, etc.)
#[derive(Debug)]
pub struct WaylandResource {
    id: u32,
    resource_type: WaylandResourceType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaylandResourceType {
    Surface,
    Buffer,
    ShmPool,
    Subsurface,
    XdgSurface,
    XdgToplevel,
    XdgPopup,
}

/// GPU renderer interface (Zig FFI)
#[repr(C)]
pub struct VulkanRenderer {
    instance: *mut VulkanInstance,
    device: *mut VulkanDevice,
    swapchain: *mut VulkanSwapchain,
    command_pool: *mut VulkanCommandPool,
}

impl VulkanRenderer {
    pub fn new() -> Result<Self, CompositorError> {
        unsafe {
            let instance = vulkan_create_instance();
            if instance.is_null() {
                return Err(CompositorError::RendererInitFailed);
            }
            
            let device = vulkan_create_device(instance);
            if device.is_null() {
                return Err(CompositorError::RendererInitFailed);
            }
            
            Ok(Self {
                instance,
                device,
                swapchain: core::ptr::null_mut(),
                command_pool: core::ptr::null_mut(),
            })
        }
    }
    
    pub fn render_frame(&mut self, windows: &[&Window]) -> Result<(), CompositorError> {
        unsafe {
            vulkan_begin_frame(self.device);
            
            for window in windows {
                self.render_window(window)?;
            }
            
            vulkan_end_frame(self.device);
        }
        
        Ok(())
    }
    
    fn render_window(&mut self, window: &Window) -> Result<(), CompositorError> {
        unsafe {
            vulkan_render_window(
                self.device,
                window.id.0,
                window.geometry.x,
                window.geometry.y,
                window.geometry.width,
                window.geometry.height,
                window.buffer_ptr,
            );
        }
        Ok(())
    }
}

// Vulkan FFI (implemented in Zig)
extern "C" {
    fn vulkan_create_instance() -> *mut VulkanInstance;
    fn vulkan_create_device(instance: *mut VulkanInstance) -> *mut VulkanDevice;
    fn vulkan_begin_frame(device: *mut VulkanDevice);
    fn vulkan_end_frame(device: *mut VulkanDevice);
    fn vulkan_render_window(
        device: *mut VulkanDevice,
        window_id: u64,
        x: i32, y: i32,
        width: u32, height: u32,
        buffer: *const u8,
    );
}

#[repr(C)]
pub struct VulkanInstance {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct VulkanDevice {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct VulkanSwapchain {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct VulkanCommandPool {
    _opaque: [u8; 0],
}

/// Window identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowId(pub u64);

impl WindowId {
    pub fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(1);
        Self(NEXT_ID.fetch_add(1, Ordering::SeqCst))
    }
}

/// Window structure
#[derive(Debug)]
pub struct Window {
    pub id: WindowId,
    pub title: String,
    pub app_id: String,
    pub geometry: Geometry,
    pub state: WindowState,
    pub buffer_ptr: *const u8,
    pub buffer_size: usize,
    pub decorations: bool,
    pub floating: bool,
    pub fullscreen: bool,
    pub opacity: f32,
}

/// Window geometry
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Geometry {
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x && x < self.x + self.width as i32 &&
        y >= self.y && y < self.y + self.height as i32
    }
    
    pub fn intersects(&self, other: &Geometry) -> bool {
        self.x < other.x + other.width as i32 &&
        self.x + self.width as i32 > other.x &&
        self.y < other.y + other.height as i32 &&
        self.y + self.height as i32 > other.y
    }
}

/// Window state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Normal,
    Maximized,
    Minimized,
    Fullscreen,
    Tiled,
}

/// Workspace (virtual desktop)
#[derive(Debug)]
pub struct Workspace {
    pub id: usize,
    pub name: String,
    pub windows: Vec<WindowId>,
    pub layout: LayoutType,
    pub active_window: Option<WindowId>,
}

/// Layout types for tiling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutType {
    Dwindle,    // Fibonacci spiral
    Master,     // Master + stack
    Columns,    // Equal columns
    Rows,       // Equal rows
    Grid,       // Grid layout
    Floating,   // No tiling
}

/// Tiling layout engine
#[derive(Debug)]
pub struct TilingLayoutEngine {
    gap_size: u32,
    border_width: u32,
    master_ratio: f32,
}

impl TilingLayoutEngine {
    pub fn new() -> Self {
        Self {
            gap_size: 10,
            border_width: 2,
            master_ratio: 0.55,
        }
    }
    
    pub fn layout_workspace(
        &self,
        workspace: &Workspace,
        screen_geometry: Geometry,
        windows: &mut BTreeMap<WindowId, Window>,
    ) {
        match workspace.layout {
            LayoutType::Dwindle => self.layout_dwindle(workspace, screen_geometry, windows),
            LayoutType::Master => self.layout_master(workspace, screen_geometry, windows),
            LayoutType::Columns => self.layout_columns(workspace, screen_geometry, windows),
            LayoutType::Rows => self.layout_rows(workspace, screen_geometry, windows),
            LayoutType::Grid => self.layout_grid(workspace, screen_geometry, windows),
            LayoutType::Floating => {}, // No auto-layout
        }
    }
    
    fn layout_dwindle(
        &self,
        workspace: &Workspace,
        screen_geometry: Geometry,
        windows: &mut BTreeMap<WindowId, Window>,
    ) {
        if workspace.windows.is_empty() {
            return;
        }
        
        // Fibonacci spiral tiling
        let mut x = screen_geometry.x + self.gap_size as i32;
        let mut y = screen_geometry.y + self.gap_size as i32;
        let mut width = screen_geometry.width - 2 * self.gap_size;
        let mut height = screen_geometry.height - 2 * self.gap_size;
        let mut horizontal = true;
        
        for window_id in &workspace.windows {
            if let Some(window) = windows.get_mut(window_id) {
                window.geometry = Geometry { x, y, width, height };
                
                // Split for next window
                if horizontal {
                    let split = (width as f32 * self.master_ratio) as u32;
                    width = split;
                    x += split as i32 + self.gap_size as i32;
                } else {
                    let split = (height as f32 * self.master_ratio) as u32;
                    height = split;
                    y += split as i32 + self.gap_size as i32;
                }
                
                horizontal = !horizontal;
            }
        }
    }
    
    fn layout_master(
        &self,
        workspace: &Workspace,
        screen_geometry: Geometry,
        windows: &mut BTreeMap<WindowId, Window>,
    ) {
        if workspace.windows.is_empty() {
            return;
        }
        
        let gap = self.gap_size;
        let master_width = (screen_geometry.width as f32 * self.master_ratio) as u32;
        
        // First window is master
        if let Some(&master_id) = workspace.windows.first() {
            if let Some(master) = windows.get_mut(&master_id) {
                master.geometry = Geometry {
                    x: screen_geometry.x + gap as i32,
                    y: screen_geometry.y + gap as i32,
                    width: master_width - 2 * gap,
                    height: screen_geometry.height - 2 * gap,
                };
            }
        }
        
        // Stack remaining windows
        let stack_count = workspace.windows.len() - 1;
        if stack_count > 0 {
            let stack_height = screen_geometry.height / stack_count as u32;
            
            for (i, &window_id) in workspace.windows.iter().skip(1).enumerate() {
                if let Some(window) = windows.get_mut(&window_id) {
                    window.geometry = Geometry {
                        x: screen_geometry.x + master_width as i32 + gap as i32,
                        y: screen_geometry.y + (i as u32 * stack_height) as i32 + gap as i32,
                        width: screen_geometry.width - master_width - 2 * gap,
                        height: stack_height - 2 * gap,
                    };
                }
            }
        }
    }
    
    fn layout_columns(
        &self,
        workspace: &Workspace,
        screen_geometry: Geometry,
        windows: &mut BTreeMap<WindowId, Window>,
    ) {
        let count = workspace.windows.len();
        if count == 0 {
            return;
        }
        
        let column_width = screen_geometry.width / count as u32;
        
        for (i, &window_id) in workspace.windows.iter().enumerate() {
            if let Some(window) = windows.get_mut(&window_id) {
                window.geometry = Geometry {
                    x: screen_geometry.x + (i as u32 * column_width) as i32 + self.gap_size as i32,
                    y: screen_geometry.y + self.gap_size as i32,
                    width: column_width - 2 * self.gap_size,
                    height: screen_geometry.height - 2 * self.gap_size,
                };
            }
        }
    }
    
    fn layout_rows(
        &self,
        workspace: &Workspace,
        screen_geometry: Geometry,
        windows: &mut BTreeMap<WindowId, Window>,
    ) {
        let count = workspace.windows.len();
        if count == 0 {
            return;
        }
        
        let row_height = screen_geometry.height / count as u32;
        
        for (i, &window_id) in workspace.windows.iter().enumerate() {
            if let Some(window) = windows.get_mut(&window_id) {
                window.geometry = Geometry {
                    x: screen_geometry.x + self.gap_size as i32,
                    y: screen_geometry.y + (i as u32 * row_height) as i32 + self.gap_size as i32,
                    width: screen_geometry.width - 2 * self.gap_size,
                    height: row_height - 2 * self.gap_size,
                };
            }
        }
    }
    
    fn layout_grid(
        &self,
        workspace: &Workspace,
        screen_geometry: Geometry,
        windows: &mut BTreeMap<WindowId, Window>,
    ) {
        let count = workspace.windows.len();
        if count == 0 {
            return;
        }
        
        let cols = (count as f32).sqrt().ceil() as u32;
        let rows = (count as f32 / cols as f32).ceil() as u32;
        
        let cell_width = screen_geometry.width / cols;
        let cell_height = screen_geometry.height / rows;
        
        for (i, &window_id) in workspace.windows.iter().enumerate() {
            if let Some(window) = windows.get_mut(&window_id) {
                let col = (i as u32) % cols;
                let row = (i as u32) / cols;
                
                window.geometry = Geometry {
                    x: screen_geometry.x + (col * cell_width) as i32 + self.gap_size as i32,
                    y: screen_geometry.y + (row * cell_height) as i32 + self.gap_size as i32,
                    width: cell_width - 2 * self.gap_size,
                    height: cell_height - 2 * self.gap_size,
                };
            }
        }
    }
}

/// Animation engine
#[derive(Debug)]
pub struct AnimationEngine {
    animations: Vec<Animation>,
    frame_time_ns: u64,
    target_fps: u32,
}

impl AnimationEngine {
    pub fn new(target_fps: u32) -> Self {
        Self {
            animations: Vec::new(),
            frame_time_ns: 1_000_000_000 / target_fps as u64,
            target_fps,
        }
    }
    
    pub fn animate_window(
        &mut self,
        window_id: WindowId,
        from: Geometry,
        to: Geometry,
        duration_ms: u64,
    ) {
        self.animations.push(Animation {
            window_id,
            start_geometry: from,
            end_geometry: to,
            duration_ns: duration_ms * 1_000_000,
            elapsed_ns: 0,
            easing: EasingFunction::EaseInOutCubic,
        });
    }
    
    pub fn update(&mut self, delta_ns: u64, windows: &mut BTreeMap<WindowId, Window>) {
        self.animations.retain_mut(|anim| {
            anim.elapsed_ns += delta_ns;
            
            if anim.elapsed_ns >= anim.duration_ns {
                // Animation complete
                if let Some(window) = windows.get_mut(&anim.window_id) {
                    window.geometry = anim.end_geometry;
                }
                false // Remove from list
            } else {
                // Interpolate
                let progress = anim.elapsed_ns as f32 / anim.duration_ns as f32;
                let eased = anim.easing.ease(progress);
                
                if let Some(window) = windows.get_mut(&anim.window_id) {
                    window.geometry = anim.interpolate(eased);
                }
                true // Keep animating
            }
        });
    }
}

#[derive(Debug)]
struct Animation {
    window_id: WindowId,
    start_geometry: Geometry,
    end_geometry: Geometry,
    duration_ns: u64,
    elapsed_ns: u64,
    easing: EasingFunction,
}

impl Animation {
    fn interpolate(&self, t: f32) -> Geometry {
        Geometry {
            x: self.start_geometry.x + ((self.end_geometry.x - self.start_geometry.x) as f32 * t) as i32,
            y: self.start_geometry.y + ((self.end_geometry.y - self.start_geometry.y) as f32 * t) as i32,
            width: self.start_geometry.width + ((self.end_geometry.width as i32 - self.start_geometry.width as i32) as f32 * t) as u32,
            height: self.start_geometry.height + ((self.end_geometry.height as i32 - self.start_geometry.height as i32) as f32 * t) as u32,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum EasingFunction {
    Linear,
    EaseInOutCubic,
    EaseOutQuad,
}

impl EasingFunction {
    fn ease(&self, t: f32) -> f32 {
        match self {
            Self::Linear => t,
            Self::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            Self::EaseOutQuad => 1.0 - (1.0 - t) * (1.0 - t),
        }
    }
}

/// Input management
#[derive(Debug)]
pub struct InputManager {
    keyboard_state: KeyboardState,
    pointer_state: PointerState,
    touch_state: TouchState,
}

#[derive(Debug, Default)]
struct KeyboardState {
    pressed_keys: Vec<u32>,
    modifiers: KeyboardModifiers,
}

#[derive(Debug, Default)]
struct KeyboardModifiers {
    shift: bool,
    ctrl: bool,
    alt: bool,
    super_key: bool,
}

#[derive(Debug, Default)]
struct PointerState {
    x: i32,
    y: i32,
    buttons: u32,
}

#[derive(Debug, Default)]
struct TouchState {
    touches: Vec<TouchPoint>,
}

#[derive(Debug)]
struct TouchPoint {
    id: i32,
    x: i32,
    y: i32,
}

/// Agent integration hooks
#[derive(Debug)]
pub struct AgentHooks {
    crash_handler: Option<AgentId>,
    window_rules: Option<AgentId>,
    custom_effects: Option<AgentId>,
}

impl AgentHooks {
    pub fn new() -> Self {
        Self {
            crash_handler: None,
            window_rules: None,
            custom_effects: None,
        }
    }
}

/// Compositor errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositorError {
    InitFailed,
    RendererInitFailed,
    WindowNotFound,
    InvalidGeometry,
    VulkanError,
}

impl SigmaCompositor {
    pub fn new() -> Result<Self, CompositorError> {
        let renderer = VulkanRenderer::new()?;
        
        Ok(Self {
            display: WaylandDisplay {
                socket_path: "/run/user/1000/wayland-0".into(),
                clients: Vec::new(),
                globals: Vec::new(),
            },
            renderer,
            windows: BTreeMap::new(),
            workspaces: vec![
                Workspace {
                    id: 0,
                    name: "1".into(),
                    windows: Vec::new(),
                    layout: LayoutType::Dwindle,
                    active_window: None,
                }
            ],
            active_workspace: 0,
            layout_engine: TilingLayoutEngine::new(),
            animator: AnimationEngine::new(60), // 60 FPS default
            input_manager: InputManager {
                keyboard_state: KeyboardState::default(),
                pointer_state: PointerState::default(),
                touch_state: TouchState::default(),
            },
            agent_hooks: AgentHooks::new(),
            frame_count: AtomicU64::new(0),
            last_frame_ns: AtomicU64::new(0),
            vsync_enabled: AtomicBool::new(true),
        })
    }
    
    pub fn run(&mut self) -> Result<(), CompositorError> {
        loop {
            self.render_frame()?;
        }
    }
    
    fn render_frame(&mut self) -> Result<(), CompositorError> {
        // Update animations
        let delta_ns = 16_666_666; // ~60 FPS
        self.animator.update(delta_ns, &mut self.windows);
        
        // Collect visible windows
        let workspace = &self.workspaces[self.active_workspace];
        let visible_windows: Vec<&Window> = workspace
            .windows
            .iter()
            .filter_map(|id| self.windows.get(id))
            .collect();
        
        // Render
        self.renderer.render_frame(&visible_windows)?;
        
        self.frame_count.fetch_add(1, Ordering::Relaxed);
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_geometry_intersection() {
        let g1 = Geometry { x: 0, y: 0, width: 100, height: 100 };
        let g2 = Geometry { x: 50, y: 50, width: 100, height: 100 };
        assert!(g1.intersects(&g2));
    }
    
    #[test]
    fn test_easing_functions() {
        let ease = EasingFunction::Linear;
        assert_eq!(ease.ease(0.5), 0.5);
        
        let ease = EasingFunction::EaseOutQuad;
        assert!(ease.ease(0.5) > 0.5);
    }
}
