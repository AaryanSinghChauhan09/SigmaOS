#![no_std]
#![no_main]

/// OOP-based Application Launcher for SigmaOS
/// Implements application launcher, search, and iconic Elementary OS / Pantheon desktop subsystems.
/// Inspired by Plank, Slingshot, Wingpanel, Gala, and Pantheon-Files.

extern crate alloc;
use alloc::boxed::Box;

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type AppID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LauncherError { Success = 0, NotFound = 1, LaunchFailed = 2, AlreadyExists = 3 }

pub trait Application {
    fn id(&self) -> AppID;
    fn name(&self) -> &[u8];
    fn executable(&self) -> &[u8];
    fn icon(&self) -> &[u8];
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SimpleApplication {
    pub id: AppID,
    pub name: [u8; 64],
    pub executable: [u8; 256],
    pub icon: [u8; 256],
}

impl SimpleApplication {
    pub fn new(id: AppID, name: &[u8], executable: &[u8], icon: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let mut exec_array = [0u8; 256];
        let mut icon_array = [0u8; 256];
        let name_len = name.len().min(63);
        let exec_len = executable.len().min(255);
        let icon_len = icon.len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
            core::ptr::copy_nonoverlapping(executable.as_ptr(), exec_array.as_mut_ptr(), exec_len);
            core::ptr::copy_nonoverlapping(icon.as_ptr(), icon_array.as_mut_ptr(), icon_len);
        }
        SimpleApplication {
            id,
            name: name_array,
            executable: exec_array,
            icon: icon_array,
        }
    }
}

impl Application for SimpleApplication {
    fn id(&self) -> AppID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn executable(&self) -> &[u8] {
        let len = self.executable.iter().position(|&b| b == 0).unwrap_or(256);
        &self.executable[..len]
    }
    fn icon(&self) -> &[u8] {
        let len = self.icon.iter().position(|&b| b == 0).unwrap_or(256);
        &self.icon[..len]
    }
}

pub trait ApplicationLauncher {
    fn register_app(&mut self, app: Box<dyn Application>) -> Result<AppID, LauncherError>;
    fn unregister_app(&mut self, id: AppID) -> Result<(), LauncherError>;
    fn launch_app(&self, id: AppID) -> Result<(), LauncherError>;
    fn search_apps(&self, query: &[u8]) -> Vec<AppID>;
    fn get_app(&self, id: AppID) -> Option<&dyn Application>;
}

#[repr(C)]
pub struct SimpleApplicationLauncher {
    pub apps: Vec<Option<Box<dyn Application>>>,
    pub next_id: AtomicUsize,
}

