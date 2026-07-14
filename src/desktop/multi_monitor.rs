#![no_std]

/// Multi-Monitor Manager for SigmaOS
/// Based on 100-Improvement-Ideas.md #46: Multi-monitor manager
/// Implements display configuration and multi-monitor management

use core::sync::atomic::{AtomicU64, Ordering};

/// Display ID type
pub type DisplayID = u64;

/// Display connection type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    HDMI = 0,
    DisplayPort = 1,
    VGA = 2,
    DVI = 3,
    USB = 4,
    Wireless = 5,
}

/// Display orientation
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayOrientation {
    Normal = 0,
    Rotated90 = 1,
    Rotated180 = 2,
    Rotated270 = 3,
}

/// Display mode
#[repr(C)]
pub struct DisplayMode {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
}

impl DisplayMode {
    pub fn new(width: u32, height: u32, refresh_rate: u32) -> Self {
        DisplayMode {
            width,
            height,
            refresh_rate,
        }
    }
}

/// Display configuration
#[repr(C)]
pub struct DisplayConfig {
    pub id: DisplayID,
    pub name: [u8; 64],
    pub connection_type: ConnectionType,
    pub current_mode: DisplayMode,
    pub preferred_mode: DisplayMode,
    pub orientation: DisplayOrientation,
    pub is_primary: bool,
    pub is_enabled: bool,
    pub position_x: i32,
    pub position_y: i32,
}

impl DisplayConfig {
    pub fn new(id: DisplayID, name: &str, connection_type: ConnectionType) -> Self {
        let mut name_array = [0u8; 64];
        let name_bytes = name.as_bytes();
        let len = name_bytes.len().min(63);
        
        unsafe {
            core::ptr::copy_nonoverlapping(name_bytes.as_ptr(), name_array.as_mut_ptr(), len);
        }
        
        let default_mode = DisplayMode::new(1920, 1080, 60);
        
        DisplayConfig {
            id,
            name: name_array,
            connection_type,
            current_mode: default_mode,
            preferred_mode: default_mode,
            orientation: DisplayOrientation::Normal,
            is_primary: false,
            is_enabled: true,
            position_x: 0,
            position_y: 0,
        }
    }
    
    pub fn name_str(&self) -> &str {
        unsafe {
            let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
            core::str::from_utf8_unchecked(&self.name[..len])
        }
    }
    
    pub fn set_mode(&mut self, mode: DisplayMode) {
        self.current_mode = mode;
    }
    
    pub fn set_orientation(&mut self, orientation: DisplayOrientation) {
        self.orientation = orientation;
    }
    
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.position_x = x;
        self.position_y = y;
    }
    
    pub fn set_primary(&mut self, is_primary: bool) {
        self.is_primary = is_primary;
    }
    
    pub fn enable(&mut self) {
        self.is_enabled = true;
    }
    
    pub fn disable(&mut self) {
        self.is_enabled = false;
    }
}

/// Multi-monitor manager
pub struct MultiMonitorManager {
    pub displays: Vec<Option<DisplayConfig>>,
    pub primary_display: AtomicU64,
    pub next_display_id: AtomicU64,
}

impl MultiMonitorManager {
    pub fn new() -> Self {
        MultiMonitorManager {
            displays: Vec::new(),
            primary_display: AtomicU64::new(0),
            next_display_id: AtomicU64::new(1),
        }
    }
    
    /// Add display
    pub fn add_display(&mut self, name: &str, connection_type: ConnectionType) -> DisplayID {
        let id = self.next_display_id.fetch_add(1, Ordering::SeqCst);
        let display = DisplayConfig::new(id, name, connection_type);
        
        // Set as primary if first display
        if self.displays.is_empty() {
            display.is_primary = true;
            self.primary_display.store(id, Ordering::SeqCst);
        }
        
        self.displays.push(Some(display));
        id
    }
    
    /// Remove display
    pub fn remove_display(&mut self, display_id: DisplayID) -> Result<(), DisplayError> {
        for display_option in &mut self.displays {
            if let Some(ref display) = *display_option {
                if display.id == display_id {
                    if display.is_primary {
                        return Err(DisplayError::CannotRemovePrimary);
                    }
                    *display_option = None;
                    return Ok(());
                }
            }
        }
        Err(DisplayError::DisplayNotFound)
    }
    
    /// Set primary display
    pub fn set_primary(&mut self, display_id: DisplayID) -> Result<(), DisplayError> {
        // Unset current primary
        let current_primary = self.primary_display.load(Ordering::SeqCst);
        if current_primary > 0 {
            for display_option in &mut self.displays {
                if let Some(ref mut display) = *display_option {
                    if display.id == current_primary {
                        display.set_primary(false);
                    }
                }
            }
        }
        
        // Set new primary
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id == display_id {
                    display.set_primary(true);
                    self.primary_display.store(display_id, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        
        Err(DisplayError::DisplayNotFound)
    }
    
    /// Get primary display
    pub fn get_primary(&self) -> Option<&DisplayConfig> {
        let primary_id = self.primary_display.load(Ordering::SeqCst);
        if primary_id == 0 {
            return None;
        }
        
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id == primary_id {
                    return Some(display);
                }
            }
        }
        None
    }
    
    /// Get display by ID
    pub fn get_display(&self, display_id: DisplayID) -> Option<&DisplayConfig> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id == display_id {
                    return Some(display);
                }
            }
        }
        None
    }
    
    /// List all displays
    pub fn list_displays(&self) -> Vec<DisplayID> {
        let mut ids = Vec::new();
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                ids.push(display.id);
            }
        }
        ids
    }
    
    /// Configure display mode
    pub fn configure_display(&mut self, display_id: DisplayID, mode: DisplayMode) -> Result<(), DisplayError> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id == display_id {
                    display.set_mode(mode);
                    return Ok(());
                }
            }
        }
        Err(DisplayError::DisplayNotFound)
    }
    
    /// Arrange displays in grid
    pub fn arrange_grid(&mut self, columns: u32) -> Result<(), DisplayError> {
        let mut enabled_displays: Vec<&mut DisplayConfig> = Vec::new();
        
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.is_enabled {
                    enabled_displays.push(display);
                }
            }
        }
        
        let mut x = 0;
        let mut y = 0;
        let mut col = 0;
        
        for display in &mut enabled_displays {
            display.set_position(x, y);
            display.set_orientation(DisplayOrientation::Normal);
            
            col += 1;
            if col >= columns {
                col = 0;
                x = 0;
                y += display.current_mode.height as i32;
            } else {
                x += display.current_mode.width as i32;
            }
        }
        
        Ok(())
    }
    
    /// Mirror displays
    pub fn mirror_displays(&mut self, source_id: DisplayID) -> Result<(), DisplayError> {
        let source_mode = match self.get_display(source_id) {
            Some(display) => display.current_mode,
            None => return Err(DisplayError::DisplayNotFound),
        };
        
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id != source_id && display.is_enabled {
                    display.set_mode(source_mode);
                    display.set_position(0, 0);
                }
            }
        }
        
        Ok(())
    }
}

/// Display error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DisplayError {
    Success = 0,
    DisplayNotFound = 1,
    CannotRemovePrimary = 2,
    InvalidMode = 3,
    ConfigurationFailed = 4,
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

    fn is_empty(&self) -> bool {
        self.len == 0
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
