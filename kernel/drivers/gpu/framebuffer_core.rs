// framebuffer_core.rs: Generic Framebuffer / Basic GPU Skeleton

#![no_std]

pub struct Framebuffer {
    pub base_address: usize,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub bpp: u8,
}

impl Framebuffer {
    /// Draw a single pixel
    pub fn draw_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }

        let offset = (y * self.pitch) + (x * (self.bpp as u32 / 8));
        
        // SAFETY: The base_address must be a valid, mapped MMIO region provided by the bootloader/GPU driver
        unsafe {
            let ptr = (self.base_address + offset as usize) as *mut u32;
            core::ptr::write_volatile(ptr, color);
        }
    }

    /// Clear the entire screen
    pub fn clear(&mut self, color: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.draw_pixel(x, y, color);
            }
        }
    }
}
