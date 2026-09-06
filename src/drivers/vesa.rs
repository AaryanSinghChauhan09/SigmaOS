#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
use std::string::{String, ToString};
use std::vec::Vec;
use std::format;
// SigmaOS VESA Framebuffer Driver
// Hardware abstraction for VESA BIOS extensions + PeripheralDevice OOP integration

use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};
use crate::security::CapabilityToken;

/// VESA mode info
#[derive(Debug, Clone)]
pub struct VesaModeInfo {
    pub width: u32,
    pub height: u32,
    pub bpp: u32,
    pub pitch: u32,
    pub framebuffer_addr: u64,
}

/// VESA driver interface
pub struct VesaDriver {
    pub mode_info: VesaModeInfo,
    pub capabilities: CapabilityToken,
    pub current_mode: u16,

    // Double buffering support inspired by Linux fbdev
    pub back_buffer_active: bool,
    pub back_buffer: Vec<u32>,
}

impl VesaDriver {
    pub fn new() -> Self {
        Self {
            mode_info: VesaModeInfo {
                width: 1024,
                height: 768,
                bpp: 32,
                pitch: 4096,
                framebuffer_addr: 0xE0000000,
            },
            capabilities: CapabilityToken::new(),
            current_mode: 0,
            back_buffer_active: false,
            back_buffer: Vec::new(),
        }
    }

    pub fn with_mode(width: u32, height: u32, bpp: u32) -> Self {
        let pitch = width * (bpp / 8);
        Self {
            mode_info: VesaModeInfo {
                width,
                height,
                bpp,
                pitch,
                framebuffer_addr: 0xE0000000,
            },
            capabilities: CapabilityToken::new(),
            current_mode: 0,
            back_buffer_active: false,
            back_buffer: Vec::new(),
        }
    }

    pub fn initialize(&mut self) -> Result<(), VesaError> {
        // Simulate VESA initialization
        self.current_mode = 0x118; // Common VESA mode
        Ok(())
    }

    pub fn set_mode(&mut self, mode: u16) -> Result<(), VesaError> {
        // Simulate mode setting
        self.current_mode = mode;

        // Update mode info based on mode
        match mode {
            0x112 => {
                self.mode_info.width = 640;
                self.mode_info.height = 480;
                self.mode_info.bpp = 32;
            }
            0x115 => {
                self.mode_info.width = 800;
                self.mode_info.height = 600;
                self.mode_info.bpp = 32;
            }
            0x118 => {
                self.mode_info.width = 1024;
                self.mode_info.height = 768;
                self.mode_info.bpp = 32;
            }
            _ => {
                return Err(VesaError::InvalidMode);
            }
        }

        self.mode_info.pitch = self.mode_info.width * (self.mode_info.bpp / 8);

        // Resize back buffer if active
        if self.back_buffer_active {
            let total_pixels = (self.mode_info.width * self.mode_info.height) as usize;
            self.back_buffer.resize(total_pixels, 0);
        }

        Ok(())
    }

    /// Initializes double-buffering backing storage
    pub fn enable_double_buffering(&mut self) {
        self.back_buffer_active = true;
        let total_pixels = (self.mode_info.width * self.mode_info.height) as usize;
        self.back_buffer.resize(total_pixels, 0);
    }

    /// Disables double-buffering backing storage
    pub fn disable_double_buffering(&mut self) {
        self.back_buffer_active = false;
        self.back_buffer.clear();
    }

