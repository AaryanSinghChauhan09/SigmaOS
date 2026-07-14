#![no_std]

/// Zenith Desktop Compositor for SigmaOS
/// Implements tiling and floating window management
/// Based on 100-Improvement-Ideas.md #41: Zenith Desktop compositor (tiling + floating)

use core::sync::atomic::{AtomicU64, Ordering};

/// Window ID type
pub type WindowID = u64;

/// Window states
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Normal = 0,
    Minimized = 1,
    Maximized = 2,
    Fullscreen = 3,
    Hidden = 4,
}

/// Layout modes
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutMode {
    Tiling = 0,
    Floating = 1,
    Tabbed = 2,
    Stacked = 3,
}

/// Window geometry
#[repr(C)]
pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Geometry {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Geometry {
            x,
            y,
            width,
            height,
        }
    }
    
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x && y >= self.y && 
        x < (self.x + self.width as i32) && 
        y < (self.y + self.height as i32)
    }
}

/// Window
#[repr(C)]
pub struct Window {
    pub id: WindowID,
    pub title: [u8; 128],
    pub geometry: Geometry,
    pub state: WindowState,
    pub focused: bool,
    pub created_at: u64,
}

impl Window {
    pub fn new(id: WindowID, title: &str, geometry: Geometry) -> Self {
        let mut title_array = [0u8; 128];
        let title_bytes = title.as_bytes();
        let len = title_bytes.len().min(127);
        
        unsafe {
            core::ptr::copy_nonoverlapping(title_bytes.as_ptr(), title_array.as_mut_ptr(), len);
        }
        
        Window {
            id,
            title: title_array,
            geometry,
            state: WindowState::Normal,
            focused: false,
            created_at: get_current_time(),
        }
    }
    
    pub fn title_str(&self) -> &str {
        unsafe {
            let len = self.title.iter().position(|&b| b == 0).unwrap_or(128);
            core::str::from_utf8_unchecked(&self.title[..len])
        }
    }
}

/// Workspace
#[repr(C)]
pub struct Workspace {
    pub id: u64,
    pub name: [u8; 32],
    pub windows: Vec<Option<Window>>,
    pub layout_mode: LayoutMode,
}

impl Workspace {
    pub fn new(id: u64, name: &str) -> Self {
        let mut name_array = [0u8; 32];
        let name_bytes = name.as_bytes();
        let len = name_bytes.len().min(31);
        
        unsafe {
            core::ptr::copy_nonoverlapping(name_bytes.as_ptr(), name_array.as_mut_ptr(), len);
        }
        
        Workspace {
            id,
            name: name_array,
            windows: Vec::new(),
            layout_mode: LayoutMode::Tiling,
        }
    }
    
    pub fn add_window(&mut self, window: Window) {
        self.windows.push(Some(window));
    }
    
    pub fn remove_window(&mut self, window_id: WindowID) -> bool {
        for window_option in &mut self.windows {
            if let Some(ref window) = *window_option {
                if window.id == window_id {
                    *window_option = None;
                    return true;
                }
            }
        }
        false
    }
    
    pub fn get_window(&self, window_id: WindowID) -> Option<&Window> {
        for window_option in &self.windows {
            if let Some(ref window) = *window_option {
                if window.id == window_id {
                    return Some(window);
                }
            }
        }
        None
    }
}

/// Zenith Compositor
pub struct ZenithCompositor {
    workspaces: Vec<Option<Workspace>>,
    current_workspace: u64,
    next_window_id: AtomicU64,
    next_workspace_id: AtomicU64,
    focused_window: AtomicU64,
}

impl ZenithCompositor {
    pub fn new() -> Self {
        let mut compositor = ZenithCompositor {
            workspaces: Vec::new(),
            current_workspace: 0,
            next_window_id: AtomicU64::new(1),
            next_workspace_id: AtomicU64::new(1),
            focused_window: AtomicU64::new(0),
        };
        
        // Create default workspace
        compositor.create_workspace("Main");
        compositor
    }
    
    /// Create new workspace
    pub fn create_workspace(&mut self, name: &str) -> u64 {
        let id = self.next_workspace_id.fetch_add(1, Ordering::SeqCst);
        let workspace = Workspace::new(id, name);
        self.workspaces.push(Some(workspace));
        id
    }
    
    /// Switch to workspace
    pub fn switch_workspace(&mut self, workspace_id: u64) -> bool {
        for workspace_option in &self.workspaces {
            if let Some(ref workspace) = *workspace_option {
                if workspace.id == workspace_id {
                    self.current_workspace = workspace_id;
                    self.focused_window.store(0, Ordering::SeqCst);
                    return true;
                }
            }
        }
        false
    }
    
    /// Create new window
    pub fn create_window(&mut self, title: &str, geometry: Geometry) -> WindowID {
        let id = self.next_window_id.fetch_add(1, Ordering::SeqCst);
        let window = Window::new(id, title, geometry);
        
        if let Some(ref mut workspace) = self.get_current_workspace_mut() {
            workspace.add_window(window);
        }
        
        id
    }
    