impl SimpleApplicationLauncher {
    pub fn new() -> Self {
        SimpleApplicationLauncher {
            apps: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ApplicationLauncher for SimpleApplicationLauncher {
    fn register_app(&mut self, app: Box<dyn Application>) -> Result<AppID, LauncherError> {
        let id = app.id();
        self.apps.push(Some(app));
        Ok(id)
    }
    
    fn unregister_app(&mut self, id: AppID) -> Result<(), LauncherError> {
        for app_option in &mut self.apps {
            if let Some(ref app) = *app_option {
                let app_ref: &dyn Application = app.as_ref();
                if app_ref.id() == id {
                    *app_option = None;
                    return Ok(());
                }
            }
        }
        Err(LauncherError::NotFound)
    }
    
    fn launch_app(&self, id: AppID) -> Result<(), LauncherError> {
        if self.get_app(id).is_some() {
            Ok(())
        } else {
            Err(LauncherError::NotFound)
        }
    }
    
    fn search_apps(&self, query: &[u8]) -> Vec<AppID> {
        let mut results = Vec::new();
        for app_option in &self.apps {
            if let Some(ref app) = *app_option {
                let app_ref: &dyn Application = app.as_ref();
                if app_ref.name().starts_with(query) {
                    results.push(app_ref.id());
                }
            }
        }
        results
    }
    
    fn get_app(&self, id: AppID) -> Option<&dyn Application> {
        for app_option in &self.apps {
            if let Some(ref app) = *app_option {
                let app_ref: &dyn Application = app.as_ref();
                if app_ref.id() == id { return Some(app_ref); }
            }
        }
        None
    }
}

pub trait RecentApps {
    fn add_recent(&mut self, app_id: AppID);
    fn get_recent(&self) -> Vec<AppID>;
}

#[repr(C)]
pub struct SimpleRecentApps {
    pub recent: Vec<AppID>,
}

impl SimpleRecentApps {
    pub fn new() -> Self {
        SimpleRecentApps {
            recent: Vec::new(),
        }
    }
}

impl RecentApps for SimpleRecentApps {
    fn add_recent(&mut self, app_id: AppID) {
        self.recent.push(app_id);
        if self.recent.len() > 10 {
            self.recent.remove(0);
        }
    }
    
    fn get_recent(&self) -> Vec<AppID> {
        let mut cloned = Vec::new();
        for &id in &self.recent {
            cloned.push(id);
        }
        cloned
    }
}

// ==============================================================================
// 1. Slingshot Application Launcher (Paginated Grid, Categories, Search)
// ==============================================================================
#[repr(C)]
pub struct SlingshotLauncher {
    pub current_page: u32,
    pub items_per_page: u32,
    pub selected_category: [u8; 32], // e.g. "System", "Internet", "Office"
}

impl SlingshotLauncher {
    pub fn new() -> Self {
        Self {
            current_page: 0,
            items_per_page: 12, // 4x3 app grid
            selected_category: [0; 32],
        }
    }

    pub fn set_category(&mut self, cat: &[u8]) {
        let len = cat.len().min(31);
        self.selected_category[..len].copy_from_slice(&cat[..len]);
        self.selected_category[len] = 0;
    }

    pub fn paginate(&mut self, forward: bool) {
        if forward {
            self.current_page += 1;
        } else if self.current_page > 0 {
            self.current_page -= 1;
        }
    }
}

impl Default for SlingshotLauncher {
    fn default() -> Self {
        Self::new()
    }
}

// ==============================================================================
// 2. Plank Desktop Application Dock (Bottom Launch Bar)
// ==============================================================================
#[repr(C)]
pub struct PlankDock {
    pub pinned_apps: Vec<AppID>,
    pub running_apps: Vec<AppID>,
    pub zoom_level_percent: u32, // Magnification hover effect
    pub active_indicator_id: Option<AppID>,
}

impl PlankDock {
    pub fn new() -> Self {
        Self {
            pinned_apps: Vec::new(),
            running_apps: Vec::new(),
            zoom_level_percent: 100, // 100% standard scaling
            active_indicator_id: None,
        }
    }

    pub fn pin_app(&mut self, app_id: AppID) -> bool {
        self.pinned_apps.push(app_id);
        true
    }

    pub fn set_hover_zoom(&mut self, hover_index: usize) {
        let _ = hover_index;
        self.zoom_level_percent = 130; // Scale up hovered icon by 130%
    }
}

impl Default for PlankDock {
    fn default() -> Self {
        Self::new()
    }
}

// ==============================================================================
// 3. Wingpanel Top System Panel (Tray Indicators)
// ==============================================================================
#[repr(C)]
pub struct Wingpanel {
    pub volume_level: u32,
    pub is_wifi_connected: bool,
    pub battery_percent: u32,
    pub translucent_opacity_percent: u32, // wingpanel transitions opacity based on maximizations
}

impl Wingpanel {
    pub fn new() -> Self {
        Self {
            volume_level: 80,
            is_wifi_connected: true,
            battery_percent: 100,
            translucent_opacity_percent: 85, // Translucent default
        }
    }

    pub fn update_opacity(&mut self, window_maximized: bool) {
        if window_maximized {
            self.translucent_opacity_percent = 100; // Solid black on maximized window
        } else {
            self.translucent_opacity_percent = 85;  // Translucent backdrop
        }
    }
}

impl Default for Wingpanel {
    fn default() -> Self {
        Self::new()
    }
}

// ==============================================================================
// 4. Gala Window Manager & Workspaces Multitasking
// ==============================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapLayout { None, LeftHalf, RightHalf, Maximize }

pub struct GalaWorkspace {
    pub id: u32,
    pub mapped_window_ids: Vec<AppID>,
    pub snap_modes: Vec<SnapLayout>,
}

pub struct GalaWindowManager {
    pub workspaces: Vec<GalaWorkspace>,
    pub active_workspace_idx: usize,
}

impl GalaWindowManager {
    pub fn new() -> Self {
        let mut spaces = Vec::new();
        spaces.push(GalaWorkspace {
            id: 1,
            mapped_window_ids: Vec::new(),
            snap_modes: Vec::new(),
        });
        Self {
            workspaces: spaces,
            active_workspace_idx: 0,
        }
    }

    pub fn add_workspace(&mut self) -> usize {
        let new_id = (self.workspaces.len() + 1) as u32;
        self.workspaces.push(GalaWorkspace {
            id: new_id,
            mapped_window_ids: Vec::new(),
            snap_modes: Vec::new(),
        });
        self.workspaces.len() - 1
    }

    pub fn switch_workspace(&mut self, index: usize) -> bool {
        if index < self.workspaces.len() {
            self.active_workspace_idx = index;
            true
        } else {
            false
        }
    }
}

impl Default for GalaWindowManager {
    fn default() -> Self {
        Self::new()
    }
}

// ==============================================================================
// 5. Pantheon Files (Column Grid File Manager Tabs)
// ==============================================================================
#[derive(Clone, Copy)]
pub struct PantheonTab {
    pub path_hash: u64,
    pub is_column_view: bool,
}

pub struct PantheonFileManager {
    pub open_tabs: Vec<PantheonTab>,
    pub active_tab_idx: usize,
    pub favorite_folders_hashes: Vec<u64>,
}

impl PantheonFileManager {
    pub fn new() -> Self {
        let mut tabs = Vec::new();
        tabs.push(PantheonTab {
            path_hash: 0x1000, // Home directory /root
            is_column_view: true,
        });
        Self {
            open_tabs: tabs,
            active_tab_idx: 0,
            favorite_folders_hashes: Vec::new(),
        }
    }

    pub fn open_new_tab(&mut self, path: u64) -> usize {
        self.open_tabs.push(PantheonTab {
            path_hash: path,
            is_column_view: true,
        });
        self.open_tabs.len() - 1
    }
}

impl Default for PantheonFileManager {
    fn default() -> Self {
        Self::new()
    }
}

// ==============================================================================
// Vec Implementation
// ==============================================================================
pub struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    pub fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    pub fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    pub fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
        }
    }
    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }


impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}


impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}