    /// Swaps the simulated back buffer elements onto the active hardware framebuffer
    pub fn swap_buffers(&self) -> Result<(), VesaError> {
        if !self.back_buffer_active {
            return Err(VesaError::InitializationFailed);
        }
        #[cfg(target_os = "none")]
        {
            let fb_addr = self.mode_info.framebuffer_addr as *mut u32;
            if !fb_addr.is_null() {
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        self.back_buffer.as_ptr(),
                        fb_addr,
                        self.back_buffer.len(),
                    );
                }
            }
        }
        Ok(())
    }

    pub fn get_mode_info(&self) -> &VesaModeInfo {
        &self.mode_info
    }

    pub fn write_pixel(&mut self, x: u32, y: u32, color: u32) -> Result<(), VesaError> {
        if x >= self.mode_info.width || y >= self.mode_info.height {
            return Err(VesaError::OutOfBounds);
        }

        // Calculate pixel offset
        let _offset = (y * self.mode_info.pitch + x * (self.mode_info.bpp / 8)) as usize;

        // In production, this would write to actual framebuffer or back-buffer
        if self.back_buffer_active {
            let total_pixels = (self.mode_info.width * self.mode_info.height) as usize;
            let idx = (y * self.mode_info.width + x) as usize;
            if idx < total_pixels {
                self.back_buffer[idx] = color;
            }
        }

        Ok(())
    }

    pub fn clear_screen(&mut self, color: u32) -> Result<(), VesaError> {
        if self.back_buffer_active {
            for pixel in self.back_buffer.iter_mut() {
                *pixel = color;
            }
        }
        Ok(())
    }

    /// Standard Bresenham's Line Drawing Algorithm (integer-only arithmetic)
    pub fn draw_line(
        &mut self,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        color: u32,
    ) -> Result<(), VesaError> {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut cx = x0;
        let mut cy = y0;

        loop {
            if cx >= 0
                && cx < self.mode_info.width as i32
                && cy >= 0
                && cy < self.mode_info.height as i32
            {
                self.write_pixel(cx as u32, cy as u32, color).ok();
            }

            if cx == x1 && cy == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                cx += sx;
            }
            if e2 <= dx {
                err += dx;
                cy += sy;
            }
        }
        Ok(())
    }

    /// Embedded 8x16 bitmap console font character blitting (inspired by BSD syscons)
    /// Emulates drawing standard printable ASCII range
    pub fn draw_char(&mut self, x: u32, y: u32, ch: char, color: u32) -> Result<(), VesaError> {
        // Minimal representation of an 8x16 font glyph for letter 'A' and legacy text blocks
        // Bit 1 indicates fill, 0 indicates empty background
        let mut glyph = [0u8; 16];
        if ch == 'A' {
            glyph = [
                0b00011000, 0b00111100, 0b01100110, 0b01100110, 0b11111111, 0b11000011, 0b11000011,
                0b00000000, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ];
        } else {
            // Default generic dash glyph
            glyph[7] = 0b11111111;
        }

        for row in 0..16 {
            let row_byte = glyph[row];
            for col in 0..8 {
                if (row_byte & (0x80 >> col)) != 0 {
                    self.write_pixel(x + col, y + row as u32, color).ok();
                }
            }
        }
        Ok(())
    }

    pub fn set_capabilities(&mut self, capabilities: CapabilityToken) {
        self.capabilities = capabilities;
    }

    pub fn has_capability(&self, capability: u64) -> bool {
        (self.capabilities.bits() & capability) != 0
    }

    /// Write a pixel directly to the framebuffer memory at (x, y) with a 32-bit ARGB color.
    /// In bare-metal targets, this writes to `framebuffer_addr`; on hosted targets it validates bounds.
    pub fn write_pixel_raw(&self, x: u32, y: u32, color: u32) -> Result<(), VesaError> {
        if x >= self.mode_info.width || y >= self.mode_info.height {
            return Err(VesaError::OutOfBounds);
        }
        #[cfg(target_os = "none")]
        {
            let offset = (y * self.mode_info.pitch + x * (self.mode_info.bpp / 8)) as usize;
            let fb_addr = self.mode_info.framebuffer_addr as *mut u32;
            if !fb_addr.is_null() {
                // Safety: Only valid when running on bare-metal with an MMIO framebuffer.
                unsafe {
                    fb_addr.add(offset / 4).write_volatile(color);
                }
            }
        }
        #[cfg(not(target_os = "none"))]
        {
            let _ = (y, x, color);
        }
        Ok(())
    }

    /// Fill the entire screen with a 32-bit ARGB color.
    pub fn fill_screen(&self, color: u32) -> Result<(), VesaError> {
        #[cfg(target_os = "none")]
        {
            let total_pixels = (self.mode_info.width * self.mode_info.height) as usize;
            let fb_addr = self.mode_info.framebuffer_addr as *mut u32;
            if !fb_addr.is_null() {
                unsafe {
                    for i in 0..total_pixels {
                        fb_addr.add(i).write_volatile(color);
                    }
                }
            }
        }
        #[cfg(not(target_os = "none"))]
        {
            let _ = color;
        }
        Ok(())
    }
}

