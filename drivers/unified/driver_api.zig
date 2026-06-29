// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Unified Driver API Definition (Zig, no stdlib)
//! Replaces: drivers/unified/driver_api.h
//! =========================================================================

pub const DeviceType = enum(u32) {
    Display = 1,
    Network = 2,
    Storage = 3,
    Input = 4,
    Audio = 5,
};

pub const UnifiedDevice = struct {
    name: []const u8,
    dev_type: DeviceType,
    bar_address: usize,

    pub fn class_name(self: *const UnifiedDevice) []const u8 {
        _ = self;
        return "UnifiedDevice";
    }
};

pub const DriverOps = struct {
    init: *const fn (dev: *UnifiedDevice) bool,
    shutdown: *const fn (dev: *UnifiedDevice) void,
    suspend: *const fn (dev: *UnifiedDevice) bool,
    resume: *const fn (dev: *UnifiedDevice) bool,
};
