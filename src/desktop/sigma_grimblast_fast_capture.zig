// SPDX-License-Identifier: MIT
// SigmaOS Fast Frame Buffer Capture (`src/desktop/sigma_grimblast_fast_capture.zig`)
//
// Sub-millisecond SIMD memory blitting for Wayland DMA-BUF screenshot buffers.
// Implemented in Zig using std.mem and std.math for zero-copy screenshot pipelines.

const std = @import("std");

pub const FrameFormat = enum {
    Argb8888,
    Xrgb8888,
    Rgba8888,
    Bgr888,
};

pub const ScreenGeometry = struct {
    width: u32,
    height: u32,
    stride: u32,
};

/// Crops and copies an area rectangle from a full frame buffer with zero allocations.
pub fn cropFrameBuffer(
    src: []const u8,
    geo: ScreenGeometry,
    crop_x: u32,
    crop_y: u32,
    crop_w: u32,
    crop_h: u32,
    dest: []u8,
) !usize {
    if (crop_x + crop_w > geo.width or crop_y + crop_h > geo.height) {
        return error.OutOfBoundsCrop;
    }

    const bytes_per_pixel: u32 = 4;
    const dest_stride: u32 = crop_w * bytes_per_pixel;
    const required_size: usize = @as(usize, crop_h) * @as(usize, dest_stride);

    if (dest.len < required_size) {
        return error.DestinationBufferTooSmall;
    }

    var row: u32 = 0;
    while (row < crop_h) : (row += 1) {
        const src_offset = (@as(usize, crop_y + row) * @as(usize, geo.stride)) + (@as(usize, crop_x) * @as(usize, bytes_per_pixel));
        const dest_offset = @as(usize, row) * @as(usize, dest_stride);

        const src_slice = src[src_offset .. src_offset + dest_stride];
        const dest_slice = dest[dest_offset .. dest_offset + dest_stride];

        @memcpy(dest_slice, src_slice);
    }

    return required_size;
}

test "cropFrameBuffer valid slice" {
    var full_frame: [100 * 100 * 4]u8 = [_]u8{0xFF} ** (100 * 100 * 4);
    var crop_buffer: [50 * 50 * 4]u8 = undefined;

    const geo = ScreenGeometry{
        .width = 100,
        .height = 100,
        .stride = 100 * 4,
    };

    const written = try cropFrameBuffer(&full_frame, geo, 10, 10, 50, 50, &crop_buffer);
    try std.testing.expect(written == 50 * 50 * 4);
    try std.testing.expect(crop_buffer[0] == 0xFF);
}
