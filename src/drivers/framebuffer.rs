//! Simple Framebuffer Driver
//!
//! Inspired by Linux simplefb and UEFI GOP (Graphics Output Protocol).
//! Provides early boot graphics before full DRM/KMS is initialized.
//!
//! # Features
//! - UEFI GOP framebuffer support
//! - VESA VBE framebuffer support
//! - Direct pixel manipulation (no hardware acceleration)
//! - Console output rendering
//!
//! # Linux Inspiration
//! - `drivers/video/fbdev/simplefb.c` - Simple framebuffer driver
//! - `drivers/video/fbdev/efifb.c` - EFI framebuffer
//! - `drivers/video/fbdev/vesafb.c` - VESA framebuffer
//!
//! # Use Cases
//! - Boot splash screen
//! - Early kernel console
//! - Panic screen display
//! - Fallback when GPU drivers fail

#![cfg_attr(not(any(feature = "standalone_test", test)), no_std)]

use core::fmt;
use core::ptr;

/// Pixel format (color channel layout)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PixelFormat {
    /// 32-bit BGRA (Blue, Green, Red, Alpha)
    Bgra8888 = 0,
    /// 32-bit RGBA (Red, Green, Blue, Alpha)
    Rgba8888 = 1,
    /// 24-bit RGB (Red, Green, Blue)
    Rgb888 = 2,
    /// 16-bit RGB (5-6-5)
    Rgb565 = 3,
}

impl PixelFormat {
    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            Self::Bgra8888 | Self::Rgba8888 => 4,
            Self::Rgb888 => 3,
            Self::Rgb565 => 2,
        }
    }
}

/// Framebuffer information (from bootloader/UEFI)
#[derive(Debug, Clone, Copy)]
pub struct FramebufferInfo {
    /// Physical address of framebuffer memory
    pub address: u64,
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Bytes per scanline (stride)
    pub stride: u32,
    /// Pixel format
    pub format: PixelFormat,
}

/// Framebuffer driver
pub struct Framebuffer {
    info: FramebufferInfo,
    base_ptr: *mut u8,
}

impl Framebuffer {
    /// Create framebuffer from bootloader-provided info
    /// # Safety
    /// Caller must ensure framebuffer address is valid and accessible
    pub unsafe fn new(info: FramebufferInfo) -> Self {
        Self {
            info,
            base_ptr: info.address as *mut u8,
        }
    }

    /// Get framebuffer dimensions
    pub fn dimensions(&self) -> (u32, u32) {
        (self.info.width, self.info.height)
    }

    /// Get pixel format
    pub fn format(&self) -> PixelFormat {
        self.info.format
    }

    /// Draw a single pixel
    /// # Safety
    /// Caller must ensure (x, y) is within bounds
    pub unsafe fn draw_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x >= self.info.width || y >= self.info.height {
            return;
        }

        let bpp = self.info.format.bytes_per_pixel();
        let offset = (y * self.info.stride + x * bpp as u32) as usize;
        let pixel_ptr = self.base_ptr.add(offset);

        match self.info.format {
            PixelFormat::Bgra8888 => {
                ptr::write_volatile(pixel_ptr as *mut u32, color);
            }
            PixelFormat::Rgba8888 => {
                // Swap R and B channels
                let b = (color >> 16) & 0xFF;
                let g = (color >> 8) & 0xFF;
                let r = color & 0xFF;
                let a = (color >> 24) & 0xFF;
                let swapped = (a << 24) | (b << 16) | (g << 8) | r;
                ptr::write_volatile(pixel_ptr as *mut u32, swapped);
            }
            PixelFormat::Rgb888 => {
                ptr::write_volatile(pixel_ptr, (color & 0xFF) as u8); // R
                ptr::write_volatile(pixel_ptr.add(1), ((color >> 8) & 0xFF) as u8); // G
                ptr::write_volatile(pixel_ptr.add(2), ((color >> 16) & 0xFF) as u8);
                // B
            }
            PixelFormat::Rgb565 => {
                let r5 = ((color >> 19) & 0x1F) as u16;
                let g6 = ((color >> 10) & 0x3F) as u16;
                let b5 = ((color >> 3) & 0x1F) as u16;
                let rgb565 = (r5 << 11) | (g6 << 5) | b5;
                ptr::write_volatile(pixel_ptr as *mut u16, rgb565);
            }
        }
    }

    /// Fill rectangle with solid color
    pub fn fill_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: u32) {
        for dy in 0..height {
            for dx in 0..width {
                unsafe {
                    self.draw_pixel(x + dx, y + dy, color);
                }
            }
        }
    }

    /// Clear screen to color
    pub fn clear(&mut self, color: u32) {
        self.fill_rect(0, 0, self.info.width, self.info.height, color);
    }

    /// Draw 8x8 ASCII character (simple bitmap font)
    pub fn draw_char(&mut self, x: u32, y: u32, ch: char, fg: u32, bg: u32) {
        let glyph = get_font_glyph(ch);
        for row in 0..8 {
            for col in 0..8 {
                let bit = (glyph[row] >> (7 - col)) & 1;
                let color = if bit == 1 { fg } else { bg };
                unsafe {
                    self.draw_pixel(x + col as u32, y + row as u32, color);
                }
            }
        }
    }

    /// Draw text string
    pub fn draw_text(&mut self, x: u32, y: u32, text: &str, fg: u32, bg: u32) {
        let mut cursor_x = x;
        for ch in text.chars() {
            self.draw_char(cursor_x, y, ch, fg, bg);
            cursor_x += 8;
            if cursor_x + 8 > self.info.width {
                break;
            }
        }
    }

    /// Scroll screen up by one line (8 pixels)
    pub unsafe fn scroll_up(&mut self) {
        let bpp = self.info.format.bytes_per_pixel();
        let line_size = self.info.stride as usize;
        let screen_size = (self.info.height as usize * line_size) as usize;
        let scroll_amount = 8 * line_size;

        // Move memory up by 8 lines
        ptr::copy(
            self.base_ptr.add(scroll_amount),
            self.base_ptr,
            screen_size - scroll_amount,
        );

        // Clear bottom 8 lines
        ptr::write_bytes(
            self.base_ptr.add(screen_size - scroll_amount),
            0,
            scroll_amount,
        );
    }
}

