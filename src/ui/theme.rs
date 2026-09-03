#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

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
    pub name_len: u8,
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
            name_len: name_len as u8,
            colors: Vec::new(),
        }
    }
}

impl Theme for SimpleTheme {
    fn id(&self) -> ThemeID { self.id }
    fn name(&self) -> &[u8] {
        // O(1) slice lookup using cached name_len, avoiding O(N) zero-byte linear scan (.position(|&b| b == 0))
        &self.name[..self.name_len as usize]
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
    #[allow(clippy::new_without_default)]
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

// --- Distro-Inspired UI/UX Theme Engine ---
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistroDesktopThemePreset {
    UbuntuYaruDark,
    PopOsTealDark,
    LinuxMintMintY,
    FedoraAdwaitaDark,
    ElementaryOsGranite,
    FreeBsdDaemonDark,
    OpenBsdOnyx,
    ArchCyanDark,
}

#[derive(Debug, Clone)]
pub struct DistroAccentPalette {
    pub primary_accent_rgba: u32,
    pub secondary_accent_rgba: u32,
    pub background_rgba: u32,
    pub surface_rgba: u32,
    pub text_rgba: u32,
    pub accent_hex_code: String,
}

pub struct SovereignThemeEngine {
    pub active_preset: DistroDesktopThemePreset,
    pub is_dark_mode: bool,
    pub high_contrast_enabled: bool,
    pub font_scale_factor: f32,
    pub corner_radius_px: u32,
}

impl SovereignThemeEngine {
    pub fn new() -> Self {
        Self {
            active_preset: DistroDesktopThemePreset::UbuntuYaruDark,
            is_dark_mode: true,
            high_contrast_enabled: false,
            font_scale_factor: 1.0,
            corner_radius_px: 8,
        }
    }

    pub fn set_preset(&mut self, preset: DistroDesktopThemePreset) {
        self.active_preset = preset;
        match preset {
            DistroDesktopThemePreset::UbuntuYaruDark |
            DistroDesktopThemePreset::PopOsTealDark |
            DistroDesktopThemePreset::FedoraAdwaitaDark |
            DistroDesktopThemePreset::FreeBsdDaemonDark |
            DistroDesktopThemePreset::OpenBsdOnyx |
            DistroDesktopThemePreset::ArchCyanDark => {
                self.is_dark_mode = true;
            }
            DistroDesktopThemePreset::LinuxMintMintY |
            DistroDesktopThemePreset::ElementaryOsGranite => {
                self.is_dark_mode = false;
            }
        }
    }

    pub fn get_palette(&self) -> DistroAccentPalette {
        match self.active_preset {
            DistroDesktopThemePreset::UbuntuYaruDark => DistroAccentPalette {
                primary_accent_rgba: 0xE95420FF,
                secondary_accent_rgba: 0x77216F_FF,
                background_rgba: 0x111111FF,
                surface_rgba: 0x222222FF,
                text_rgba: 0xFFFFFFFF,
                accent_hex_code: "#E95420".to_string(),
            },
            DistroDesktopThemePreset::PopOsTealDark => DistroAccentPalette {
                primary_accent_rgba: 0x48B9C7FF,
                secondary_accent_rgba: 0xF08250FF,
                background_rgba: 0x1E1E1EFF,
                surface_rgba: 0x2D2D2DFF,
                text_rgba: 0xF6F6F6FF,
                accent_hex_code: "#48B9C7".to_string(),
            },
            DistroDesktopThemePreset::LinuxMintMintY => DistroAccentPalette {
                primary_accent_rgba: 0x2A9D8FFF,
                secondary_accent_rgba: 0x9B51E0FF,
                background_rgba: 0xF5F5F5FF,
                surface_rgba: 0xFFFFFFFF,
                text_rgba: 0x222222FF,
                accent_hex_code: "#2A9D8F".to_string(),
            },
            DistroDesktopThemePreset::FedoraAdwaitaDark => DistroAccentPalette {
                primary_accent_rgba: 0x3584E4FF,
                secondary_accent_rgba: 0x1C71D8FF,
                background_rgba: 0x1E1E1EFF,
                surface_rgba: 0x303030FF,
                text_rgba: 0xFFFFFFFF,
                accent_hex_code: "#3584E4".to_string(),
            },
            DistroDesktopThemePreset::ElementaryOsGranite => DistroAccentPalette {
                primary_accent_rgba: 0x388E3CFF,
                secondary_accent_rgba: 0x0288D1FF,
                background_rgba: 0xFAFAFAFF,
                surface_rgba: 0xFFFFFFFF,
                text_rgba: 0x333333FF,
                accent_hex_code: "#388E3C".to_string(),
            },
            DistroDesktopThemePreset::FreeBsdDaemonDark => DistroAccentPalette {
                primary_accent_rgba: 0xAB1212FF,
                secondary_accent_rgba: 0x880E0EFF,
                background_rgba: 0x121212FF,
                surface_rgba: 0x1E1E1EFF,
                text_rgba: 0xEEEEEEFF,
                accent_hex_code: "#AB1212".to_string(),
            },
            DistroDesktopThemePreset::OpenBsdOnyx => DistroAccentPalette {
                primary_accent_rgba: 0x222222FF,
                secondary_accent_rgba: 0x444444FF,
                background_rgba: 0x0A0A0AFF,
                surface_rgba: 0x161616FF,
                text_rgba: 0xDDDDDDFF,
                accent_hex_code: "#222222".to_string(),
            },
            DistroDesktopThemePreset::ArchCyanDark => DistroAccentPalette {
                primary_accent_rgba: 0x1793D1FF,
                secondary_accent_rgba: 0x0D6EFDFF,
                background_rgba: 0x0F1419FF,
                surface_rgba: 0x1A232AFF,
                text_rgba: 0xE6EDF3FF,
                accent_hex_code: "#1793D1".to_string(),
            },
        }
    }

    pub fn toggle_dark_mode(&mut self) {
        self.is_dark_mode = !self.is_dark_mode;
    }
}

impl Default for SovereignThemeEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distro_desktop_theme_presets() {
        let mut engine = SovereignThemeEngine::new();
        assert_eq!(engine.active_preset, DistroDesktopThemePreset::UbuntuYaruDark);
        assert!(engine.is_dark_mode);

        engine.set_preset(DistroDesktopThemePreset::PopOsTealDark);
        assert_eq!(engine.active_preset, DistroDesktopThemePreset::PopOsTealDark);
        assert!(engine.is_dark_mode);

        engine.set_preset(DistroDesktopThemePreset::LinuxMintMintY);
        assert_eq!(engine.active_preset, DistroDesktopThemePreset::LinuxMintMintY);
        assert!(!engine.is_dark_mode);
    }

    #[test]
    fn test_accent_color_palette_queries() {
        let mut engine = SovereignThemeEngine::new();
        let ubuntu_palette = engine.get_palette();
        assert_eq!(ubuntu_palette.accent_hex_code, "#E95420");
        assert_eq!(ubuntu_palette.primary_accent_rgba, 0xE95420FF);

        engine.set_preset(DistroDesktopThemePreset::ArchCyanDark);
        let arch_palette = engine.get_palette();
        assert_eq!(arch_palette.accent_hex_code, "#1793D1");
        assert_eq!(arch_palette.primary_accent_rgba, 0x1793D1FF);
    }

    #[test]
    fn test_dark_light_mode_switching() {
        let mut engine = SovereignThemeEngine::new();
        assert!(engine.is_dark_mode);

        engine.toggle_dark_mode();
        assert!(!engine.is_dark_mode);

        engine.toggle_dark_mode();
        assert!(engine.is_dark_mode);
    }
}
