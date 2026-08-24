// #![no_std]
extern crate alloc;
use alloc::vec::Vec;
use core::ptr;

pub struct Framebuffer {
    pub addr: u32,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub bpp: u8,
    pub back_buffer: Vec<u8>,
}

impl Framebuffer {
    pub fn new(addr: u32, width: u32, height: u32, pitch: u32, bpp: u8) -> Self {
        let size = (pitch * height) as usize;
        Self {
            addr, width, height, pitch, bpp,
            back_buffer: alloc::vec![0; size],
        }
    }
    
    pub fn put_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x >= self.width || y >= self.height { return; }
        let offset = (y * self.pitch + x * (self.bpp as u32 / 8)) as usize;
        let bytes = color.to_le_bytes();
        self.back_buffer[offset] = bytes[0];
        self.back_buffer[offset+1] = bytes[1];
        self.back_buffer[offset+2] = bytes[2];
        if self.bpp == 32 {
            self.back_buffer[offset+3] = bytes[3];
        }
    }
    
    pub fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: u32) {
        for dy in 0..h {
            for dx in 0..w {
                self.put_pixel(x + dx, y + dy, color);
            }
        }
    }
    
    pub fn draw_line(&mut self, mut x0: i32, mut y0: i32, x1: i32, y1: i32, color: u32) {
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        
        loop {
            self.put_pixel(x0 as u32, y0 as u32, color);
            if x0 == x1 && y0 == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy { err += dy; x0 += sx; }
            if e2 <= dx { err += dx; y0 += sy; }
        }
    }
    
    pub fn draw_char(&mut self, x: u32, y: u32, c: char, fg: u32, bg: u32) {
        // Stub for rendering a character using embedded font
        self.draw_rect(x, y, 8, 16, fg);
    }
    
    pub fn draw_string(&mut self, mut x: u32, y: u32, s: &str, fg: u32, bg: u32) {
        for c in s.chars() {
            self.draw_char(x, y, c, fg, bg);
            x += 8;
        }
    }
    
    pub fn swap_buffers(&self) {
        unsafe {
            let fb_ptr = self.addr as *mut u8;
            ptr::copy_nonoverlapping(self.back_buffer.as_ptr(), fb_ptr, self.back_buffer.len());
        }
    }
}
