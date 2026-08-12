/// SigmaOS: VESA/GOP Framebuffer Driver
/// Phase G Blocker #7: VESA/GOP framebuffer
/// Migrated from C/C++ to Rust — no_std, no alloc, no external crates.

#[allow(dead_code)]

// ─── Kernel Primitive Types ─────────────────────────────────────────────────

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// ─── Framebuffer Constants ─────────────────────────────────────────────────

pub const FRAMEBUFFER_WIDTH: SigmaU32 = 1024;
pub const FRAMEBUFFER_HEIGHT: SigmaU32 = 768;
pub const FRAMEBUFFER_BPP: SigmaU32 = 32;

// ─── Color Constants ───────────────────────────────────────────────────────

pub const COLOR_BLACK: SigmaU32 = 0x000000;
pub const COLOR_WHITE: SigmaU32 = 0xFFFFFF;
pub const COLOR_RED: SigmaU32 = 0xFF0000;
pub const COLOR_GREEN: SigmaU32 = 0x00FF00;
pub const COLOR_BLUE: SigmaU32 = 0x0000FF;

// ─── Framebuffer Info ─────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FramebufferInfo {
    pub address: SigmaU64,
    pub width: SigmaU32,
    pub height: SigmaU32,
    pub pitch: SigmaU32,
    pub bpp: SigmaU32,
    pub red_mask_size: SigmaU8,
    pub red_mask_shift: SigmaU8,
    pub green_mask_size: SigmaU8,
    pub green_mask_shift: SigmaU8,
    pub blue_mask_size: SigmaU8,
    pub blue_mask_shift: SigmaU8,
}

// ─── Framebuffer Mode ────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub enum FramebufferMode {
    TextMode,
    GraphicsMode,
}

// ─── Framebuffer Driver ───────────────────────────────────────────────────

pub struct FramebufferDriver {
    initialized: SigmaBool,
    info: FramebufferInfo,
    mode: FramebufferMode,
    current_bg_color: SigmaU32,
    current_fg_color: SigmaU32,
    cursor_x: SigmaU32,
    cursor_y: SigmaU32,
}

impl FramebufferDriver {
    pub const fn new() -> Self {
        Self {
            initialized: false,
            info: FramebufferInfo {
                address: 0,
                width: FRAMEBUFFER_WIDTH,
                height: FRAMEBUFFER_HEIGHT,
                pitch: FRAMEBUFFER_WIDTH * (FRAMEBUFFER_BPP / 8),
                bpp: FRAMEBUFFER_BPP,
                red_mask_size: 8,
                red_mask_shift: 16,
                green_mask_size: 8,
                green_mask_shift: 8,
                blue_mask_size: 8,
                blue_mask_shift: 0,
            },
            mode: FramebufferMode::TextMode,
            current_bg_color: COLOR_BLACK,
            current_fg_color: COLOR_WHITE,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    /// Initialize framebuffer with GOP (Graphics Output Protocol)
    pub unsafe fn init_gop(&mut self, gop_info: FramebufferInfo) -> Result<(), &'static str> {
        if self.initialized {
            return Err("Framebuffer already initialized");
        }

        self.info = gop_info;
        self.mode = FramebufferMode::GraphicsMode;
        self.initialized = true;

        // Clear screen
        self.clear_screen(COLOR_BLACK);

        Ok(())
    }

    /// Initialize framebuffer with VESA (VESA BIOS Extensions)
    pub unsafe fn init_vesa(&mut self, vesa_info: FramebufferInfo) -> Result<(), &'static str> {
        if self.initialized {
            return Err("Framebuffer already initialized");
        }

        self.info = vesa_info;
        self.mode = FramebufferMode::GraphicsMode;
        self.initialized = true;

        // Clear screen
        self.clear_screen(COLOR_BLACK);

        Ok(())
    }

    /// Initialize text mode framebuffer
    pub unsafe fn init_text_mode(&mut self) -> Result<(), &'static str> {
        if self.initialized {
            return Err("Framebuffer already initialized");
        }

        self.mode = FramebufferMode::TextMode;
        self.initialized = true;

        // Set VGA text mode
        self.set_vga_text_mode();

