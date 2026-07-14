#![no_std]
#![no_main]

/// OOP-based Theme System for SigmaOS
/// Based on Ideas-999-Structured: User Experience & Desktop Item 706
/// Implements theme management and color schemes

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ThemeID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ThemeError { Success = 0, NotFound = 1, InvalidColor = 2 }

pub trait Color {
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;
    fn a(&self) -> u8;
    fn to_rgba(&self) -> u32;
}

#[repr(C)]
pub struct SimpleColor {
    pub r: AtomicUsize,
    pub g: AtomicUsize,
    pub b: AtomicUsize,
    pub a: AtomicUsize,
}

impl SimpleColor {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        SimpleColor {
            r: AtomicUsize::new(r as usize),
            g: AtomicUsize::new(g as usize),
            b: AtomicUsize::new(b as usize),
            a: AtomicUsize::new(a as usize),
        }
    }
}

impl Color for SimpleColor {
    fn r(&self) -> u8 { self.r.load(Ordering::SeqCst) as u8 }
    fn g(&self) -> u8 { self.g.load(Ordering::SeqCst) as u8 }
    fn b(&self) -> u8 { self.b.load(Ordering::SeqCst) as u8 }
    fn a(&self) -> u8 { self.a.load(Ordering::SeqCst) as u8 }

    fn to_rgba(&self) -> u32 {
        (self.r() as u32) << 24 | (self.g() as u32) << 16 | (self.b() as u32) << 8 | self.a() as u32
    }
}

pub trait Theme {
    fn id(&self) -> ThemeID;
    fn name(&self) -> &[u8];
    fn get_color(&self, color_name: &[u8]) -> Option<&dyn Color>;
    fn set_color(&mut self, color_name: &[u8], color: Box<dyn Color>) -> Result<(), ThemeError>;
}

#[repr(C)]
pub struct SimpleTheme {
    pub id: ThemeID,
    pub name: [u8; 64],
    pub colors: Vec<([u8; 32], Option<Box<dyn Color>>)>,
}

impl SimpleTheme {
    pub fn new(id: ThemeID, name: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleTheme {
            id,
            name: name_array,
            colors: Vec::new(),
        }
    }
}

impl Theme for SimpleTheme {
    fn id(&self) -> ThemeID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }

    fn get_color(&self, color_name: &[u8]) -> Option<&dyn Color> {
        for &(ref name, ref color_option) in &self.colors {
            let name_len = name.iter().position(|&b| b == 0).unwrap_or(32);
            if &name[..name_len] == color_name {
                if let Some(ref color) = *color_option {
                    return Some(color.as_ref());
                }
            }
        }
        None
    }

    fn set_color(&mut self, color_name: &[u8], color: Box<dyn Color>) -> Result<(), ThemeError> {
        let mut name_array = [0u8; 32];
        let name_len = color_name.len().min(31);
        for i in 0..name_len {
            name_array[i] = color_name[i];
        }
        self.colors.push((name_array, Some(color)));
        Ok(())
    }
}

pub trait ThemeManager {
    fn register_theme(&mut self, theme: Box<dyn Theme>) -> Result<ThemeID, ThemeError>;
    fn get_theme(&self, id: ThemeID) -> Option<&dyn Theme>;
    fn set_active_theme(&mut self, id: ThemeID) -> Result<(), ThemeError>;
    fn get_active_theme(&self) -> Option<&dyn Theme>;
}

#[repr(C)]
pub struct SimpleThemeManager {
    pub themes: Vec<Option<Box<dyn Theme>>>,
    pub active: AtomicUsize,
    pub next_id: AtomicUsize,
}

impl SimpleThemeManager {
    pub fn new() -> Self {
        SimpleThemeManager {
            themes: Vec::new(),
            active: AtomicUsize::new(0),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ThemeManager for SimpleThemeManager {
    fn register_theme(&mut self, theme: Box<dyn Theme>) -> Result<ThemeID, ThemeError> {
        let id = theme.id();
        self.themes.push(Some(theme));
        Ok(id)
    }

    fn get_theme(&self, id: ThemeID) -> Option<&dyn Theme> {
        for theme_option in &self.themes {
            if let Some(ref theme) = *theme_option {
                if theme.id() == id { return Some(theme.as_ref()); }
            }
        }
        None
    }

    fn set_active_theme(&mut self, id: ThemeID) -> Result<(), ThemeError> {
        for theme_option in &self.themes {
            if let Some(ref theme) = *theme_option {
                if theme.id() == id {
                    self.active.store(id, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ThemeError::NotFound)
    }

    fn get_active_theme(&self) -> Option<&dyn Theme> {
        let active_id = self.active.load(Ordering::SeqCst);
        if active_id > 0 {
            self.get_theme(active_id)
        } else {
            None
        }
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
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
