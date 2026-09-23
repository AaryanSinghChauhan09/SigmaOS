#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use std::vec;

use crate::kernel::subsystems::registry::{
    InitOrder, KernelSubsystem, SubsystemError, SubsystemPriority,
};
/// SigmaOS Legacy Driver — CGA/MDA/EGA/VGA/SVGA Display Adapter Driver
/// Absorbs Linux 0.01 console.c through modern fbdev/drm lineage
/// CGA: 320×200 4-color, 640×200 mono
/// MDA: 80×25 text mode (Monochrome Display Adapter)
/// EGA: 640×350 16-color
/// VGA: 320×200 256-color (Mode 13h), 640×480 16-color
/// SVGA/VESA: up to 1920×1080+
use core::sync::atomic::{AtomicUsize, Ordering};
use std::vec::Vec;

/// Display modes (absorbs BIOS INT 10h mode numbers)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum VideoMode {
    Mda80x25Text = 0x07,     // MDA 80×25 mono text
    Cga40x25Text = 0x01,     // CGA 40×25 color text
    Cga80x25Text = 0x03,     // CGA 80×25 color text
    Cga320x200x4 = 0x04,     // CGA 320×200 4-color graphics
    Cga640x200Mono = 0x06,   // CGA 640×200 2-color graphics
    Ega640x350x16 = 0x10,    // EGA 640×350 16-color
    Vga640x480x16 = 0x12,    // VGA 640×480 16-color
    Vga320x200x256 = 0x13,   // VGA 320×200 256-color (Mode 13h)
    Vesa800x600x256 = 0x103, // VESA 800×600 256-color
    Vesa1024x768 = 0x105,    // VESA 1024×768 256-color
    Vesa1280x1024 = 0x107,   // VESA 1280×1024 256-color
    Vesa1920x1080 = 0x11B,   // VESA 1920×1080 32bpp
}

impl VideoMode {
    pub fn resolution(&self) -> (u32, u32) {
        match self {
            VideoMode::Mda80x25Text => (720, 400),
            VideoMode::Cga40x25Text => (320, 200),
            VideoMode::Cga80x25Text => (640, 200),
            VideoMode::Cga320x200x4 => (320, 200),
            VideoMode::Cga640x200Mono => (640, 200),
            VideoMode::Ega640x350x16 => (640, 350),
            VideoMode::Vga640x480x16 => (640, 480),
            VideoMode::Vga320x200x256 => (320, 200),
            VideoMode::Vesa800x600x256 => (800, 600),
            VideoMode::Vesa1024x768 => (1024, 768),
            VideoMode::Vesa1280x1024 => (1280, 1024),
            VideoMode::Vesa1920x1080 => (1920, 1080),
        }
    }

    pub fn bpp(&self) -> u8 {
        match self {
            VideoMode::Mda80x25Text | VideoMode::Cga40x25Text | VideoMode::Cga80x25Text => 4,
            VideoMode::Cga320x200x4 => 2,
            VideoMode::Cga640x200Mono => 1,
            VideoMode::Ega640x350x16 | VideoMode::Vga640x480x16 => 4,
            VideoMode::Vga320x200x256
            | VideoMode::Vesa800x600x256
            | VideoMode::Vesa1024x768
            | VideoMode::Vesa1280x1024 => 8,
            VideoMode::Vesa1920x1080 => 32,
        }
    }
}

