// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Intel HDA Audio Driver (Zig, no stdlib, no libc)
//! Replaces: drivers/audio/sigma_audio_hda.cpp
//! =========================================================================

const HDA_GCAP:     u32 = 0x00; // Global Capabilities
const HDA_GCTL:     u32 = 0x08; // Global Control
const HDA_WAKEEN:   u32 = 0x0C; // Wake Enable
const HDA_STATESTS: u32 = 0x0E; // State Change Status
const HDA_CORBSIZE: u32 = 0x4E; // CORB Size
const HDA_RIRB:     u32 = 0x50; // RIRB Base

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

    pub fn new(index: u8, channels: u8, rate: u32) HDAStream {
        return HDAStream{ .index = index, .active = false,
                          .channels = channels, .rate_hz = rate };
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

    pub fn new(base: usize) HDAController {
        return HDAController{ .mmio_base = base, .initialized = false,
                              .num_outputs = 0, .num_inputs = 0 };
    }

    /// Reset and initialize the HDA controller
    pub fn initialize(self: *HDAController) bool {
        // Assert reset
        mmio_write32(self.mmio_base, HDA_GCTL, 0);
        // Wait cycles (stub)
        var i: usize = 0;
        while (i < 1000) : (i += 1) {}
        // Deassert reset
        mmio_write32(self.mmio_base, HDA_GCTL, 1);

        const gcap = mmio_read32(self.mmio_base, HDA_GCAP);
        self.num_outputs = @intCast(u8, (gcap >> 12) & 0xF);
        self.num_inputs  = @intCast(u8, (gcap >> 8)  & 0xF);
        self.initialized = true;
        return true;
    }

    pub fn enable_wake(self: *HDAController) void {
        mmio_write32(self.mmio_base, HDA_WAKEEN, 0xFF);
    }

    pub fn class_name(self: *const HDAController) []const u8 {
        _ = self;
        return "HDAController";
    }
};
