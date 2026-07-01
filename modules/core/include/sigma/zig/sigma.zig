// SigmaOS: Sovereign Zig Bridge (v1.0)
// USP: Comptime-verified silicon primitives.

const std = @import("std");

pub const hal = struct {
    extern fn sigma_hal_init() void;
    extern fn sigma_hal_personalized_pulse() void;

    pub fn init() void {
        sigma_hal_init();
    }

    pub fn pulse() void {
        sigma_hal_personalized_pulse();
    }
};

pub const Log = struct {
    pub fn info(msg: []const u8) void {
        // Interface with core/lattice/kernel/logging.c
        _ = msg;
    }
};