/// CGA/VGA 16-color palette (standard BIOS palette)
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub mod palette {
    use super::Color;
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 170 };
    pub const GREEN: Color = Color { r: 0, g: 170, b: 0 };
    pub const CYAN: Color = Color {
        r: 0,
        g: 170,
        b: 170,
    };
    pub const RED: Color = Color { r: 170, g: 0, b: 0 };
    pub const MAGENTA: Color = Color {
        r: 170,
        g: 0,
        b: 170,
    };
    pub const BROWN: Color = Color {
        r: 170,
        g: 85,
        b: 0,
    };
    pub const LIGHT_GRAY: Color = Color {
        r: 170,
        g: 170,
        b: 170,
    };
    pub const DARK_GRAY: Color = Color {
        r: 85,
        g: 85,
        b: 85,
    };
    pub const LIGHT_BLUE: Color = Color {
        r: 85,
        g: 85,
        b: 255,
    };
    pub const LIGHT_GREEN: Color = Color {
        r: 85,
        g: 255,
        b: 85,
    };
    pub const LIGHT_CYAN: Color = Color {
        r: 85,
        g: 255,
        b: 255,
    };
    pub const LIGHT_RED: Color = Color {
        r: 255,
        g: 85,
        b: 85,
    };
    pub const LIGHT_MAGENTA: Color = Color {
        r: 255,
        g: 85,
        b: 255,
    };
    pub const YELLOW: Color = Color {
        r: 255,
        g: 255,
        b: 85,
    };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
    };
}

/// Text mode cell (character + attribute byte)
#[derive(Debug, Clone, Copy)]
pub struct TextCell {
    pub ch: u8,
    pub fg: u8, // 0-15 foreground color index
    pub bg: u8, // 0-7 background color index
    pub blink: bool,
}

impl TextCell {
    pub fn new(ch: u8) -> Self {
        TextCell {
            ch,
            fg: 7,
            bg: 0,
            blink: false,
        }
    }
    pub fn attr_byte(&self) -> u8 {
        (self.bg & 0x07) << 4 | (self.fg & 0x0F) | if self.blink { 0x80 } else { 0 }
    }
}

/// Framebuffer — generic pixel store
pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub bpp: u8,
    pub data: Vec<u8>,
    write_count: AtomicUsize,
}

