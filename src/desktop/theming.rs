#![no_std]

/// Declarative Theming Engine for SigmaOS
/// Based on 100-Improvement-Ideas.md #44: Declarative theming engine
/// Implements theme management with declarative syntax

use core::sync::atomic::{AtomicU64, Ordering};

/// Color
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }
    
    pub fn from_hex(hex: u32) -> Self {
        Color {
            r: ((hex >> 24) & 0xFF) as u8,
            g: ((hex >> 16) & 0xFF) as u8,
            b: ((hex >> 8) & 0xFF) as u8,
            a: (hex & 0xFF) as u8,
        }
    }
}

/// Theme color role
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeColorRole {
    Background = 0,
    Foreground = 1,
    Primary = 2,
    Secondary = 3,
    Accent = 4,
    Success = 5,
    Warning = 6,
    Error = 7,
}

/// Theme
#[repr(C)]
pub struct Theme {
    pub id: u64,
    pub name: [u8; 64],
    pub colors: [Color; 8],
    pub font_family: [u8; 32],
    pub font_size: u8,
    pub border_radius: u8,
    pub enabled: bool,
}

impl Theme {
    pub fn new(id: u64, name: &str) -> Self {
        let mut name_array = [0u8; 64];
        let name_bytes = name.as_bytes();
        let len = name_bytes.len().min(63);
        
        unsafe {
            core::ptr::copy_nonoverlapping(name_bytes.as_ptr(), name_array.as_mut_ptr(), len);
        }
        
        Theme {
            id,
            name: name_array,
            colors: [
                Color::new(30, 30, 30, 255),   // Background
                Color::new(255, 255, 255, 255), // Foreground
                Color::new(0, 120, 215, 255),   // Primary
                Color::new(108, 117, 125, 255), // Secondary
                Color::new(255, 193, 7, 255),   // Accent
                Color::new(40, 167, 69, 255),   // Success
                Color::new(255, 193, 7, 255),   // Warning
                Color::new(220, 53, 69, 255),   // Error
            ],
            font_family: [0u8; 32],
            font_size: 14,
            border_radius: 4,
            enabled: false,
        }
    }
    
    pub fn set_color(&mut self, role: ThemeColorRole, color: Color) {
        self.colors[role as usize] = color;
    }
    
    pub fn get_color(&self, role: ThemeColorRole) -> Color {
        self.colors[role as usize]
    }
    
    pub fn set_font_family(&mut self, font: &str) {
        let font_bytes = font.as_bytes();
        let len = font_bytes.len().min(31);
        
        unsafe {
            core::ptr::copy_nonoverlapping(font_bytes.as_ptr(), self.font_family.as_mut_ptr(), len);
        }
    }
}

/// Theme engine
pub struct ThemeEngine {
    pub themes: Vec<Option<Theme>>,
    pub active_theme: AtomicU64,
    pub next_theme_id: AtomicU64,
}

impl ThemeEngine {
    pub fn new() -> Self {
        ThemeEngine {
            themes: Vec::new(),
            active_theme: AtomicU64::new(0),
            next_theme_id: AtomicU64::new(1),
        }
    }
    
    /// Create theme
    pub fn create_theme(&mut self, name: &str) -> u64 {
        let id = self.next_theme_id.fetch_add(1, Ordering::SeqCst);
        let theme = Theme::new(id, name);
        self.themes.push(Some(theme));
        id
    }
    
    /// Delete theme
    pub fn delete_theme(&mut self, id: u64) -> bool {
        let active_id = self.active_theme.load(Ordering::SeqCst);
        if active_id == id {
            return false; // Cannot delete active theme
        }
        
        for theme_option in &mut self.themes {
            if let Some(ref theme) = *theme_option {
                if theme.id == id {
                    *theme_option = None;
                    return true;
                }
            }
        }
        false
    }
    
    /// Set active theme
    pub fn set_active_theme(&mut self, id: u64) -> Result<(), ThemeError> {
        // Deactivate current theme
        let current_id = self.active_theme.load(Ordering::SeqCst);
        if current_id > 0 {
            for theme_option in &mut self.themes {
                if let Some(ref mut theme) = *theme_option {
                    if theme.id == current_id {
                        theme.enabled = false;
                    }
                }
            }
        }
        
        // Activate new theme
        for theme_option in &mut self.themes {
            if let Some(ref mut theme) = *theme_option {
                if theme.id == id {
                    theme.enabled = true;
                    self.active_theme.store(id, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        
        Err(ThemeError::ThemeNotFound)
    }
    
    /// Get active theme
    pub fn get_active_theme(&self) -> Option<&Theme> {
        let active_id = self.active_theme.load(Ordering::SeqCst);
        if active_id == 0 {
            return None;
        }
        
        for theme_option in &self.themes {
            if let Some(ref theme) = *theme_option {
                if theme.id == active_id {
                    return Some(theme);
                }
            }
        }
        None
    }
    
    /// Get theme by ID
    pub fn get_theme(&self, id: u64) -> Option<&Theme> {
        for theme_option in &self.themes {
            if let Some(ref theme) = *theme_option {
                if theme.id == id {
                    return Some(theme);
                }
            }
        }
        None
    }
    
    /// List themes
    pub fn list_themes(&self) -> Vec<&Theme> {
        let mut themes = Vec::new();
        for theme_option in &self.themes {
            if let Some(ref theme) = *theme_option {
                themes.push(theme);
            }
        }
        themes
    }
    
    /// Apply theme color
    pub fn apply_color(&self, role: ThemeColorRole) -> Color {
        if let Some(theme) = self.get_active_theme() {
            theme.get_color(role)
        } else {
            Color::new(0, 0, 0, 255)
        }
    }
    
    /// Initialize default themes
    pub fn initialize_defaults(&mut self) {
        // Dark theme
        let dark_id = self.create_theme("Dark");
        if let Some(ref mut theme) = self.themes.iter_mut().find_map(|t| t.as_mut()) {
            if theme.id == dark_id {
                theme.set_color(ThemeColorRole::Background, Color::new(30, 30, 30, 255));
                theme.set_color(ThemeColorRole::Foreground, Color::new(255, 255, 255, 255));
            }
        }
        
        // Light theme
        let light_id = self.create_theme("Light");
        if let Some(ref mut theme) = self.themes.iter_mut().find_map(|t| t.as_mut()) {
            if theme.id == light_id {
                theme.set_color(ThemeColorRole::Background, Color::new(255, 255, 255, 255));
                theme.set_color(ThemeColorRole::Foreground, Color::new(30, 30, 30, 255));
            }
        }
    }
}

/// Theme error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ThemeError {
    Success = 0,
    ThemeNotFound = 1,
    CannotDeleteActive = 2,
    InvalidColor = 3,
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
