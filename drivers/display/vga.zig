// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: VGA Display Driver (Zig, no stdlib, no libc)
//! Replaces: drivers/display/sigma_vga.cpp, sigma_vga_driver.cpp
//! =========================================================================

const VGA_MISC_READ: u16 = 0x3CC;
const VGA_MISC_WRITE: u16 = 0x3C2;
const VGA_CRTC_INDEX: u16 = 0x3D4;
const VGA_CRTC_DATA: u16  = 0x3D5;

fn inb(port: u16) u8 {
    return asm volatile ("inb %[port], %[result]"
        : [result] "={al}" (-> u8),
        : [port] "N{dx}" (port));
}

fn outb(port: u16, val: u8) void {
    asm volatile ("outb %[val], %[port]"
        : : [port] "N{dx}" (port), [val] "{al}" (val));
}

pub const VGADisplay = struct {
    vram: [*]volatile u8,
    width: u32,
    height: u32,

    pub fn new() VGADisplay {
        return VGADisplay{
            .vram = @intToPtr([*]volatile u8, 0xB8000),
            .width = 80,
            .height = 25,
        };
    }

    pub fn initialize(self: *VGADisplay) void {
        // Simple initialization to VGA Text Mode 3 (80x25)
        _ = self;
        outb(VGA_MISC_WRITE, inb(VGA_MISC_READ) | 0x01);
    }

    pub fn write_char(self: *const VGADisplay, x: u32, y: u32, c: u8, attr: u8) void {
        if (x >= self.width or y >= self.height) return;
        const offset = (y * self.width + x) * 2;
        self.vram[offset] = c;
        self.vram[offset + 1] = attr;
    }

    pub fn clear(self: *const VGADisplay) void {
        var y: u32 = 0;
        while (y < self.height) : (y += 1) {
            var x: u32 = 0;
            while (x < self.width) : (x += 1) {
                self.write_char(x, y, ' ', 0x07);
            }
        }
    }
};