/// Simple 8x8 bitmap font (ASCII subset)
/// Each character is 8 bytes (8x8 pixels)
fn get_font_glyph(ch: char) -> &'static [u8; 8] {
    match ch {
        ' ' => &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        'A' => &[0x18, 0x3C, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x00],
        'B' => &[0x7C, 0x66, 0x66, 0x7C, 0x66, 0x66, 0x7C, 0x00],
        'E' => &[0x7E, 0x60, 0x60, 0x7C, 0x60, 0x60, 0x7E, 0x00],
        'S' => &[0x3C, 0x66, 0x60, 0x3C, 0x06, 0x66, 0x3C, 0x00],
        'i' => &[0x00, 0x18, 0x00, 0x18, 0x18, 0x18, 0x18, 0x00],
        'g' => &[0x00, 0x3C, 0x66, 0x66, 0x3E, 0x06, 0x7C, 0x00],
        'm' => &[0x00, 0x6C, 0xFE, 0xD6, 0xC6, 0xC6, 0xC6, 0x00],
        'a' => &[0x00, 0x00, 0x3C, 0x06, 0x3E, 0x66, 0x3E, 0x00],
        'O' => &[0x3C, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00],
        '0' => &[0x3C, 0x66, 0x6E, 0x76, 0x66, 0x66, 0x3C, 0x00],
        '1' => &[0x18, 0x38, 0x18, 0x18, 0x18, 0x18, 0x7E, 0x00],
        '.' => &[0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00],
        _ => &[0x00, 0x00, 0x3C, 0x3C, 0x3C, 0x00, 0x00, 0x00], // Unknown char placeholder
    }
}

/// Console writer using framebuffer
pub struct FbConsole {
    fb: Framebuffer,
    cursor_x: u32,
    cursor_y: u32,
    fg_color: u32,
    bg_color: u32,
}

impl FbConsole {
    pub fn new(fb: Framebuffer) -> Self {
        Self {
            fb,
            cursor_x: 0,
            cursor_y: 0,
            fg_color: 0xFFFFFF, // White
            bg_color: 0x000000, // Black
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for ch in s.chars() {
            match ch {
                '\n' => {
                    self.cursor_x = 0;
                    self.cursor_y += 8;
                    if self.cursor_y + 8 > self.fb.info.height {
                        unsafe {
                            self.fb.scroll_up();
                        }
                        self.cursor_y -= 8;
                    }
                }
                '\r' => {
                    self.cursor_x = 0;
                }
                _ => {
                    self.fb.draw_char(
                        self.cursor_x,
                        self.cursor_y,
                        ch,
                        self.fg_color,
                        self.bg_color,
                    );
                    self.cursor_x += 8;
                    if self.cursor_x + 8 > self.fb.info.width {
                        self.cursor_x = 0;
                        self.cursor_y += 8;
                        if self.cursor_y + 8 > self.fb.info.height {
                            unsafe {
                                self.fb.scroll_up();
                            }
                            self.cursor_y -= 8;
                        }
                    }
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.fb.clear(self.bg_color);
        self.cursor_x = 0;
        self.cursor_y = 0;
    }
}

impl fmt::Write for FbConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

/// RGB color constants
pub mod colors {
    pub const BLACK: u32 = 0x000000;
    pub const WHITE: u32 = 0xFFFFFF;
    pub const RED: u32 = 0xFF0000;
    pub const GREEN: u32 = 0x00FF00;
    pub const BLUE: u32 = 0x0000FF;
    pub const YELLOW: u32 = 0xFFFF00;
    pub const CYAN: u32 = 0x00FFFF;
    pub const MAGENTA: u32 = 0xFF00FF;
    pub const GRAY: u32 = 0x808080;
}

#[cfg(test)]
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_format_sizes() {
        assert_eq!(PixelFormat::Bgra8888.bytes_per_pixel(), 4);
        assert_eq!(PixelFormat::Rgb888.bytes_per_pixel(), 3);
        assert_eq!(PixelFormat::Rgb565.bytes_per_pixel(), 2);
    }

    #[test]
    fn test_framebuffer_info() {
        let info = FramebufferInfo {
            address: 0xB8000,
            width: 1920,
            height: 1080,
            stride: 1920 * 4,
            format: PixelFormat::Bgra8888,
        };

        assert_eq!(info.width, 1920);
        assert_eq!(info.height, 1080);
    }
}
