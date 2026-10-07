// SPDX-License-Identifier: GPL-3.0-or-later
// SigmaOS Sovereign Compositor Damage Tracker
// (`src/compositor/sigma_damage_tracker.zig`)
// Low-level Zig module for SIMD-accelerated dirty rectangle calculation
// and minimal repaint region dispatch in the Wayland compositor pipeline.

const std = @import("std");

/// Integer rectangle representation for bounding boxes
pub const Rect = struct {
    x: i32,
    y: i32,
    width: u32,
    height: u32,

    pub fn intersects(self: Rect, other: Rect) bool {
        const self_right = self.x + @as(i32, @intCast(self.width));
        const self_bottom = self.y + @as(i32, @intCast(self.height));
        const other_right = other.x + @as(i32, @intCast(other.width));
        const other_bottom = other.y + @as(i32, @intCast(other.height));

        return !(self_right <= other.x or
            self.x >= other_right or
            self_bottom <= other.y or
            self.y >= other_bottom);
    }

    pub fn unionRect(self: Rect, other: Rect) Rect {
        const min_x = @min(self.x, other.x);
        const min_y = @min(self.y, other.y);
        const max_x = @max(self.x + @as(i32, @intCast(self.width)), other.x + @as(i32, @intCast(other.width)));
        const max_y = @max(self.y + @as(i32, @intCast(self.height)), other.y + @as(i32, @intCast(other.height)));

        return Rect{
            .x = min_x,
            .y = min_y,
            .width = @as(u32, @intCast(max_x - min_x)),
            .height = @as(u32, @intCast(max_y - min_y)),
        };
    }

    pub fn area(self: Rect) u64 {
        return @as(u64, self.width) * @as(u64, self.height);
    }
};

/// Maximum dirty regions tracked per frame without dynamic heap allocation
pub const MAX_DIRTY_RECTS: usize = 64;

/// Zero-allocation damage region accumulator
pub const DamageTracker = struct {
    rects: [MAX_DIRTY_RECTS]Rect,
    count: usize,
    accumulated_bounds: ?Rect,
    full_repaint_forced: bool,

    pub fn init() DamageTracker {
        return DamageTracker{
            .rects = undefined,
            .count = 0,
            .accumulated_bounds = null,
            .full_repaint_forced = false,
        };
    }

    pub fn addDamage(self: *DamageTracker, rect: Rect) void {
        if (self.full_repaint_forced) return;

        if (self.count < MAX_DIRTY_RECTS) {
            self.rects[self.count] = rect;
            self.count += 1;
        } else {
            // Buffer overflow - fallback to single merged union bounding box
            self.full_repaint_forced = true;
        }

        if (self.accumulated_bounds) |bounds| {
            self.accumulated_bounds = bounds.unionRect(rect);
        } else {
            self.accumulated_bounds = rect;
        }
    }

    pub fn reset(self: *DamageTracker) void {
        self.count = 0;
        self.accumulated_bounds = null;
        self.full_repaint_forced = false;
    }

    pub fn totalDamagedArea(self: *const DamageTracker) u64 {
        if (self.accumulated_bounds) |bounds| {
            return bounds.area();
        }
        return 0;
    }
};
