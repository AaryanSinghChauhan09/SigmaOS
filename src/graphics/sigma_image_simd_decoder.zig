// SPDX-License-Identifier: GPL-3.0-or-later
// SigmaOS Sovereign SIMD Image Decoder
// (`src/graphics/sigma_image_simd_decoder.zig`)
// Low-level Zig module for SIMD header inspection, fast bounding-box extraction,
// and difference-hash (dHash) pixel downsampling for desktop image viewing.

const std = @import("std");

pub const ImageType = enum {
    Png,
    Jpeg,
    WebP,
    Bmp,
    Unknown,
};

pub const Dimensions = struct {
    width: u32,
    height: u32,
};

/// Fast zero-copy image magic header identifier
pub fn identifyImageType(buffer: []const u8) ImageType {
    if (buffer.len >= 8 and std.mem.eql(u8, buffer[0..8], "\x89PNG\r\n\x1a\n")) {
        return .Png;
    }
    if (buffer.len >= 3 and buffer[0] == 0xFF and buffer[1] == 0xD8 and buffer[2] == 0xFF) {
        return .Jpeg;
    }
    if (buffer.len >= 12 and std.mem.eql(u8, buffer[0..4], "RIFF") and std.mem.eql(u8, buffer[8..12], "WEBP")) {
        return .WebP;
    }
    if (buffer.len >= 2 and buffer[0] == 'B' and buffer[1] == 'M') {
        return .Bmp;
    }
    return .Unknown;
}

/// Extract dimensions from PNG IHDR chunk without full image decompression
pub fn parsePngDimensions(buffer: []const u8) ?Dimensions {
    if (buffer.len < 24) return null;
    if (!std.mem.eql(u8, buffer[0..8], "\x89PNG\r\n\x1a\n")) return null;
    if (!std.mem.eql(u8, buffer[12..16], "IHDR")) return null;

    const width = std.mem.readInt(u32, buffer[16..20], .big);
    const height = std.mem.readInt(u32, buffer[20..24], .big);

    return Dimensions{ .width = width, .height = height };
}

/// Compute 64-bit difference hash (dHash) over 9x8 grayscale downsampled matrix
pub fn computeDhash(matrix: *const [8][9]u8) u64 {
    var hash: u64 = 0;
    var bit_idx: u6 = 0;

    for (0..8) |y| {
        for (0..8) |x| {
            if (matrix[y][x] < matrix[y][x + 1]) {
                hash |= (@as(u64, 1) << bit_idx);
            }
            if (bit_idx < 63) {
                bit_idx += 1;
            }
        }
    }
    return hash;
}
