// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/display/vga_console.rs — VGA text-mode console + VESA framebuffer
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

// ── VGA text-mode ─────────────────────────────────────────────────────────
const VGA_BASE: u64  = 0xB8000;
const COLS: usize    = 80;
const ROWS: usize    = 25;

pub struct VgaConsole {
    col:   usize,
    row:   usize,
    color: u8,
}

impl VgaConsole {
    pub const fn new() -> Self { Self { col: 0, row: 0, color: 0x0F } }

    unsafe fn cell(&self, col: usize, row: usize) -> *mut u16 {
        (VGA_BASE as *mut u16).add(row * COLS + col)
    }

    pub unsafe fn clear(&mut self) {
        let blank = (self.color as u16) << 8 | b' ' as u16;
        for r in 0..ROWS { for c in 0..COLS { *self.cell(c, r) = blank; } }
        self.col = 0; self.row = 0;
    }

    unsafe fn scroll(&mut self) {
        let base = VGA_BASE as *mut u16;
        core::ptr::copy(base.add(COLS), base, (ROWS-1)*COLS);
        let blank = (self.color as u16) << 8 | b' ' as u16;
        for c in 0..COLS { *base.add((ROWS-1)*COLS + c) = blank; }
        self.row = ROWS - 1;
    }

    unsafe fn update_cursor(&self) {
        let pos = (self.row * COLS + self.col) as u16;
        let outb = |port: u16, val: u8| {
            core::arch::asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack));
        };
        outb(0x3D4, 0x0F); outb(0x3D5, (pos & 0xFF) as u8);
        outb(0x3D4, 0x0E); outb(0x3D5, (pos >> 8) as u8);
    }

    pub unsafe fn putc(&mut self, c: u8) {
        match c {
            b'\n' => { self.col = 0; self.row += 1; }
            b'\r' => { self.col = 0; }
            b'\x08' => { if self.col > 0 { self.col -= 1; *self.cell(self.col, self.row) = (self.color as u16)<<8 | b' ' as u16; } }
            _ => {
                *self.cell(self.col, self.row) = (self.color as u16)<<8 | c as u16;
                self.col += 1;
                if self.col >= COLS { self.col = 0; self.row += 1; }
            }
        }
        if self.row >= ROWS { self.scroll(); }
        self.update_cursor();
    }

    pub unsafe fn puts(&mut self, s: &[u8]) {
        for &b in s { self.putc(b); }
    }

    pub unsafe fn print_str(&mut self, s: &str) { self.puts(s.as_bytes()); }

    pub fn set_color(&mut self, fg: u8, bg: u8) {
        self.color = (bg << 4) | (fg & 0x0F);
    }
}

// ── VESA linear framebuffer ───────────────────────────────────────────────
pub struct VesaFb {
    pub base:   u64,
    pub width:  u32,
    pub height: u32,
    pub stride: u32,   // bytes per row (pixels_per_scan_line * 4)
    pub bpp:    u8,    // bits per pixel (usually 32)
    pub initialized: bool,
}

impl VesaFb {
    pub const fn new() -> Self {
        Self { base: 0, width: 0, height: 0, stride: 0, bpp: 32, initialized: false }
    }

    pub fn init(&mut self, base: u64, w: u32, h: u32, stride: u32) {
        self.base = base; self.width = w; self.height = h;
        self.stride = stride; self.initialized = true;
    }

    pub unsafe fn fill(&self, color: u32) {
        if !self.initialized { return; }
        let pixel_count = (self.height * self.stride / 4) as usize;
        let fb = self.base as *mut u32;
        for i in 0..pixel_count { *fb.add(i) = color; }
    }

    pub unsafe fn put_pixel(&self, x: u32, y: u32, color: u32) {
        if !self.initialized || x >= self.width || y >= self.height { return; }
        let offset = (y * self.stride / 4 + x) as usize;
        *((self.base as *mut u32).add(offset)) = color;
    }

    pub unsafe fn fill_rect(&self, x: u32, y: u32, w: u32, h: u32, color: u32) {
        for row in y..y.saturating_add(h).min(self.height) {
            for col in x..x.saturating_add(w).min(self.width) {
                self.put_pixel(col, row, color);
            }
        }
    }

    pub unsafe fn draw_boot_splash(&self) {
        if !self.initialized { return; }
        self.fill(0x001A1A2E);           // dark navy background
        // Draw a centered Σ glyph using a filled rectangle as placeholder
        let cx = self.width / 2;
        let cy = self.height / 2;
        self.fill_rect(cx - 40, cy - 40, 80, 80, 0x0060EFFF); // cyan square
        self.fill_rect(cx - 20, cy - 60, 40, 20, 0xFFFFFF);   // top bar
        self.fill_rect(cx - 20, cy + 40, 40, 20, 0xFFFFFF);   // bottom bar
        self.fill_rect(cx - 20, cy - 10, 40, 20, 0xFFFFFF);   // middle bar
    }
}

static mut VGA: VgaConsole = VgaConsole::new();
static mut FB:  VesaFb     = VesaFb::new();

#[no_mangle]
pub unsafe extern "C" fn vga_console_init() { VGA.clear(); }
#[no_mangle]
pub unsafe extern "C" fn vga_console_putc(c: u8, _color: u8) { VGA.putc(c); }
#[no_mangle]
pub unsafe extern "C" fn vga_console_puts(s: *const u8, len: usize) {
    if s.is_null() { return; }
    VGA.puts(core::slice::from_raw_parts(s, len));
}
#[no_mangle]
pub unsafe extern "C" fn vesa_fb_init(base: u64, w: u32, h: u32, stride: u32) {
    FB.init(base, w, h, stride);
    FB.draw_boot_splash();
}
#[no_mangle]
pub unsafe extern "C" fn vesa_fb_fill(color: u32) { FB.fill(color); }
#[no_mangle]
pub unsafe extern "C" fn vesa_fb_put_pixel(x: u32, y: u32, color: u32) { FB.put_pixel(x, y, color); }
#[no_mangle]
pub unsafe extern "C" fn vesa_fb_fill_rect(x: u32, y: u32, w: u32, h: u32, color: u32) {
    FB.fill_rect(x, y, w, h, color);
}