    /// Close window
    pub fn close_window(&mut self, window_id: WindowID) -> bool {
        if let Some(ref mut workspace) = self.get_current_workspace_mut() {
            if workspace.remove_window(window_id) {
                if self.focused_window.load(Ordering::SeqCst) == window_id {
                    self.focused_window.store(0, Ordering::SeqCst);
                }
                return true;
            }
        }
        false
    }
    
    /// Focus window
    pub fn focus_window(&mut self, window_id: WindowID) -> bool {
        if let Some(ref mut workspace) = self.get_current_workspace_mut() {
            if workspace.get_window(window_id).is_some() {
                self.focused_window.store(window_id, Ordering::SeqCst);
                return true;
            }
        }
        false
    }
    
    /// Set window geometry
    pub fn set_window_geometry(&mut self, window_id: WindowID, geometry: Geometry) -> bool {
        if let Some(ref mut workspace) = self.get_current_workspace_mut() {
            for window_option in &mut workspace.windows {
                if let Some(ref mut window) = *window_option {
                    if window.id == window_id {
                        window.geometry = geometry;
                        return true;
                    }
                }
            }
        }
        false
    }
    
    /// Set window state
    pub fn set_window_state(&mut self, window_id: WindowID, state: WindowState) -> bool {
        if let Some(ref mut workspace) = self.get_current_workspace_mut() {
            for window_option in &mut workspace.windows {
                if let Some(ref mut window) = *window_option {
                    if window.id == window_id {
                        window.state = state;
                        return true;
                    }
                }
            }
        }
        false
    }
    
    /// Set workspace layout mode
    pub fn set_layout_mode(&mut self, layout_mode: LayoutMode) -> bool {
        if let Some(ref mut workspace) = self.get_current_workspace_mut() {
            workspace.layout_mode = layout_mode;
            self.relayout_workspace();
            true
        } else {
            false
        }
    }
    
    /// Relayout workspace based on current layout mode
    fn relayout_workspace(&mut self) {
        if let Some(ref mut workspace) = self.get_current_workspace_mut() {
            match workspace.layout_mode {
                LayoutMode::Tiling => self.apply_tiling_layout(workspace),
                LayoutMode::Floating => { /* Floating windows don't auto-layout */ }
                LayoutMode::Tabbed => self.apply_tabbed_layout(workspace),
                LayoutMode::Stacked => self.apply_stacked_layout(workspace),
            }
        }
    }
    
    fn apply_tiling_layout(&self, workspace: &mut Workspace) {
        // Simple tiling layout - divide screen horizontally
        let mut window_count = 0;
        for window_option in &workspace.windows {
            if window_option.is_some() {
                window_count += 1;
            }
        }
        
        if window_count == 0 {
            return;
        }
        
        let screen_width = 1920;
        let screen_height = 1080;
        let window_width = screen_width / window_count as u32;
        
        let mut x = 0;
        for window_option in &mut workspace.windows {
            if let Some(ref mut window) = *window_option {
                window.geometry = Geometry::new(x, 0, window_width, screen_height);
                x += window_width as i32;
            }
        }
    }
    
    fn apply_tabbed_layout(&self, workspace: &mut Workspace) {
        // Tabbed layout - all windows full screen, only focused visible
        let screen_width = 1920;
        let screen_height = 1080;
        
        let focused = self.focused_window.load(Ordering::SeqCst);
        
        for window_option in &mut workspace.windows {
            if let Some(ref mut window) = *window_option {
                if window.id == focused {
                    window.geometry = Geometry::new(0, 0, screen_width, screen_height);
                    window.state = WindowState::Normal;
                } else {
                    window.state = WindowState::Hidden;
                }
            }
        }
    }
    
    fn apply_stacked_layout(&self, workspace: &mut Workspace) {
        // Stacked layout - windows stacked vertically
        let screen_width = 1920;
        let screen_height = 1080;
        
        let mut window_count = 0;
        for window_option in &workspace.windows {
            if window_option.is_some() {
                window_count += 1;
            }
        }
        
        if window_count == 0 {
            return;
        }
        
        let window_height = screen_height / window_count as u32;
        let mut y = 0;
        
        for window_option in &mut workspace.windows {
            if let Some(ref mut window) = *window_option {
                window.geometry = Geometry::new(0, y, screen_width, window_height);
                y += window_height as i32;
            }
        }
    }
    
    fn get_current_workspace_mut(&mut self) -> Option<&mut Workspace> {
        for workspace_option in &mut self.workspaces {
            if let Some(ref workspace) = *workspace_option {
                if workspace.id == self.current_workspace {
                    return Some(workspace_option.as_mut().unwrap());
                }
            }
        }
        None
    }
    
    /// Get focused window ID
    pub fn focused_window(&self) -> WindowID {
        self.focused_window.load(Ordering::SeqCst)
    }
    
    /// Get current workspace ID
    pub fn current_workspace(&self) -> u64 {
        self.current_workspace
    }
}

/// Simple Vec implementation for no_std
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * core::mem::size_of::<T>()) as *mut T;

        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }

            if self.capacity > 0 {
                free(self.data as *mut u8);
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

/// Get current time (nanoseconds)
fn get_current_time() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    COUNTER.fetch_add(1_000_000, Ordering::SeqCst)
}