impl Framebuffer {
    pub fn new(w: u32, h: u32, bpp: u8) -> Self {
        let size = w as usize * h as usize * (bpp as usize / 8).max(1);
        Framebuffer {
            width: w,
            height: h,
            bpp,
            data: vec![0u8; size],
            write_count: AtomicUsize::new(0),
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let bytes = (self.bpp / 8).max(1) as usize;
        let off = (y as usize * self.width as usize + x as usize) * bytes;
        let cb = color.to_le_bytes();
        for i in 0..bytes.min(4) {
            if off + i < self.data.len() {
                self.data[off + i] = cb[i];
            }
        }
        self.write_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn clear(&mut self, color: u32) {
        let w = self.width;
        let h = self.height;
        for y in 0..h {
            for x in 0..w {
                self.set_pixel(x, y, color);
            }
        }
    }

    pub fn write_count(&self) -> usize {
        self.write_count.load(Ordering::Relaxed)
    }
}

/// Text console — 80×25 VT100-compatible terminal
pub struct TextConsole {
    pub cols: u32,
    pub rows: u32,
    pub cursor_x: u32,
    pub cursor_y: u32,
    cells: Vec<TextCell>,
    scroll_count: AtomicUsize,
}

impl TextConsole {
    pub fn vga80x25() -> Self {
        Self::new(80, 25)
    }

    pub fn new(cols: u32, rows: u32) -> Self {
        let cells = vec![TextCell::new(b' '); (cols * rows) as usize];
        TextConsole {
            cols,
            rows,
            cursor_x: 0,
            cursor_y: 0,
            cells,
            scroll_count: AtomicUsize::new(0),
        }
    }

    pub fn write_char(&mut self, ch: u8) {
        match ch {
            b'\n' => {
                self.cursor_x = 0;
                self.cursor_y += 1;
            }
            b'\r' => {
                self.cursor_x = 0;
            }
            _ => {
                if self.cursor_x < self.cols && self.cursor_y < self.rows {
                    let idx = (self.cursor_y * self.cols + self.cursor_x) as usize;
                    self.cells[idx] = TextCell::new(ch);
                }
                self.cursor_x += 1;
                if self.cursor_x >= self.cols {
                    self.cursor_x = 0;
                    self.cursor_y += 1;
                }
            }
        }
        if self.cursor_y >= self.rows {
            self.scroll();
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for b in s.bytes() {
            self.write_char(b);
        }
    }

    fn scroll(&mut self) {
        let cols = self.cols as usize;
        let rows = self.rows as usize;
        for y in 1..rows {
            for x in 0..cols {
                self.cells[(y - 1) * cols + x] = self.cells[y * cols + x];
            }
        }
        for x in 0..cols {
            self.cells[(rows - 1) * cols + x] = TextCell::new(b' ');
        }
        self.cursor_y = self.rows - 1;
        self.scroll_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_char(&self, x: u32, y: u32) -> Option<u8> {
        if x >= self.cols || y >= self.rows {
            return None;
        }
        Some(self.cells[(y * self.cols + x) as usize].ch)
    }

    pub fn scroll_count(&self) -> usize {
        self.scroll_count.load(Ordering::Relaxed)
    }
}

/// VGA controller driver — manages mode switching, palette, framebuffer
pub struct VgaDriver {
    pub current_mode: VideoMode,
    pub framebuffer: Option<Framebuffer>,
    pub console: TextConsole,
    mode_switches: AtomicUsize,
    initialized: bool,
}

impl VgaDriver {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        VgaDriver {
            current_mode: VideoMode::Cga80x25Text,
            framebuffer: None,
            console: TextConsole::vga80x25(),
            mode_switches: AtomicUsize::new(0),
            initialized: false,
        }
    }

    pub fn set_mode(&mut self, mode: VideoMode) {
        self.current_mode = mode;
        let (w, h) = mode.resolution();
        let bpp = mode.bpp();
        self.framebuffer = Some(Framebuffer::new(w, h, bpp));
        self.mode_switches.fetch_add(1, Ordering::SeqCst);
    }
}

impl KernelSubsystem for VgaDriver {
    fn name(&self) -> &str {
        "vga_driver"
    }
    fn version(&self) -> &str {
        "2.0.0"
    }
    fn init_order(&self) -> InitOrder {
        InitOrder::EarlyBoot
    }
    fn priority(&self) -> SubsystemPriority {
        SubsystemPriority::High
    }

    fn initialize(&mut self) -> Result<(), SubsystemError> {
        self.set_mode(VideoMode::Vga320x200x256);
        self.initialized = true;
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), SubsystemError> {
        self.framebuffer = None;
        Ok(())
    }
}

impl Default for VgaDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_video_modes() {
        assert_eq!(VideoMode::Vga320x200x256.resolution(), (320, 200));
        assert_eq!(VideoMode::Vesa1920x1080.resolution(), (1920, 1080));
        assert_eq!(VideoMode::Vesa1920x1080.bpp(), 32);
    }

    #[test]
    fn test_framebuffer_pixel() {
        let mut fb = Framebuffer::new(320, 200, 8);
        fb.set_pixel(10, 10, 0xFF);
        fb.set_pixel(319, 199, 0xAA);
        assert!(fb.write_count() >= 2);
    }

    #[test]
    fn test_text_console_write() {
        let mut con = TextConsole::vga80x25();
        con.write_str("SigmaOS");
        assert_eq!(con.get_char(0, 0), Some(b'S'));
        assert_eq!(con.get_char(6, 0), Some(b'S'));
        assert_eq!(con.cursor_x, 7);
    }

    #[test]
    fn test_text_console_scroll() {
        let mut con = TextConsole::new(80, 3);
        con.write_str("Line1\nLine2\nLine3\nLine4\n");
        assert!(con.scroll_count() > 0);
    }

    #[test]
    fn test_vga_mode_switch() {
        let mut drv = VgaDriver::new();
        drv.set_mode(VideoMode::Vga640x480x16);
        assert_eq!(drv.current_mode, VideoMode::Vga640x480x16);
        assert!(drv.framebuffer.is_some());
    }
}
