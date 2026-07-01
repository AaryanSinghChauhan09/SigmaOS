// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Linear Framebuffer Display Driver (Zig, no stdlib, no libc)
//! Replaces: drivers/display/sigma_fb.cpp
//! =========================================================================

pub const PixelFormat = enum(u8) {
    RGB24,
    BGR24,
    RGBA32,
    BGRA32,
};

pub const Color = struct {
    r: u8,
    g: u8,
    b: u8,
    a: u8,

    pub fn new(r: u8, g: u8, b: u8) Color {
        return Color{ .r = r, .g = g, .b = b, .a = 0xFF };
    }

    pub fn with_alpha(r: u8, g: u8, b: u8, a: u8) Color {
        return Color{ .r = r, .g = g, .b = b, .a = a };
    }

    pub fn to_u32_rgba(self: Color) u32 {
        return (@as(u32, self.r) << 24) |
               (@as(u32, self.g) << 16) |
               (@as(u32, self.b) << 8)  |
                @as(u32, self.a);
    }
};

/// Sovereign Framebuffer Driver — OOP struct with pure Zig
pub const Framebuffer = struct {
    base:   usize,
    width:  u32,
    height: u32,
    pitch:  u32,
    bpp:    u8,
    format: PixelFormat,

    pub fn new(base: usize, w: u32, h: u32, pitch: u32, bpp: u8, fmt: PixelFormat) Framebuffer {
        return Framebuffer{ .base = base, .width = w, .height = h,
                            .pitch = pitch, .bpp = bpp, .format = fmt };
    }

    /// Set a single pixel at (x, y)
    pub fn put_pixel(self: *const Framebuffer, x: u32, y: u32, color: Color) void {
        if (x >= self.width or y >= self.height) return;
        const offset = y * self.pitch + x * (@as(u32, self.bpp) / 8);
        const ptr: [*]volatile u8 = @intToPtr([*]volatile u8, self.base + offset);
        ptr[0] = color.b;
        ptr[1] = color.g;
        ptr[2] = color.r;
        if (self.bpp == 32) {
            ptr[3] = color.a;
        }
    }

    /// Fill a rectangle with a solid color
    pub fn fill_rect(self: *const Framebuffer, x: u32, y: u32, w: u32, h: u32, color: Color) void {
        var row: u32 = 0;
        while (row < h) : (row += 1) {
            var col: u32 = 0;
            while (col < w) : (col += 1) {
                self.put_pixel(x + col, y + row, color);
            }
        }
    }

    /// Clear the entire framebuffer to black
    pub fn clear(self: *const Framebuffer) void {
        self.fill_rect(0, 0, self.width, self.height, Color.new(0, 0, 0));
    }
};
