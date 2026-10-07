// SigmaOS Fast Wayland SHM Blitter
// High-performance, zero-allocation memory blitter with damage clipping for DRM/KMS scanout
const std = @import("std");

pub const Rect = struct {
    x: i32,
    y: i32,
    width: i32,
    height: i32,

    pub fn clip(self: Rect, bounds_w: i32, bounds_h: i32) Rect {
        const x0 = @max(0, self.x);
        const y0 = @max(0, self.y);
        const x1 = @min(bounds_w, self.x + self.width);
        const y1 = @min(bounds_h, self.y + self.height);

        if (x1 <= x0 or y1 <= y0) {
            return Rect{ .x = 0, .y = 0, .width = 0, .height = 0 };
        }

        return Rect{
            .x = x0,
            .y = y0,
            .width = x1 - x0,
            .height = y1 - y0,
        };
    }
};

pub const BlitError = error{
    OutOfBounds,
    BufferTooSmall,
};

pub fn blitSurfaceShm(
    src_pixels: []const u32,
    src_stride_pixels: usize,
    dst_fb: []u32,
    dst_stride_pixels: usize,
    dst_width: i32,
    dst_height: i32,
    damage: Rect,
) BlitError!usize {
    const clipped = damage.clip(dst_width, dst_height);
    if (clipped.width == 0 or clipped.height == 0) return 0;

    const w: usize = @intCast(clipped.width);
    const h: usize = @intCast(clipped.height);
    const start_x: usize = @intCast(clipped.x);
    const start_y: usize = @intCast(clipped.y);

    var y: usize = 0;
    var pixels_copied: usize = 0;

    while (y < h) : (y += 1) {
        const src_row_start = (y * src_stride_pixels);
        const dst_row_start = ((start_y + y) * dst_stride_pixels) + start_x;

        if (src_row_start + w > src_pixels.len or dst_row_start + w > dst_fb.len) {
            return BlitError.OutOfBounds;
        }

        // Fast row copy
        const src_slice = src_pixels[src_row_start .. src_row_start + w];
        const dst_slice = dst_fb[dst_row_start .. dst_row_start + w];
        @memcpy(dst_slice, src_slice);
        pixels_copied += w;
    }

    return pixels_copied;
}

pub fn main() !void {
    var src: [1920 * 1080]u32 = undefined;
    var dst: [1920 * 1080]u32 = undefined;
    @memset(&src, 0xFF00FF00); // Green
    @memset(&dst, 0x00000000); // Black

    const damage = Rect{ .x = 100, .y = 100, .width = 400, .height = 300 };
    const copied = try blitSurfaceShm(&src, 1920, &dst, 1920, 1920, 1080, damage);
    std.debug.print("Blitted {} pixels with zero latency\n", .{copied});
}