        Ok(())
    }

    /// Set VGA text mode
    unsafe fn set_vga_text_mode(&self) {
        // TODO: Implement VGA text mode setting
        // This would involve writing to VGA registers
    }

    /// Clear screen with color
    pub unsafe fn clear_screen(&mut self, color: SigmaU32) {
        if !self.initialized || self.info.address == 0 {
            return;
        }

        let fb_ptr = self.info.address as *mut SigmaU32;
        let total_pixels = (self.info.width * self.info.height) as SigmaUsize;

        for i in 0..total_pixels {
            *fb_ptr.add(i) = color;
        }

        self.cursor_x = 0;
        self.cursor_y = 0;
    }

    /// Put pixel at position
    pub unsafe fn put_pixel(&mut self, x: SigmaU32, y: SigmaU32, color: SigmaU32) {
        if !self.initialized || self.info.address == 0 {
            return;
        }

        if x >= self.info.width || y >= self.info.height {
            return;
        }

        let fb_ptr = self.info.address as *mut SigmaU32;
        let offset = (y * self.info.width + x) as SigmaUsize;
        *fb_ptr.add(offset) = color;
    }

    /// Get pixel at position
    pub unsafe fn get_pixel(&self, x: SigmaU32, y: SigmaU32) -> SigmaU32 {
        if !self.initialized || self.info.address == 0 {
            return 0;
        }

        if x >= self.info.width || y >= self.info.height {
            return 0;
        }

        let fb_ptr = self.info.address as *const SigmaU32;
        let offset = (y * self.info.width + x) as SigmaUsize;
        *fb_ptr.add(offset)
    }

    /// Draw rectangle
    pub unsafe fn draw_rect(&mut self, x: SigmaU32, y: SigmaU32, width: SigmaU32, height: SigmaU32, color: SigmaU32) {
        for py in y..(y + height) {
            for px in x..(x + width) {
                self.put_pixel(px, py, color);
            }
        }
    }

    /// Draw line
    pub unsafe fn draw_line(&mut self, x1: SigmaU32, y1: SigmaU32, x2: SigmaU32, y2: SigmaU32, color: SigmaU32) {
        let dx = if x2 > x1 { x2 - x1 } else { x1 - x2 };
        let dy = if y2 > y1 { y2 - y1 } else { y1 - y2 };
        let sx = if x1 < x2 { 1 } else { -1i32 };
        let sy = if y1 < y2 { 1 } else { -1i32 };

        let mut err = if dx > dy { dx as SigmaI32 - dy as SigmaI32 } else { dy as SigmaI32 - dx as SigmaI32 };

        let mut x = x1 as SigmaI32;
        let mut y = y1 as SigmaI32;

        loop {
            self.put_pixel(x as SigmaU32, y as SigmaU32, color);

            if x == x2 as SigmaI32 && y == y2 as SigmaI32 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -(dy as SigmaI32) {
                err -= dy as SigmaI32;
                x += sx;
            }
            if e2 < dx as SigmaI32 {
                err += dx as SigmaI32;
                y += sy;
            }
        }
    }

    /// Draw circle
    pub unsafe fn draw_circle(&mut self, cx: SigmaU32, cy: SigmaU32, radius: SigmaU32, color: SigmaU32) {
        let mut x = radius as SigmaI32;
        let mut y: SigmaI32 = 0;
        let mut err = 0;

        while x >= y {
            self.put_pixel((cx as SigmaI32 + x) as SigmaU32, (cy as SigmaI32 + y) as SigmaU32, color);
            self.put_pixel((cx as SigmaI32 + y) as SigmaU32, (cy as SigmaI32 + x) as SigmaU32, color);
            self.put_pixel((cx as SigmaI32 - y) as SigmaU32, (cy as SigmaI32 + x) as SigmaU32, color);
            self.put_pixel((cx as SigmaI32 - x) as SigmaU32, (cy as SigmaI32 + y) as SigmaU32, color);
            self.put_pixel((cx as SigmaI32 - x) as SigmaU32, (cy as SigmaI32 - y) as SigmaU32, color);
            self.put_pixel((cx as SigmaI32 - y) as SigmaU32, (cy as SigmaI32 - x) as SigmaU32, color);
            self.put_pixel((cx as SigmaI32 + y) as SigmaU32, (cy as SigmaI32 - x) as SigmaU32, color);
            self.put_pixel((cx as SigmaI32 + x) as SigmaU32, (cy as SigmaI32 - y) as SigmaU32, color);

            if err <= 0 {
                y += 1;
                err += 2 * y + 1;
            }

            if err > 0 {
                x -= 1;
                err -= 2 * x + 1;
            }
        }
    }

    /// Set background color
    pub unsafe fn set_bg_color(&mut self, color: SigmaU32) {
        self.current_bg_color = color;
    }

    /// Set foreground color
    pub unsafe fn set_fg_color(&mut self, color: SigmaU32) {
        self.current_fg_color = color;
    }

    /// Get framebuffer info
    pub unsafe fn get_info(&self) -> FramebufferInfo {
        self.info
    }

    /// Get current mode
    pub unsafe fn get_mode(&self) -> FramebufferMode {
        self.mode
    }

    /// Check if initialized
    pub unsafe fn is_initialized(&self) -> SigmaBool {
        self.initialized
    }

    /// Get cursor position
    pub unsafe fn get_cursor(&self) -> (SigmaU32, SigmaU32) {
        (self.cursor_x, self.cursor_y)
    }

    /// Set cursor position
    pub unsafe fn set_cursor(&mut self, x: SigmaU32, y: SigmaU32) {
        self.cursor_x = x;
        self.cursor_y = y;
    }
}

