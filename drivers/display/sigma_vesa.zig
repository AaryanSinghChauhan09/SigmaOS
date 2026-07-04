// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/display/sigma_vesa.zig — VESA/GOP Framebuffer Driver
// Replaces: sigma_vesa.cpp (C++ stub, removed)
//
// Language: Zig — direct MMIO, no hidden allocations, comptime pixel formats
// Pattern: struct with methods (OOP equivalent)

const sigma_boot = @import("../../sigma-boot/sigma_boot.zig");

// ── Types ─────────────────────────────────────────────────────────────────────

pub const Color = packed struct {
    b: u8, g: u8, r: u8, a: u8 = 0xFF,

    pub fn rgb(r: u8, g: u8, b: u8) Color { return .{ .r=r, .g=g, .b=b }; }
    pub const BLACK  = Color.rgb(0,   0,   0);
    pub const WHITE  = Color.rgb(255, 255, 255);
    pub const CYAN   = Color.rgb(69,  243, 255);
    pub const PURPLE = Color.rgb(168, 85,  247);
};

pub const Rect = struct { x: u32, y: u32, w: u32, h: u32 };

// ── Framebuffer Driver ────────────────────────────────────────────────────────

pub const VesaFb = struct {
    base:   usize,
    width:  u32,
    height: u32,
    stride: u32,  // pixels per scanline (may differ from width)

    pub fn init(info: *const sigma_boot.BootInfo) VesaFb {
        return VesaFb{
            .base   = info.framebuffer,
            .width  = info.fb_width,
            .height = info.fb_height,
            .stride = info.fb_stride,
        };
    }

    /// Plot a single pixel at (x, y)
    pub fn put_pixel(self: *const VesaFb, x: u32, y: u32, color: Color) void {
        if (x >= self.width or y >= self.height) return;
        const offset: usize = (y * self.stride + x) * 4;
        const ptr: *volatile u32 = @ptrFromInt(self.base + offset);
        ptr.* = @bitCast(color);
    }

    /// Fill a rectangle with a solid color
    pub fn fill_rect(self: *const VesaFb, rect: Rect, color: Color) void {
        var row: u32 = 0;
        while (row < rect.h) : (row += 1) {
            var col: u32 = 0;
            while (col < rect.w) : (col += 1) {
                self.put_pixel(rect.x + col, rect.y + row, color);
            }
        }
    }

    /// Clear entire screen to a color
    pub fn clear(self: *const VesaFb, color: Color) void {
        const c32: u32 = @bitCast(color);
        var y: u32 = 0;
        while (y < self.height) : (y += 1) {
            var x: u32 = 0;
            while (x < self.stride) : (x += 1) {
                const offset: usize = (y * self.stride + x) * 4;
                const ptr: *volatile u32 = @ptrFromInt(self.base + offset);
                ptr.* = c32;
            }
        }
    }

    /// Draw a horizontal line
    pub fn hline(self: *const VesaFb, x: u32, y: u32, len: u32, color: Color) void {
        var i: u32 = 0;
        while (i < len) : (i += 1) self.put_pixel(x + i, y, color);
    }

    /// Draw a vertical line
    pub fn vline(self: *const VesaFb, x: u32, y: u32, len: u32, color: Color) void {
        var i: u32 = 0;
        while (i < len) : (i += 1) self.put_pixel(x, y + i, color);
    }

    /// Draw rectangle outline
    pub fn draw_rect(self: *const VesaFb, rect: Rect, color: Color) void {
        self.hline(rect.x, rect.y, rect.w, color);
        self.hline(rect.x, rect.y + rect.h - 1, rect.w, color);
        self.vline(rect.x, rect.y, rect.h, color);
        self.vline(rect.x + rect.w - 1, rect.y, rect.h, color);
    }

    /// Blit a raw BGRA buffer (width × height × 4 bytes) to position (dx, dy)
    pub fn blit(self: *const VesaFb, src: []const u8,
                src_w: u32, src_h: u32, dx: u32, dy: u32) void {
        var row: u32 = 0;
        while (row < src_h) : (row += 1) {
            var col: u32 = 0;
            while (col < src_w) : (col += 1) {
                const idx: usize = (row * src_w + col) * 4;
                if (idx + 3 >= src.len) break;
                const c = Color{ .b=src[idx], .g=src[idx+1], .r=src[idx+2], .a=src[idx+3] };
                self.put_pixel(dx + col, dy + row, c);
            }
        }
    }

    /// Print 8×8 bitmap font character at (x, y)
    /// `glyph` is a 8-byte bitmap (one byte per row, MSB = leftmost pixel)
    pub fn draw_glyph(self: *const VesaFb, glyph: [8]u8,
                      x: u32, y: u32, fg: Color, bg: Color) void {
        for (glyph, 0..) |row_bits, row| {
            for (0..8) |col| {
                const bit: u8 = (row_bits >> @intCast(7 - col)) & 1;
                self.put_pixel(x + @as(u32, @intCast(col)),
                               y + @as(u32, @intCast(row)),
                               if (bit != 0) fg else bg);
            }
        }
    }
};