impl PeripheralDevice for VesaDriver {
    fn name(&self) -> &'static str {
        "VESA Framebuffer"
    }
    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Legacy
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        self.initialize().map_err(|_| "VESA: Initialization failed")
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        // Framebuffers are write-only; return mode info bytes instead
        let info = [
            (self.mode_info.width >> 8) as u8,
            (self.mode_info.width & 0xFF) as u8,
            (self.mode_info.height >> 8) as u8,
            (self.mode_info.height & 0xFF) as u8,
            self.mode_info.bpp as u8,
            self.current_mode as u8,
        ];
        let len = buffer.len().min(info.len());
        buffer[..len].copy_from_slice(&info[..len]);
        Ok(len)
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        // Interpret data as (x_hi, x_lo, y_hi, y_lo, r, g, b, a) pixel write packets
        let mut written = 0;
        let mut idx = 0;
        while idx + 7 < data.len() {
            let x = u32::from_be_bytes([0, 0, data[idx], data[idx + 1]]);
            let y = u32::from_be_bytes([0, 0, data[idx + 2], data[idx + 3]]);
            let color =
                u32::from_be_bytes([data[idx + 4], data[idx + 5], data[idx + 6], data[idx + 7]]);
            self.write_pixel_raw(x, y, color).ok();
            idx += 8;
            written += 8;
        }
        Ok(written)
    }

    fn set_power_state(&mut self, _state: PowerState) -> Result<(), &'static str> {
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.fill_screen(0x00000000).ok();
        Ok(())
    }
}

impl Default for VesaDriver {
    fn default() -> Self {
        Self::new()
    }
}

/// VESA errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VesaError {
    InvalidMode,
    OutOfBounds,
    PermissionDenied,
    InitializationFailed,
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_vesa_creation() {
        let vesa = VesaDriver::new();
        assert_eq!(vesa.mode_info.width, 1024);
        assert_eq!(vesa.mode_info.height, 768);
        assert_eq!(vesa.mode_info.bpp, 32);
    }

    #[test]
    fn test_vesa_with_mode() {
        let vesa = VesaDriver::with_mode(1920, 1080, 32);
        assert_eq!(vesa.mode_info.width, 1920);
        assert_eq!(vesa.mode_info.height, 1080);
    }

    #[test]
    fn test_set_mode() {
        let mut vesa = VesaDriver::new();
        assert!(vesa.set_mode(0x112).is_ok());
        assert_eq!(vesa.mode_info.width, 640);
        assert_eq!(vesa.mode_info.height, 480);
    }

    #[test]
    fn test_invalid_mode() {
        let mut vesa = VesaDriver::new();
        assert!(vesa.set_mode(0x999).is_err());
    }

    #[test]
    fn test_write_pixel_and_double_buffer() {
        let mut vesa = VesaDriver::new();
        vesa.enable_double_buffering();
        assert!(vesa.write_pixel(100, 100, 0xFFFFFF).is_ok());
        assert_eq!(vesa.back_buffer[(100 * 1024 + 100) as usize], 0xFFFFFF);

        assert!(vesa.clear_screen(0x123456).is_ok());
        assert_eq!(vesa.back_buffer[0], 0x123456);
        assert!(vesa.swap_buffers().is_ok());
    }

    #[test]
    fn test_draw_line_and_char() {
        let mut vesa = VesaDriver::new();
        vesa.enable_double_buffering();
        assert!(vesa.draw_line(0, 0, 10, 10, 0xFFFFFF).is_ok());
        assert_eq!(vesa.back_buffer[0], 0xFFFFFF);

        assert!(vesa.draw_char(20, 20, 'A', 0xFFFFFF).is_ok());
    }

    #[test]
    fn test_out_of_bounds() {
        let mut vesa = VesaDriver::new();
        assert!(vesa.write_pixel(9999, 9999, 0xFFFFFF).is_err());
    }
}
