// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Intel HDA Audio Driver (Zig, no stdlib, no libc)
//! Replaces: drivers/audio/sigma_audio_hda.cpp / updates drivers/audio/hda.zig
//! =========================================================================

const HDA_GCAP:     u32 = 0x00; // Global Capabilities
const HDA_GCTL:     u32 = 0x08; // Global Control
const HDA_WAKEEN:   u32 = 0x0C; // Wake Enable
const HDA_STATESTS: u32 = 0x0E; // State Change Status
const HDA_CORBSIZE: u32 = 0x4E; // CORB Size
const HDA_CORBWP:   u32 = 0x4A; // CORB Write Pointer
const HDA_CORBRP:   u32 = 0x48; // CORB Read Pointer
const HDA_RIRBWP:   u32 = 0x5A; // RIRB Write Pointer
const HDA_RIRBRP:   u32 = 0x58; // RIRB Read Pointer
const HDA_CORBBASE: u32 = 0x40; // CORB Lower Base Address
const HDA_RIRBBASE: u32 = 0x50; // RIRB Lower Base Address

fn mmio_read8(base: usize, offset: u32) u8 {
    const ptr: *volatile u8 = @intToPtr(*volatile u8, base + offset);
    return ptr.*;
}

fn mmio_write8(base: usize, offset: u32, val: u8) void {
    const ptr: *volatile u8 = @intToPtr(*volatile u8, base + offset);
    ptr.* = val;
}

fn mmio_read16(base: usize, offset: u32) u16 {
    const ptr: *volatile u16 = @intToPtr(*volatile u16, base + offset);
    return ptr.*;
}

fn mmio_write16(base: usize, offset: u32, val: u16) void {
    const ptr: *volatile u16 = @intToPtr(*volatile u16, base + offset);
    ptr.* = val;
}

fn mmio_read32(base: usize, offset: u32) u32 {
    const ptr: *volatile u32 = @intToPtr(*volatile u32, base + offset);
    return ptr.*;
}

fn mmio_write32(base: usize, offset: u32, val: u32) void {
    const ptr: *volatile u32 = @intToPtr(*volatile u32, base + offset);
    ptr.* = val;
}

pub const HDAStream = struct {
    index:    u8,
    active:   bool,
    channels: u8,
    rate_hz:  u32,
    stream_type: u8, // 0 = input, 1 = output

    pub fn new(index: u8, channels: u8, rate: u32, s_type: u8) HDAStream {
        return HDAStream{
            .index = index,
            .active = false,
            .channels = channels,
            .rate_hz = rate,
            .stream_type = s_type,
        };
    }

    pub fn start(self: *HDAStream, base: usize) void {
        const stream_offset = 0x80 + (self.index * 0x20);
        var val = mmio_read8(base, stream_offset);
        val |= 0x02; // Stream Run
        mmio_write8(base, stream_offset, val);
        self.active = true;
    }

    pub fn stop(self: *HDAStream, base: usize) void {
        const stream_offset = 0x80 + (self.index * 0x20);
        var val = mmio_read8(base, stream_offset);
        val &= ~@as(u8, 0x02); // Clear Stream Run
        mmio_write8(base, stream_offset, val);
        self.active = false;
    }

    pub fn class_name(self: *const HDAStream) []const u8 {
        _ = self;
        return "HDAStream";
    }
};

/// Intel High Definition Audio Controller — sovereign Zig OOP struct
pub const HDAController = struct {
    mmio_base:   usize,
    initialized: bool,
    num_outputs: u8,
    num_inputs:  u8,
    corb_entries: u16 = 256,
    rirb_entries: u16 = 256,

    pub fn new(base: usize) HDAController {
        return HDAController{
            .mmio_base = base,
            .initialized = false,
            .num_outputs = 0,
            .num_inputs = 0,
        };
    }

    /// Reset and initialize the HDA controller
    pub fn initialize(self: *HDAController) bool {
        // Assert reset
        var gctl = mmio_read32(self.mmio_base, HDA_GCTL);
        gctl &= ~@as(u32, 1);
        mmio_write32(self.mmio_base, HDA_GCTL, gctl);

        // Wait for reset to be active
        var timeout: u32 = 10000;
        while (mmio_read32(self.mmio_base, HDA_GCTL) & 1 != 0) {
            timeout -= 1;
            if (timeout == 0) return false;
        }

        // Deassert reset
        gctl = mmio_read32(self.mmio_base, HDA_GCTL);
        gctl |= 1;
        mmio_write32(self.mmio_base, HDA_GCTL, gctl);

        // Wait for reset to be deasserted
        timeout = 10000;
        while (mmio_read32(self.mmio_base, HDA_GCTL) & 1 == 0) {
            timeout -= 1;
            if (timeout == 0) return false;
        }

        const gcap = mmio_read32(self.mmio_base, HDA_GCAP);
        self.num_outputs = @intCast(u8, (gcap >> 12) & 0xF);
        self.num_inputs  = @intCast(u8, (gcap >> 8)  & 0xF);

        self.setup_ring_buffers();
        self.initialized = true;
        return true;
    }

    fn setup_ring_buffers(self: *HDAController) void {
        // Set CORB size to 256 entries (size capability 0x40 = 256 entries)
        mmio_write8(self.mmio_base, HDA_CORBSIZE, 0x02);
        // Set RIRB size to 256 entries
        mmio_write8(self.mmio_base, HDA_CORBSIZE + 0x10, 0x02);

        // Reset CORB/RIRB read/write pointers
        mmio_write16(self.mmio_base, HDA_CORBWP, 0);
        // Reset RIRB WP (Read write index)
        mmio_write16(self.mmio_base, HDA_RIRBWP, 0x8000); // Reset RIRB WP bit
    }

    pub fn enable_wake(self: *HDAController) void {
        mmio_write32(self.mmio_base, HDA_WAKEEN, 0xFF);
    }

    pub fn class_name(self: *const HDAController) []const u8 {
        _ = self;
        return "HDAController";
    }
};