// ─── Global Framebuffer Driver Instance ─────────────────────────────────────

static mut FRAMEBUFFER_DRIVER: FramebufferDriver = FramebufferDriver::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_fb_init_gop(address: SigmaU64, width: SigmaU32, height: SigmaU32, pitch: SigmaU32, bpp: SigmaU32) -> SigmaI32 {
    let info = FramebufferInfo {
        address,
        width,
        height,
        pitch,
        bpp,
        red_mask_size: 8,
        red_mask_shift: 16,
        green_mask_size: 8,
        green_mask_shift: 8,
        blue_mask_size: 8,
        blue_mask_shift: 0,
    };
    
    match FRAMEBUFFER_DRIVER.init_gop(info) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fb_init_vesa(address: SigmaU64, width: SigmaU32, height: SigmaU32, pitch: SigmaU32, bpp: SigmaU32) -> SigmaI32 {
    let info = FramebufferInfo {
        address,
        width,
        height,
        pitch,
        bpp,
        red_mask_size: 8,
        red_mask_shift: 16,
        green_mask_size: 8,
        green_mask_shift: 8,
        blue_mask_size: 8,
        blue_mask_shift: 0,
    };
    
    match FRAMEBUFFER_DRIVER.init_vesa(info) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fb_init_text() -> SigmaI32 {
    match FRAMEBUFFER_DRIVER.init_text_mode() {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fb_clear(color: SigmaU32) {
    FRAMEBUFFER_DRIVER.clear_screen(color);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fb_put_pixel(x: SigmaU32, y: SigmaU32, color: SigmaU32) {
    FRAMEBUFFER_DRIVER.put_pixel(x, y, color);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fb_get_pixel(x: SigmaU32, y: SigmaU32) -> SigmaU32 {
    FRAMEBUFFER_DRIVER.get_pixel(x, y)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fb_draw_rect(x: SigmaU32, y: SigmaU32, width: SigmaU32, height: SigmaU32, color: SigmaU32) {
    FRAMEBUFFER_DRIVER.draw_rect(x, y, width, height, color);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fb_draw_line(x1: SigmaU32, y1: SigmaU32, x2: SigmaU32, y2: SigmaU32, color: SigmaU32) {
    FRAMEBUFFER_DRIVER.draw_line(x1, y1, x2, y2, color);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fb_draw_circle(cx: SigmaU32, cy: SigmaU32, radius: SigmaU32, color: SigmaU32) {
    FRAMEBUFFER_DRIVER.draw_circle(cx, cy, radius, color);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fb_get_address() -> SigmaU64 {
    FRAMEBUFFER_DRIVER.get_info().address
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fb_get_width() -> SigmaU32 {
    FRAMEBUFFER_DRIVER.get_info().width
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fb_get_height() -> SigmaU32 {
    FRAMEBUFFER_DRIVER.get_info().height
}
